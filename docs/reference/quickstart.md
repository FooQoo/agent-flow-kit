# Quickstart

In about **10 minutes**, this walkthrough takes one small task from start to finish so you can see every core part of Agent Flow Kit in action:

1. Sending a multi-step request
2. Checking the files AFK creates in `.afk/`
3. Answering a clarifying question (`Q-001`)
4. Holding a plan in `Backlog` vs. running right away (`authorize`)
5. Adding a new requirement mid-flight (*"Also reject empty values"*)
6. Checking `## Acceptance` bullets and `## Result`
7. Closing a session and resuming from a fresh one

Before you start, connect AFK to your client ([Install and set up](getting-started.md)) and open an empty test folder.

---

## Step 1: Send your request

Invoke AFK in your project and ask for a JSON fixture and verifier. Leave the exact fields open on purpose so you can watch AFK ask you before deciding:

- **Claude Code:**
  ```text
  /agent-flow-kit:afk Create a sample JSON fixture and a verifier script with tests. Ask me which fields to include.
  ```
- **Codex:** Pick `$agent-flow-kit:afk` and enter the same message.
- **Antigravity / MCP:** Trigger your `afk` skill with the same message.

The agent calls `record_request` first and writes your prompt into `.afk/requests/001.md`.

---

## Step 2: Check what AFK creates in `.afk/`

Next, the agent prepares Draft `#1` with two ordered tasks (`add: [[], [-1]]`, so Task `#2` waits for Task `#1`), and creates a question file for the fixture fields.

If you look inside `.afk/`, you will see:

```text
.afk/
├── afk.db
├── requests/
│   └── 001.md          # Your request
├── plans/
│   └── 001-r001.md     # Draft #1 for Plan #1
├── tasks/
│   ├── 001.md          # Task #1 (create fixture)
│   └── 002.md          # Task #2 (write verifier & tests, after #1)
└── questions/
    └── 001.md          # Question about fixture fields
```

Each plan and task file starts from a blank template with `# `, `## Acceptance`, and `## Result`, which the agent fills in with normal file tools.

---

## Step 3: Answer the question (`Q-001`)

Instead of guessing your schema, the agent writes `.afk/questions/001.md`, publishes it, and asks you in chat:

```markdown
# Which fields should sample.json contain?

Pick the shape you prefer:
1. `{ "id": "item-1", "value": "ready" }` (strings)
2. `{ "id": 1, "count": 42 }` (numbers)

## Answer
```

While a question is waiting for an answer, AFK blocks task execution—calling `start_next` returns an error until you reply.

Answer in chat:

```text
Use { "id": "item-1", "value": "ready" } with both fields as strings.
```

The agent writes your reply under `## Answer` in `.afk/questions/001.md`, locks the answer with `update_question(answer)`, updates the draft plan and tasks to match, and commits revision 1 (`requests: [1]`, `answers: [1]`).

---

## Step 4: Backlog vs immediate execution

What happens right after `commit` depends on what you asked for:

| What you asked | `commit` setting | Plan status | What happens next |
| --- | --- | --- | --- |
| *"Create a JSON fixture and verifier..."* (do the work) | `execute: true` | `Todo` | The agent calls `start_next` right away and begins Task `#001`. |
| *"Plan how to create a JSON fixture—don't start yet"* (plan only) | `execute: false` | `Backlog` | AFK waits at `await_start` while you read `.afk/plans/001-r001.md` and `.afk/tasks/*.md`. |

When a plan is waiting in `Backlog`, tell the agent when you are ready:

```text
Looks good—start Plan #1.
```

The agent calls `update_plan(authorize)`, moving the plan and its tasks to `Todo`, and starts Task `#001`.

---

## Step 5: Steer mid-flight ("Also reject empty values")

After Task `#001` (*Create `fixtures/sample.json`*) finishes—or while Task `#002` is running—add a new requirement in chat:

```text
Also reject empty or whitespace-only strings in the verifier, and add a test for whitespace.
```

Here is what AFK does behind the scenes:

1. **Saves the new request:** The agent calls `record_request(plan_id: 1)` and writes `.afk/requests/002.md`.
2. **Pauses completion:** Because a new request is waiting, AFK blocks `finish_task` and `update_plan(finish)` until the plan is updated (`next: replan`).
3. **Checks existing tasks:** Calling `open_plan(plan_id: 1, tasks: true)` shows `#001` is `done` and `#002` is unfinished.
4. **Commits revision 2 cleanly:** The agent prepares a new draft (`revision: 1`, `remove: [2]`, `add: [[1]]`), writes `.afk/plans/001-r002.md` and `.afk/tasks/003.md`, and commits it (`requests: [2]`).

When revision 2 commits:
- Task `#001` stays `Done` with its original text and `## Result` untouched.
- Old Task `#002` is retired (`lifecycle = 'retired'`), though its `.md` file stays on disk.
- New Task `#003` enters `Todo`, waiting on `#001` (which is already `Done`).
- `001-r002.md` becomes the active plan (`revision: 2`).

---

## Step 6: Check acceptance bullets and `## Result`

The agent starts Task `#003` with `start_next` (receiving `attempt: 1`), writes the code and tests, and runs your test suite. `.afk/tasks/003.md` now looks like this:

```markdown
# Build verifier that rejects empty and whitespace strings

Check `fixtures/sample.json` and fail when `id` or `value` is missing, empty, or whitespace-only.

## Acceptance

- Passes on `fixtures/sample.json` with exit code 0
- Exits non-zero when `id` or `value` is empty or whitespace-only
- Automated tests cover valid, empty, and whitespace-only inputs

## Result

Ran `python3 -m unittest discover -s tests` — all 3 tests passed.
```

The agent finishes Task `#003` by passing the 1-based bullet numbers it verified:

```json
{
  "plan_id": 1,
  "task_id": 3,
  "attempt": 1,
  "verified": [1, 2, 3]
}
```

AFK checks three things before marking `#003` `Done`:
- Nothing above `## Result` changed since revision 2 was committed.
- `verified` lists all three bullets (`[1, 2, 3]`).
- `## Result` is not empty.

Once all active tasks are `Done`, the agent checks the plan's own `## Acceptance` bullets in `001-r002.md`, writes the summary under `## Result`, and calls `update_plan(finish)`.

In Claude Code and Codex, the `Stop` hook prints:

```text
AFK · JSON fixture & verifier [Done] · 2/2 complete
Plan complete
```

---

## Step 7: Close the session and resume in a new one

To test resuming across sessions, quit your terminal (or open a fresh chat) mid-plan and invoke AFK again:

```text
/agent-flow-kit:afk Resume my latest plan
```

1. `open_plan` (with no ID) lists recent plans from `.afk/afk.db`.
2. `open_plan(plan_id: ...)` loads the current plan path, any open questions, and the exact `next` step—no old chat history needed.
3. If the previous session closed while a task was still in `Doing`, the agent confirms the old session is gone and calls `start_next(plan_id: ..., take_over: true)`, bumping `attempt` from `1` to `2` and continuing from there.

---

## What you verified here

- **Plans and requests stay in `.afk/`** instead of getting lost in chat logs.
- **Questions stop execution** until you answer, then lock your reply under `## Answer`.
- **`Backlog` lets you review first**, moving to `Todo` only when you say to start.
- **Mid-flight changes keep finished work intact** while replacing only unfinished tasks.
- **`Done` requires proof:** unchanged spec text, every acceptance bullet number, and a non-empty `## Result`.
- **New sessions pick right back up** from `.afk/`.

## Next to read

- **[The work model](work-model.md)** — How all the pieces fit together, what AFK guarantees itself, and what still relies on the agent.
- **[Questions and replanning](decisions-and-replanning.md)** — When the agent asks you and how replanning handles task dependencies.
- **[Resuming and recovery](resuming-and-recovery.md)** — Taking over stopped tasks, restoring missing draft files, and fixing accidental edits.
