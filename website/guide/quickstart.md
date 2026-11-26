---
title: Quickstart
description: Delegate your first multi-step task with Agent Flow Kit from request and clarification to verified completion and session handoff.
---

# Quickstart

*For first-time users who have finished [Installation](/guide/installation) and want to run one complete task from request to verified completion.*

**Prerequisites:** AFK is installed in Claude Code or Codex, and you have opened a project you are comfortable editing.

## End-to-end example

The conversation below illustrates the full lifecycle—request, plan, question, execution, mid-work change, and completion:

```text
Human: Improve the search feature and add tests.
AI:    I drafted a plan:
       1. Inspect existing search implementation
       2. Define ranking specification
       3. Implement search logic
       4. Add automated tests
       5. Verify acceptance conditions
AI:    Should search results be sorted by relevance or newest first?
[AFK]  Improve search feature [Backlog] · ░░░░░░░░░░ 0/5 complete (0%)

Human: Sort by relevance, and go ahead.
AI:    Recorded your answer, updated the plan, and finished tasks 1–3.
[AFK]  Improve search feature [Doing] · ██████░░░░ 3/5 complete (60%)

Human: Also improve the empty-results view when 0 items match.
AI:    Recorded the follow-up request and updated the plan and acceptance criteria.
[AFK]  Improve search feature [Doing] · █████░░░░░ 3/6 complete (50%)

Human: Continue.
AI:    Completed implementation, tests, and verified every acceptance bullet.
[AFK]  Improve search feature [Done] · ██████████ 6/6 complete (100%)
```

*(Illustrative example. The `[AFK]` lines represent turn-end Stop Hook reports, distinct from the agent's own text. You do not have to prompt the agent on every task; the number of tasks and pauses depends on your request and client.)*

## Step-by-step walkthrough

### 1. Request

Open a new session in your project and describe what you want in plain language. In Claude Code, prefix with `/agent-flow-kit:afk`; in Codex, select `$agent-flow-kit:afk`.

No special template is required. AFK records your original instruction under `.afk/requests/`.

### 2. Plan

The agent breaks the request into ordered tasks with explicit `## Acceptance` bullets and writes them to `.afk/plans/` and `.afk/tasks/`. If you asked for a plan only, AFK keeps it in `Backlog` so implementation does not start until you say so.

### 3. Clarify

If a decision depends on your intent, preference, irreversible action, or scope choice, the agent prepares a question in `.afk/questions/` and asks you in chat. Work that depends on the answer waits; if nothing is ambiguous, this step is skipped.

### 4. Execute

Once authorized (or when you asked for direct execution), the agent works through ready tasks in dependency order, checking off acceptance bullets and writing verification evidence under `## Result`.

### 5. Adapt

Need to add a requirement or change direction mid-work? Send the follow-up instruction in chat. AFK records the new request, lets the agent revise remaining tasks and acceptance conditions in a new plan revision, and keeps already-completed tasks intact as evidence.

*(Mid-work interruption timing follows what your host client supports.)*

### 6. Complete

Before finishing a task or the entire plan, AFK checks that every acceptance bullet has been explicitly verified and that `## Result` is populated. You can inspect the recorded results directly in `.afk/tasks/` and `.afk/plans/`.

### 7. Resume across sessions

To leave a checkpoint for a later session, run `/agent-flow-kit:afk handoff` in Claude Code or select `$agent-flow-kit:afk` and send `handoff` in Codex.

When you open a new session in the same project, ask AFK to resume the plan (for example, `"Resume the previous AFK plan"`). AFK loads the saved state, handoff note, and remaining tasks so the new session can pick up where the last one stopped.

## Common prompt patterns

| Pattern | Example prompt | What happens |
| --- | --- | --- |
| Plan only | *"Draft an implementation plan first. Don't start yet."* | Commits the plan in `Backlog` without starting execution. |
| Start a confirmed plan | *"Looks good—go ahead with that plan."* | Authorizes the `Backlog` plan into `Todo` and begins executing tasks. |
| Direct execution | *"Implement this feature and verify it with tests."* | Plans and executes immediately, pausing only if a decision needs clarification. |
| Mid-work change | *"Actually, sort by newest first and handle empty results."* | Records an additional request, assesses impact, and replans remaining work. |
| Resume | *"Continue the previous AFK work."* | Opens saved plans, reads the handoff note and remaining tasks, and resumes. |
| Status check | *"Where are we on this plan?"* | Summarizes progress, open questions, and the next ready task (also shown automatically at turn end when Hooks are active). |

## Next steps

- [Work Model](/concepts/work-model) — Understand Request, Plan, Task, and Question.
- [Human in / on the Loop](/concepts/human-in-on-the-loop) — See how AFK balances autonomy and control.
- [Troubleshooting](/troubleshooting) — Resolve setup, hook, or resume issues.
