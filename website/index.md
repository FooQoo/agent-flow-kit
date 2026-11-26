---
layout: page
title: Agent Flow Kit
description: A lightweight harness for AI agents to carry out work—going beyond ephemeral Plan Mode with durable .afk/ state, revisioned replanning, verification gates, and cross-session handoff.
sidebar: false
aside: false
landing:
  copy:
    label: Copy commands
    done: Copied
  hero:
    eyebrow: Delegate work. Stay in control.
    title: "Beyond Plan Mode:\na harness that finishes the job."
    lead: Agent Flow Kit is a lightweight harness for AI agents to carry out work. Instead of ephemeral chat plans, it persists requests, revisioned plans, acceptance gates, and handoff state in project-local .afk/ files across sessions.
    primary:
      text: Get started
      link: /guide/installation
    secondary:
      text: GitHub
      link: https://github.com/FooQoo/agent-flow-kit
    meta: Claude Code / Codex · Local-first (.afk/) · Rust + SQLite · Open Source
    demo:
      header: Cross-session resume & revisioned replanning via .afk/
      sampleBadge: Sample
      ariaLabel: Illustrative terminal showing Session 1 saving a handoff and Session 2 resuming from .afk/ with a mid-work requirement revised to rev 2
      boundaryLabel: Next day · New session (Claude Code or Codex)
      caption: Illustrative example. Exact formatting depends on the host client and hook configuration.
      sessions:
        - label: "Session #1 — Claude Code"
          turns:
            - kind: prompt
              text: /agent-flow-kit:afk handoff
            - kind: saved
              text: "Saved handoff for #4 (3/5 done) → .afk/afk.db"
        - label: "Session #2 — Resumed in a new session"
          turns:
            - kind: prompt
              text: Continue from last time, and also fix the empty-results view.
            - kind: reply
              text: "Loaded Plan #1 (rev 1) & handoff from .afk/. Keeping #1–#3 as verified evidence and revising to rev 2."
      status:
        title: Improve search feature
        revision: rev 2
        progressText: 3/6 (50%)
        percent: 50
        tasks:
          - state: done
            text: "#1–#3 Audit, spec & search logic"
            tag: rev 1 evidence
          - state: doing
            text: "#4 Add automated tests"
            tag: from handoff
          - state: todo
            text: "#6 Fix empty-results view"
            tag: added mid-work
  problem:
    eyebrow: Problem
    title: Drafting a plan is easy. Keeping it intact is hard.
    lead: Built-in Plan Mode and todo lists help before you start. Once work spans long turns, mid-work changes, or multiple sessions, different failure modes appear.
    items:
      - icon: list-x
        title: Plans drift on mid-work changes
        text: Follow-up requirements and context compaction quietly separate the original plan from what is actually being built.
      - icon: circle-check
        title: No durable proof of completion
        text: When an agent says "done", there is no structured record of which acceptance checks were actually verified.
      - icon: history
        title: Closing the session resets context
        text: Picking up tomorrow—or in another agent—means re-explaining the plan, past decisions, and remaining tasks.
      - icon: message-circle-question
        title: Decisions get buried in chat logs
        text: Clarifying questions and human answers disappear into scrollback instead of staying attached to the plan.
    comparison:
      title: How AFK differs from Plan Mode & built-in Todos
      lead: Plan Mode is an ephemeral single-session scratchpad. AFK is a persistent, state-checked harness that carries work to verified completion.
      axisHeader: Dimension
      planModeHeader: Built-in Plan Mode / Todos
      afkHeader: Agent Flow Kit (AFK)
      rows:
        - axis: Persistence & scope
          planMode: Ephemeral inside one session; fades on compaction or exit.
          afk: Stored in project-local .afk/; resumes cleanly across sessions and clients.
        - axis: Mid-work changes
          planMode: Overwritten in chat; hard to tell what was already verified.
          afk: Keeps completed tasks as evidence while revising remaining tasks in a new revision.
        - axis: Completion check
          planMode: Self-reported by the model ("All done!").
          afk: Enforced gate requiring SHA-256 spec match, every acceptance bullet verified, and ## Result.
        - axis: Progress visibility
          planMode: Scroll through chat or spend a turn asking for status.
          afk: Stop Hook reads SQLite directly at turn end with zero extra model reasoning.
    closing: What you need is not another upfront checklist—it is a harness that enforces state and evidence all the way to Done.
  workflow:
    eyebrow: Workflow
    title: Ask. Clarify. Let it run.
    lead: Questions are skipped when nothing is ambiguous, and plan-only requests (Backlog) stay separate from execution (Todo).
    steps:
      - step: "01"
        title: Request
        actor: human
        actorLabel: Human
        text: Describe what you want in plain language, just as you already do.
      - step: "02"
        title: Plan
        actor: ai
        actorLabel: AI
        text: Break the request into ordered tasks with explicit ## Acceptance bullets.
      - step: "03"
        title: Clarify
        actor: both
        actorLabel: Human + AI
        text: Ask only when human judgment is needed and seal the answer.
      - step: "04"
        title: Execute
        actor: ai
        actorLabel: AI
        text: Follow dependencies and record verification evidence under ## Result.
      - step: "05"
        title: Complete
        actor: ai
        actorLabel: AI
        text: Pass the completion gate across every acceptance bullet and report.
    adaptTitle: Change direction anytime.
    adaptText: Add follow-up requirements mid-work and revise remaining tasks in a new plan revision without erasing completed evidence. Interruption timing follows your host client.
  humanOnTheLoop:
    eyebrow: Human on the Loop
    title: AI does the work. You stay in control.
    lead: Instead of chasing unconstrained autonomy, AFK centers on Human on the Loop—bringing Human in the Loop only when judgment matters.
    human:
      badge: Human
      title: Your role
      items:
        - Set the goal
        - Make key decisions
        - Adjust direction
    ai:
      badge: AI Agent
      title: AI's role
      items:
        - Plan the work
        - Execute and verify
        - Record evidence & finish
    harness:
      label: Agent Flow Kit
      text: Supports both sides with a deterministic state machine, SHA-256 spec guards, local persistence, and turn-end reporting. AFK does not run models or execute tasks itself.
    note: Questions are reserved for unclear intent, human preferences, irreversible actions, or scope expansion—not rubber-stamp approvals on every step.
  features:
    eyebrow: Features
    title: Four mechanics Plan Mode doesn't give you.
    items:
      - kind: revision
        eyebrow: REVISION
        title: Revisioned replanning without losing history
        text: When requirements change mid-work, AFK retires superseded unfinished tasks and adds updated ones in a new revision—keeping completed tasks locked as evidence.
        visualTitle: Plan #1 · Mid-work revision
        revision:
          from: rev 1
          to: rev 2
          rows:
            - id: "#1–#3"
              state: done
              badge: Kept as evidence
              text: Audit, spec & search logic
            - id: "#5"
              state: retired
              badge: Retired
              text: Old basic empty state
            - id: "#4"
              state: todo
              badge: Retained
              text: Add automated tests
            - id: "#6"
              state: added
              badge: Added
              text: Improve empty-results view
      - kind: gate
        eyebrow: VERIFICATION GATE
        title: Deterministic completion gate & spec locks
        text: Confirmed specs are locked with SHA-256 hashes. A task cannot become Done until every ## Acceptance bullet is explicitly verified and ## Result evidence is written.
        visualTitle: finish_task gate
        gate:
          command: "finish_task(plan_id=1, task_id=4, verified=[1, 2, 3])"
          checks:
            - label: spec_hash (SHA-256)
              value: Intact
            - label: "## Acceptance (1..=3)"
              value: All 3 verified
            - label: "## Result evidence"
              value: Recorded
          outcome: "✓ Gate passed → Task #4: Done"
      - kind: handoff
        eyebrow: HANDOFF & .afk/
        title: Cross-session & cross-client continuity
        text: Requests, plans, questions, and results live in project-local .afk/. Save a handoff note and pick up cleanly in a fresh session—or even another MCP client.
        visualTitle: .afk/ + handoff
        handoff:
          tree:
            - ".afk/"
            - "├── afk.db        # state, dependencies & hashes"
            - "├── plans/        # revisioned plans"
            - "├── tasks/        # acceptance, attempts & Result"
            - "├── requests/     # initial & follow-up requests"
            - "└── questions/    # prompts & sealed answers"
          noteLabel: "handoff (Attempt 1: claude-code → Attempt 2: codex)"
          noteText: "Search logic implemented; remaining work is unit tests and empty-state edge cases."
      - kind: progress
        eyebrow: STOP HOOK
        title: Zero-reasoning turn-end progress reports
        text: A Stop Hook reads .afk/afk.db directly at turn end and prints a concise status line—so you stay informed without spending extra turns asking.
        note: Designed to avoid extra model reasoning or reporting tool calls. Display details vary by client.
        visualTitle: Turn-end report
        progress:
          percent: 60
          tag: 0 model tokens
          detail: Aggregated directly from SQLite (read-only, fail-open)
          banner: "AFK · Improve search feature [Doing] · 3/5 complete · Next: Add automated tests"
  efficiency:
    eyebrow: State Machine & Efficiency
    title: Backed by a state machine, not polite prompts.
    lead: Prompt-only rules fade as context grows. Offloading deterministic transitions, hash checks, and progress aggregation to Rust + SQLite lets your agent focus on reasoning and code.
    ai:
      badge: AI Agent
      title: "AI handles: reasoning, creation & execution"
      items:
        - Drafting & revising plans
        - Understanding specifications
        - Producing deliverables
        - Running actual verification
    afk:
      badge: AFK (Rust + SQLite)
      title: "AFK handles: deterministic state & gates"
      items:
        - Enforcing transitions & dependencies
        - Verifying SHA-256 spec integrity
        - Gating open questions & pending requests
        - Aggregating progress & persisting .afk/
    note: Lower management overhead and token efficiency are active design goals; comparative benchmarks against standalone agents will be published once measured.
  architecture:
    eyebrow: Architecture
    title: No new environment required.
    lead: Don't replace your AI agent. Extend how it works. Install AFK as a plugin and use MCP, Skills, and Hooks together as one kit.
    diagram:
      ariaLabel: Architecture diagram showing Claude Code and Codex using Agent Flow Kit MCP, Skills, and Hooks over Markdown and SQLite
      clientsLabel: AI Agents
      clients:
        - Claude Code
        - Codex
      kitLabel: Agent Flow Kit (Plugin)
      kit:
        - name: MCP
          role: State machine & verification gates
        - name: Skills
          role: Execution guidance
        - name: Hooks
          role: Turn-end progress reports
      storageLabel: Project (.afk/)
      storage:
        - name: Markdown
          role: Content edited with native tools
        - name: SQLite
          role: State, dependencies & hashes
    items:
      - icon: laptop
        title: Local-first
        text: Requests, plans, and state stay inside .afk/ in your repository.
      - icon: cpu
        title: Lightweight Rust binary
        text: No separate runtime or background server required.
      - icon: plug
        title: Installed as a plugin
        text: Works inside your existing agent without a separate dashboard.
      - icon: code
        title: Open Source
        text: Inspect every schema, hook, and state transition directly.
    note: "Note: Local-first refers to where AFK stores workflow data. Model traffic from Claude Code or Codex still follows your client's own configuration."
  start:
    eyebrow: Getting Started
    title: Install once. Work as usual.
    lead: Add the AFK plugin to Claude Code or Codex and delegate your next task in plain language.
    steps:
      - title: Install the plugin
        text: Add the AFK Git marketplace to Claude Code or Codex.
      - title: Open your project
        text: Keep using your existing repository and terminal workflow.
      - title: Delegate a task
        text: Ask in natural language. On first run, the launcher downloads and SHA256-verifies the binary for your OS.
    clientSelectLabel: Select client
    clients:
      - id: claude-code
        label: Claude Code
        commands:
          - claude plugin marketplace add "https://github.com/FooQoo/agent-flow-kit.git"
          - claude plugin install agent-flow-kit@afk
      - id: codex
        label: Codex
        commands:
          - codex plugin marketplace add "https://github.com/FooQoo/agent-flow-kit.git"
          - codex plugin add agent-flow-kit@afk
    installNote: Network access is only needed on the first run for a given version to fetch and verify the release binary. For offline archives and hook setup, see the
    guideLink:
      text: Installation guide
      link: /guide/installation
    cta:
      tagline: "Delegate work.\nStay in control."
      sub: A lightweight harness that goes beyond Plan Mode.
      primary:
        text: Get started with AFK
        link: /guide/installation
      secondary:
        text: GitHub
        link: https://github.com/FooQoo/agent-flow-kit
---

<LandingPage />
