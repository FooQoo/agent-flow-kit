---
layout: page
title: Agent Flow Kit
description: A lightweight harness for AI agents to carry out work. Don't replace your AI agent. Extend how it works.
sidebar: false
aside: false
landing:
  copy:
    label: Copy commands
    done: Copied
  hero:
    eyebrow: Delegate work. Stay in control.
    title: "Give your AI agent\na way to finish the job."
    lead: Agent Flow Kit is a lightweight harness for AI agents to carry out work. From request and plan to clarifying questions, execution, and completion—extend your existing agent without leaving your workflow.
    primary:
      text: Get started
      link: /guide/
    secondary:
      text: GitHub
      link: https://github.com/FooQoo/agent-flow-kit
    meta: Claude Code / Codex · Local-first · Open Source
    demo:
      header: Claude Code + AFK
      sampleBadge: Sample
      ariaLabel: Illustrative terminal conversation showing planning, a clarifying question, and AFK task progress
      caption: Illustrative example. Exact formatting depends on the host client and hook configuration.
      turns:
        - kind: prompt
          text: Improve the search feature
        - kind: reply
          text: "I drafted a plan. One question before starting:"
        - kind: question
          text: Should search results be sorted by relevance or newest first?
        - kind: prompt
          text: Sort by relevance.
        - kind: reply
          text: Got it. Proceeding with the implementation.
      status:
        title: Improve search feature
        progressText: 3/5 (60%)
        percent: 60
        tasks:
          - state: done
            text: Inspect existing search implementation
          - state: done
            text: Define ranking specification
          - state: done
            text: Implement search logic
          - state: doing
            text: Add automated tests
          - state: todo
            text: Verify acceptance conditions
  problem:
    eyebrow: Problem
    title: AI can code. Handing off real work is still hard.
    items:
      - icon: list-x
        title: Plans drift mid-way
        text: Over longer tasks, the original goal and requirements quietly drop out of view.
      - icon: message-circle-question
        title: Unasked assumptions
        text: The agent guesses product decisions that should have been confirmed with a human.
      - icon: circle-check
        title: Unclear completion
        text: You still have to manually audit whether every requirement was actually satisfied.
      - icon: history
        title: Context resets between sessions
        text: Starting a new session means re-explaining the plan, decisions, and remaining work.
    closing: Capable models still need a reliable workflow harness to carry work across the finish line.
  workflow:
    eyebrow: Workflow
    title: Ask. Clarify. Let it run.
    lead: Questions are skipped when nothing is ambiguous, and plan-only requests stay separate from execution.
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
        text: Break the request into ordered tasks with explicit acceptance conditions.
      - step: "03"
        title: Clarify
        actor: both
        actorLabel: Human + AI
        text: Ask only when a human decision, preference, or scope choice is needed.
      - step: "04"
        title: Execute
        actor: ai
        actorLabel: AI
        text: Work through tasks in dependency order and record completion evidence.
      - step: "05"
        title: Complete
        actor: ai
        actorLabel: AI
        text: Verify acceptance conditions and report the finished result.
    adaptTitle: Change direction anytime.
    adaptText: Add follow-up requirements mid-work and revise the plan and acceptance conditions. Mid-work updates take effect at the boundaries supported by your client.
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
        - Report progress
    harness:
      label: Agent Flow Kit
      text: Supports both sides with structure, state-aware guidance, persistence, and progress reporting. AFK does not run models or execute tasks itself.
    note: Questions are reserved for unclear intent, human preferences, irreversible actions, or scope expansion—not rubber-stamp approvals on every step.
  features:
    eyebrow: Features
    title: Four mechanics for delegating work.
    items:
      - kind: plan
        eyebrow: PLAN
        title: Structured plans & tasks
        text: Decompose large requests into executable tasks with explicit order and acceptance conditions.
        visualTitle: Improve search feature
        items:
          - state: done
            text: Gather requirements
          - state: done
            text: Design ranking rules
          - state: todo
            text: Implement search
          - state: todo
            text: Add tests
      - kind: question
        eyebrow: QUESTION
        title: Ask only when needed
        text: When intent or preference matters, pause on the open question instead of guessing silently.
        visualTitle: How should search results be sorted?
        items:
          - text: By relevance
          - text: Newest first
      - kind: progress
        eyebrow: PROGRESS
        title: Automatic progress reports
        text: A Stop Hook summarizes current progress at the end of each turn so you do not have to keep asking.
        note: Designed to avoid extra model reasoning or reporting tool calls. Display details vary by client.
        visualTitle: Turn-end report
        progress:
          percent: 60
          label: 60%
          detail: 3 of 5 tasks completed
          banner: "AFK · Improve search feature [Doing] · 3/5 complete · Next: Add automated tests"
      - kind: workspace
        eyebrow: WORKSPACE
        title: Cross-session continuity
        text: Requests, plans, tasks, and questions stay in .afk/ inside your project so the next session can pick right up.
        visualTitle: .afk/
        tree:
          - ".afk/"
          - "├── plans/"
          - "├── tasks/"
          - "├── requests/"
          - "├── questions/"
          - "└── afk.db"
  efficiency:
    eyebrow: Efficiency
    title: Let AI focus on the work itself.
    lead: Rather than burning context on bookkeeping, AFK offloads deterministic state management to a lightweight Rust binary.
    ai:
      badge: AI Agent
      title: "AI handles: reasoning, creation & execution"
      items:
        - Drafting plans
        - Understanding specifications
        - Producing deliverables
        - Verifying outcomes
    afk:
      badge: AFK (Rust)
      title: "AFK handles: deterministic coordination"
      items:
        - Tracking state transitions
        - Enforcing task order & dependencies
        - Aggregating progress
        - Persisting local data
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
          role: State-aware workflow tools
        - name: Skills
          role: Execution guidance
        - name: Hooks
          role: Turn-end progress reports
      storageLabel: Project (.afk/)
      storage:
        - name: Markdown
          role: Content edited with native tools
        - name: SQLite
          role: State & dependency store
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
      link: /guide/
    cta:
      tagline: "Delegate work.\nStay in control."
      sub: A lightweight harness for AI agents to carry out work.
      primary:
        text: Get started with AFK
        link: /guide/
      secondary:
        text: GitHub
        link: https://github.com/FooQoo/agent-flow-kit
---

<LandingPage />
