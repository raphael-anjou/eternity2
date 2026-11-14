"""Calibration (seeds 9001..; never measured): unpinned row-scan tree size,
solution count and seconds per (n, cf, ci). Usage: calib.py n cf ci nseeds [cap]"""
import json, os, subprocess, sys
EXP = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BIN = os.path.join(EXP, "engine/target/release/pinmap")
n, cf, ci, ns = map(int, sys.argv[1:5])
cap = sys.argv[5] if len(sys.argv) > 5 else "3e9"
os.makedirs(os.path.join(EXP, "calib/boards"), exist_ok=True)
out = open(os.path.join(EXP, "calib/calib.jsonl"), "a")
for s in range(9001, 9001 + ns):
    b = os.path.join(EXP, f"calib/boards/n{n}-f{cf}-c{ci}-s{s}.txt")
    subprocess.run([BIN, "gen", "--n", str(n), "--cf", str(cf), "--ci", str(ci), "--seed", str(s), "--out", b], check=True, capture_output=True)
    for o in ("fwd",):
        r = subprocess.run([BIN, "rs", "--board", b, "--order", o, "--cap", cap], input="none\t-\n", capture_output=True, text=True, check=True).stdout
        d = json.loads(r)
        d.update({"n": n, "cf": cf, "ci": ci, "seed": s})
        out.write(json.dumps(d) + "\n"); out.flush()
        print(n, cf, ci, s, o, d["complete"], f"{d['nodes']:.3e}", d["solutions"], d["first"], d["seconds"], flush=True)
