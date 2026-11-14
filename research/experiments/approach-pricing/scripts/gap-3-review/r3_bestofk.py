"""Gap 3 review, item 1: the memo's model, and the best of K orders.

  uv run --with numpy python r3_bestofk.py

A. The memo's supermartingale model: every level-k subtree has the same size, the planted
   node has 1 + Poisson(lambda) children, M_k = prod_j lambda / (1 + n_j), gain = log2 M - 1.
   Ville gives P(sup M >= a) <= 1/a. The model's own tail is thinner: a product of iid
   factors with E[X] < 1 has P(sup >= a) of order a^-theta with E[X^theta] = 1.
   Printed: theta for several lambda, and the simulated tail of the best gain.
B. K independent orders, each with best gain G, P(G >= g) = min(1, c 2^-g), gains capped at
   gmax bits. c = 1/2 is the memo's bound, c = 1 is what the 24 cases look like.
     best member  : E[max G], and E[max G] - log2 K (a portfolio that counted only its best
                    member and paid K for it)
     portfolio    : log2 of mean over members of 2^G: the true net gain of a blind uniform
                    portfolio whose members each run at their own best cutoff (unknowable
                    without the solution; with blind cutoffs the mean of 2^gain is about 1/2)
"""
import math

import numpy as np

rng = np.random.default_rng(3)


def theta(lam):
    ns = np.arange(0, 80)
    logp = -lam + ns * math.log(lam) - np.array([math.lgamma(n + 1) for n in ns])
    p = np.exp(logp)
    f = lambda th: float((p * (lam / (1.0 + ns)) ** th).sum()) - 1.0
    lo, hi = 1.0001, 50.0
    if f(hi) < 0:
        return float("inf")
    for _ in range(80):
        mid = (lo + hi) / 2
        if f(mid) < 0:
            lo = mid
        else:
            hi = mid
    return lo


print("A. the memo's model")
for lam in (1.2, 1.5, 2.0, 2.06, 3.0, 5.0):
    drift = math.log2(lam) - float(sum(math.exp(-lam + n * math.log(lam) - math.lgamma(n + 1)) * math.log2(1 + n) for n in range(80)))
    print("   lambda %.2f: E[lambda/(1+n)] = %.3f, mean step %.2f bits, tail exponent theta = %.2f (Ville uses 1)" % (lam, 1 - math.exp(-lam), drift, theta(lam)))
R, L = 400000, 120
for lam in (2.0,):
    n = rng.poisson(lam, size=(R, L))
    steps = np.log2(lam / (1.0 + n))
    best = np.maximum(np.cumsum(steps, axis=1).max(axis=1), 0.0) - 1.0      # sup over cutoff depths, started at M = 1
    print("   lambda %.1f, %d levels, %d paths: P(best gain >= g) for g = 0..5: %s ; Ville bound 2^-(g+1): %s"
          % (lam, L, R, " ".join("%.4f" % float((best >= g).mean()) for g in range(6)), " ".join("%.4f" % 2.0 ** -(g + 1) for g in range(6))))

print("\nB. best of K independent orders, tail c 2^-g capped at gmax")
for c, gmax in ((0.5, 126.0), (1.0, 126.0), (1.0, 20.0)):
    print("   c = %.1f, cap %.0f bits" % (c, gmax))
    for K in (1, 4, 8, 64, 1024, 2 ** 16, 2 ** 20):
        reps = max(20, min(4000, (1 << 23) // K))
        mx, pf = [], []
        for _ in range(reps):
            g = np.minimum(np.log2(c / rng.random(K)), gmax)
            mx.append(float(g.max()))
            pf.append(math.log2(float(np.mean(2.0 ** g))))
        mx.sort(); pf.sort()
        print("     K = %7d: best member mean %.2f, minus log2 K %.2f | portfolio net: mean %.2f, median %.2f, 90th percentile %.2f, P(net >= 10 bits) %.4f"
              % (K, sum(mx) / reps, sum(mx) / reps - math.log2(K), sum(pf) / reps, pf[reps // 2], pf[int(0.9 * reps)], sum(1 for x in pf if x >= 10) / reps))
    print("     ceiling of the expected mean of 2^G under this tail: 1 + c * ln(2) * (gmax - log2 c) -> log2 = %.2f bits"
          % math.log2(c + c * math.log(2) * (gmax - math.log2(c))))
