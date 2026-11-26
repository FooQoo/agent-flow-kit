---
title: Work Model
description: How Agent Flow Kit structures work using four concepts—Request, Plan, Task, and Question—with explicit acceptance conditions.
---

# Work Model

*For readers who want to understand how AFK models a piece of work so it is always clear what happens next and what counts as done.*

> Structure the work so the next step and the definition of done are always explicit.

## The four core entities

AFK models every piece of work with four Markdown-backed entities stored under `.afk/`:

| Entity | Role | Stored in |
| --- | --- | --- |
| **Request** | Captures the initial human instruction and every follow-up addition or change. | `.afk/requests/NNN.md` |
| **Plan** | Organizes how to fulfill the requests—scope, task breakdown, order, and plan-level acceptance criteria. | `.afk/plans/NNN-rMMM.md` |
| **Task** | An executable unit of work with prerequisites, acceptance bullets, attempt history, and completion evidence. | `.afk/tasks/NNN.md` |
| **Question** | A clarifying prompt from AI to the human, plus the sealed human answer once replied. | `.afk/questions/NNN.md` |

## Work lifecycle

<ConceptDiagram name="work-model" lang="en" />

1. **Request**: The agent records your instruction in `.afk/requests/`.
2. **Plan**: The agent prepares a draft revision (`update_plan / prepare`), writes the plan and tasks in Markdown, and commits the draft (`update_plan / commit`).
3. **Clarify (optional)**: Whenever human judgment is needed—during planning or mid-execution—the agent prepares and publishes a Question. Dependent work waits until your answer is recorded and reflected in a committed plan revision.
4. **Execute**: The agent runs ready tasks in dependency order (`start_next`), verifies each `## Acceptance` bullet, writes evidence under `## Result`, and calls `finish_task`.
5. **Complete**: Once every active task is `Done`, the agent verifies the plan's own acceptance bullets and calls `update_plan / finish`.

## Planning vs. execution

AFK distinguishes a **plan-only** request from an **execution** request without forcing a blanket approval gate on every task:

- **Plan only (`execute: false`)**: Committing the draft leaves the plan in `Backlog`. Execution does not start until you explicitly tell the agent to begin (`update_plan / authorize` moves it to `Todo`).
- **Direct execution (`execute: true`)**: Committing moves the plan directly to `Todo` so the agent can start immediately—pausing only if an open question requires your decision.

## Acceptance conditions

Every plan and task has an `## Acceptance` section of ordered `- ` bullets defined **before** execution starts. When committing a draft, AFK locks a SHA-256 hash of the confirmed specification. When finishing a task or plan, the agent must pass the 1-based positions of every verified bullet (`[1, 2, ...]`) and write non-empty evidence under `## Result`.

## Progress and next work

AFK computes the next highest-priority action deterministically from SQLite state—checking open questions, pending requests or answers, open drafts, in-progress tasks (`Doing`), dependency-satisfied `Todo` tasks, and final plan verification. See [MCP Reference](/reference/mcp#progress-and-nextaction-shapes) for all 10 `NextAction` variants.

## Adapting to change

When you add requirements or change direction mid-work:

1. AFK records the new instruction as a new Request.
2. The agent prepares a new plan revision, retiring superseded unfinished tasks (`remove`) and adding updated tasks (`add`).
3. Completed (`Done`) tasks are never silently deleted—they remain as evidence of what was already verified.

## Resuming work

Because requests, confirmed plans, task files, answers, and optional handoff notes live in `.afk/`, a fresh session can call `open_plan` to inspect state, read the latest handoff note, and resume cleanly via `start_next` (using `take_over: true` only after confirming the previous worker has stopped).

## Next steps

- [Human in / on the Loop](/concepts/human-in-on-the-loop) — How responsibility is split between human, AI, and AFK.
- [MCP Reference](/reference/mcp) — Exact tool schemas, `NextAction` priority order, and error rules.
