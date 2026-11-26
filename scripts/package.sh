#!/bin/sh
# Developer release assembly. It requires only the built binaries supplied by CI.
set -eu
if [ "$#" -lt 3 ] || [ "$#" -gt 4 ]; then
    printf '%s\n' 'usage: scripts/package.sh BIN_DIR OUTPUT_DIR host|all [git]' >&2
    exit 2
fi
case "$3" in
    host|all) mode=$3 ;;
    *) exit 2 ;;
esac
case "${4-}" in
    '') layout=archive ;;
    git) layout=git ;;
    *) exit 2 ;;
esac
source_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
binaries=$1
output=$2
mkdir -p "$output"
for client in claude-code codex; do
    if [ "$layout" = git ]; then
        stage="$output/plugins/$client"
    else
        stage="$output/$client/plugins/agent-flow-kit"
    fi
    mkdir -p "$stage/bin"
    cp -R "$source_root/plugins/$client/." "$stage/"
    cp -R "$source_root/plugins/shared/." "$stage/"
    cp "$source_root/THIRD_PARTY_NOTICES.txt" "$stage/"
    for binary in afk-macos afk-linux-x64 afk-linux-arm64; do
        if [ -f "$binaries/$binary" ]; then
            install -m 755 "$binaries/$binary" "$stage/bin/$binary"
        elif [ "$mode" = all ]; then
            printf 'Missing release binary %s\n' "$binary" >&2
            exit 1
        fi
    done
    chmod +x "$stage/run"
done
if [ "$layout" = git ]; then
    mkdir -p "$output/.claude-plugin" "$output/.agents/plugins"
    cp "$source_root/plugins/claude-code-marketplace.json" "$output/.claude-plugin/marketplace.json"
    cp "$source_root/plugins/codex-git-marketplace.json" "$output/.agents/plugins/marketplace.json"
else
    mkdir -p "$output/codex/.agents/plugins"
    cp "$source_root/plugins/codex-marketplace.json" "$output/codex/.agents/plugins/marketplace.json"
fi
