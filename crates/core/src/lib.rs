mod column;
mod documents;
mod plain_title;
mod plan;
mod progress;

use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::{Context, Result, bail, ensure};
use rusqlite::{
    Connection, OpenFlags, OptionalExtension, Transaction, TransactionBehavior, params,
};
use serde::Serialize;

pub use column::Column;
pub use plan::{Draft, PlanView, TaskRun};
pub use progress::{NextAction, Progress};

const SCHEMA_VERSION: i64 = 2;

#[derive(Clone, Debug)]
pub struct Store {
    base: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
pub struct Document {
    pub id: i64,
    pub path: PathBuf,
}

#[derive(Debug, Serialize)]
pub struct Intake {
    pub plan_id: i64,
    pub request: Document,
}

impl Store {
    pub fn initialize(root: &Path) -> Result<Self> {
        let root = root.canonicalize().context("resolve project root")?;
        ensure!(root.is_dir(), "project root must be a directory");
        let base = root.join(".afk");
        create_directory(&base)?;
        for name in ["plans", "tasks", "requests", "questions"] {
            create_directory(&base.join(name))?;
        }
        let store = Self { base };
        let mut connection =
            store.connect(OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        match schema_version(&tx)? {
            0 => {
                for name in ["plans", "tasks", "requests", "questions"] {
                    ensure!(
                        fs::read_dir(store.base.join(name))?.next().is_none(),
                        "AFK database is uninitialized but Markdown exists; restore afk.db or move the existing .afk directory aside"
                    );
                }
                tx.execute_batch(include_str!("schema.sql"))?;
                tx.execute_batch(include_str!("execution.sql"))?;
            }
            1 => tx.execute_batch(include_str!("execution.sql"))?,
            SCHEMA_VERSION => {}
            version => bail!("unsupported AFK database version {version}"),
        }
        tx.commit()?;
        Ok(store)
    }

    pub fn open_existing(root: &Path) -> Result<Self> {
        let base = root.canonicalize()?.join(".afk");
        check_path(&base, true)?;
        let store = Self { base };
        ensure!(
            matches!(
                schema_version(&store.connect(OpenFlags::SQLITE_OPEN_READ_ONLY)?)?,
                1 | SCHEMA_VERSION
            ),
            "unsupported AFK database version"
        );
        Ok(store)
    }

    pub fn root(&self) -> &Path {
        self.base.parent().expect(".afk has a parent")
    }

    fn connect(&self, flags: OpenFlags) -> Result<Connection> {
        check_path(&self.base, true)?;
        for name in ["afk.db", "afk.db-journal", "afk.db-wal", "afk.db-shm"] {
            let path = self.base.join(name);
            match fs::symlink_metadata(&path) {
                Ok(meta) => ensure!(
                    meta.is_file() && !is_link(&meta),
                    "unsafe AFK database path"
                ),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(error.into()),
            }
        }
        let connection = Connection::open_with_flags(
            self.base.join("afk.db"),
            flags | OpenFlags::SQLITE_OPEN_NOFOLLOW,
        )?;
        connection.busy_timeout(if flags.contains(OpenFlags::SQLITE_OPEN_READ_ONLY) {
            Duration::from_millis(100)
        } else {
            Duration::from_secs(3)
        })?;
        connection.pragma_update(None, "foreign_keys", true)?;
        Ok(connection)
    }

    fn write<T>(&self, action: impl FnOnce(&Transaction<'_>) -> Result<T>) -> Result<T> {
        let mut connection = self.connect(OpenFlags::SQLITE_OPEN_READ_WRITE)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let result = action(&tx)?;
        tx.commit()?;
        Ok(result)
    }

    fn read<T>(&self, action: impl FnOnce(&Transaction<'_>) -> Result<T>) -> Result<T> {
        let mut connection = self.connect(OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        let tx = connection.transaction()?;
        let result = action(&tx)?;
        tx.commit()?;
        Ok(result)
    }

    fn document(&self, collection: &str, id: i64) -> Document {
        Document {
            id,
            path: self.base.join(collection).join(format!("{id:03}.md")),
        }
    }

    /// Identify unreferenced Markdown, including files restored from a mismatched backup.
    pub fn orphan_documents(&self) -> Result<Vec<PathBuf>> {
        self.read(|tx| {
            let mut query = tx.prepare(
                "SELECT path FROM drafts UNION ALL SELECT path FROM tasks
                UNION ALL SELECT path FROM requests UNION ALL SELECT path FROM questions",
            )?;
            let known = query
                .query_map([], |row| row.get::<_, String>(0))?
                .collect::<rusqlite::Result<HashSet<_>>>()?;
            let mut orphans = Vec::new();
            for collection in ["plans", "tasks", "requests", "questions"] {
                let directory = self.base.join(collection);
                check_path(&directory, true)?;
                for entry in fs::read_dir(directory)? {
                    let entry = entry?;
                    if entry
                        .path()
                        .extension()
                        .is_some_and(|extension| extension == "md")
                    {
                        let key = format!("{collection}/{}", entry.file_name().to_string_lossy());
                        if !known.contains(&key) {
                            orphans.push(entry.path());
                        }
                    }
                }
            }
            orphans.sort();
            orphans.truncate(32);
            Ok(orphans)
        })
    }

    pub fn record_request(
        &self,
        source_plan: Option<i64>,
        receipt: Option<&str>,
    ) -> Result<Intake> {
        if let Some(key) = receipt {
            ensure!(
                !key.is_empty() && key.len() <= 100,
                "receipt must be 1–100 bytes"
            );
        }
        let (plan_id, id, pending) = self.write(|tx| {
            if let Some(key) = receipt {
                let existing = tx.query_row(
                    "SELECT plan_id,id,source_plan,applied_revision IS NULL FROM requests WHERE receipt=?1",
                    [key], |r| Ok((r.get(0)?, r.get(1)?, r.get::<_, Option<i64>>(2)?, r.get(3)?)),
                ).optional()?;
                if let Some((plan_id, id, original, pending)) = existing {
                    ensure!(source_plan == original, "receipt already belongs to another request");
                    return Ok((plan_id, id, pending));
                }
            }
            let plan_id = match source_plan {
                Some(id) => { plan_exists(tx, id)?; id }
                None => {
                    tx.execute("INSERT INTO plans DEFAULT VALUES", [])?;
                    tx.last_insert_rowid()
                }
            };
            tx.execute("INSERT INTO requests(plan_id,receipt,source_plan) VALUES(?1,?2,?3)", params![plan_id, receipt, source_plan])?;
            let id = tx.last_insert_rowid();
            self.require_unowned_path_available(&self.document("requests", id))?;
            history(tx, plan_id, "request_received", id, None)?;
            Ok((plan_id, id, true))
        })?;
        let request = self.document("requests", id);
        if pending {
            self.prepare_file(&request, documents::REQUEST_TEMPLATE)?;
        }
        Ok(Intake { plan_id, request })
    }

    pub fn prepare_question(
        &self,
        plan_id: i64,
        task_id: Option<i64>,
        draft_id: Option<i64>,
    ) -> Result<Document> {
        let id = self.write(|tx| {
            let status = plan_exists(tx, plan_id)?;
            ensure!(status != Column::Done || pending_count(tx, "requests", plan_id)? > 0,
                "register a new request before asking about a completed plan");
            ensure!(task_id.is_none() || draft_id.is_none(), "scope a question to a task or a draft");
            if let Some(task) = task_id {
                ensure!(tx.query_row("SELECT EXISTS(SELECT 1 FROM tasks WHERE id=?1 AND plan_id=?2 AND lifecycle='active' AND status<>'done')",
                    params![task, plan_id], |r| r.get::<_, bool>(0))?, "question requires an active unfinished task");
            }
            if let Some(draft) = draft_id {
                ensure!(tx.query_row("SELECT EXISTS(SELECT 1 FROM drafts WHERE id=?1 AND plan_id=?2 AND state='open')",
                    params![draft, plan_id], |r| r.get::<_, bool>(0))?, "question requires an open draft");
            }
            tx.execute("INSERT INTO questions(plan_id,task_id,draft_id) VALUES(?1,?2,?3)", params![plan_id, task_id, draft_id])?;
            let id = tx.last_insert_rowid();
            self.require_unowned_path_available(&self.document("questions", id))?;
            history(tx, plan_id, "question_prepared", id, None)?;
            Ok(id)
        })?;
        let doc = self.document("questions", id);
        self.prepare_file(&doc, documents::QUESTION_TEMPLATE)?;
        Ok(doc)
    }

    pub fn publish_question(&self, plan_id: i64, id: i64) -> Result<()> {
        self.write(|tx| {
            let (_, existing) = question(tx, plan_id, id)?;
            let prompt = self.read_question(id, false)?;
            ensure!(
                existing
                    .as_ref()
                    .is_none_or(|digest| digest == &prompt.digest),
                "question prompt changed after publication"
            );
            if existing.is_none() {
                tx.execute(
                    "UPDATE questions SET title=?1,prompt_digest=?2 WHERE id=?3",
                    params![prompt.title, prompt.digest, id],
                )?;
                history(tx, plan_id, "question_asked", id, None)?;
            }
            Ok(())
        })
    }

    pub fn answer_question(&self, plan_id: i64, id: i64) -> Result<()> {
        self.write(|tx| {
            let (answer, prompt) = question(tx, plan_id, id)?;
            let doc = self.read_question(id, true)?;
            ensure!(
                prompt.as_deref() == Some(&doc.digest),
                "publish the question first and preserve its prompt"
            );
            let digest = doc.final_hash.expect("answer required");
            ensure!(
                answer.as_ref().is_none_or(|previous| previous == &digest),
                "record a new question to revise a sealed answer"
            );
            if answer.is_none() {
                tx.execute(
                    "UPDATE questions SET answer_digest=?1 WHERE id=?2",
                    params![digest, id],
                )?;
                history(tx, plan_id, "question_answered", id, None)?;
            }
            Ok(())
        })
    }
}

fn question(tx: &Connection, plan_id: i64, id: i64) -> Result<(Option<String>, Option<String>)> {
    tx.query_row(
        "SELECT answer_digest,prompt_digest FROM questions WHERE id=?1 AND plan_id=?2",
        params![id, plan_id],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )
    .optional()?
    .context("question does not belong to this plan")
}

fn check_path(path: &Path, directory: bool) -> Result<()> {
    let meta = fs::symlink_metadata(path)?;
    ensure!(
        !is_link(&meta)
            && if directory {
                meta.is_dir()
            } else {
                meta.is_file()
            },
        "unsafe AFK path {}",
        path.display()
    );
    Ok(())
}

fn is_link(meta: &fs::Metadata) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        meta.file_attributes() & 0x400 != 0
    }
    #[cfg(not(windows))]
    meta.file_type().is_symlink()
}

fn create_directory(path: &Path) -> Result<()> {
    let mut builder = fs::DirBuilder::new();
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    match builder.create(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => check_path(path, true),
        Err(error) => Err(error.into()),
    }
}

fn schema_version(connection: &Connection) -> Result<i64> {
    Ok(connection.pragma_query_value(None, "user_version", |r| r.get(0))?)
}

fn plan_exists(connection: &Connection, id: i64) -> Result<Column> {
    let state: String = connection
        .query_row("SELECT status FROM plans WHERE id=?1", [id], |r| r.get(0))
        .optional()?
        .context("unknown plan; supply a plan ID from AFK")?;
    state
        .parse()
        .map_err(|()| anyhow::anyhow!("invalid stored plan state"))
}

fn history(
    tx: &Transaction<'_>,
    plan: i64,
    event: &str,
    entity: i64,
    checked: Option<u32>,
) -> Result<()> {
    tx.execute(
        "INSERT INTO history(plan_id,event,entity_id,acceptance_checked) VALUES(?1,?2,?3,?4)",
        params![plan, event, entity, checked],
    )?;
    Ok(())
}

fn ids(connection: &Connection, sql: &str, id: i64) -> Result<Vec<i64>> {
    let mut statement = connection.prepare_cached(sql)?;
    Ok(statement
        .query_map([id], |r| r.get(0))?
        .collect::<rusqlite::Result<_>>()?)
}

fn pending_count(connection: &Connection, table: &str, plan: i64) -> Result<u32> {
    Ok(connection.query_row(
        &format!("SELECT count(*) FROM {table} WHERE plan_id=?1 AND applied_revision IS NULL"),
        [plan],
        |r| r.get(0),
    )?)
}
