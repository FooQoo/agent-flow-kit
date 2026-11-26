#!/bin/sh
set -eu
if [ "$#" -ne 2 ]; then
    printf '%s\n' 'usage: verify.sh TRIAL_ROOT repo|baseline|afk' >&2
    exit 2
fi
case "$2" in
    repo|baseline|afk) condition=$2 ;;
    *) exit 2 ;;
esac
source_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
trial_root=$1
checkout="$trial_root/$condition"
cd "$checkout"
export AGPJ_HOME="$trial_root/verification-data/$condition"
export CARGO_TARGET_DIR="$trial_root/verify-target"
export TZ=UTC
log_dir="$trial_root/logs"
binary_dir="$trial_root/verification-binaries/$condition"
mkdir -p "$AGPJ_HOME" "$binary_dir"
pnpm --dir web build > "$log_dir/$condition-verify-build.log" 2>&1
cargo build --offline --locked >> "$log_dir/$condition-verify-build.log" 2>&1
install -m 755 "$CARGO_TARGET_DIR/debug/agpj" "$binary_dir/agpj"
cargo test --offline --locked --test monitor_phone_serve --no-run --message-format=json     > "$log_dir/$condition-verify-phone-build.jsonl" 2>> "$log_dir/$condition-verify-build.log"
phone_backend=$(python3 - "$log_dir/$condition-verify-phone-build.jsonl" <<'PY'
import json
import sys
with open(sys.argv[1]) as log:
    for line in log:
        info = json.loads(line)
        if info.get("reason") == "compiler-artifact" and info.get("target", {}).get("name") == "monitor_phone_serve":
            print(info["executable"])
            break
PY
)
install -m 755 "$phone_backend" "$binary_dir/monitor_phone_serve"
spec=e2e/tests/afk-759-benchmark-verification.spec.ts
test ! -e "$spec"
cp "$source_dir/acceptance.spec.ts" "$spec"
trap 'rm "$spec"' EXIT
AGPJ_E2E_PREBUILT=1 AGPJ_E2E_BINARY="$binary_dir/agpj"     AGPJ_E2E_PHONE_BACKEND="$binary_dir/monitor_phone_serve"     pnpm --dir e2e exec playwright test --project=chromium --workers=1 "${spec##*/}"     > "$log_dir/$condition-verify-browser.log" 2>&1
