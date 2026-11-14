"""TOY BOARDS ONLY. Experiment 5: the signed low-rank factor (the low-rank study's bm.py,
imported, not modified) with entrywise nonnegativity penalised ONLY on pairs of placements whose
cells are within lattice (Manhattan) distance d. d = 0: no nonnegativity at all. d = 99: all pairs
(the unmodified solver; `check` mode verifies value and gradient agree with bm.SphereAL).
Everything else is bm.py: sphere coordinates, vector-form sums, conflict zeros, augmented
Lagrangian, L-BFGS. Settings follow the Rust study's steps S1, S2, S5: r = 16, 20 inner
iterations per outer step, greedy rounding after every outer step, stop at the first rounding
with 0 mismatches. Same starts as run_unit.py (rng 1000 r + start, init_U). One start per process.
Usage: bm_local.py n cf ci seed r d start seconds out.jsonl [broken|check]
"""
import os, sys, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
import scipy.sparse as sp
from cp_common import load, is_solution
import bm

n, cf, ci, seed, r, d, st = map(int, sys.argv[1:8])
tlim = float(sys.argv[8]); out = sys.argv[9]
mode = sys.argv[10] if len(sys.argv) > 10 else ""
B, S, C = load(n, cf, ci, seed, mode == "broken")
P = B.P
rr, cc = np.divmod(B.cellOf, n)
dist = np.abs(rr[:, None] - rr[None, :]) + np.abs(cc[:, None] - cc[None, :])
MK = (dist <= d).astype(float) if d > 0 else np.zeros((P, P))      # includes conflicts and diagonal
ZM = MK[S.ci, S.cj]                                                 # conflict pairs inside the mask
free = np.ones((P, P), bool); free[S.ci, S.cj] = False; free[S.cj, S.ci] = False; free[np.arange(P), np.arange(P)] = False
masked_free_pairs = int((free & (MK > 0)).sum() // 2)


class LocalAL(bm.SphereAL):
    def fV(self, V, grad=True):
        S_, rho = self.S, self.rho
        x = V[:, 0]
        g = np.zeros_like(V)
        f = float(x @ x); g[:, 0] += 2 * x
        for Cm, lam in ((S_.Ccell, self.lc), (S_.Ctile, self.lt)):
            H = self.sums_residual(Cm, V)
            f += (lam * H).sum() + 0.5 * rho * (H * H).sum()
            g += Cm.T @ (lam + rho * H)
        hz = (V[S_.ci] * V[S_.cj]).sum(1)
        wz = np.maximum(0.0, -rho * hz) * ZM
        f += self.lz @ hz + 0.5 * rho * hz @ hz - (wz @ wz) / (2 * rho)
        g += S_.conflict_matrix(self.lz + rho * hz + wz) @ V
        if self._mu is None:
            mus = [self.M2[rows].toarray() for rows in S_.blocks]
            self._mu = (mus, sum(float((m * m).sum()) for m in mus))
        mus, mu2 = self._mu
        f -= 0.5 * mu2 / (2 * rho)
        for rows, mu in zip(S_.blocks, mus):
            G = V[rows] @ V.T
            W = np.maximum(0.0, mu - rho * G) * MK[rows]
            f += 0.5 * (W * W).sum() / (2 * rho)
            g[rows] -= W @ V
        return f, g

    def update_multipliers(self, V):
        S_, rho = self.S, self.rho
        self.lc += rho * self.sums_residual(S_.Ccell, V); self.lt += rho * self.sums_residual(S_.Ctile, V)
        self.lz += rho * (V[S_.ci] * V[S_.cj]).sum(1)
        parts = []
        for rows in S_.blocks:
            G = V[rows] @ V.T
            parts.append(sp.csr_matrix(np.maximum(0.0, self.M2[rows].toarray() - rho * G) * MK[rows]))
        M2 = sp.vstack(parts).tocsr()
        M2 = (M2 - M2.multiply(S_.conflict_matrix(np.ones(S_.K)))).tocsr()
        M2.eliminate_zeros()
        self.M2 = M2
        self._mu = None


if mode == "check":
    rng = np.random.default_rng(0)
    a = bm.SphereAL(S, r, rho=7.0, vec=True); b = LocalAL(S, r, rho=7.0, vec=True)
    V = bm.V_of_U(bm.init_U(B, r, rng))
    for al in (a, b):
        al.lz = np.random.default_rng(1).standard_normal(S.K) * 0.1
        al.update_multipliers(V); al.update_multipliers(V * 0.9)
    V2 = bm.V_of_U(bm.init_U(B, r, rng))
    fa, ga = a.fV(V2); fb, gb = b.fV(V2)
    print("d", d, "masked free pairs", masked_free_pairs, "of", S.F, "f", fa, fb, "max grad diff", float(np.abs(ga - gb).max()),
          "multipliers", a.M2.nnz, b.M2.nnz)
    sys.exit()

bm.SphereAL = LocalAL
U0 = bm.init_U(B, r, np.random.default_rng(1000 * r + st))
V, info = bm.solve(S, r, U0, time_limit=tlim, inner_iters=20, max_outer=100000, check_every=1,
                   stop_check=lambda x: is_solution(B, x)[0])
ok, js = is_solution(B, V[:, 0])
sol = np.zeros(P, bool)
for s in B.sol_idx:
    sol[s] = True
G = V @ V.T
rec = dict(method="bm_local", board=[n, cf, ci, seed], broken=mode == "broken", r=r, d=d, start=st,
           masked_free_pairs=masked_free_pairs, free_pairs=int(S.F), solved=bool(ok), status=info["status"],
           evals=info["evals"], outer=info["outer"], seconds=info["seconds"], viol=info["viol"],
           most_negative_free_entry=float(G[free].min()), negative_free_share=float((G[free] < -1e-6).mean()),
           mass_off_solutions_per_cell=float(np.clip(V[:, 0], 0, None)[~sol].sum() / B.N) if B.sol_idx else None,
           mismatches=int(B.mismatches(js.tolist())))
with open(out, "a") as f:
    f.write(json.dumps(rec) + "\n")
print(json.dumps({k: rec[k] for k in ("board", "d", "start", "solved", "evals", "seconds", "mismatches")}))
