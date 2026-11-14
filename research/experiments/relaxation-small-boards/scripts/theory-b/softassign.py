"""TOY BOARDS ONLY. Experiment 1: one nonnegative column = soft assignment descent.

The completely positive form with vector sums separates into r independent copies of
    minimise  s^T C s   over  s >= 0, cell sums = 1, tile sums = 1
(C = 0/1 conflict matrix). This script runs that single-column problem by mirror descent
(multiplicative update s <- s * exp(-eta * 2 C s), then Sinkhorn scaling back onto the sums),
from random starts around the Sinkhorn centre, rounds greedily every `check` sweeps and stops at
the first rounding that is a solution, or when the point is integral, or at the sweep cap.
Usage: softassign.py n cf ci seed starts eta max_sweeps out.jsonl [broken]
"""
import os, sys, json, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
from cp_common import load, is_solution

n, cf, ci, seed, starts = map(int, sys.argv[1:6])
eta = float(sys.argv[6]); max_sweeps = int(sys.argv[7]); out = sys.argv[8]
broken = len(sys.argv) > 9 and sys.argv[9] == "broken"
# argv[10] == "anneal": deterministic annealing (softassign proper): beta = eta * 1.02^k,
# s <- (s + Sinkhorn(exp(-beta * 2 C s))) / 2, started at the same random point.
anneal = len(sys.argv) > 10 and sys.argv[10] == "anneal"
B, S, C = load(n, cf, ci, seed, broken)
check = 5
rows = []
t_all = time.time()
for st in range(starts):
    rng = np.random.default_rng(7000 + st)
    s = B.sinkhorn(B.X0 * np.exp(0.5 * rng.standard_normal(B.P)))
    solved_at = None; t0 = time.time(); k = 0
    for k in range(1, max_sweeps + 1):
        g = 2.0 * (C @ s)
        if anneal:
            s = 0.5 * s + 0.5 * B.sinkhorn(np.exp(-min(eta * 1.02 ** k, 1e3) * (g - g.min())) + 1e-300, it=10)
        else:
            s = B.sinkhorn(s * np.exp(-eta * (g - g.min())), it=5)
        if k % check == 0:
            ok, js = is_solution(B, s)
            if ok:
                solved_at = k
                break
            cmax = np.maximum.reduceat(s, np.r_[0, np.flatnonzero(np.diff(B.cellOf)) + 1])
            if cmax.min() > 0.999:
                break
    ok, js = is_solution(B, s)
    rows.append(dict(board=[n, cf, ci, seed], broken=broken, anneal=anneal, start=st, eta=eta, sweeps=k,
                     solved=bool(ok), solved_at=solved_at, mismatches=int(B.mismatches(js.tolist())),
                     conflict=float(s @ (C @ s)), seconds=round(time.time() - t0, 3)))
with open(out, "a") as f:
    for r in rows:
        f.write(json.dumps(r) + "\n")
sol = [r for r in rows if r["solved"]]
mm = sorted(r["mismatches"] for r in rows)
print(f"board {n} {cf} {ci} {seed} broken={broken} anneal={anneal} sols={len(B.sols)} P={B.P} K={S.K} eta={eta}: "
      f"solved {len(sol)}/{len(rows)}, median sweeps to solution "
      f"{int(np.median([r['solved_at'] or r['sweeps'] for r in sol])) if sol else None}, "
      f"mismatches median {mm[len(mm)//2]} max {mm[-1]} min {mm[0]}, "
      f"median sweeps all {int(np.median([r['sweeps'] for r in rows]))}, total {time.time()-t_all:.1f}s")
