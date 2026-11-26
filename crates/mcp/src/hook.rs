//! Route client events without writing to the workflow database.

use std::{
    env, fs,
    io::{self, Read, Write},
    path::{Path, PathBuf},
    time::Duration,
};

use afk_core::{Column, NextAction, Progress, Store, TaskSummary};
use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

#[derive(Deserialize)]
struct Event {
    hook_event_name: String,
    session_id: String,
    cwd: PathBuf,
    turn_id: Option<String>,
    #[serde(default)]
    stop_hook_active: bool,
    tool_name: Option<String>,
    tool_response: Option<Value>,
}

#[derive(Serialize, Deserialize)]
struct Route {
    plan_id: i64,
    afk_root: PathBuf,
}

#[derive(Serialize, Deserialize)]
struct Marker {
    cwd: PathBuf,
    route: Route,
}

pub fn run() {
    // The client also imposes a one-second command timeout.
    let outcome = (|| -> Result<Option<String>> {
        let mut input = Vec::new();
        io::stdin().take(1024 * 1024 + 1).read_to_end(&mut input)?;
        ensure!(input.len() <= 1024 * 1024, "hook payload too large");
        let event: Event = serde_json::from_slice(&input)?;
        let base = ["AFK_HOOK_DIR", "PLUGIN_DATA", "CLAUDE_PLUGIN_DATA"]
            .into_iter()
            .find_map(env::var_os)
            .filter(|value| !value.is_empty())
            .context("hook data directory unavailable")?;
        handle(&PathBuf::from(base).join("report"), event)
    })();
    if let Ok(Some(message)) = outcome {
        println!("{}", json!({"systemMessage": message}));
    }
}

fn handle(base: &Path, event: Event) -> Result<Option<String>> {
    ensure!(!event.session_id.is_empty(), "empty hook session ID");
    let key = serde_json::to_vec(&(&event.session_id, &event.turn_id))?;
    let slot = base.join(format!("{:x}.json", Sha256::digest(key)));
    match event.hook_event_name.as_str() {
        "UserPromptSubmit" if event.turn_id.is_none() => {
            let _ = fs::remove_file(slot);
            Ok(None)
        }
        "PostToolUse" => {
            let name = event.tool_name.as_deref().unwrap_or("");
            if !is_afk_tool(name) {
                return Ok(None);
            }
            let route = event
                .tool_response
                .as_ref()
                .and_then(|value| route(value, 0));
            if let Some(route) = route {
                ensure!(
                    route.plan_id > 0 && route.afk_root.is_absolute(),
                    "invalid AFK route"
                );
                fs::create_dir_all(base)?;
                let mut temp = tempfile::NamedTempFile::new_in(base)?;
                let marker = Marker {
                    cwd: event.cwd.canonicalize()?,
                    route,
                };
                serde_json::to_writer(temp.as_file_mut(), &marker)?;
                temp.flush()?;
                temp.persist(slot)?;
            }
            Ok(None)
        }
        "Stop" if !event.stop_hook_active && slot.is_file() => {
            // Claim only this turn's marker. Concurrent Stop processes cannot both rename it.
            let claim = tempfile::NamedTempFile::new_in(base)?.into_temp_path();
            fs::rename(slot, &claim)?;
            ensure!(
                fs::metadata(&claim)?.modified()?.elapsed()? < Duration::from_secs(24 * 3600),
                "expired report marker"
            );
            let file = fs::File::open(&claim)?;
            let marker: Marker = serde_json::from_reader(file.take(8192))?;
            ensure!(event.cwd.canonicalize()? == marker.cwd, "hook cwd changed");
            let store = Store::open_existing(&marker.route.afk_root)?;
            let progress = store.progress(marker.route.plan_id)?;
            let tasks = store.task_summaries(marker.route.plan_id)?;
            Ok(Some(report(&progress, &tasks)))
        }
        _ => Ok(None),
    }
}

fn is_afk_tool(name: &str) -> bool {
    name.strip_prefix("mcp__")
        .and_then(|name| name.rsplit_once("__"))
        .is_some_and(|(server, tool)| {
            (server == "afk" || server.ends_with("_afk") || server.ends_with("-afk"))
                && matches!(
                    tool,
                    "record_request"
                        | "open_plan"
                        | "update_plan"
                        | "update_question"
                        | "start_next"
                        | "finish_task"
                )
        })
}

fn route(value: &Value, depth: u8) -> Option<Route> {
    if depth > 6 {
        return None;
    }
    match value {
        Value::String(text) => route(&serde_json::from_str(text).ok()?, depth + 1),
        Value::Array(items) => items.iter().find_map(|item| route(item, depth + 1)),
        Value::Object(map) if !map.get("isError").and_then(Value::as_bool).unwrap_or(false) => {
            if let (Some(plan_id), Some(afk_root)) = (
                map.get("plan_id").and_then(Value::as_i64),
                map.get("afk_root").and_then(Value::as_str),
            ) {
                return Some(Route {
                    plan_id,
                    afk_root: afk_root.into(),
                });
            }
            ["structuredContent", "content", "text", "result", "output"]
                .into_iter()
                .find_map(|key| map.get(key).and_then(|value| route(value, depth + 1)))
        }
        _ => None,
    }
}

const BAR_WIDTH: usize = 10;
const MAX_TASK_LINES: usize = 6;

fn progress_bar(done: u32, total: u32) -> (String, u32) {
    let percent = done
        .saturating_mul(100)
        .saturating_add(total / 2)
        .checked_div(total)
        .unwrap_or(0)
        .min(100);
    let filled = (done as usize)
        .saturating_mul(BAR_WIDTH)
        .saturating_add((total as usize) / 2)
        .checked_div(total as usize)
        .unwrap_or(0)
        .min(BAR_WIDTH);
    let mut bar = String::with_capacity(BAR_WIDTH * 3);
    for _ in 0..filled {
        bar.push('█');
    }
    for _ in filled..BAR_WIDTH {
        bar.push('░');
    }
    (bar, percent)
}

fn task_symbol(status: Column) -> char {
    match status {
        Column::Done => '✓',
        Column::Doing => '●',
        Column::Todo | Column::Backlog => '○',
    }
}

fn report(progress: &Progress, tasks: &[TaskSummary]) -> String {
    let mut lines = Vec::new();
    if progress.revision == 0 {
        lines.push(format!(
            "[AFK] {} [{}] · task count pending",
            progress.title,
            progress.status.label()
        ));
    } else {
        let (bar, percent) = progress_bar(progress.done, progress.total);
        lines.push(format!(
            "[AFK] {} [{}] · {bar} {}/{} complete ({percent}%)",
            progress.title,
            progress.status.label(),
            progress.done,
            progress.total
        ));
    }

    if !tasks.is_empty() {
        if tasks.len() <= MAX_TASK_LINES {
            for task in tasks {
                lines.push(format!("{} {}", task_symbol(task.status), task.title));
            }
        } else {
            let mut done_count = 0usize;
            let mut remaining = Vec::new();
            for task in tasks {
                if task.status == Column::Done {
                    done_count += 1;
                } else {
                    remaining.push(task);
                }
            }
            if done_count > 0 {
                lines.push(format!("✓ {done_count} completed"));
            }
            let budget = if done_count > 0 {
                MAX_TASK_LINES - 1
            } else {
                MAX_TASK_LINES
            };
            let shown = remaining.len().min(budget);
            for task in &remaining[..shown] {
                lines.push(format!("{} {}", task_symbol(task.status), task.title));
            }
            let hidden = remaining.len().saturating_sub(shown);
            if hidden > 0 {
                lines.push(format!("… +{hidden} more"));
            }
        }
    }

    let next = match &progress.next {
        NextAction::PrepareQuestion { id } => format!("Prepare question #{id}"),
        NextAction::WaitForAnswer { title, count, .. } => {
            format!("Waiting for {count} answer(s) · {title}")
        }
        NextAction::Replan { requests, answers } => {
            format!("Update the plan with {requests} request(s) and {answers} answer(s)")
        }
        NextAction::ConfirmDraft { .. } => "Confirm the prepared revision".to_owned(),
        NextAction::PreparePlan => "Prepare the plan".to_owned(),
        NextAction::AwaitStart => "Waiting for an instruction to start".to_owned(),
        NextAction::ResumeTask { title, .. } => format!("In progress · {title}"),
        NextAction::StartTask { title, .. } => format!("Next · {title}"),
        NextAction::VerifyPlan => "Verify the plan's acceptance conditions".to_owned(),
        NextAction::Complete => "Plan complete".to_owned(),
    };
    lines.push(next);
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_progress(done: u32, total: u32, status: Column, next: NextAction) -> Progress {
        Progress {
            plan_id: 1,
            title: "Improve search".to_owned(),
            revision: 1,
            status,
            total,
            done,
            doing: u32::from(status == Column::Doing),
            next,
        }
    }

    #[test]
    fn formats_bar_and_task_list() {
        let progress = sample_progress(
            3,
            5,
            Column::Doing,
            NextAction::ResumeTask {
                id: 4,
                title: "Add tests".to_owned(),
            },
        );
        let tasks = vec![
            TaskSummary {
                id: 1,
                title: "Audit".to_owned(),
                status: Column::Done,
            },
            TaskSummary {
                id: 2,
                title: "Spec".to_owned(),
                status: Column::Done,
            },
            TaskSummary {
                id: 3,
                title: "Implement".to_owned(),
                status: Column::Done,
            },
            TaskSummary {
                id: 4,
                title: "Add tests".to_owned(),
                status: Column::Doing,
            },
            TaskSummary {
                id: 5,
                title: "Verify".to_owned(),
                status: Column::Todo,
            },
        ];
        let out = report(&progress, &tasks);
        assert!(out.starts_with("[AFK] Improve search [Doing] · ██████░░░░ 3/5 complete (60%)\n"));
        assert!(out.contains("✓ Audit\n✓ Spec\n✓ Implement\n● Add tests\n○ Verify\n"));
        assert!(out.ends_with("In progress · Add tests"));
    }

    #[test]
    fn collapses_completed_tasks_when_list_is_long() {
        let progress = sample_progress(
            4,
            8,
            Column::Doing,
            NextAction::ResumeTask {
                id: 5,
                title: "Task 5".to_owned(),
            },
        );
        let tasks: Vec<TaskSummary> = (1..=8)
            .map(|i| TaskSummary {
                id: i,
                title: format!("Task {i}"),
                status: if i <= 4 {
                    Column::Done
                } else if i == 5 {
                    Column::Doing
                } else {
                    Column::Todo
                },
            })
            .collect();
        let out = report(&progress, &tasks);
        assert!(out.contains("█████░░░░░ 4/8 complete (50%)"));
        assert!(out.contains("✓ 4 completed\n● Task 5\n○ Task 6\n○ Task 7\n○ Task 8\n"));
    }

    #[test]
    fn keeps_pending_message_before_first_revision() {
        let progress = Progress {
            plan_id: 1,
            title: "Plan #1".to_owned(),
            revision: 0,
            status: Column::Backlog,
            total: 0,
            done: 0,
            doing: 0,
            next: NextAction::PreparePlan,
        };
        let out = report(&progress, &[]);
        assert_eq!(
            out,
            "[AFK] Plan #1 [Backlog] · task count pending\nPrepare the plan"
        );
    }
}
