"""Shared helpers for the gap 6b review scripts (stdlib only)."""
from fractions import Fraction
from math import lgamma, log, log2, factorial, pi
LN2 = log(2)
lf = lambda n: lgamma(n + 1) / LN2
def lfall(n, k): return lf(n) - lf(n - k)
def l2(fr): return log2(fr.numerator) - log2(fr.denominator) if fr > 0 else float("-inf")

def load(p):
    rows = [l.strip().split(",") for l in open(p) if "," in l]
    return [tuple(int(x, 2) for x in r[:4]) for r in rows], [tuple(int(x) for x in r[4:7]) for r in rows]

def match_table(counts, emax):
    """log2 P(e slot pairs all monochromatic) when the colour multiset is dealt without replacement."""
    N = sum(counts); poly = [Fraction(1)]
    for n in counts:
        g = []; ff = 1
        for k in range(0, n // 2 + 1):
            if k > 0: ff *= (n - 2 * k + 2) * (n - 2 * k + 1)
            g.append(Fraction(ff, factorial(k)))
        new = [Fraction(0)] * min(emax + 1, len(poly) + len(g) - 1)
        for i, a in enumerate(poly):
            for j, b in enumerate(g):
                if i + j < len(new): new[i + j] += a * b
        poly = new
    out = []; den = 1
    for e in range(0, emax + 1):
        if e > 0: den *= (N - 2 * e + 2) * (N - 2 * e + 1)
        out.append(l2(poly[e] * factorial(e) / den) if e < len(poly) and poly[e] > 0 else float("-inf"))
    return out

INT = [50] * 12 + [48] * 5
FRM = [24] * 5
def tables():
    return (match_table(INT, 420), match_table(FRM, 60),
            [-e * log2(17) for e in range(421)], [-e * log2(5) for e in range(61)])

def kind(r, c): return (r in (0, 15)) + (c in (0, 15))
CLUES = {(2, 2), (2, 13), (13, 2), (13, 13), (7, 8)}   # centre clue at row 7, column 8 (I8), which puts the model peak at depth 161

def lsum(xs):
    m = max(xs)
    return m + log2(sum(2 ** (x - m) for x in xs))

def logdet(M):
    """log2 det of a symmetric positive definite matrix (plain elimination)."""
    n = len(M); A = [row[:] for row in M]; s = 0.0
    for i in range(n):
        p = max(range(i, n), key=lambda r: abs(A[r][i]))
        if abs(A[p][i]) < 1e-12: return float("-inf")
        if p != i: A[i], A[p] = A[p], A[i]
        s += log2(abs(A[i][i]))
        for r in range(i + 1, n):
            f = A[r][i] / A[i][i]
            if f:
                for c in range(i, n): A[r][c] -= f * A[i][c]
    return s
