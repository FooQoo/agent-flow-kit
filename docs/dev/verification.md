# Verification and benchmarks

AFK pairs deterministic automated checks (covering core invariants, multi-threaded races, stdio JSON-RPC framing, and native hook edge cases) with a reproducible client evaluation protocol for measuring live-model adherence and token overhead.

## Verified environment summary

The baseline verification suite was executed on `2026-11-26` with **macOS `26.6.2` (Apple Silicon)**, **Rust `1.96.0`**, **Claude Code `2.1.331`**, and **Codex `0.171.0`**:

| Layer | How it was verified | Observed outcome |
| --- | --- | --- |
| **Core & persistence** | `cargo test --locked --workspace` | All unit and workflow integration tests pass (see full test breakdown below). |
| **Stdio MCP & `--hook`** | `python3 tests/integration/stdio_workflow.py target/release/afk --trials 5` | 5 consecutive release-binary trials passed all artifact, dependency, root-binding, session/turn isolation, duplicate `Stop`, and SQLite lock-contention checks. |
| **Claude Code (`2.1.331`)** | Extracted plugin loaded via `--plugin-dir` from a path containing spaces and Japanese characters | MCP server received requests; `Stop` emitted one informational report on the AFK turn and stayed silent on a follow-up non-AFK turn. |
| **Codex (`0.171.0`)** | Installed from local marketplace (`afk-local`); trusted `postToolUse` and `stop` hooks in temporary profile | Bundled skill enabled; MCP exposed all 6 tools; `Stop` report appeared in interactive terminal and `app-server` (`hook/completed`); follow-up unrelated turn stayed silent. |
| **Model context isolation** | Both real clients wired to scripted loopback HTTP model endpoints | AFK turn used 1 model response to invoke the tool and 1 to finish; `Stop` triggered **zero** extra model requests and its banner text was absent from the next turn's model input. |
| **Release packaging** | Extracted Apple Silicon archives (`dist/SHA256SUMS`); inspected with `otool -L` | `5,614,016`-byte binary bundles SQLite and links dynamically only to macOS system libraries. |

## Rust unit and workflow test suites

Run the entire Rust suite in under a second with:

```sh
cargo test --locked --workspace
```

The workspace contains **10 deterministic Rust tests** split across two targets:

### 1. Title extraction unit tests (`crates/core/src/plain_title.rs` — 5 tests)

| Test function | Invariants verified |
| --- | --- |
| `strips_emphasis_code_and_strikethrough` | Removes `**`, `__`, `*`, `_`, `` ` ``, and `~~` markers cleanly from inline text. |
| `strips_block_markers` | Strips nested leading `#` headings, `>` blockquotes, `- ` / `* ` / `+ ` bullets, and `12. ` / `1) ` ordered list markers while preserving `#hashtag` (no space after `#`). |
| `keeps_link_text_and_image_alt` | Unwraps `[the **docs**](url)` to `the docs` and `![shot](a.png)` to `shot`, while preserving unclosed `[open` literally. |
| `keeps_plain_characters` | Keeps intra-word underscores (`snake_case_name`) and spaced multiplication asterisks (`2 * 3 = 6`) intact. |
| `first_line_skips_blank_lines` | Skips leading whitespace-only lines, strips Markdown only on the first non-empty line, and returns `""` on blank documents. |

### 2. Core state-machine integration tests (`crates/core/tests/workflow.rs` — 5 tests)

| Test function | Invariants verified |
| --- | --- |
| `clarification_replan_dependency_and_resume` | End-to-end lifecycle: `receipt` idempotency on `record_request`; rejecting premature `authorize_execution`; rejecting unfilled draft commits; question `prepare` -> `publish` (`WaitForAnswer` blocking commit) -> `answer`; `Backlog` commit (`execute: false`) blocking `start_next` until `authorize_execution`; negative prerequisite ordering (`[vec![], vec![-1]]`); rejecting `finish_task` without `## Result`; mid-flight additional `record_request` blocking `finish_task`; replanning (`prepare_plan` retiring Task `#2` and adding Task `#3` depending on `#1`); rejecting a second open draft without `replace_draft`; rejecting completion of retired Task `#2`; preserving superseded `.md` files on disk; **renaming/moving the project directory (`fs::rename`)** and resuming under a new `Store::initialize` with relocated canonical paths; rejecting empty `verified: &[]`; and idempotent `finish_plan`. |
| `concurrent_workers_sealed_files_and_stale_attempts` | Tamper & race resistance: altering confirmed task prose above `## Result` blocks `start_next` until restored; two threads synchronized on a `std::sync::Barrier` racing `start_next(plan, owner, false)` result in **exactly one winner**; `start_next(..., take_over: true)` increments `attempt`; the superseded worker's `finish_task` with the stale `attempt` is rejected; and mutating a completed task's `## Result` afterward causes `finish_plan` to fail hash verification. |
| `rejects_symlinked_state_and_documents` *(Unix)* | Filesystem boundary: replacing `.afk` with a symlink to an external directory causes `Store::initialize` to fail; replacing `.afk/requests/001.md` with a symlink to an external file causes `commit_plan` to abort without modifying the external target. |
| `recovers_reserved_files_and_reports_unowned_documents` | Crash & backup safety: deleting a reserved draft task file and calling `recover_draft` recreates the missing file without overwriting already-written plan prose; creating an unreferenced `.afk/tasks/002.md` is surfaced by `orphan_documents()`, and `prepare_plan(..., replace_draft)` refuses to overwrite that unowned path while keeping the prior open draft intact. |
| `initial_templates_are_written_and_unfilled_documents_are_rejected` | Template guards: asserts exact initial contents of `REQUEST_TEMPLATE` (`"# \n"`), `WORK_TEMPLATE`, and `QUESTION_TEMPLATE`; verifies unfilled templates are rejected by `publish_question` and `commit_plan` (even when plan/task specs are filled if the request remains `# \n`); confirms replaying the same `receipt` preserves already-written request prose. |

## Stdio MCP & hook integration harness (`stdio_workflow.py`)

`tests/integration/stdio_workflow.py` exercises the compiled `afk` binary (or packaged `plugins/agent-flow-kit/run` shell wrapper) across real OS pipes and subprocesses:

```sh
cargo build --locked --release
python3 tests/integration/stdio_workflow.py target/release/afk --trials 5
```

Every trial runs inside a fresh temporary directory named `afk smoke 日本語 <random>`—deliberately combining ASCII spaces and multi-byte UTF-8 Japanese characters to test shell quoting in `plugins/shared/run`, `file://` URI decoding in MCP Roots, and canonical path comparisons in `hook.rs`.

### What the 21 MCP calls exercise per trial

Each trial spawns **two sequential `afk` MCP server processes** (`tools/list` + **21 `tools/call` requests**, including **3 deliberate error assertions**):

1. **Server Process 1 — Explicit `workspace` mode (`AFK_NEEDS_WORKSPACE=1`, launched outside `project/`)**:
   - Verifies neither the launch directory nor `project/` has `.afk/` before the first bound call.
   - **[Error check #1]** Calls `open_plan` *without* `workspace` -> asserts `isError: true` requiring `workspace`.
   - Calls `record_request(workspace=project, receipt="initial")` -> binds `project/`, creates Plan `#1` and Request `#1`.
   - Calls `update_plan(prepare, revision=0, add=[[], [-1]])` -> reserves Draft `#1` and sequential Tasks `#1` and `#2`.
   - Calls `update_question(prepare)` and `update_question(publish)` after writing `# Which output format?`.
   - **[Error check #2]** Calls `update_plan(commit, answers=[])` -> asserts `isError: true` because Question `#1` is unanswered/unapplied.
   - Appends `## Answer\nJSON.`, calls `update_question(answer)`, and calls `update_plan(commit, execute=false)` -> confirms Revision `1` in `Backlog`.
   - Calls `update_plan(authorize)` -> moves Plan `#1` to `Todo`.
   - Calls `start_next` -> claims Task `#1`, writes `project/fixture.json` (`{"value": 42}`), appends `## Result`, and calls `finish_task(verified=[1])`.
   - Calls `start_next` -> claims Task `#2`.
   - Calls `record_request(plan_id=1)` -> registers mid-flight Request `#2` (`"Also verify the exact value."`).
   - Calls `update_plan(prepare, revision=1, add=[[first.id]], remove=[second.id])` and `update_plan(commit)` -> retires Task `#2` and activates replacement Task `#3` at Revision `2`.
   - Calls `start_next` -> claims Task `#3` under Server Process 1's worker UUID, then closes Server Process 1 (simulating an interrupted session).

2. **Server Process 2 — Legacy MCP Roots & session takeover (`capabilities.roots` enabled, no `--root` or `workspace` arg)**:
   - Responds to AFK's `roots/list` JSON-RPC callback with `file:///.../project` and calls `open_plan(plan_id=1)`, asserting `revision == 2` and verifying `.afk/` was never created in the parent launch directory.
   - **[Error check #3]** Calls `start_next(plan_id=1)` with default `take_over=false` -> asserts `isError: true` (`"different session"`).
   - Calls `start_next(plan_id=1, take_over=true)` -> increments `attempt`, verifies `fixture.json` parses to `{"value": 42}`, writes `## Result`, and calls `finish_task`.
   - Appends `## Result` to the Revision `2` plan Markdown and calls `update_plan(finish, revision=2, verified=[1])` -> seals Plan `#1` as `Done`.

### What the 25 `--hook` invocations exercise per trial

Alongside the MCP calls, the harness spawns `afk --hook` **25 times** to verify every routing, isolation, and fail-open guarantee:

- **Silent no-op before routing**: Calling `Stop` before any `PostToolUse` emits nothing.
- **Cross-session & cross-turn isolation**: After routing `(session="codex-a", turn="1")`, calling `Stop` for `session="codex-b"` or `turn="other"` emits nothing and leaves `codex-a`'s marker intact.
- **Single-delivery atomic claim**: The matching `Stop` for `(codex-a, 1)` emits `"task count pending"`, and an immediate duplicate `Stop` for `(codex-a, 1)` emits nothing.
- **Claude Code envelope & `UserPromptSubmit` cleanup**: Routes a Claude-formatted content-array `PostToolUse` payload with `turn=None`, fires `UserPromptSubmit` (`turn=None`), and confirms the subsequent `Stop` is silent; then routes again and confirms `Stop` delivers once and silences duplicates.
- **Lifecycle banner accuracy**: Asserts `Stop` output across question waiting (`"Which output format?"`), `Backlog` gate (`"Waiting for an instruction to start"`), active work (`"In progress · Write the fixture"`), mid-flight request (`"1/2 complete"` + `"1 request(s)"`), and completion (`"2/2 complete"` + `"Plan complete"`).
- **Fail-open under SQLite `BEGIN EXCLUSIVE` lock**: Routes a valid marker, opens `.afk/afk.db` in Python, executes `BEGIN EXCLUSIVE` to hold a write lock, invokes `Stop`, and asserts `afk --hook` exits `0` with **empty stdout and empty stderr** (after its 100 ms read-only `busy_timeout`), and that after `ROLLBACK` a repeat `Stop` also stays silent because the marker was already atomically claimed.

## Baseline metrics (Apple Silicon)

Observed across 5 consecutive release-binary runs of `stdio_workflow.py` on macOS `26.6.2` (Apple Silicon, Rust `1.96.0`):

| Metric | Observed value | Notes |
| --- | --- | --- |
| **Release executable (`afk`)** | `5,614,016 bytes` (~5.61 MB) | `lto = "thin"`, `strip = true`, statically bundled SQLite. |
| **6 MCP tool schemas (`tools/list`)** | `5,557 bytes` | Compact JSON across all 6 tool definitions. |
| **Shared skill (`SKILL.md`)** | `2,627 bytes` | Loaded only when the user explicitly invokes AFK (`disable-model-invocation: true`). |
| **MCP tool call arguments (21 calls)** | `994 bytes` | Compact JSON sent to `tools/call` across the entire 2-session scenario. |
| **MCP tool text results (21 calls)** | `5,348 bytes` | Compact JSON / error strings returned by `afk` across all 21 calls. |
| **Hook `stdout` total (25 calls)** | `851 bytes` | Human-only `{"systemMessage": "..."}` JSON; **0 bytes** injected into model context. |
| **Per-hook invocation median** | `7.07 ms` | Median of per-trial medians, including OS process spawn and SQLite read. |
| **End-to-end trial wall time median** | `436 ms` | Covers 2 MCP server launches, 21 tool calls, file writes, 25 hook processes, and a 100 ms SQLite lock-timeout check. |

> [!NOTE]
> `stdio_workflow.py` reports `model_tokens: null` because it drives the protocol deterministically without a live LLM, and its byte counters measure MCP JSON-RPC traffic and hook stdout (excluding standard `Read`/`Write`/`Edit` file-tool I/O). Use the controlled client trial protocol below to measure live model token usage and adherence.

## Controlled 5-scenario client trial protocol

`benchmarks/README.md` defines the controlled experimental protocol for comparing four conditions:
1. **Stock client** (baseline agent with no workflow plugin or prompt)
2. **Prompt-only workflow** (same workflow instructions without AFK persistence/guards)
3. **AFK** (plugin + MCP server + native hooks)
4. **Agent Project** (where available on the target environment)

### Experimental controls

- Run every trial in a **separate directory** and **fresh client session** with the exact same client build, model ID, reasoning effort setting, tool permissions, and initial repository files.
- Run `stdio_workflow.py` first as a deterministic preflight check for the AFK binary.
- Rotate condition execution order across **at least 3 trials per scenario** to control for provider latency and caching variance.

### The 5 evaluation scenarios

| Scenario | Prompt & human intervention | Objective acceptance check |
| --- | --- | --- |
| **1. Simple** | Ask the agent to write a Markdown conversion guide and a separate example input/output pair. | Both artifacts exist on disk and the example conforms to every rule in the guide. |
| **2. Clarification** | Ask for an importer fixture whose input format is deliberately left unspecified. When asked, reply: *"JSON with numeric value 42"*. | The agent asks **only** for the missing format decision (no redundant questions), and the saved fixture parses to `42`. |
| **3. Additional request** | Ask for a fixture and verifier for `value: 42`. After the fixture is written, add a request to reject all other values. | Valid fixture passes, verifier rejects wrong values, and the original completed task artifact/evidence remains preserved. |
| **4. Resume** | Start the fixture task and terminate the first session right after the fixture is written. Open a fresh session in the same directory and ask to continue. | The second session inspects `.afk/`, reuses the saved fixture without redoing completed work, and finishes verification. |
| **5. Verification** | Ask for a signed-integer parser plus tests covering empty input, a negative integer, and non-numeric input. | All requested edge-case tests exist and pass, and `## Result` cites the actual verification commands executed. |

### Automated repository harness example (`benchmarks/task-759/`)

For larger real-codebase comparisons on Codex, `benchmarks/task-759/` provides an end-to-end harness against an Agent Project worktree task:

- **`benchmarks/task-759/run.py`**: Drives `codex app-server --stdio` with isolated `HOME`, `CODEX_HOME`, `AGPJ_HOME`, `TZ=UTC`, and disabled web search/multi-agent flags. Supports `probe` (verifying `agent-flow-kit@afk-local` hooks and skill discovery and writing hook trust hashes to `config.toml`), `baseline`, and `afk` runs, logging JSONL wire events plus summary JSON (`usage` from `thread/tokenUsage/updated`, `afk_calls`, `afk_stop_reports` from `hook/completed`, and `elapsed_seconds`).
- **`benchmarks/task-759/verify.sh` & `acceptance.spec.ts`**: Builds the trial checkout offline (`pnpm --dir web build` + `cargo build --offline --locked`) and runs Playwright (`chromium`, desktop `1280x900` and phone `390x844` viewports) to verify UI/CLI behavior objectively outside the agent session.

### Required trial telemetry and privacy rules

For every client trial—including **failed or aborted trials**—record:

- **Environment & config**: Client version, model ID, reasoning effort, sandbox/approval settings.
- **Outcome & quality**: Completion status, independent acceptance checks passed/failed, total elapsed wall time, human interventions (count and duration), total model turns, and MCP/standard tool call counts.
- **Hook behavior**: Number of `Stop` progress banners observed, confirmation that `Stop` caused zero extra model requests, and confirmation that `Stop` text did not enter the next model input.
- **Database state**: Matching `.afk/afk.db` metadata snapshots (plan status, revision, task counts, history events).
- **Token usage**: Input, output, cached-input, and reasoning token counters when exposed by the client, alongside the **source** (e.g., `app-server` event vs provider dashboard) and **certainty of attribution**. Record `null`—never `0` or estimates—when a counter is unavailable.
- **Redaction rule**: Never include raw user prompts, Markdown specification bodies, or human answer text in shared benchmark results.

## Current release gates

While local Apple Silicon packages are verified for developer trials, the following gates remain open before public release:

| Gate | Current status & required action |
| --- | --- |
| **Full CI matrix & Linux / Intel verification** | `.github/workflows/ci.yml` defines Linux (`x86_64`, `aarch64`) and macOS (`arm64`, `x86_64` -> Universal `lipo`) jobs; end-to-end execution across all non-arm64-macOS runners must be completed and archived. |
| **Desktop UI surface checks** | CLI and `codex app-server` surfaces are verified; native desktop GUI rendering of hook `systemMessage` outputs (and documenting `codex exec --json`'s omission of hook reports) needs final sign-off. |
| **Live-model adherence & token study** | Complete multi-trial runs of the 5-scenario protocol comparing stock clients, prompt-only instructions, AFK, and Agent Project with live token telemetry. |
| **Code signing & notarization** | CI currently applies ad-hoc `codesign --sign -` to `afk-macos`; public macOS distribution requires Developer ID signing and Apple notarization. |
| **Project-wide OSS license** | `THIRD_PARTY_NOTICES.txt` covers MIT-licensed Agent Project adaptations, but AFK's root `LICENSE` is still pending decision (flagged by Claude Code's plugin validator for public submission). |
| **Name clearance & marketplace publishing** | Finalize package/product naming clearance and public marketplace listing policies for Claude Code and Codex. |

## Related pages

- [Build and package](index.md) — Workspace code map, local `scripts/package.sh` usage, and CI archive assembly.
- [Architecture and storage](architecture.md) — SQLite schema, SHA-256 section sealing, and state transition walkthroughs.
- [Hooks, roots, and security](integrations.md) — Root resolution order, `hook.rs` sequence diagram, and `O_NOFOLLOW` security guards.
