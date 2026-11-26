"""Integration test for plugins/shared/run auto-download, SHA256 verification, caching, and --hook fail-open."""

import argparse
import functools
import hashlib
import http.server
import json
import os
from pathlib import Path
import platform
import subprocess
import tarfile
import tempfile
import threading


def host_binary_name():
    system = platform.system()
    machine = platform.machine()
    if system == "Darwin" and machine in ("arm64", "x86_64"):
        return "afk-macos"
    if system == "Linux" and machine == "x86_64":
        return "afk-linux-x64"
    if system == "Linux" and machine in ("aarch64", "arm64"):
        return "afk-linux-arm64"
    raise RuntimeError(f"Unsupported platform: {system}:{machine}")


class TrackingHandler(http.server.SimpleHTTPRequestHandler):
    requests_seen = []

    def do_GET(self):
        self.requests_seen.append(self.path)
        super().do_GET()

    def log_message(self, format, *args):
        return


def mcp_initialize(command, cwd, env):
    proc = subprocess.Popen(
        command,
        cwd=cwd,
        env=env,
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    try:
        req = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2025-11-25",
                "capabilities": {},
                "clientInfo": {"name": "run-bootstrap-test", "version": "1.0.0"},
            },
        }
        proc.stdin.write(json.dumps(req).encode() + b"\n")
        proc.stdin.flush()
        line = proc.stdout.readline()
        assert line, f"no stdout; stderr={proc.stderr.read().decode(errors='replace')}"
        resp = json.loads(line)
        assert resp.get("id") == 1 and "result" in resp, resp
        assert resp["result"]["serverInfo"]["name"] == "agent-flow-kit", resp
        proc.stdin.close()
        assert proc.wait(timeout=10) == 0, proc.stderr.read().decode(errors="replace")
    finally:
        if proc.poll() is None:
            proc.kill()
            proc.wait()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("run_script", type=Path)
    parser.add_argument("binary", type=Path)
    args = parser.parse_args()

    run_script = args.run_script.resolve()
    real_binary = args.binary.resolve()
    binary_name = host_binary_name()
    archive_name = f"{binary_name}.tar.gz"
    version = "0.1.0"

    with tempfile.TemporaryDirectory(prefix="afk-run-bootstrap-") as tmp:
        root = Path(tmp).resolve()
        serve_dir = root / "serve"
        good_dir = serve_dir / "good"
        bad_dir = serve_dir / "bad"
        good_dir.mkdir(parents=True)
        bad_dir.mkdir(parents=True)

        archive_path = good_dir / archive_name
        with tarfile.open(archive_path, "w:gz") as tf:
            tf.add(real_binary, arcname=binary_name)

        digest = hashlib.sha256(archive_path.read_bytes()).hexdigest()
        (good_dir / "SHA256SUMS").write_text(f"{digest}  ./{archive_name}\n", encoding="utf-8")

        (bad_dir / archive_name).write_bytes(archive_path.read_bytes())
        (bad_dir / "SHA256SUMS").write_text(f"{'0' * 64}  ./{archive_name}\n", encoding="utf-8")

        TrackingHandler.requests_seen = []
        handler = functools.partial(TrackingHandler, directory=str(serve_dir))
        server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), handler)
        port = server.server_address[1]
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()

        try:
            cache_dir = root / "cache"
            project_dir = root / "project"
            project_dir.mkdir()

            base_env = {
                **os.environ,
                "AFK_CACHE_DIR": str(cache_dir),
                "AFK_VERSION": version,
                "AFK_RELEASE_BASE_URL": f"http://127.0.0.1:{port}/good",
            }

            # 1. --hook must fail-open without touching the network or cache.
            hook_res = subprocess.run(
                [str(run_script), "--hook"],
                input=json.dumps({"hook_event_name": "Stop", "session_id": "s1", "cwd": str(project_dir)}),
                text=True,
                capture_output=True,
                env=base_env,
                timeout=2,
                check=True,
            )
            assert hook_res.stdout == "" and hook_res.stderr == "", (hook_res.stdout, hook_res.stderr)
            assert TrackingHandler.requests_seen == [], TrackingHandler.requests_seen
            assert not (cache_dir / f"v{version}" / binary_name).exists()

            # 2. Checksum mismatch must fail and leave no cached binary or temp directory.
            bad_env = {**base_env, "AFK_RELEASE_BASE_URL": f"http://127.0.0.1:{port}/bad"}
            bad_res = subprocess.run(
                [str(run_script), "--root", str(project_dir)],
                input="",
                text=True,
                capture_output=True,
                env=bad_env,
                timeout=10,
            )
            assert bad_res.returncode != 0, "expected checksum mismatch failure"
            assert "SHA256 mismatch" in bad_res.stderr, bad_res.stderr
            assert not (cache_dir / f"v{version}" / binary_name).exists()
            leftovers = list((cache_dir / f"v{version}").glob(".afk-dl.*"))
            assert leftovers == [], leftovers

            # 3. First MCP invocation downloads, verifies, caches, and serves stdio MCP.
            TrackingHandler.requests_seen.clear()
            mcp_initialize([str(run_script), "--root", str(project_dir)], project_dir, base_env)
            cached_bin = cache_dir / f"v{version}" / binary_name
            assert cached_bin.is_file() and os.access(cached_bin, os.X_OK)
            assert f"/good/{archive_name}" in TrackingHandler.requests_seen
            assert "/good/SHA256SUMS" in TrackingHandler.requests_seen
            leftovers = list((cache_dir / f"v{version}").glob(".afk-dl.*"))
            assert leftovers == [], leftovers

            # 4. Subsequent MCP invocation reuses cache without network access.
            TrackingHandler.requests_seen.clear()
            offline_env = {**base_env, "AFK_RELEASE_BASE_URL": "http://127.0.0.1:1/unreachable"}
            mcp_initialize([str(run_script), "--root", str(project_dir)], project_dir, offline_env)
            assert TrackingHandler.requests_seen == [], TrackingHandler.requests_seen

            print(json.dumps({"run_bootstrap_passed": True, "binary": binary_name, "version": version}))
        finally:
            server.shutdown()
            server.server_close()


if __name__ == "__main__":
    main()
