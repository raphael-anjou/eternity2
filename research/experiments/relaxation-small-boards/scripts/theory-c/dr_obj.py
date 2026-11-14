"""TOY BOARDS ONLY (5x5). The facially reduced Douglas-Rachford splitting of
../relaxation-engineering/admm.py with the objective as the only change, used here as a
DIAGNOSTIC of the convex program (it needs a dense eigendecomposition, so it is not a method for
large boards).

  mode l2   : ||x||^2 / rho         (admm.py as it is)
  mode feas : no objective          (then the iteration has NO penalty parameter at all)
  mode lin  : tau <c, x>, c uniform on [0, 1]

Logged every `every` iterations: residuals, largest violation, mass off solutions, the norm of the
fixed-point variable v (it diverges when the dual is not attained), and whether the greedy rounding
of x is a solution (first iteration where it is = the number a solver cares about).
At the end, two complementarity counts at the last iterate:
  spectrum of W^T v W: positive part = primal Z, negative part = dual slack; eigenvalues near 0 are
  directions where strict complementarity fails;
  entries of M = 2 Z - v on the free pairs: M > 0 primal positive, M < 0 multiplier positive,
  M near 0 neither.

Optional congruence scale sc (default 1): the iteration runs on Zs = Dg Z Dg, Dg = diag(1, sc, ...,
sc), i.e. the lifted vector is (1, sc x). PSD is unchanged by a congruence, but the Frobenius
metric is not: the border entries weigh sc^2 and the Y entries sc^4 against the corner. For pure
feasibility this is the ONLY metric parameter the splitting has (rho multiplies the objective only).

Usage: dr_obj.py seed mode param cseed max_iters seconds out.jsonl [every] [sc]
  param = rho for l2, tau for lin, ignored for feas
"""
import os, sys, json, time
os.environ.setdefault("VECLIB_MAXIMUM_THREADS", "1")
os.environ.setdefault("OMP_NUM_THREADS", "1")
os.environ.setdefault("OPENBLAS_NUM_THREADS", "1")
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import common  # noqa: F401,E402
import numpy as np  # noqa: E402
from board import Board  # noqa: E402
import bm  # noqa: E402

seed = int(sys.argv[1]); mode = sys.argv[2]; param = float(sys.argv[3]); cseed = int(sys.argv[4])
max_iters = int(sys.argv[5]); seconds = float(sys.argv[6]); out = sys.argv[7]
every = int(sys.argv[8]) if len(sys.argv) > 8 else 25
sc = float(sys.argv[9]) if len(sys.argv) > 9 else 1.0
alpha = 1.6

B = Board(5, 3, 4, seed); S = bm.Structure(B)
P, N, T = B.P, B.N, B.T
D = P + 1
conf = np.zeros((D, D), bool)
conf[S.ci + 1, S.cj + 1] = True; conf[S.cj + 1, S.ci + 1] = True
C = np.vstack([S.Ccell.toarray(), S.Ctile.toarray()])
CCt_pinv = np.linalg.pinv(C @ C.T)
sol = np.zeros(P, bool)
for s in B.sol_idx:
    sol[s] = True
idx = np.arange(1, D)
Wn = np.hstack([np.ones((N + T, 1)), -C / sc])
sv_s, sv_vt = np.linalg.svd(Wn, full_matrices=True)[1:]
rank_w = int((sv_s > 1e-9 * sv_s[0]).sum())
Wb = np.ascontiguousarray(sv_vt[rank_w:].T)
cvec = np.random.default_rng(cseed).uniform(0, 1, P)
free = ~conf; free[0, :] = False; free[:, 0] = False; free[np.arange(D), np.arange(D)] = False


def proj_A(M):
    Y = np.maximum(M, 0.0)
    Y[conf] = 0.0
    xbar = (2.0 * sc * M[0, 1:] + sc * sc * M[idx, idx]) / (2.0 * sc * sc + sc ** 4)   # weighted fit of x
    if mode == "l2":
        x0 = 3.0 * param * xbar / (3.0 * param + 2.0)
    elif mode == "lin":
        x0 = xbar - param * cvec / 3.0
    else:
        x0 = xbar
    x = x0 - C.T @ (CCt_pinv @ (C @ x0 - 1.0))
    Y[0, 0] = 1.0; Y[0, 1:] = sc * x; Y[1:, 0] = sc * x; Y[idx, idx] = sc * sc * x
    return Y


def proj_psd(M):
    w, Q = np.linalg.eigh(Wb.T @ M @ Wb)
    k = int(np.searchsorted(w, 0.0))
    Qp = Wb @ Q[:, k:]
    return (Qp * w[k:]) @ Qp.T, len(w) - k


def viol_max(Z):
    x = Z[0, 1:] / sc; Yb = Z[1:, 1:] / (sc * sc)
    off = Yb.copy(); off[idx - 1, idx - 1] = 0.0
    return float(max(np.abs(C @ x - 1).max(), np.abs(Yb[idx - 1, idx - 1] - x).max(),
                     np.abs(Yb[conf[1:, 1:]]).max(), max(0.0, -np.where(conf[1:, 1:], 0.0, off).min())))


Z = np.zeros((D, D)); Z[0, 0] = 1.0
x0 = B.X0.copy(); Z[0, 1:] = sc * x0; Z[1:, 0] = sc * x0; Z[1:, 1:] = sc * sc * np.outer(x0, x0); Z[idx, idx] = sc * sc * x0
v = Z.copy()
t0 = time.time(); it = 0; first_round = None; first_mass = {}
rows = []
while True:
    Zn, npos = proj_psd(v)
    Yk = proj_A(2.0 * Zn - v)
    rp = float(np.linalg.norm(Yk - Zn)); rd = float(np.linalg.norm(Zn - Z))
    Z = Zn
    v = v + alpha * (Yk - Zn)
    it += 1
    el = time.time() - t0
    done = it >= max_iters or el > seconds or (rp < 1e-8 and rd < 1e-8)
    if it % every == 0 or done:
        x = Z[0, 1:] / sc
        js = bm.greedy_round(B, x)
        ok = B.mismatches(js.tolist()) == 0
        if ok and first_round is None:
            first_round = it
        mass = float(np.clip(x, 0, None)[~sol].sum() / N)
        for th in (1e-2, 1e-3, 1e-5):
            if mass < th and th not in first_mass:
                first_mass[th] = it
        rec = dict(it=it, t=round(el, 1), rp=rp, rd=rd, npos=npos, viol=viol_max(Z), mass_off=mass,
                   vnorm=float(np.linalg.norm(v)), round_ok=bool(ok), xmax=float(x.max()),
                   near_int=float(np.minimum(x, 1 - x).clip(0).sum() / N), linobj=float(cvec @ x), l2=float(x @ x))
        rows.append(rec)
        if it % (every * 8) == 0 or done:
            print(json.dumps(rec), flush=True)
    if done:
        break
ev = np.linalg.eigvalsh(Wb.T @ v @ Wb)
spec_sc = np.abs(ev).max()
M = (2.0 * Z - v)[free]
x = Z[0, 1:] / sc
zr = np.linalg.eigvalsh(Z)
fin = dict(final=True, sc=sc, seed=seed, n_sols=len(B.sols), mode=mode, param=param, cseed=cseed, iters=it,
           seconds=round(time.time() - t0, 1), face_order=int(Wb.shape[1]), first_round=first_round,
           first_mass={str(k): val for k, val in first_mass.items()}, rp=rp, rd=rd,
           rank_Z_1e6=int((zr > 1e-6 * zr[-1]).sum()), rank_Z_1e3=int((zr > 1e-3 * zr[-1]).sum()),
           spec={f"{tol:g}": dict(pos=int((ev > tol * spec_sc).sum()), neg=int((ev < -tol * spec_sc).sum()),
                                  near0=int((np.abs(ev) <= tol * spec_sc).sum())) for tol in (1e-4, 1e-6, 1e-8)},
           spec_scale=float(spec_sc), spec_min=float(ev.min()), spec_max=float(ev.max()),
           free_entries={f"{tol:g}": dict(pos=int((M > tol).sum() // 2), neg=int((M < -tol).sum() // 2),
                                          near0=int((np.abs(M) <= tol).sum() // 2)) for tol in (1e-4, 1e-6, 1e-8)},
           x_int=int(((x > 0.99) | (x < 0.01)).sum()), P=P, loadavg=os.getloadavg()[0])
print(json.dumps(fin), flush=True)
with open(out, "a") as f:
    f.write(json.dumps(dict(fin, rows=rows)) + "\n")
