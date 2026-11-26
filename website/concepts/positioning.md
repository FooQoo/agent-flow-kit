---
title: Positioning
description: How Agent Flow Kit differs from—and relates to—AI-DLC, SDD, GitHub Spec Kit, Vercel Board, and Backlog.md.
---

# Positioning — How AFK compares

*For developers evaluating how AFK fits alongside existing AI development methodologies and task management tools.*

> Agent Flow Kit is a harness for carrying out work with AI agents. Its starting point differs from software development methodologies and team task boards—though they share concepts such as planning, tasks, and review, and each serves distinct needs.

## A. AI-driven development methodologies: AI-DLC / SDD / Spec Kit

- **[AI-DLC Workflows](https://awslabs.github.io/aidlc-workflows/guide/00-introduction/)**: A methodology and implementation organizing AI-assisted software development into reproducible engineering phases, including tooling that runs inside existing CLI harnesses.
- **SDD (Spec-Driven Development)**: An approach centered on authoring and maintaining specifications as the primary driver of software development.
- **[GitHub Spec Kit](https://github.github.com/spec-kit/index.html)**: A toolkit for spec-driven development workflows that goes beyond static templates to provide process-level guidance.

| Aspect | AI-DLC / SDD (typical focus) | AFK (design focus) |
| --- | --- | --- |
| Primary goal | Structure software engineering phases and specifications | Support carrying a delegated piece of work through to verified completion |
| Core concepts | Development phases, specs, engineering artifacts | Request / Plan / Task / Question |
| Scope | Primarily software engineering | Starts from coding, designed to apply to research or writing too |
| Methodology | Concrete process and specification guidance | Methodology-agnostic; does not prescribe a specific dev process |
| Human involvement | Phase reviews and decisions defined by the methodology | Targeted clarifying questions, mid-work steering, progress checks |

**Relationship:** AFK does not oppose AI-DLC or SDD—a team following those methodologies could in principle delegate work through AFK. However, Skill interactions, dual state tracking, plan granularity, and approval boundaries have not been formally tested together, so AFK does not currently claim official integration or guaranteed compatibility.

## B. Task boards & work tracking: Vercel Board / Backlog.md

- **[Vercel Board](https://github.com/vercel/board)**: A collaborative workspace where teams and coding agents coordinate work through channels, threads, and tasks.
- **[Backlog.md](https://github.com/MrLesk/Backlog.md)**: A Markdown-centric tool for managing human and AI tasks in Git repositories. Its [Manifesto](https://github.com/MrLesk/Backlog.md/blob/main/MANIFESTO.md) also emphasizes explicit planning, acceptance criteria, history, and AI workflow guidance—meaning it shares substantial conceptual overlap with AFK.

| Aspect | Vercel Board | Backlog.md | AFK (design focus) |
| --- | --- | --- | --- |
| Primary goal | Shared coordination for teams and coding agents | Explicit task management and human–AI collaboration in Git | Supporting execution of delegated work inside existing agents |
| Core concepts | Channels, threads, tasks | Tasks, plans, acceptance criteria, history | Request-to-completion workflow progression |
| Primary surface | Shared collaborative workspace | Explicit task/backlog management UI & CLI | Natural conversation inside your existing coding agent |
| Human involvement | Shared threads, review, delegation | Task and review management | Requests, targeted decisions, mid-work steering |
| Intended AFK difference | — | — | Requires no separate management UI; extends how your existing agent runs |

Because tools like Backlog.md also provide planning, acceptance criteria, and AI rules, AFK does not claim that other tools "only track tasks while only AFK supports execution." The difference is primarily the center of gravity and interaction surface.

## C. How they fit together

```text
  Human (goals & decisions) <──▶ Existing AI Agent (Claude Code / Codex)
                                     ├──▶ AFK (work execution harness)
                                     ├──▷ AI-DLC / SDD / Spec Kit (optional methodology)
                                     └──▷ Vercel Board / Backlog.md (optional shared board/backlog)
```

*(Conceptual map of primary roles—not technical dependencies, official integrations, or mutually exclusive categories.)*

## Summary

> Do not impose a development methodology. Do not require a management dashboard. Do not replace your agent. Simply support the AI in carrying work through.

*Basis & verification date: Based on the linked official documentation reviewed on **2026-11-26**. No side-by-side empirical benchmark against these tools has been conducted.*

## Next steps

- [Workflow Harness](/concepts/workflow-harness) — Read AFK's core architecture philosophy.
- [MCP Reference](/reference/mcp) — Inspect the exact 6 tools AFK exposes.
