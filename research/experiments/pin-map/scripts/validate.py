"""Exactness checks on calibration boards (never measured boards):
count_fast against the original enumerate_first, the one-pass multi counts
against direct enumeration of each pinned tree (singles and pairs, both scan
directions). Usage: validate.py board [n_cells_checked]"""
import json, os, random, subprocess, sys
EXP = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BIN = os.path.join(EXP, "engine/target/release/pinmap")
b = sys.argv[1]; K = int(sys.argv[2]) if len(sys.argv) > 2 else 8
n = int([l for l in open(b) if not l.startswith("#")][0])
rng = random.Random(7)
def run(args, inp):
    return [json.loads(l) for l in subprocess.run([BIN] + args, input=inp, capture_output=True, text=True, check=True).stdout.splitlines()]
bad = 0
for o in ("fwd", "rev"):
    for base in ([], [rng.randrange(n * n)]):
        bs = ",".join(map(str, base)) or "-"
        mu = run(["multi", "--board", b, "--order", o], f"m\t{bs}\tall\n")[0]
        cells = rng.sample(mu["cand"], K)
        jobs = "".join(f"c{q}\t{','.join(map(str, base + [q]))}\n" for q in cells)
        direct = run(["rs", "--board", b, "--order", o, "--verify"], jobs + f"base\t{bs}\n")
        assert direct[-1]["nodes"] == mu["base_nodes"], (direct[-1], mu["base_nodes"])
        for q, d in zip(cells, direct):
            i = mu["cand"].index(q)
            got = (mu["nodes"][i], mu["solutions"][i], mu["first"][i])
            want = (d["nodes"], d["solutions"], d["first"])
            if got != want:
                bad += 1
                print("MISMATCH", o, base, q, got, want)
        print(o, base, "checked", K, "cells; base", mu["base_nodes"], "multi s", mu["seconds"], flush=True)
print("mismatches", bad)
sys.exit(1 if bad else 0)
