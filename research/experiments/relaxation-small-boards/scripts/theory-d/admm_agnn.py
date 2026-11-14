"""CLASS-AGGREGATED VARIANT (block marked AGNN). Entrywise Y >= 0 is kept on pairs of cells within
distance knn. For every farther pair of cells the 1.2e10-pair condition is replaced by block sums:
the placements of each cell are partitioned into classes by their colours on the sides named in
argv[12] (for example N, NW, NESW), and for each pair of classes of two far cells the SUM of Y over
the free pairs of the block must be nonnegative. Implied by the full lift (a sum of nonnegative
entries), so this is a valid relaxation between the local form and the full lift; with NESW the
classes are single placements up to tile repeats and the form is the full lift.

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
zero = conf & ~(same_tile & (dist > kz))
offd = ~np.eye(P, dtype=bool)
nn = offd & (dist <= knn) & ~zero
# AGNN: classes and far free blocks
sides = sys.argv[12]
key = {}
cls = np.zeros(P, int)
for p in range(P):
    k_ = (int(B.cellOf[p]),) + tuple(int(B.cols[p, "NESW".index(ch)]) for ch in sides)
    cls[p] = key.setdefault(k_, len(key))
G = len(key)
far = offd & (dist > knn) & ~zero            # free pairs handled by block sums
Mc = np.zeros((P, G)); Mc[np.arange(P), cls] = 1.0
cnt = Mc.T @ far.astype(float) @ Mc
cnt_safe = np.maximum(cnt, 1.0)
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
Cface = C if tilefr else Cc
Wn = np.hstack([np.ones((Cface.shape[0], 1)), -Cface])
sv_u, sv_s, sv_vt = np.linalg.svd(Wn, full_matrices=True)
rank_w = int((sv_s > 1e-9 * sv_s[0]).sum())
Wb = np.ascontiguousarray(sv_vt[rank_w:].T)


def proj_A(M):
    Y = M.copy()
    Y[nnD] = np.maximum(M[nnD], 0.0)
    Y[zeroD] = 0.0
    Yb = Y[1:, 1:]
    bs = Mc.T @ (Yb * far) @ Mc                      # block sums over far free pairs
    corr = np.minimum(bs, 0.0) / cnt_safe            # exact projection: the blocks are disjoint
    Yb -= corr[cls[:, None], cls[None, :]] * far
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
             zero=float(np.abs(Yb[zero]).max()), x_neg=float(max(0.0, -x.min())))
    d["nn"] = float(max(0.0, -Yb[nn].min())) if nn.any() else 0.0
    d["block"] = float(max(0.0, -(Mc.T @ (Yb * far) @ Mc).min()))
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
           n_zero=int(zero.sum() // 2), sides=sides, n_classes=G, n_far_blocks=int((cnt > 0).sum() // 2), n_nonneg=int(nn.sum() // 2), n_conf=int(conf.sum() // 2),
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
np.save(os.path.join(HERE, "raw", f"xagnn{sides}_{n}_{cf}_{ci}_{seed}_knn{knn}_kz{kz}_tf{tilefr}_{obj}_rho{int(rho)}.npy"), Z[0, 1:])
