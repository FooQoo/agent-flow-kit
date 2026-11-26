# Agent Flow Kit

[日本語](README.ja.md)

Agent Flow Kit (AFK) gives Claude Code, Codex, and other MCP clients a way to plan, track, and resume work. The plan, questions, and checked results stay in `.afk/` in your project.

- [Install AFK from Git](website/guide/index.md)
- [Try the quickstart](website/guide/quickstart.md)

## Work with AFK

Ask your agent for a plan before it starts. Review the tasks, answer questions in chat, and tell it when to begin. You can add requirements while it works and read the results in `.afk/tasks/`.

To leave a checkpoint for the next session, run `/agent-flow-kit:afk handoff` in Claude Code or choose `$agent-flow-kit:afk` and send `handoff` in Codex.

## Project documentation

- [Documentation site source](website/index.md)
- [Development and reference notes](docs/README.md)
- [Benchmarks](benchmarks/README.md)

## Source reuse

AFK's state enum and Markdown title helpers were adapted from [Agent Project](https://github.com/FooQoo/agent-project) at commit `095bc9bdad1c10cee9fb11ed25d616416e990167` under the MIT license. See [THIRD_PARTY_NOTICES.txt](THIRD_PARTY_NOTICES.txt).
