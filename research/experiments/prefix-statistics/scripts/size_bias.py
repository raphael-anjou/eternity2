#!/usr/bin/env python3
"""The reviewer's prediction checked on the test rows: for S6 (a log count of
completions, in nats), a true prefix that is a uniform prefix reweighted by
its completions should move by about sd^2, so delta should be about sd.

    python3 -I scripts/size_bias.py results/test

Per depth, over live rows (census-feasible reference): mean sd of S6, mean
shift (true minus mean), mean of shift / sd^2, mean delta. Stdlib only.
"""
import glob
import json
import os
import sys

S6 = 5
rows = [json.loads(l) for f in sorted(glob.glob(os.path.join(sys.argv[1], "board-*.jsonl"))) for l in open(f)]
rows = [r for r in rows if "depth" in r and not r["extinct"] and "sd_feas" in r]
print("depth  rows  sd(S6) nats  shift  shift/sd^2  delta")
for k in sorted({r["depth"] for r in rows}):
    rk = [r for r in rows if r["depth"] == k and r["sd_feas"][S6] > 0]
    if not rk:
        continue
    sd = sum(r["sd_feas"][S6] for r in rk) / len(rk)
    sh = sum(r["true"][S6] - r["mean_feas"][S6] for r in rk) / len(rk)
    ratio = sum((r["true"][S6] - r["mean_feas"][S6]) / r["sd_feas"][S6] ** 2 for r in rk) / len(rk)
    dl = sum((r["true"][S6] - r["mean_feas"][S6]) / r["sd_feas"][S6] for r in rk) / len(rk)
    print(f"{k:5d} {len(rk):5d} {sd:11.2f} {sh:6.2f} {ratio:10.2f} {dl:6.2f}")
