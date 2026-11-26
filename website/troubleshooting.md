---
title: Troubleshooting
description: Diagnose and safely recover from installation, MCP startup, hook, resume, and data consistency issues in Agent Flow Kit.
---

# Troubleshooting

*For users who hit an error or unexpected behavior and want to diagnose and recover safely without losing recorded work.*

## 1. Cannot install the plugin

- **Check:** Verify your OS/architecture (`macOS arm64/x86_64`, `Linux x86_64/arm64`), host client version, and network access to `https://github.com/FooQoo/agent-flow-kit.git`.
- **Fix:** Re-run the exact marketplace and install commands from [Installation](/guide/installation). If your network blocks GitHub, use the offline plugin archives from [GitHub Releases](https://github.com/FooQoo/agent-flow-kit/releases).
- **Healthy state:** The plugin appears in your client's installed plugin list and exposes `/agent-flow-kit:afk` (Claude Code) or `$agent-flow-kit:afk` (Codex).
- **Still stuck?** Open an issue on [GitHub Issues](https://github.com/FooQoo/agent-flow-kit/issues) with OS, client version, and command output.

## 2. MCP server does not start

- **Check:** On first launch for a version, `run` downloads the release archive and `SHA256SUMS` from GitHub Releases. Verify `curl` or `wget`, `tar`, and `shasum`/`sha256sum` are available, and that the host can reach `github.com`.
- **Fix:** Run `plugins/codex/run --help` (or inspect your client's MCP stderr log) to see whether a download or checksum error occurred. In air-gapped environments, use the pre-bundled archives from GitHub Releases so `bin/afk-*` sits beside `run`.
- **Healthy state:** The client lists the 6 AFK tools (`record_request`, `open_plan`, `update_plan`, `update_question`, `start_next`, `finish_task`).
- **Still stuck?** Report the stderr output (without secrets) in GitHub Issues.

## 3. The agent does not use AFK

- **Check:** Verify the plugin is enabled, the session was started *after* installation, and you invoked `/agent-flow-kit:afk` (Claude Code) or `$agent-flow-kit:afk` (Codex) or asked explicitly to use AFK.
- **Fix:** Start a fresh session and check whether another installed Skill conflicts with AFK's planning instructions. Remember that a Skill guides model behavior and does not force every turn to use AFK.
- **Healthy state:** The agent calls `record_request` or `open_plan` and creates/reads files under `.afk/`.
- **Still stuck?** Confirm the 6 MCP tools are visible in the client's tool list first.

## 4. Plan or tasks will not advance

- **Check:** Ask the agent to inspect `open_plan` (`progress.next`). Common blockers are:
  - `wait_for_answer`: an open question still needs your reply.
  - `await_start`: the plan is in `Backlog` waiting for your instruction to start.
  - `replan`: an additional request or answered question has not yet been committed into a new revision.
  - Prerequisite task not yet `Done`, or an incomplete `## Acceptance` / `## Result`.
- **Fix:** Answer any open question, tell the agent to start (`authorize`), or let it commit the pending revision.
- **Healthy state:** `progress.next` shows `start_task`, `resume_task`, or `verify_plan`.
- **Still stuck?** Inspect `.afk/plans/` and `.afk/questions/` directly to see what is still open.

## 5. Progress banner does not appear at turn end

- **Check:** Confirm client Hooks are enabled and trusted (in Codex, approve the `PostToolUse` and `Stop` hooks when prompted). Note that turn-end banners only fire on turns that called an AFK tool for a specific plan (`open_plan` without `plan_id` does not trigger a banner), and non-interactive modes such as `codex exec --json` omit hook banners in their JSON stream.
- **Fix:** Verify the plugin hooks are trusted and that the AFK binary has already been cached by an MCP call (un-cached `--hook` calls fail-open silently to stay under the 1-second hook timeout).
- **Healthy state:** After a turn that calls an AFK tool, a concise `[AFK] <title> [<Status>] · ██████░░░░ ...` summary and task list appear.
- **Still stuck?** Check whether `PLUGIN_DATA` / `CLAUDE_PLUGIN_DATA` is writable.

## 6. Cannot resume work from a new session

- **Check:** Ensure you opened the **same project root** containing `.afk/`. If you see `a different session owns this task`, the previous session left a task in `Doing`.
- **Fix:** Confirm the earlier agent session has actually stopped, then ask the agent to inspect `open_plan` (with `tasks: true`) and take over the task (`start_next` with `take_over: true`). If `AFK_NEEDS_WORKSPACE=1` is active, make sure the agent passes the absolute project path on its first call.
- **Healthy state:** `open_plan` returns your previous plan, handoff note (if saved), and remaining tasks.
- **Still stuck?** Check directory permissions on `.afk/` (it is created with mode `0700`).

## 7. Data consistency error (`confirmed text changed` or missing draft files)

- **Check:** Stop making further manual edits inside `.afk/`. Do **not** delete `.afk/afk.db`—copy the entire `.afk/` directory somewhere safe as a backup first (while no AFK session is running).
- **Fix:**
  - If a draft file is missing before commit, ask the agent to run `update_plan / recover` with the open `draft_id`.
  - If a confirmed plan or task file was edited after its SHA-256 hash was locked, save your intended edits outside `.afk/`, restore the confirmed text, and apply the change via a new plan revision (`prepare` + `commit`).
  - Use `open_plan` with `check_files: true` to list any unreferenced `.md` files in `.afk/`.
- **Healthy state:** `open_plan` succeeds without hash or missing-file errors.
- **Still stuck?** Keep your backup of `.afk/` and open a GitHub Issue with the exact error message.

## 8. Issues after updating AFK

- **Check:** Confirm the client loaded the updated plugin version and that the launcher downloaded the matching versioned binary (`v<version>/`).
- **Fix:** Restart the client session so the updated MCP server process starts fresh. If a download was interrupted, remove the version folder under your AFK cache directory and restart.
- **Healthy state:** The new session starts cleanly and `open_plan` reads existing plans.
- **Still stuck?** Check the release notes on [GitHub Releases](https://github.com/FooQoo/agent-flow-kit/releases).

## 9. Higher-than-expected token usage

- **Check:** Very long plans, large Markdown documents read repeatedly, or broad non-AFK context in the same session can dominate token usage.
- **Fix:** Keep task files concise, split unrelated work into separate plans, and use `/agent-flow-kit:afk handoff` (`$agent-flow-kit:afk` + `handoff`) to continue in a fresh session when context grows large.
- **Healthy state:** Each session stays focused on the active plan and current task.

## 10. Reporting a bug

When opening an issue on [GitHub Issues](https://github.com/FooQoo/agent-flow-kit/issues), include:

- AFK version, host client and version (Claude Code / Codex / other), and OS/architecture
- Steps to reproduce, expected behavior, and actual behavior
- Relevant error messages or logs **with secrets, personal paths, and private repository data removed**
