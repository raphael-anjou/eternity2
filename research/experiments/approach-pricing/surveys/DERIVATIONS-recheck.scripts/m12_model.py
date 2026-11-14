"""First-moment (ensemble) model of partial-fill counts for framed edge-matching boards.

Ensemble: the multiset of colours on half-edges is fixed (taken from the piece
set), their assignment to half-edge slots is uniformly random. This is the
ensemble behind the canonical 14,702 figure
(web/content/research/build/analysis/solution-counting.mdx, lines 284-296).

P_int(e): probability that e given disjoint pairs of interior-colour half-edge
slots are all monochromatic.
P_frame(f): same for f frame edges, each pairing one clockwise-facing with one
counter-clockwise-facing frame half-edge.
"""
from fractions import Fraction
from math import lgamma, log, log10


def _falling(n, k):
    r = 1
    for i in range(k):
        r *= n - i
    return r


def log10_fact(n):
    return lgamma(n + 1) / log(10)


def log10_falling(n, k):
    if k > n:
        return float("-inf")
    return (lgamma(n + 1) - lgamma(n - k + 1)) / log(10)


def interior_match_logp(counts, emax):
    """log10 P_int(e) for e = 0..emax. counts: half-edge count per colour."""
    H = sum(counts)
    poly = [Fraction(1)]
    for n in counts:
        g = []
        f = 1
        for k in range(n // 2 + 1):
            if k > 0:
                f *= k
            g.append(Fraction(_falling(n, 2 * k), f))
        new = [Fraction(0)] * min(len(poly) + len(g) - 1, emax + 1)
        for i, a in enumerate(poly):
            if a == 0:
                continue
            for j, b in enumerate(g):
                if i + j > emax:
                    break
                new[i + j] += a * b
        poly = new
    out = []
    fact = 1
    for e in range(emax + 1):
        if e > 0:
            fact *= e
        coef = poly[e] if e < len(poly) else Fraction(0)
        val = coef * fact / _falling(H, 2 * e) if 2 * e <= H else Fraction(0)
        out.append(_log10_frac(val))
    return out


def frame_match_logp(counts, fmax):
    """counts: number of frame edges per frame colour (cw count == ccw count)."""
    M = sum(counts)
    poly = [Fraction(1)]
    for m in counts:
        g = []
        f = 1
        for k in range(m + 1):
            if k > 0:
                f *= k
            g.append(Fraction(_falling(m, k) ** 2, f))
        new = [Fraction(0)] * min(len(poly) + len(g) - 1, fmax + 1)
        for i, a in enumerate(poly):
            if a == 0:
                continue
            for j, b in enumerate(g):
                if i + j > fmax:
                    break
                new[i + j] += a * b
        poly = new
    out = []
    fact = 1
    for e in range(fmax + 1):
        if e > 0:
            fact *= e
        coef = poly[e] if e < len(poly) else Fraction(0)
        val = coef * fact / (_falling(M, e) ** 2) if e <= M else Fraction(0)
        out.append(_log10_frac(val))
    return out


def _log10_frac(fr):
    if fr <= 0:
        return float("-inf")
    n, d = fr.numerator, fr.denominator
    return _log10_int(n) - _log10_int(d)


def _log10_int(n):
    b = n.bit_length()
    if b <= 1000:
        return log10(n)
    s = b - 900
    return log10(n >> s) + s * log10(2)


def cell_kind(n, r, c):
    rim = (r == 0) + (r == n - 1) + (c == 0) + (c == n - 1)
    return ("interior", "edge", "corner")[rim]


def edge_kind(n, r1, c1, r2, c2):
    """Kind of the internal edge between two adjacent cells: 'frame' or 'int'."""
    if r1 == r2 and (r1 == 0 or r1 == n - 1):
        return "frame"
    if c1 == c2 and (c1 == 0 or c1 == n - 1):
        return "frame"
    return "int"


def profile(n, order, pools, logp_int, logp_frame, fixed=()):
    """log10 N(d) after each cell of `order` is filled.

    pools: dict kind -> number of free pieces of that kind (fixed pieces excluded).
    fixed: set of cells holding a pinned piece (one choice, one rotation).
    Returns list of (log10 N, e_int, e_frame).
    """
    left = dict(pools)
    placed = set()
    la = 0.0
    ei = ef = 0
    out = []
    for (r, c) in order:
        k = cell_kind(n, r, c)
        if (r, c) not in fixed:
            if left[k] <= 0:
                la = float("-inf")
            else:
                la += log10(left[k]) + (log10(4) if k == "interior" else 0.0)
                left[k] -= 1
        for (rr, cc) in ((r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)):
            if (rr, cc) in placed:
                if edge_kind(n, r, c, rr, cc) == "frame":
                    ef += 1
                else:
                    ei += 1
        placed.add((r, c))
        out.append((la + logp_int[ei] + logp_frame[ef], ei, ef))
    return out


def logsum10(xs):
    m = max(xs)
    if m == float("-inf"):
        return m
    return m + log10(sum(10 ** (x - m) for x in xs))
