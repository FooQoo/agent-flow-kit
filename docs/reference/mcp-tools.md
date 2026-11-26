# MCP tools reference

Agent Flow Kit gives your coding agent **six MCP tools**. Together with standard file tools (`Read`, `Write`, `Edit`), they handle the entire workflow from first request to final verification.

You rarely type these JSON payloads yourself—your agent calls them for you. This page is your reference when inspecting tool calls, building custom integrations, or troubleshooting an error message.

---

## Tool overview

| Tool | What it does | Writes to `.afk/afk.db`? |
| --- | --- | --- |
| [`record_request`](#record-request) | Saves a new or follow-up instruction and creates `.afk/requests/NNN.md`. | Yes |
| [`open_plan`](#open-plan) | Lists recent plans, or inspects one plan's files, tasks, and next step. | Read-only |
| [`update_plan`](#update-plan) | Prepares, recovers, commits, authorizes, or finishes a plan revision. Also saves task handoffs. | Yes (`recover` is read + file restore) |
| [`update_question`](#update-question) | Creates a question file, locks the prompt, or locks the human answer. | Yes |
| [`start_next`](#start-next) | Starts the next ready task (or takes over a stopped `Doing` task). | Yes |
| [`finish_task`](#finish-task) | Checks acceptance bullet numbers and `## Result`, then marks a task `Done`. | Yes |

All tool inputs reject unknown fields (`deny_unknown_fields`). Every response tied to a single plan automatically includes `"plan_id"` and `"afk_root"` at the top level.

---

<a name="record-request"></a>

## `record_request`

Registers your initial request (creating a new plan) or attaches a follow-up change to an existing plan. After calling this tool, the agent writes your words into the returned Markdown file.

### Parameters

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `workspace` | `string` | Conditional | Absolute path to your project folder. Required on the first call when `AFK_NEEDS_WORKSPACE=1` is set. |
| `plan_id` | `integer` | No | Omit to start a brand-new plan. Pass an existing `plan_id` to add or change requirements. |
| `receipt` | `string` | No | Optional retry key (`1–100` bytes). Reusing the same key for the same request returns the existing record instead of creating a duplicate. |

### Example

```json
{
  "workspace": "/Users/alice/projects/payments",
  "receipt": "req-initial-01"
}
```

### Response

```json
{
  "plan_id": 1,
  "afk_root": "/Users/alice/projects/payments",
  "request": {
    "id": 1,
    "path": "/Users/alice/projects/payments/.afk/requests/001.md"
  }
}
```

---

<a name="open-plan"></a>

## `open_plan`

Resumes a specific plan by ID, or lists up to 10 recent plans when `plan_id` is omitted.

### Parameters

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `workspace` | `string` | Conditional | Absolute project path (needed only if the session is not bound yet). |
| `plan_id` | `integer` | No | Plan to inspect. Omit to list recent plans (newest first). |
| `tasks` | `boolean` | No | Default `false`. Set `true` with `plan_id` to include active task paths, statuses, and `after` prerequisites. |
| `before` | `integer` | No | Used when `plan_id` is omitted. Returns up to 10 plans with `id < before` for pagination. |
| `check_files` | `boolean` | No | Default `false`. Set `true` to list up to 32 `.md` files in `.afk/` that are not tracked in `afk.db`. |

### Example: Inspecting a plan with tasks

```json
{
  "plan_id": 1,
  "tasks": true
}
```

### Response (`PlanView`)

Empty lists (`requests`, `questions`) and absent optional fields (`draft`, `tasks`) are omitted from the JSON output to keep responses compact.

```json
{
  "plan_id": 1,
  "afk_root": "/Users/alice/projects/payments",
  "plan": {
    "id": 1,
    "path": "/Users/alice/projects/payments/.afk/plans/001-r001.md"
  },
  "questions": [
    {
      "id": 1,
      "path": "/Users/alice/projects/payments/.afk/questions/001.md",
      "state": "waiting"
    }
  ],
  "tasks": [
    {
      "id": 1,
      "path": "/Users/alice/projects/payments/.afk/tasks/001.md",
      "status": "done"
    },
    {
      "id": 2,
      "path": "/Users/alice/projects/payments/.afk/tasks/002.md",
      "status": "todo",
      "after": [1]
    }
  ],
  "progress": {
    "plan_id": 1,
    "title": "Add token refresh guard",
    "revision": 1,
    "status": "doing",
    "total": 2,
    "done": 1,
    "doing": 0,
    "next": {
      "action": "wait_for_answer",
      "id": 1,
      "title": "Which HTTP status should empty tokens return?",
      "count": 1
    }
  }
}
```

*(In `questions`, `state` is `"draft"` before publication, `"waiting"` while awaiting your answer, and `"answered"` once sealed but not yet committed into a plan revision.)*

---

<a name="update-plan"></a>

## `update_plan`

Manages the lifecycle of a plan and saves task handoffs. Every call takes `plan_id` and a `change` object.

### `op: "prepare"` — Create a draft revision

Reserves a new plan file (`plans/NNN-rMMM.md`) and up to 32 new task files (`tasks/NNN.md`).

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `revision` | `integer` | Yes | Current `progress.revision` (`0` for a brand-new plan). Fails if the plan changed underneath you. |
| `add` | `integer[][]` | Yes | One prerequisite list per new task (max 32 tasks, max 32 prerequisites each). Use positive IDs (`1`, `2`) for kept active tasks, and negative 1-based offsets (`-1`, `-2`) for earlier new tasks in this same `add` array. |
| `remove` | `integer[]` | No | Default `[]`. Unfinished active task IDs to retire when this draft commits (max 32). Completed (`Done`) tasks cannot be removed. |
| `replace_draft` | `integer` | No | Required only if an `open` draft already exists and you want to discard and replace it. |

```json
{
  "plan_id": 1,
  "change": {
    "op": "prepare",
    "revision": 1,
    "remove": [2],
    "add": [[1], [-1]]
  }
}
```

Returns a `Draft` object (`plan_id`, `draft_id`, `base_revision`, `plan_path`, `tasks`, `afk_root`).

---

### `op: "recover"` — Recreate missing draft files

Restores blank template files for an `open` draft if a process stopped before creating them or a draft file was deleted. Existing draft files are never overwritten.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `draft_id` | `integer` | Yes | ID of the currently `open` draft. |

```json
{
  "plan_id": 1,
  "change": {
    "op": "recover",
    "draft_id": 2
  }
}
```

---

### `op: "commit"` — Confirm a draft into the active revision

Checks all titles and `## Acceptance` bullets, locks SHA-256 `spec_hash` values, retires `remove` tasks, and advances `revision` by 1.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `draft_id` | `integer` | Yes | ID of the `open` draft to confirm. (Calling again on the already-active draft is safe and returns current `Progress`.) |
| `requests` | `integer[]` | Yes | IDs of **all** pending unapplied requests reflected in this revision. |
| `answers` | `integer[]` | Yes | IDs of **all** pending answered questions reflected in this revision. |
| `execute` | `boolean` | No | Default `false`. On a `Backlog` plan, `true` moves the plan to `Todo` right away; `false` keeps it in `Backlog` until `authorize`. |

```json
{
  "plan_id": 1,
  "change": {
    "op": "commit",
    "draft_id": 1,
    "requests": [1],
    "answers": [1],
    "execute": true
  }
}
```

Returns updated [`Progress`](#progress-and-nextaction-shapes).

---

### `op: "authorize"` — Start a confirmed `Backlog` plan

Moves a confirmed (`revision > 0`) `Backlog` plan and its tasks to `Todo` after you tell the agent to begin.

```json
{
  "plan_id": 1,
  "change": {
    "op": "authorize"
  }
}
```

Returns updated `Progress`.

---

### `op: "finish"` — Complete the entire plan

Marks the plan `Done` once every active task is `Done`, the plan's own `## Acceptance` bullets are verified, and a non-empty `## Result` is written in the active plan file.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `revision` | `integer` | Yes | Current `progress.revision`. |
| `verified` | `integer[]` | Yes | 1-based positions covering every bullet in the plan's `## Acceptance` section (`[1, 2, ...]`). |

```json
{
  "plan_id": 1,
  "change": {
    "op": "finish",
    "revision": 2,
    "verified": [1, 2]
  }
}
```

Returns updated `Progress` (`status: "done"`, `next: { "action": "complete" }`).

### Save a task handoff with `op: "save_handoff"`

Pass `task_id`, the current `attempt` from `start_next`, and a Markdown `note` of up to 16 KiB. Only the session that currently owns the Doing task can save a note. Saving replaces the last note and keeps the task Doing. To read it from another session, call `open_plan` with `tasks: true` and look for `handoff` on the task.

---

<a name="update-question"></a>

## `update_question`

Manages clarifying questions in `.afk/questions/NNN.md` through three `change.op` variants.

| `change.op` | Extra fields | What it does |
| --- | --- | --- |
| `"prepare"` | `task_id?: integer`, `draft_id?: integer` | Creates `.afk/questions/NNN.md` from the question template. Scope to either an unfinished active `task_id` or an open `draft_id` (not both), or omit both. |
| `"publish"` | `question_id: integer` | Validates the question title and locks the SHA-256 hash of everything above `## Answer`. |
| `"answer"` | `question_id: integer` | Verifies the prompt was not altered and locks the SHA-256 hash of the reply written under `## Answer`. |

### Examples

```json
{
  "plan_id": 1,
  "change": { "op": "prepare", "draft_id": 1 }
}
```

```json
{
  "plan_id": 1,
  "change": { "op": "publish", "question_id": 1 }
}
```

```json
{
  "plan_id": 1,
  "change": { "op": "answer", "question_id": 1 }
}
```

---

<a name="start-next"></a>

## `start_next`

Picks and starts the next task in order. If a task is already `Doing`, AFK selects it before any `Todo` task; otherwise it picks the lowest-ID `Todo` task whose prerequisites are all `Done`.

### Parameters

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `plan_id` | `integer` | Yes | Target plan ID. |
| `take_over` | `boolean` | No | Default `false`. Set `true` only to claim a `Doing` task whose previous MCP session has stopped. |
| `model` | `string` | No | Executing model name, when known. AFK shows it alongside the MCP client name in the task's execution history. |

### Example & Response (`TaskRun`)

```json
{
  "plan_id": 1,
  "take_over": false
}
```

```json
{
  "plan_id": 1,
  "afk_root": "/Users/alice/projects/payments",
  "id": 1,
  "path": "/Users/alice/projects/payments/.afk/tasks/001.md",
  "attempt": 1,
  "status": "doing"
}
```

---

<a name="finish-task"></a>

## `finish_task`

Completes the running task after the agent checks every acceptance bullet and writes evidence beneath `## Result`.

### Parameters

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `plan_id` | `integer` | Yes | Target plan ID. |
| `task_id` | `integer` | Yes | ID of the `Doing` task. |
| `attempt` | `integer` | Yes | Exact `attempt` number returned by `start_next`. |
| `verified` | `integer[]` | Yes | 1-based positions covering every bullet in the task's `## Acceptance` section (`[1, 2, ...]`). |

### Example

```json
{
  "plan_id": 1,
  "task_id": 1,
  "attempt": 1,
  "verified": [1, 2]
}
```

Returns updated `Progress`.

---

## `Progress` and `NextAction` shapes

`commit`, `authorize`, `finish_task`, `update_plan(finish)`, and `open_plan` all return a `Progress` object:

| Field | Type | Meaning |
| --- | --- | --- |
| `plan_id` | `integer` | Numeric plan ID. |
| `title` | `string` | Plain-text plan title (or `"Plan #<id>"` before revision 1 is committed). |
| `revision` | `integer` | Confirmed revision number (`0` before the first commit). |
| `status` | `string` | `"backlog"`, `"todo"`, `"doing"`, or `"done"`. |
| `total` | `integer` | Count of `active` tasks in the current revision. |
| `done` | `integer` | Count of `active` tasks in `"done"`. |
| `doing` | `integer` | Count of `active` tasks in `"doing"` (`0` or `1`). |
| `next` | `NextAction` | Highest-priority next step, tagged by `"action"`. |

### All 10 `NextAction` variants (in priority order)

| `next.action` | Extra fields | Meaning |
| --- | --- | --- |
| `"wait_for_answer"` | `id`, `title`, `count` | At least one published question is waiting for your reply. |
| `"prepare_question"` | `id` | A question file was reserved but has not been published yet. |
| `"replan"` | `requests`, `answers` | Pending requests or answered questions must be committed in a draft. |
| `"confirm_draft"` | `id` | An open draft exists and needs `update_plan(commit)`. |
| `"complete"` | — | Plan is `Done` with no pending changes. |
| `"prepare_plan"` | — | New plan (`revision == 0`) needs its first draft (`update_plan(prepare)`). |
| `"await_start"` | — | Confirmed `Backlog` plan is waiting for `update_plan(authorize)`. |
| `"resume_task"` | `id`, `title` | An active task is currently `Doing` (`start_next` / `finish_task`). |
| `"start_task"` | `id`, `title` | A ready `Todo` task can be started via `start_next`. |
| `"verify_plan"` | — | All active tasks are `Done`; check plan acceptance bullets and call `update_plan(finish)`. |

---

## Common error messages and fixes

| Error message | Why it happened | How to resolve it |
| --- | --- | --- |
| `provide the current project directory as workspace to record_request or open_plan` | `AFK_NEEDS_WORKSPACE=1` is set and the first tool call omitted `workspace`. | Pass `"workspace": "/absolute/path/to/project"` on `record_request` or `open_plan`. |
| `AFK is already bound to ...; start a new MCP session for another workspace` | Passed a different `workspace` path than the project already pinned by this MCP process. | Start a new session in the other project directory. |
| `open draft changed; inspect it, recover its files or replace it by ID` | Called `prepare` while an `open` draft already exists without matching `replace_draft`. | Call `open_plan` to inspect the existing draft; either commit it, `recover` it, or pass its `draft_id` as `replace_draft`. |
| `request set changed; inspect and reflect all pending requests` *(or `question set changed...`)* | `requests` or `answers` passed to `commit` did not match all unapplied IDs in `afk.db`. | Call `open_plan`, check `requests` and `questions`, reflect all of them in the draft, and pass the complete ID lists. |
| `replace each retained task whose prerequisite is being removed` | A kept task depends on a task listed in `remove`. | Also include the dependent unfinished task in `remove`, and re-add an updated task in `add`. |
| `resolve questions and reflect pending requests or answers in a confirmed plan` | Tried to start or finish work while a request or question is still pending. | Finish answering/publishing any open question and commit a new plan revision first. |
| `a different session owns this task; take over only after its worker stops` | Called `start_next` on a `Doing` task owned by an earlier MCP process without `take_over: true`. | Confirm the old session has exited, then call `start_next` with `"take_over": true`. |
| `task attempt changed; inspect before continuing` | `finish_task` passed an outdated `attempt` or came from a superseded session. | Call `open_plan` (and `start_next` if needed) to inspect current task ownership before continuing. |
| `verify each of the N acceptance bullets by its one-based position` | `verified` did not match `1..=N` for the `## Acceptance` section. | Check every bullet in `## Acceptance` and pass `[1, 2, ..., N]`. |
| `confirmed text changed at ...; preserve your edits elsewhere, restore the confirmed text and replan in a new draft` | A confirmed plan or task `.md` file was edited after its hash was locked. | Save your edits outside `.afk/`, restore the file's confirmed text, and apply changes via `prepare` + `commit`. |

Next, see **[Markdown format & templates](file-format.md)** for the exact structure, templates, and hashing rules of every file inside `.afk/`.
