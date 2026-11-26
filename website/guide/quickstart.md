# Quickstart

[Install AFK](./index.md) and open a new agent session in the project where you want to work.

## Ask for a plan

In Claude Code, run `/agent-flow-kit:afk` followed by the message below. In Codex, select `$agent-flow-kit:afk` and send the same message.

> Plan a small JSON fixture and a test that checks its fields. Show me the plan before starting. Ask me which fields to include.

AFK stores the request and the plan in `.afk/` in your project. Answer any questions in chat and review the tasks and their completion criteria.

## Start and adjust the work

When the plan looks right, tell the agent to start. You can add a requirement while the work is running.

> Also add a test case for an empty name.

The agent updates the plan for the remaining work. Open the task's `## Result` in `.afk/tasks/` to see what it verified.

## Save a handoff and resume

To leave a note for the next session, run `/agent-flow-kit:afk handoff` in Claude Code. In Codex, select `$agent-flow-kit:afk` and send `handoff`.

After the working session has stopped, open a new one in the same project and ask AFK to resume the plan by number. The saved note shows what was checked and what to do next.
