//! Route client events without writing to the workflow database.

use std::{
    env, fs,
    io::{self, Read, Write},
    path::{Path, PathBuf},
    time::Duration,
};

use afk_core::{NextAction, Progress, Store};
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
            let progress =
                Store::open_existing(&marker.route.afk_root)?.progress(marker.route.plan_id)?;
            Ok(Some(report(&progress)))
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

fn report(progress: &Progress) -> String {
    let mut report = if progress.revision == 0 {
        format!(
            "AFK · {} [{}] · task count pending",
            progress.title,
            progress.status.label()
        )
    } else {
        format!(
            "AFK · {} [{}] · {}/{} complete",
            progress.title,
            progress.status.label(),
            progress.done,
            progress.total
        )
    };
    if progress.total > progress.done {
        let remaining = progress.total - progress.done - progress.doing;
        report.push_str(&format!(
            " · {} in progress / {remaining} unstarted",
            progress.doing
        ));
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
    report.push('\n');
    report.push_str(&next);
    report
}
