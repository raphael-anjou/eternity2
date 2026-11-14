"""Second analysis script 5 (gap 1 review). TOY board, not Eternity II.

How much of a clause-learning solver's work is shared between branches of a row
scan? Same solver, same propagation, three ways to do the same complete search
(every solution, then the proof that there is no other):

  whole       one solver on the whole board
  persistent  the board split into every valid row-scan prefix of k cells; one solver
              kept alive, prefixes passed as assumptions in depth-first order
  fresh       the same prefixes, a new solver for each (nothing learned under one
              prefix can serve under another)

log2(fresh / persistent) is the value of what the solver carries from one prefix to
the next. If learning gave a clause-learning solver a growing edge over a search that
forgets, this number would grow with the number of prefixes.

usage: uv run --with python-sat python3 r5_cubes.py SOLVER n cf ci seed MAXCUBES k1 k2 ...
"""
import os
import sys
import time
from math import log2

from pysat.solvers import Solver

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from toylib import generate, encode, rot  # noqa: E402

SOLVER = sys.argv[1]
n, cf, ci, seed, MAXCUBES = (int(x) for x in sys.argv[2:7])
KS = [int(x) for x in sys.argv[7:]]
pieces, pin = generate(n, cf, ci, seed)
cnf, X, nvars = encode(n, pieces, pin)
xs = set(X.values())
by_cell = {}
for (pos, p, o), v in X.items():
    by_cell.setdefault(pos, []).append((p, o, v))


def prefixes(k, limit):
    out, cur, used = [], [], set()
    board = {}

    def rec(pos):
        if len(out) > limit:
            return
        if pos == k:
            out.append(list(cur))
            return
        y, x = divmod(pos, n)
        for p, o, v in by_cell[pos]:
            if p in used:
                continue
            if x and board[pos - 1][1] != o[3]:
                continue
            if y and board[pos - n][2] != o[0]:
                continue
            used.add(p)
            board[pos] = o
            cur.append(v)
            rec(pos + 1)
            cur.pop()
            used.discard(p)

    rec(0)
    return out


def exhaust(S, assumptions):
    k = 0
    while S.solve(assumptions=assumptions):
        k += 1
        S.add_clause([-v for v in S.get_model() if v > 0 and v in xs])
    return k


t0 = time.time()
with Solver(name=SOLVER, bootstrap_with=cnf) as S:
    sols = exhaust(S, [])
    whole = S.accum_stats()["conflicts"]
print("# TOY. solver %s, n %d, frame colours %d, inner colours %d, seed %d: solutions %d, whole-board conflicts %d (%.0f s)" % (SOLVER, n, cf, ci, seed, sols, whole, time.time() - t0))
print("k | prefixes | conflicts persistent | conflicts fresh | log2(fresh / persistent) | log2(fresh / whole) | s")
for k in KS:
    cubes = prefixes(k, MAXCUBES)
    if len(cubes) > MAXCUBES:
        print("%d | more than %d prefixes, skipped" % (k, MAXCUBES))
        continue
    with Solver(name=SOLVER, bootstrap_with=cnf) as S:
        sp = sum(exhaust(S, c) for c in cubes)
        pers = S.accum_stats()["conflicts"]
    fresh, sf = 0, 0
    for c in cubes:
        with Solver(name=SOLVER, bootstrap_with=cnf) as S:
            sf += exhaust(S, c)
            fresh += S.accum_stats()["conflicts"]
    assert sp == sols == sf, (sp, sf, sols)
    print("%d | %d | %d | %d | %+.2f | %+.2f | %.0f" % (k, len(cubes), pers, fresh, log2(max(fresh, 1) / max(pers, 1)), log2(max(fresh, 1) / max(whole, 1)), time.time() - t0), flush=True)
