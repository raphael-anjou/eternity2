#!/usr/bin/env python3
"""Referee script 6. Strict row-scan tree of the 16x16, own formula, exact rationals.
Run: python3 -I r6_strict_tree.py (seconds, stdlib only)

P(e given interior joints all match) = e! A_e / (C(840, 2e) (2e-1)!! e!) with A_e the number
of sets of e disjoint one-coloured pairs; P(e given ring joints all match) = e! r_e / ((60)_e)^2
with r_e the number of sets of e disjoint one-coloured (cw, ccw) pairs. Community figure
for scan-row: 9.2766e42 nodes per solution (site, solution-counting.mdx, msg 8924).
"""
from fractions import Fraction
from math import comb, factorial, log10

INT = [48] * 5 + [50] * 12
RING = [12] * 5


def pmul(a, b, cap):
    out = [0] * min(cap + 1, len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            if i + j <= cap:
                out[i + j] += x * y
    return out


def dfact(n):
    r = 1
    while n > 1:
        r *= n
        n -= 2
    return r


A = [1]
for n in INT:
    A = pmul(A, [factorial(n) // (factorial(j) * factorial(n - 2 * j) * 2 ** j) for j in range(n // 2 + 1)], 420)
R = [1]
for n in RING:
    R = pmul(R, [comb(n, i) ** 2 * factorial(i) for i in range(n + 1)], 60)


def falling(n, a):
    r = 1
    for i in range(a):
        r *= n - i
    return r


def p_int(e):
    return Fraction(A[e], comb(840, 2 * e) * dfact(2 * e - 1))


def p_ring(e):
    return Fraction(factorial(e) * R[e], falling(60, e) ** 2)


n = 16
left = {'c': 4, 'e': 56, 'i': 196}
Acount, er, ei = 1, 0, 0
tot = Fraction(0)
peak = (0, 0)
last = None
for d in range(1, 257):
    x, y = (d - 1) % n, (d - 1) // n
    bx, by = x in (0, n - 1), y in (0, n - 1)
    cls = 'c' if bx and by else ('e' if bx or by else 'i')
    Acount *= left[cls] * (4 if cls == 'i' else 1)
    left[cls] -= 1
    if x > 0:
        er, ei = (er + 1, ei) if by else (er, ei + 1)
    if y > 0:
        er, ei = (er + 1, ei) if bx else (er, ei + 1)
    t = Acount * p_int(ei) * p_ring(er)
    tot += t
    if t > peak[1]:
        peak = (d, t)
    last = t
print('joints closed at depth 256: ring %d, interior %d' % (er, ei))
print('perfect boards (no clue): %.6g' % float(last))
print('strict tree: total nodes 10^%.3f, peak at depth %d (10^%.2f)' % (log10(float(tot)), peak[0], log10(float(peak[1]))))
print('nodes per perfect board: %.5g  (community: 9.2766e42)' % float(tot / last))
