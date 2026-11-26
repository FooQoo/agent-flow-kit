---
title: Installation
description: Install Agent Flow Kit as a plugin for Claude Code, Codex, or another stdio MCP client.
---

# Installation

*For developers ready to install AFK in their coding agent and verify that MCP, Skills, and Hooks are active.*

AFK is designed for zero-runtime setup: add the plugin to your existing agent, open your project, and delegate work in natural language. No Rust toolchain, Homebrew formula, or separate server is required.

## Requirements

- **Operating System / Architecture**:
  - macOS (`arm64` / `x86_64` — universal binary `afk-macos`)
  - Linux (`x86_64` `afk-linux-x64`, `arm64`/`aarch64` `afk-linux-arm64`)
  - Windows (`x86_64` `afk-windows-x64.exe` — via Git Bash / POSIX `sh`, or direct binary from [GitHub Releases](https://github.com/FooQoo/agent-flow-kit/releases))
- **First-run network access**: On the first MCP launch for a given version, the plugin launcher (`run`) downloads the matching release archive and `SHA256SUMS` from [GitHub Releases](https://github.com/FooQoo/agent-flow-kit/releases), verifies the SHA-256 digest, and caches the binary locally. Subsequent launches run offline from cache.
- **Tools required by the launcher**: `curl` or `wget`, `tar`, and `shasum` or `sha256sum` (standard on macOS and Linux).
- **Supported hosts**: Claude Code, Codex, or any stdio MCP client.

## Claude Code

Add the AFK Git marketplace and install the plugin:

```sh
claude plugin marketplace add "https://github.com/FooQoo/agent-flow-kit.git"
claude plugin install agent-flow-kit@afk
```

1. Start a new Claude Code session inside your project directory.
2. Invoke `/agent-flow-kit:afk` followed by your request.
3. Claude Code supplies `CLAUDE_PROJECT_DIR` automatically, so AFK binds to the project root on startup and registers `PostToolUse`, `Stop`, and `UserPromptSubmit` hooks.

## Codex

Add the AFK Git marketplace and enable the plugin:

```sh
codex plugin marketplace add "https://github.com/FooQoo/agent-flow-kit.git"
codex plugin add agent-flow-kit@afk
```

1. Start a new conversation inside your project directory.
2. Select `$agent-flow-kit:afk` and send your request.
3. If Codex prompts you to trust the plugin hooks (`PostToolUse` and `Stop`), review and approve them so turn-end progress banners can run.
4. Codex runs the plugin MCP server with `AFK_NEEDS_WORKSPACE=1`; the bundled AFK skill instructs the agent to pass your project directory on its first AFK call.

## Verification

To verify that AFK is working in your project:

1. Start a new session in a sample or scratch repository.
2. Ask for a small plan only (for example: `/agent-flow-kit:afk Plan a small test fixture. Do not start yet.`).
3. Confirm that:
   - The agent calls AFK tools and creates `.afk/afk.db`, `.afk/requests/`, `.afk/plans/`, and `.afk/tasks/` inside the project root.
   - At the end of the turn, a concise `[AFK] ... · ██████░░░░ ...` progress message appears (when hooks are enabled in Claude Code or interactive Codex).

If anything does not appear as expected, see [Troubleshooting](/troubleshooting).

## Project initialization

- AFK creates `.afk/` at the resolved project root only when a session initializes a plan or opens `.afk/`.
- Root resolution follows a strict safety check: AFK refuses to initialize `.afk/` in `/`, `$HOME`, or inside the plugin installation directory itself.
- AFK does **not** modify your `.gitignore` automatically. Add `/.afk/` to `.gitignore` if you want to keep workflow state local to your machine, or track it explicitly if your team wants to share plans.

## Update

Update the plugin through your host client's marketplace update command (or `git pull` if using a manual clone). Because the launcher caches binaries under a versioned directory (`v<version>/`), upgrading to a new version automatically fetches and verifies the matching release binary on first launch.

Before upgrading across major schema changes, back up your `.afk/` directory while no AFK session is actively writing to it.

## Uninstall

- **Remove the plugin**: Uninstall `agent-flow-kit` using your host client's plugin commands.
- **Remove cached binaries** (optional): Delete the cached binary directory under your plugin data directory or `${XDG_CACHE_HOME:-$HOME/.cache}/agent-flow-kit/bin`.
- **Project data**: Uninstalling the plugin never deletes `.afk/` inside your repositories. Remove `.afk/` manually per project only if you no longer need the recorded plans, tasks, and results.

## Offline / air-gapped installation

If first-run access to GitHub Releases is unavailable, download the pre-bundled plugin archives (`afk-claude-code.tar.gz` or `afk-codex.tar.gz`) and `SHA256SUMS` from [GitHub Releases](https://github.com/FooQoo/agent-flow-kit/releases). When `bin/afk-*` is already present beside `run`, AFK executes the bundled binary directly without making any network requests.

## Manual MCP setup (Antigravity and other stdio MCP clients)

> **Note:** Connecting only the MCP server provides the 6 workflow tools, but does not automatically install client-specific lifecycle Hooks or Skills. Add the Skill manually for best results.

1. Clone the repository:
   ```sh
   git clone https://github.com/FooQoo/agent-flow-kit.git
   ```
2. Configure your MCP client to run the absolute path to `plugins/codex/run` over stdio, and set `AFK_NEEDS_WORKSPACE=1` in the server environment (or pass `--root /absolute/path/to/project` for a single-project setup).
3. Add `plugins/codex/skills/afk/SKILL.md` to your client's skills directory.

## Next steps

- [Quickstart](/guide/quickstart) — Run your first request from planning to completion.
- [Troubleshooting](/troubleshooting) — Fix common setup, hook, or workspace binding issues.
