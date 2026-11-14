"""TOY BOARDS ONLY (5x5). The low-rank augmented Lagrangian of bm.py (unmodified copy) with the
objective as the only change, to measure evaluations to the FIRST VALID ROUNDING.

  mode l2    : + ||x||^2            (bm.py as it is: pulls toward mixtures of solutions)
  mode none  : no objective         (pure feasibility)
  mode conc  : - ||x||^2            (on the feasible set sum x = N, so N - ||x||^2 = sum x(1-x) is
                                     the non-integrality; this pushes toward 0/1 points)
  mode lin   : + s <c, x>, c uniform on [0, 1] (tie break between solutions)
  mode linconc : - ||x||^2 + s <c, x>
  mode w03, w3, w10 : + w ||x||^2 with w = 0.3, 3, 10 (is the pull toward the interior the useful part?)
  mode linl2 : + ||x||^2 + s <c, x>
  mode eq, eqconc : l2 or conc with the PAIR constraints equilibrated: the zero and nonnegativity
                    constraints are written on the rescaled rows v_p / sqrt(xbar_p), xbar the x of
                    the last outer step clipped to [0.2 N/P, 1] (a Jacobi scaling: an entry of Y
                    has natural size x_p x_q, about (N/P)^2, while a sum has size 1). The sums are
                    untouched. Multipliers are rescaled when xbar changes so that <lambda, h> is
                    unchanged.

Everything else is bm.solve's loop (rho from 10, times 1.3 when the violation did not halve, cap
1e5, 300 L-BFGS iterations per outer step), with the greedy rounding tested after every inner
iteration (as the Rust study's round_inner=1) and the run stopped at the first solution.

A mode may carry options: l2@i30 (30 L-BFGS iterations per outer step instead of 300), @u2 (penalty
factor 2 instead of 1.3), @r0100 (first penalty 100 instead of 10), @p0 (a hint: the placement that
solution 0 puts on the centre cell is pinned, by freezing its row at x = 1 and the rows of every
placement in conflict with it at x = 0, which is the model with those placements deleted).

Usage: bm_obj.py seed mode r start0 start1 out.jsonl [max_seconds_per_start] [max_evals] [s]
"""
import os, sys, json, time
os.environ.setdefault("VECLIB_MAXIMUM_THREADS", "1")
os.environ.setdefault("OMP_NUM_THREADS", "1")
os.environ.setdefault("OPENBLAS_NUM_THREADS", "1")
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import common  # noqa: F401,E402
import numpy as np  # noqa: E402
from scipy.optimize import minimize  # noqa: E402
import scipy.sparse as sp  # noqa: E402
from board import Board  # noqa: E402
import bm  # noqa: E402

seed = int(sys.argv[1]); label = sys.argv[2]; mode = label.split("@")[0]
OPT = dict(i=300.0, u=1.3, r0=0.0, p=-1.0)          # label "l2@i30@u1.3@r0100": inner cap, penalty factor, first penalty
for tok in label.split("@")[1:]:
    key = "r0" if tok.startswith("r0") else tok[0]
    OPT[key] = float(tok[len(key):])
_ = sys.argv[2]; r = int(sys.argv[3]); s0, s1 = int(sys.argv[4]), int(sys.argv[5])
out = sys.argv[6]
tmax = float(sys.argv[7]) if len(sys.argv) > 7 else 270.0
emax = int(sys.argv[8]) if len(sys.argv) > 8 else 30000
scale = float(sys.argv[9]) if len(sys.argv) > 9 else 1.0
RHO0 = float(sys.argv[10]) if len(sys.argv) > 10 else (OPT["r0"] or 10.0)
W_OF = dict(l2=1.0, none=0.0, conc=-1.0, lin=0.0, linconc=-1.0, eq=1.0, eqconc=-1.0, w3=3.0, w10=10.0, w03=0.3, linl2=1.0)

B = Board(5, 3, 4, seed); S = bm.Structure(B)
sol = np.zeros(B.P, bool)
for s in B.sol_idx:
    sol[s] = True


FROZEN = None; PIN = None; SOLS_AFTER_PIN = len(B.sol_idx)
if OPT["p"] >= 0:
    centre = B.N // 2
    PIN = int(B.sol_idx[int(OPT["p"])][centre])
    FROZEN = np.zeros(B.P, bool)
    FROZEN[S.cj[S.ci == PIN]] = True; FROZEN[S.ci[S.cj == PIN]] = True
    SOLS_AFTER_PIN = int(sum(int(sj[centre]) == PIN for sj in B.sol_idx))


class ObjAL(bm.SphereAL):
    def fg(self, w):
        f, g = super().fg(w)
        if FROZEN is not None:
            G = g.reshape(self.S.P, self.r); G[FROZEN] = 0.0; G[PIN] = 0.0
        return f, g

    w = 1.0
    c = None

    def fV(self, V, grad=True):
        f, g = super().fV(V)
        x = V[:, 0]
        f += (self.w - 1.0) * float(x @ x)
        g[:, 0] += (self.w - 1.0) * 2.0 * x
        if self.c is not None:
            f += float(self.c @ x); g[:, 0] += self.c
        return f, g


class EqAL(ObjAL):
    """Pair constraints on Vs = diag(s) V, s frozen between outer steps."""
    s = None

    def set_scale(self, x):
        new = 1.0 / np.sqrt(np.clip(x, 0.2 * self.S.N / self.S.P, 1.0))
        if self.s is not None:
            ratio = self.s / new
            self.lz *= ratio[self.S.ci] * ratio[self.S.cj]
            Dm = sp.diags(ratio)
            self.M2 = (Dm @ self.M2 @ Dm).tocsr()
            self._mu = None
        self.s = new

    def fV(self, V, grad=True):
        S, rho = self.S, self.rho
        x = V[:, 0]
        g = np.zeros_like(V)
        f = self.w * float(x @ x); g[:, 0] += self.w * 2 * x
        if self.c is not None:
            f += float(self.c @ x); g[:, 0] += self.c
        for Cm, lam in ((S.Ccell, self.lc), (S.Ctile, self.lt)):
            H = self.sums_residual(Cm, V)
            f += (lam * H).sum() + 0.5 * rho * (H * H).sum()
            g += Cm.T @ (lam + rho * H)
        Vs = V * self.s[:, None]
        hz = (Vs[S.ci] * Vs[S.cj]).sum(1)
        wz = np.maximum(0.0, -rho * hz)
        f += self.lz @ hz + 0.5 * rho * hz @ hz - (wz @ wz) / (2 * rho)
        gs = S.conflict_matrix(self.lz + rho * hz + wz) @ Vs
        if self._mu is None:
            mus = [self.M2[rows].toarray() for rows in S.blocks]
            self._mu = (mus, sum(float((m * m).sum()) for m in mus))
        mus, mu2 = self._mu
        f -= 0.5 * mu2 / (2 * rho)
        for rows, mu in zip(S.blocks, mus):
            G = Vs[rows] @ Vs.T
            W = np.maximum(0.0, mu - rho * G)
            f += 0.5 * (W * W).sum() / (2 * rho)
            gs[rows] -= W @ Vs
        g += gs * self.s[:, None]
        return f, g

    def update_multipliers(self, V):
        S, rho = self.S, self.rho
        self.lc += rho * self.sums_residual(S.Ccell, V); self.lt += rho * self.sums_residual(S.Ctile, V)
        Vs = V * self.s[:, None]
        self.lz += rho * (Vs[S.ci] * Vs[S.cj]).sum(1)
        parts = []
        for rows in S.blocks:
            parts.append(sp.csr_matrix(np.maximum(0.0, self.M2[rows].toarray() - rho * (Vs[rows] @ Vs.T))))
        M2 = sp.vstack(parts).tocsr()
        M2 = (M2 - M2.multiply(S.conflict_matrix(np.ones(S.K)))).tocsr()
        M2.setdiag(0.0); M2.eliminate_zeros()
        self.M2 = M2
        self._mu = None
        self.set_scale(V[:, 0])


class Solved(Exception):
    pass


for start in range(s0, s1):
    rng = np.random.default_rng(1000 * r + start)          # run_unit.py's seeds
    U0 = bm.init_U(B, r, rng)
    if FROZEN is not None:
        U0[FROZEN] = 0.0; U0[FROZEN, 0] = -1.0; U0[PIN] = 0.0; U0[PIN, 0] = 1.0
    al = (EqAL if mode.startswith("eq") else ObjAL)(S, r, rho=RHO0, vec=True)
    al.w = W_OF[mode]
    if mode.startswith("eq"):
        al.set_scale(bm.V_of_U(U0)[:, 0])
    al.c = scale * np.random.default_rng(777 + start).uniform(0, 1, B.P) if mode in ("lin", "linconc", "linl2") else None
    w = U0.ravel().copy()
    t0 = time.time(); prev_v = np.inf
    state = dict(solved=False)
    hist = []

    def cb(wk):
        Wt = wk.reshape(B.P, r)
        x = 0.5 * (1.0 + Wt[:, 0] / np.linalg.norm(Wt, axis=1))
        js = bm.greedy_round(B, x)
        if B.mismatches(js.tolist()) == 0:
            state.update(solved=True, evals=al.nevals, t=round(time.time() - t0, 2), w=wk.copy())
            raise Solved
        if al.nevals > emax or time.time() - t0 > tmax:
            state.update(w=wk.copy())
            raise Solved

    status = "max_outer"
    for it in range(100000):
        try:
            res = minimize(al.fg, w, jac=True, method="L-BFGS-B", callback=cb,
                           options=dict(maxiter=int(OPT["i"]), maxcor=10, gtol=1e-9, ftol=1e-14))
            w = res.x
        except Solved:
            w = state["w"]; status = "solved" if state["solved"] else "limit"
        Wt = w.reshape(B.P, r); U = Wt / np.linalg.norm(Wt, axis=1, keepdims=True)
        w = U.ravel().copy(); V = bm.V_of_U(U)
        viol = bm.max_violation(S, V)["max"]; x = V[:, 0]
        hist.append(dict(outer=it, rho=round(al.rho, 2), viol=float(f"{viol:.3e}"), obj=round(float(x @ x), 4),
                         mass_off=round(float(x[~sol].sum() / B.N), 4), evals=al.nevals, t=round(time.time() - t0, 1)))
        if status != "max_outer":
            break
        al.update_multipliers(V)
        if viol > 0.5 * prev_v:
            al.rho = min(1e5, al.rho * OPT["u"])
        prev_v = viol
    sv = np.linalg.svd(np.vstack([np.eye(1, r), V]), compute_uv=False)
    rec = dict(board=[5, 3, 4, seed], n_sols=len(B.sols), sols_after_pin=SOLS_AFTER_PIN, frozen=int(FROZEN.sum()) if FROZEN is not None else 0, mode=label, scale=scale, rho0=RHO0, r=r, start=start, status=status,
               solved=state["solved"], evals_solution=state.get("evals"), t_solution=state.get("t"),
               evals=al.nevals, seconds=round(time.time() - t0, 1), outer=len(hist), final_rho=al.rho,
               viol_end=hist[-1]["viol"], mass_off_end=hist[-1]["mass_off"], obj_end=hist[-1]["obj"],
               rank_1e2=int((sv ** 2 > 1e-2 * sv[0] ** 2).sum()), loadavg=os.getloadavg()[0], hist=hist)
    with open(out, "a") as f:
        f.write(json.dumps(rec) + "\n")
    print(seed, label, r, start, status, rec["evals_solution"], rec["evals"], rec["seconds"], rec["viol_end"],
          rec["mass_off_end"], rec["rank_1e2"], flush=True)
