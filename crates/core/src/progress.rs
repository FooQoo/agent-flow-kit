use anyhow::Result;
use rusqlite::{Connection, OptionalExtension};
use serde::Serialize;

use crate::{Column, Store, plan_exists};

#[derive(Debug, Serialize)]
pub struct Progress {
    pub plan_id: i64,
    pub title: String,
    pub revision: i64,
    pub status: Column,
    pub total: u32,
    pub done: u32,
    pub doing: u32,
    pub next: NextAction,
}

#[derive(Debug, Serialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum NextAction {
    PrepareQuestion { id: i64 },
    WaitForAnswer { id: i64, title: String, count: u32 },
    Replan { requests: u32, answers: u32 },
    ConfirmDraft { id: i64 },
    PreparePlan,
    AwaitStart,
    ResumeTask { id: i64, title: String },
    StartTask { id: i64, title: String },
    VerifyPlan,
    Complete,
}

impl Store {
    pub fn progress(&self, id: i64) -> Result<Progress> {
        self.read(|tx| self.snapshot(tx, id))
    }

    pub fn recent_plans(&self, before: i64) -> Result<Vec<Progress>> {
        self.read(|tx| {
            crate::ids(
                tx,
                "SELECT id FROM plans WHERE id<?1 ORDER BY id DESC LIMIT 10",
                before,
            )?
            .into_iter()
            .map(|id| self.snapshot(tx, id))
            .collect()
        })
    }

    pub(crate) fn snapshot(&self, tx: &Connection, id: i64) -> Result<Progress> {
        let status = plan_exists(tx, id)?;
        let (title, revision): (Option<String>, i64) =
            tx.query_row("SELECT title,revision FROM plans WHERE id=?1", [id], |r| {
                Ok((r.get(0)?, r.get(1)?))
            })?;
        let (total, done, doing): (u32, u32, u32) = tx.query_row(
            "SELECT count(*),count(*) FILTER (WHERE status='done'),count(*) FILTER (WHERE status='doing')
             FROM tasks WHERE plan_id=?1 AND lifecycle='active'", [id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;
        let waiting = tx
            .query_row(
                "SELECT id,title,count(*) OVER () FROM questions WHERE plan_id=?1
            AND prompt_digest IS NOT NULL AND answer_digest IS NULL ORDER BY id LIMIT 1",
                [id],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .optional()?;
        let draft_question = tx.query_row("SELECT id FROM questions WHERE plan_id=?1 AND prompt_digest IS NULL ORDER BY id LIMIT 1", [id],
            |r| r.get(0)).optional()?;
        let draft = tx
            .query_row(
                "SELECT id FROM drafts WHERE plan_id=?1 AND state='open'",
                [id],
                |r| r.get(0),
            )
            .optional()?;
        let requests = crate::pending_count(tx, "requests", id)?;
        let answers = crate::pending_count(tx, "questions", id)?;
        let next = if let Some((id, title, count)) = waiting {
            NextAction::WaitForAnswer { id, title, count }
        } else if let Some(id) = draft_question {
            NextAction::PrepareQuestion { id }
        } else if requests > 0 || answers > 0 {
            NextAction::Replan { requests, answers }
        } else if let Some(id) = draft {
            NextAction::ConfirmDraft { id }
        } else {
            match status {
                Column::Done => NextAction::Complete,
                Column::Backlog if revision == 0 => NextAction::PreparePlan,
                Column::Backlog => NextAction::AwaitStart,
                Column::Todo | Column::Doing => match candidate(tx, id)? {
                    Some((id, title, Column::Doing)) => NextAction::ResumeTask { id, title },
                    Some((id, title, _)) => NextAction::StartTask { id, title },
                    None => NextAction::VerifyPlan,
                },
            }
        };
        Ok(Progress {
            plan_id: id,
            title: title.unwrap_or_else(|| format!("Plan #{id}")),
            revision,
            status,
            total,
            done,
            doing,
            next,
        })
    }
}

// Resume Doing before picking Todo, then gate Todo on its prerequisites.
pub(crate) fn candidate(tx: &Connection, plan: i64) -> Result<Option<(i64, String, Column)>> {
    Ok(tx
        .query_row(
            "SELECT id,title,status='doing' FROM tasks WHERE plan_id=?1 AND lifecycle='active' AND
        (status='doing' OR (status='todo' AND NOT EXISTS (
            SELECT 1 FROM dependencies JOIN tasks p ON p.id=prerequisite
            WHERE task_id=tasks.id AND (p.status<>'done' OR p.lifecycle<>'active'))))
        ORDER BY (status='doing') DESC,id LIMIT 1",
            [plan],
            |r| {
                let running = r.get::<_, bool>(2)?;
                Ok((
                    r.get(0)?,
                    r.get(1)?,
                    if running { Column::Doing } else { Column::Todo },
                ))
            },
        )
        .optional()?)
}
