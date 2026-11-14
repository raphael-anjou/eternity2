"""Second analysis script 1b (gap 1 review): merge the ROW lines of several r1_real.py runs and
weight them with the committed Track 10 widths.

usage: python3 -I r1_merge.py out1 out2 ...
"""
import json
import subprocess
import sys
from math import log2, log10

REPO = "."
REF = "dc71ad56:research/experiments/real-puzzle-profile/results/m2/five-np1e6.jsonl"
BS = (20, 40, 61, 80)
txt = subprocess.run(["git", "-C", REPO, "show", REF], capture_output=True, text=True, check=True).stdout
reps = [json.loads(l)["log10_width"] for l in txt.splitlines() if l.strip()]
W = []
for d in range(min(len(r) for r in reps)):
    vals = [r[d] for r in reps if r[d] is not None and r[d] > -1e300]
    if not vals:
        break
    W.append(sum(10 ** (v - 30) for v in vals) / len(reps))
tot = sum(W)
rows, gates = {}, []
for f in sys.argv[1:]:
    for l in open(f):
        v = l.split()
        if l.startswith("ROW "):
            rows[int(v[1])] = [float(x) for x in v[2:]]
        elif "|" in l and v and v[0].isdigit():
            gates.append((int(v[0]), float(v[4])))
ds = sorted(rows)
print("depths %d to %d (%d), share of the tree at these depths %.4f" % (ds[0], ds[-1], len(ds), sum(W[d] for d in ds) / tot))
g = [abs(x) for d, x in gates if d <= 176]
print("level widths against Track 10, log10 difference: largest %.3f up to depth 176, largest %.3f beyond" % (max(g), max([abs(x) for d, x in gates if d > 176] or [0])))
print("bits of nodes removed over the whole tree (depths below %d untouched, above %d removed)" % (ds[0], ds[-1]))


def gain(col):
    rem = sum(W[d] for d in range(ds[0])) + sum(W[d] * (1 - rows[d][col]) for d in ds)
    return log2(tot / rem)


for j, B in enumerate(BS):
    print("   reasons <= %2d bits: memo definition %.2f, after shrinking %.2f" % (B, gain(j), gain(4 + j)))
print("   free oracle, height <= 16: %.2f" % gain(8))
for lo, hi in ((80, 113), (114, 140), (141, 172)):
    w = sum(W[d] for d in ds if lo <= d <= hi)
    print("   width-weighted share saved at B 61, depths %d to %d: memo definition %.2f, after shrinking %.2f" % (
        lo, hi, sum(W[d] * rows[d][2] for d in ds if lo <= d <= hi) / w, sum(W[d] * rows[d][6] for d in ds if lo <= d <= hi) / w))
