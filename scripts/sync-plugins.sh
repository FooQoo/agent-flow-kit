#!/bin/sh
# Sync shared plugin files and root marketplace manifests for single-repo distribution.
# Usage: scripts/sync-plugins.sh [--check]
set -eu

mode=sync
case "${1-}" in
    '') mode=sync ;;
    --check) mode=check ;;
    *)
        printf '%s\n' 'usage: scripts/sync-plugins.sh [--check]' >&2
        exit 2
        ;;
esac

if [ "$#" -gt 1 ]; then
    printf '%s\n' 'usage: scripts/sync-plugins.sh [--check]' >&2
    exit 2
fi

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
status=0

sync_file() {
    src=$1
    dst=$2
    exec_mode=${3:-0}

    if [ "$mode" = sync ]; then
        mkdir -p "$(dirname -- "$dst")"
        cp "$src" "$dst"
        if [ "$exec_mode" -eq 1 ]; then
            chmod 755 "$dst"
        else
            chmod 644 "$dst"
        fi
    else
        if [ ! -f "$dst" ]; then
            printf 'Missing synced file: %s\n' "$dst" >&2
            status=1
        elif ! cmp -s "$src" "$dst"; then
            printf 'Out of sync: %s (expected copy of %s)\n' "$dst" "$src" >&2
            status=1
        elif [ "$exec_mode" -eq 1 ] && [ ! -x "$dst" ]; then
            printf 'Not executable: %s\n' "$dst" >&2
            status=1
        fi
    fi
}

sync_file "$root/plugins/claude-code-marketplace.json" "$root/.claude-plugin/marketplace.json" 0
sync_file "$root/plugins/codex-git-marketplace.json" "$root/.agents/plugins/marketplace.json" 0

for client in claude-code codex; do
    sync_file "$root/plugins/shared/run" "$root/plugins/$client/run" 1
    sync_file "$root/plugins/shared/README.md" "$root/plugins/$client/README.md" 0
    sync_file "$root/plugins/shared/skills/afk/SKILL.md" "$root/plugins/$client/skills/afk/SKILL.md" 0
    sync_file "$root/THIRD_PARTY_NOTICES.txt" "$root/plugins/$client/THIRD_PARTY_NOTICES.txt" 0
done

# Verify version consistency across Cargo.toml, plugin manifests, and run script (if pinned).
cargo_version=$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$root/Cargo.toml" | head -n 1)
if [ -z "$cargo_version" ]; then
    printf 'Could not read version from Cargo.toml\n' >&2
    exit 1
fi

check_json_version() {
    file=$1
    ver=$(sed -n 's/.*"version"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' "$file" | head -n 1)
    if [ "$ver" != "$cargo_version" ]; then
        printf 'Version mismatch in %s: got %s, expected %s\n' "$file" "${ver:-<empty>}" "$cargo_version" >&2
        status=1
    fi
}

check_json_version "$root/plugins/claude-code/.claude-plugin/plugin.json"
check_json_version "$root/plugins/codex/.codex-plugin/plugin.json"

run_version=$(sed -n 's/^AFK_DEFAULT_VERSION="\([^"]*\)"/\1/p' "$root/plugins/shared/run" | head -n 1)
if [ -n "$run_version" ] && [ "$run_version" != "$cargo_version" ]; then
    printf 'Version mismatch in plugins/shared/run: got %s, expected %s\n' "$run_version" "$cargo_version" >&2
    status=1
fi

exit "$status"
