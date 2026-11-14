"""TOY (5x5 only). The structured sweep against bm.py's, same relaxation, same schedule.

StructuredAL: augmented Lagrangian on the sphere coordinates for
  minimise ||x||^2  s.t.  L V = b e_1^T  (cells, tiles, edge-colour; multipliers Lam, m x r)
                          <v_p, v_q> >= 0 for EVERY pair p != q (multipliers mu, dense at toy size)
No conflict-pair list, no zero multipliers: the zeros follow from the equalities and the
nonnegativity (memo, section 2). Reported violations are bm.max_violation's, i.e. the ORIGINAL
constraint list (explicit zeros included).

Modes:
  check                      gradient against central differences
  time r                     median ms per evaluation, bm.SphereAL against StructuredAL
  run which r start tlim     which = orig | struct ; early stop at the first rounding that solves;
                             one JSON line per outer step and a final line. For struct, also the
                             working-set statistics of the memo's lazy scheme.
Usage: structured.py mode ... [board seed last, default 0]
"""
import os, sys, json, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
from scipy.optimize import minimize
from structure import Board, Structure, linear_system
from bm import SphereAL, V_of_U, init_U, max_violation, greedy_round
from measure import round_board

TAUS = (0.0, 1e-3, 3e-3)


class StructuredAL:
    def __init__(self, S, r, rho=1.0):
        self.S, self.r, self.rho = S, r, rho
        self.L, self.b = linear_system(S.B)
        self.LT = self.L.T.tocsr()
        self.Lam = np.zeros((self.L.shape[0], r))
        self.mu = np.zeros((S.P, S.P))
        self.mu2 = 0.0
        self.nevals = 0
        self.track = None            # lazy-scheme bookkeeping, set by begin_outer()
        self.active_share = []

    def fV(self, V):
        rho = self.rho
        x = V[:, 0]
        g = np.zeros_like(V)
        f = float(x @ x); g[:, 0] += 2 * x
        H = self.L @ V; H[:, 0] -= self.b
        f += (self.Lam * H).sum() + 0.5 * rho * (H * H).sum()
        g += self.LT @ (self.Lam + rho * H)
        G = V @ V.T
        W = np.maximum(0.0, self.mu - rho * G)
        np.fill_diagonal(W, 0.0)
        f += 0.5 * ((W * W).sum() - self.mu2) / (2 * rho)
        g -= W @ V
        if self.track is not None:
            act = W > 0
            self.active_share.append(float(act.sum()) / (self.S.P * (self.S.P - 1)))
            self.track["ever"] |= act
        return f, g

    def fg(self, w):
        S, r = self.S, self.r
        self.nevals += 1
        Wt = w.reshape(S.P, r)
        nrm = np.linalg.norm(Wt, axis=1, keepdims=True)
        U = Wt / nrm
        f, gV = self.fV(V_of_U(U))
        gU = 0.5 * gV
        return f, ((gU - (gU * U).sum(1, keepdims=True) * U) / nrm).ravel()

    def update_multipliers(self, V):
        H = self.L @ V; H[:, 0] -= self.b
        self.Lam += self.rho * H
        mu = np.maximum(0.0, self.mu - self.rho * (V @ V.T))
        np.fill_diagonal(mu, 0.0)
        self.mu = mu; self.mu2 = float((mu * mu).sum())

    def begin_outer(self, V):
        """Working sets a lazy scheme would build from one full scan at the start of the inner
        solve: pairs with mu > 0 or <v_p, v_q> < tau."""
        G = V @ V.T
        P = self.S.P
        off = ~np.eye(P, dtype=bool)
        self.track = dict(ws={t: ((self.mu > 0) | (G < t)) & off for t in TAUS},
                          ever=np.zeros((P, P), bool), V0=V.copy())
        self.active_share = []

    def end_outer(self, V):
        tr = self.track; self.track = None
        P = self.S.P; npairs = P * (P - 1)
        ever = tr["ever"]
        d = np.linalg.norm(V - tr["V0"], axis=1)
        out = dict(active_share_mean=float(np.mean(self.active_share)) if self.active_share else None,
                   ever_active_share=float(ever.sum()) / npairs,
                   row_move_max=float(d.max()), row_move_median=float(np.median(d)),
                   row_norm_median=float(np.median(np.linalg.norm(V, axis=1))))
        for t in TAUS:
            ws = tr["ws"][t]
            out[f"ws_share_tau{t:g}"] = float(ws.sum()) / npairs
            out[f"missed_share_of_ever_tau{t:g}"] = float((ever & ~ws).sum()) / max(1, int(ever.sum()))
        return out


def gradcheck(B, S):
    rng = np.random.default_rng(0)
    r = 4
    al = StructuredAL(S, r, rho=3.0)
    al.Lam = rng.standard_normal(al.Lam.shape)
    M = np.triu(np.abs(rng.standard_normal((B.P, B.P))) * (rng.random((B.P, B.P)) < 0.05), 1)
    al.mu = M + M.T; al.mu2 = float((al.mu ** 2).sum())
    w = rng.standard_normal(B.P * r)
    f0, g0 = al.fg(w)
    errs = []
    for k in rng.choice(B.P * r, 60, replace=False):
        e = np.zeros(B.P * r); e[k] = 1e-6
        fd = (al.fg(w + e)[0] - al.fg(w - e)[0]) / 2e-6
        errs.append(abs(fd - g0[k]) / max(1.0, abs(fd)))
    return float(max(errs))


def run(B, S, which, r, start, tlim, rho0=10.0, rho_max=1e5, rho_up=1.3, inner_iters=300, max_outer=400):
    """bm.solve's schedule, with the early stop at every outer step."""
    al = SphereAL(S, r, rho=rho0, vec=True) if which == "orig" else StructuredAL(S, r, rho=rho0)
    w = init_U(B, r, np.random.default_rng(1000 * r + start)).ravel().copy()
    conf = np.zeros((B.P, B.P), bool); conf[S.ci, S.cj] = True; conf |= conf.T
    sol_used = np.zeros(B.P, bool)
    for sidx in B.sol_idx:
        sol_used[sidx] = True
    t0 = time.time(); prev_v = np.inf
    status = "max_outer"
    for it in range(max_outer):
        if which == "struct":
            Wt = w.reshape(B.P, r); al.begin_outer(V_of_U(Wt / np.linalg.norm(Wt, axis=1, keepdims=True)))
        res = minimize(al.fg, w, jac=True, method="L-BFGS-B",
                       options=dict(maxiter=inner_iters, maxcor=10, gtol=1e-9, ftol=1e-14))
        Wt = res.x.reshape(B.P, r); U = Wt / np.linalg.norm(Wt, axis=1, keepdims=True)
        w = U.ravel().copy(); V = V_of_U(U)
        t_solve = time.time() - t0
        viol = max_violation(S, V)
        x = V[:, 0]
        rec = dict(which=which, r=r, start=start, outer=it, rho=al.rho, evals=al.nevals, t=round(t_solve, 2),
                   viol_max=viol["max"], viol_zero=viol["zero"], viol_y_nonneg=viol["y_nonneg"],
                   viol_cell=viol["cell_sum"], obj=float(x @ x))
        if which == "struct":
            rec.update(al.end_outer(V))
            rec["mu_nonzero_share_conflict_pairs"] = float((al.mu[conf] > 0).mean())
            rec["mu_nonzero_share_free_pairs"] = float((al.mu[~conf & ~np.eye(B.P, dtype=bool)] > 0).mean())
        js = greedy_round(B, x)
        solved = B.mismatches(js.tolist()) == 0 or bool(round_board(B, np.clip(x, 0, None)).get("is_solution"))
        rec["rounds_to_solution"] = bool(solved)
        rec["greedy_mismatches"] = int(B.mismatches(js.tolist()))
        rec["mass_off_solution_placements_per_cell"] = float(np.clip(x, 0, None)[~sol_used].sum() / B.N)
        sv = np.linalg.svd(np.vstack([np.eye(1, r), V]), compute_uv=False) ** 2
        rec["rank_1e3"] = int((sv > 1e-3 * sv[0]).sum())
        print(json.dumps(rec), flush=True)
        if solved:
            status = "solved_early"; break
        al.update_multipliers(V)
        if viol["max"] > 0.5 * prev_v:
            al.rho = min(rho_max, al.rho * rho_up)
        prev_v = viol["max"]
        if time.time() - t0 > tlim:
            status = "time_limit"; break
    print(json.dumps(dict(final=True, which=which, r=r, start=start, status=status, outer=it + 1,
                          evals=al.nevals, seconds=round(time.time() - t0, 2))), flush=True)


if __name__ == "__main__":
    mode = sys.argv[1]
    if mode == "check":
        B = Board(5, 3, 4, 0); S = Structure(B)
        print(json.dumps(dict(grad_max_rel_err=gradcheck(B, S))))
    elif mode == "time":
        r = int(sys.argv[2]); B = Board(5, 3, 4, 0); S = Structure(B)
        U = init_U(B, r, np.random.default_rng(0)); w = U.ravel()
        out = dict(r=r, P=B.P)
        for name, al in (("orig", SphereAL(S, r, rho=10.0, vec=True)), ("struct", StructuredAL(S, r, rho=10.0))):
            al.update_multipliers(V_of_U(U))
            ts = []
            for _ in range(200):
                a = time.perf_counter(); al.fg(w); ts.append(time.perf_counter() - a)
            out[name + "_ms_median"] = round(1000 * float(np.median(ts)), 3)
            out[name + "_ms_min"] = round(1000 * float(np.min(ts)), 3)
        print(json.dumps(out))
    elif mode == "run":
        which, r, start, tlim = sys.argv[2], int(sys.argv[3]), int(sys.argv[4]), float(sys.argv[5])
        seed = int(sys.argv[6]) if len(sys.argv) > 6 else 0
        B = Board(5, 3, 4, seed); S = Structure(B)
        run(B, S, which, r, start, tlim)
