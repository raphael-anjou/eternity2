"""TOY BOARDS ONLY (5x5). What a domain filter computed ONCE at the root is worth to row-scan DFS,
per depth. Domains: all rim-consistent placements; plain arc consistency (board.arc_consistency);
the lift's support, which at 5x5 equals the set of placements some solution uses (measured by the
lift and engineering studies), so the third row is the ceiling of any sound root filter.
Prints nodes per depth and log2 ratios. Usage: rootfilter.py n cf ci seed"""
import os, sys, json, math
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, os.path.join(HERE, "bmcopy"))
import common  # noqa: F401,E402
import numpy as np  # noqa: E402
from board import Board  # noqa: E402

n, cf, ci, seed = map(int, sys.argv[1:5])
assert n <= 5
B = Board(n, cf, ci, seed)
P, N = B.P, B.N
sol = np.zeros(P, bool)
for s in B.sol_idx:
    sol[s] = True
ac = B.arc_consistency()


def widths(mask):
    by = [{} for _ in range(N)]
    for j in range(P):
        if mask[j]:
            c = B.COLS[j]
            by[B.cellOf[j]].setdefault((c[0], c[3]), []).append(j)
    w = [0] * (N + 1)
    used = [False] * B.T
    js = [0] * N

    def rec(x):
        w[x] += 1
        if x == N:
            return
        r, c = divmod(x, n)
        north = B.COLS[js[x - n]][2] if r > 0 else 0
        west = B.COLS[js[x - 1]][1] if c > 0 else 0
        for j in by[x].get((north, west), ()):
            t = B.TILE[j]
            if used[t]:
                continue
            used[t] = True; js[x] = j
            rec(x + 1)
            used[t] = False
    rec(0)
    return w


res = {}
for name, m in (("all", np.ones(P, bool)), ("ac", ac), ("support", sol)):
    w = widths(m)
    res[name] = dict(kept=int(m.sum()), widths=w, nodes=sum(w), log2_nodes=round(math.log2(sum(w)), 2))
a, s_ = res["all"]["widths"], res["support"]["widths"]
res["bits_by_depth_all_over_support"] = [round(math.log2(a[d] / s_[d]), 2) for d in range(N + 1)]
res["board"] = [n, cf, ci, seed]; res["n_sols"] = len(B.sols); res["P"] = P
print(json.dumps(res))
