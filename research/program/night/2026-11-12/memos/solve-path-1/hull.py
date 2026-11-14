"""Can any vector of per-piece prices make a true solution the unique best colour-only board?
Run: uv run --with scipy python hull.py
For each tiny board, enumerate the usage vector u (how many times each piece appears) of every colour-only
board, then solve the LP: maximise m subject to price.(u - 1) <= -m for every u != all-ones, -1 <= price <= 1.
m > 0 means some prices isolate the true solutions; m = 0 means none do (the dual has a gap)."""
import random, sys, os
from collections import defaultdict
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from toy import gen, table
from scipy.optimize import linprog

def usage_vectors(n, pcs):
    t = table(n, pcs); m = n * n
    cur = {((0,) * n, 0, (0,) * m)}
    for r in range(n):
        for c in range(n):
            nxt = set()
            for (fr, east, u) in cur:
                west = 0 if c == 0 else east
                for (i, E, S) in t.get((fr[c], west, c == n - 1, r == n - 1), ()):
                    nxt.add((fr[:c] + (S,) + fr[c + 1:], E, u[:i] + (u[i] + 1,) + u[i + 1:]))
            cur = nxt
            if len(cur) > 400000: return None
    return {u for (_, _, u) in cur}

for (n, cb, ci, seeds) in ((3, 2, 2, range(101, 109)), (3, 3, 4, range(1, 5)), (4, 3, 4, range(1, 7)), (4, 2, 3, range(101, 104))):
    res = []
    for s in seeds:
        pcs = gen(n, cb, ci, random.Random(s)); m = n * n
        U = usage_vectors(n, pcs)
        if U is None: res.append("skipped"); continue
        fakes = [u for u in U if any(x != 1 for x in u)]
        if not fakes: res.append("no fakes"); continue
        A = [[x - 1 for x in u] + [1.0] for u in fakes]
        r = linprog([0.0] * m + [-1.0], A_ub=A, b_ub=[0.0] * len(fakes), bounds=[(-1, 1)] * m + [(0, 1)], method="highs")
        res.append("%d vectors, margin %.3g" % (len(fakes), r.x[-1]))
    print("n=%d cb=%d ci=%d: %s" % (n, cb, ci, " | ".join(res)), flush=True)
