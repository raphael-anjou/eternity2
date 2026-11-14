#!/usr/bin/env python3
"""Run a test-seed command only after the preregistration is fixed, and log it.

    python3 ../common/prereg_guard.py <experiment-dir> <stage> -- <command> [args...]

Every command that touches test seeds goes through this wrapper. It closes a
gap a judge cannot close afterwards: file times only show the last write, so a
test run that happened before PREREG.md was committed, was looked at and then
overwritten, leaves no trace.

What it does, in order:
  1. If the experiment is in a git checkout: refuse to run unless PREREG.md is
     committed and has no uncommitted change. (On a machine that only has a
     copied tree, there is no git: the check is skipped and the log says so.)
  2. Append one line to <experiment-dir>/results/run-log.jsonl BEFORE the
     command starts: UTC time, host, stage, command, git HEAD, the commit that
     last touched PREREG.md and the SHA-256 of PREREG.md.
  3. Run the command, then append a second line with the exit code and the
     duration. Both lines carry the same `run_id`, so runs that overlap in time
     can be paired; a start line with no end line means the run was killed.

The start line also records how many files of the experiment directory had
uncommitted changes, results excluded (`dirty_paths`), so a judge can see whether
the code that ran was the committed code.

The log is append-only by convention: commit it with the results, never edit
or truncate it. A judge compares its first test-stage line with the PREREG
commit, and checks that the PREREG hash in every line is the committed one.

Stdlib only. Exit code is the command's own, or 3 when the guard refuses.
"""
import hashlib
import json
import os
import socket
import subprocess
import sys
import time
import uuid
from datetime import datetime, timezone
from pathlib import Path


def git(exp, *args):
    try:
        out = subprocess.run(["git", "-C", str(exp), *args], capture_output=True, text=True, timeout=20)
    except (OSError, subprocess.TimeoutExpired):
        return None
    return out.stdout.strip() if out.returncode == 0 else None


def main():
    argv = sys.argv[1:]
    if "--" not in argv or argv.index("--") != 2 or len(argv) < 4:
        sys.exit("usage: prereg_guard.py <experiment-dir> <stage> -- <command> [args...]")
    exp, stage, cmd = Path(argv[0]).resolve(), argv[1], argv[3:]
    prereg = exp / "PREREG.md"
    if not prereg.is_file():
        print(f"prereg_guard: {prereg} does not exist; write and commit it first", file=sys.stderr)
        sys.exit(3)

    in_git = git(exp, "rev-parse", "--is-inside-work-tree") == "true"
    head = prereg_commit = None
    dirty_paths = None
    if in_git:
        head = git(exp, "rev-parse", "HEAD")
        prereg_commit = git(exp, "log", "-1", "--format=%H %cI", "--", "PREREG.md")
        dirty = git(exp, "status", "--porcelain", "--", "PREREG.md")
        if not prereg_commit or dirty:
            print("prereg_guard: PREREG.md is not committed, or has uncommitted changes. "
                  "Commit it (alone) before running test seeds.", file=sys.stderr)
            sys.exit(3)
        status = git(exp, "status", "--porcelain", "--untracked-files=no", "--", ".")
        # Result files change from run to run by design: count code and prose only.
        dirty_paths = len([l for l in (status or "").splitlines() if l.strip() and "/results/" not in l])

    run_id = uuid.uuid4().hex[:12]
    log = exp / "results" / "run-log.jsonl"
    log.parent.mkdir(parents=True, exist_ok=True)
    record = {
        "event": "start",
        "run_id": run_id,
        "utc": datetime.now(timezone.utc).isoformat(timespec="seconds"),
        "host": socket.gethostname(),
        "stage": stage,
        "cmd": cmd,
        "cwd": os.getcwd(),
        "git": "checkout" if in_git else "unavailable (copied tree)",
        "head": head,
        "prereg_commit": prereg_commit,
        "dirty_paths": dirty_paths,
        "prereg_sha256": hashlib.sha256(prereg.read_bytes()).hexdigest(),
    }
    with open(log, "a") as fh:
        fh.write(json.dumps(record) + "\n")

    started = time.time()
    code = subprocess.call(cmd)
    with open(log, "a") as fh:
        fh.write(json.dumps({"event": "end", "run_id": run_id, "utc": datetime.now(timezone.utc).isoformat(timespec="seconds"),
                             "stage": stage, "exit": code, "seconds": round(time.time() - started, 1)}) + "\n")
    sys.exit(code)


if __name__ == "__main__":
    main()
