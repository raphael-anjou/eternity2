"""TOY BOARDS ONLY (generated; never Eternity II progress).

Shared board model for the semidefinite-lift experiment. Everything here is taken from probe B
(../far-ideas-geometry/probe_b_birkhoff_flow.py) so that placements, the centre X0, the
random controls and both near-solution generators are the same objects as in that probe:

  placements p = (cell, tile, rotation), rim-consistent, with the pin (the corner tile of smallest
  index, at cell 0 = top-left, in its only rotation with grey on N and W; the pin tile is excluded
  from every other cell);
  X0 = Sinkhorn of the uniform point (the centre of the rotation-lifted Birkhoff face);
  ctrl = 200 random rim-consistent boards (numpy rng seed 1, as probe B);
  anneal() and dfs_near() copied from probe B with the step / node budgets as parameters.

Added here: arc_consistency() (plain AC, defined in its docstring) and bits().
"""
import os
import sys
import random

import numpy as np

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "scripts", "far-ideas-geometry"))
from common import gen, enumerate_all, oriented, pin_of  # noqa: E402


class Board:
    def __init__(self, n, cf, ci, seed, tiles=None):
        self.n, self.cf, self.ci, self.seed = n, cf, ci, seed
        self.tiles = tiles if tiles is not None else gen(n, cf, ci, seed)
        n = self.n
        N = self.N = n * n
        T = self.T = len(self.tiles)
        base = enumerate_all(n, self.tiles)
        self.sols = base["sols"]
        self.dfs_nodes = base["nodes"]
        pin = self.pin = pin_of(n, self.tiles)
        orient = oriented(n, self.tiles)
        cellOf, tileOf, rotOf, cols = [], [], [], []
        for x in range(N):
            for (i, k, p) in orient[x]:
                if x == 0 and (i, k) != (pin[0], pin[1]):
                    continue
                if x != 0 and i == pin[0]:
                    continue
                cellOf.append(x); tileOf.append(i); rotOf.append(k); cols.append(p)
        self.cellOf = np.array(cellOf); self.tileOf = np.array(tileOf)
        self.rotOf = np.array(rotOf); self.cols = np.array(cols)
        P = self.P = len(cellOf)
        self.index = {(c, t, k): j for j, (c, t, k) in enumerate(zip(cellOf, tileOf, rotOf))}
        nb = np.full((N, 4), -1)
        for x in range(N):
            r, c = divmod(x, n)
            if r > 0: nb[x, 0] = x - n
            if c < n - 1: nb[x, 1] = x + 1
            if r < n - 1: nb[x, 2] = x + n
            if c > 0: nb[x, 3] = x - 1
        self.nb = nb
        self.inner_edges = 2 * n * (n - 1)
        self.sol_idx = [np.array([self.index.get((x, a[0], a[1]), -1) for x, a in enumerate(s)]) for s in self.sols]
        assert all((s >= 0).all() for s in self.sol_idx), "a solution uses a placement outside the model"
        self.X0 = self.sinkhorn(np.ones(P))
        # random controls, exactly as probe B (rng seed 1, 200 boards)
        rng = np.random.default_rng(1)
        self.flow_rng = rng
        kinds = {}
        for x in range(N):
            kinds.setdefault(sum((x // n == 0, x % n == n - 1, x // n == n - 1, x % n == 0)), []).append(x)
        self.rim_cells = kinds
        ctrl = []
        for _ in range(200):
            js = np.full(N, -1)
            for kind, cells in kinds.items():
                cand = sorted({tileOf[j] for j in range(P) if cellOf[j] in cells})
                perm = rng.permutation(cand)
                free = [x for x in cells if x != 0]
                perm = [t for t in perm if t != pin[0]]
                for x, t in zip(free, perm):
                    opts = [j for j in range(P) if cellOf[j] == x and tileOf[j] == t]
                    js[x] = opts[0] if opts else -1
            js[0] = self.index[(0, pin[0], pin[1])]
            ctrl.append(js)
        self.ctrl_idx = ctrl
        self.COLS = self.cols.tolist(); self.NB = nb.tolist(); self.TILE = self.tileOf.tolist()
        opt = {}
        for j in range(P):
            opt.setdefault((int(cellOf[j]), int(tileOf[j])), []).append(j)
        self.OPT = opt

    # ---- probe B pieces -------------------------------------------------------------------
    def sinkhorn(self, X, it=30):
        for _ in range(it):
            cs = np.bincount(self.cellOf, X, self.N); X = X / cs[self.cellOf]
            ts = np.bincount(self.tileOf, X, self.T); X = X / ts[self.tileOf]
        cs = np.bincount(self.cellOf, X, self.N)
        return X / cs[self.cellOf]

    def mismatches(self, js):
        m = 0
        for x in range(self.N):
            for d in (1, 2):
                y = self.NB[x][d]
                if y >= 0 and self.COLS[js[x]][d] != self.COLS[js[y]][(d + 2) % 4]:
                    m += 1
        return m

    def dist_to_sols(self, js):
        js = np.asarray(js)
        return int(min((s != js).sum() for s in self.sol_idx))

    def _mism_cell(self, x, js):
        m = 0
        cx = self.COLS[js[x]]
        for d in range(4):
            y = self.NB[x][d]
            if y >= 0 and cx[d] != self.COLS[js[y]][(d + 2) % 4]:
                m += 1
        return m

    def anneal(self, R, steps):
        """probe B's annealer, unchanged except that the step count is a parameter."""
        js = [int(v) for v in self.ctrl_idx[R.randrange(len(self.ctrl_idx))]]
        kinds_l = [[x for x in self.rim_cells[k] if x != 0] for k in range(3)]
        NB, TILE, OPT = self.NB, self.TILE, self.OPT
        for st in range(steps):
            temp = 1.5 * (1 - st / steps) + 0.05
            cells = kinds_l[R.randrange(3)]
            if len(cells) < 2:
                continue
            x, y = R.sample(cells, 2)
            if cells is kinds_l[0] and R.random() < 0.3:
                y = x
            aff = {x, y} | {z for z in NB[x] if z >= 0} | {z for z in NB[y] if z >= 0}
            before = sum(self._mism_cell(z, js) for z in aff)
            old = (js[x], js[y])
            tx, ty = TILE[js[x]], TILE[js[y]]
            if x == y:
                js[x] = R.choice(OPT[(x, tx)])
            else:
                js[x] = R.choice(OPT[(x, ty)])
                js[y] = R.choice(OPT[(y, tx)])
            delta = (sum(self._mism_cell(z, js) for z in aff) - before) / 2
            if not (delta <= 0 or R.random() < np.exp(-delta / temp)):
                js[x], js[y] = old
        return np.array(js), self.mismatches(js)

    def dfs_near(self, R, maxm, want, budget):
        """probe B's mismatch-tolerant randomised row-scan DFS, budget as a parameter."""
        N, P, COLS, TILE, n = self.N, self.P, self.COLS, self.TILE, self.n
        by_cell = [[j for j in range(P) if self.cellOf[j] == x] for x in range(N)]
        found = []
        nodes = [0]

        class Found(Exception):
            pass

        def rec(x, js, used, mm):
            if len(found) >= want or nodes[0] > budget:
                return
            nodes[0] += 1
            if x == N:
                if self.dist_to_sols(js) >= 8:
                    found.append(np.array(js))
                    raise Found
                return
            r, c = divmod(x, n)
            cand = by_cell[x][:]
            R.shuffle(cand)
            for j in cand:
                t = TILE[j]
                if used[t]:
                    continue
                add = 0
                if r > 0 and COLS[j][0] != COLS[js[x - n]][2]:
                    add += 1
                if c > 0 and COLS[j][3] != COLS[js[x - 1]][1]:
                    add += 1
                if mm + add > maxm:
                    continue
                used[t] = True
                js.append(j)
                rec(x + 1, js, used, mm + add)
                js.pop()
                used[t] = False

        while len(found) < want and nodes[0] <= budget:
            used = [False] * self.T
            used[self.pin[0]] = True
            try:
                rec(1, [self.index[(0, self.pin[0], self.pin[1])]], used, 0)
            except Found:
                pass
        return found, nodes[0]

    def flow_point(self, runs=4):
        """probe B's graduated-assignment flow; returns, per run, the soft point X at the beta
        where the solutions' mean overlap beats the 200 random boards by the largest z
        (probe B's reported row)."""
        N, P, K = self.N, self.P, int(self.cols.max()) + 1
        cellOf, cols, nb = self.cellOf, self.cols, self.nb

        def grad(X):
            S = np.zeros((N, 4, K))
            for d in range(4):
                np.add.at(S[:, d, :], (cellOf, cols[:, d]), X)
            Q = np.zeros(P)
            for d in range(4):
                nbr = nb[cellOf, d]
                has = nbr >= 0
                Q[has] += S[nbr[has], (d + 2) % 4, cols[has, d]]
            return Q

        rng = np.random.default_rng(1)
        pts = []
        sol = np.array(self.sol_idx); ctl = np.array(self.ctrl_idx)
        for run in range(runs):
            X = self.sinkhorn(self.X0 * np.exp(0.05 * rng.standard_normal(P)))
            beta = 0.5
            best = (-1e9, None, None)
            while beta < 200:
                for _ in range(4):
                    Q = grad(X)
                    Y = np.exp(beta * (Q - Q.max())) * np.maximum(X, 1e-300) ** 0.5
                    X = self.sinkhorn(np.maximum(Y, 1e-200))
                ovs = X[sol].sum(1); ovc = X[ctl[(ctl >= 0).all(1)]].sum(1)
                z = (ovs.mean() - ovc.mean()) / max(1e-9, ovc.std())
                if z > best[0]:
                    best = (z, X.copy(), beta)
                beta *= 1.15
            pts.append(best)
        return pts

    # ---- added ------------------------------------------------------------------------------
    def compatible(self, p, q):
        """True when placements p, q (in adjacent cells) have matching facing sides."""
        cp, cq = self.cellOf[p], self.cellOf[q]
        for d in range(4):
            if self.nb[cp, d] == cq:
                return self.cols[p, d] == self.cols[q, (d + 2) % 4]
        return True

    def arc_consistency(self):
        """Plain arc consistency, to a fixpoint, on the binary CSP whose variables are the cells and
        whose values are the placements above. Binary constraints: two adjacent cells must have
        matching facing colours and different tiles; any two cells must have different tiles (so a
        cell whose live values all use one tile t removes t from every other cell). Nothing else
        (no tile-must-be-placed counting, no all-different matching). Returns a boolean mask."""
        P, N = self.P, self.N
        alive = np.ones(P, bool)
        cells = [np.where(self.cellOf == x)[0] for x in range(N)]
        changed = True
        while changed:
            changed = False
            for x in range(N):
                for p in cells[x]:
                    if not alive[p]:
                        continue
                    for d in range(4):
                        y = self.nb[x, d]
                        if y < 0:
                            continue
                        ok = False
                        for q in cells[y]:
                            if alive[q] and self.tileOf[q] != self.tileOf[p] and \
                                    self.cols[p, d] == self.cols[q, (d + 2) % 4]:
                                ok = True
                                break
                        if not ok:
                            alive[p] = False
                            changed = True
                            break
            for x in range(N):
                ts = set(self.tileOf[cells[x][alive[cells[x]]]].tolist())
                if len(ts) == 0:
                    raise RuntimeError("AC wiped out a cell")
                if len(ts) == 1:
                    t = ts.pop()
                    kill = (self.tileOf == t) & (self.cellOf != x) & alive
                    if kill.any():
                        alive[kill] = False
                        changed = True
        return alive


def bits(X, X0, boards, floor):
    """probe B's measure: sum over cells of log2(X[c, s_c] / X0[c, s_c]), with X floored."""
    ok = [s for s in boards if not (np.asarray(s) < 0).any()]
    return np.array([np.log2(np.maximum(X[s], floor) / X0[s]).sum() for s in ok])
