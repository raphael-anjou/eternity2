"""TOY. Code check before any reading: on a 4x4 generated board with exactly one solution (under
the pin), the true solution's rank-one lift must satisfy every constraint of the SDP and of the LP
(a relaxation that excludes the solution is a bug). Then both are solved, and the solver point is
compared with the solution. Also checks a random rim-consistent board is NOT feasible (it violates
a zero), so the check can fail.
Usage: validate.py
"""
import os, sys, json, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
from board import Board
from common import gen, enumerate_all
from lift import build, solve, assemble_Z
from measure import check_rank_one, round_board, hull_distance, kills

pick = None
for cf, ci in [(2, 3), (2, 4), (3, 3), (3, 4), (2, 5)]:
    for seed in range(30):
        t = gen(4, cf, ci, seed)
        if t is None:
            continue
        if len(enumerate_all(4, t)["sols"]) == 1:
            pick = (cf, ci, seed); break
    if pick:
        break
cf, ci, seed = pick
B = Board(4, cf, ci, seed)
res = dict(board=[4, cf, ci, seed], n_sols=len(B.sols), P=B.P,
           pin=dict(tile=int(B.pin[0]), rotation=int(B.pin[1]), cell=0, sides=list(B.pin[2])))
alive = B.arc_consistency()
for name, psd in (("sdp", True), ("lp", False)):
    data, cone, lay = build(B, psd=psd)
    res[f"{name}_rank_one_solution_residuals"] = check_rank_one(B, data, cone, lay, B.sol_idx[0])
    res[f"{name}_rank_one_random_board_residuals"] = check_rank_one(B, data, cone, lay, B.ctrl_idx[0])
    sol, secs = solve(data, cone, eps=1e-6)
    x = sol["x"][lay["ix"]]
    r = dict(status=sol["info"]["status"], iters=sol["info"]["iter"], seconds=round(secs, 2),
             max_abs_x_minus_solution=float(np.abs(x - np.isin(np.arange(B.P), B.sol_idx[0])).max()),
             hull_distance=hull_distance(B, x), rounding=round_board(B, x))
    r.update(kills(B, x, alive))
    if psd:
        ev = np.linalg.eigvalsh(assemble_Z(lay, sol["x"]))[::-1]
        r["top_eigs"] = [round(float(e), 5) for e in ev[:5]]
    res[name] = r
print(json.dumps(res, indent=1))
