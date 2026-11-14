"""TOY BOARDS ONLY. Where the time goes in the low-rank NumPy solver (bmcopy/bm.py, an unmodified
copy of ../lowrank-relaxation/bm.py; see bmcopy/COPIED_SHA256.txt).

The solver is run unchanged except that SphereAL.fV is replaced by a line-for-line copy with a
wall-clock timer around each group of statements (same arithmetic, checked: the timed and the
original fV return the same value and gradient to 1e-12 at the start point). Buckets:

  dense     V_B V^T and W V            (BLAS dgemm; the only P^2 r work)
  elemwise  max(0, mu - rho G), W*W    (P^2 elementwise NumPy passes over the Gram block)
  gather    V[ci] * V[cj] summed       (K x r fancy-index gathers and the row dot products)
  scatter   conflict_matrix(vals) @ V  (CSR copy, value permutation, sparse product: the
                                        scatter-add of the conflict gradient)
  sums      cell and tile sums         (two small sparse products each way)
  sphere    fg(): normalisation and tangent projection
  lbfgs     scipy L-BFGS-B outside fg  (two-loop recursion, line search bookkeeping, Python glue)
  outer     max_violation + update_multipliers + the rest of solve()

Usage: profile_bm.py seed r seconds [n cf ci]      (one process, one BLAS thread)
"""
import os, sys, json, time
os.environ.setdefault("VECLIB_MAXIMUM_THREADS", "1")
os.environ.setdefault("OMP_NUM_THREADS", "1")
os.environ.setdefault("OPENBLAS_NUM_THREADS", "1")
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, os.path.join(HERE, "bmcopy"))
import common  # noqa: F401,E402
import numpy as np  # noqa: E402
import scipy.sparse as sp  # noqa: E402
from board import Board  # noqa: E402
import bm  # noqa: E402

seed, r, seconds = int(sys.argv[1]), int(sys.argv[2]), float(sys.argv[3])
n, cf, ci = (int(a) for a in sys.argv[4:7]) if len(sys.argv) > 6 else (5, 3, 4)
T = dict(dense=0.0, elemwise=0.0, gather=0.0, scatter=0.0, sums=0.0, sphere=0.0, fV_other=0.0)
pc = time.perf_counter
orig_fV = bm.SphereAL.fV
orig_fg = bm.SphereAL.fg
fg_total = [0.0]


def fV_timed(self, V, grad=True):
    S, rho = self.S, self.rho
    t_in = pc()
    acc = 0.0
    x = V[:, 0]
    g = np.zeros_like(V)
    f = float(x @ x); g[:, 0] += 2 * x
    t = pc()
    for Cm, lam in ((S.Ccell, self.lc), (S.Ctile, self.lt)):
        H = self.sums_residual(Cm, V)
        f += (lam * H).sum() + 0.5 * rho * (H * H).sum()
        g += Cm.T @ (lam + rho * H)
    d = pc() - t; T["sums"] += d; acc += d
    t = pc()
    hz = (V[S.ci] * V[S.cj]).sum(1)
    d = pc() - t; T["gather"] += d; acc += d
    wz = np.maximum(0.0, -rho * hz)
    f += self.lz @ hz + 0.5 * rho * hz @ hz - (wz @ wz) / (2 * rho)
    t = pc()
    g += S.conflict_matrix(self.lz + rho * hz + wz) @ V
    d = pc() - t; T["scatter"] += d; acc += d
    if self._mu is None:
        mus = [self.M2[rows].toarray() for rows in S.blocks]
        mu2 = sum(float((m * m).sum()) for m in mus)
        if self.cache_ok:
            self._mu = (mus, mu2)
    else:
        mus, mu2 = self._mu
    f -= 0.5 * mu2 / (2 * rho)
    for rows, mu in zip(S.blocks, mus):
        t = pc()
        G = V[rows] @ V.T
        d = pc() - t; T["dense"] += d; acc += d
        t = pc()
        W = np.maximum(0.0, mu - rho * G)
        f += 0.5 * (W * W).sum() / (2 * rho)
        d = pc() - t; T["elemwise"] += d; acc += d
        t = pc()
        g[rows] -= W @ V
        d = pc() - t; T["dense"] += d; acc += d
    T["fV_other"] += pc() - t_in - acc
    return f, g


def fg_timed(self, w):
    t = pc()
    S, r_ = self.S, self.r
    self.nevals += 1
    Wt = w.reshape(S.P, r_)
    nrm = np.linalg.norm(Wt, axis=1, keepdims=True)
    U = Wt / nrm
    t1 = pc()
    f, gV = self.fV(bm.V_of_U(U))
    t2 = pc()
    gU = 0.5 * gV
    gW = (gU - (gU * U).sum(1, keepdims=True) * U) / nrm
    out = f, gW.ravel()
    t3 = pc()
    T["sphere"] += (t1 - t) + (t3 - t2)
    fg_total[0] += t3 - t
    return out


B = Board(n, cf, ci, seed); S = bm.Structure(B)
rng = np.random.default_rng(1000 * r)
U0 = bm.init_U(B, r, rng)

# check: timed fV equals the original
al = bm.SphereAL(S, r, rho=10.0, vec=True)
f0, g0 = orig_fV(al, bm.V_of_U(U0)); f1, g1 = fV_timed(al, bm.V_of_U(U0))
assert abs(f0 - f1) <= 1e-12 * abs(f0) and np.abs(g0 - g1).max() <= 1e-12 * np.abs(g0).max()
for k in T:
    T[k] = 0.0

bm.SphereAL.fV = fV_timed
bm.SphereAL.fg = fg_timed
minimize_time = [0.0]
orig_minimize = bm.minimize


def minimize_timed(*a, **k):
    t = pc(); res = orig_minimize(*a, **k); minimize_time[0] += pc() - t
    return res


bm.minimize = minimize_timed
t0 = pc()
V, info = bm.solve(S, r, U0, time_limit=seconds)
total = pc() - t0
T["lbfgs"] = minimize_time[0] - fg_total[0]
T["outer"] = total - minimize_time[0]
x = V[:, 0]
sol = np.zeros(B.P, bool)
for s in B.sol_idx:
    sol[s] = True
rec = dict(board=[n, cf, ci, seed], r=r, P=B.P, K=int(S.K), F=int(S.F), blocks=len(S.blocks),
           seconds=round(total, 2), evals=info["evals"], outer=info["outer"], status=info["status"],
           final_rho=info["final_rho"], viol=info["viol"]["max"],
           mass_off_per_cell=float(np.clip(x, 0, None)[~sol].sum() / B.N),
           ms_per_eval=round(1e3 * fg_total[0] / info["evals"], 3),
           seconds_by_bucket={k: round(v, 2) for k, v in T.items()},
           share_by_bucket={k: round(v / total, 4) for k, v in T.items()},
           inner_iters_per_outer=[h["inner"] for h in info["hist"]],
           viol_hist=[[h["outer"], h["rho"], h["viol"], h["t"]] for h in info["hist"][::5]],
           loadavg=os.getloadavg())
print(json.dumps(rec))
