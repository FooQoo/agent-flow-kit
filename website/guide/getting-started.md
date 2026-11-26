---
title: Getting Started
description: What Agent Flow Kit (AFK) is, what problems it addresses, and what it intentionally leaves to your AI agent.
---

# What is Agent Flow Kit?

*For first-time visitors who want to understand what AFK supports—and what it does not replace—before installing.*

> Agent Flow Kit is a lightweight harness for AI agents to carry out work.
>
> Starting from a human request, it supports planning, task decomposition, clarifying questions, execution, and completion—extending your existing AI agent without replacing it.

## What is AFK?

**Agent Flow Kit (AFK)** is a **workflow harness** installed as a plugin into coding agents such as Claude Code and Codex. Instead of asking you to adopt a new IDE, runtime, or task dashboard, AFK lets you keep working in natural language inside your usual terminal while maintaining durable structure and state behind the scenes in `.afk/`.

Its tagline summarizes the split of responsibility: **Delegate work. Stay in control.**

## Why AFK?

Modern coding agents can write code, run tests, and navigate large repositories. Even so, delegating multi-step work often breaks down in predictable ways:

- **Goals drift over long tasks** — early requirements or acceptance criteria fall out of active context.
- **Unasked assumptions** — the agent guesses product or design preferences that should have been confirmed with a human.
- **Unclear completion** — you still have to audit whether every requested requirement was actually verified.
- **Lost continuity across sessions** — starting a new chat or terminal session forces you to re-explain the plan and remaining work.

## What AFK does

AFK pairs six stateful MCP tools, a concise Skill, and lightweight lifecycle Hooks with plain Markdown and SQLite stored inside your project:

- **Records requests** — captures both the initial instruction and mid-work additions or changes.
- **Structures plans and tasks** — organizes work into ordered tasks with explicit prerequisites and acceptance conditions.
- **Isolates clarifying questions** — pauses on open questions only when human intent, preference, irreversible impact, or scope expansion requires a decision.
- **Guides execution and verification** — tracks task attempts, requires explicit acceptance checks and completion evidence (`## Result`), and guards confirmed specs with SHA-256 hashes.
- **Reports progress automatically** — summarizes current status at the end of a turn via a Stop Hook without requiring extra model reasoning.
- **Preserves continuity** — keeps all work state in project-local `.afk/` files and supports handoff notes between sessions.

## How it feels

You do not call MCP tools manually. You talk to your agent as usual:

```text
You:   Improve the search feature and add tests. Show me the plan first.
Agent: Drafted a 5-task plan. One question before starting:
       Should results be sorted by relevance or newest first?
You:   Relevance. Go ahead.
Agent: Updated the plan and started executing tasks in order.
[AFK]  Improve search feature [Doing] · ██████░░░░ 3/5 complete (60%)
```

*(Illustrative example. See [Quickstart](/guide/quickstart) for the full end-to-end flow, including mid-work changes and session handoff.)*

## What AFK does not do

To stay small and composable, AFK intentionally does **not** provide:

- Its own LLM or model hosting
- An agent launcher, orchestrator, or parallel worker runtime
- A separate web dashboard or project management UI (and it does not require Agent Board or Agent Project)
- A mandatory software development methodology

AFK extends how your existing agent carries out work; it does not replace the agent or guarantee model behavior.

## Supported agents

- **Claude Code** — supported via the Git plugin marketplace (MCP + Skill + Hooks).
- **Codex** — supported via the Git plugin marketplace (MCP + Skill + Hooks).
- **Other stdio MCP clients** (such as Antigravity) — supported via manual stdio MCP + Skill configuration.

Exact installation steps and client differences are covered in [Installation](/guide/installation).

## Next steps

- [Installation](/guide/installation) — Add AFK to Claude Code, Codex, or another MCP client.
- [Quickstart](/guide/quickstart) — Delegate your first task from request to verified completion.
- [Workflow Harness](/concepts/workflow-harness) — Learn why AFK is designed as a state-aware harness.
