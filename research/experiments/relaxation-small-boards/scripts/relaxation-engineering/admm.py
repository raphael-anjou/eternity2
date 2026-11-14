"""TOY BOARDS ONLY. Matrix-free two-set splitting for the SAME lift as
../geometry-sdp-lift/lift.py with obj="l2" (through bmcopy/bm.py's constraint list, which
that study checked against lift.py: F free pairs match, row sums implied).

  minimise ||x||^2  over  Z = [[1, x^T], [x, Y]]  in  A  and  Z  in  PSD(P + 1)

  A (polyhedral, closed-form projection):  Z[0,0] = 1,  Z[0,p] = Z[p,0] = Z[p,p] = x_p,
      sum of x over each cell = 1, over each tile = 1,
      Y[p,q] = 0 on the K conflict pairs,  Y[p,q] >= 0 on the F free pairs.
  The row-sum equalities of lift.py are implied by A and PSD together (bm.py, simplification (b));
  they are not imposed, only measured.

Douglas-Rachford with relaxation alpha, as a fixed-point map on one symmetric matrix v:
  Z = proj_PSD(v)                       one symmetric eigendecomposition
  Y = prox_{f/rho + A}(2 Z - v)         closed form: clamp or zero the off-diagonal entries; for x a
                                        scaled average of the three copies then one projection on
                                        the (N + T) sum equalities (pseudo-inverse factored once)
  v = v + alpha (Y - Z)
Memory: four dense matrices of order P + 1 (Z, U, the work copy, the eigenvectors kept) plus a
boolean mask. Flops per iteration: one full eigendecomposition (about 9 P^3) plus P^2 k for the
positive part of rank k, plus O(P^2).

Facial reduction (fr=1, the default): with w_c = e_0 - sum of e_p over cell c (and the same per
tile), every feasible Z has w^T Z w = 1 - 2 + 1 = 0, hence Z w = 0. So Z = W R W^T with W an
orthonormal basis of the complement of the span of the w (dimension P + 1 - (N + T - 1)) and R PSD.
The PSD step becomes W proj_PSD(W^T M W) W^T: the exact projection on that face, of smaller order,
and it enforces every cell sum, tile sum and row sum at each iterate. This is the splitting of
Oliveira, Wolkowicz and Xu for the assignment relaxation (arXiv 1512.05448). fr=0 projects on the
full cone (the first pilot runs).

Usage: admm.py n cf ci seed rho seconds [alpha] [log_every] [fr] [aa_memory]
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
rho, seconds = float(sys.argv[5]), float(sys.argv[6])
alpha = float(sys.argv[7]) if len(sys.argv) > 7 else 1.6
log_every = int(sys.argv[8]) if len(sys.argv) > 8 else 50
fr = int(sys.argv[9]) if len(sys.argv) > 9 else 1
aa = int(sys.argv[10]) if len(sys.argv) > 10 else 0

B = Board(n, cf, ci, seed); S = bm.Structure(B)
P, N, T = B.P, B.N, B.T
D = P + 1
conf = np.zeros((D, D), bool)
conf[S.ci + 1, S.cj + 1] = True; conf[S.cj + 1, S.ci + 1] = True
C = np.vstack([S.Ccell.toarray(), S.Ctile.toarray()])          # (N + T) x P
CCt_pinv = np.linalg.pinv(C @ C.T)
sol = np.zeros(P, bool)
for s in B.sol_idx:
    sol[s] = True
xs_path = os.path.join(HERE, "..", "lowrank-relaxation", "raw", f"x_sdp_{n}_{cf}_{ci}_{seed}.npy")
x_sdp = np.load(xs_path) if os.path.exists(xs_path) else None    # read only (the SCS point)
idx = np.arange(1, D)
Wn = np.hstack([np.ones((N + T, 1)), -C])                       # rows are the w vectors
sv_u, sv_s, sv_vt = np.linalg.svd(Wn, full_matrices=True)
rank_w = int((sv_s > 1e-9 * sv_s[0]).sum())
Wb = np.ascontiguousarray(sv_vt[rank_w:].T)                     # D x (D - rank_w), orthonormal


def proj_A(M):
    """prox of ||x||^2 / rho plus the indicator of A, in the Frobenius metric. M symmetric."""
    Y = np.maximum(M, 0.0)
    Y[conf] = 0.0
    x0 = rho * (2.0 * M[0, 1:] + M[idx, idx]) / (3.0 * rho + 2.0)
    x = x0 - C.T @ (CCt_pinv @ (C @ x0 - 1.0))
    Y[0, 0] = 1.0; Y[0, 1:] = x; Y[1:, 0] = x; Y[idx, idx] = x
    return Y


def proj_psd(M):
    """Positive part of a symmetric matrix; returns it with the number of positive eigenvalues."""
    if fr:
        M = Wb.T @ M @ Wb
    w, Q = np.linalg.eigh(M)            # LAPACK dsyevd; the "evr" subset driver was 10x slower here
    k = int(np.searchsorted(w, 0.0))
    Qp = Wb @ Q[:, k:] if fr else Q[:, k:]
    return (Qp * w[k:]) @ Qp.T, len(w) - k


def violations(Z):
    x = Z[0, 1:]
    Yb = Z[1:, 1:]
    off = Yb.copy(); off[idx - 1, idx - 1] = 0.0
    free_min = np.where(conf[1:, 1:], 0.0, off).min()
    Sc = S.Ccell @ Yb; Ut = S.Ctile @ Yb                # sums of Y[q, p] over q in a cell / tile
    own_c = np.ones((N, P), bool); own_c[B.cellOf, np.arange(P)] = False
    own_t = np.ones((T, P), bool); own_t[B.tileOf, np.arange(P)] = False
    return dict(cell_tile_sum=float(np.abs(C @ x - 1).max()), diag=float(np.abs(Yb[idx - 1, idx - 1] - x).max()),
                zero=float(np.abs(Yb[conf[1:, 1:]]).max()), y_nonneg=float(max(0.0, -free_min)),
                row_cell=float(np.abs(Sc - x[None, :])[own_c].max()), row_tile=float(np.abs(Ut - x[None, :])[own_t].max()))


Z = np.zeros((D, D)); Z[0, 0] = 1.0
x0 = B.X0.copy(); Z[0, 1:] = x0; Z[1:, 0] = x0; Z[1:, 1:] = np.outer(x0, x0); Z[idx, idx] = x0
# The iteration as a fixed-point map on one symmetric matrix v (= relaxed point plus multiplier):
#   Z = proj_face(v),  Y = prox_A(2 Z - v),  T(v) = v + alpha (Y - Z).
# aa > 0 adds Anderson acceleration (type II, memory aa) on v, with a safeguard: a step whose
# residual is more than 1.5 times the previous one is rejected, the plain step is taken, and the
# memory is cleared. A rejected step costs one eigendecomposition and is counted as an iteration.
v = Z.copy()
t0 = time.time(); t_eig = 0.0
hist = []
it = 0; rejected = 0
dV, dG = [], []
v_prev = g_prev = v_plain_prev = None
gnorm_prev = np.inf
while True:
    te = time.time()
    Znew, npos = proj_psd(v)
    t_eig += time.time() - te
    Yk = proj_A(2.0 * Znew - v)
    g = alpha * (Yk - Znew)                       # T(v) - v
    gnorm = float(np.linalg.norm(g))
    it += 1
    if aa and v_plain_prev is not None and gnorm > 1.5 * gnorm_prev:
        v = v_plain_prev; v_plain_prev = None; v_prev = g_prev = None; dV, dG = [], []
        rejected += 1
        continue
    rd = float(np.linalg.norm(Znew - Z)); rp = float(np.linalg.norm(Yk - Znew))
    Z = Znew
    v_plain = v + g
    if aa:
        if v_prev is not None:
            dV.append((v - v_prev).ravel()); dG.append((g - g_prev).ravel())
            if len(dV) > aa:
                dV.pop(0); dG.pop(0)
        v_prev, g_prev = v, g
        if dG:
            Gm = np.array(dG).T
            gam = np.linalg.lstsq(Gm.T @ Gm + 1e-10 * np.trace(Gm.T @ Gm) / len(dG) * np.eye(len(dG)), Gm.T @ g.ravel(), rcond=None)[0]
            v_new = v_plain - ((np.array(dV).T + Gm) @ gam).reshape(D, D)
            v_plain_prev = v_plain
        else:
            v_new = v_plain; v_plain_prev = None
        gnorm_prev = gnorm
        v = v_new
    else:
        v = v_plain
    el = time.time() - t0
    if it % log_every == 0 or el > seconds or (rp < 1e-7 and rd < 1e-7):
        x = Z[0, 1:]
        vv = violations(Z)
        rec = dict(it=it, t=round(el, 1), r_primal=rp, r_dual=rd, npos=npos, obj=float(x @ x),
                   viol_max=max(vv.values()), mass_off_per_cell=float(np.clip(x, 0, None)[~sol].sum() / N))
        if x_sdp is not None:
            rec["linf_vs_scs"] = float(np.abs(x - x_sdp).max())
        hist.append(rec)
        print(json.dumps(rec), flush=True)
        if el > seconds or (rp < 1e-7 and rd < 1e-7):
            break
ev = np.linalg.eigvalsh(Z)
print(json.dumps(dict(final=True, board=[n, cf, ci, seed], P=P, K=int(S.K), F=int(S.F), rho=rho, alpha=alpha, fr=fr, aa=aa, aa_rejected=rejected, face_order=int(Wb.shape[1]) if fr else D,
                      iters=it, seconds=round(time.time() - t0, 1), ms_per_iter=round(1e3 * (time.time() - t0) / it, 2),
                      eig_share=round(t_eig / (time.time() - t0), 3), viol=violations(Z),
                      rank_1e3=int((ev > 1e-3 * ev[-1]).sum()), top_eigs=[round(float(e), 4) for e in ev[::-1][:8]],
                      n_sols=len(B.sols), obj_scs=None if x_sdp is None else float(x_sdp @ x_sdp),
                      loadavg=os.getloadavg())), flush=True)
