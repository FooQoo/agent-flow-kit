CREATE TABLE plans (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    status TEXT NOT NULL DEFAULT 'backlog' CHECK(status IN ('backlog','todo','doing','done')),
    revision INTEGER NOT NULL DEFAULT 0,
    active_draft INTEGER REFERENCES drafts(id),
    title TEXT,
    spec_hash TEXT,
    final_hash TEXT,
    criteria INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE drafts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    plan_id INTEGER NOT NULL REFERENCES plans(id),
    base_revision INTEGER NOT NULL,
    path TEXT GENERATED ALWAYS AS (printf('plans/%03lld-r%03lld.md', plan_id, id)) STORED,
    state TEXT NOT NULL DEFAULT 'open' CHECK(state IN ('open','accepted','discarded'))
);
CREATE UNIQUE INDEX one_draft ON drafts(plan_id) WHERE state='open';
CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    plan_id INTEGER NOT NULL REFERENCES plans(id),
    draft_id INTEGER NOT NULL REFERENCES drafts(id),
    path TEXT GENERATED ALWAYS AS (printf('tasks/%03lld.md', id)) STORED,
    lifecycle TEXT NOT NULL DEFAULT 'draft' CHECK(lifecycle IN ('draft','active','retired')),
    status TEXT NOT NULL DEFAULT 'backlog' CHECK(status IN ('backlog','todo','doing','done')),
    title TEXT,
    spec_hash TEXT,
    final_hash TEXT,
    criteria INTEGER NOT NULL DEFAULT 0,
    attempt INTEGER NOT NULL DEFAULT 0,
    owner TEXT
);
CREATE INDEX task_plan ON tasks(plan_id, lifecycle, status, id);
CREATE UNIQUE INDEX one_running ON tasks(plan_id) WHERE lifecycle='active' AND status='doing';
CREATE TABLE dependencies (
    task_id INTEGER NOT NULL REFERENCES tasks(id),
    prerequisite INTEGER NOT NULL REFERENCES tasks(id),
    PRIMARY KEY(task_id, prerequisite),
    CHECK(task_id > prerequisite)
);
CREATE TABLE draft_removals (
    draft_id INTEGER NOT NULL REFERENCES drafts(id),
    task_id INTEGER NOT NULL REFERENCES tasks(id),
    PRIMARY KEY(draft_id, task_id)
);
CREATE TABLE requests (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    plan_id INTEGER NOT NULL REFERENCES plans(id),
    receipt TEXT UNIQUE,
    path TEXT GENERATED ALWAYS AS (printf('requests/%03lld.md', id)) STORED,
    source_plan INTEGER,
    applied_revision INTEGER,
    digest TEXT
);
CREATE INDEX request_plan ON requests(plan_id, applied_revision, id);
CREATE TABLE questions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    plan_id INTEGER NOT NULL REFERENCES plans(id),
    task_id INTEGER REFERENCES tasks(id),
    draft_id INTEGER REFERENCES drafts(id),
    path TEXT GENERATED ALWAYS AS (printf('questions/%03lld.md', id)) STORED,
    title TEXT,
    prompt_digest TEXT,
    answer_digest TEXT,
    applied_revision INTEGER
);
CREATE INDEX question_plan ON questions(plan_id, applied_revision, id);
CREATE TABLE history (
    id INTEGER PRIMARY KEY,
    plan_id INTEGER NOT NULL REFERENCES plans(id),
    event TEXT NOT NULL,
    entity_id INTEGER NOT NULL,
    acceptance_checked INTEGER,
    at INTEGER NOT NULL DEFAULT (unixepoch())
);
PRAGMA user_version = 1;
