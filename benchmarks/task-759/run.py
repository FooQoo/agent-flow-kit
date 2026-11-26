"""Run a local Codex comparison against the same Agent Project worktree base."""

import argparse
from datetime import datetime
import json
import os
from pathlib import Path
import select
import subprocess
import time

REQUEST = (
    "カンバンのモード選択はなくしたいです。設定した時間帯に入ったら、"
    "スリープ中だと画面で分かるようにしてください。"
    "時間帯の設定欄も、今よりコンパクトにしてほしいです。"
)
SCOPE = (
    "このworktree内で実装と検証を行ってください。比較実験なので、"
    "Agent Projectの管理用Pluginや既存カンバンを使ったタスク管理、commit、push、PR作成は行わないでください。"
    "製品を検証する場合は必ず一時的なAGPJ_HOMEとループバックの非標準ポートを使ってください。"
)
PLUGIN = "agent-flow-kit@afk-local"


class AppServer:
    def __init__(self, worktree, profile, log, active_trial):
        self.log = log
        self.buffer = b""
        self.request_number = 0
        self.latest_usage = None
        self.hook_reports = 0
        self.afk_calls = 0
        self.turns = {}
        root = profile.parent
        condition = worktree.name
        env = {
            "HOME": str(root / f"{condition}-runtime"),
            "CODEX_HOME": str(profile),
            "AGPJ_HOME": str(profile / "test-project-data"),
            "PATH": os.environ["BENCHMARK_RUNTIME_PATH"],
            "LANG": "en_US.UTF-8",
            "GIT_CONFIG_GLOBAL": "/dev/null",
            "GIT_CONFIG_SYSTEM": "/dev/null",
            "TZ": "UTC",
        }
        command = ["codex", "app-server", "--stdio", "--disable", "remote_plugin", "--disable", "multi_agent"]
        if active_trial:
            command.extend(["-c", 'web_search="disabled"'])
        self.process = subprocess.Popen(
            command,
            cwd=worktree,
            env=env,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            bufsize=0,
        )

    def send(self, item):
        self.process.stdin.write((json.dumps(item, ensure_ascii=False) + "\n").encode())

    def read(self, timeout=420):
        while b"\n" not in self.buffer:
            if not select.select([self.process.stdout], [], [], timeout)[0]:
                raise TimeoutError("No app-server activity within timeout")
            data = os.read(self.process.stdout.fileno(), 65536)
            if not data:
                raise RuntimeError(f"app-server exited: {self.process.poll()}")
            self.buffer += data
        line, self.buffer = self.buffer.split(b"\n", 1)
        item = json.loads(line)
        if self.log:
            self.log.write(json.dumps(item, ensure_ascii=False) + "\n")
            self.log.flush()
        return item

    def request(self, method, params):
        self.request_number += 1
        number = self.request_number
        self.send({"id": number, "method": method, "params": params})
        while True:
            item = self.read()
            if item.get("id") == number and "method" not in item:
                if "error" in item:
                    raise RuntimeError(f"{method}: {item['error']}")
                return item["result"]
            self.observe(item)

    def observe(self, item):
        method = item.get("method")
        params = item.get("params", {})
        if "id" in item and method:
            print(f"Client request needs attention: {method}", flush=True)
            if method in {"item/commandExecution/requestApproval", "item/fileChange/requestApproval"}:
                self.send({"id": item["id"], "result": {"decision": "decline"}})
            else:
                self.send({"id": item["id"], "error": {"code": -32601, "message": "No interactive reply supplied by this benchmark driver"}})
        if method == "thread/tokenUsage/updated":
            self.latest_usage = params.get("tokenUsage", {}).get("total")
        if method == "turn/completed":
            turn = params.get("turn", {})
            self.turns[turn.get("id")] = turn
        if method == "item/completed":
            completed = params.get("item", {})
            if completed.get("type") == "mcpToolCall" and completed.get("server") == "afk":
                self.afk_calls += 1
        if method == "hook/completed":
            run = params.get("run", {})
            if run.get("eventName") == "stop" and any(
                entry.get("text", "").startswith("AFK · ") for entry in run.get("entries", [])
            ):
                self.hook_reports += 1

    def initialize(self):
        self.request("initialize", {
            "clientInfo": {"name": "afk-benchmark", "version": "0.1.0"},
            "capabilities": {"experimentalApi": True},
        })
        self.send({"method": "initialized"})

    def close(self):
        self.process.stdin.close()
        try:
            self.process.wait(timeout=4)
        except subprocess.TimeoutExpired:
            self.process.terminate()
            self.process.wait(timeout=5)
        self.process.stdout.close()


def trust_test_hooks(profile, hooks):
    allowed = {name for name in ("postToolUse", "stop")}
    definitions = [entry for entry in hooks if entry.get("pluginId") == PLUGIN]
    if {entry["eventName"] for entry in definitions} != allowed:
        raise RuntimeError("AFK hooks did not load in the test profile")
    path = profile / "config.toml"
    if all(entry.get("trustStatus") == "trusted" for entry in definitions):
        return
    with path.open("a", encoding="utf-8") as file:
        for entry in definitions:
            declaration, content = entry["currentHash"].split("/")
            file.write(f'\n[hooks.state.{json.dumps(entry["key"])}]\n')
            file.write(f'trusted_hash = "{declaration}"\ntrusted_content_hash = "{content}"\n')


def probe(trial_root):
    model_catalog = None
    for condition in ("baseline", "afk"):
        worktree = trial_root / condition
        profile = trial_root / f"{condition}-codex"
        app = AppServer(worktree, profile, None, False)
        try:
            app.initialize()
            models = app.request("model/list", {"limit": 25}).get("data", [])
            catalog = [{k: model.get(k) for k in ("model", "id", "isDefault", "defaultReasoningEffort")}
                       for model in models]
            if condition == "baseline":
                model_catalog = catalog
            hooks = app.request("hooks/list", {"cwds": [str(worktree)]})["data"][0]["hooks"]
            ids = sorted({entry["pluginId"] for entry in hooks if entry.get("pluginId")})
            expected = ["agent-flow-kit@afk-local"] if condition == "afk" else []
            if ids != expected:
                raise RuntimeError(f"unexpected hook plugins for {condition}: {ids}")
            if condition == "afk":
                trust_test_hooks(profile, hooks)
                listing = app.request("skills/list", {"cwds": [str(worktree)]})["data"][0]["skills"]
                skill = next((item for item in listing if item.get("pluginId") == PLUGIN and item["enabled"]), None)
                if not skill:
                    raise RuntimeError("AFK skill missing")
                (trial_root / "skill.json").write_text(json.dumps({"type": "skill", "name": skill["name"], "path": skill["path"]}))
            print(f"{condition}: plugins for hooks = {ids}", flush=True)
        finally:
            app.close()
    print(json.dumps(model_catalog, ensure_ascii=False, indent=2))


def run(trial_root, condition, model, effort):
    worktree = trial_root / condition
    profile = trial_root / f"{condition}-codex"
    log_path = trial_root / "logs" / f"{condition}.jsonl"
    started = time.monotonic()
    with log_path.open("x", encoding="utf-8") as log:
        app = AppServer(worktree, profile, log, True)
        try:
            app.initialize()
            thread = app.request("thread/start", {
                "cwd": str(worktree),
                "model": model,
                "approvalPolicy": "never",
                "sandbox": "workspace-write",
                "ephemeral": True,
            })["thread"]
            print(f"{condition}: thread {thread['id']} model {model}, effort {effort}", flush=True)
            inputs = [{"type": "text", "text": REQUEST + "\n\n" + SCOPE}]
            if condition == "afk":
                inputs.append({"type": "text", "text": (
                    "この試行では Agent Flow Kit (AFK) を明示的に使ってください。"
                    "AFK の MCP で依頼受付から結果記録まで進め、終了時に進捗が表示されるようにしてください。"
                    "使わない管理用Pluginは Agent Project のものだけです。"
                )})
                inputs.append(json.loads((trial_root / "skill.json").read_text()))
            turn = app.request("turn/start", {
                "threadId": thread["id"], "input": inputs, "model": model, "effort": effort,
            })["turn"]
            while turn["id"] not in app.turns:
                app.observe(app.read())
            completed = app.turns[turn["id"]]
            result = {
                "condition": condition,
                "model": model,
                "effort": effort,
                "status": completed.get("status"),
                "error": completed.get("error"),
                "usage": app.latest_usage,
                "afk_calls": app.afk_calls,
                "afk_stop_reports": app.hook_reports,
                "elapsed_seconds": round(time.monotonic() - started, 1),
                "finished_at": datetime.now().astimezone().isoformat(timespec="seconds"),
            }
            (trial_root / "logs" / f"{condition}.json").write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n")
            print(json.dumps(result, ensure_ascii=False), flush=True)
        finally:
            app.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("trial_root", type=Path)
    parser.add_argument("condition", choices=("probe", "baseline", "afk"))
    parser.add_argument("--model")
    parser.add_argument("--effort", default="medium")
    args = parser.parse_args()
    if args.condition == "probe":
        probe(args.trial_root.resolve())
    else:
        if not args.model:
            parser.error("--model is required for a trial")
        run(args.trial_root.resolve(), args.condition, args.model, args.effort)


if __name__ == "__main__":
    main()
