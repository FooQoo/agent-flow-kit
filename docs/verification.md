# Verification

Verified on 2026-11-26 with macOS 26.6.2 (Apple Silicon), Rust 1.96.0, Claude Code 2.1.331, and Codex 0.171.0.

| Check | Observed result |
| --- | --- |
| Core and persistence | Nine Rust tests passed. Covered Backlog authorization, clarification, ordered dependencies, adding a request during work, replan rollback, completed result preservation, concurrent claims, stale attempts, project relocation, draft recovery, orphan detection, and symlink rejection. |
| MCP and Stop | The stdio scenario passed five consecutive release-binary trials. It checks the saved artifact and plan completion, legacy Roots and explicit workspace binding, session and turn isolation, duplicate Stop, and fail-open behavior while SQLite is locked. |
| Claude Code | The extracted plugin loaded from a path with spaces and Japanese characters. The bundled MCP server received the test request. Stop emitted one informational report. A follow-up unrelated turn was silent. |
| Codex | The extracted local marketplace installed; the bundled skill was enabled and the server exposed six tools. After trusting the two plugin hooks in the temporary test profile, a test request produced a Stop warning in app-server and a visible report in the interactive terminal. A follow-up unrelated app-server turn was silent. |
| Model interaction | Both real clients were connected to scripted loopback endpoints for the native tests. The AFK turn used one model response to call the tool and one to finish. The scripts observed no additional model request caused by Stop, and the report text was absent from the next model input. |
| Packaging | Both Apple Silicon archives were extracted and tested. The release executable is 5,614,016 bytes and dynamically links only to macOS system libraries. SQLite is bundled. Local archive checksums are in `dist/SHA256SUMS`. |

The Codex package uses the compatibility manifest and a plugin-relative working directory. On Codex 0.171.0, the portable manifest exposed the MCP tools but left its lifecycle hooks out of the hook list. With the compatibility manifest, both are discoverable. The interactive terminal displays the Stop report. The `codex exec --json` adapter did not include it in the tested JSON output, so consumers that need the report event can use app-server. [Codex hook specification](https://learn.chatgpt.com/docs/hooks)

## Basic benchmark

Five release-binary trials passed all fixture and state assertions. Each trial used 21 MCP calls, including deliberate error checks, and 25 hook invocations, including scope and lock checks. The six tool definitions used 5,557 bytes of compact JSON. Tool arguments used 994 bytes and tool text results used 5,348 bytes on the test paths. Hook stdout was 851 bytes. The shared skill was 2,627 bytes.

Median elapsed time was 436 ms across the five trials. The median of the per-trial hook process medians was 7.07 ms, including process launch. The end-to-end timing covers two server launches, local file operations, and intentional busy-timeout testing. The synthetic model endpoints used fixed responses. Real input, output, cache and reasoning tokens and comparisons with stock clients or Agent Project are pending.

## Release gates

The full CI matrix, macOS Intel / Universal and Linux builds, desktop UI behavior, live-model workflow adherence, token comparison, signing and notarization, license, name clearance and public distribution still need verification or decisions. The Apple Silicon packages are usable for local trials. Claude Code’s directory validator accepted the local package and flagged the absent project-wide license for public submission.
