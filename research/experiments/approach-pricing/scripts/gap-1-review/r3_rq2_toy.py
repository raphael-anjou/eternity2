"""Second analysis script 3 (gap 1 review). TOY board, not Eternity II.

The memo says RQ2's kill line (no downward trend in the cost per frame, for frames
that share a bottom arc, fed to one solver kept alive) cannot fire. This script runs
that design on one generated board and prints the cost per frame against the index.

  arc       the planted bottom row and the planted side cells below row FREE
  frames    every way to finish the border above the arc with the remaining border
            pieces, frame joints matching (up to LIMIT of them, shuffled)
  A         one solver kept alive, frames passed as assumptions (the RQ2 design)
  B         a fresh solver for each frame (first NFRESH frames)
  D         one solver, the arc alone as assumptions, every completion blocked until
            UNSAT: the plain fill under the arc, the memo's proposed yardstick

usage: uv run --with python-sat python3 r3_rq2_toy.py SOLVER n cf ci seed FREE LIMIT NFRESH
"""
import os
import random
import sys
import time

from pysat.solvers import Solver

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from toylib import generate, encode, rot  # noqa: E402

SOLVER = sys.argv[1]
n, cf, ci, seed, FREE, LIMIT, NFRESH = (int(x) for x in sys.argv[2:9])
pieces, where = generate(n, cf, ci, seed, full=True)
cnf, X, nvars = encode(n, pieces, None)
rng = random.Random(seed + 1)


def oriented(p, pos):
    y, x = divmod(pos, n)
    for r in range(4):
        o = rot(pieces[p], r)
        if (o[0] == 0) == (y == 0) and (o[1] == 0) == (x == n - 1) and (o[2] == 0) == (y == n - 1) and (o[3] == 0) == (x == 0):
            return o
    return None


# the border as a ring, clockwise from the top-left corner
ring = [x for x in range(n)] + [y * n + n - 1 for y in range(1, n)] + [(n - 1) * n + x for x in range(n - 2, -1, -1)] + [y * n for y in range(n - 2, 0, -1)]
planted = {pos: (where[pos][0], rot(pieces[where[pos][0]], where[pos][1])) for pos in ring}
border_pieces = [p for p, _ in planted.values()]


def joint_ok(pa, oa, pb, ob):
    """cells pa, pb adjacent on the ring"""
    ya, xa = divmod(pa, n)
    yb, xb = divmod(pb, n)
    if ya == yb:
        return oa[1] == ob[3] if xb == xa + 1 else oa[3] == ob[1]
    return oa[2] == ob[0] if yb == ya + 1 else oa[0] == ob[2]


def completions(fixed, limit):
    """Valid frames that extend the dict fixed (pos -> (piece, edges)). Random order."""
    free = [pos for pos in ring if pos not in fixed]
    pool = [p for p in border_pieces if p not in {q for q, _ in fixed.values()}]
    out, cur = [], dict(fixed)
    idx = {pos: i for i, pos in enumerate(ring)}

    def rec(k, pool):
        if len(out) >= limit:
            return
        if k == len(free):
            out.append(dict(cur))
            return
        pos = free[k]
        cand = pool[:]
        rng.shuffle(cand)
        for p in cand:
            o = oriented(p, pos)
            if o is None:
                continue
            ok = True
            for nb in (ring[(idx[pos] - 1) % len(ring)], ring[(idx[pos] + 1) % len(ring)]):
                if nb in cur and not joint_ok(pos, o, nb, cur[nb][1]):
                    ok = False
            if not ok:
                continue
            cur[pos] = (p, o)
            rec(k + 1, [q for q in pool if q != p])
            del cur[pos]

    rec(0, pool)
    return out


def assume(frame, cells=None):
    return [X[(pos, p, o)] for pos, (p, o) in frame.items() if cells is None or pos in cells]


arc_cells = {pos for pos in ring if pos // n == n - 1 or pos // n > FREE}
arc = {pos: planted[pos] for pos in arc_cells}
frames = completions(arc, LIMIT)
rng.shuffle(frames)
print("# TOY. solver %s, n %d, frame colours %d, inner colours %d, seed %d. Arc: %d of %d border cells; frames that extend it: %d%s" % (
    SOLVER, n, cf, ci, seed, len(arc_cells), len(ring), len(frames), " (limit reached)" if len(frames) >= LIMIT else " (all)"))


def run_persistent(fs):
    out = []
    with Solver(name=SOLVER, bootstrap_with=cnf) as S:
        prev = 0
        for f in fs:
            sat = S.solve(assumptions=assume(f))
            c = S.accum_stats()["conflicts"]
            out.append((c - prev, sat))
            prev = c
    return out


def blocks(name, res, size):
    print("%s: frames %d, satisfiable %d, total conflicts %d" % (name, len(res), sum(1 for _, s in res if s), sum(c for c, _ in res)))
    for i in range(0, len(res), size):
        b = [c for c, _ in res[i:i + size]]
        if len(b) == size:
            print("   frames %4d to %4d: mean conflicts %.1f, median %d" % (i, i + size - 1, sum(b) / len(b), sorted(b)[len(b) // 2]))


t0 = time.time()
size = max(10, len(frames) // 10)
A = run_persistent(frames)
blocks("A persistent, shared arc", A, size)
print("   first frame %d, frames 1 to 9 mean %.1f   (%.0f s)" % (A[0][0], sum(c for c, _ in A[1:10]) / 9, time.time() - t0))
B = []
for f in frames[:NFRESH]:
    with Solver(name=SOLVER, bootstrap_with=cnf) as S:
        sat = S.solve(assumptions=assume(f))
        B.append((S.accum_stats()["conflicts"], sat))
blocks("B fresh solver per frame", B, max(10, NFRESH // 4))
print("   mean over the same first %d frames: fresh %.1f, persistent %.1f   (%.0f s)" % (len(B), sum(c for c, _ in B) / len(B), sum(c for c, _ in A[:len(B)]) / len(B), time.time() - t0))
with Solver(name=SOLVER, bootstrap_with=cnf) as S:
    k = 0
    xs = set(X.values())
    while S.solve(assumptions=assume(arc)):
        k += 1
        S.add_clause([-v for v in S.get_model() if v > 0 and v in xs])
    print("D plain fill under the arc: solutions %d, conflicts %d   (%.0f s)" % (k, S.accum_stats()["conflicts"], time.time() - t0))
