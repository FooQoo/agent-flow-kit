# AFK internals

## Workflow state

Agent Project’s Backlog, Todo, Doing and Done conventions carry over. Retained tasks keep their original order by ID. A new task can depend on an earlier task in its draft, or on a retained task in the plan. AFK selects Doing before eligible Todo and runs one task per plan. An interrupted Doing task can be claimed by a new MCP process after its former worker stops. Attempts protect SQLite updates from stale completions.

A confirmed Backlog plan waits for a human instruction to start. An execution request can go directly to Todo. Pending requests, open drafts, and questions awaiting publication, answer or reflection block starts and completions. Replanning atomically replaces the plan reference, retires superseded unfinished tasks and returns retained Doing work to Todo. A new request can reopen a completed plan. Cancellation of an entire plan has no separate state in this revision.

## Markdown and SQLite

`.afk/afk.db` contains states, relationships, revisions, check counts, fingerprints and events. Markdown contains the request, work, questions, answers and result text. Each revision has its own plan path. Task and question IDs are stable. Paths are relative to `.afk` and are surfaced as absolute paths while the project is open. To move or back up a project, close AFK processes and copy the whole `.afk` directory.

A draft reserves IDs before creating its empty files. Use `open_plan` and `update_plan` with `recover` if a process stopped between those steps. An existing draft can be superseded by supplying its exact ID to `prepare`. Superseded files are preserved. Use `open_plan` with `check_files: true` to list unreferenced Markdown paths. When a task is revised, use a new task document and explicitly retire the old task. When Markdown has been edited after confirmation, preserve the changed text elsewhere, restore the last confirmed text, then prepare a new revision. The fingerprints allow appending a result or an answer at its designated heading. SQLite serializes writers. Standard file edits are outside SQLite transactions and concurrent editors must coordinate on an open draft.

Both plan and task use `## Acceptance` with ordered bullet lines. Completion verifies that the provided one-based positions cover every bullet, requirements remained intact, and a nonempty `## Result` exists. The agent checks the substantive evidence with its normal tools. The database stores a task’s current owner and attempt. History records transitions and verification counts. It keeps full body text only in Markdown.

## Native integrations

The Stop command reads the work DB without modifying it. A scoped PostToolUse hook records the route from AFK’s response to the native session and turn. The Claude Code UserPromptSubmit hook clears an unconsumed route when a new prompt arrives. Codex supplies a distinct turn ID. The route lives in the plugin data directory and is consumed atomically, so repeat Stop events are silent. No route is written for unrelated tools. A turn that only uses ordinary tools will have no report unless it used an AFK work tool earlier in that turn. Stop and the routing hooks emit no context for the model. Stop returns only `systemMessage` on success and exits quietly on failure.

Root selection order is `--root`, `AFK_PROJECT_ROOT`, Claude Code’s project directory, an explicit tool `workspace`, MCP Roots when available, then launch cwd. A packaged Codex server can be launched inside the plugin cache, so its fallback is the absolute workspace submitted to the first AFK tool. Each process pins one root. A client workspace change requires reconnecting if the same MCP process is retained.
