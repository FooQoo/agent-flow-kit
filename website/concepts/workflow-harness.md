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

<figure class="doc-diagram">
  <svg viewBox="0 0 760 310" role="img" aria-label="Responsibility diagram showing Human, AI Agent, Native Tools, Agent Flow Kit, and project-local .afk files">
    <defs>
      <marker id="wh-en-arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
        <path d="M 0 1 L 10 5 L 0 9 z" fill="var(--vp-c-brand-1)" />
      </marker>
      <marker id="wh-en-arrow-muted" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
        <path d="M 0 1 L 10 5 L 0 9 z" fill="var(--vp-c-text-3)" />
      </marker>
    </defs>

    <rect x="230" y="16" width="300" height="58" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
    <text x="380" y="40" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="14" font-weight="700">Human</text>
    <text x="380" y="60" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="12">Goals · Key decisions · Direction changes</text>

    <line x1="380" y1="74" x2="380" y2="112" stroke="var(--vp-c-text-3)" stroke-width="1.5" marker-start="url(#wh-en-arrow-muted)" marker-end="url(#wh-en-arrow-muted)" />
    <text x="392" y="97" fill="var(--vp-c-text-2)" font-size="11.5">Natural conversation</text>

    <rect x="210" y="116" width="340" height="60" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-brand-1)" stroke-width="1.5" />
    <text x="380" y="141" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="14" font-weight="700">AI Agent (Claude Code / Codex)</text>
    <text x="380" y="161" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="12">Reasoning · Planning · Execution · Verification</text>

    <path d="M 290 176 L 190 218" stroke="var(--vp-c-text-3)" stroke-width="1.5" fill="none" marker-end="url(#wh-en-arrow-muted)" />
    <path d="M 470 176 L 560 218" stroke="var(--vp-c-brand-1)" stroke-width="1.8" fill="none" marker-end="url(#wh-en-arrow)" />

    <rect x="30" y="222" width="310" height="66" rx="12" fill="var(--vp-c-bg)" stroke="var(--vp-c-divider)" stroke-width="1.5" />
    <text x="185" y="247" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="13.5" font-weight="700">Native Tools (Read / Write / Edit / Shell)</text>
    <text x="185" y="269" text-anchor="middle" fill="var(--vp-c-text-2)" font-size="12">Project files &amp; .afk/*.md content</text>

    <rect x="410" y="222" width="320" height="66" rx="12" fill="var(--vp-c-brand-soft)" stroke="var(--vp-c-brand-1)" stroke-width="1.5" />
    <text x="570" y="247" text-anchor="middle" fill="var(--vp-c-brand-1)" font-size="13.5" font-weight="700">Agent Flow Kit (Plugin)</text>
    <text x="570" y="269" text-anchor="middle" fill="var(--vp-c-text-1)" font-size="12">State, order &amp; gates in .afk/afk.db</text>

    <path d="M 680 222 L 680 45 L 538 45" stroke="var(--vp-c-brand-1)" stroke-width="1.5" stroke-dasharray="5 4" fill="none" marker-end="url(#wh-en-arrow)" />
    <text x="692" y="138" fill="var(--vp-c-brand-1)" font-size="11.5" font-weight="600">Turn-end</text>
    <text x="692" y="154" fill="var(--vp-c-brand-1)" font-size="11.5" font-weight="600">progress</text>
  </svg>
  <figcaption>Conceptual separation of responsibilities. AFK does not proxy every message between the human and the agent.</figcaption>
</figure>

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
