"""Cost-model numbers for the counting/algebra memo, from the official piece set.
Stdlib only: python3 -I costs.py <path to research/starter-kit/data/official.json>
All counts are first-moment (annealed) estimates unless marked exact."""
import json, sys
from math import lgamma, log, log10, pi, e
from collections import Counter

d = json.load(open(sys.argv[1]))
P = d["pieces"]
L10 = log(10)
def lf(n): return lgamma(n + 1) / L10            # log10 n!
def lperm(n, k): return lf(n) - lf(n - k)         # log10 n!/(n-k)!

corners = [p for p in P if p.count(0) == 2]
edges = [p for p in P if p.count(0) == 1]
inner = [p for p in P if p.count(0) == 0]
print("pieces", len(P), "corners", len(corners), "edges", len(edges), "interior", len(inner))

# colour census (exact, from the piece list)
innerc = Counter(c for p in inner for c in p)
framec, inwardc = Counter(), Counter()
for p in edges:
    i = p.index(0)
    inwardc[p[(i + 2) % 4]] += 1
    framec[p[(i + 1) % 4]] += 1; framec[p[(i + 3) % 4]] += 1
for p in corners:
    for c in p:
        if c: framec[c] += 1
print("frame colours", dict(sorted(framec.items())))
print("interior colours on interior pieces", dict(sorted(innerc.items())))
print("inward sides of edge pieces", dict(sorted(inwardc.items())))
print("frame colours seen on interior pieces:", sorted(set(framec) & set(innerc)))
F = sum(framec.values()); H = sum(innerc.values()); A = sum(inwardc.values())
p_ff = sum((v / F) ** 2 for v in framec.values())
p_ii = sum((v / H) ** 2 for v in innerc.values())
p_fi = sum(inwardc[c] / A * innerc[c] / H for c in innerc)
print("match prob frame-frame %.5f (1/%.2f)  int-int %.5f (1/%.2f)  frame-int %.5f (1/%.2f)"
      % (p_ff, 1 / p_ff, p_ii, 1 / p_ii, p_fi, 1 / p_fi))
lii, lff, lfi = log10(p_ii), log10(p_ff), log10(p_fi)

# 1. uniqueness entropy: colour-consistent boards with reuse, per real solution
def fake(n): return n * log10(n) - lf(n)
fr = fake(4) + fake(56) + fake(196)
print("\n[1] fakes per real solution, log10: corners %.2f + edges %.2f + interior %.2f = %.2f"
      % (fake(4), fake(56), fake(196), fr))
print("    with 14,702 solutions (reported): colour-only boards ~ 10^%.1f" % (fr + log10(14702)))
print("    per open cell asymptotically log10(e) = %.4f" % log10(e))

# 2. exact colour-only contraction: frontier between two rows, and mid-row
nf, ni = len(framec), len(innerc)
row = 2 * log10(nf) + 14 * log10(ni)
print("\n[2] colour-only frontier states: row cut 10^%.2f, mid-row (one more edge) 10^%.2f"
      % (row, row + log10(ni)))
print("    bytes at 8 B/state: 10^%.2f ; inclusion-exclusion multiplier 2^256 = 10^%.1f"
      % (row + log10(ni) + log10(8), 256 * log10(2)))

# 3. naive first-moment profile T(k) of strict partial boards, row-major scan
def profile():
    T = 0.0; out = []; nc, ne, nin = 4, 56, 196
    for r in range(16):
        for c in range(16):
            onb = (r in (0, 15)) + (c in (0, 15))
            if onb == 2: T += log10(nc); nc -= 1
            elif onb == 1: T += log10(ne); ne -= 1
            else: T += log10(4 * nin); nin -= 1
            for (rr, cc) in ((r - 1, c), (r, c - 1)):      # edges to already placed cells
                if rr < 0 or cc < 0: continue
                ob2 = (rr in (0, 15)) + (cc in (0, 15))
                if onb >= 1 and ob2 >= 1 and (r == rr and r in (0, 15) or c == cc and c in (0, 15)):
                    T += lff
                elif onb == 0 and ob2 == 0: T += lii
                else: T += lfi
            out.append(T)
    return out
T = profile()
pk = max(range(256), key=lambda k: T[k])
print("\n[3] naive first-moment row-scan profile, log10 T(k):")
for k in (15, 31, 63, 95, 127, 143, 159, 175, pk, 207, 223, 239, 255):
    print("    k=%3d  %.1f" % (k + 1, T[k]))
print("    peak at k=%d: 10^%.1f ; sum over levels 10^%.1f ; end value 10^%.2f (site: naive product ~0.02, corrected 14,702)"
      % (pk + 1, T[pk], log10(sum(10 ** (t - T[pk]) for t in T)) + T[pk], T[255]))

# 4. where colour-only lookahead with the exact remaining pool becomes informative:
#    fake completions per partial board = N_sol/T(k) * r^r/r!
print("\n[4] expected fake completions per depth-k partial (pool = unused pieces, reuse allowed), log10:")
corr = log10(14702) - T[255]          # put the reported end value on the naive profile
for k in (96, 128, 144, 152, 160, 168, 176, 184, 192, 208):
    r = 256 - k
    print("    k=%3d r=%3d  %.1f   (T(k) rescaled 10^%.1f)" % (k, r, log10(14702) - (T[k - 1] + corr * k / 256) + fake(r), T[k - 1] + corr * k / 256))

# 5. block moves in the matched-with-duplicates ensemble: colour-only alternatives of a w-row strip
#    (interior rows, both horizontal boundaries fixed), and with a fraction q of pieces "tracked"
def strip_alt(w):
    cells_i = 14 * w; cells_e = 2 * w
    a = cells_i * log10(4 * 196) + cells_e * log10(56)
    e_ii = 13 * w + 14 * (w - 1) + 2 * 14       # horizontal int-int, vertical inside, two boundaries
    e_fi = 2 * w
    e_ff = 2 * (w - 1) + 2 * 2
    return a + e_ii * lii + e_fi * lfi + e_ff * lff
def track_cost(r, q, n=196):
    k = round(q * r)
    return lperm(r, k) - k * log10(n) + (r - k) * log10(1 - q)
print("\n[5] w-row strip resample: log10 colour-only alternatives, memory 17^w, and log10 alternatives that keep q of the pieces used exactly once")
for w in (2, 3, 4, 5, 6, 7, 8):
    alt = strip_alt(w); r = 14 * w
    row5 = " ".join("q=%.1f:%6.1f" % (q, alt + track_cost(r, q)) for q in (0.2, 0.3, 0.4, 0.5, 0.6, 0.8))
    print("    w=%d  alt 10^%5.1f  states 10^%4.1f   %s" % (w, alt, w * log10(ni), row5))
for w in (6, 7, 8):
    alt = strip_alt(w); r = 14 * w
    q = 0.0
    while q < 0.99 and alt + track_cost(r, q + 0.01) > 0: q += 0.01
    k = q * 256
    print("    w=%d freezes near q*=%.2f (k*=%.0f tracked pieces); residual fakes e^(256-k*) ~ 10^%.0f" % (w, q, k, (256 - k) * log10(e)))

# 6. hole size below which colour-only counting has no fakes in expectation (s x s hole, pool = its s^2 pieces)
print("\n[6] s x s interior hole, pool of s^2 pieces with reuse: log10 expected colour-only fillings; strict DFS root branching")
for s in (4, 6, 8, 9, 10, 11, 12, 14):
    r = s * s
    print("    s=%2d  %.1f   contraction states 10^%.1f   first-cell candidates %.2f"
          % (s, r * log10(4 * r) + (2 * s * (s - 1) + 4 * s) * lii, s * log10(ni), 4 * r * p_ii ** 2))

# 7. region enumeration sizes
def region(nc, ne, nin, eff, efi, eii):
    return lperm(4, nc) + lperm(56, ne) + lperm(196, nin) + nin * log10(4) + eff * lff + efi * lfi + eii * lii
print("\n[7] strict tilings of regions (first moment, log10): top half %.1f ; 8x8 corner quadrant %.1f ; C(196,98) = 10^%.1f"
      % (region(2, 28, 98, 29, 14 + 14, 7 * 13 + 6 * 14), region(1, 14, 49, 14, 14, 7 * 6 * 2), lf(196) - 2 * lf(98)))
print("    budget: 1,000 core-years at 1e8..1e9 nodes/s = 10^%.1f..10^%.1f node visits; three 16-core desktops for a year = 10^%.1f..10^%.1f"
      % (log10(1000 * 3.156e7 * 1e8), log10(1000 * 3.156e7 * 1e9), log10(48 * 3.156e7 * 1e8), log10(48 * 3.156e7 * 1e9)))

# 8. the same freeze estimate for a generated n x n board with cb frame and ci interior colours (uniform colours)
def freeze(n, cb, ci, w):
    ni_, ne_ = (n - 2) ** 2, 4 * (n - 2)
    alt = (n - 2) * w * log10(4 * ni_) + 2 * w * log10(ne_) \
        - ((n - 3) * w + (n - 2) * (w - 1) + 2 * (n - 2) + 2 * w) * log10(ci) - (2 * (w - 1) + 4) * log10(cb)
    r = (n - 2) * w; q = 0.0
    while q < 0.99 and alt + track_cost(r, q + 0.01, ni_) > 0: q += 0.01
    return alt, q
print("\n[8] predicted freeze of w-row strip moves on generated boards (alternatives at q=0, q*, k* = q* n^2)")
for (n, cb, ci) in ((6, 3, 6), (8, 3, 8), (8, 3, 9), (10, 4, 11)):
    print("    n=%d cb=%d ci=%d: " % (n, cb, ci) + " ; ".join("w=%d alt 10^%.1f q*=%.2f k*=%.0f" % ((w,) + freeze(n, cb, ci, w) + (freeze(n, cb, ci, w)[1] * n * n,)) for w in (2, 3, 4, 5) if w <= n - 3))
