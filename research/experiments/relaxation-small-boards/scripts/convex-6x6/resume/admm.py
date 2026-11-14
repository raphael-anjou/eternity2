"""TOY BOARDS ONLY. Copy of ../relaxation-engineering/admm.py (SHA-256 of the original in
COPIED_SHA256.txt) with the additions that memo's plan, step 1, asks for:
  - objective "leak": maximise the x mass on placements that belong to no enumerated solution
    (linear); objective "l2" is the original minimum-norm point;
  - a residual-balanced automatic penalty: every BAL_EVERY iterations, rho times 2 if the primal
    residual ||Y - Z|| exceeds 10 times the step ||Z_k - Z_(k-1)||, divided by 2 in the opposite
    case (both in the units of v; the textbook form, which multiplies the step by rho, parked rho
    near 5 and stalled on 5x5 seed 1, see ../convex-6x6.md). On a change, v - Z is rescaled so the
    multiplier rho (Z - v) is unchanged and proj_face(v) is the same matrix;
  - a stop when ||Y - Z|| and ||Z_k - Z_(k-1)|| are both below 1e-7 (the original's own test,
    Frobenius norms); rho ||Z_k - Z_(k-1)|| is logged as r_dual, not used;
  - a checkpoint (v, rho, iteration, elapsed) and one progress line every 100 iterations, appended
    to OUT/PROGRESS_<objective>_<n>_<cf>_<ci>_<seed>.txt; a rerun of the same command resumes.
Removed: Anderson acceleration and the fr = 0 path (the memo dropped both). Facial reduction is on.

Same lift as ../geometry-sdp-lift/lift.py, through bm.py's constraint list:
  Z = [[1, x^T], [x, Y]] PSD, Z[0,p] = Z[p,p] = x_p, cell and tile sums 1, Y = 0 on conflict pairs,
  Y >= 0 on free pairs (row sums implied, measured only).
Douglas-Rachford on v:  Z = proj_face(v),  Y = prox_{f/rho + A}(2 Z - v),  v = v + alpha (Y - Z).
  l2:   f = ||x||^2,          x = rho (2 a + b) / (3 rho + 2)
  leak: f = -sum_{p off} x_p, x = (2 a + b + c / rho) / 3,  c = indicator of placements in no solution
  (a = the two off-diagonal copies of x, b = the diagonal copy), then the affine sums.

Usage: admm.py n cf ci seed objective rho0 seconds out_dir
"""
import os, sys, json, time
for _k in ("VECLIB_MAXIMUM_THREADS", "OMP_NUM_THREADS", "OPENBLAS_NUM_THREADS", "MKL_NUM_THREADS"):
    os.environ.setdefault(_k, "1")
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import common  # noqa: F401,E402
import numpy as np  # noqa: E402
from board import Board  # noqa: E402
import bm  # noqa: E402

n, cf, ci, seed = map(int, sys.argv[1:5])
objective = sys.argv[5]
assert objective in ("l2", "leak")
rho, seconds, out = float(sys.argv[6]), float(sys.argv[7]), sys.argv[8]
alpha = 1.6
TOL, LOG_EVERY, BAL_EVERY, BAL_MU, BAL_TAU, RHO_MIN, RHO_MAX = 1e-7, 100, 50, 10.0, 2.0, 1e-3, 1e7
os.makedirs(out, exist_ok=True)
tag = f"{objective}_{n}_{cf}_{ci}_{seed}"
prog_path = os.path.join(out, f"PROGRESS_{tag}.txt")
ck_v, ck_meta = os.path.join(out, f"ckpt_{tag}.npy"), os.path.join(out, f"ckpt_{tag}.json")


def log(rec):
    with open(prog_path, "a") as fh:
        fh.write(json.dumps(rec) + "\n")
    print(json.dumps(rec), flush=True)


t_build = time.time()
B = Board(n, cf, ci, seed); S = bm.Structure(B)
P, N, T = B.P, B.N, B.T
D = P + 1
conf = np.zeros((D, D), bool)
conf[S.ci + 1, S.cj + 1] = True; conf[S.cj + 1, S.ci + 1] = True
C = np.vstack([S.Ccell.toarray(), S.Ctile.toarray()])          # (N + T) x P
CCt_pinv = np.linalg.pinv(C @ C.T, rcond=1e-10)                # resume/: the default 1e-15 keeps noise singular values on x86 (entries 1e11)
sol = np.zeros(P, bool)
for s in B.sol_idx:
    sol[s] = True
leakw = (~sol).astype(float)                                    # c of the leak objective
idx = np.arange(1, D)
Wn = np.hstack([np.ones((N + T, 1)), -C])                       # rows are the w vectors
sv_u, sv_s, sv_vt = np.linalg.svd(Wn, full_matrices=True)
rank_w = int((sv_s > 1e-9 * sv_s[0]).sum())
Wb = np.ascontiguousarray(sv_vt[rank_w:].T)                     # D x (D - rank_w), orthonormal


def proj_A(M):
    """prox of f / rho plus the indicator of A, in the Frobenius metric. M symmetric."""
    Y = np.maximum(M, 0.0)
    Y[conf] = 0.0
    if objective == "l2":
        x0 = rho * (2.0 * M[0, 1:] + M[idx, idx]) / (3.0 * rho + 2.0)
    else:
        x0 = (2.0 * M[0, 1:] + M[idx, idx] + leakw / rho) / 3.0
    x = x0 - C.T @ (CCt_pinv @ (C @ x0 - 1.0))
    Y[0, 0] = 1.0; Y[0, 1:] = x; Y[1:, 0] = x; Y[idx, idx] = x
    return Y


def proj_psd(M):
    """Projection on the face: W proj_PSD(W^T M W) W^T, with the number of positive eigenvalues."""
    w, Q = np.linalg.eigh(Wb.T @ M @ Wb)
    k = int(np.searchsorted(w, 0.0))
    Qp = Wb @ Q[:, k:]
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


MAX_IT = int(os.environ.get("ADMM_MAX_IT", "0"))              # resume/: optional iteration cap (tests)
ck_z = os.path.join(out, f"ckpt_{tag}_Z.npy")                  # resume/: previous Z, for an exact resume
resumed = os.path.exists(ck_meta)
rp = rd = s_dual = np.inf
if resumed:
    meta = json.load(open(ck_meta)); v = np.load(ck_v)
    it, rho, t_prev = meta["it"], meta["rho"], meta["elapsed"]
    if "rp" in meta and os.path.exists(ck_z):                  # resume/: restore the balance inputs and Z
        rp, rd, s_dual = meta["rp"], meta["rd"], rho * meta["rd"]
else:
    Z0 = np.zeros((D, D)); Z0[0, 0] = 1.0
    x0 = B.X0.copy(); Z0[0, 1:] = x0; Z0[1:, 0] = x0; Z0[1:, 1:] = np.outer(x0, x0); Z0[idx, idx] = x0
    v = Z0
    it, t_prev = 0, 0.0
Z = np.load(ck_z) if np.isfinite(rp) else proj_psd(v)[0]
log(dict(start=True, resumed=resumed, exact_resume=bool(resumed and np.isfinite(rp)), board=[n, cf, ci, seed], objective=objective, P=P, K=int(S.K), F=int(S.F),
         face_order=int(Wb.shape[1]), n_sols=len(B.sols), n_off=int((~sol).sum()), rho=rho, alpha=alpha, it=it,
         build_s=round(time.time() - t_build, 1), host=os.uname().nodename, threads=os.environ.get("OPENBLAS_NUM_THREADS"),
         clock=time.strftime("%H:%M:%S")))
t0 = time.time() - t_prev; t_eig = 0.0; t_run0 = time.time(); it_run0 = it
status = "time_limit"
while True:
    te = time.time()
    Znew, npos = proj_psd(v)
    t_eig += time.time() - te
    if it > 0 and it % BAL_EVERY == 0 and np.isfinite(rp):
        new = rho * BAL_TAU if rp > BAL_MU * rd else rho / BAL_TAU if rd > BAL_MU * rp else rho
        new = min(RHO_MAX, max(RHO_MIN, new))
        if new != rho:
            v = Znew + (v - Znew) * (rho / new)    # same multiplier rho (Z - v); proj_face(v) unchanged
            rho = new
    Yk = proj_A(2.0 * Znew - v)
    rp = float(np.linalg.norm(Yk - Znew)); rd = float(np.linalg.norm(Znew - Z)); s_dual = rho * rd
    Z = Znew
    v = v + alpha * (Yk - Znew)
    it += 1
    el = time.time() - t0
    done = max(rp, rd) < TOL
    capped = MAX_IT > 0 and it >= MAX_IT
    if done:
        status = "converged"
    if it % LOG_EVERY == 0 or done or el > seconds or capped:
        x = Z[0, 1:]
        log(dict(it=it, t=round(el, 1), s_per_it=round((time.time() - t_run0) / max(1, it - it_run0), 4),
                 r_primal=rp, r_step=rd, r_dual=s_dual, rho=rho, npos=npos,
                 mass_off_per_cell=float(np.clip(x, 0, None)[~sol].sum() / N), leak_obj=float(x @ leakw),
                 l2_obj=float(x @ x), viol_max=max(violations(Z).values()), clock=time.strftime("%H:%M:%S")))
        np.save(ck_v + ".tmp.npy", v); os.replace(ck_v + ".tmp.npy", ck_v)
        np.save(ck_z + ".tmp.npy", Z); os.replace(ck_z + ".tmp.npy", ck_z)
        with open(ck_meta + ".tmp", "w") as fh:
            json.dump(dict(it=it, rho=rho, elapsed=el, rp=rp, rd=rd), fh)
        os.replace(ck_meta + ".tmp", ck_meta)
    if done or el > seconds or capped:
        break
ev = np.linalg.eigvalsh(Z)
log(dict(final=True, status=status, board=[n, cf, ci, seed], objective=objective, P=P, face_order=int(Wb.shape[1]),
         iters=it, seconds=round(time.time() - t0, 1), r_primal=rp, r_step=rd, r_dual=s_dual, rho=rho,
         eig_share_this_run=round(t_eig / max(1e-9, time.time() - t_run0), 3), viol=violations(Z),
         mass_off_per_cell=float(np.clip(Z[0, 1:], 0, None)[~sol].sum() / N),
         rank_1e3=int((ev > 1e-3 * ev[-1]).sum()), top_eigs=[round(float(e), 4) for e in ev[::-1][:8]],
         n_sols=len(B.sols), loadavg=os.getloadavg(), clock=time.strftime("%H:%M:%S")))
