"""Route audit, 2026-11-14. Reads the gap 4 re-check's committed exact enumerations
(../gap-4-review/exact.jsonl, toys) and prints, per board class, the median
log2(branches tried by plain row scan / branches tried by each rule and propagation level).
Purpose: a propagation-only yardstick to set beside the gap 1 re-check's clause-learning ratio
(log2 tree nodes / conflicts = 6.1 at 5x5 and 6x6), which had no such control. Different boards
and different units (branches tried against conflicts), so the comparison is indicative only.
Run: python3 -I propagation_control.py
"""
import json, math, collections, statistics, os
HERE = os.path.dirname(os.path.abspath(__file__))
rows = [json.loads(l) for l in open(os.path.join(HERE, "..", "gap-4-review", "exact.jsonl"))]
g = collections.defaultdict(dict)
for r in rows:
    if r["censored"]:
        continue
    g[r["board"]][(r["rule"], r["prop"])] = r["tried"]
    g[r["board"]]["n"] = r["n"]
agg = collections.defaultdict(list)
for b, d in g.items():
    base = d.get(("ROW", "PLAIN"))
    cls = (d["n"], ":".join(b.split(":")[1:4]))
    for k, v in d.items():
        if k == "n" or not base:
            continue
        agg[cls + k].append(math.log2(base / v))
for k in sorted(agg, key=str):
    if k[2] in ("ROW", "DOM", "DOMF", "WDEG"):
        print(k, len(agg[k]), "median bits %.2f range %.2f to %.2f" % (statistics.median(agg[k]), min(agg[k]), max(agg[k])))
