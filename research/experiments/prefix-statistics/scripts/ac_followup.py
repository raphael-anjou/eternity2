#!/usr/bin/env python3
"""Unplanned follow-up table: share of sampled prefixes arc consistency does not refute.

    python3 -I scripts/ac_followup.py results/acfollowup [results/test]

Per depth, over (board, direction) trees: the mean of the per-tree share of the
m uniform subsample that is not refuted (refuted = some empty cell's domain is
empty at the AC fixed point), the same share work-weighted (each sampled
particle weighted by its estimated subtree size, from its SMC descendants), the
median work ESS, the mean ns per check, and true prefixes refuted. Within a
level the SMC gives every particle the same weight (level estimate / N), so the
unweighted share is the estimate of the share of that level's nodes. With the
test directory, also prints the test stage's lower bound (share with S7 at or
above the true prefix's S7) on the same trees. Stdlib only.
"""
import glob
import json
import os
import statistics
import sys

rows = []
for f in sorted(glob.glob(os.path.join(sys.argv[1], "board-*.jsonl"))):
    rows += [json.loads(l) for l in open(f)]
lb = {}
if len(sys.argv) > 2:
    for f in sorted(glob.glob(os.path.join(sys.argv[2], "board-*.jsonl"))):
        for l in open(f):
            r = json.loads(l)
            if "depth" in r and not r["extinct"]:
                lb[(r["board"], r["dir"], r["depth"])] = 1 - r["true_below"][6]
boards = sorted({r["board"] for r in rows})
print(f"boards {len(boards)} ({boards[0]} to {boards[-1]}), directions 0-3, np 500000, m 1000 per tree")
print("depth trees extinct  share(node)  [min, max]        share(work)  med work ESS  ns/check  true refuted  test lower bound")
for k in sorted({r["depth"] for r in rows}):
    rk = [r for r in rows if r["depth"] == k]
    live = [r for r in rk if not r["extinct"]]
    tref = sum(1 for r in rk if r["true_refuted"])
    if not live:
        print(f"{k:5d} {len(rk):5d} {len(rk):7d}  all extinct; true refuted {tref}")
        continue
    sh = [r["share"] for r in live]
    ws = [r["work_share"] for r in live]
    ess = statistics.median(r["work_ess"] for r in live)
    ns = statistics.mean(r["ns_per_check"] for r in live)
    lbs = [lb[(r["board"], r["dir"], k)] for r in live if (r["board"], r["dir"], k) in lb]
    lbt = f"{statistics.mean(lbs):.4f} ({len(lbs)} trees)" if lbs else "n/a"
    print(f"{k:5d} {len(rk):5d} {len(rk) - len(live):7d}  {statistics.mean(sh):.4f}  [{min(sh):.4f}, {max(sh):.4f}]  "
          f"{statistics.mean(ws):.4f}  {ess:12.1f}  {ns:8.0f}  {tref:12d}  {lbt}")
