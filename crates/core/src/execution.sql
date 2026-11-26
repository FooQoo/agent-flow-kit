CREATE TABLE task_runs (
    task_id INTEGER NOT NULL REFERENCES tasks(id),
    attempt INTEGER NOT NULL,
    agent TEXT NOT NULL,
    model TEXT,
    PRIMARY KEY (task_id, attempt)
);
CREATE TABLE handoffs (
    task_id INTEGER PRIMARY KEY REFERENCES tasks(id),
    attempt INTEGER NOT NULL,
    note TEXT NOT NULL,
    updated_at INTEGER NOT NULL
);
PRAGMA user_version = 2;
