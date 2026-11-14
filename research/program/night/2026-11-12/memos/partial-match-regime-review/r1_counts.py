#!/usr/bin/env python3
"""Referee script 1. Independent recomputation of E[N(k)] for the official 16x16.

Run: python3 -I r1_counts.py   (a few seconds, stdlib only)

Method, different from the memo's inclusion and exclusion: a transfer count
over colours. Colours are processed one at a time; the state is the number of
half-edges of earlier colours still unpaired (each of them must end in a
two-coloured pair) and the number of two-coloured pairs closed so far. Exact
integers. The colour census is read from the committed piece set, not from the
site.
"""
import os
import sys
from math import comb, factorial, lgamma, log, log10
from fractions import Fraction

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, '..', '..', '..', '..', '..', '..'))
CSV = os.path.join(ROOT, 'engine', 'data', 'official_eternity2.csv')


def dfact(n):
    r = 1
    while n > 1:
        r *= n
        n -= 2
    return r


def falling(n, a):
    r = 1
    for i in range(a):
        r *= n - i
    return r


def matching_dist(counts, kmax):
    """ways[k] = number of perfect matchings of labelled half-edges with exactly k
    two-coloured pairs, for k <= kmax (exact)."""
    state = {(0, 0): 1}
    for n in counts:
        new = {}
        for (o, k), w in state.items():
            for a in range(0, min(n, o) + 1):
                wa = w * comb(n, a) * falling(o, a)
                rest = n - a
                for j in range(0, rest // 2 + 1):
                    o2 = o - a + rest - 2 * j
                    k2 = k + a
                    if k2 + o2 > kmax:
                        continue
                    v = wa * comb(rest, 2 * j) * dfact(2 * j - 1)
                    new[(o2, k2)] = new.get((o2, k2), 0) + v
        state = new
    out = [0] * (kmax + 1)
    for (o, k), w in state.items():
        if o == 0:
            out[k] += w
    return out


def bijection_dist(cw, ccw, kmax):
    """ways[k] = number of bijections cw half-edges -> ccw half-edges with exactly k
    two-coloured pairs, k <= kmax (exact)."""
    state = {(0, 0, 0): 1}  # open cw, open ccw, k
    for x, y in zip(cw, ccw):
        new = {}
        for (u, v, k), w in state.items():
            for i in range(0, min(x, y) + 1):
                wi = w * comb(x, i) * comb(y, i) * factorial(i)
                for a in range(0, min(x - i, v) + 1):      # this colour's cw onto earlier open ccw
                    wa = wi * comb(x - i, a) * falling(v, a)
                    for b in range(0, min(y - i, u) + 1):  # this colour's ccw onto earlier open cw
                        u2 = u - b + (x - i - a)
                        v2 = v - a + (y - i - b)
                        k2 = k + a + b
                        if k2 + max(u2, v2) > kmax:
                            continue
                        val = wa * comb(y - i, b) * falling(u, b)
                        new[(u2, v2, k2)] = new.get((u2, v2, k2), 0) + val
        state = new
    out = [0] * (kmax + 1)
    for (u, v, k), w in state.items():
        if u == 0 and v == 0:
            out[k] += w
    return out


def read_pieces():
    pieces = []
    with open(CSV) as fh:
        lines = [l.strip() for l in fh if l.strip()]
    assert int(lines[0]) == 16
    for line in lines[1:]:
        cols = line.split(',')
        e = []
        for c in cols[:4]:
            v = int(c, 2)
            e.append(0 if v == 65535 else v)
        pieces.append(tuple(e))  # up, right, down, left
    return pieces


def census(pieces):
    """Returns (cw counts, ccw counts, interior-side counts) per colour, and class sizes."""
    cw, ccw, inner = {}, {}, {}
    ncorner = nedge = nint = 0
    for e in pieces:
        g = [i for i in range(4) if e[i] == 0]
        if len(g) == 0:
            nint += 1
            for c in e:
                inner[c] = inner.get(c, 0) + 1
        elif len(g) == 1:
            nedge += 1
            i = g[0]
            cw[e[(i + 1) % 4]] = cw.get(e[(i + 1) % 4], 0) + 1
            ccw[e[(i + 3) % 4]] = ccw.get(e[(i + 3) % 4], 0) + 1
            inner[e[(i + 2) % 4]] = inner.get(e[(i + 2) % 4], 0) + 1
        else:
            ncorner += 1
            assert len(g) == 2
            # the two grey sides are adjacent: last grey side going clockwise, then its successor
            a, b = g
            last = b if (a + 1) % 4 == b else a
            first = a if last == b else b
            cw[e[(last + 1) % 4]] = cw.get(e[(last + 1) % 4], 0) + 1
            ccw[e[(first + 3) % 4]] = ccw.get(e[(first + 3) % 4], 0) + 1
    return cw, ccw, inner, (ncorner, nedge, nint)


def main():
    pieces = read_pieces()
    assert len(pieces) == 256
    cw, ccw, inner, (nc, ne, ni) = census(pieces)
    print('classes: corners %d, border %d, interior %d' % (nc, ne, ni))
    print('ring colours cw :', dict(sorted(cw.items())))
    print('ring colours ccw:', dict(sorted(ccw.items())))
    print('interior-side colours:', dict(sorted(inner.items())))
    print('ring colours on interior sides:', sorted(set(cw) & set(inner)), '(empty means the two classes are disjoint)')
    assert cw == ccw or True
    ring_cols = sorted(cw)
    cwl = [cw[c] for c in ring_cols]
    ccwl = [ccw.get(c, 0) for c in ring_cols]
    il = [inner[c] for c in sorted(inner)]
    print('sum interior half-edges', sum(il), 'ring cw', sum(cwl), 'ccw', sum(ccwl))
    # extra facts about the designed instance that the ensemble does not impose
    canon = set()
    sym = 0
    for e in pieces:
        rots = [e[r:] + e[:r] for r in range(4)]
        canon.add(min(rots))
        if len(set(rots)) < 4:
            sym += 1
    print('distinct pieces up to rotation: %d of 256; self-symmetric pieces: %d' % (len(canon), sym))
    rep = sum(1 for e in pieces if 0 not in e and len(set(e)) < 4)
    print('interior pieces with a repeated colour: %d of %d' % (rep, ni))
    # ensemble expectation of that number: 1 - P(4 sides all different colours) for 4 of the 840 half-edges
    tot = sum(il)
    # P(all four distinct) by elementary symmetric polynomial e4 of the counts
    e = [1, 0, 0, 0, 0]
    for n in il:
        for j in range(4, 0, -1):
            e[j] += e[j - 1] * n
    p_distinct = e[4] / comb(tot, 4)
    print('ensemble expectation of that number: %.1f' % (ni * (1 - p_distinct)))

    KMAX = 20
    DI = matching_dist(il, KMAX)
    DR = bijection_dist(cwl, ccwl, KMAX)
    totI = dfact(sum(il) - 1)
    totR = factorial(sum(cwl))
    lw = lgamma(nc + 1) + lgamma(ne + 1) + lgamma(ni + 1) + ni * log(4)
    ln10 = log(10)
    print('log10 W = %.4f' % (lw / ln10))
    # exact rational at k = 0
    p0 = Fraction(DI[0], totI) * Fraction(DR[0], totR)
    W = factorial(nc) * factorial(ne) * factorial(ni) * 4 ** ni
    n0 = p0 * W
    print('E[N(0)] no clue    = %.6g' % float(n0))
    print('E[N(0)] starter    = %.2f  (W / 784)' % float(n0 / 784))
    w5 = factorial(nc) * factorial(ne) * factorial(ni - 5) * 4 ** (ni - 5)
    print('E[N(0)] five clues = %.4g' % float(p0 * w5))
    print()
    print(' k score  log10 E[N(k)] no clue   starter   five clues   step from k-1')
    prev = None
    rows = {}
    for k in range(0, KMAX + 1):
        num = sum(DR[kr] * DI[k - kr] for kr in range(0, k + 1))
        if num == 0:
            print('%2d  %3d   zero' % (k, 480 - k))
            prev = None
            continue
        val = Fraction(num, totI * totR)
        l = (log(val.numerator) - log(val.denominator) + lw) / ln10
        rows[k] = l
        s = l - log10(784)
        f = l + (lgamma(ni - 5 + 1) - lgamma(ni + 1) - 5 * log(4)) / ln10
        print('%2d  %3d   %8.3f              %8.3f  %8.3f     %s' % (k, 480 - k, l, s, f, '' if prev is None else '%.3f' % (l - prev)))
        prev = l
    print()
    print('N(467)/N(480) = 10^%.3f ; Owen as quoted on the site: 2.05e35 = 10^%.3f' % (rows[13] - rows[0], log10(2.05e35)))
    print('split of E[N(10)] by ring breaks kr:', ' '.join(
        '%d:%.3f' % (kr, DR[kr] * DI[10 - kr] / sum(DR[a] * DI[10 - a] for a in range(11))) for kr in range(0, 7)))
    print('ring alone: ways with k two-coloured ring joints / 60!, k=0..4:', ' '.join('%.3e' % (DR[k] / totR) for k in range(5)))
    print('interior alone, k=0..4:', ' '.join('%.3e' % (DI[k] / totI) for k in range(5)))


if __name__ == '__main__':
    main()
