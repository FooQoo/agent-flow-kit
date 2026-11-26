# Workspace configuration and state

See the [installation guide](../../website/guide/index.md) for plugin setup.

## How AFK picks your project folder

Each `afk` server process binds to **one project folder** for its lifetime. It checks these sources in order:

| Order | Source | Typical setup |
| --- | --- | --- |
| **1** | `--root <directory>` CLI argument | Per-project MCP config |
| **2** | `AFK_PROJECT_ROOT` environment variable | Scripts or CI |
| **3** | `CLAUDE_PROJECT_DIR` environment variable | Claude Code plugin |
| **4** | `workspace` field on `record_request` or `open_plan` | Codex plugin, Antigravity global setup (`AFK_NEEDS_WORKSPACE=1`) |
| **5** | MCP `Roots` capability (`roots/list`, 800 ms timeout) | MCP clients that expose one local `file://` root |
| **6** | Process launch directory (`cwd`, only if `AFK_NEEDS_WORKSPACE` is unset) | Running `afk` directly inside a repo |

### Folder safety rules

To avoid creating `.afk/` in the wrong place, AFK refuses unsafe directories:

- **Blocks root, home, and plugin cache folders:** `/` (no parent), `HOME` / `USERPROFILE`, and paths inside `PLUGIN_ROOT` / `CLAUDE_PLUGIN_ROOT` are rejected with `"no project directory; configure AFK_PROJECT_ROOT"`.
- **Checks multiple MCP roots:** If a client reports multiple MCP roots, AFK uses `cwd` only if it matches one of them; otherwise it asks for `AFK_PROJECT_ROOT`.
- **One folder per session:** Once bound to Project A, passing Project B as `workspace` in the same session returns an error. Start a new session when switching projects.

---

## Versioning `.afk/`

AFK stores everything for a project inside `.afk/` (using `0700` folder permissions and `0600` file permissions on Unix):

```text
.afk/
├── afk.db              # SQLite states, order, hashes, and history
├── requests/NNN.md     # Your recorded instructions
├── plans/NNN-rMMM.md   # Plan specs and results for each revision
├── tasks/NNN.md        # Task specs, acceptance bullets, and results
└── questions/NNN.md    # Questions and your answers
```

Whether to commit `.afk/` to Git is up to your project:

| Choice | Best when | Keep in mind |
| --- | --- | --- |
| **Add `.afk/` to `.gitignore`** | You use AFK as a personal tool and want Git history to hold only code and tests. | Leave `.afk/` in your working folder so you can resume plans across sessions. |
| **Commit `.afk/` to Git** | Teammates hand off plans, or reviewers want to read the plan and test evidence in pull requests. | Always commit **the whole `.afk/` folder together** (`afk.db` + all `.md` files), and do not paste secrets into requests or `## Result`. |

> [!IMPORTANT]
> `afk.db` and the Markdown files check each other with SHA-256 hashes. If `.afk/*.md` files exist without a matching `afk.db`, AFK stops rather than overwriting them. Always copy, commit, or restore `.afk/` as one complete folder.

Next, walk through a complete example in the **[Quickstart](quickstart.md)**.
