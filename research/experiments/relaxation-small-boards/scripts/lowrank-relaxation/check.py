"""TOY. Code checks before any reading (5x5 seed 0):
 1. gradient of the augmented Lagrangian (sphere coordinates) against central finite differences,
    with random multipliers, several row blocks, and a point violating every constraint;
 2. every true solution's rank-one point has zero violation of every constraint of lift.py;
 3. a random rim-consistent board's rank-one point violates a zero (so the check can fail);
 4. the conflict list reproduces lift.py's free-pair count (86,800 on this board), and the
    index-list exclusion in max_violation agrees with a structure mask on a random point.
Usage: check.py
"""
import os, sys, json
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import common  # noqa: F401  (local copy, loaded before board.py)
import numpy as np
import scipy.sparse as sp
from board import Board
from bm import Structure, SphereAL, max_violation, V_of_U, U_of_solution, init_U

out = {}
B = Board(5, 3, 4, 0)
S = Structure(B, block_bytes=200 * 8 * B.P)
out.update(P=B.P, K_conflict_pairs=int(S.K), F_free_pairs=int(S.F), blocks=len(S.blocks),
           F_matches_lift_86800=bool(S.F == 86800))
rng = np.random.default_rng(0)
r = 4
al = SphereAL(S, r, rho=3.0)
al = SphereAL(S, r, rho=3.0, vec=True)
al.lc = rng.standard_normal((B.N, r)); al.lt = rng.standard_normal((B.T, r)); al.lz = rng.standard_normal(S.K)
fm_all = np.vstack([S.free_mask(rows) for rows in S.blocks])
M = np.triu(np.abs(rng.standard_normal((B.P, B.P))) * (rng.random((B.P, B.P)) < 0.02) * fm_all, 1)
al.M2 = sp.csr_matrix(M + M.T)
w = rng.standard_normal(B.P * r)
f0, g0 = al.fg(w)
errs = []
for k in rng.choice(B.P * r, 60, replace=False):
    e = np.zeros(B.P * r); e[k] = 1e-6
    fd = (al.fg(w + e)[0] - al.fg(w - e)[0]) / 2e-6
    errs.append(abs(fd - g0[k]) / max(1.0, abs(fd)))
out["grad_max_rel_err"] = float(max(errs))
out["solution_rank_one_max_violation"] = max(max_violation(S, V_of_U(U_of_solution(B, 1, s)))["max"] for s in B.sol_idx)
out["random_board_rank_one_violation"] = max_violation(S, V_of_U(U_of_solution(B, 1, B.ctrl_idx[0])))
V = V_of_U(init_U(B, 8, rng)); V[:, 1:] *= -1
G = V @ V.T
out["y_nonneg_index_list"] = max_violation(S, V)["y_nonneg"]
out["y_nonneg_structure_mask"] = float(max(0.0, -G[fm_all].min()))
print(json.dumps(out, indent=1))
