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

<figure class="doc-diagram">
  <svg viewBox="0 0 760 270" role="img" aria-label="Lifecycle diagram from Request to Plan, Task execution, and Verification, with optional Question and Follow-up Request loops">
    <defs>
      <marker id="wm-en-arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
        <path d="M 0 1 L 10 5 L 0 9 z" fill="var(--vp-c-brand-1)" />
      </marker>
      <marker id="wm-en-arrow-muted" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
        <path d="M 0 1 L 10 5 L 0 9 z" fill="var(--vp-c-text-3)" />
      </marker>
    </defs>

    <rect x="24" y="34" width="132" height="58" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
    <text x="90" y="59" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="14" font-weight="700">1. Request</text>
    <text x="90" y="78" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">Record goal</text>

    <line x1="156" y1="63" x2="202" y2="63" stroke="var(--vp-c-brand-1)" stroke-width="1.8" marker-end="url(#wm-en-arrow)" />

    <rect x="208" y="34" width="144" height="58" rx="12" fill="var(--vp-c-brand-soft)" stroke="var(--vp-c-brand-1)" stroke-width="1.5" />
    <text x="280" y="59" text-anchor="middle" fill="var(--vp-c-brand-1)" font-size="14" font-weight="700">2. Plan</text>
    <text x="280" y="78" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="11.5">Order &amp; acceptance</text>

    <line x1="352" y1="63" x2="398" y2="63" stroke="var(--vp-c-brand-1)" stroke-width="1.8" marker-end="url(#wm-en-arrow)" />

    <rect x="404" y="34" width="144" height="58" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
    <text x="476" y="59" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="14" font-weight="700">3. Execute</text>
    <text x="476" y="78" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">Run &amp; log evidence</text>

    <line x1="548" y1="63" x2="594" y2="63" stroke="var(--vp-c-brand-1)" stroke-width="1.8" marker-end="url(#wm-en-arrow)" />

    <rect x="600" y="34" width="136" height="58" rx="12" fill="var(--vp-c-brand-soft)" stroke="var(--vp-c-brand-1)" stroke-width="1.5" />
    <text x="668" y="59" text-anchor="middle" fill="var(--vp-c-brand-1)" font-size="14" font-weight="700">4. Done</text>
    <text x="668" y="78" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="11.5">All checks verified</text>

    <rect x="330" y="168" width="210" height="58" rx="12" fill="var(--vp-c-bg)" stroke="var(--lp-pending-border, #d99b26)" stroke-width="1.5" />
    <text x="435" y="193" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="13.5" font-weight="700">Question (when needed)</text>
    <text x="435" y="212" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">Pause &amp; seal human answer</text>

    <path d="M 460 92 L 460 162" stroke="var(--vp-c-text-3)" stroke-width="1.5" stroke-dasharray="4 4" fill="none" marker-end="url(#wm-en-arrow-muted)" />
    <path d="M 380 168 L 315 98" stroke="var(--vp-c-brand-1)" stroke-width="1.5" fill="none" marker-end="url(#wm-en-arrow)" />
    <text x="326" y="144" fill="var(--vp-c-brand-1)" font-size="11">Reflect answer</text>

    <rect x="44" y="168" width="196" height="58" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
    <text x="142" y="193" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="13.5" font-weight="700">Follow-up Request</text>
    <text x="142" y="212" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">Keep Done tasks &amp; replan</text>

    <path d="M 190 168 L 250 98" stroke="var(--vp-c-brand-1)" stroke-width="1.5" stroke-dasharray="5 4" fill="none" marker-end="url(#wm-en-arrow)" />
  </svg>
  <figcaption>Core lifecycle from Request to verified Done, with optional Question and mid-work replanning loops.</figcaption>
</figure>

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
