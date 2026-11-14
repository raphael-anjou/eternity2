"""Annealed score landscape: the page's independent-Bernoulli model against the census-exact pairing model
(uniform random pairing of the 840 interior half-edges, uniform random bijection of the 60 ring outs to ins)."""
import math, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from fractions import Fraction
from math import comb, factorial
L10 = math.log(10)
logW = (math.lgamma(5) + math.lgamma(57) + math.lgamma(197) + 196 * math.log(4)) / L10
# --- Bernoulli model (the-470-wall.mdx)
pi_ = (5 * 48 ** 2 + 12 * 50 ** 2) / 840 ** 2
dist = [1.0]
for p, k in ((0.2, 60), (pi_, 420)):
    for _ in range(k):
        new = [0.0] * (len(dist) + 1)
        for s, v in enumerate(dist):
            new[s] += v * (1 - p); new[s + 1] += v * p
        dist = new
# top of the distribution underflows in floats, redo exactly with Fractions for the top rows
def exact_bern():
    from fractions import Fraction as F
    d = [F(1)]
    for p, k in ((F(1, 5), 60), (F(5 * 48 ** 2 + 12 * 50 ** 2, 840 ** 2), 420)):
        for _ in range(k):
            new = [F(0)] * (len(d) + 1)
            for s, v in enumerate(d):
                new[s] += v * (1 - p); new[s + 1] += v * p
            d = new
    return d
def lg(fr): return (math.log(fr.numerator) - math.log(fr.denominator)) / L10
bern = exact_bern()
mean = sum(s * float(v) for s, v in enumerate(bern))
print("Bernoulli model: mean", round(mean, 3), " log10 W_geom", round(logW, 4))
# --- census-exact pairing model
def df(m): return 1 if m <= 0 else math.prod(range(m, 0, -2))
counts = [48] * 5 + [50] * 12
poly = [1]                                   # coefficient k: number of ways to choose k disjoint monochromatic pairs
for nc in counts:
    f = [comb(nc, 2 * k) * df(2 * k - 1) for k in range(nc // 2 + 1)]
    new = [0] * (len(poly) + len(f) - 1)
    for i, a in enumerate(poly):
        for j, b in enumerate(f): new[i + j] += a * b
    poly = new
tot = df(839)
fact_mom = [Fraction(poly[k] * df(840 - 2 * k - 1), tot) for k in range(421)]      # E[C(X,k)]
def pmf_from_factorial_moments(fm, top, depth):
    out = {}
    for j in range(top, top - depth - 1, -1):
        out[j] = sum((-1) ** (k - j) * comb(k, j) * fm[k] for k in range(j, top + 1))
    return out
pint = pmf_from_factorial_moments(fact_mom, 420, 40)
rp = [1]                                     # ring: choose k matched positions, colour by colour: C(12,k)^2 k!
for _ in range(5):
    f = [comb(12, k) ** 2 * factorial(k) for k in range(13)]
    new = [0] * (len(rp) + len(f) - 1)
    for i, a in enumerate(rp):
        for j, b in enumerate(f): new[i + j] += a * b
    rp = new
fm_ring = [Fraction(rp[k] * factorial(60 - k), factorial(60)) for k in range(61)]
pring = pmf_from_factorial_moments(fm_ring, 60, 40)
assert all(v >= 0 for v in pint.values()) and all(v >= 0 for v in pring.values())
print("check: P(all 420 interior pairs match) * P(ring) * W_geom = 10^", round(lg(pint[420]) + lg(pring[60]) + logW, 4), "(first moment without starter, 10^7.0617)")
print("score | log10 count, Bernoulli (page) | log10 count, census-exact pairing | ratio (decades)")
for s in (480, 479, 478, 477, 476, 475, 472, 470, 465, 460):
    d = 480 - s
    tot_p = sum(pint[420 - a] * pring[60 - (d - a)] for a in range(d + 1))
    b = lg(bern[s]) + logW
    c = lg(tot_p) + logW if tot_p > 0 else float("-inf")
    print(f"  {s} | {b:8.2f} | {c:8.2f} | {c - b:+.2f}")
print("P(exactly one interior mismatch) in the pairing model:", pint[419], " ring:", pring[59], "(the parity theorem, recovered)")
