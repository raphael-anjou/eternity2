#!/usr/bin/env python3
"""Referee script 2. Exact enumeration on small boards against the first-moment formula.

Run: python3 -I r2_enumerate.py   (about two minutes, one core, stdlib only)

Own instance generator, own scorer, own depth-first enumerator. The formula side
uses the transfer counts of r1_counts.py (not the memo's code).

Part 1  2x2, every instance of the ensemble, every placement: exact rationals.
Part 2  3x3, sampled instances, every placement (2304): full law of the number of breaks.
Part 3  4x4, sampled instances, depth-first count of partial and complete boards
        with at most J breaks: the memo's T(d, j), my own thinning.
Part 4  4x4, planted instances with the same colour census: what the ensemble
        average hides (mean, spread, share of instances with no perfect board).
"""
import itertools
import math
import random
import sys
import os
from fractions import Fraction
from math import comb, factorial

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from r1_counts import matching_dist, bijection_dist, dfact


# ---------- geometry ----------
def cell_class(n, x, y):
    bx, by = x in (0, n - 1), y in (0, n - 1)
    return 'c' if (bx and by) else ('e' if (bx or by) else 'i')


def ring_roles(n, x, y):
    """For a rim cell: (index of the clockwise-facing side, index of the anticlockwise-facing
    side, index of the inward side or None). Sides are U, R, D, L = 0..3."""
    top, right, bottom, left = y == 0, x == n - 1, y == n - 1, x == 0
    if top and left:
        return 1, 2, None
    if top and right:
        return 2, 3, None
    if bottom and right:
        return 3, 0, None
    if bottom and left:
        return 0, 1, None
    if top:
        return 1, 3, 2
    if right:
        return 2, 0, 3
    if bottom:
        return 3, 1, 0
    if left:
        return 0, 2, 1
    return None


def options(n, pieces):
    """Per cell: list of (piece id, side colours U R D L) for every class-respecting way
    to put a piece there. Corner piece = (cw, ccw); border = (cw, ccw, inner); interior = 4 colours."""
    out = []
    for y in range(n):
        for x in range(n):
            cls = cell_class(n, x, y)
            opts = []
            for pid, (pc, cols) in enumerate(pieces):
                if pc != cls:
                    continue
                if cls == 'i':
                    for r in range(4):
                        opts.append((pid, cols[r:] + cols[:r]))
                else:
                    cw, ccw, inn = ring_roles(n, x, y)
                    s = [None] * 4
                    s[cw], s[ccw] = ('r', cols[0]), ('r', cols[1])
                    if inn is not None:
                        s[inn] = cols[2]
                    opts.append((pid, tuple(s)))
            out.append(opts)
    return out


def shuffled_instance(n, ring_counts, int_half_counts, rng):
    """One draw of the colour-shuffle ensemble."""
    cws = [c for c, k in enumerate(ring_counts) for _ in range(k)]
    ccws = list(cws)
    inner = [100 + c for c, k in enumerate(int_half_counts) for _ in range(k)]
    rng.shuffle(cws); rng.shuffle(ccws); rng.shuffle(inner)
    return build(n, cws, ccws, inner)


def build(n, cws, ccws, inner):
    cws, ccws, inner = list(cws), list(ccws), list(inner)
    pieces = []
    for _ in range(4):
        pieces.append(('c', (cws.pop(), ccws.pop())))
    for _ in range(4 * (n - 2)):
        pieces.append(('e', (cws.pop(), ccws.pop(), inner.pop())))
    for _ in range((n - 2) ** 2):
        pieces.append(('i', (inner.pop(), inner.pop(), inner.pop(), inner.pop())))
    assert not cws and not ccws and not inner
    return pieces


def planted_instance(n, ring_counts, int_joint_counts, rng):
    """A solved board: joints coloured at random with the given numbers of joints per colour,
    pieces read off the board, then shuffled. Same colour census as the ensemble."""
    rj = [c for c, k in enumerate(ring_counts) for _ in range(k)]
    ij = [100 + c for c, k in enumerate(int_joint_counts) for _ in range(k)]
    rng.shuffle(rj); rng.shuffle(ij)
    grid = [[[None] * 4 for _ in range(n)] for _ in range(n)]
    for y in range(n):
        for x in range(n):
            if x + 1 < n:
                ring = y in (0, n - 1)
                c = rj.pop() if ring else ij.pop()
                grid[y][x][1] = grid[y][x + 1][3] = c
            if y + 1 < n:
                ring = x in (0, n - 1)
                c = rj.pop() if ring else ij.pop()
                grid[y][x][2] = grid[y + 1][x][0] = c
    pieces = []
    for y in range(n):
        for x in range(n):
            cls = cell_class(n, x, y)
            s = grid[y][x]
            if cls == 'i':
                r = rng.randrange(4)
                pieces.append(('i', tuple(s[r:] + s[:r])))
            else:
                cw, ccw, inn = ring_roles(n, x, y)
                pieces.append((cls, (s[cw], s[ccw]) if inn is None else (s[cw], s[ccw], s[inn])))
    rng.shuffle(pieces)
    return pieces


def col(v):
    return v[1] if isinstance(v, tuple) else v


def full_histogram(n, pieces):
    """Number of breaks of EVERY class-respecting placement (small boards only)."""
    opts = options(n, pieces)
    cells = n * n
    by_class = {'c': [], 'e': [], 'i': []}
    for d in range(cells):
        by_class[cell_class(n, d % n, d // n)].append(d)
    ids = {k: sorted({pid for pid, _ in opts[d]}) for k, v in by_class.items() for d in v[:1]}
    look = [dict() for _ in range(cells)]
    for d in range(cells):
        for pid, s in opts[d]:
            look[d].setdefault(pid, []).append(s)
    hist = {}
    board = [None] * cells
    joints = []
    for y in range(n):
        for x in range(n):
            d = y * n + x
            if x + 1 < n:
                joints.append((d, 1, d + 1, 3))
            if y + 1 < n:
                joints.append((d, 2, d + n, 0))
    icells = by_class['i']
    for pc in itertools.permutations(ids.get('c', [])):
        for d, pid in zip(by_class['c'], pc):
            board[d] = look[d][pid][0]
        for pe in itertools.permutations(ids.get('e', [])):
            for d, pid in zip(by_class['e'], pe):
                board[d] = look[d][pid][0]
            for pi in itertools.permutations(ids.get('i', [])):
                for rots in itertools.product(range(4), repeat=len(icells)):
                    for d, pid, r in zip(icells, pi, rots):
                        board[d] = look[d][pid][r]
                    k = 0
                    for a, sa, b, sb in joints:
                        if col(board[a][sa]) != col(board[b][sb]):
                            k += 1
                    hist[k] = hist.get(k, 0) + 1
    return hist


def dfs_counts(n, pieces, J):
    """counts[d][j] = number of row-scan partial boards of d pieces with exactly j breaks (j <= J)."""
    opts = options(n, pieces)
    cells = n * n
    counts = [[0] * (J + 1) for _ in range(cells + 1)]
    board = [None] * cells
    used = [False] * len(pieces)

    def rec(d, j):
        counts[d][j] += 1
        if d == cells:
            return
        x, y = d % n, d // n
        for pid, s in opts[d]:
            if used[pid]:
                continue
            b = 0
            if x > 0 and col(board[d - 1][1]) != col(s[3]):
                b += 1
            if y > 0 and col(board[d - n][2]) != col(s[0]):
                b += 1
            if j + b > J:
                continue
            used[pid] = True
            board[d] = s
            rec(d + 1, j + b)
            used[pid] = False

    rec(0, 0)
    return counts


# ---------- formula side (own implementation) ----------
def laws(ring_counts, int_half_counts):
    er = sum(ring_counts)
    ei = sum(int_half_counts) // 2
    DR = bijection_dist(ring_counts, ring_counts, er)
    DI = matching_dist(int_half_counts, ei) if ei else [1]
    R = [Fraction(v, factorial(er)) for v in DR]
    I = [Fraction(v, dfact(2 * ei - 1)) for v in DI] if ei else [Fraction(1)]
    assert sum(R) == 1 and sum(I) == 1
    return R, I


def weight(n):
    ne, ni = 4 * (n - 2), (n - 2) ** 2
    return 24 * factorial(ne) * factorial(ni) * 4 ** ni


def expected_full(n, ring_counts, int_half_counts):
    R, I = laws(ring_counts, int_half_counts)
    W = weight(n)
    out = {}
    for kr, a in enumerate(R):
        for ki, b in enumerate(I):
            if a and b:
                out[kr + ki] = out.get(kr + ki, 0) + W * a * b
    return out


def thin(law, e, j):
    E = len(law) - 1
    if j < 0 or j > e:
        return Fraction(0)
    return sum(law[k] * comb(k, j) * comb(E - k, e - j) for k in range(j, E - (e - j) + 1)) / comb(E, e)


def expected_prefix(n, ring_counts, int_half_counts, d, J):
    """E[number of row-scan partial boards with d pieces and exactly j breaks], j = 0..J."""
    R, I = laws(ring_counts, int_half_counts)
    left = {'c': 4, 'e': 4 * (n - 2), 'i': (n - 2) ** 2}
    A, er, ei = 1, 0, 0
    for t in range(d):
        x, y = t % n, t // n
        cls = cell_class(n, x, y)
        A *= left[cls] * (4 if cls == 'i' else 1)
        left[cls] -= 1
        if x > 0:
            if y in (0, n - 1):
                er += 1
            else:
                ei += 1
        if y > 0:
            if x in (0, n - 1):
                er += 1
            else:
                ei += 1
    return [A * sum(thin(R, er, jr) * thin(I, ei, j - jr) for jr in range(j + 1)) for j in range(J + 1)]


def mean_se(xs):
    m = sum(xs) / len(xs)
    v = sum((x - m) ** 2 for x in xs) / (len(xs) - 1)
    return m, math.sqrt(v / len(xs)), math.sqrt(v)


def main():
    rng = random.Random(4711)

    print('== Part 1: 2x2, all instances, all placements, exact ==')
    for ring_counts in ([2, 2], [2, 1, 1], [3, 1], [1, 1, 1, 1]):
        base = [c for c, k in enumerate(ring_counts) for _ in range(k)]
        arr = sorted(set(itertools.permutations(base)))
        tot, ninst = {}, 0
        for cws in arr:
            for ccws in arr:
                h = full_histogram(2, build(2, cws, ccws, []))
                ninst += 1
                for k, v in h.items():
                    tot[k] = tot.get(k, 0) + v
        got = {k: Fraction(v, ninst) for k, v in sorted(tot.items())}
        exp = {k: v for k, v in sorted(expected_full(2, ring_counts, []).items()) if v}
        print('ring %s: %d instances; enumerated mean N(k) %s ; formula %s ; %s' % (
            ring_counts, ninst, {k: str(v) for k, v in got.items()}, {k: str(v) for k, v in exp.items()},
            'EQUAL' if got == exp else 'DIFFERENT'))

    print('\n== Part 2: 3x3, sampled instances, every one of the 2304 placements ==')
    for ring_counts, ihc, ninst in (([4, 4], [4, 4], 500), ([4, 2, 2], [4, 2, 2], 500)):
        exp = expected_full(3, ring_counts, ihc)
        acc = {}
        hs = []
        for _ in range(ninst):
            h = full_histogram(3, shuffled_instance(3, ring_counts, ihc, rng))
            hs.append(h)
        ks = sorted(set(exp) | {k for h in hs for k in h})
        worst = 0.0
        print('ring %s, interior half-edges %s, %d instances' % (ring_counts, ihc, ninst))
        print('  k   formula E[N(k)]   enumerated mean +- s.e.     z    share of instances with N(k) = 0')
        for k in ks:
            xs = [h.get(k, 0) for h in hs]
            m, se, sd = mean_se(xs)
            e = float(exp.get(k, 0))
            z = (m - e) / se if se > 0 else 0.0
            worst = max(worst, abs(z))
            print(' %2d   %12.4f   %12.4f +- %-8.4f %6.2f   %.3f' % (k, e, m, se, z, sum(1 for x in xs if x == 0) / len(xs)))
        print('  largest |z| %.2f over %d values of k' % (worst, len(ks)))

    print('\n== Part 3: 4x4, sampled instances, depth-first count with at most J = 2 breaks anywhere ==')
    n, ring_counts, ijc, J, ninst = 4, [6, 3, 3], [5, 4, 3], 2, 160
    ihc = [2 * c for c in ijc]
    ens = [dfs_counts(n, shuffled_instance(n, ring_counts, ihc, rng), J) for _ in range(ninst)]
    print('ring joints per colour %s, interior joints per colour %s, %d instances' % (ring_counts, ijc, ninst))
    print('  d  j   formula        enumerated mean +- s.e.      z')
    worst, ncmp = 0.0, 0
    for d in (3, 5, 7, 9, 11, 13, 15, 16):
        exp = expected_prefix(n, ring_counts, ihc, d, J)
        for j in range(J + 1):
            xs = [c[d][j] for c in ens]
            m, se, sd = mean_se(xs)
            e = float(exp[j])
            z = (m - e) / se if se > 0 else 0.0
            worst = max(worst, abs(z)); ncmp += 1
            print(' %2d  %d  %12.3f   %12.3f +- %-9.3f %6.2f' % (d, j, e, m, se, z))
    print('  largest |z| %.2f over %d comparisons' % (worst, ncmp))

    print('\n== Part 4: 4x4, planted instances with the same census, same enumeration ==')
    pl = [dfs_counts(n, planted_instance(n, ring_counts, ijc, rng), J) for _ in range(ninst)]
    exp = expected_prefix(n, ring_counts, ihc, 16, J)
    print('complete boards with exactly j breaks (no piece pinned, so each board counts with its 4 turns):')
    print('  j   formula    ensemble mean (sd, share zero)      planted mean (sd, share zero)    planted / formula')
    for j in range(J + 1):
        a = [c[16][j] for c in ens]
        b = [c[16][j] for c in pl]
        ma, sea, sda = mean_se(a)
        mb, seb, sdb = mean_se(b)
        e = float(exp[j])
        print('  %d  %9.3f   %9.3f (%8.3f, %.2f)          %9.3f (%8.3f, %.2f)         %s' % (
            j, e, ma, sda, sum(1 for x in a if x == 0) / len(a), mb, sdb, sum(1 for x in b if x == 0) / len(b),
            '%.2f' % (mb / e) if e else 'formula is zero'))
    for d in (8, 12):
        exp = expected_prefix(n, ring_counts, ihc, d, J)
        a = [sum(c[d]) for c in ens]
        b = [sum(c[d]) for c in pl]
        print('  nodes at depth %d (all j <= 2): formula %.1f, ensemble mean %.1f, planted mean %.1f' % (
            d, float(sum(exp)), sum(a) / len(a), sum(b) / len(b)))
    a = [sum(sum(r) for r in c[1:]) for c in ens]
    b = [sum(sum(r) for r in c[1:]) for c in pl]
    fa = [c[16][0] + c[16][1] + c[16][2] for c in ens]
    fb = [c[16][0] + c[16][1] + c[16][2] for c in pl]
    print('  nodes per board found, ratio of sums: ensemble %.1f, planted %.1f' % (sum(a) / max(1, sum(fa)), sum(b) / max(1, sum(fb))))
    per = sorted((x / y) for x, y in zip(a, fa) if y)
    print('  nodes per board, instance by instance (ensemble, instances with a board: %d of %d): min %.1f median %.1f max %.1f' % (
        len(per), ninst, per[0], per[len(per) // 2], per[-1]))


if __name__ == '__main__':
    main()
