---
title: MCP Reference
description: Complete reference for Agent Flow Kit's six MCP tools, Progress/NextAction schemas, and common error messages.
---

# MCP Reference

*For advanced users and integrators inspecting the exact MCP tool schemas, state transitions, and error messages exposed by AFK.*

Agent Flow Kit exposes **six MCP tools** over stdio. Combined with your agent's standard file tools (`Read`, `Write`, `Edit`), they handle the full workflow from first request to final verification.

You do not need to write these JSON payloads by hand during normal use—your coding agent calls them for you.

## Tool overview

| Tool | Purpose | Writes `.afk/afk.db`? |
| --- | --- | --- |
| [`record_request`](#record-request) | Saves a new or follow-up instruction and creates `.afk/requests/NNN.md`. | Yes |
| [`open_plan`](#open-plan) | Lists recent plans, or inspects one plan's files, tasks, and next action. | Read-only |
| [`update_plan`](#update-plan) | Prepares, recovers, commits, authorizes, or finishes a plan revision; also saves task handoffs. | Yes (`recover` restores missing files only) |
| [`update_question`](#update-question) | Prepares a question file, locks the prompt, or locks the human answer. | Yes |
| [`start_next`](#start-next) | Starts the next ready task (or takes over a stopped `Doing` task). | Yes |
| [`finish_task`](#finish-task) | Verifies acceptance bullet positions and `## Result`, then marks a task `Done`. | Yes |

All tool inputs reject unknown fields (`deny_unknown_fields`). Every response bound to a single plan includes `"plan_id"` and `"afk_root"` at the top level.

## `record_request` {#record-request}

Registers an initial request (creating a new plan) or attaches a follow-up requirement to an existing plan. The agent then writes the human's instruction to the returned Markdown path.

### Parameters

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `workspace` | `string` | Conditional | Absolute path to your project folder. Required on the first call when `AFK_NEEDS_WORKSPACE=1` is set. |
| `plan_id` | `integer` | No | Omit to start a new plan. Pass an existing `plan_id` for follow-up additions or changes. |
| `receipt` | `string` | No | Optional retry key (`1–100` bytes). Reusing the same key for the same request returns the existing record without duplicating it. |

### Example & Response

```json
{
  "workspace": "/Users/alice/projects/payments",
  "receipt": "req-initial-01"
}
```

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

## `open_plan` {#open-plan}

Inspects a specific plan by ID, or lists up to 10 recent plans (newest first) when `plan_id` is omitted.

### Parameters

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `workspace` | `string` | Conditional | Absolute project path (only needed if the session is not yet bound). |
| `plan_id` | `integer` | No | Plan ID to inspect. Omit to list recent plans. |
| `tasks` | `boolean` | No | Default `false`. Set `true` with `plan_id` to include active task paths, statuses, `after` prerequisites, and any saved `handoff` note. |
| `before` | `integer` | No | Pagination cursor when listing plans; returns up to 10 plans with `id < before`. |
| `check_files` | `boolean` | No | Default `false`. Set `true` to surface up to 32 `.md` files in `.afk/` not referenced by `afk.db`. |

### Example & Response

```json
{
  "plan_id": 1,
  "tasks": true
}
```

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

## `update_plan` {#update-plan}

Manages plan revisions, execution authorization, completion, and task handoff notes via `change.op`.

| `change.op` | Required fields | Optional fields | Behavior |
| --- | --- | --- | --- |
| `"prepare"` | `revision`, `add` (`integer[][]`, max 32 tasks / 32 prereqs each) | `remove` (`integer[]`, unfinished IDs only), `replace_draft` (`integer`) | Reserves a new plan file (`plans/NNN-rMMM.md`) and task files (`tasks/NNN.md`). Positive IDs in `add` reference retained tasks; negative 1-based offsets (`-1`, `-2`) reference earlier new tasks in `add`. |
| `"recover"` | `draft_id` | — | Recreates missing blank template files for an `open` draft without overwriting existing files. |
| `"commit"` | `draft_id`, `requests` (`integer[]`), `answers` (`integer[]`) | `execute` (`boolean`, default `false`) | Validates titles and `## Acceptance` bullets, locks SHA-256 hashes, retires `remove` tasks, and advances `revision`. On `Backlog`, `execute: true` moves the plan to `Todo`. |
| `"authorize"` | — | — | Moves a confirmed (`revision > 0`) `Backlog` plan to `Todo` after human approval to start. |
| `"finish"` | `revision`, `verified` (`integer[]`) | — | Marks the plan `Done` after all active tasks are `Done`, all plan acceptance bullets (`1..=N`) are verified, and `## Result` is non-empty. |
| `"save_handoff"` | `task_id`, `attempt`, `note` (Markdown up to 16 KiB) | — | Saves or replaces the handoff note for the currently owned `Doing` task without changing its status. |

## `update_question` {#update-question}

Manages clarifying questions in `.afk/questions/NNN.md` via three `change.op` variants:

| `change.op` | Extra fields | Behavior |
| --- | --- | --- |
| `"prepare"` | `task_id?: integer`, `draft_id?: integer` | Creates `.afk/questions/NNN.md`. Scope to an unfinished `task_id`, an open `draft_id`, or neither (not both). |
| `"publish"` | `question_id: integer` | Validates the title and locks the SHA-256 hash of everything above `## Answer`. |
| `"answer"` | `question_id: integer` | Verifies the prompt was not altered and locks the SHA-256 hash of the human reply under `## Answer`. |

## `start_next` {#start-next}

Selects and starts the next ready task. If a task is already `Doing`, AFK returns it; otherwise it starts the lowest-ID `Todo` task whose prerequisites are all `Done`.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `plan_id` | `integer` | Yes | Target plan ID. |
| `take_over` | `boolean` | No | Default `false`. Set `true` only after confirming the previous session has stopped. |
| `model` | `string` | No | Executing model name, recorded alongside the MCP client name in the task's attempt log. |

Returns `TaskRun`: `{ plan_id, afk_root, id, path, attempt, status: "doing" }`.

## `finish_task` {#finish-task}

Completes the running task after every acceptance bullet is verified and evidence is written under `## Result`.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `plan_id` | `integer` | Yes | Target plan ID. |
| `task_id` | `integer` | Yes | ID of the `Doing` task. |
| `attempt` | `integer` | Yes | Exact `attempt` number returned by `start_next`. |
| `verified` | `integer[]` | Yes | 1-based positions covering every bullet in `## Acceptance` (`[1, 2, ..., N]`). |

## `Progress` and `NextAction` shapes {#progress-and-nextaction-shapes}

| Field | Type | Meaning |
| --- | --- | --- |
| `plan_id` | `integer` | Numeric plan ID. |
| `title` | `string` | Plain-text plan title (`"Plan #<id>"` before revision 1). |
| `revision` | `integer` | Confirmed revision number (`0` before first commit). |
| `status` | `string` | `"backlog"`, `"todo"`, `"doing"`, or `"done"`. |
| `total` / `done` / `doing` | `integer` | Active task counts in the current revision. |
| `next` | `NextAction` | Highest-priority next step, tagged by `"action"`. |

### All 10 `NextAction` variants (in priority order)

| `next.action` | Extra fields | Meaning |
| --- | --- | --- |
| `"wait_for_answer"` | `id`, `title`, `count` | One or more published questions await a human reply. |
| `"prepare_question"` | `id` | A question file was prepared but not yet published. |
| `"replan"` | `requests`, `answers` | Unapplied requests or answered questions must be reflected in a new revision. |
| `"confirm_draft"` | `id` | An open draft is waiting for `update_plan / commit`. |
| `"complete"` | — | Plan is `Done` with no pending changes. |
| `"prepare_plan"` | — | New plan (`revision == 0`) needs its first draft. |
| `"await_start"` | — | Confirmed `Backlog` plan awaits `update_plan / authorize`. |
| `"resume_task"` | `id`, `title` | An active task is currently `Doing`. |
| `"start_task"` | `id`, `title` | A ready `Todo` task can be started via `start_next`. |
| `"verify_plan"` | — | All active tasks are `Done`; verify plan acceptance and call `update_plan / finish`. |

## Common error messages

| Error message | Cause | Fix |
| --- | --- | --- |
| `provide the current project directory as workspace to record_request or open_plan` | `AFK_NEEDS_WORKSPACE=1` is set and the first call omitted `workspace`. | Pass `"workspace": "/absolute/path/to/project"`. |
| `AFK is already bound to ...; start a new MCP session for another workspace` | Passed a different `workspace` than the pinned root for this process. | Start a new session in the other project. |
| `open draft changed; inspect it, recover its files or replace it by ID` | Called `prepare` while an open draft already exists without `replace_draft`. | Inspect with `open_plan`, then commit, `recover`, or pass `replace_draft`. |
| `request set changed; inspect and reflect all pending requests` | `requests` or `answers` passed to `commit` omitted pending IDs. | Call `open_plan`, reflect all pending items, and pass complete ID lists. |
| `a different session owns this task; take over only after its worker stops` | Called `start_next` on a `Doing` task owned by an earlier session without `take_over: true`. | Confirm the earlier session stopped, then pass `"take_over": true`. |
| `verify each of the N acceptance bullets by its one-based position` | `verified` did not match `1..=N`. | Check every bullet in `## Acceptance` and pass `[1, ..., N]`. |
| `confirmed text changed at ...` | A confirmed plan or task `.md` file was edited after its hash was locked. | Copy edits aside, restore the confirmed text, and replan in a new draft. |

For operational recovery steps, see [Troubleshooting](/troubleshooting).
