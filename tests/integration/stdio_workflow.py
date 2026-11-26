"""AFK smoke test and deterministic protocol benchmark (Python is test-only)."""

import argparse
import json
import os
from pathlib import Path
import selectors
import sqlite3
import subprocess
import sys
import tempfile
import time


class Mcp:
    def __init__(self, binary, root, launch_dir, workspace_required=False, roots=False, name="afk-smoke"):
        environment = os.environ.copy()
        for variable in ("AFK_PROJECT_ROOT", "AFK_NEEDS_WORKSPACE", "CLAUDE_PROJECT_DIR"):
            environment.pop(variable, None)
        if workspace_required:
            environment["AFK_NEEDS_WORKSPACE"] = "1"
        command = [str(binary)]
        if not workspace_required and not roots:
            command.extend(("--root", str(root)))
        self.process = subprocess.Popen(
            command, cwd=launch_dir, env=environment,
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, bufsize=0,
        )
        self.root = root
        self.registered_roots = [root.as_uri()] if roots else []
        self.buffer = b""
        self.next_id = 0
        self.calls = 0
        self.input_bytes = 0
        self.output_bytes = 0
        self.selector = selectors.DefaultSelector()
        self.selector.register(self.process.stdout, selectors.EVENT_READ)
        initialized = self.rpc("initialize", {
            "protocolVersion": "2025-11-25",
            "capabilities": {"roots": {}} if roots else {},
            "clientInfo": {"name": name, "version": "1.0.0"},
        })
        assert initialized["serverInfo"]["name"] == "agent-flow-kit", initialized
        self.write({"jsonrpc": "2.0", "method": "notifications/initialized"})

    def write(self, message):
        self.process.stdin.write(json.dumps(message).encode() + b"\n")

    def read(self):
        while b"\n" not in self.buffer:
            assert self.selector.select(10), "timed out reading AFK stdio"
            block = os.read(self.process.stdout.fileno(), 65536)
            assert block, "AFK closed stdio"
            self.buffer += block
        wire, self.buffer = self.buffer.split(b"\n", 1)
        return json.loads(wire)

    def rpc(self, method, params):
        self.next_id += 1
        self.write({"jsonrpc": "2.0", "id": self.next_id, "method": method, "params": params})
        while True:
            message = self.read()
            if message.get("method") == "roots/list":
                self.write({"jsonrpc": "2.0", "id": message["id"], "result": {
                    "roots": [{"uri": uri} for uri in self.registered_roots],
                }})
                continue
            assert message.get("id") == self.next_id and "error" not in message, message
            return message["result"]

    def tool(self, tool, error=False, **args):
        self.calls += 1
        result = self.rpc("tools/call", {"name": tool, "arguments": args})
        value = result["content"][0]["text"]
        self.input_bytes += compact(args)
        self.output_bytes += len(value.encode())
        assert bool(result.get("isError")) == error, (tool, value)
        return value if error else json.loads(value)

    def close(self):
        self.process.stdin.close()
        try:
            assert self.process.wait(timeout=5) == 0
        finally:
            if self.process.poll() is None:
                self.process.kill()
                self.process.wait()
            self.selector.close()
            self.process.stdout.close()

    def __enter__(self):
        return self

    def __exit__(self, *exc):
        self.close()


def compact(value):
    return len(json.dumps(value, separators=(",", ":"), ensure_ascii=False).encode())


def write(path, text):
    Path(path).write_text(text, encoding="utf-8")


def work(title):
    return f"# {title}\n\n## Acceptance\n- Verify the example output.\n"


def result(path):
    with open(path, "a", encoding="utf-8") as file:
        file.write("\n## Result\nVerified against the fixture.\n")


def workflow(binary):
    started = time.perf_counter()
    with tempfile.TemporaryDirectory(prefix="afk smoke 日本語 ") as directory:
        root = Path(directory).resolve()
        project = root / "project"
        project.mkdir()
        hook_dir = root / "hook data"
        environment = {**os.environ, "AFK_HOOK_DIR": str(hook_dir)}
        hook_runs = []

        def hook(event, session="codex-a", turn="1", payload=None, tool="record_request", cwd=project):
            data = {"hook_event_name": event, "session_id": session, "cwd": str(cwd)}
            if turn is not None:
                data["turn_id"] = turn
            if payload is not None:
                data["tool_name"] = f"mcp__afk__{tool}"
                data["tool_response"] = payload
            tick = time.perf_counter()
            report = subprocess.run(
                [binary, "--hook"], input=json.dumps(data), text=True,
                capture_output=True, env=environment, timeout=2, check=True,
            )
            hook_runs.append((1000 * (time.perf_counter() - tick), len(report.stdout.encode())))
            assert not report.stderr, report.stderr
            if report.stdout:
                value = json.loads(report.stdout)
                assert list(value) == ["systemMessage"], value
                return value["systemMessage"]
            return ""

        def route(data, session="codex-a", turn="1", claude=False):
            content = [{"type": "text", "text": json.dumps(data)}]
            payload = content if claude else {"content": content, "isError": False}
            assert not hook("PostToolUse", session, turn, payload)

        assert not hook("Stop")
        with Mcp(binary, project, root, workspace_required=True) as mcp:
            tools = mcp.rpc("tools/list", {})["tools"]
            assert len(tools) == 6 and all(x["inputSchema"]["type"] == "object" for x in tools)
            assert not (root / ".afk").exists() and not (project / ".afk").exists()
            assert "workspace" in mcp.tool("open_plan", error=True)
            intake = mcp.tool("record_request", workspace=str(project), receipt="initial")
            plan = intake["plan_id"]
            route(intake)
            assert not hook("Stop", "codex-b")
            assert not hook("Stop", turn="other")
            assert "task count pending" in hook("Stop")
            assert not hook("Stop")
            write(intake["request"]["path"], "Create the example fixture. Plan first.")
            route(intake, "claude-a", None, claude=True)
            assert not hook("UserPromptSubmit", "claude-a", None)
            assert not hook("Stop", "claude-a", None)
            route(intake, "claude-a", None, claude=True)
            assert hook("Stop", "claude-a", None)
            assert not hook("Stop", "claude-a", None)

            def change(op, error=False, **fields):
                return mcp.tool("update_plan", error, plan_id=plan, change={"op": op, **fields})

            def question(op, **fields):
                return mcp.tool("update_question", plan_id=plan, change={"op": op, **fields})

            draft = change("prepare", revision=0, add=[[], [-1]])
            first, second = draft["tasks"]
            write(draft["plan_path"], work("Example fixture"))
            write(first["path"], work("Write the fixture"))
            write(second["path"], work("Check the fixture"))
            ask = question("prepare", draft_id=draft["draft_id"])
            prompt = "# Which output format?\n\nJSON or plain text?\n"
            write(ask["path"], prompt)
            published = question("publish", question_id=ask["id"])
            route(published)
            assert "Which output format?" in hook("Stop")
            assert "question" in change(
                "commit", True, draft_id=draft["draft_id"], requests=[intake["request"]["id"]], answers=[]
            )
            write(ask["path"], prompt + "\n## Answer\nJSON.\n")
            question("answer", question_id=ask["id"])
            committed = change("commit", draft_id=draft["draft_id"],
                               requests=[intake["request"]["id"]], answers=[ask["id"]])
            route(committed)
            assert "Waiting for an instruction to start" in hook("Stop")

            change("authorize")
            current = mcp.tool("start_next", plan_id=plan)
            route(current)
            assert current["id"] == first["id"]
            assert "In progress · Write the fixture" in hook("Stop")
            write(project / "fixture.json", '{"value": 42}\n')
            result(first["path"])
            mcp.tool("finish_task", plan_id=plan, task_id=first["id"], attempt=current["attempt"], verified=[1])
            current = mcp.tool("start_next", plan_id=plan)
            assert current["id"] == second["id"]
            extra = mcp.tool("record_request", plan_id=plan)
            write(extra["request"]["path"], "Also verify the exact value.")
            route(extra)
            assert "1/2 complete" in (pending := hook("Stop"))
            assert "1 request(s)" in pending
            new = change("prepare", revision=1, add=[[first["id"]]], remove=[second["id"]])
            write(new["plan_path"], work("Example fixture"))
            write(new["tasks"][0]["path"], work("Check the exact JSON value"))
            change("commit", draft_id=new["draft_id"], requests=[extra["request"]["id"]], answers=[])
            pending_task = mcp.tool("start_next", plan_id=plan, model="example-one")
            assert pending_task == mcp.tool("start_next", plan_id=plan)
            pending_path = Path(pending_task["path"])
            assert pending_path.read_text().count("Attempt 1:") == 1
            assert "`afk-smoke` (model: `example-one`)" in pending_path.read_text()
            note = "Changed fixture.json. Parsed it; exact value check remains."
            checkpoint = {
                "op": "save_handoff", "task_id": pending_task["id"],
                "attempt": pending_task["attempt"], "note": note,
            }
            saved = mcp.tool("update_plan", plan_id=plan, change=checkpoint)
            assert saved["status"] == "saved"
            response = [{"type": "text", "text": json.dumps(saved)}]
            assert not hook("PostToolUse", "claude-a", None, response, tool="update_plan")
            assert "In progress · Check the exact JSON value" in hook("Stop", "claude-a", None)
            assert mcp.tool("open_plan", plan_id=plan)["progress"]["status"] == "doing"
            interrupted_save = subprocess.run(
                [sys.executable, "-c",
                 "import os, sqlite3, sys; db = sqlite3.connect(sys.argv[1]); "
                 "db.execute('UPDATE handoffs SET note=?', ('incomplete save',)); os._exit(9)",
                 str(project / ".afk" / "afk.db")],
                stdin=subprocess.DEVNULL, timeout=5,
            )
            assert interrupted_save.returncode == 9
            stats = [mcp.calls, mcp.input_bytes, mcp.output_bytes]

        with Mcp(binary, project, root, roots=True, name="afk-successor") as mcp:
            handoff_view = mcp.tool("open_plan", plan_id=plan, tasks=True)
            assert handoff_view["progress"]["revision"] == 2
            assert handoff_view["tasks"][-1]["handoff"]["note"] == note
            assert not (root / ".afk").exists()
            assert "different session" in mcp.tool("start_next", True, plan_id=plan)
            assert "not owned" in mcp.tool("update_plan", True, plan_id=plan, change=checkpoint)
            revised = mcp.tool("start_next", plan_id=plan, take_over=True)
            assert "Attempt 1: `afk-smoke`" in pending_path.read_text()
            assert "Attempt 2: `afk-successor` (model: `Unknown`)" in pending_path.read_text()
            assert json.loads((project / "fixture.json").read_text()) == {"value": 42}
            result(revised["path"])
            mcp.tool("finish_task", plan_id=plan, task_id=revised["id"], attempt=revised["attempt"], verified=[1])
            result(new["plan_path"])
            done = mcp.tool("update_plan", plan_id=plan, change={"op": "finish", "revision": 2, "verified": [1]})
            route(done)
            assert "2/2 complete" in (report := hook("Stop"))
            assert "Plan complete" in report
            route(done)
            with sqlite3.connect(project / ".afk" / "afk.db", isolation_level=None) as lock:
                lock.execute("BEGIN EXCLUSIVE")
                assert not hook("Stop")
                lock.execute("ROLLBACK")
            assert not hook("Stop")
            stats = [a + b for a, b in zip(stats, (mcp.calls, mcp.input_bytes, mcp.output_bytes))]
        return {
            "quality_checks_passed": True,
            "tools": len(tools),
            "tool_definitions_json_bytes": compact(tools),
            "tool_calls": stats[0],
            "tool_argument_json_bytes": stats[1],
            "tool_text_result_bytes": stats[2],
            "hook_invocations": len(hook_runs),
            "hook_stdout_bytes": sum(size for _, size in hook_runs),
            "median_hook_ms": round(sorted(ms for ms, _ in hook_runs)[len(hook_runs) // 2], 2),
            "elapsed_ms": round(1000 * (time.perf_counter() - started), 2),
            "model_tokens": None,
        }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("binary", type=Path)
    parser.add_argument("--trials", type=int, default=1)
    args = parser.parse_args()
    assert args.trials > 0
    for _ in range(args.trials):
        print(json.dumps(workflow(args.binary.resolve()), ensure_ascii=False))


if __name__ == "__main__":
    main()
