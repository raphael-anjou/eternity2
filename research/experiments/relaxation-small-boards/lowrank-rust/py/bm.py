"""TOY BOARDS ONLY. Burer-Monteiro (low-rank, matrix-free) solver for the SAME relaxation as
../geometry-sdp-lift/lift.py (obj="l2"), so its x is comparable entry by entry.

Lift:  Z = [[1, x^T], [x, Y]] = V_full V_full^T,  V_full = [v_0; V], v_0 = e_1 (gauge fixed: any
factor can be rotated so that v_0 lies along e_1, and Z[0,0] = 1 makes it a unit vector), V of
shape P x r. Then x_p = V[p, 0], Y[p, q] = <v_p, v_q>, and Z is PSD of rank <= r by construction.

lift.py's constraints, written on V:
  E1  sum_{p in cell c} x_p = 1                                   N rows
  E2  sum_{p of tile t} x_p = 1                                   T rows
  E3  ||v_p||^2 = x_p                                             P rows   (Y[p,p] = x[p])
  E4  <v_p, S_c> = x_p,  S_c = sum_{q in c} v_q,  c != cell(p)    row sums over cells
  E5  <v_p, U_t> = x_p,  U_t = sum_{q of t} v_q,  t != tile(p)    row sums over tiles
  E6  <v_p, v_q> = 0 for every conflict pair (same cell, same tile, or adjacent cells with
      mismatched facing sides): an index list of K pairs built from structure
  I1  x_p >= 0
  I2  <v_p, v_q> >= 0 for every free pair (p < q, not a conflict)
Objective: minimise ||x||^2 (lift.py's "l2").

Two exact simplifications, both leaving the feasible set unchanged:
 (a) E3 is a sphere: ||v_p||^2 = <v_0, v_p> iff ||v_p - e_1/2|| = 1/2. So v_p = (e_1 + u_p)/2
     with ||u_p|| = 1, i.e. x_p = (1 + u_p[0])/2 (the 0/1 to +-1 change of variable). E3 and I1
     then hold by construction, and the search space is a product of P unit spheres in R^r.
 (b) E4 and E5 are implied by E1, E3 and E6: ||S_c - v_0||^2 = sum_{p,q in c} <v_p, v_q>
     - 2 sum_{p in c} x_p + 1 = sum_{p in c} x_p - 2 + 1 = 0 (same-cell pairs are conflicts, so
     only the diagonal survives), hence S_c = v_0 and <v_p, S_c> = <v_p, v_0> = x_p. Same for
     tiles with same-tile conflicts. (In lift.py the row sums run over free q only, the conflict
     entries being the constant 0; at a feasible point that is the same sum.) So E4, E5 are not
     penalised, but max_violation() still evaluates them, and every other constraint, explicitly.
What is penalised: E1, E2, E6 and I2.

Method: augmented Lagrangian (Powell-Hestenes-Rockafellar; inequalities by the shifted
max(0, mu - rho g) form) in the sphere coordinates, inner solve by L-BFGS (scipy L-BFGS-B, no
bounds) on W with u_p = w_p / ||w_p||. I2 is evaluated block by block over ALL pairs, then the K
conflict pairs (index list) are subtracted back out, so no P x P mask or matrix is ever formed;
the diagonal contributes nothing since <v_p, v_p> = x_p >= 0 and its multiplier is 0.

Cost per function+gradient evaluation (P placements, r rank, N cells, T tiles, K conflict pairs,
A active nonnegativity multipliers, B row-block size):
  flops  ~ 4 P^2 r + 3 P^2   (I2: Gram block V_B V^T, the max, and W_B V)
           + 4 K r           (E6 and the conflict correction of I2)
           + O(P r + A)
  memory ~ 8 [ (2 m + 4) P r (V, U, gradient, m = 10 L-BFGS pairs)
               + 4 K (pair indices, multipliers, CSR values) + 2 A (sparse I2 multipliers)
               + 3 B P (one Gram block, its multiplier block, W block) ] bytes
The only P^2 work is the entrywise nonnegativity I2 of the full relaxation; nothing of size
P x P is stored (B P is chosen at 16 MB).
"""
import time

import numpy as np
import scipy.sparse as sp
from scipy.optimize import minimize


class Structure:
    """Index lists for one board.Board: the conflict pairs, cell and tile incidence, row blocks."""

    def __init__(self, B, block_bytes=16 * 2**20):
        self.B = B
        P, N, T = B.P, B.N, B.T
        self.P, self.N, self.T = P, N, T
        c, t = B.cellOf, B.tileOf
        cells = [np.where(c == x)[0] for x in range(N)]
        tiles = [np.where(t == k)[0] for k in range(T)]
        keys = []
        for grp in cells + tiles:
            i, j = np.triu_indices(len(grp), 1)
            keys.append(np.minimum(grp[i], grp[j]) * P + np.maximum(grp[i], grp[j]))
        for x in range(N):
            for d in (1, 2):                       # each adjacency once (east, south)
                y = B.nb[x, d]
                if y < 0:
                    continue
                a, b = cells[x], cells[y]
                mis = B.cols[a, d][:, None] != B.cols[b, (d + 2) % 4][None, :]
                ia, ib = np.nonzero(mis)
                pa, pb = a[ia], b[ib]
                keys.append(np.minimum(pa, pb) * P + np.maximum(pa, pb))
        keys = np.unique(np.concatenate(keys))
        self.ci, self.cj = keys // P, keys % P
        self.K = len(keys)
        self.F = P * (P - 1) // 2 - self.K
        # CSR pattern of the symmetric conflict matrix, and a map from each CSR slot to its pair
        rows = np.concatenate([self.ci, self.cj]); cols = np.concatenate([self.cj, self.ci])
        M = sp.csr_matrix((np.arange(1, 2 * self.K + 1, dtype=float), (rows, cols)), shape=(P, P))
        M.sort_indices()
        self.conf_csr = M
        self.conf_slot = (M.data.astype(np.int64) - 1) % self.K
        self.Ccell = sp.csr_matrix((np.ones(P), (c, np.arange(P))), shape=(N, P))
        self.Ctile = sp.csr_matrix((np.ones(P), (t, np.arange(P))), shape=(T, P))
        self.bs = max(16, int(block_bytes // (8 * P)))
        self.blocks = [np.arange(s, min(P, s + self.bs)) for s in range(0, P, self.bs)]

    def conflict_matrix(self, vals):
        """Symmetric CSR with vals[k] at both slots of pair k."""
        M = self.conf_csr.copy(); M.data = vals[self.conf_slot]
        return M

    def free_mask(self, rows):
        """Boolean (len(rows), P) from structure; used only by the checks (check.py)."""
        B = self.B
        c, t = B.cellOf, B.tileOf
        cr, tr = c[rows], t[rows]
        bad = (cr[:, None] == c[None, :]) | (tr[:, None] == t[None, :])
        for d in range(4):
            nbc = B.nb[cr, d]
            bad |= (nbc[:, None] == c[None, :]) & (B.cols[rows, d][:, None] != B.cols[:, (d + 2) % 4][None, :])
        return ~bad


def V_of_U(U):
    V = 0.5 * U
    V[:, 0] += 0.5
    return V


def max_violation(S, V):
    """Largest violation of every constraint of lift.py's relaxation at V (v_0 = e_1):
    equalities |h|, inequalities max(0, -g); I2 over every free pair, block by block."""
    x = V[:, 0]
    Sc = S.Ccell @ V; Ut = S.Ctile @ V
    own_c = np.ones((S.P, S.N), bool); own_c[np.arange(S.P), S.B.cellOf] = False
    own_t = np.ones((S.P, S.T), bool); own_t[np.arange(S.P), S.B.tileOf] = False
    hz = (V[S.ci] * V[S.cj]).sum(1)
    v = dict(cell_sum=np.abs(S.Ccell @ x - 1).max(), tile_sum=np.abs(S.Ctile @ x - 1).max(),
             diag=np.abs((V * V).sum(1) - x).max(),
             row_cell=np.abs((V @ Sc.T - x[:, None])[own_c]).max(),
             row_tile=np.abs((V @ Ut.T - x[:, None])[own_t]).max(),
             zero=np.abs(hz).max(), x_nonneg=max(0.0, -x.min()))
    # lift.py's row sums run over free q only; with conflicts not exactly 0 the two differ by
    # the conflict entries, which "zero" bounds (at most n_c of them per row).
    m = 0.0
    big = S.conflict_matrix(np.ones(S.K)).tocsr()
    for rows in S.blocks:
        G = V[rows] @ V.T
        G[np.arange(len(rows)), rows] = np.inf                 # diagonal
        sub = big[rows].tocoo()
        G[sub.row, sub.col] = np.inf                           # conflict pairs
        m = max(m, -G.min())
    v["y_nonneg"] = max(0.0, m)
    v = {k: float(a) for k, a in v.items()}
    v["max"] = max(v.values())
    return v


class SphereAL:
    """vec=True penalises the cell and tile sums in vector form, S_c = v_0 and U_t = v_0 (N r and
    T r residuals; their first coordinates are E1, E2), which by (b) holds at every feasible point;
    vec=False penalises the scalar E1, E2 only."""

    def __init__(self, S, r, rho=1.0, vec=False):
        self.S, self.r, self.rho, self.vec = S, r, rho, vec
        self.lc = np.zeros((S.N, r)); self.lt = np.zeros((S.T, r)); self.lz = np.zeros(S.K)
        self.E = np.zeros((1, r)); self.E[0, 0] = 1.0
        # Toy-size speed-up only: when P^2 doubles fit in 100 MB, the dense multiplier blocks and
        # sum(mu^2) are cached between multiplier updates instead of being re-expanded from the
        # sparse store at every evaluation. Values are identical; the memory formula in the module
        # docstring is for the uncached path.
        self.cache_ok = S.P * S.P * 8 <= 100 * 2**20
        self._mu = None
        self.M2 = sp.csr_matrix((S.P, S.P))
        self.nevals = 0

    def fV(self, V, grad=True):
        """AL value and gradient with respect to V."""
        S, rho = self.S, self.rho
        x = V[:, 0]
        g = np.zeros_like(V)
        f = float(x @ x); g[:, 0] += 2 * x
        for Cm, lam in ((S.Ccell, self.lc), (S.Ctile, self.lt)):
            H = self.sums_residual(Cm, V)
            f += (lam * H).sum() + 0.5 * rho * (H * H).sum()
            g += Cm.T @ (lam + rho * H)
        hz = (V[S.ci] * V[S.cj]).sum(1)
        # E6, plus the correction removing the conflict pairs from the all-pairs I2 sum below
        wz = np.maximum(0.0, -rho * hz)
        f += self.lz @ hz + 0.5 * rho * hz @ hz - (wz @ wz) / (2 * rho)
        g += S.conflict_matrix(self.lz + rho * hz + wz) @ V
        if self._mu is None:
            mus = [self.M2[rows].toarray() for rows in S.blocks]
            mu2 = sum(float((m * m).sum()) for m in mus)
            if self.cache_ok:
                self._mu = (mus, mu2)
        else:
            mus, mu2 = self._mu
        f -= 0.5 * mu2 / (2 * rho)
        for rows, mu in zip(S.blocks, mus):
            G = V[rows] @ V.T
            W = np.maximum(0.0, mu - rho * G)
            f += 0.5 * (W * W).sum() / (2 * rho)
            g[rows] -= W @ V
        return f, g

    def sums_residual(self, Cm, V):
        if self.vec:
            return Cm @ V - self.E
        H = np.zeros((Cm.shape[0], self.r)); H[:, 0] = Cm @ V[:, 0] - 1
        return H

    def fg(self, w):
        S, r = self.S, self.r
        self.nevals += 1
        Wt = w.reshape(S.P, r)
        nrm = np.linalg.norm(Wt, axis=1, keepdims=True)
        U = Wt / nrm
        f, gV = self.fV(V_of_U(U))
        gU = 0.5 * gV
        gW = (gU - (gU * U).sum(1, keepdims=True) * U) / nrm
        return f, gW.ravel()

    def update_multipliers(self, V):
        S, rho = self.S, self.rho
        x = V[:, 0]
        self.lc += rho * self.sums_residual(S.Ccell, V); self.lt += rho * self.sums_residual(S.Ctile, V)
        hz = (V[S.ci] * V[S.cj]).sum(1)
        self.lz += rho * hz
        parts = []
        for rows in S.blocks:
            G = V[rows] @ V.T
            mu = self.M2[rows].toarray()
            parts.append(sp.csr_matrix(np.maximum(0.0, mu - rho * G)))
        M2 = sp.vstack(parts).tocsr()
        # conflict pairs carry no I2 multiplier (their mu was 0, so the new one is max(0, -rho z))
        M2 = (M2 - M2.multiply(S.conflict_matrix(np.ones(S.K)))).tocsr()
        M2.eliminate_zeros()
        self.M2 = M2
        self._mu = None


def init_U(B, r, rng):
    """Random start on the spheres: x_p = X0_p (1 + 0.2 xi) (X0 = Sinkhorn centre), i.e.
    u_p[0] = 2 x_p - 1, the rest a uniformly random direction of length sqrt(1 - u_p[0]^2)."""
    P = B.P
    x = np.clip(B.X0 * (1 + 0.2 * rng.standard_normal(P)), 1e-3, 1 - 1e-3)
    U = np.zeros((P, r))
    U[:, 0] = 2 * x - 1
    if r > 1:
        Z = rng.standard_normal((P, r - 1))
        Z /= np.linalg.norm(Z, axis=1, keepdims=True)
        U[:, 1:] = np.sqrt(1 - U[:, :1] ** 2) * Z
    return U


def U_of_solution(B, r, js):
    U = -np.eye(1, r).repeat(B.P, 0)
    U[js, 0] = 1.0
    return U


def solve(S, r, U0, tol=1e-5, max_outer=200, inner_iters=300, time_limit=1500, rho0=10.0,
          rho_max=1e5, rho_up=1.3, verbose=False, vec=True):
    """Defaults fixed from the tuning runs in raw/tune*/ (5x5 seeds 0 and 1, before stage 1):
    vector-form sums, rho from 10, times 1.3 whenever the violation did not halve, capped at 1e5."""
    al = SphereAL(S, r, rho=rho0, vec=vec)
    w = U0.ravel().copy()
    t0 = time.time()
    prev_v = np.inf; prev_obj = np.inf
    hist = []
    status = "max_outer"
    for it in range(max_outer):
        res = minimize(al.fg, w, jac=True, method="L-BFGS-B",
                       options=dict(maxiter=inner_iters, maxcor=10, gtol=1e-9, ftol=1e-14))
        w = res.x
        Wt = w.reshape(S.P, r); U = Wt / np.linalg.norm(Wt, axis=1, keepdims=True)
        w = U.ravel().copy()                     # renormalise (keeps L-BFGS scale sane)
        V = V_of_U(U)
        viol = max_violation(S, V)["max"]
        x = V[:, 0]; obj = float(x @ x)
        hist.append(dict(outer=it, rho=al.rho, viol=viol, obj=obj, inner=int(res.nit),
                         t=round(time.time() - t0, 2)))
        if verbose:
            print(hist[-1], flush=True)
        if viol <= tol and abs(obj - prev_obj) <= 1e-6 * max(1.0, obj):
            status = "converged"
            break
        prev_obj = obj
        al.update_multipliers(V)
        if viol > 0.5 * prev_v:
            al.rho = min(rho_max, al.rho * rho_up)
        prev_v = viol
        if time.time() - t0 > time_limit:
            status = "time_limit"
            break
    return V, dict(status=status, outer=len(hist), evals=al.nevals, seconds=round(time.time() - t0, 2),
                   final_rho=al.rho, hist=hist, viol=max_violation(S, V),
                   active_nonneg_multipliers=int(al.M2.nnz // 2))


def greedy_round(B, x):
    """Largest entry first: walk the placements in decreasing x (ties by index); take a placement
    when its cell is still empty and its tile still unused. Every (cell, tile) pair of the same rim
    class has a placement in the model, so this always fills every cell."""
    order = np.lexsort((np.arange(B.P), -x))
    js = np.full(B.N, -1); used = np.zeros(B.T, bool)
    for p in order:
        c, t = B.cellOf[p], B.tileOf[p]
        if js[c] < 0 and not used[t]:
            js[c] = p; used[t] = True
    assert (js >= 0).all()
    return js
