#!/usr/bin/env python3
"""Unplanned observation: arc consistency (S7) refutes sampled prefixes.

    python3 -I scripts/ac_observation.py results/test

Per depth: rows where every one of the m sampled prefixes has S7 = 0 (a wiped
domain empties the whole region, so S7 = 0 means arc-inconsistent), the mean
share of sampled prefixes with S7 above 0 (bounded below from the true prefix's
rank: S7 = 0 for a sample implies it is below the true prefix, whose S7 is
always > 0), and the number of rows where the true prefix was refuted (must
be 0: arc consistency is sound). Stdlib only.
"""
import glob
import json
import os
import sys

S7 = 6
rows = []
for f in sorted(glob.glob(os.path.join(sys.argv[1], "board-*.jsonl"))):
    for line in open(f):
        r = json.loads(line)
        if "depth" in r and not r["extinct"]:
            rows.append(r)
print("depth  rows  all-refuted rows  true refuted  mean share of samples with S7 > 0 (lower bound 1 - true_below)")
for k in sorted({r["depth"] for r in rows}):
    rk = [r for r in rows if r["depth"] == k]
    allz = sum(1 for r in rk if r["mean"][S7] == 0 and r["sd"][S7] == 0)
    tz = sum(1 for r in rk if r["true"][S7] == 0)
    ub = sum(1 - r["true_below"][S7] for r in rk) / len(rk)
    print(f"{k:5d} {len(rk):5d} {allz:17d} {tz:13d}   {ub:.4f}")
