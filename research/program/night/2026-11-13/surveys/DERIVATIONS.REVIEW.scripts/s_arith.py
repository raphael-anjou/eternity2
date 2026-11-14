"""Pure arithmetic rechecks (items 3, 5, 6). Run: python3 -I s_arith.py"""
from math import lgamma, log, log2, comb, factorial, sqrt, erfc, exp
from statistics import NormalDist
L2 = lambda x: log2(x)
def lf2(n): return lgamma(n + 1) / log(2)
print("== item 3: exact-algorithms prices ==")
row = 2 * L2(5) + 14 * L2(17)
print("row cut 5^2*17^14 = 2^%.2f ; with in-row leg 2^%.2f" % (row, row + L2(17)))
# contraction time: per cell, every frontier state is extended by the tiles fitting (N,W): 784/289 on average
ops = 0.0
for r in range(16):
    for c in range(16):
        ops += 2 ** (row + L2(17)) * (784 / 289)
print("contraction time, 256 cells x 2^65.96 states x 2.71 fits = 2^%.1f = 10^%.2f  (path 1: 10^22.7 = 2^75.4)" % (L2(ops), L2(ops) * log(2) / log(10)))
for name, v in (("full sieve 251+75.4", 251 + 75.4), ("corners enumerated", L2(24) + 247 + 75.4), ("half sieve", 125.5 + 75.4),
                ("narrow sieve 1.66", 251 * L2(1.66) + 75.4), ("subset DP worst 256+row", 256 + row), ("colour coding 251+row", 251 + row),
                ("subset convolution row+16+256", row + 16 + 256)):
    print("  %-32s %.1f bits" % (name, v))
print("  break-even sieve terms: 134.4-75.4 = %.1f ; 134.4-61.87 = %.1f ; per free tile %.3f to %.3f" % (134.4 - 75.4, 134.4 - row, 59 / 251, (134.4 - row) / 251))
print("  134.4/256 = %.3f bit per cell, 134.4/251 = %.3f per free tile, base %.3f" % (134.4 / 256, 134.4 / 251, 2 ** (134.4 / 256)))
print("  C(256,135) = 2^%.1f, C(256,128) = 2^%.1f ; peak 130.1 - row = %.1f" % (L2(comb(256, 135)), L2(comb(256, 128)), 130.1 - row))
print("  2x2 rearrangement 4!*4^4/17^12 = %.2e ; middle-row guess (256)_16*4^16 = 2^%.1f" % (24 * 256 / 17 ** 12, lf2(256) - lf2(240) + 32))
print("  56^j/(56)_j at j=30: %.2f bits" % (30 * L2(56) - (lf2(56) - lf2(26))))
print("\n== item 5: SAT D1 and the arc formula ==")
print("  784^196/17^420 = 2^%.1f ; 196!*4^196/17^420 = 2^%.1f" % (196 * L2(784) - 420 * L2(17), lf2(196) + 392 - 420 * L2(17)))
for m in (2, 3, 6, 12, 13, 20, 30):
    print("  m=%2d  m!/5^(m+1) = 2^%6.1f   (C(m,2)/25 twin swaps expected = %.2f)" % (m, lf2(m) - (m + 1) * L2(5), comb(m, 2) / 25))
print("  30 pieces in 10 arcs: 30!/5^40 = 2^%.1f" % (lf2(30) - 40 * L2(5)))
print("\n== item 6: partition entropies, Wagner, ISD ==")
H2 = L2(comb(4, 2)) + L2(comb(56, 28)) + L2(comb(196, 98))
def multi(n, parts): return lf2(n) - sum(lf2(p) for p in parts)
H4 = multi(4, [2, 0, 0, 2]) + multi(56, [20, 8, 8, 20]) + multi(196, [42, 56, 56, 42])
H8 = multi(4, [2, 0, 0, 0, 0, 0, 0, 2]) + multi(56, [16, 4, 4, 4, 4, 4, 4, 16]) + multi(196, [14, 28, 28, 28, 28, 28, 28, 14])
for k, H in ((2, H2), (4, H4), (8, H8)):
    print("  k=%d  H_k = %.1f bits  list 2^(H/(1+lg k)) = 2^%.1f  solutions needed 2^%.1f" % (k, H, H / (1 + L2(k)), k * H / (1 + L2(k)) - H))  # Wagner: k lists of that size hold 2^(k*l) tuples, 2^-H of them partition
unk = 196 * 784 + 56 * 56 + 16
print("  unknowns %d, checks 992, rate %.4f, weight ratio %.5f" % (unk, 1 - 992 / unk, 256 / unk))
place = 196 * L2(784) + 56 * L2(56) + 4 * L2(4)
print("  Prange blocks: 2^(%.1f - %.1f) = 2^-%.1f ; generic 2^%.0f" % (256 * L2(992 / 256), place, place - 256 * L2(992 / 256), -L2(992 / unk) * 256))
print("  regular vectors 2^%.1f against syndrome space 257^992 = 2^%.1f ; placements 2^%.1f" % (place, 992 * L2(257), lf2(196) + 392 + lf2(56) + lf2(4)))
print("\n== item 6: extreme pruning gain ==")
nd = NormalDist()
Q = lambda t: 0.5 * erfc(t / sqrt(2))
print("  delta   K | exact bits | d*z_K/ln2 + d^2/(2 ln2) + ln-corr | survey's d*sqrt(2lnK)/ln2 | K=1 at 50%% success")
for d in (0.5, 1.4, 3.0):
    for K in (4, 100, 1000, 10 ** 6):
        z = nd.inv_cdf(1 - 1 / K); t = z + d
        g = L2((1 / K) / Q(t))
        approx = (d * z + d * d / 2 + log(1 + d / z)) / log(2)
        print("  %.1f %7d | %5.1f | %5.1f | %5.1f | %4.1f" % (d, K, g, approx, d * sqrt(2 * log(K)) / log(2), L2(0.5 / Q(d))))
# dependence: equicorrelated presentations, statistic of the planted prefix s_i = d + sqrt(rho) z0 + sqrt(1-rho) z_i.
# One instance, K presentations, no further retries: fix the overall success probability at 0.5 and compare costs.
def p_any(t, d, K, rho, n=1201):
    if rho == 0: return 1 - (1 - Q(t - d)) ** K
    if rho == 1: return Q(t - d)
    acc = 0.0
    for i in range(n):
        z0 = -8 + 16 * i / (n - 1)
        w = exp(-z0 * z0 / 2) / sqrt(2 * 3.141592653589793) * 16 / (n - 1)
        pf = 1 - Q((t - d - sqrt(rho) * z0) / sqrt(1 - rho))
        acc += w * (1 - pf ** K)
    return acc
def gain_at(d, K, rho, target=0.5):
    lo, hi = -10.0, 15.0
    for _ in range(50):
        mid = (lo + hi) / 2
        if p_any(mid, d, K, rho) > target: lo = mid
        else: hi = mid
    return L2(1 / (K * Q(lo)))
print("  gain in bits (all K runs paid, overall success 0.5), by correlation rho between presentations")
for d in (1.4, 3.0):
    print("  delta=%.1f   K  | rho=0   0.3   0.6   0.9   1.0" % d)
    for K in (1, 2, 4, 100, 1000):
        print("  %12d | %s" % (K, "  ".join("%5.1f" % gain_at(d, K, rho) for rho in (0, 0.3, 0.6, 0.9, 1.0))))
