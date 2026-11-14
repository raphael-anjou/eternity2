"""Difference map (Douglas-Rachford) for edge matching, in half-edge colour space.

Run: uv run --with numpy --with scipy python dm.py <mode> ...
Stdlib + numpy + scipy only. One core.

State x: real array (n, n, 4 sides, C colours), sides ordered N, E, S, W.
Sets:
  A  layouts: every cell holds one oriented piece, every piece used once
     (exact projection: one linear assignment, Hungarian);
  R  rows:    every row is a horizontally matched sequence of oriented pieces,
     repeats allowed (exact projection: Viterbi per row, O(n*T));
  K  columns: same for columns;
  B  seams:   facing half-edges equal, rim grey (linear subspace, exact
     projection: average the two sides).
Two-set map: A against B. Multi-set map (divide and concur): replicas for
A, R, K, concur = average replicas then project on B.
"""
import sys, time, random, math
import numpy as np
from scipy.optimize import linear_sum_assignment

N_, E_, S_, W_ = 0, 1, 2, 3


def gen(n, b, c, rng):
    """Planted n x n puzzle. Colours: 0 grey, 1..b ring seams, b+1..b+c the rest."""
    for _try in range(20000):
        h = [[0] * (n - 1) for _ in range(n)]  # h[r][j]: seam between (r,j),(r,j+1)
        v = [[0] * n for _ in range(n - 1)]    # v[r][j]: seam between (r,j),(r+1,j)
        for r in range(n):
            for j in range(n - 1):
                ring = r in (0, n - 1)
                h[r][j] = rng.randint(1, b) if ring else rng.randint(b + 1, b + c)
        for r in range(n - 1):
            for j in range(n):
                ring = j in (0, n - 1)
                v[r][j] = rng.randint(1, b) if ring else rng.randint(b + 1, b + c)
        sol = []
        for r in range(n):
            for j in range(n):
                t = (v[r - 1][j] if r > 0 else 0, h[r][j] if j < n - 1 else 0,
                     v[r][j] if r < n - 1 else 0, h[r][j - 1] if j > 0 else 0)
                sol.append(t)
        canon = set()
        ok = True
        for t in sol:
            rots = [t[k:] + t[:k] for k in range(4)]
            if len(set(rots)) < 4 and t.count(0) == 0:
                ok = False
                break
            m = min(rots)
            if m in canon:
                ok = False
                break
            canon.add(m)
        if ok:
            pieces = sol[:]
            rng.shuffle(pieces)
            pieces = [p[k:] + p[:k] for p in pieces for k in [rng.randrange(4)]]
            return pieces
    raise SystemExit(f"no instance with distinct pieces for n={n} b={b} c={c}")


def rim_sides(n, r, j):
    return (r == 0, j == n - 1, r == n - 1, j == 0)


def dfs_count(n, pieces, cap=300_000):
    """Row-scan backtracking. Returns (nodes, solutions, capped). A node is a
    consistent placement."""
    rots = [[p[k:] + p[:k] for k in range(4)] for p in pieces]
    P = len(pieces)
    used = [False] * P
    board = [None] * (n * n)
    nodes = 0
    sols = 0
    capped = False

    def rec(pos):
        nonlocal nodes, sols, capped
        if capped:
            return
        if pos == n * n:
            sols += 1
            return
        r, j = divmod(pos, n)
        rim = rim_sides(n, r, j)
        left = board[pos - 1][E_] if j > 0 else None
        up = board[pos - n][S_] if r > 0 else None
        for p in range(P):
            if used[p]:
                continue
            seen = set()
            for t in rots[p]:
                if t in seen:
                    continue
                seen.add(t)
                if any((t[s] == 0) != rim[s] for s in range(4)):
                    continue
                if left is not None and t[W_] != left:
                    continue
                if up is not None and t[N_] != up:
                    continue
                nodes += 1
                if nodes > cap:
                    capped = True
                    return
                used[p] = True
                board[pos] = t
                rec(pos + 1)
                used[p] = False
                if capped:
                    return
    rec(0)
    return nodes, sols, capped


class DM:
    def __init__(self, n, pieces, C, sets="A", form="a", seed=0):
        self.n, self.C = n, C
        self.P = len(pieces)
        st = []  # oriented states (piece, rot) -> colours
        for p, t in enumerate(pieces):
            for k in range(4):
                st.append((p, t[k:] + t[:k]))
        self.T = len(st)
        self.st_piece = np.array([s[0] for s in st])
        self.col = np.array([s[1] for s in st])  # (T,4)
        # validity mask: grey sides exactly on the rim
        V = np.zeros((n, n, self.T), bool)
        for r in range(n):
            for j in range(n):
                rim = np.array(rim_sides(n, r, j))
                V[r, j] = ((self.col == 0) == rim[None, :]).all(axis=1)
        self.V = V
        self.neg = np.where(V, 0.0, -1e9)
        self.sidx = np.arange(4)[None, :]
        self.sets = sets
        self.form = form
        self.rng = np.random.default_rng(seed)
        self.onehot = np.zeros((self.T, 4, C))
        for t in range(self.T):
            for s in range(4):
                self.onehot[t, s, self.col[t, s]] = 1.0

    def unary(self, x):
        # (n,n,T): sum over sides of x[r,j,s,col[t,s]]
        return x[:, :, self.sidx, self.col].sum(axis=3) + self.neg

    def proj_A(self, x):
        n, T, P = self.n, self.T, self.P
        U = self.unary(x).reshape(n * n, P, 4)
        rot = U.argmax(axis=2)
        Sc = U.max(axis=2)
        rows, cols = linear_sum_assignment(Sc, maximize=True)
        states = np.empty(n * n, int)
        states[rows] = cols * 4 + rot[rows, cols]
        return states.reshape(n, n)

    def viterbi(self, U, out_side, in_side):
        # U: (L, T) unary along a line; consecutive cells: col[out_side] == col[in_side]
        L, T, C = U.shape[0], self.T, self.C
        co, ci = self.col[:, out_side], self.col[:, in_side]
        m = U[0].copy()
        back = np.empty((L, C), int)
        for i in range(1, L):
            order = np.argsort(m)  # ascending, last write wins = max
            best = np.full(C, -1e18)
            arg = np.zeros(C, int)
            best[co[order]] = m[order]
            arg[co[order]] = order
            back[i] = arg
            m = best[ci] + U[i]
        states = np.empty(L, int)
        states[L - 1] = int(m.argmax())
        for i in range(L - 1, 0, -1):
            states[i - 1] = back[i][ci[states[i]]]
        return states

    def proj_R(self, x):
        U = self.unary(x)
        return np.stack([self.viterbi(U[r], E_, W_) for r in range(self.n)])

    def proj_K(self, x):
        U = self.unary(x)
        return np.stack([self.viterbi(U[:, j], S_, N_) for j in range(self.n)], axis=1)

    def embed(self, states):
        return self.onehot[states]  # (n,n,4,C)

    def proj_B(self, y):
        z = y.copy()
        a = 0.5 * (y[:, :-1, E_] + y[:, 1:, W_])
        z[:, :-1, E_] = a
        z[:, 1:, W_] = a
        a = 0.5 * (y[:-1, :, S_] + y[1:, :, N_])
        z[:-1, :, S_] = a
        z[1:, :, N_] = a
        for side, sl in ((N_, (0, slice(None))), (S_, (-1, slice(None))),
                         (W_, (slice(None), 0)), (E_, (slice(None), -1))):
            z[sl[0], sl[1], side, :] = 0.0
            z[sl[0], sl[1], side, 0] = 1.0
        return z

    def solved(self, states):
        c = self.col[states]
        return bool((c[:, :-1, E_] == c[:, 1:, W_]).all() and (c[:-1, :, S_] == c[1:, :, N_]).all())

    def mismatches(self, states):
        c = self.col[states]
        return int((c[:, :-1, E_] != c[:, 1:, W_]).sum() + (c[:-1, :, S_] != c[1:, :, N_]).sum())

    def divide(self, X):
        out, lay = [], None
        for k, name in enumerate(self.sets):
            s = {"A": self.proj_A, "R": self.proj_R, "K": self.proj_K}[name](X[k])
            if name == "A":
                lay = s
            out.append(self.embed(s))
        return np.stack(out), lay

    def concur(self, X):
        y = self.proj_B(X.mean(axis=0))
        return np.broadcast_to(y, X.shape).copy()

    def run(self, max_iter, beta=1.0):
        n, C, K = self.n, self.C, len(self.sets)
        X = self.rng.random((K, n, n, 4, C))
        best = 10 ** 9
        for it in range(1, max_iter + 1):
            if self.form == "a":   # x + PD(2PC - x) - PC
                pc = self.concur(X)
                pd, lay = self.divide(2 * pc - X)
                X = X + beta * (pd - pc)
            else:                  # x + PC(2PD - x) - PD
                pd, lay = self.divide(X)
                pc = self.concur(2 * pd - X)
                X = X + beta * (pc - pd)
            if it % 1 == 0:
                mm = self.mismatches(lay)
                if mm < best:
                    best = mm
                if mm == 0:
                    return it, 0
        return None, best


def main():
    mode = sys.argv[1]
    if mode == "dfs":
        # dfs n b c seeds
        n, b, c, ns = map(int, sys.argv[2:6])
        for seed in range(ns):
            rng = random.Random(1000 * n + 10 * c + seed)
            pieces = gen(n, b, c, rng)
            t0 = time.process_time()
            nodes, sols, capped = dfs_count(n, pieces)
            print(f"dfs n={n} b={b} c={c} seed={seed} nodes={nodes} sols={sols} capped={capped} "
                  f"t={time.process_time() - t0:.2f}", flush=True)
    elif mode == "dm":
        # dm n b c seeds runs max_iter sets form
        n, b, c, ns, runs, max_iter = map(int, sys.argv[2:8])
        sets, form = sys.argv[8], sys.argv[9]
        beta = float(sys.argv[10]) if len(sys.argv) > 10 else 1.0
        first = int(sys.argv[11]) if len(sys.argv) > 11 else 0
        for seed in range(first, first + ns):
            rng = random.Random(1000 * n + 10 * c + seed)
            pieces = gen(n, b, c, rng)
            its = []
            t0 = time.process_time()
            for run in range(runs):
                dm = DM(n, pieces, b + c + 1, sets=sets, form=form, seed=run)
                it, best = dm.run(max_iter, beta)
                its.append(it if it is not None else f"cap(best={best})")
            dt = time.process_time() - t0
            print(f"dm n={n} b={b} c={c} seed={seed} sets={sets} form={form} beta={beta} its={its} t={dt:.1f}", flush=True)


if __name__ == "__main__":
    main()
