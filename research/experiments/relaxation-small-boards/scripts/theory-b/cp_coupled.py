"""TOY BOARDS ONLY. Experiment 2: the completely positive form with coupled columns.

Y = V V^T, V = W * W entrywise (P x r, so V >= 0 and Y >= 0 by construction), no border:
x_p = Y_pp = |v_p|^2. Penalised (quadratic penalty, weight rho, raised in stages):
  cell sums and tile sums of x equal 1,
  the lift's row sums: <v_p, m_c> = x_p for every placement p and cell c (m_c = sum of the rows
  of cell c), and the same over tiles,
and the conflict zeros as ONE linear term <C, V V^T> (every summand is nonnegative).
Columns are coupled through the row sums and the sums of x (scalar form), unlike softassign.py.
Inner solve: L-BFGS on W. After each stage x is rounded greedily; stop at the first solution.
Usage: cp_coupled.py n cf ci seed r start0 start1 seconds out.jsonl [broken|gradcheck]
"""
import os, sys, json, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
import scipy.sparse as sp
from scipy.optimize import minimize
from cp_common import load, is_solution

n, cf, ci, seed, r, s0, s1 = map(int, sys.argv[1:8])
tlim = float(sys.argv[8]); out = sys.argv[9]
mode = sys.argv[10] if len(sys.argv) > 10 else "random"
B, S, C = load(n, cf, ci, seed, mode == "broken")
P, N, T = B.P, B.N, B.T
Ac, At = S.Ccell, S.Ctile
nev = [0]


def fg(w, rho):
    nev[0] += 1
    W = w.reshape(P, r); V = W * W
    d = (V * V).sum(1)
    CV = C @ V
    f = float((V * CV).sum()); g = 2 * CV
    for A in (Ac, At):
        h = A @ d - 1
        M = A @ V
        R = V @ M.T - d[:, None]
        f += 0.5 * rho * (h @ h + (R * R).sum())
        g += rho * ((2 * (A.T @ h) - 2 * R.sum(1))[:, None] * V + R @ M + A.T @ (R.T @ V))
    return f, (g * 2 * W).ravel()


if mode == "gradcheck":
    rng = np.random.default_rng(0); w = rng.random(P * r)
    f0, g0 = fg(w, 3.0); e = rng.standard_normal(P * r); eps = 1e-6
    num = (fg(w + eps * e, 3.0)[0] - fg(w - eps * e, 3.0)[0]) / (2 * eps)
    print("gradcheck rel err", abs(num - g0 @ e) / abs(num)); sys.exit()

for st in range(s0, s1):
    rng = np.random.default_rng(100000 * r + st)
    # rows with |v_p|^2 = X0_p (Sinkhorn centre), mass spread at random over the r columns
    share = rng.dirichlet(np.ones(r), size=P) if r > 1 else np.ones((P, 1))
    x0 = B.sinkhorn(B.X0 * np.exp(0.5 * rng.standard_normal(P)))
    V = np.sqrt(x0[:, None] * share)
    w = np.sqrt(V).ravel() + 1e-3
    t0 = time.time(); nev[0] = 0; solved_stage = None; hist = []
    rho = 1.0; stage = 0
    while time.time() - t0 < tlim:
        res = minimize(fg, w, args=(rho,), jac=True, method="L-BFGS-B", options=dict(maxiter=300, maxcor=10))
        w = res.x; V = w.reshape(P, r) ** 2; x = (V * V).sum(1)
        conf = float((V * (C @ V)).sum())
        sums = max(np.abs(Ac @ x - 1).max(), np.abs(At @ x - 1).max())
        ok, js = is_solution(B, x)
        hist.append(dict(stage=stage, rho=rho, conflict=conf, sum_viol=float(sums), mism=int(B.mismatches(js.tolist())), evals=nev[0]))
        if ok:
            solved_stage = stage
            break
        stage += 1
        rho = min(rho * 3, 1e5)
        if rho >= 1e5 and len(hist) > 14 and abs(hist[-1]["conflict"] - hist[-3]["conflict"]) < 1e-9:
            break
    cols = int(((V * V).sum(0) > 1e-3).sum())
    rec = dict(board=[n, cf, ci, seed], mode=mode, r=r, start=st, solved=bool(ok), solved_stage=solved_stage,
               evals=nev[0], seconds=round(time.time() - t0, 2), final=hist[-1], live_columns=cols,
               mass_off_solutions=float(x[~np.isin(np.arange(P), np.concatenate(B.sol_idx))].sum() / N) if B.sol_idx else None)
    with open(out, "a") as f:
        f.write(json.dumps(rec) + "\n")
    print(json.dumps({k: rec[k] for k in ("r", "start", "solved", "evals", "seconds", "final", "live_columns", "mass_off_solutions")}), flush=True)
