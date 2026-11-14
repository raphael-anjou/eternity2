"""Check of the memo's reading of D5 (pin map). Reads D5's committed rows, read only.
Run: python3 -I d5_check.py research/experiments/pin-map/results
1. unpinned bb-mrv runs: how many, how many censored, at what cap, against the row-scan tree of the same board.
2. every pin set run by both solvers and complete under bb-mrv: log2(row-scan nodes) - log2(bb-mrv nodes), by number of pins.
   (Different node measures: row scan counts consistent partial boards, bb-mrv counts options tried. Censored bb-mrv runs
   are left out, which favours bb-mrv.)"""
import sys, json, os
from math import log2
from collections import defaultdict
d = sys.argv[1]
def rows(f): return [json.loads(l) for l in open(os.path.join(d, f)) if l.strip()]
mrv = rows("mrv8.jsonl")
base = {}
for r in rows("s8.jsonl"):
    lab = r["label"].split(":")
    if lab[2] == "fwd" and not r.get("base"): base[int(lab[1])] = r["base_nodes"]; 
single = {}
for r in rows("s8.jsonl"):
    lab = r["label"].split(":")
    if lab[2] == "fwd" and not r.get("base"):
        for c, nd in zip(r["cand"], r["nodes"]): single[(int(lab[1]), (c,))] = nd
rs = {}
for f in ("p8.jsonl", "g8.jsonl"):
    for r in rows(f):
        if r.get("solver") != "rs-fwd" or "pins" not in r or "nodes" not in r or not isinstance(r["nodes"], int): continue
        rs[(int(r["label"].split(":")[1]), tuple(sorted(r["pins"])))] = r["nodes"]
rs.update(single)
un = [r for r in mrv if not r["pins"]]
print("unpinned bb-mrv runs: %d, censored: %d, node cap %s" % (len(un), sum(not r["complete"] for r in un), {r["nodes"] for r in un}))
for r in un:
    b = int(r["label"].split(":")[1])
    print("  board %2d: row-scan tree 2^%.2f ; bb-mrv stopped at 2^%.2f, so bb-mrv could be up to %.1f bits smaller than row scan and still be censored" % (b, log2(base[b]), log2(r["nodes"]), log2(base[b]) - log2(r["nodes"])))
by = defaultdict(list); cens = defaultdict(int); tot = defaultdict(int)
for r in mrv:
    if not r["pins"]: continue
    b = int(r["label"].split(":")[1]); k = len(r["pins"]); key = (b, tuple(sorted(r["pins"]))); tot[k] += 1
    if not r["complete"]: cens[k] += 1; continue
    if key in rs: by[k].append(log2(rs[key]) - log2(r["nodes"]))
print("\nlog2(row scan) - log2(bb-mrv), same board and pin set, bb-mrv complete:")
for k in sorted(by):
    x = sorted(by[k]); print("  k=%2d  pairs %3d (bb-mrv runs %3d, censored %3d)  median %+.2f  mean %+.2f  range [%+.2f, %+.2f]" % (k, len(x), tot[k], cens[k], x[len(x) // 2], sum(x) / len(x), x[0], x[-1]))
