"""TOY BOARDS ONLY (5x5). re-check check: the smallest ||x||^2 over the convex hull of the enumerated
solutions, per board, next to the lift's minimum-norm objective reported by the splitting runs.
If the lift were exact (its x-projection equal to the hull), the two minima would be equal. A lift
minimum strictly below the hull minimum means the lift contains points outside the hull.

The hull minimum is a small quadratic program, solved here as a nonnegative least-squares problem
(sum of weights forced to 1 by a heavily weighted row), then renormalised; the value printed is
therefore an UPPER bound on the hull minimum only through renormalisation error, reported.

Usage: hull_minnorm.py
"""
import os, sys, json
import numpy as np
from scipy.optimize import nnls

HERE = os.path.dirname(os.path.abspath(__file__))
MEMOS = os.path.dirname(HERE)
sys.path.insert(0, os.path.join(MEMOS, "far-ideas-geometry"))
from common import gen  # noqa: E402

ns = {"__file__": os.path.join(HERE, "check_mass.py")}
exec(compile(open(ns["__file__"]).read().split("POINTS = {")[0], "check_mass_head", "exec"), ns)
LIFT = {  # ||x||^2 at the end of the splitting runs with the l2 objective (scripts/theory-d/raw, rho 300)
    0: ("p_s0_full.jsonl", None), 1: ("admm_s1_knn99_kz99_tf1_l2_rho300.jsonl", None), 2: ("p_s2_full.jsonl", None),
    3: ("p_s3_full.jsonl", None), 4: ("p_s4_full.jsonl", None)}
n, cf, ci = 5, 3, 4
for seed in range(5):
    tiles = gen(n, cf, ci, seed)
    pl, pin = ns["build"](n, tiles)
    P = len(pl)
    sols = ns["enumerate_reverse"](n, tiles, pl)
    S = np.zeros((P, len(sols)))
    for k, s in enumerate(sols):
        S[list(s), k] = 1.0
    M = 1e3
    A = np.vstack([S, M * np.ones((1, len(sols)))])
    b = np.concatenate([np.zeros(P), [M]])
    lam, _ = nnls(A, b, maxiter=200000)
    tot = lam.sum()
    x = S @ (lam / tot)
    f = os.path.join(MEMOS, "theory-d", "raw", LIFT[seed][0])
    fin = [json.loads(l) for l in open(f) if l.startswith("{") and '"final"' in l]
    lift_obj = fin[0]["obj_val"] if fin else None
    lift_viol = max(fin[0]["viol"].values()) if fin else None
    print(json.dumps(dict(seed=seed, solutions=len(sols), hull_min_norm_sq=float(x @ x), weights_sum_before_renorm=float(tot),
                          weights_positive=int((lam > 1e-12).sum()), lift_min_norm_sq=lift_obj, lift_point_violation=lift_viol,
                          lift_minus_hull=None if lift_obj is None else float(lift_obj - x @ x))), flush=True)
