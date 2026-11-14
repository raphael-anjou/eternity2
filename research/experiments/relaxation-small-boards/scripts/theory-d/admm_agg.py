"""AGGREGATED VARIANT (see the block marked AGG below): every pair zero is replaced by vector
equalities put in the face: cell sums, tile sums, and for each inner edge and colour, "the sum of the
placements of one cell showing that colour on the shared side equals the same sum from the other
cell". No entrywise zero is imposed. knn here: 0 = no entrywise constraint at all; 1 = Y >= 0 on the
local pairs (same cell, same tile, adjacent cells), which by the memo's derivation gives back every
zero. kz and tilefr are ignored.

TOY BOARDS ONLY (5x5). Copy of ../relaxation-engineering/admm.py (matrix-free splitting
with facial reduction) with three switches that WEAKEN the lift, to see which parts carry exactness:

  knn    entrywise nonnegativity Y[p,q] >= 0 only for pairs whose cells are within Chebyshev
         distance knn (99 = all pairs = the full lift; 0 = none = theta form with the zeros)
  kz     same-tile zeros Y[p,q] = 0 only for cells within distance kz (99 = all)
  tilefr 1 = tile vectors in the face (tile row sums hold); 0 = cells only, tile sums linear in x
  obj    l2 (minimise ||x||^2) or leak (maximise the mass on placements no solution uses)

A feasible point of a weakened form with mass off the solutions proves that form inexact on that
board. A point with none is a reading, not a certificate, unless obj = leak.

Usage: admm_var.py n cf ci seed rho seconds knn kz tilefr obj [log_every]
"""
import os, sys, json, time
os.environ.setdefault("VECLIB_MAXIMUM_THREADS", "1")
os.environ.setdefault("OMP_NUM_THREADS", "1")
os.environ.setdefault("OPENBLAS_NUM_THREADS", "1")
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, os.path.join(HERE, "bmcopy"))
import common  # noqa: F401,E402
import numpy as np  # noqa: E402
from board import Board  # noqa: E402
import bm  # noqa: E402

n, cf, ci, seed = map(int, sys.argv[1:5])
assert n <= 5, "5x5 at most (project's decision)"
rho, seconds = float(sys.argv[5]), float(sys.argv[6])
knn, kz, tilefr = int(sys.argv[7]), int(sys.argv[8]), int(sys.argv[9])
obj = sys.argv[10]
log_every = int(sys.argv[11]) if len(sys.argv) > 11 else 500
alpha = 1.6

B = Board(n, cf, ci, seed); S = bm.Structure(B)
P, N, T = B.P, B.N, B.T
D = P + 1
rr, cc = B.cellOf // n, B.cellOf % n
dist = np.maximum(np.abs(rr[:, None] - rr[None, :]), np.abs(cc[:, None] - cc[None, :]))
same_tile = (B.tileOf[:, None] == B.tileOf[None, :]) & (B.cellOf[:, None] != B.cellOf[None, :])
conf = np.zeros((P, P), bool)
conf[S.ci, S.cj] = True; conf[S.cj, S.ci] = True
offd = ~np.eye(P, dtype=bool)
# AGG: no zero mask; nonnegativity on the local pairs only when knn = 1
zero = np.zeros((P, P), bool)
same_cell = B.cellOf[:, None] == B.cellOf[None, :]
nn = offd & (same_cell | same_tile | (dist <= 1)) if knn == 1 else np.zeros((P, P), bool)
zeroD = np.zeros((D, D), bool); zeroD[1:, 1:] = zero
nnD = np.zeros((D, D), bool); nnD[1:, 1:] = nn
Cc, Ct = S.Ccell.toarray(), S.Ctile.toarray()
C = np.vstack([Cc, Ct])
CCt_pinv = np.linalg.pinv(C @ C.T)
sol = np.zeros(P, bool)
for s in B.sol_idx:
    sol[s] = True
w_leak = (~sol).astype(float)
idx = np.arange(1, D)
erows = []
for xx in range(N):
    for d in (1, 2):
        yy = int(B.nb[xx, d])
        if yy < 0:
            continue
        for col in sorted(set(B.cols[B.cellOf == xx, d].tolist()) | set(B.cols[B.cellOf == yy, (d + 2) % 4].tolist())):
            w_ = np.zeros(P)
            w_[(B.cellOf == xx) & (B.cols[:, d] == col)] = 1.0
            w_[(B.cellOf == yy) & (B.cols[:, (d + 2) % 4] == col)] -= 1.0
            erows.append(w_)
Ecol = np.array(erows)
Wn = np.vstack([np.hstack([np.ones((C.shape[0], 1)), -C]), np.hstack([np.zeros((Ecol.shape[0], 1)), Ecol])])
sv_u, sv_s, sv_vt = np.linalg.svd(Wn, full_matrices=True)
rank_w = int((sv_s > 1e-9 * sv_s[0]).sum())
Wb = np.ascontiguousarray(sv_vt[rank_w:].T)


def proj_A(M):
    Y = M.copy()
    Y[nnD] = np.maximum(M[nnD], 0.0)
    Y[zeroD] = 0.0
    if obj == "l2":
        x0 = rho * (2.0 * M[0, 1:] + M[idx, idx]) / (3.0 * rho + 2.0)
    else:
        x0 = (2.0 * M[0, 1:] + M[idx, idx]) / 3.0 + w_leak / (3.0 * rho)
    x = x0 - C.T @ (CCt_pinv @ (C @ x0 - 1.0))
    Y[0, 0] = 1.0; Y[0, 1:] = x; Y[1:, 0] = x; Y[idx, idx] = x
    return Y


def proj_psd(M):
    M = Wb.T @ M @ Wb
    w, Q = np.linalg.eigh(M)
    k = int(np.searchsorted(w, 0.0))
    Qp = Wb @ Q[:, k:]
    return (Qp * w[k:]) @ Qp.T, len(w) - k


def violations(Z):
    x = Z[0, 1:]; Yb = Z[1:, 1:]
    d = dict(sums=float(np.abs(C @ x - 1).max()), diag=float(np.abs(np.diag(Yb) - x).max()),
             zero=0.0, x_neg=float(max(0.0, -x.min())))
    d["nn"] = float(max(0.0, -Yb[nn].min())) if nn.any() else 0.0
    return d


def full_lift_gap(Z):
    """How far the point is from the FULL lift (not imposed here): diagnostics only."""
    Yb = Z[1:, 1:]
    free = offd & ~conf
    return dict(min_free_entry=float(Yb[free].min()), neg_free_share=float((Yb[free] < -1e-6).mean()),
                max_abs_conflict=float(np.abs(Yb[conf]).max()),
                max_abs_tile_zero_dropped=float(np.abs(Yb[conf & ~zero]).max()) if (conf & ~zero).any() else 0.0)


Z = np.zeros((D, D)); Z[0, 0] = 1.0
x0 = B.X0.copy(); Z[0, 1:] = x0; Z[1:, 0] = x0; Z[1:, 1:] = np.outer(x0, x0); Z[idx, idx] = x0
v = Z.copy()
t0 = time.time(); it = 0
tag = dict(board=[n, cf, ci, seed], P=P, rho=rho, knn=knn, kz=kz, tilefr=tilefr, obj=obj,
           n_zero=0, n_edge_colour_vectors=int(Ecol.shape[0]), n_nonneg=int(nn.sum() // 2), n_conf=int(conf.sum() // 2),
           n_free=int((offd & ~conf).sum() // 2), face_order=int(Wb.shape[1]), n_sols=len(B.sols),
           n_sol_placements=int(sol.sum()))
print(json.dumps(tag), flush=True)
while True:
    Znew, npos = proj_psd(v)
    Yk = proj_A(2.0 * Znew - v)
    it += 1
    rd = float(np.linalg.norm(Znew - Z)); rp = float(np.linalg.norm(Yk - Znew))
    Z = Znew
    v = v + alpha * (Yk - Znew)
    el = time.time() - t0
    done = el > seconds or (rp < 1e-7 and rd < 1e-7)
    if it % log_every == 0 or done:
        x = Z[0, 1:]; vv = violations(Z)
        rec = dict(it=it, t=round(el, 1), r_primal=rp, r_dual=rd, npos=npos, obj=float(x @ x),
                   viol_max=max(vv.values()), mass_off_per_cell=float(np.clip(x, 0, None)[~sol].sum() / N),
                   n_off_above_1e3=int((x[~sol] > 1e-3).sum()), n_sol_below_1e4=int((x[sol] < 1e-4).sum()))
        print(json.dumps(rec), flush=True)
        if done:
            break
ev = np.linalg.eigvalsh(Z)
out = dict(final=True, **tag, iters=it, seconds=round(time.time() - t0, 1), viol=violations(Z),
           full_lift_gap=full_lift_gap(Z), rank_1e3=int((ev > 1e-3 * ev[-1]).sum()),
           mass_off_per_cell=rec["mass_off_per_cell"], obj_val=rec["obj"], loadavg=os.getloadavg())
print(json.dumps(out), flush=True)
np.save(os.path.join(HERE, "raw", f"xagg_{n}_{cf}_{ci}_{seed}_knn{knn}_kz{kz}_tf{tilefr}_{obj}_rho{int(rho)}.npy"), Z[0, 1:])
