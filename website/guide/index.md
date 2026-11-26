# Install Agent Flow Kit

AFK is available as a Git marketplace for Claude Code and Codex. The plugin includes the AFK binary for macOS and Linux. Use the Git repository URL provided with the distribution in place of `YOUR_GIT_MARKETPLACE_URL`.

## Claude Code

```sh
claude plugin marketplace add "YOUR_GIT_MARKETPLACE_URL"
claude plugin install agent-flow-kit@afk
```

Start a new Claude Code session in your project and run `/agent-flow-kit:afk` with your request.

## Codex

```sh
codex plugin marketplace add "YOUR_GIT_MARKETPLACE_URL"
codex plugin add agent-flow-kit@afk
```

Start a new conversation in your project and select `$agent-flow-kit:afk`. Review the plugin hooks if Codex prompts you.

## Antigravity and other stdio MCP clients

Clone the same Git marketplace. Set the MCP command to the absolute path to `plugins/codex/run` in that checkout and set `AFK_NEEDS_WORKSPACE=1` in the server environment. Add `plugins/codex/skills/afk/SKILL.md` to your client's skills and pass the project directory on the first AFK call.

[Try the quickstart](./quickstart.md)
