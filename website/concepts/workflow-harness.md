---
title: Workflow Harness
description: Why Agent Flow Kit is built as a lightweight, state-aware workflow harness that extends existing AI agents rather than replacing them.
---

# Workflow Harness

*For readers evaluating AFK's architecture and core philosophy: why a harness, and what makes it different from prompts alone.*

> AI does the work. AFK supports how that work is carried out. The human stays in control.

## Capability vs. carrying work through

Modern coding models are capable at individual steps—editing files, writing tests, explaining code. Carrying a multi-step request from initial intent to verified completion is a different problem. It requires remembering the original request, keeping task order intact, pausing on genuine ambiguities, recording evidence, and surviving session boundaries.

## What the harness supports

AFK supports six recurring responsibilities across a piece of work:

- **Intent** — preserving the original request and any mid-work additions.
- **Next action** — computing what can be done next from explicit state rather than guessing from chat history.
- **Decision boundaries** — surfacing clarifying questions separately from execution.
- **Progress state** — tracking `Backlog`, `Todo`, `Doing`, and `Done` deterministically in SQLite.
- **Acceptance conditions** — requiring explicit verification bullets and completion evidence (`## Result`).
- **Reporting to the human** — summarizing current progress at turn end via a Stop Hook.

## Structure, guidance, and continuity

Structure (requests, plans, tasks, questions) is a means, not the goal. The harness exists to provide **guidance** (what to do next and what must be verified) and **continuity** (carrying durable state across turns and sessions).

<ConceptDiagram name="workflow-harness" lang="en" />

## Why a state-aware harness—not just a Skill

A Skill alone gives the model behavioral advice, but advice competes with long conversation context and resets on every new session. AFK pairs a short Skill with a stateful local store (`.afk/afk.db` + `.afk/*.md`) exposed through six focused MCP tools. State transitions, dependency checks, SHA-256 spec guards, and progress aggregation happen deterministically in Rust.

## Extend, not replace

**Don't replace your AI agent. Extend how it works.**

AFK preserves your existing model, host client (Claude Code, Codex, or another MCP client), native file and shell tools, and permission boundaries. The agent writes plan and task Markdown with its native file tools while AFK validates structure and manages state.

## Limits

A workflow harness helps keep work legible and verifiable, but it is not a magic guarantee:

- A Skill guides model behavior; it cannot force a model to follow every instruction.
- AFK does not intercept arbitrary shell commands or file edits outside `.afk/`.
- AFK cannot guarantee that an agent makes the right technical judgment or completes every task autonomously.

## Next steps

- [Work Model](/concepts/work-model) — How Request, Plan, Task, and Question fit together.
- [Human in / on the Loop](/concepts/human-in-on-the-loop) — When the agent acts and when it asks.
- [Positioning](/concepts/positioning) — How AFK relates to AI-DLC, SDD, Spec Kit, Vercel Board, and Backlog.md.
