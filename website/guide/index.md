# Install Agent Flow Kit

AFK is distributed directly from the `FooQoo/agent-flow-kit` Git repository for Claude Code and Codex. On first MCP launch, the plugin launcher (`run`) downloads the matching macOS, Linux, or Windows (`x86_64`) binary from [GitHub Releases](https://github.com/FooQoo/agent-flow-kit/releases), verifies it against `SHA256SUMS`, and caches it locally. Network access is only needed on the first run for a given version.

## Claude Code

```sh
claude plugin marketplace add "https://github.com/FooQoo/agent-flow-kit.git"
claude plugin install agent-flow-kit@afk
```

Start a new Claude Code session in your project and run `/agent-flow-kit:afk` with your request.

## Codex

```sh
codex plugin marketplace add "https://github.com/FooQoo/agent-flow-kit.git"
codex plugin add agent-flow-kit@afk
```

Start a new conversation in your project and select `$agent-flow-kit:afk`. Review the plugin hooks if Codex prompts you.

## Antigravity and other stdio MCP clients

Clone `https://github.com/FooQoo/agent-flow-kit.git`. Set the MCP command to the absolute path to `plugins/codex/run` in that checkout and set `AFK_NEEDS_WORKSPACE=1` in the server environment. Add `plugins/codex/skills/afk/SKILL.md` to your client's skills and pass the project directory on the first AFK call.

## Offline / air-gapped installation

If your environment cannot reach GitHub Releases on first launch, download the pre-bundled plugin archives (`afk-claude-code.tar.gz` or `afk-codex.tar.gz`) and `SHA256SUMS` from [GitHub Releases](https://github.com/FooQoo/agent-flow-kit/releases). When `bin/afk-*` is already present beside `run`, AFK uses the bundled binary directly without network access.

[Try the quickstart](./quickstart.md)
