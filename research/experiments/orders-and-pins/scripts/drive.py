"""Run the units of a plan, resumably, at most --jobs at a time.

    python3 -I scripts/drive.py plans/<stage>.jsonl results/<stage>.jsonl [--jobs 2]

A plan line is {"unit": "<id>", "cmd": [binary args...]}. Each unit's stdout
lines (JSON objects) are written to the output file with the unit id added,
one unit at a time, flushed. A unit whose id is already in the output file is
skipped, so a killed run is resumed by rerunning the same command; a unit cut
by a kill leaves no line and runs again. Stdlib only.
"""
import json
import os
import subprocess
import sys
import threading
from concurrent.futures import ThreadPoolExecutor

EXP = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def main():
    a = sys.argv[1:]
    jobs = 2
    if "--jobs" in a:
        i = a.index("--jobs")
        jobs = int(a[i + 1])
        del a[i:i + 2]
    plan, out = a
    # amendment A7: stages listed in REMOTE_STAGES run on another host; skip them here
    rs = os.path.join(EXP, "REMOTE_STAGES")
    stage = os.path.basename(plan)[:-6]
    if os.path.exists(rs) and stage in open(rs).read().split():
        print(f"# {plan}: stage {stage} runs on another host (REMOTE_STAGES), skipped here", file=sys.stderr, flush=True)
        return
    units = [json.loads(l) for l in open(plan) if l.strip()]
    done = set()
    if os.path.exists(out):
        for l in open(out):
            if l.strip():
                done.add(json.loads(l)["unit"])
    todo = [u for u in units if u["unit"] not in done]
    print(f"# {plan}: {len(units)} units, {len(done)} done, {len(todo)} to run", file=sys.stderr, flush=True)
    lock = threading.Lock()

    def run(u):
        cmd = ["nice", "-n", "10"] + u["cmd"]
        p = subprocess.run(cmd, cwd=EXP, capture_output=True, text=True)
        if p.returncode != 0:
            print(f"# unit {u['unit']} failed ({p.returncode}): {p.stderr[-500:]}", file=sys.stderr, flush=True)
            return
        lines = []
        for l in p.stdout.splitlines():
            if l.strip():
                d = json.loads(l)
                lines.append(json.dumps({"unit": u["unit"], **d}))
        with lock:
            with open(out, "a") as fh:
                fh.write("\n".join(lines) + "\n")
                fh.flush()
                os.fsync(fh.fileno())
        print(f"# done {u['unit']}", file=sys.stderr, flush=True)

    with ThreadPoolExecutor(max_workers=jobs) as ex:
        list(ex.map(run, todo))


if __name__ == "__main__":
    main()
