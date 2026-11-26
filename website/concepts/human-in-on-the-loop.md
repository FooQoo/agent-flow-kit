---
title: Human in / on the Loop
description: How Agent Flow Kit centers on Human on the Loop and brings Human in the Loop only when judgment matters.
---

# Human in / on the Loop

*For readers who want to understand how AFK splits responsibility between humans, AI agents, and the harness without adding friction.*

> AI does the work. You stay in control.

## Human on the Loop (HOTL)

AFK's default posture is **Human on the Loop**: you do not babysit every file edit or tool call. You state the goal, review the plan when you want to, glance at automatic turn-end progress reports, and step in to adjust direction when needed.

## Human in the Loop (HITL)

When a task hits a genuine decision point, AFK shifts cleanly to **Human in the Loop**. Instead of guessing silently, the agent writes a structured Question in `.afk/questions/`, asks you in chat, seals your answer, and reflects it in the plan before proceeding.

## When to ask

AFK's Skill guides the agent to ask **only** when a human decision is actually required:

1. **Unclear intent** — the request can reasonably be interpreted in conflicting ways.
2. **Human preference** — design, UX, naming, or trade-off choices where multiple valid options exist.
3. **Irreversible actions** — public publishing, destructive deletes, external side effects, or schema changes that are hard to undo.
4. **Scope expansion** — work that goes materially beyond what the user asked for.

*(Host security and permission prompts remain controlled by your client and are always respected independently of AFK.)*

## Who does what

| Actor | Responsibilities |
| --- | --- |
| **Human** | Set the goal, make key decisions, adjust direction, review results. |
| **AI Agent** | Draft plans, ask clarifying questions when needed, execute tasks, verify acceptance conditions. |
| **AFK** | Maintain structure and state, enforce prerequisites and acceptance checks, persist `.afk/`, and report progress at turn end. |

## Mid-work requests

Requirements change while work is underway. You can send additions or corrections in chat; AFK records each as a new Request so the agent can assess impact, keep completed tasks as evidence, and revise remaining work. How quickly an in-flight turn can accept a new message depends on your host client's capabilities—AFK does not force-stop a running model over MCP.

## Progress at turn end

When client Hooks are enabled, AFK's Stop Hook reads `.afk/afk.db` in read-only mode at the end of any turn that touched AFK and prints a concise status line (`AFK · <title> [<Status>] · ...`). This is **turn-end reporting**, not a live background dashboard, and it avoids asking the model for extra reasoning or reporting tool calls.

## Autonomy and limits

- AFK does **not** add a mandatory approval gate to every single task.
- AFK does **not** guarantee unattended background execution, automatic answers to open questions, or error-free autonomy.

## Next steps

- [Positioning](/concepts/positioning) — Compare AFK with development methodologies and task boards.
- [Quickstart](/guide/quickstart) — See HOTL and HITL in action on a sample task.
