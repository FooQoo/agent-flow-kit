use std::{env, path::PathBuf, sync::Arc, time::Duration};

use afk_core::Store;
use anyhow::{Context, Result, anyhow, ensure};
use rmcp::{
    RoleServer, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, ContentBlock, Implementation, ServerCapabilities, ServerConfig},
    service::{NotificationContext, Peer, RequestContext},
    tool, tool_handler, tool_router,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::sync::OnceCell;

#[derive(Clone)]
pub(crate) struct Server {
    tool_router: ToolRouter<Self>,
    root: Option<PathBuf>,
    store: Arc<OnceCell<Store>>,
    worker: String,
}

impl Server {
    pub fn new(root: Option<PathBuf>) -> Self {
        Self {
            tool_router: Self::tool_router(),
            root: root
                .or_else(|| setting("AFK_PROJECT_ROOT"))
                .or_else(|| setting("CLAUDE_PROJECT_DIR")),
            store: Arc::new(OnceCell::new()),
            worker: uuid::Uuid::new_v4().to_string(),
        }
    }

    pub async fn initialize_explicit_root(&self) -> Result<()> {
        if let Some(root) = self.root.clone() {
            let store = tokio::task::spawn_blocking(move || Store::initialize(&root)).await??;
            self.store.set(store)?;
        }
        Ok(())
    }

    async fn store(&self, peer: &Peer<RoleServer>, workspace: Option<PathBuf>) -> Result<Store> {
        let workspace = workspace
            .map(|path| {
                ensure!(
                    path.is_absolute(),
                    "workspace must be the absolute project directory"
                );
                safe_workspace(path.canonicalize()?)
            })
            .transpose()?;
        let store = self
            .store
            .get_or_try_init(|| self.initialize_store(peer, workspace.clone()))
            .await?;
        if let Some(workspace) = workspace {
            ensure!(
                store.root() == workspace,
                "AFK is already bound to {}; start a new MCP session for another workspace",
                store.root().display()
            );
        }
        Ok(store.clone())
    }

    // Older MCP clients still expose Roots. Explicit workspace selection works without it.
    #[allow(deprecated)]
    async fn initialize_store(
        &self,
        peer: &Peer<RoleServer>,
        workspace: Option<PathBuf>,
    ) -> Result<Store> {
        let root = match self.root.clone().or(workspace) {
            Some(root) => root,
            None if peer
                .peer_info()
                .is_some_and(|info| info.capabilities.roots.is_some()) =>
            {
                let roots = tokio::time::timeout(Duration::from_millis(800), peer.list_roots())
                    .await
                    .context("MCP Roots timed out; configure AFK_PROJECT_ROOT")?
                    .context("could not query MCP Roots; configure AFK_PROJECT_ROOT")?
                    .roots;
                match roots.as_slice() {
                    [] => implicit_cwd()?,
                    [root] => url::Url::parse(&root.uri)?
                        .to_file_path()
                        .map_err(|()| anyhow!("MCP root must be a local file URI"))?,
                    _ => {
                        let cwd = implicit_cwd()?;
                        ensure!(
                            roots
                                .iter()
                                .filter_map(|root| url::Url::parse(&root.uri).ok())
                                .filter_map(|url| url.to_file_path().ok())
                                .any(|root| root.canonicalize().ok().as_ref() == Some(&cwd)),
                            "multiple MCP roots; configure AFK_PROJECT_ROOT"
                        );
                        cwd
                    }
                }
            }
            None => implicit_cwd()?,
        };
        tokio::task::spawn_blocking(move || Store::initialize(&root)).await?
    }

    async fn run<T, F>(
        &self,
        context: RequestContext<RoleServer>,
        plan_id: Option<i64>,
        workspace: Option<PathBuf>,
        operation: F,
    ) -> CallToolResult
    where
        T: Serialize,
        F: FnOnce(&Store) -> Result<T> + Send + 'static,
    {
        let result = async {
            let store = self.store(&context.peer, workspace).await?;
            tokio::task::spawn_blocking(move || {
                let data = serde_json::to_value(operation(&store)?)?;
                let plan = plan_id.or_else(|| data.get("plan_id").and_then(Value::as_i64));
                let mut value = data;
                if let (Some(id), Some(object)) = (plan, value.as_object_mut()) {
                    object.insert("plan_id".to_owned(), json!(id));
                    object.insert("afk_root".to_owned(), json!(store.root()));
                }
                Ok::<_, anyhow::Error>(value.to_string())
            })
            .await?
        }
        .await;
        match result {
            Ok(value) => CallToolResult::success(vec![ContentBlock::text(value)]),
            Err(error) => CallToolResult::error(vec![ContentBlock::text(format!("{error:#}"))]),
        }
    }
}

fn setting(name: &str) -> Option<PathBuf> {
    env::var_os(name)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

fn implicit_cwd() -> Result<PathBuf> {
    ensure!(
        env::var_os("AFK_NEEDS_WORKSPACE").is_none(),
        "provide the current project directory as workspace to record_request or open_plan"
    );
    safe_workspace(env::current_dir()?.canonicalize()?)
}

fn safe_workspace(cwd: PathBuf) -> Result<PathBuf> {
    ensure!(
        cwd.parent().is_some()
            && [setting("HOME"), setting("USERPROFILE")]
                .into_iter()
                .flatten()
                .all(|home| home.canonicalize().ok().as_ref() != Some(&cwd))
            && [setting("PLUGIN_ROOT"), setting("CLAUDE_PLUGIN_ROOT")]
                .into_iter()
                .flatten()
                .all(|root| !root.canonicalize().is_ok_and(|root| cwd.starts_with(root))),
        "no project directory; configure AFK_PROJECT_ROOT"
    );
    Ok(cwd)
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Intake {
    /// Absolute current project directory if the client does not provide it.
    workspace: Option<PathBuf>,
    /// Existing plan for an additional request; omit for a new plan.
    plan_id: Option<i64>,
    /// Optional opaque retry key. Reuse it only for the same request.
    receipt: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Inspect {
    /// Absolute current project directory if the client does not provide it.
    workspace: Option<PathBuf>,
    plan_id: Option<i64>,
    /// Include active task paths and dependencies for replanning.
    #[serde(default)]
    tasks: bool,
    /// Paginate recent plans, excluding this ID and newer.
    before: Option<i64>,
    /// Surface unreferenced Markdown paths for recovery.
    #[serde(default)]
    check_files: bool,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct PlanChange {
    plan_id: i64,
    change: PlanOperation,
}

#[derive(Deserialize, JsonSchema)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
enum PlanOperation {
    Prepare {
        revision: i64,
        /// One prerequisite list per new task. Use existing positive IDs or -1, -2 etc. for earlier new tasks.
        add: Vec<Vec<i64>>,
        #[serde(default)]
        remove: Vec<i64>,
        replace_draft: Option<i64>,
    },
    Recover {
        draft_id: i64,
    },
    Commit {
        draft_id: i64,
        requests: Vec<i64>,
        answers: Vec<i64>,
        /// True when the human asked for execution.
        #[serde(default)]
        execute: bool,
    },
    /// Send a confirmed Backlog plan after an explicit instruction to start.
    Authorize,
    /// Save a checkpoint for the task owned by this session, without transferring ownership.
    SaveHandoff {
        task_id: i64,
        attempt: i64,
        note: String,
    },
    Finish {
        revision: i64,
        /// One-based numbers for the verified acceptance bullets.
        verified: Vec<i64>,
    },
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct QuestionChange {
    plan_id: i64,
    change: QuestionOperation,
}

#[derive(Deserialize, JsonSchema)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
enum QuestionOperation {
    Prepare {
        task_id: Option<i64>,
        draft_id: Option<i64>,
    },
    Publish {
        question_id: i64,
    },
    Answer {
        question_id: i64,
    },
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct NextTask {
    plan_id: i64,
    /// Model name supplied by the executing agent when known.
    model: Option<String>,
    /// Take over a Doing task only after its previous worker stopped.
    #[serde(default)]
    take_over: bool,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct FinishTask {
    plan_id: i64,
    task_id: i64,
    attempt: i64,
    /// One-based numbers for the verified acceptance bullets.
    verified: Vec<i64>,
}

#[tool_router]
impl Server {
    #[tool(
        description = "Register a new or additional human request. Write its Markdown at the returned path."
    )]
    async fn record_request(
        &self,
        context: RequestContext<RoleServer>,
        Parameters(input): Parameters<Intake>,
    ) -> CallToolResult {
        self.run(
            context,
            input.plan_id,
            input.workspace.clone(),
            move |store| store.record_request(input.plan_id, input.receipt.as_deref()),
        )
        .await
    }

    #[tool(description = "Resume by plan ID, or list recent plans if the ID is omitted.")]
    async fn open_plan(
        &self,
        context: RequestContext<RoleServer>,
        Parameters(input): Parameters<Inspect>,
    ) -> CallToolResult {
        self.run(
            context,
            input.plan_id,
            input.workspace.clone(),
            move |store| {
                let mut value = match input.plan_id {
                    Some(id) => serde_json::to_value(store.inspect(id, input.tasks)?)?,
                    None => json!({"plans": store.recent_plans(input.before.unwrap_or(i64::MAX))?}),
                };
                if input.check_files {
                    value["orphan_documents"] = json!(store.orphan_documents()?);
                }
                Ok(value)
            },
        )
        .await
    }

    #[tool(
        description = "Prepare, recover, confirm, authorize or complete a plan revision, or save a task handoff."
    )]
    async fn update_plan(
        &self,
        context: RequestContext<RoleServer>,
        Parameters(input): Parameters<PlanChange>,
    ) -> CallToolResult {
        let id = input.plan_id;
        let worker = self.worker.clone();
        self.run(context, Some(id), None, move |store| match input.change {
            PlanOperation::Prepare {
                revision,
                add,
                remove,
                replace_draft,
            } => Ok(serde_json::to_value(store.prepare_plan(
                id,
                revision,
                &add,
                &remove,
                replace_draft,
            )?)?),
            PlanOperation::Recover { draft_id } => {
                Ok(serde_json::to_value(store.recover_draft(id, draft_id)?)?)
            }
            PlanOperation::Commit {
                draft_id,
                requests,
                answers,
                execute,
            } => Ok(serde_json::to_value(
                store.commit_plan(id, draft_id, &requests, &answers, execute)?,
            )?),
            PlanOperation::Authorize => Ok(serde_json::to_value(store.authorize_execution(id)?)?),
            PlanOperation::SaveHandoff {
                task_id,
                attempt,
                note,
            } => {
                store.save_handoff(id, task_id, &worker, attempt, &note)?;
                Ok(json!({"task_id": task_id, "attempt": attempt, "status": "saved"}))
            }
            PlanOperation::Finish { revision, verified } => Ok(serde_json::to_value(
                store.finish_plan(id, revision, &verified)?,
            )?),
        })
        .await
    }

    #[tool(
        description = "Prepare a question path, publish the written prompt, or seal the written human answer."
    )]
    async fn update_question(
        &self,
        context: RequestContext<RoleServer>,
        Parameters(input): Parameters<QuestionChange>,
    ) -> CallToolResult {
        let id = input.plan_id;
        self.run(context, Some(id), None, move |store| match input.change {
            QuestionOperation::Prepare { task_id, draft_id } => Ok(serde_json::to_value(
                store.prepare_question(id, task_id, draft_id)?,
            )?),
            QuestionOperation::Publish { question_id } => {
                store.publish_question(id, question_id)?;
                Ok(json!({"id": question_id, "status": "published"}))
            }
            QuestionOperation::Answer { question_id } => {
                store.answer_question(id, question_id)?;
                Ok(json!({"id": question_id, "status": "answered"}))
            }
        })
        .await
    }

    #[tool(
        description = "Select and start the next ready task, or return the current task's attempt."
    )]
    async fn start_next(
        &self,
        context: RequestContext<RoleServer>,
        Parameters(input): Parameters<NextTask>,
    ) -> CallToolResult {
        let worker = self.worker.clone();
        let agent = context
            .peer
            .peer_info()
            .map(|info| info.client_info.name.clone());
        self.run(context, Some(input.plan_id), None, move |store| {
            store.start_next_as(
                input.plan_id,
                &worker,
                input.take_over,
                agent.as_deref(),
                input.model.as_deref(),
            )
        })
        .await
    }

    #[tool(
        description = "Complete a task after verifying its acceptance bullets and writing its result."
    )]
    async fn finish_task(
        &self,
        context: RequestContext<RoleServer>,
        Parameters(input): Parameters<FinishTask>,
    ) -> CallToolResult {
        let worker = self.worker.clone();
        self.run(context, Some(input.plan_id), None, move |store| {
            store.finish_task(
                input.plan_id,
                input.task_id,
                &worker,
                input.attempt,
                &input.verified,
            )
        })
        .await
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for Server {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("agent-flow-kit", env!("CARGO_PKG_VERSION")))
            .with_instructions("Use AFK when the user invokes its workflow. Read and write Markdown with standard file tools.")
    }

    async fn on_initialized(&self, context: NotificationContext<RoleServer>) {
        let server = self.clone();
        tokio::spawn(async move {
            if let Err(error) = server.store(&context.peer, None).await
                && env::var_os("AFK_NEEDS_WORKSPACE").is_none()
            {
                eprintln!("AFK project initialization: {error:#}");
            }
        });
    }
}
