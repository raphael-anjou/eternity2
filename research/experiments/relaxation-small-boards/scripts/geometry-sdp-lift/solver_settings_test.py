"""TOY. Solver settings on 5x5 seed 1 before trying 6x6: per-iteration cost and iteration count of
the SDP (l2 objective) for the three linear-solver backends at eps 1e-4, and whether the
reading (mass off solution placements, hull distance) moves. Usage: solver_settings_test.py n cf ci seed"""
import os, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np, scs
from board import Board
from lift import build
from measure import hull_distance
n, cf, ci, seed = map(int, sys.argv[1:5])
B = Board(n, cf, ci, seed)
used = np.zeros(B.P, bool)
for s in B.sol_idx: used[s] = True
data, cone, lay = build(B, psd=True)
print("P", B.P, "F", lay["F"], "rows", data["A"].shape[0], "nnz", data["A"].nnz, flush=True)
for ls in os.environ.get("BACKENDS", "accelerate,qdldl,cpu_indirect").split(","):
    for eps in (1e-4,):
        t0 = time.time()
        s = scs.SCS(data, cone, eps_abs=eps, eps_rel=eps, linear_solver=ls, time_limit_secs=150, verbose=False).solve()
        x = np.clip(s["x"][lay["ix"]], 0, 1)
        print(dict(linear_solver=ls, eps=eps, status=s["info"]["status"], iters=s["info"]["iter"],
                   secs=round(time.time() - t0, 1), setup_ms=round(s["info"]["setup_time"]),
                   mass_off_per_cell=float(x[~used].sum() / B.N), hull=round(hull_distance(B, x), 5)), flush=True)
