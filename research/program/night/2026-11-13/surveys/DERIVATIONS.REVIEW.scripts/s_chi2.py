"""Item 4: planted against null in the survey's own model (colour census fixed, colours exchangeable over half-edge slots).
A. Exact likelihood ratio of one statistic of the piece set: e_c = number of the 56 edge pieces whose inward colour is c.
   Null: 56 of the 840 interior-colour half-edge slots. Planted: 56 of the 420 painted edges.
   chi2 and TV of a statistic are lower bounds for chi2 and TV of the full piece set (data processing).
B. Second statistic: equal-colour side pairs inside interior tiles, null against painted 16x16 boards (Monte Carlo).
C. Corner statistic (exact enumeration).
D. Toy 4x4, census fixed: E_pl[Z] against E_0[Z], with and without the no-duplicate, no-symmetric-tile condition.
Run: python3 -I s_chi2.py <engine/data/official_eternity2.csv>"""
import sys, random, functools
from math import comb, log, lgamma, sqrt, factorial, exp
from collections import Counter, defaultdict
from itertools import product
print = functools.partial(print, flush=True)
rng = random.Random(20261113)

def load(fn):
    rows = [l.strip() for l in open(fn) if l.strip()]
    return [[0 if int(x, 2) == 65535 else int(x, 2) for x in l.split(",")[:4]] for l in rows[1:]]
pieces = load(sys.argv[1])
ic = Counter(); inward = Counter(); interior = []
for p in pieces:
    z = [i for i, v in enumerate(p) if v == 0]
    if len(z) == 0:
        interior.append(p)
        for v in p: ic[v] += 1
    elif len(z) == 1:
        v = p[(z[0] + 2) % 4]; ic[v] += 1; inward[v] += 1
cols = sorted(ic); n = [ic[c] for c in cols]; e_real = [inward[c] for c in cols]
assert sum(n) == 840 and sum(e_real) == 56

print("A. inward-colour statistic of the 56 edge pieces")
lC = lambda a, b: lgamma(a + 1) - lgamma(b + 1) - lgamma(a - b + 1)
c0 = lC(840, 56) - lC(420, 56)
def logL(e): return c0 + sum(lC(nc // 2, ec) - lC(nc, ec) if ec <= nc // 2 else -1e9 for nc, ec in zip(n, e))
pool = [i for i, nc in enumerate(n) for _ in range(nc)]
M = 300000; s1 = s2 = sabs = 0.0; below = 0; lr = logL(e_real); ls = []
for _ in range(M):
    cnt = [0] * 17
    for x in rng.sample(pool, 56): cnt[x] += 1
    l = logL(cnt); L = exp(l); s1 += L; s2 += L * L; sabs += abs(L - 1); below += l < lr
print("   null mean of L = %.4f (must be 1) ; chi2 = E0[L^2]-1 = %.4f ; TV = 0.5*E0|L-1| = %.4f ; 0.5*sqrt(chi2) = %.4f" % (s1 / M, s2 / M - 1, 0.5 * sabs / M, 0.5 * sqrt(s2 / M - 1)))
print("   real set: e = %s, sum e^2 = %d, L = %.3f, null percentile of L %.1f%%" % (e_real, sum(x * x for x in e_real), exp(lr), 100 * below / M))
chiA = s2 / M - 1

print("B. equal-colour side pairs inside the 196 interior tiles (6 pairs per tile; adjacent and opposite)")
def stat(tiles):
    adj = sum((t[0] == t[1]) + (t[1] == t[2]) + (t[2] == t[3]) + (t[3] == t[0]) for t in tiles)
    opp = sum((t[0] == t[2]) + (t[1] == t[3]) for t in tiles)
    return adj, opp
real = stat(interior)
K = 6000; nul = []; pla = []
slots = list(pool)
V = {(r, c): i for i, (r, c) in enumerate((r, c) for r in range(1, 16) for c in range(1, 15))}
H = {(r, c): 210 + i for i, (r, c) in enumerate((r, c) for r in range(1, 15) for c in range(1, 16))}
cells = [(V[(r, c)], H[(r, c + 1)], V[(r + 1, c)], H[(r, c)]) for r in range(1, 15) for c in range(1, 15)]
epool = [i for i, nc in enumerate(n) for _ in range(nc // 2)]
for _ in range(K):
    rng.shuffle(slots); nul.append(stat([slots[4 * i:4 * i + 4] for i in range(196)]))
    rng.shuffle(epool); pla.append(stat([[epool[x] for x in cell] for cell in cells]))
def ms(v): m = sum(v) / len(v); return m, sqrt(sum((x - m) ** 2 for x in v) / (len(v) - 1))
chiB = 0.0
for j, lab in ((0, "adjacent sides"), (1, "opposite sides"), (2, "all six pairs")):
    f = (lambda x: x[0] + x[1]) if j == 2 else (lambda x, j=j: x[j])
    a = [f(x) for x in nul]; b = [f(x) for x in pla]; (m0, s0), (m1, sd1) = ms(a), ms(b)
    ha, hb = Counter(a), Counter(b); tv = 0.5 * sum(abs(ha[k] / K - hb[k] / K) for k in set(ha) | set(hb))
    d = (m0 - m1) / s0
    print("   %-15s null %.2f +- %.2f  painted %.2f +- %.2f  shift %.3f sd -> chi2 about %.4f, TV (normal) %.3f, TV (histograms, biased up) %.3f | real set %d (z = %+.2f against null)" % (lab, m0, s0, m1, sd1, d, exp(d * d) - 1, 2 * (0.5 - 0.5 * (1 + __import__("math").erf(-abs(d) / 2 / sqrt(2)))) , tv, f(real), (f(real) - m0) / s0))
    if j == 2: chiB = exp(d * d) - 1

print("C. corner statistic: colours on the 4 clockwise and 4 counter-clockwise corner half-edges (exact)")
def comps(tot, k):
    if k == 1: yield (tot,); return
    for a in range(tot + 1):
        for rest in comps(tot - a, k - 1): yield (a,) + rest
chiC = 0.0
for a in comps(4, 5):
    for b in comps(4, 5):
        p0 = 1.0; p1 = 1.0
        for x, y in zip(a, b):
            p0 *= comb(12, x) * comb(12, y); p1 *= comb(12, x) * comb(12 - x, y)
        p0 /= comb(60, 4) ** 2; p1 /= comb(60, 4) * comb(56, 4)
        chiC += p1 * p1 / p0
chiC -= 1
print("   chi2 = %.4f" % chiC)
tot = (1 + chiA) * (1 + chiB) * (1 + chiC) - 1
print("   A, B, C concern disjoint slot classes or colour sets; treated as independent the combined lower bound is chi2 >= about %.3f" % tot)
print("   survey: chi2 = 1.03/14702 = %.1e, TV <= %.1e" % (1.03 / 14702, 0.5 * sqrt(1.03 / 14702)))

print("D. toy 4x4, 2 frame colours (6,6), 3 interior colours (4,4,4), census fixed")
nn = 4
def first_moment():
    sig = 24 * factorial(8) * factorial(4) * 4 ** 4
    pf = (factorial(6) ** 2) / factorial(12)
    pi = (factorial(12) / factorial(4) ** 3) / (factorial(24) / factorial(8) ** 3)
    return sig * pf * pi
E0 = first_moment()
FE = [("H", r, c) for r in (0, 3) for c in (1, 2, 3)] + [("V", r, c) for r in (1, 2, 3) for c in (0, 3)]
IE = [("H", r, c) for r in (1, 2) for c in (1, 2, 3)] + [("V", r, c) for r in (1, 2, 3) for c in (1, 2)]
assert len(FE) == 12 and len(IE) == 12
fcol = [1] * 6 + [2] * 6; icol = [3] * 4 + [4] * 4 + [5] * 4
def paint():
    rng.shuffle(fcol); rng.shuffle(icol)
    col = dict(zip(FE, fcol)); col.update(zip(IE, icol))
    g = lambda k: col.get(k, 0)
    return [(g(("V", r, c)), g(("H", r, c + 1)), g(("V", r + 1, c)), g(("H", r, c))) for r in range(4) for c in range(4)]
cw = [1] * 6 + [2] * 6; ccw = [1] * 6 + [2] * 6; ih = [3] * 8 + [4] * 8 + [5] * 8
def null():
    rng.shuffle(cw); rng.shuffle(ccw); rng.shuffle(ih)
    t = [(0, cw[i], ccw[i], 0) for i in range(4)]
    t += [(0, cw[4 + i], ih[i], ccw[4 + i]) for i in range(8)]
    t += [tuple(ih[8 + 4 * i:12 + 4 * i]) for i in range(4)]
    return t
def isD(pcs):
    seen = set()
    for p in pcs:
        rots = [p[i:] + p[:i] for i in range(4)]
        m = min(rots)
        if len(set(rots)) < 4 or m in seen: return False
        seen.add(m)
    return True
def count(pcs):
    t = defaultdict(list)
    for i, p in enumerate(pcs):
        for k in range(4):
            N, E, S, W = p[k:] + p[:k]
            t[(N, W, E == 0, S == 0)].append((1 << i, E, S))
    south = [0] * 4
    def rec(pos, used, east):
        if pos == 16: return 1
        r, c = divmod(pos, 4); tot = 0
        for (b, E, S) in t.get((south[c], 0 if c == 0 else east, c == 3, r == 3), ()):
            if used & b: continue
            old = south[c]; south[c] = S
            tot += rec(pos + 1, used | b, E)
            south[c] = old
        return tot
    return rec(0, 0, 0)
NP = 250000; zs_all = []; zs_D = []; nD = 0
for i in range(NP):
    p = paint(); d = isD(p)
    if d: nD += 1; zs_D.append(count(p))
    if i < 6000: zs_all.append(count(p))
N0 = 600000; n0D = sum(isD(null()) for _ in range(N0))
ma, sa = ms(zs_all)
print("   E_0[Z] (first moment, labelled) = %.3f" % E0)
print("   no condition:  E_pl[Z] = %.2f +- %.2f  ->  chi2 = E_pl[Z]/E_0[Z] - 1 = %.2f   (U4 form would give (1+small)/E_0[Z] = %.3f)" % (ma, sa / sqrt(len(zs_all)), ma / E0 - 1, 1.03 / E0))
pD, p0D = nD / NP, n0D / N0
E0D = E0 * pD / p0D
md, sd = ms(zs_D)
print("   distinct, non-symmetric tiles (D): P_pl(D) = %.5f, P_0(D) = %.5f, E_0[Z|D] = %.2f ; E_pl[Z|D] = %.2f +- %.2f (%d boards)" % (pD, p0D, E0D, md, sd / sqrt(len(zs_D)), len(zs_D)))
print("   -> chi2 under D = %.3f (+- about %.3f) ; U4 form (1+small)/E_0[Z|D] = %.3f ; excess E_pl - E_0 - 1 = %.2f" % (md / E0D - 1, sqrt((sd / sqrt(len(zs_D)) / E0D) ** 2 + (md / E0D) ** 2 * (1 / max(nD, 1) + 1 / max(n0D, 1))), 1.03 / E0D, md - E0D - 1))
