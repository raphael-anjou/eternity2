"""TOY BOARDS ONLY. The degree-2 lift in factored (Burer-Monteiro) form, used as a one-sided test.

Usage: uv run --with numpy --with scipy python -I -B bm.py n cf ci seed rank seconds [variant] [rngseed]
  Z = V V^T with V of shape (P+1, rank): Z is PSD by construction, so only the linear conditions of
  lift.py remain: Z00 = 1, Z0p = Zpp, cell sums 1, tile sums 1, Zpq = 0 on forbidden pairs
  (same cell, same tile, adjacent mismatch), Zpq >= 0 on every other pair.
  (The row-sum equalities of lift.py follow from these and PSD: with orthogonal placements in a cell
  and Z0p = Zpp, |v0 - sum_p v_p|^2 = 1 - sum_p x_p = 0, so v0 = sum_p v_p in every cell and tile.)
  Objective: MAXIMISE the mass on placements that are in no solution (the leak), by an augmented
  Lagrangian with L-BFGS inner solves. Environment: OBJ (weight of that objective, 0 = feasibility
  only from a random start), MU_MAX, INNER, CKPT (see the code).
What a run can show. A point with all residuals near 0 and a large leak is a feasible point of the lift
outside the solutions: the lift is not exact on that board (no dual certificate is needed, PSD being
built in). A run ending at leak near 0 is only evidence of exactness: the factored problem is not
convex. variant: full (default) | theta (no Zpq >= 0) | notile (same-tile zeros and tile sums dropped:
a positive control, the leak must be large).
"""
import os, sys, json, time
for v in ("VECLIB_MAXIMUM_THREADS", "OMP_NUM_THREADS", "OPENBLAS_NUM_THREADS"):
    os.environ[v] = "1"
here = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(here, "..", "geometry-sdp-lift"))
import numpy as np
from scipy.optimize import minimize
from board import Board

n, cf, ci, seed, r = map(int, sys.argv[1:6])
budget = float(sys.argv[6])
variant = sys.argv[7] if len(sys.argv) > 7 else "full"
rngseed = int(sys.argv[8]) if len(sys.argv) > 8 else 0
t_start = time.time()
B = Board(n, cf, ci, seed)
P, N, T = B.P, B.N, B.T
c, t = B.cellOf, B.tileOf
samecell = c[:, None] == c[None, :]
sametile = t[:, None] == t[None, :]
mis = np.zeros((P, P), bool)
for d in range(4):
    nbc = B.nb[c, d]
    mis |= (nbc[:, None] == c[None, :]) & (B.cols[:, d][:, None] != B.cols[:, (d + 2) % 4][None, :])
off = ~np.eye(P, dtype=bool)
Zm = (samecell | mis | (sametile if variant != "notile" else False)) & off
Fm = ~Zm & off
Zf = Zm.astype(float); Ff = Fm.astype(float)
insol = np.zeros(P, bool)
for s in B.sol_idx:
    insol[s] = True
w = (~insol).astype(float)
OBJ = float(os.environ.get("OBJ", "1"))   # weight of the leak objective; 0 = pure feasibility from a random start
Ac = np.zeros((N, P)); Ac[c, np.arange(P)] = 1
At = np.zeros((T, P)); At[t, np.arange(P)] = 1
use_tile = variant != "notile"
use_nonneg = variant != "theta"
rng = np.random.default_rng(rngseed)
k_cell = np.bincount(c, minlength=N)[c]
V = rng.standard_normal((P + 1, r)) / np.sqrt(r)
V[0] /= np.linalg.norm(V[0])
V[1:] *= (1.0 / np.sqrt(k_cell))[:, None]
lam = dict(Z=np.zeros((P, P)), F=np.zeros((P, P)), d=np.zeros(P), c=np.zeros(N), t=np.zeros(T), o=0.0)
mu = 5.0
state = {}


def parts(Vf):
    V = Vf.reshape(P + 1, r)
    v0, U = V[0], V[1:]
    G = U @ U.T
    dg = np.einsum("ij,ij->i", U, U)
    g0 = U @ v0
    return V, v0, U, G, dg, g0


def fg(Vf):
    V, v0, U, G, dg, g0 = parts(Vf)
    hd = g0 - dg
    hc = Ac @ dg - 1
    ht = At @ dg - 1
    ho = v0 @ v0 - 1
    f = -OBJ * (w @ dg)
    # penalty over ordered pairs with weight 1/2, so each unordered pair counts once
    S = (lam["Z"] + mu * G) * Zf
    f += 0.5 * np.sum((lam["Z"] * G + 0.5 * mu * G * G) * Zf)
    if use_nonneg:
        m = np.maximum(0.0, lam["F"] - mu * G) * Ff
        f += 0.5 * np.sum(m * m - (lam["F"] * Ff) ** 2) / (2 * mu)
        S -= m
    cd = lam["d"] + mu * hd
    cc = lam["c"] + mu * hc
    ct = (lam["t"] + mu * ht) if use_tile else np.zeros(T)
    co = lam["o"] + mu * ho
    f += lam["d"] @ hd + 0.5 * mu * hd @ hd + lam["c"] @ hc + 0.5 * mu * hc @ hc + lam["o"] * ho + 0.5 * mu * ho * ho
    if use_tile:
        f += lam["t"] @ ht + 0.5 * mu * ht @ ht
    gU = S @ U                                        # S is symmetric
    coef = -2 * OBJ * w - 2 * cd + 2 * (Ac.T @ cc) + 2 * (At.T @ ct)
    gU += coef[:, None] * U + cd[:, None] * v0[None, :]
    g0v = U.T @ cd + 2 * co * v0
    g = np.vstack([g0v[None, :], gU])
    return f, g.ravel()


def residuals(Vf):
    V, v0, U, G, dg, g0 = parts(Vf)
    out = dict(zero_max=float(np.abs(G[Zm]).max()), neg_min=float(G[Fm].min()) if use_nonneg else None,
               diag_max=float(np.abs(g0 - dg).max()), cell_max=float(np.abs(Ac @ dg - 1).max()),
               tile_max=float(np.abs(At @ dg - 1).max()) if use_tile else None, z00=float(abs(v0 @ v0 - 1)),
               leak=float(w @ dg), leak_per_cell=float(w @ dg) / N,
               off_solution_placements_above_1e_3=int(((dg > 1e-3) & ~insol).sum()))
    out["worst"] = max(out["zero_max"], -(out["neg_min"] or 0.0), out["diag_max"], out["cell_max"], out["tile_max"] or 0.0, out["z00"])
    return out


if os.environ.get("GRADCHECK") == "1":
    x0 = V.ravel().copy(); f0, g0_ = fg(x0)
    lam["Z"] = rng.standard_normal((P, P)); lam["Z"] = (lam["Z"] + lam["Z"].T) / 2
    lam["F"] = np.abs(rng.standard_normal((P, P))); lam["F"] = (lam["F"] + lam["F"].T) / 2
    lam["d"] = rng.standard_normal(P); lam["c"] = rng.standard_normal(N); lam["t"] = rng.standard_normal(T); lam["o"] = 0.3
    f0, g0_ = fg(x0)
    for _ in range(5):
        dx = rng.standard_normal(x0.size); dx /= np.linalg.norm(dx)
        e = 1e-6
        num = (fg(x0 + e * dx)[0] - fg(x0 - e * dx)[0]) / (2 * e)
        print("gradcheck", num, g0_ @ dx)
    raise SystemExit

x = V.ravel().copy()
hist = []
outer = 0
MU_MAX = float(os.environ.get("MU_MAX", "2000"))
INNER = int(os.environ.get("INNER", "400"))
ckpt = os.environ.get("CKPT")            # optional .npz: a later run continues from it (each run stays under 10 min)
if ckpt and os.path.exists(ckpt):
    z = np.load(ckpt)
    x = z["x"]; mu = float(z["mu"])
    for k_ in ("Z", "F", "d", "c", "t"):
        lam[k_] = z[k_]
    lam["o"] = float(z["o"])
while time.time() - t_start < budget and outer < 400:
    res = minimize(fg, x, jac=True, method="L-BFGS-B", options=dict(maxiter=INNER, maxcor=20, ftol=1e-13, gtol=1e-9))
    x = res.x
    V_, v0, U, G, dg, g0 = parts(x)
    lam["Z"] = (lam["Z"] + mu * G) * Zf
    if use_nonneg:
        lam["F"] = np.maximum(0.0, lam["F"] - mu * G) * Ff
    lam["d"] += mu * (g0 - dg); lam["c"] += mu * (Ac @ dg - 1); lam["o"] += mu * (v0 @ v0 - 1)
    if use_tile:
        lam["t"] += mu * (At @ dg - 1)
    rs = residuals(x)
    hist.append(dict(outer=outer, mu=mu, worst=round(rs["worst"], 6), leak_per_cell=round(rs["leak_per_cell"], 5),
                     seconds=round(time.time() - t_start, 1)))
    if rs["worst"] < 2e-6:
        break
    if outer >= 3 and hist[-1]["worst"] > 0.5 * hist[-2]["worst"]:
        mu = min(mu * 2, MU_MAX)
    outer += 1
if ckpt:
    np.savez(ckpt, x=x, mu=mu, Z=lam["Z"], F=lam["F"], d=lam["d"], c=lam["c"], t=lam["t"], o=lam["o"])
rs = residuals(x)
V_, v0, U, G, dg, g0 = parts(x)
sv = np.linalg.svd(V_, compute_uv=False)
out = dict(board=[n, cf, ci, seed], variant=variant + ("" if OBJ == 1 else f",obj={OBJ:g}"), rank=r, rngseed=rngseed, P=P, n_sols=len(B.sols),
           in_some_solution=int(insol.sum()), final=rs, numerical_rank_1e_3=int((sv > 1e-3 * sv[0]).sum()),
           seconds=round(time.time() - t_start, 1), history=hist)
print(json.dumps(out))
