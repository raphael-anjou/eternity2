"""Critical colour counts, bulge peak position, asymptotic checks, value-order spread.

Stdlib only. Run: python3 -I regime.py
"""
import math, os, random, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from model import Profile, critical_c, owen


def g(x):
    return x - 1.0 - math.log(x)


def interior_peak(p):
    """Depth of the maximum of ln A, as the fraction of interior cells placed there."""
    d, lnA = p.peak()
    placed = sum(1 for (cls, r, rho, a, f) in p.st[:d] if cls == "interior")
    return d, lnA, placed / float((p.n - 2) ** 2)


print("A. whole-board first moment and the bulge at the critical colour count (row-major, naive q)")
print("n\tOwen\tc_crit\tpeak d/N\tinterior frac\tlnA*\tlnA*/P\tP/e\tln(frames)\tcontinuum 1-1/e")
for n in (6, 7, 8, 9, 10, 12, 16, 24, 32, 48, 64):
    cc = critical_c(n)
    p = Profile(n, cc, "rowmajor", deplete=False)
    d, lnA, fr = interior_peak(p)
    w = n - 2
    lnF = math.log(24) + math.lgamma(4 * w + 1) - 4 * (n - 1) * math.log(5)
    print("%d\t%.2f\t%.2f\t%.3f\t%.3f\t%.1f\t%.3f\t%.1f\t%.1f\t%.3f" % (n, owen(n), cc, d / p.N, fr, lnA, lnA / w / w, w * w / math.e, lnF, 1 - 1 / math.e))

print()
print("B. rungs: x = c^2/4P, eps = 1/x (Edmonds et al.), continuum rate g(x) per interior cell, exact lnA*, lnS, colours above c_crit")
print("n\tc\tx\teps\tP*g(x)\tlnA* rowmajor\tlnS\tc - c_crit\tinterior frac at peak\t1-x")
for n, c in ((6, 5), (6, 6), (6, 7), (7, 6), (7, 7), (7, 8), (8, 7), (8, 8), (8, 9), (8, 10), (9, 11), (10, 12), (16, 17)):
    P = (n - 2) ** 2
    x = c * c / (4.0 * P)
    p = Profile(n, c, "rowmajor", deplete=False)
    d, lnA, fr = interior_peak(p)
    print("%d\t%d\t%.3f\t%.2f\t%.1f\t%.1f\t%.1f\t%+.2f\t%.2f\t%.2f" % (n, c, x, 1 / x, P * g(x) if x < 1 else 0.0, lnA, p.lnA[-1], c - critical_c(n), fr, max(0, 1 - x)))

print()
print("C. 16x16, 17 interior + 5 frame colours, generator ensemble (uniform colours), row-major")
p = Profile(16, 17, "rowmajor")
d, lnA, fr = interior_peak(p)
print("lnS = %.2f (S = %.3g, all four turns counted); peak at depth %d of 256 (interior fraction %.3f); lnA* = %.1f (log10 %.1f)" % (
    p.lnA[-1], math.exp(p.lnA[-1]), d, fr, lnA, lnA / math.log(10)))
print("model first-solution mean, plain row-major DFS: 10^%.1f nodes; full planted tree 10^%.1f" % (
    math.log10(p.first_solution_mean()), math.log10(p.full_tree())))
for c in (15, 16, 17, 18, 19, 20):
    q = Profile(16, c, "rowmajor")
    print("  c=%d lnS=%.1f lnA*=%.1f first-solution mean 10^%.1f" % (c, q.lnA[-1], q.peak()[1], math.log10(q.first_solution_mean())))

print()
print("D. value-order spread in the model: 8x8 c=10 row-major profile, 30 instances, two independent value orders")
rng = random.Random(3)
q = Profile(8, 10, "rowmajor")
s = q.first_solution_sample(rng, 6000)
print("single run over value orders: q10 %.3g q25 %.3g median %.3g q75 %.3g q90 %.3g (q75/q25 = %.2f)" % (
    s[600], s[1500], s[3000], s[4500], s[5400], s[4500] / s[1500]))
pool = list(s)
meds = []
for _ in range(2000):
    r = sorted(math.log(rng.choice(pool) / rng.choice(pool)) for _ in range(30))
    meds.append(math.exp(r[15]))
meds.sort()
print("median of 30 paired ratios between two random orders: 2.5%% %.2f, 25%% %.2f, 75%% %.2f, 97.5%% %.2f" % (
    meds[50], meds[500], meds[1500], meds[1950]))
