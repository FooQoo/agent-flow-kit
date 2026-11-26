# AFK benchmarks

Run the deterministic workflow using the packaged executable or `cargo build --locked --release` followed by:

```sh
python3 tests/integration/stdio_workflow.py target/release/afk --trials 5
```

Each isolated trial plans a JSON fixture, asks for its format, accepts an additional request, restarts the server and verifies the exact value. The trial asserts the work product, acceptance guards, dependency order and Stop reports. Output gives model-facing tool definition and text byte counts, call counts, hook stdout bytes and process wall time. It includes deliberate invalid calls for assertions. It leaves model token counts unknown. Standard file-tool I/O and model context are outside these byte counters.

## Client trials

Use separate directories and fresh sessions with the same client build, model, reasoning setting, tool permissions and initial files. Rotate condition order across at least three trials per scenario. Compare the stock client, the same workflow instructions without persistence, AFK, and Agent Project where available. Use the script above as a deterministic preflight for AFK.

| Scenario | Instruction and intervention | Acceptance |
| --- | --- | --- |
| Simple | Write a Markdown conversion guide and a separate example input/output pair. | Both artifacts exist. The example follows the guide. |
| Clarification | Create a fixture for an importer with an unspecified input format. Reply “JSON with numeric value 42” when asked. | The agent asks only for the missing format. The saved fixture parses to the agreed value. |
| Additional request | Create a fixture and verifier for `value: 42`. After the fixture is written request rejection of all other values. | A valid fixture passes. The verifier rejects wrong values. The original result remains available. |
| Resume | Use the same fixture task. End the first session after the fixture is written. Reopen the directory and ask to continue. | The second session uses the saved work and completes verification. |
| Verification | Create a parser for signed integers. Request tests for empty input, a negative integer and nonnumeric input. | Tests assert agreed behavior and pass. Completion evidence names the checks actually run. |

For each trial record client/model versions, settings, outcome, checks passed, elapsed time, interventions and their duration, tool and model calls, progress reports seen, and matching DB snapshots. Record input/output/cache/reasoning counters when exposed by the client, plus source and certainty of attribution. Use null for unavailable values. Capture failed trials and their usage. For progress, inspect whether a Stop warning is visible, whether it causes an additional request, and whether it enters the next model input. Do not include prompt, document, or answer bodies in shared results.
