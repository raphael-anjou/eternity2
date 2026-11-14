"""TOY. Near-solution controls for one board, with probe B's two definitions:
  annealed: probe B's annealer; keep a board with <= 2 mismatched inner edges and >= 8 cells
            (placement indices) from every solution;
  sampled:  probe B's mismatch-tolerant randomised DFS (<= 2 mismatches), same distance filter.
Only the budgets differ from probe B (probe B found no 6x6 control with 150 anneals of 15,000
steps and 3 million DFS nodes). Stops at 20 of each or at the time budget.
Usage: controls.py n cf ci seed anneal_steps anneal_seconds dfs_nodes out.json
"""
import os, sys, time, json, random
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from collections import Counter
from board import Board

n, cf, ci, seed, steps = map(int, sys.argv[1:6])
asec = float(sys.argv[6]); dnodes = int(sys.argv[7]); out = sys.argv[8]
B = Board(n, cf, ci, seed)
R = random.Random(7)
ann, hist, tries = [], Counter(), 0
t0 = time.time()
while len(ann) < 20 and time.time() - t0 < asec:
    tries += 1
    js, m = B.anneal(R, steps)
    hist[m] += 1
    if m <= 2:
        d = B.dist_to_sols(js)
        hist[f"m<=2,d={d}"] += 1
        if d >= 8:
            ann.append([int(v) for v in js])
ta = time.time() - t0
t0 = time.time()
smp, nodes = B.dfs_near(random.Random(7), 2, 20, dnodes)
ts = time.time() - t0
res = dict(board=[n, cf, ci, seed], n_sols=len(B.sols), P=B.P, anneal_steps=steps, anneal_tries=tries,
           anneal_seconds=round(ta, 1), anneal_hist={str(k): v for k, v in sorted(hist.items(), key=str)},
           annealed=ann, dfs_nodes=nodes, dfs_seconds=round(ts, 1), sampled=[[int(v) for v in s] for s in smp],
           annealed_mismatch=[B.mismatches(a) for a in ann], sampled_mismatch=[B.mismatches(s) for s in smp],
           annealed_dist=[B.dist_to_sols(a) for a in ann], sampled_dist=[B.dist_to_sols(s) for s in smp])
json.dump(res, open(out, "w"))
print(json.dumps({k: v for k, v in res.items() if k not in ("annealed", "sampled")}))
