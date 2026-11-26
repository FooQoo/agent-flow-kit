# Resuming and recovery

Long jobs regularly span multiple terminal sessions, context resets, or closed laptops. Because Agent Flow Kit keeps states in `.afk/afk.db` and text in `.afk/**/*.md`, a fresh session can see right where things stand in one call—and recover cleanly if an earlier session stopped mid-step.

---

## Listing recent plans

If you open a new chat and do not have a `plan_id` handy, ask your agent to list recent plans (calling `open_plan` with `plan_id` omitted):

```json
{}
```

*(When `AFK_NEEDS_WORKSPACE=1` is set and the session has not bound a folder yet, include `"workspace": "/absolute/path/to/project"`.)*

AFK returns up to **10 plans**, newest first (`ORDER BY id DESC LIMIT 10`), each with its `Progress` summary (`plan_id`, `title`, `revision`, `status`, `total`, `done`, `doing`, and `next`).

To see older plans, pass `before` set to the lowest `plan_id` from the page you just saw:

```json
{
  "before": 12
}
```

This returns up to 10 plans with `id < 12`.

---

## Inspecting a plan to continue

Once you know which plan you want, `open_plan` loads everything needed to pick up without reading old chat logs:

```json
{
  "plan_id": 1,
  "tasks": true
}
```

The response gives the agent:
- **`plan`:** Path to the confirmed plan file (for example `.afk/plans/001-r002.md`), or `null` before revision 1 is committed.
- **`draft`:** Shown if an uncommitted `open` draft exists, with its `draft_id`, `plan_path`, and draft `tasks`.
- **`requests`:** Pending requests not yet folded into a confirmed plan.
- **`questions`:** Pending questions, labeled `"draft"`, `"waiting"`, or `"answered"`.
- **`tasks`:** *(When `tasks: true`)* Every active task's path, `status`, and `after` prerequisites.
- **`progress`:** Counts and the exact `next` step (`wait_for_answer`, `replan`, `confirm_draft`, `await_start`, `resume_task`, `start_task`, `verify_plan`, or `complete`).

---

## Taking over a stopped `Doing` task

Suppose Session A started Task `#2` (`status: "doing"`, `attempt: 1`), edited a file, and then your terminal closed before `finish_task` ran.

When you open Session B, a **new `afk` server process** starts with a new worker ID. `open_plan` shows:

```json
{
  "action": "resume_task",
  "id": 2,
  "title": "Serialize concurrent token refreshes"
}
```

### How `take_over` and `attempt` keep sessions from colliding

1. **Inside the same session:** Calling `start_next(plan_id: 1)` again in Session A returns Task `#2` with the same `attempt: 1`.
2. **From a new session without `take_over`:** Calling `start_next(plan_id: 1)` in Session B with the default `take_over: false` is refused so two live sessions do not step on each other:
   > `a different session owns this task; take over only after its worker stops`
3. **Taking over once the old session has stopped:** After confirming Session A is gone, Session B calls:
   ```json
   {
     "plan_id": 1,
     "take_over": true
   }
   ```
   AFK hands ownership to Session B, bumps `attempt` from `1` to `2`, and records `task_resumed` in `history`.

```mermaid
sequenceDiagram
    participant S1 as Session A (old worker)
    participant DB as .afk/afk.db
    participant S2 as Session B (new worker)

    S1->>DB: start_next(plan_id: 1)
    DB-->>S1: task_id: 2, attempt: 1
    Note over S1: Session A closes or stops
    S2->>DB: start_next(plan_id: 1, take_over: true)
    DB-->>S2: task_id: 2, attempt: 2
    opt If stalled Session A wakes up later
        S1->>DB: finish_task(task_id: 2, attempt: 1)
        DB-->>S1: Refused: "task attempt changed; inspect before continuing"
    end
    S2->>DB: finish_task(task_id: 2, attempt: 2, verified: [1, 2])
    DB-->>S2: Task #2 → Done
```

Because `finish_task` checks both the worker ID and the `attempt` number, a stalled old session can never overwrite or close a task after Session B takes over.

---

## Save a handoff checkpoint

To save a handoff for the current task, use one of these commands in the session doing the work.

- **Claude Code** — `/agent-flow-kit:afk handoff`
- **Codex** — Select `$agent-flow-kit:afk` and send `handoff`.

You can also ask the working agent to prepare a handoff. AFK saves the progress, changed files or worktree, verification, remaining work, and the next action. The task stays Doing with its current owner.

If the original agent stops during preparation, use `open_plan` with `tasks: true` to read the last saved `handoff`. Check the actual working files against the note. Once the former worker has stopped, use `start_next` with `take_over: true` to continue. For long tasks, save a checkpoint at meaningful milestones so a sudden stop still leaves useful context.

## Restoring missing files of an open draft (`recover`)

When `update_plan(prepare)` runs, AFK reserves IDs in `afk.db` first and writes the blank template `.md` files right afterward.

If the process stopped in between—or a draft `.md` file was deleted by mistake before `commit`—call `update_plan` with `op: "recover"`:

```json
{
  "plan_id": 1,
  "change": {
    "op": "recover",
    "draft_id": 2
  }
}
```

- **Missing files** for that open draft are recreated from the blank template.
- **Existing files** on disk are left alone—any text already written in the draft is preserved.

---

## Replacing an abandoned open draft (`replace_draft`)

A plan can have only one `open` draft at a time. If an old session left an unfinished draft open (say `draft_id: 2`) and you want to start a fresh draft instead of finishing it:

1. Call `open_plan(plan_id: 1)` to check `draft.draft_id` (`2`) and `progress.revision`.
2. Pass `replace_draft: 2` to `update_plan(prepare)`:

```json
{
  "plan_id": 1,
  "change": {
    "op": "prepare",
    "revision": 1,
    "remove": [],
    "add": [[], [-1]],
    "replace_draft": 2
  }
}
```

AFK marks draft `#2` as `discarded`, keeps any files already written for `#2` on disk so nothing is lost, and opens draft `#3` (`plans/001-r003.md`).

---

## Finding unreferenced `.md` files (`check_files`)

If someone created extra `.md` files inside `.afk/` by hand or copied files from a mismatched backup, pass `check_files: true` to `open_plan`:

```json
{
  "plan_id": 1,
  "check_files": true
}
```

AFK checks `.afk/{plans,tasks,requests,questions}/*.md` against `afk.db` and lists up to 32 untracked paths in `orphan_documents`.

> [!TIP]
> **If an untracked file blocks a new ID:** Before creating `.afk/tasks/005.md`, AFK checks that nothing already sits at that path. If an untracked file is there, AFK stops with `"unreferenced file ...; preserve it elsewhere before reserving this path"`. Move that file outside `.afk/` and try again.

---

## Fixing a confirmed `.md` file edited by hand

Once a plan or task is committed, AFK locks the SHA-256 hash (`spec_hash`) of everything above `## Result` (and once `Done`, locks `final_hash` over `## Result`).

If you edit a confirmed file in `.afk/plans/` or `.afk/tasks/` by hand, the next AFK call stops with:

> `confirmed text changed at /path/to/.afk/tasks/002.md; preserve your edits elsewhere, restore the confirmed text and replan in a new draft`

Fix it in three steps—**without editing `afk.db`**:

1. **Copy your edits somewhere outside `.afk/`** (for example into a scratch note) so you do not lose them.
2. **Restore the file to its confirmed text** using editor undo, `git checkout -- .afk/tasks/002.md`, or your backup so its hash matches `afk.db` again.
3. **Put the change through a new revision:** Call `record_request(plan_id)`, then `update_plan(prepare)` (retiring the old unfinished task in `remove` and adding the updated one in `add`) and `commit`.

---

## Backing up or moving `.afk/`

Inside `afk.db`, all file paths are stored **relative to `.afk/`** (`plans/001-r001.md`, `tasks/001.md`, and so on), and the project root is resolved when `afk` starts. You can move a project folder or back up `.afk/` cleanly by following three rules:

1. **Wait until AFK is idle:** Close active agent sessions (or let the current turn finish) before copying so no database write is mid-flight.
2. **Copy the whole `.afk/` folder together:** Always keep `afk.db` and `plans/`, `tasks/`, `requests/`, `questions/` as one matched set.
3. **Use normal files and folders (no symlinks):** Copy with ordinary file tools (`cp -R`). AFK refuses symlinks inside `.afk/`.

Next, see the **[MCP tools reference](mcp-tools.md)** for every tool's exact fields, JSON examples, and error messages.
