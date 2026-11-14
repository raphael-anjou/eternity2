"""TOY BOARDS ONLY (5x5). re-check check of "mass off solutions per cell".

Independent of the memos' enumerator: the tiles come from the shared painter (that is the board's
definition), but the placement list, the pin and the solution enumeration are rebuilt here, with a
depth-first search that scans the cells in REVERSE order (the memos scan forward with buckets).
Then the stored points of three studies are re-read and the mass is recomputed, with the hull
distance (LP) next to it.

Usage: check_mass.py   (prints JSON lines; one core, a few seconds per board)
"""
import os, sys, json
import numpy as np
from scipy.optimize import linprog

HERE = os.path.dirname(os.path.abspath(__file__))
MEMOS = os.path.dirname(HERE)
sys.path.insert(0, os.path.join(MEMOS, "far-ideas-geometry"))
from common import gen  # noqa: E402  (the painter only)


def rot(t, k):
    return t[k:] + t[:k]


def build(n, tiles):
    """Placements (cell, tile, rot, sides), rim-consistent, pin = smallest-index corner at cell 0."""
    pin = None
    for i, t in enumerate(tiles):
        if t.count(0) == 2:
            for k in range(4):
                p = rot(t, k)
                if p[0] == 0 and p[3] == 0:
                    pin = (i, k)
            break
    pl = []
    for x in range(n * n):
        r, c = divmod(x, n)
        rim = (r == 0, c == n - 1, r == n - 1, c == 0)
        for i, t in enumerate(tiles):
            seen = set()
            for k in range(4):
                p = rot(t, k)
                if p in seen:
                    continue
                seen.add(p)
                if not all((s == 0) == q for s, q in zip(p, rim)):
                    continue
                if x == 0 and (i, k) != pin:
                    continue
                if x != 0 and i == pin[0]:
                    continue
                pl.append((x, i, k, p))
    return pl, pin


def enumerate_reverse(n, tiles, pl):
    """All solutions, scanning cells N-1 down to 0. Returns list of tuples of placement indices."""
    N = n * n
    by_cell = [[] for _ in range(N)]
    for j, (x, i, k, p) in enumerate(pl):
        by_cell[x].append(j)
    used = [False] * len(tiles)
    js = [None] * N
    sols = []

    def rec(x):
        if x < 0:
            sols.append(tuple(js))
            return
        r, c = divmod(x, n)
        for j in by_cell[x]:
            _, i, k, p = pl[j]
            if used[i]:
                continue
            if r < n - 1 and p[2] != pl[js[x + n]][3][0]:
                continue
            if c < n - 1 and p[1] != pl[js[x + 1]][3][3]:
                continue
            used[i] = True
            js[x] = j
            rec(x - 1)
            used[i] = False
        js[x] = None

    rec(N - 1)
    return sols


def hull_distance(P, sols, x):
    S = np.zeros((P, len(sols)))
    for k, s in enumerate(sols):
        S[list(s), k] = 1.0
    K = S.shape[1]
    c = np.zeros(K + 1); c[-1] = 1
    A_ub = np.block([[-S, -np.ones((P, 1))], [S, -np.ones((P, 1))]])
    b_ub = np.concatenate([-x, x])
    A_eq = np.concatenate([np.ones(K), [0]])[None, :]
    r = linprog(c, A_ub=A_ub, b_ub=b_ub, A_eq=A_eq, b_eq=[1.0], bounds=[(0, None)] * (K + 1), method="highs")
    return float(r.x[-1])


POINTS = {
    "theory-d splitting, l2, rho 300": "theory-d/raw/x_5_3_4_%d_knn99_kz99_tf1_l2_rho300.npy",
    "theory-d splitting, leak, rho 300": "theory-d/raw/x_5_3_4_%d_knn99_kz99_tf1_leak_rho300.npy",
    "SCS point (lift study, via low-rank folder)": "lowrank-relaxation/raw/x_sdp_5_3_4_%d.npy",
    "re-check leak run": "recheck-convex/raw/x_leak_5_3_4_%d.npy",
}

n, cf, ci = 5, 3, 4
for seed in range(5):
    tiles = gen(n, cf, ci, seed)
    pl, pin = build(n, tiles)
    P, N = len(pl), n * n
    sols = enumerate_reverse(n, tiles, pl)
    sol = np.zeros(P, bool)
    for s in sols:
        sol[list(s)] = True
    cell = np.array([p[0] for p in pl])
    print(json.dumps(dict(seed=seed, P=P, solutions=len(sols), placements_in_a_solution=int(sol.sum()),
                          placements_in_none=int((~sol).sum()), pin=pin)), flush=True)
    for name, pat in POINTS.items():
        f = os.path.join(MEMOS, pat % seed)
        if not os.path.exists(f):
            continue
        x = np.load(f)
        if x.shape != (P,):
            print(json.dumps(dict(seed=seed, point=name, error="shape %s" % (x.shape,))))
            continue
        cs = np.bincount(cell, x, N)
        off = np.where(~sol)[0]
        big = sorted(((float(x[j]), pl[j][:3]) for j in off if x[j] > 1e-3), reverse=True)
        print(json.dumps(dict(
            seed=seed, point=name,
            mass_off_per_cell=float(np.clip(x, 0, None)[~sol].sum() / N),
            mass_off_total=float(np.clip(x, 0, None)[~sol].sum()),
            n_off_above_1e3=len(big), n_off_above_1e5=int((x[~sol] > 1e-5).sum()),
            largest_off=[(round(v, 5), list(map(int, p))) for v, p in big[:12]],
            sol_placements_below_1e4=int((x[sol] < 1e-4).sum()),
            cell_sum_err=float(np.abs(cs - 1).max()), x_min=float(x.min()),
            hull_distance=hull_distance(P, sols, x))), flush=True)
