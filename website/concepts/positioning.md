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

<figure class="doc-diagram">
  <svg viewBox="0 0 760 270" role="img" aria-label="Positioning map showing Human and AI Agent connected to AFK, optional methodologies, and optional shared boards">
    <defs>
      <marker id="pos-en-arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
        <path d="M 0 1 L 10 5 L 0 9 z" fill="var(--vp-c-brand-1)" />
      </marker>
      <marker id="pos-en-arrow-muted" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
        <path d="M 0 1 L 10 5 L 0 9 z" fill="var(--vp-c-text-3)" />
      </marker>
    </defs>

    <rect x="20" y="106" width="146" height="60" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
    <text x="93" y="131" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="13.5" font-weight="700">Human</text>
    <text x="93" y="151" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">Goals &amp; decisions</text>

    <line x1="166" y1="136" x2="210" y2="136" stroke="var(--vp-c-text-3)" stroke-width="1.5" marker-start="url(#pos-en-arrow-muted)" marker-end="url(#pos-en-arrow-muted)" />

    <rect x="216" y="98" width="184" height="76" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-brand-1)" stroke-width="1.5" />
    <text x="308" y="128" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="13.5" font-weight="700">Existing AI Agent</text>
    <text x="308" y="150" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">Claude Code / Codex</text>

    <path d="M 400 118 L 466 54" stroke="var(--vp-c-brand-1)" stroke-width="2" fill="none" marker-end="url(#pos-en-arrow)" />
    <path d="M 400 136 L 466 136" stroke="var(--vp-c-text-3)" stroke-width="1.5" stroke-dasharray="5 4" fill="none" marker-end="url(#pos-en-arrow-muted)" />
    <path d="M 400 154 L 466 218" stroke="var(--vp-c-text-3)" stroke-width="1.5" stroke-dasharray="5 4" fill="none" marker-end="url(#pos-en-arrow-muted)" />

    <rect x="472" y="22" width="268" height="62" rx="12" fill="var(--vp-c-brand-soft)" stroke="var(--vp-c-brand-1)" stroke-width="1.8" />
    <text x="606" y="47" text-anchor="middle" fill="var(--vp-c-brand-1)" font-size="14" font-weight="700">Agent Flow Kit (AFK)</text>
    <text x="606" y="68" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="12">Work execution harness</text>

    <rect x="472" y="105" width="268" height="62" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
    <text x="606" y="130" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="13.5" font-weight="700">AI-DLC / SDD / Spec Kit</text>
    <text x="606" y="151" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">Optional development methodology</text>

    <rect x="472" y="188" width="268" height="62" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
    <text x="606" y="213" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="13.5" font-weight="700">Vercel Board / Backlog.md</text>
    <text x="606" y="234" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="11.5">Optional shared board / backlog</text>
  </svg>
  <figcaption>Conceptual map of primary roles—not technical dependencies, official integrations, or mutually exclusive categories.</figcaption>
</figure>

## Summary

> Do not impose a development methodology. Do not require a management dashboard. Do not replace your agent. Simply support the AI in carrying work through.

*Basis & verification date: Based on the linked official documentation reviewed on **2026-11-26**. No side-by-side empirical benchmark against these tools has been conducted.*

## Next steps

- [Workflow Harness](/concepts/workflow-harness) — Read AFK's core architecture philosophy.
- [MCP Reference](/reference/mcp) — Inspect the exact 6 tools AFK exposes.
