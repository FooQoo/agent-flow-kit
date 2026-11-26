use anyhow::{Context, Result, bail, ensure};
use rusqlite::{Connection, OptionalExtension, Transaction, params};
use serde::Serialize;

use crate::{
    Column, Document, Store, history, ids, pending_count, plan_exists, progress::candidate,
};

const MAX_DRAFT_TASKS: usize = 32;

fn execution_label(value: Option<&str>) -> Option<String> {
    let label: String = value?
        .trim()
        .chars()
        .filter(|character| !character.is_control())
        .take(120)
        .map(|character| if character == '`' { '\'' } else { character })
        .collect();
    (!label.trim().is_empty()).then_some(label)
}

#[derive(Debug, Serialize)]
pub struct Draft {
    pub plan_id: i64,
    pub draft_id: i64,
    pub base_revision: i64,
    pub plan_path: std::path::PathBuf,
    pub tasks: Vec<Document>,
}

#[derive(Debug, Serialize)]
pub struct PlanView {
    pub plan_id: i64,
    pub plan: Option<Document>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<Draft>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub requests: Vec<Document>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub questions: Vec<QuestionView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<TaskView>>,
    pub progress: crate::Progress,
}

#[derive(Debug, Serialize)]
pub struct QuestionView {
    #[serde(flatten)]
    document: Document,
    state: &'static str,
}

#[derive(Debug, Serialize)]
pub struct TaskView {
    #[serde(flatten)]
    document: Document,
    status: Column,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    after: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    handoff: Option<Handoff>,
}

#[derive(Debug, Serialize)]
pub struct Handoff {
    attempt: i64,
    note: String,
    updated_at: i64,
}

#[derive(Debug, Serialize)]
pub struct TaskRun {
    pub plan_id: i64,
    #[serde(flatten)]
    pub task: Document,
    pub attempt: i64,
    pub status: Column,
}

struct WorkState {
    digest: String,
    final_hash: Option<String>,
    criteria: u32,
}

impl Store {
    pub(crate) fn plan_document(&self, plan: i64, draft: i64) -> Document {
        Document {
            id: plan,
            path: self
                .base
                .join("plans")
                .join(format!("{plan:03}-r{draft:03}.md")),
        }
    }

    pub fn inspect(&self, plan_id: i64, include_tasks: bool) -> Result<PlanView> {
        self.read(|tx| {
            let progress = self.snapshot(tx, plan_id)?;
            let (active, open): (Option<i64>, Option<i64>) = tx.query_row(
                "SELECT active_draft,(SELECT id FROM drafts WHERE plan_id=plans.id AND state='open') FROM plans WHERE id=?1",
                [plan_id], |r| Ok((r.get(0)?, r.get(1)?)))?;
            let draft = open.map(|id| self.draft(tx, plan_id, id)).transpose()?;
            let requests = pending_requests(tx, plan_id)?.into_iter().map(|id| self.document("requests", id)).collect();
            let mut statement = tx.prepare("SELECT id,prompt_digest IS NOT NULL,answer_digest IS NOT NULL
                FROM questions WHERE plan_id=?1 AND applied_revision IS NULL ORDER BY id")?;
            let questions = statement.query_map([plan_id], |r| {
                let state = match (r.get::<_, bool>(1)?, r.get::<_, bool>(2)?) {
                    (_, true) => "answered",
                    (true, false) => "waiting",
                    _ => "draft",
                };
                Ok(QuestionView { document: self.document("questions", r.get(0)?), state })
            })?.collect::<rusqlite::Result<_>>()?;
            let tasks = include_tasks.then(|| self.list_tasks(tx, plan_id)).transpose()?;
            Ok(PlanView { plan_id, plan: active.map(|id| self.plan_document(plan_id, id)), draft, requests, questions, tasks, progress })
        })
    }

    fn list_tasks(&self, tx: &Connection, plan_id: i64) -> Result<Vec<TaskView>> {
        let mut statement = tx.prepare(
            "SELECT id,status FROM tasks WHERE plan_id=?1 AND lifecycle='active' ORDER BY id",
        )?;
        let rows = statement.query_map([plan_id], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })?;
        rows.map(|row| {
            let (id, status) = row?;
            Ok(TaskView {
                document: self.document("tasks", id),
                status: status
                    .parse()
                    .map_err(|()| anyhow::anyhow!("invalid task state"))?,
                after: ids(
                    tx,
                    "SELECT prerequisite FROM dependencies WHERE task_id=?1 ORDER BY prerequisite",
                    id,
                )?,
                handoff: tx
                    .query_row(
                        "SELECT attempt,note,updated_at FROM handoffs WHERE task_id=?1",
                        [id],
                        |row| {
                            Ok(Handoff {
                                attempt: row.get(0)?,
                                note: row.get(1)?,
                                updated_at: row.get(2)?,
                            })
                        },
                    )
                    .optional()?,
            })
        })
        .collect()
    }

    pub fn prepare_plan(
        &self,
        plan_id: i64,
        revision: i64,
        task_dependencies: &[Vec<i64>],
        remove: &[i64],
        replace_draft: Option<i64>,
    ) -> Result<Draft> {
        ensure!(
            task_dependencies.len() <= MAX_DRAFT_TASKS && remove.len() <= MAX_DRAFT_TASKS,
            "limit 32 task additions and removals per draft"
        );
        let draft = self.write(|tx| {
            plan_exists(tx, plan_id)?;
            let current: i64 = tx.query_row("SELECT revision FROM plans WHERE id=?1", [plan_id], |r| r.get(0))?;
            ensure!(current == revision, "plan changed; inspect it before replanning");
            let open: Option<i64> = tx.query_row("SELECT id FROM drafts WHERE plan_id=?1 AND state='open'", [plan_id], |r| r.get(0)).optional()?;
            ensure!(open == replace_draft, "open draft changed; inspect it, recover its files or replace it by ID");
            if let Some(id) = open {
                tx.execute("UPDATE drafts SET state='discarded' WHERE id=?1", [id])?;
                history(tx, plan_id, "draft_discarded", id, None)?;
            }
            tx.execute("INSERT INTO drafts(plan_id,base_revision) VALUES(?1,?2)", params![plan_id, revision])?;
            let draft = tx.last_insert_rowid();
            self.require_unowned_path_available(&self.plan_document(plan_id, draft))?;
            for &id in remove {
                ensure!(tx.query_row("SELECT EXISTS(SELECT 1 FROM tasks WHERE id=?1 AND plan_id=?2 AND lifecycle='active' AND status<>'done')",
                    params![id, plan_id], |r| r.get::<_, bool>(0))?, "only unfinished active tasks can be removed");
                tx.execute("INSERT INTO draft_removals VALUES(?1,?2)", params![draft, id])?;
            }
            let mut added = Vec::new();
            for dependencies in task_dependencies {
                ensure!(dependencies.len() <= MAX_DRAFT_TASKS, "limit 32 prerequisites per task");
                tx.execute("INSERT INTO tasks(plan_id,draft_id) VALUES(?1,?2)", params![plan_id, draft])?;
                let task = tx.last_insert_rowid();
                self.require_unowned_path_available(&self.document("tasks", task))?;
                for &reference in dependencies {
                    let prerequisite = if reference < 0 {
                        let index = reference.unsigned_abs() - 1;
                        *added.get(index as usize).context("negative prerequisites must index an earlier new task")?
                    } else {
                        ensure!(tx.query_row("SELECT EXISTS(SELECT 1 FROM tasks WHERE id=?1 AND plan_id=?2 AND lifecycle='active')",
                            params![reference, plan_id], |r| r.get::<_, bool>(0))?, "prerequisite must be an active task in this plan");
                        ensure!(!remove.contains(&reference), "prerequisite is marked for removal");
                        reference
                    };
                    tx.execute("INSERT INTO dependencies VALUES(?1,?2)", params![task, prerequisite])?;
                }
                added.push(task);
            }
            history(tx, plan_id, "draft_prepared", draft, None)?;
            self.draft(tx, plan_id, draft)
        })?;
        self.prepare_draft_files(&draft)?;
        Ok(draft)
    }

    pub fn recover_draft(&self, plan_id: i64, draft_id: i64) -> Result<Draft> {
        let draft = self.read(|tx| {
            ensure!(tx.query_row("SELECT EXISTS(SELECT 1 FROM drafts WHERE id=?1 AND plan_id=?2 AND state='open')",
                params![draft_id, plan_id], |r| r.get::<_, bool>(0))?, "draft is no longer open");
            self.draft(tx, plan_id, draft_id)
        })?;
        self.prepare_draft_files(&draft)?;
        Ok(draft)
    }

    fn draft(&self, tx: &Connection, plan_id: i64, draft_id: i64) -> Result<Draft> {
        let base_revision = tx.query_row(
            "SELECT base_revision FROM drafts WHERE id=?1 AND plan_id=?2",
            params![draft_id, plan_id],
            |r| r.get(0),
        )?;
        let tasks = ids(
            tx,
            "SELECT id FROM tasks WHERE draft_id=?1 ORDER BY id",
            draft_id,
        )?
        .into_iter()
        .map(|id| self.document("tasks", id))
        .collect();
        Ok(Draft {
            plan_id,
            draft_id,
            base_revision,
            plan_path: self.plan_document(plan_id, draft_id).path,
            tasks,
        })
    }

    fn prepare_draft_files(&self, draft: &Draft) -> Result<()> {
        self.prepare_file(
            &self.plan_document(draft.plan_id, draft.draft_id),
            crate::documents::WORK_TEMPLATE,
        )?;
        for task in &draft.tasks {
            self.prepare_file(task, crate::documents::TASK_TEMPLATE)?;
        }
        Ok(())
    }

    pub fn commit_plan(
        &self,
        plan: i64,
        draft: i64,
        requests: &[i64],
        answers: &[i64],
        execute: bool,
    ) -> Result<crate::Progress> {
        self.write(|tx| {
            let status = plan_exists(tx, plan)?;
            let (revision, current): (i64, Option<i64>) = tx.query_row(
                "SELECT revision,active_draft FROM plans WHERE id=?1", [plan], |r| Ok((r.get(0)?, r.get(1)?)))?;
            if current == Some(draft) {
                return self.snapshot(tx, plan);
            }
            let model = self.draft(tx, plan, draft)?;
            ensure!(model.base_revision == revision && tx.query_row(
                "SELECT state='open' FROM drafts WHERE id=?1", [draft], |r| r.get::<_, bool>(0))?,
                "draft is stale; inspect the current plan");
            ensure!(pending_requests(tx, plan)? == sorted(requests), "request set changed; inspect and reflect all pending requests");
            let pending_answers = ids(tx, "SELECT id FROM questions WHERE plan_id=?1 AND applied_revision IS NULL ORDER BY id", plan)?;
            ensure!(pending_answers == sorted(answers), "question set changed; inspect and reflect all pending answers");
            for &id in answers {
                let body = self.read_question(id, true)?;
                let matches = tx.query_row("SELECT prompt_digest=?1 AND answer_digest=?2 FROM questions WHERE id=?3",
                    params![body.digest, body.final_hash, id], |r| r.get::<_, Option<bool>>(0))?;
                ensure!(matches == Some(true), "answer question {id} and preserve its sealed text before applying it");
            }
            for &id in requests {
                let digest = self.read_request(id)?;
                tx.execute("UPDATE requests SET applied_revision=?1,digest=?2 WHERE id=?3", params![revision+1, digest, id])?;
                history(tx, plan, "request_applied", id, None)?;
            }
            ensure!(status != Column::Done || !requests.is_empty(), "register a request before reopening a completed plan");
            if let Some(current) = current {
                self.check_plan(tx, plan, current, status == Column::Done)?;
            }

            let retained = "lifecycle='active' AND id NOT IN (SELECT task_id FROM draft_removals WHERE draft_id=?2)";
            let count: u32 = tx.query_row(&format!("SELECT count(*) FROM tasks WHERE plan_id=?1 AND (({retained}) OR (lifecycle='draft' AND draft_id=?2))"),
                params![plan, draft], |r| r.get(0))?;
            ensure!(count > 0, "a confirmed plan needs at least one task");
            ensure!(!tx.query_row(&format!("SELECT EXISTS(SELECT 1 FROM tasks WHERE plan_id=?1 AND ({retained}) AND EXISTS(
                SELECT 1 FROM dependencies JOIN draft_removals ON dependencies.task_id=tasks.id AND dependencies.prerequisite=draft_removals.task_id AND draft_removals.draft_id=?2))"),
                params![plan,draft], |r| r.get::<_, bool>(0))?, "replace each retained task whose prerequisite is being removed");

            let spec = self.read_work(&self.plan_document(plan, draft), false)?;
            let new_status = match (status, execute) {
                (Column::Backlog, false) => Column::Backlog,
                (Column::Backlog | Column::Todo, _) => Column::Todo,
                (Column::Doing | Column::Done, _) => Column::Doing,
            };
            let unstarted = if new_status == Column::Backlog { Column::Backlog } else { Column::Todo };
            for task in &model.tasks {
                let spec = self.read_work(task, false)?;
                tx.execute("UPDATE tasks SET lifecycle='active',status=?1,title=?2,spec_hash=?3,criteria=?4 WHERE id=?5",
                    params![unstarted.as_str(), spec.title, spec.digest, spec.criteria, task.id])?;
            }
            tx.execute("UPDATE tasks SET lifecycle='retired',owner=NULL WHERE id IN (SELECT task_id FROM draft_removals WHERE draft_id=?1)", [draft])?;
            tx.execute("UPDATE tasks SET status=?1,owner=NULL WHERE plan_id=?2 AND lifecycle='active' AND status<>'done'",
                params![unstarted.as_str(), plan])?;
            tx.execute("UPDATE drafts SET state='accepted' WHERE id=?1", [draft])?;
            tx.execute("UPDATE plans SET status=?1,revision=revision+1,active_draft=?2,title=?3,spec_hash=?4,final_hash=NULL,criteria=?5 WHERE id=?6",
                params![new_status.as_str(), draft, spec.title, spec.digest, spec.criteria, plan])?;
            for &id in answers {
                tx.execute("UPDATE questions SET applied_revision=?1 WHERE id=?2", params![revision+1, id])?;
                history(tx, plan, "answer_applied", id, None)?;
            }
            history(tx, plan, "plan_confirmed", draft, None)?;
            self.snapshot(tx, plan)
        })
    }

    pub fn authorize_execution(&self, plan: i64) -> Result<crate::Progress> {
        self.write(|tx| {
            let snapshot = self.snapshot(tx, plan)?;
            if snapshot.status == Column::Backlog {
                ensure!(snapshot.revision > 0, "confirm the plan before authorizing it");
                execution_ready(tx, plan)?;
                let active: i64 = tx.query_row("SELECT active_draft FROM plans WHERE id=?1", [plan], |r| r.get(0))?;
                self.check_plan(tx, plan, active, false)?;
                tx.execute("UPDATE plans SET status='todo' WHERE id=?1", [plan])?;
                tx.execute("UPDATE tasks SET status='todo' WHERE plan_id=?1 AND lifecycle='active' AND status='backlog'", [plan])?;
                history(tx, plan, "execution_authorized", plan, None)?;
            }
            self.snapshot(tx, plan)
        })
    }

    pub fn start_next(&self, plan: i64, owner: &str, take_over: bool) -> Result<TaskRun> {
        self.start_next_as(plan, owner, take_over, None, None)
    }

    pub fn start_next_as(
        &self,
        plan: i64,
        owner: &str,
        take_over: bool,
        agent: Option<&str>,
        model: Option<&str>,
    ) -> Result<TaskRun> {
        let run = self.write(|tx| {
            let status = plan_exists(tx, plan)?;
            ensure!(
                matches!(status, Column::Todo | Column::Doing),
                "confirm and authorize this plan before starting"
            );
            execution_ready(tx, plan)?;
            let active: i64 =
                tx.query_row("SELECT active_draft FROM plans WHERE id=?1", [plan], |r| {
                    r.get(0)
                })?;
            self.check_plan(tx, plan, active, false)?;
            let (id, _title, state) =
                candidate(tx, plan)?.context("no ready task; inspect plan progress")?;
            let (assigned, old_attempt): (Option<String>, i64) =
                tx.query_row("SELECT owner,attempt FROM tasks WHERE id=?1", [id], |r| {
                    Ok((r.get(0)?, r.get(1)?))
                })?;
            let task = self.document("tasks", id);
            self.check_task(tx, &task, false)?;
            for prerequisite in ids(
                tx,
                "SELECT prerequisite FROM dependencies WHERE task_id=?1",
                id,
            )? {
                self.check_task(tx, &self.document("tasks", prerequisite), true)?;
            }
            if assigned.as_deref() == Some(owner) && state == Column::Doing {
                return Ok(TaskRun {
                    plan_id: plan,
                    task,
                    attempt: old_attempt,
                    status: state,
                });
            }
            ensure!(
                state != Column::Doing || take_over,
                "a different session owns this task; take over only after its worker stops"
            );
            let attempt = old_attempt + 1;
            tx.execute(
                "UPDATE tasks SET status='doing',owner=?1,attempt=?2 WHERE id=?3",
                params![owner, attempt, id],
            )?;
            tx.execute(
                "INSERT INTO task_runs(task_id,attempt,agent,model) VALUES (?1,?2,?3,?4)",
                params![
                    id,
                    attempt,
                    execution_label(agent).as_deref().unwrap_or("Unknown"),
                    execution_label(model)
                ],
            )?;
            tx.execute("UPDATE plans SET status='doing' WHERE id=?1", [plan])?;
            history(
                tx,
                plan,
                if state == Column::Doing {
                    "task_resumed"
                } else {
                    "task_started"
                },
                id,
                None,
            )?;
            Ok(TaskRun {
                plan_id: plan,
                task,
                attempt,
                status: Column::Doing,
            })
        })?;
        // Serialize projections of the committed history with concurrent starts and takeovers.
        self.write(|tx| self.refresh_task_runs(tx, &run.task))?;
        Ok(run)
    }

    fn refresh_task_runs(&self, tx: &Connection, task: &Document) -> Result<()> {
        let mut query = tx.prepare(
            "SELECT attempt,agent,model FROM task_runs WHERE task_id=?1 ORDER BY attempt",
        )?;
        let lines = query
            .query_map([task.id], |row| {
                let attempt: i64 = row.get(0)?;
                let agent: String = row.get(1)?;
                let model: Option<String> = row.get(2)?;
                let model = model.as_deref().unwrap_or("Unknown");
                Ok(format!("- Attempt {attempt}: `{agent}` (model: `{model}`)"))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        if !lines.is_empty() {
            self.render_task_runs(task, &lines.join("\n"))?;
        }
        Ok(())
    }

    pub fn save_handoff(
        &self,
        plan: i64,
        task: i64,
        owner: &str,
        attempt: i64,
        note: &str,
    ) -> Result<()> {
        let note = note.trim();
        ensure!(
            !note.is_empty() && note.len() <= 16 * 1024,
            "handoff must be 1–16384 bytes"
        );
        self.write(|tx| {
            let owns_task: bool = tx.query_row(
                "SELECT EXISTS(SELECT 1 FROM tasks WHERE id=?1 AND plan_id=?2
                    AND lifecycle='active' AND status='doing' AND owner=?3 AND attempt=?4)",
                params![task, plan, owner, attempt],
                |row| row.get(0),
            )?;
            ensure!(
                owns_task,
                "task attempt changed or is not owned by this session"
            );
            tx.execute(
                "INSERT INTO handoffs(task_id,attempt,note,updated_at) VALUES (?1,?2,?3,unixepoch())
                 ON CONFLICT(task_id) DO UPDATE SET attempt=excluded.attempt, note=excluded.note,
                     updated_at=excluded.updated_at
                 WHERE handoffs.attempt<>excluded.attempt OR handoffs.note<>excluded.note",
                params![task, attempt, note],
            )?;
            Ok(())
        })
    }

    pub fn finish_task(
        &self,
        plan: i64,
        id: i64,
        owner: &str,
        attempt: i64,
        verified: &[i64],
    ) -> Result<crate::Progress> {
        self.write(|tx| {
            let (status, assigned, current): (String, Option<String>, i64) = tx.query_row(
                "SELECT status,owner,attempt FROM tasks WHERE id=?1 AND plan_id=?2 AND lifecycle='active'",
                params![id, plan], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))
                .optional()?.context("task is no longer active in this plan")?;
            ensure!(current == attempt && assigned.as_deref() == Some(owner), "task attempt changed; inspect before continuing");
            let doc = self.document("tasks", id);
            let spec = self.check_task(tx, &doc, status == "done")?;
            if status == "doing" {
                self.refresh_task_runs(tx, &doc)?;
            }
            let result = self.read_work(&doc, true)?;
            ensure!(spec.digest == result.digest, "task specification changed during validation");
            verify(spec.criteria, verified)?;
            if status == "done" {
                ensure!(spec.final_hash == result.final_hash, "task result changed after completion");
                return self.snapshot(tx, plan);
            }
            ensure!(status == "doing", "start this task before finishing it");
            execution_ready(tx, plan)?;
            let active = tx.query_row("SELECT active_draft FROM plans WHERE id=?1", [plan], |r| r.get(0))?;
            self.check_plan(tx, plan, active, false)?;
            tx.execute("UPDATE tasks SET status='done',final_hash=?1 WHERE id=?2", params![result.final_hash, id])?;
            history(tx, plan, "task_completed", id, Some(spec.criteria))?;
            self.snapshot(tx, plan)
        })
    }

    pub fn finish_plan(
        &self,
        plan: i64,
        revision: i64,
        verified: &[i64],
    ) -> Result<crate::Progress> {
        self.write(|tx| {
            let snapshot = self.snapshot(tx, plan)?;
            ensure!(
                snapshot.revision == revision,
                "plan revision changed; verify the current revision"
            );
            execution_ready(tx, plan)?;
            ensure!(
                matches!(snapshot.status, Column::Doing | Column::Done)
                    && snapshot.total > 0
                    && snapshot.done == snapshot.total,
                "finish every active task before completing the plan"
            );
            let active =
                tx.query_row("SELECT active_draft FROM plans WHERE id=?1", [plan], |r| {
                    r.get(0)
                })?;
            let status = snapshot.status;
            let spec = self.check_plan(tx, plan, active, status == Column::Done)?;
            verify(spec.criteria, verified)?;
            let result = self.read_work(&self.plan_document(plan, active), true)?;
            ensure!(
                spec.digest == result.digest,
                "plan specification changed during validation"
            );
            for task in self.list_tasks(tx, plan)? {
                self.check_task(tx, &task.document, true)?;
            }
            if status == Column::Done {
                ensure!(
                    spec.final_hash == result.final_hash,
                    "plan result changed after completion"
                );
                return Ok(snapshot);
            }
            tx.execute(
                "UPDATE plans SET status='done',final_hash=?1 WHERE id=?2",
                params![result.final_hash, plan],
            )?;
            history(tx, plan, "plan_completed", plan, Some(spec.criteria))?;
            self.snapshot(tx, plan)
        })
    }

    fn check_plan(
        &self,
        tx: &Connection,
        plan: i64,
        draft: i64,
        finished: bool,
    ) -> Result<WorkState> {
        self.check_work(tx, "plans", &self.plan_document(plan, draft), finished)
    }

    fn check_task(&self, tx: &Connection, task: &Document, finished: bool) -> Result<WorkState> {
        self.check_work(tx, "tasks", task, finished)
    }

    fn check_work(
        &self,
        tx: &Connection,
        table: &str,
        document: &Document,
        finished: bool,
    ) -> Result<WorkState> {
        let state = tx.query_row(
            &format!("SELECT spec_hash,final_hash,criteria FROM {table} WHERE id=?1"),
            [document.id],
            |r| {
                Ok(WorkState {
                    digest: r.get(0)?,
                    final_hash: r.get(1)?,
                    criteria: r.get(2)?,
                })
            },
        )?;
        let spec = self.read_work(document, finished)?;
        ensure!(
            spec.digest == state.digest && (!finished || spec.final_hash == state.final_hash),
            "confirmed text changed at {}; preserve your edits elsewhere, restore the confirmed text and replan in a new draft",
            document.path.display()
        );
        Ok(state)
    }
}

fn pending_requests(tx: &Connection, plan: i64) -> Result<Vec<i64>> {
    ids(
        tx,
        "SELECT id FROM requests WHERE plan_id=?1 AND applied_revision IS NULL ORDER BY id",
        plan,
    )
}

fn sorted(values: &[i64]) -> Vec<i64> {
    let mut ordered = values.to_vec();
    ordered.sort_unstable();
    ordered
}

fn execution_ready(tx: &Transaction<'_>, plan: i64) -> Result<()> {
    if pending_count(tx, "requests", plan)? > 0 || pending_count(tx, "questions", plan)? > 0 {
        bail!("resolve questions and reflect pending requests or answers in a confirmed plan");
    }
    ensure!(
        !tx.query_row(
            "SELECT EXISTS(SELECT 1 FROM drafts WHERE plan_id=?1 AND state='open')",
            [plan],
            |r| r.get::<_, bool>(0)
        )?,
        "resolve the open draft before execution"
    );
    Ok(())
}

fn verify(criteria: u32, checked: &[i64]) -> Result<()> {
    ensure!(
        checked.len() == criteria as usize
            && sorted(checked).into_iter().eq(1..=i64::from(criteria)),
        "verify each of the {criteria} acceptance bullets by its one-based position"
    );
    Ok(())
}
