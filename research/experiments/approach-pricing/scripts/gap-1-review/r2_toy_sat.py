"""Second analysis script 2 (gap 1 review). TOY boards, not Eternity II.

Generated framed edge-matching boards near the critical colour count, one corner
pinned. For each board, two complete searches (every solution found, then the
proof that there is no other):

  tree    plain row-scan backtracking, no propagation, no learning (rowscan.c)
  cdcl    an off-the-shelf clause-learning solver on a direct encoding
          (placement variables, joint-colour variables, each cell once, each tile
          once), solutions blocked one by one until UNSAT

Reported: tree nodes and dead ends, solver conflicts, and log2 of the ratios.

usage: uv run --with python-sat python3 r2_toy_sat.py SOLVER n cf ci seed0 nseeds [conflict budget]
"""
import os
import subprocess
import sys
import time
from math import lgamma, log, log2

from pysat.solvers import Solver

HERE = os.path.dirname(os.path.abspath(__file__))
SOLVER = sys.argv[1]
n, cf, ci, seed0, nseeds = (int(x) for x in sys.argv[2:7])
BUDGET = int(sys.argv[7]) if len(sys.argv) > 7 else 0


sys.path.insert(0, HERE)
from toylib import generate, encode  # noqa: E402


def first_moment():
    m = n - 2
    return (lgamma(4) + lgamma(4 * m + 1) + lgamma(m * m + 1) + m * m * log(4) - 4 * (n - 1) * log(cf) - 2 * (n - 1) * (n - 2) * log(ci)) / log(2)


def tree(pieces, pin):
    txt = "%d\n" % n + "".join("%d %d %d %d\n" % p for p in pieces) + "%d %d\n" % pin
    out = subprocess.run([os.path.join(HERE, "rowscan"), "200000000000"], input=txt, capture_output=True, text=True).stdout.split()
    return None if out[0] == "CAP" else tuple(int(x) for x in out)


def cdcl(pieces, pin):
    cnf, X, nvars = encode(n, pieces, pin)
    sols = 0
    xset = set(X.values())
    with Solver(name=SOLVER, bootstrap_with=cnf) as S:
        while True:
            if BUDGET:
                left = BUDGET - S.accum_stats()["conflicts"]
                if left <= 0:
                    return None, sols, nvars, len(cnf)
                S.conf_budget(left)
                res = S.solve_limited()
                if res is None:
                    return None, sols, nvars, len(cnf)
            else:
                res = S.solve()
            if not res:
                break
            sols += 1
            S.add_clause([-v for v in S.get_model() if v > 0 and v in xset])
        st = S.accum_stats()
    return st, sols, nvars, len(cnf)


print("# TOY boards. solver %s, n %d, frame colours %d, inner colours %d, log2 expected solutions with one corner pinned %.1f" % (SOLVER, n, cf, ci, first_moment()))
print("seed | solutions | tree nodes, dead ends, s | conflicts, decisions, s | log2(nodes / conflicts) | log2(dead ends / conflicts) | vars clauses")
r1, r2 = [], []
for seed in range(seed0, seed0 + nseeds):
    pieces, pin = generate(n, cf, ci, seed)
    t0 = time.time()
    tr = tree(pieces, pin)
    t1 = time.time()
    st, sols, nvars, ncl = cdcl(pieces, pin)
    t2 = time.time()
    if tr is None or st is None:
        print("%d | capped: tree %s (nodes, dead ends, solutions), solver %s after %.0f s, conflict budget %d, solutions found before the cap %d" % (seed, tr, st, t2 - t1, BUDGET, sols), flush=True)
        if tr is not None and BUDGET:
            print("    so log2(nodes / conflicts) is below %.2f and log2(dead ends / conflicts) below %.2f on this board" % (log2(tr[0] / BUDGET), log2(tr[1] / BUDGET)), flush=True)
        continue
    assert tr[2] == sols, (tr, sols)
    c = max(st["conflicts"], 1)
    a, b = log2(tr[0] / c), log2(max(tr[1], 1) / c)
    r1.append(a)
    r2.append(b)
    print("%d | %d | %d %d %.1f | %d %d %.1f | %+.2f | %+.2f | %d %d" % (seed, sols, tr[0], tr[1], t1 - t0, st["conflicts"], st["decisions"], t2 - t1, a, b, nvars, ncl), flush=True)
if r1:
    med = lambda v: sorted(v)[len(v) // 2]
    print("MEDIAN n %d ci %d %s: log2(nodes / conflicts) %+.2f, log2(dead ends / conflicts) %+.2f, boards %d" % (n, ci, SOLVER, med(r1), med(r2), len(r1)))
