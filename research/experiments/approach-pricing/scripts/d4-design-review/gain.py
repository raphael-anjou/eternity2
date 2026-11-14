"""Design review D4, item 6: gain in bits of one prune on a Gaussian statistic.
Model: per presentation z = sqrt(rho) g + sqrt(1-rho) e, unit variance in both
classes, true class shifted by delta. Prune at one depth d: keep z > t.
Work of one unpruned run = A + B (A = levels <= d, B = levels > d), relative
cost of running K presentations = C (1 for the best one). Success = at least
one of K true prefixes kept. Gain = log2( P (A+B) / (C (A + Q(t) B)) ),
maximised over t. Level profile: log-linear interpolation of the calibration
means of log10 width (crude; the study must redo it with every level).
stdlib only."""
import math
Q = lambda x: 0.5 * math.erfc(x / math.sqrt(2))
def psucc(t, delta, rho, K, n=801):
    if K == 1 or rho == 0: return 1 - (1 - Q(t - delta)) ** K
    s, tot = 0.0, 0.0
    for i in range(n):
        g = -8 + 16 * i / (n - 1); w = math.exp(-g * g / 2)
        q = Q((t - delta - math.sqrt(rho) * g) / math.sqrt(1 - rho))
        s += w * (1 - (1 - q) ** K); tot += w
    return s / tot
prof = {0: 0, 32: 17.99, 64: 26.69, 96: 34.90, 128: 38.27, 144: 38.87, 160: 38.98, 176: 37.75, 192: 34.91, 251: 0.0}
ks = sorted(prof)
def lw(k):
    for a, b in zip(ks, ks[1:]):
        if a <= k <= b: return prof[a] + (prof[b] - prof[a]) * (k - a) / (b - a)
W = [10 ** lw(k) for k in range(252)]
tot = sum(W)
print("log10 total (interpolated profile) %.2f" % math.log10(tot))
def best(delta, rho, K, C, share_below):
    bestg = (-9, 0, 0)
    for i in range(-200, 601):
        t = i / 100
        p = psucc(t, delta, rho, K)
        g = math.log2(p / (C * ((1 - share_below) + Q(t) * share_below)))
        if g > bestg[0]: bestg = (g, t, p)
    return bestg
print("\nshare of the work strictly below depth d, and the cap log2(total/A):")
sh = {}
for d in (96, 112, 128, 144, 160, 176):
    A = sum(W[: d + 1]); sh[d] = 1 - A / tot
    print("  d=%d share below %.3f cap %.2f bits" % (d, sh[d], math.log2(tot / A)))
print("\nuncapped (all work below the prune), success fixed at 0.5, K=1: bits = log2(0.5/Q(delta))")
for dl in (1.1, 1.4, 1.57, 2.0, 2.1): print("  delta %.2f: %.2f bits" % (dl, math.log2(0.5 / Q(dl))))
print("\nbest threshold, one prune at depth d, delta from the calibration table (node / work):")
cal = {96: (1.10, None), 128: (1.43, 1.34), 144: (1.57, 1.40), 160: (2.00, 1.85), 176: (2.12, 2.05)}
for d, (dn, dw) in cal.items():
    for lab, dl in (("node", dn), ("work", dw)):
        if dl is None: continue
        for K, C, name in ((1, 1.0, "K=1 best dir"), (2, 2.02, "K=2 two cheap dirs"), (4, 2 ** 4.19, "K=4 all dirs")):
            for rho in ((0.0,) if K == 1 else (0.0, 0.23, 0.6)):
                g, t, p = best(dl, rho, K, C, sh[d])
                gu, tu, pu = best(dl, rho, K, C, 1.0)
                print("  d=%3d %s delta %.2f %-18s rho %.2f: capped %+.2f bits (t=%.2f, success %.2f) | if all work were below: %+.2f (success %.2f)" % (d, lab, dl, name, rho, g, t, p, gu, pu))

print("\nONE FIXED INSTANCE: total success probability constrained to >= 0.5 (and the 0.8 line), capped:")
def cons(delta, rho, K, C, share, pmin):
    b = None
    for i in range(-300, 601):
        t = i / 100
        p = psucc(t, delta, rho, K)
        if p < pmin: break
        g = math.log2(p / (C * ((1 - share) + Q(t) * share)))
        if b is None or g > b[0]: b = (g, t, p, Q(t))
    return b
for d, (dn, dw) in cal.items():
    dl = dw if dw is not None else dn
    for K, C, name, rho in ((1, 1.0, "K=1", 0.0), (2, 2.02, "K=2", 0.23), (4, 2 ** 4.19, "K=4", 0.23), (4, 2 ** 4.19, "K=4", 0.0)):
        for pmin in (0.5, 0.8):
            g, t, p, f = cons(dl, rho, K, C, sh[d], pmin)
            print("  d=%3d delta %.2f %s rho %.2f success>=%.1f: %+.2f bits (keep %.3f per run, success %.2f)" % (d, dl, name, rho, pmin, g, f, p))
