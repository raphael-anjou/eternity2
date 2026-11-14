"""TOY BOARDS ONLY. Experiment 4: the convex lift with entrywise nonnegativity kept only on
pairs of placements whose cells are within lattice (Manhattan) distance d.

This is the relaxation-engineering study's splitting (../relaxation-engineering/admm.py:
Douglas-Rachford, facial reduction, objective ||x||^2 / rho), copied without its Anderson option,
with ONE change: the clamp Y >= 0 acts only on the masked pairs. d = 0 keeps no nonnegativity
(the theta form with the sums), d >= 2 n - 2 keeps all of it (the full lift).
At 16x16 the masked pairs number about 2e7 (d = 1) and 6e8 (d = 2) against 1.2e10 in full.
Reading: mass per cell on placements that are in no solution, at small violation.
Usage: local_nonneg.py n cf ci seed d rho seconds
"""
import os, sys, json, time
os.environ.setdefault("VECLIB_MAXIMUM_THREADS", "1")
os.environ.setdefault("OMP_NUM_THREADS", "1")
os.environ.setdefault("OPENBLAS_NUM_THREADS", "1")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
from cp_common import load

n, cf, ci, seed, d = map(int, sys.argv[1:6])
rho, seconds = float(sys.argv[6]), float(sys.argv[7])
alpha = 1.6; log_every = 250
B, S, _ = load(n, cf, ci, seed)
P, N, T = B.P, B.N, B.T
D = P + 1
conf = np.zeros((D, D), bool)
conf[S.ci + 1, S.cj + 1] = True; conf[S.cj + 1, S.ci + 1] = True
rr, cc = np.divmod(B.cellOf, n)
dist = np.abs(rr[:, None] - rr[None, :]) + np.abs(cc[:, None] - cc[None, :])
clamp = np.zeros((D, D), bool)
clamp[1:, 1:] = (dist <= d) & ~conf[1:, 1:]
clamp[np.arange(D), np.arange(D)] = False
if d == 0:
    clamp[:] = False
C = np.vstack([S.Ccell.toarray(), S.Ctile.toarray()])
CCt_pinv = np.linalg.pinv(C @ C.T)
sol = np.zeros(P, bool)
for s in B.sol_idx:
    sol[s] = True
idx = np.arange(1, D)
Wn = np.hstack([np.ones((N + T, 1)), -C])
_, sv_s, sv_vt = np.linalg.svd(Wn, full_matrices=True)
rank_w = int((sv_s > 1e-9 * sv_s[0]).sum())
Wb = np.ascontiguousarray(sv_vt[rank_w:].T)


def proj_A(M):
    Y = M.copy()
    Y[clamp] = np.maximum(M[clamp], 0.0)
    Y[conf] = 0.0
    x0 = rho * (2.0 * M[0, 1:] + M[idx, idx]) / (3.0 * rho + 2.0)
    x = x0 - C.T @ (CCt_pinv @ (C @ x0 - 1.0))
    Y[0, 0] = 1.0; Y[0, 1:] = x; Y[1:, 0] = x; Y[idx, idx] = x
    return Y


def proj_psd(M):
    w, Q = np.linalg.eigh(Wb.T @ M @ Wb)
    k = int(np.searchsorted(w, 0.0))
    Qp = Wb @ Q[:, k:]
    return (Qp * w[k:]) @ Qp.T, len(w) - k


Z = np.zeros((D, D)); Z[0, 0] = 1.0
x0 = B.X0.copy(); Z[0, 1:] = x0; Z[1:, 0] = x0; Z[1:, 1:] = np.outer(x0, x0); Z[idx, idx] = x0
v = Z.copy(); t0 = time.time(); it = 0
free_all = ~conf[1:, 1:]; free_all[np.arange(P), np.arange(P)] = False
while True:
    Znew, npos = proj_psd(v)
    Yk = proj_A(2.0 * Znew - v)
    it += 1
    rd = float(np.linalg.norm(Znew - Z)); rp = float(np.linalg.norm(Yk - Znew))
    Z = Znew
    v = v + alpha * (Yk - Znew)
    el = time.time() - t0
    stop = el > seconds or (rp < 1e-7 and rd < 1e-7)
    if it % log_every == 0 or stop:
        x = Z[0, 1:]; Yb = Z[1:, 1:]
        rec = dict(board=[n, cf, ci, seed], d=d, clamped_pairs=int(clamp.sum() // 2), free_pairs=int(S.F), it=it, t=round(el, 1),
                   r_primal=rp, r_dual=rd, npos=npos, obj=float(x @ x),
                   viol_zero=float(np.abs(Yb[conf[1:, 1:]]).max()), viol_sum=float(np.abs(C @ x - 1).max()),
                   viol_clamped=float(max(0.0, -Yb[clamp[1:, 1:]].min())) if clamp.any() else 0.0,
                   most_negative_free_entry=float(Yb[free_all].min()),
                   mass_off_per_cell=float(np.clip(x, 0, None)[~sol].sum() / N), final=bool(stop))
        print(json.dumps(rec), flush=True)
        if stop:
            break
