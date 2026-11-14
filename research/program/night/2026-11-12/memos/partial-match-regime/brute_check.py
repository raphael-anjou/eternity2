#!/usr/bin/env python3
"""Numerical check of the algebra in model.py on tiny boards. Stdlib only.

Run:  python3 -I brute_check.py      (under a minute, one core)

Check 1. interior_dist and ring_dist against brute-force enumeration of every
         perfect matching / bijection of a small coloured multiset.
Check 2. The tree formula E[#partial boards at depth d with j breaks] against a
         real depth-first enumeration on random 4x4 instances drawn from the
         colour-shuffle ensemble (3 ring colours, 3 interior colours), breaks
         allowed anywhere, budget 2.
"""
import importlib.util
import itertools
import math
import os
import random
import sys

here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location('model', os.path.join(here, 'model.py'))
model = importlib.util.module_from_spec(spec)
spec.loader.exec_module(model)


def all_matchings(items):
    if not items:
        yield []
        return
    a = items[0]
    for i in range(1, len(items)):
        rest = items[1:i] + items[i + 1:]
        for m in all_matchings(rest):
            yield [(a, items[i])] + m


def check1():
    ok = True
    for counts in ([2, 2, 2], [4, 2, 2], [4, 4], [2, 2, 2, 2], [4, 4, 2]):
        cols = [c for c, n in enumerate(counts) for _ in range(n)]
        hist = [0] * (len(cols) // 2 + 1)
        for m in all_matchings(list(range(len(cols)))):
            hist[sum(1 for a, b in m if cols[a] != cols[b])] += 1
        got = model.interior_dist(counts)
        ok &= (hist == got)
        print('interior_dist', counts, 'brute', hist, 'formula', got, 'OK' if hist == got else 'MISMATCH')
    for cw, ccw in (([2, 2, 2], [2, 2, 2]), ([3, 2, 1], [3, 2, 1]), ([2, 2], [2, 2]), ([3, 3], [3, 3])):
        a = [c for c, n in enumerate(cw) for _ in range(n)]
        b = [c for c, n in enumerate(ccw) for _ in range(n)]
        hist = [0] * (len(a) + 1)
        for perm in itertools.permutations(range(len(b))):
            hist[sum(1 for i, j in enumerate(perm) if a[i] != b[j])] += 1
        got = model.ring_dist(cw, ccw)
        ok &= (hist == got)
        print('ring_dist', cw, 'brute', hist, 'formula', got, 'OK' if hist == got else 'MISMATCH')
    return ok


def ensemble_instance(n, ring_counts, int_counts, rng):
    """Pieces of an n x n board whose side colours are shuffled (not planted)."""
    outs = [c + 1 for c, k in enumerate(ring_counts) for _ in range(k)]
    ins = list(outs)
    nr = len(ring_counts)
    inner = [nr + 1 + c for c, k in enumerate(int_counts) for _ in range(2 * k)]
    rng.shuffle(outs)
    rng.shuffle(ins)
    rng.shuffle(inner)
    pieces = []
    for y in range(n):
        for x in range(n):
            t = [None, None, None, None]  # top, right, bottom, left
            top, right, bottom, left = y == 0, x == n - 1, y == n - 1, x == 0
            for side, rim in enumerate((top, right, bottom, left)):
                if rim:
                    t[side] = 0
            # clockwise walk: top row -> right, right column -> down, bottom row -> left, left column -> up
            if top and left:
                i, o = 2, 1
            elif top and right:
                i, o = 3, 2
            elif bottom and right:
                i, o = 0, 3
            elif bottom and left:
                i, o = 1, 0
            elif top:
                i, o = 3, 1
            elif right:
                i, o = 0, 2
            elif bottom:
                i, o = 1, 3
            elif left:
                i, o = 2, 0
            else:
                i = o = None
            if i is not None:
                t[i] = ('in', ins.pop())
                t[o] = ('out', outs.pop())
            for side in range(4):
                if t[side] is None:
                    t[side] = ('x', inner.pop())
            pieces.append(tuple(t))
    return pieces


def enumerate_tree(n, pieces, J):
    """Row-scan DFS over every class-respecting partial board with at most J broken
    joints (anywhere). Returns counts[d][j]. Labelled placements, as in the model."""
    cells = []
    for y in range(n):
        for x in range(n):
            rim = (y == 0, x == n - 1, y == n - 1, x == 0)
            cand = []
            for pid, p in enumerate(pieces):
                for r in range(4):
                    q = p[r:] + p[:r]
                    if all((q[s] == 0) == rim[s] for s in range(4)):
                        cand.append((pid, q))
            cells.append(cand)
    counts = [[0] * (J + 1) for _ in range(n * n + 1)]
    board = [None] * (n * n)
    used = [False] * len(pieces)

    def col(v):
        return v[1]

    def rec(d, j):
        counts[d][j] += 1
        if d == n * n:
            return
        x, y = d % n, d // n
        for pid, q in cells[d]:
            if used[pid]:
                continue
            b = 0
            if x > 0 and col(board[d - 1][1]) != col(q[3]):
                b += 1
            if y > 0 and col(board[d - n][2]) != col(q[0]):
                b += 1
            if j + b > J:
                continue
            used[pid] = True
            board[d] = q
            rec(d + 1, j + b)
            used[pid] = False
        board[d] = None

    rec(0, 0)
    return counts


def check2(instances=120, seed=20261112):
    n, ring_counts, int_counts, J = 4, [4, 4, 4], [4, 4, 4], 2
    b = model.Board(n, ring_counts, int_counts)
    levels = b.tree(J, lambda d: J, per_cell=2)
    rng = random.Random(seed)
    acc = [[[] for _ in range(J + 1)] for _ in range(n * n + 1)]
    for _ in range(instances):
        c = enumerate_tree(n, ensemble_instance(n, ring_counts, int_counts, rng), J)
        for d in range(n * n + 1):
            for j in range(J + 1):
                acc[d][j].append(c[d][j])
    print('4x4 ensemble, %d instances, breaks anywhere, budget %d' % (instances, J))
    print(' d  j   formula E[count]   measured mean +- s.e.   z')
    worst = 0.0
    for d in (2, 4, 6, 8, 10, 12, 14, 15, 16):
        for j in range(J + 1):
            pred = math.exp(levels[d].get(j, -math.inf))
            xs = acc[d][j]
            m = sum(xs) / len(xs)
            se = math.sqrt(sum((x - m) ** 2 for x in xs) / (len(xs) - 1) / len(xs)) if len(xs) > 1 else 0.0
            z = (m - pred) / se if se > 0 else 0.0
            worst = max(worst, abs(z))
            print('%2d  %d   %14.4f   %14.4f +- %-10.4f %6.2f' % (d, j, pred, m, se, z))
    print('largest |z| over the 27 cells shown: %.2f (about 2 to 3 is expected for 27 comparisons)' % worst)
    return worst


if __name__ == '__main__':
    ok = check1()
    print('check 1:', 'all exact' if ok else 'FAILED')
    check2()
