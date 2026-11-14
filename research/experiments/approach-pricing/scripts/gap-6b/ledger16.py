"""Gap 6b: entropy ledger of reformulations on the 16x16, uniform first-moment model.

Uniform model: 17 interior colours and 5 frame colours, each equiprobable on a
half-edge. The repo's census model differs by a fraction of a bit per quantity
(the re-check's census figures are quoted in the memo where they exist).
Units: bits. Run: python3 -I ledger16.py
"""
from math import lgamma, log, log2, exp, pi, sqrt, comb

LN2 = log(2)
def lf(n): return lgamma(n + 1) / LN2            # log2 n!
def lfall(n, k): return lf(n) - lf(n - k)        # log2 n(n-1)..(n-k+1)

CI, CF = 17, 5
NI, NE, NC = 196, 56, 4
E_II = 2 * 14 * 13        # interior-interior edges: 364
E_FI = 56                 # frame-edge piece to interior: 56
E_FF = 60                 # frame ring edges
b17, b5 = log2(CI), log2(CF)

print("== 1. totals")
H = lf(NI) + 2 * NI + lf(NE) + lf(NC)
C = (E_II + E_FI) * b17 + E_FF * b5
Hrel = NI * log2(4 * NI) + NE * log2(NE) + NC * log2(NC)
print(f"strict arrangements          {H:8.1f}")
print(f"edge constraints             {C:8.1f}   (420 x {b17:.3f} + 60 x {b5:.3f})")
print(f"log2 E[Z], no clue           {H - C:8.1f}")
print(f"colour-only arrangements     {Hrel:8.1f}")
print(f"colour-only solutions        {Hrel - C:8.1f}")
print(f"each-tile-once deficit       {Hrel - H:8.1f}   (= bits by which a relaxed solution fails to be strict)")

print("\n== 2. interior surplus of a tile order that closes 2 interior edges per tile")
s = 0.0; peak = 0.0; arg = 0
for j in range(NI):
    s += log2(4 * (NI - j)) - 2 * b17
    if s > peak: peak, arg = s, j + 1
print(f"peak of sum log2(4r/289)     {peak:8.1f} at {arg} interior tiles placed (r = {NI-arg} left); end value {s:.1f}")
for g, name in ((2, "axis known (2 options per tile)"), (1, "orientation known")):
    s = 0.0; pk = 0.0
    for j in range(NI):
        s += log2(g * (NI - j)) - 2 * b17
        pk = max(pk, s)
    print(f"same with {name:32s} peak {pk:6.1f}; information supplied {NI*log2(4/g):.0f} bits")

print("\n== 3. convexity of the check in the number of known sides of an interior cell")
print("sides known  expected candidates  P(domain non-empty)  check bits   bits committed (half an edge each side)")
for sides, pats in ((1, CI), (2, CI**2), (3, CI**3), (4, CI**4)):
    lam = 4 * NI / pats
    p = 1 - exp(-lam)
    print(f"   {sides}          {lam:10.3f}          {p:8.4f}          {-log2(p):6.2f}        {sides*b17/2:6.2f}")

print("\n== 4. edge colourings, all left-right edges first (one edge closed per tile)")
h_edges = 14 * 13 * b17 + 14 * 2 * b17 + 2 * 15 * b5     # 14 interior rows incl. frame ends, 2 frame rows
nmatch = lf(NI) + 2 * NI - NI * 2 * b17                 # log2 E[# perfect matchings] interior
p_noiso = NI * log2(1 - exp(-4 * NI / CI**2))
print(f"bits committed (240 edges)   {h_edges:8.1f}")
print(f"log2 E[perfect matchings]    {nmatch:8.1f}; isolated-tile bound on P(matching) 2^{p_noiso:.1f}")
rows = lf(NI) + 2 * NI - 14 * 13 * b17
print(f"boards matched left-right only (interior, first moment) {rows:8.1f}")

print("\n== 5. colour coarsening (multigrid): solutions of the puzzle with interior colours merged")
print("classes  q=sum p^2   log2 abstract solutions (interior ledger + frame), no clue")
frame_part = lf(NE) + lf(NC) - E_FF * b5
def coarse(sizes):
    q = sum((x / CI) ** 2 for x in sizes)
    pats = 1 / q ** 4
    ent = min(lf(NI) + 2 * NI, NI * log2(pats))
    return q, ent + frame_part + (E_II + E_FI) * log2(q)
for sizes in ([1]*17, [2]+[1]*15, [2,2]+[1]*13, [2]*4+[1]*9, [2]*8+[1], [3]*5+[2], [5,4,4,4], [9,8]):
    q, v = coarse(sizes)
    print(f"  {len(sizes):3d}     {q:7.4f}    {v:8.1f}")

print("\n== 6. region first: pools")
print(f"split interior tiles in two halves        C(196,98)      = {log2(comb(196,98)):6.1f}")
print(f"split in four quadrant pools of 49        196!/(49!)^4   = {lf(196)-4*lf(49):6.1f}")
for s_, (w, h) in ((49, (7, 7)), (98, (14, 7))):
    e = 2 * w * h - w - h
    n = lfall(NI, s_) + 2 * s_ - e * b17
    pool = log2(comb(NI, s_))
    ptile = lf(s_) + 2 * s_ - e * b17
    print(f"{w}x{h} free-boundary block: strict fillings {n:6.1f}; pools {pool:6.1f}; log2 E[tilings | pool] {ptile:6.1f}; pools that tile = {pool+ptile:6.1f}")

print("\n== 7. colour layers")
ec = (E_II + E_FI) / CI
dimers = lgamma(421) / LN2 - lgamma(421 - ec) / LN2 - lgamma(ec + 1) / LN2
print(f"edges of one colour {ec:.1f}; ways to choose them {dimers:.1f}; ways to seat ~{2*ec:.0f} tiles on them about {lf(round(2*ec)):.1f}; checks: 0")

print("\n== 8. orientation first")
var = NI * (1/CI) * (1 - 1/CI) * 2     # variance of (#N-sides=c) - (#S-sides=c)
bits_eq = -log2(1 / sqrt(2 * pi * var))
print(f"bits committed 392; census equalities 2 x 16 independent, {bits_eq:.2f} bits each = {32*bits_eq:.0f}; survivors {392-32*bits_eq:.0f}")

print("\n== 9. sampling the colour-only relaxation")
print(f"P(strict | relaxed), first moment 2^-{Hrel-H:.1f}; tile types absent from a relaxed solution about {256*exp(-1):.0f} of 256")
print(f"expected cells of a relaxed sample that agree with a given strict solution about {256/(4*NI):.2f}")

print("\n== 10. row band is the isoperimetric optimum at the peak")
print("Bollobas-Leader: a k-subset of the n x n grid, n^2/4 <= k <= 3n^2/4, has at least n open edges; a band has exactly n = 16")
