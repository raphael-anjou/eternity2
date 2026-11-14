"""Gap 2, script 1: the sketch-size / false-positive / candidate-join trade-off on the real 16x16.
First-moment model on the real colour census (m12_model.py of the derivations second analysis, imported read-only).
Baseline order = the re-check's 'rows bottom-up' (model total 2^134.6 with five clues).
For every cut d (top list = first d cells of the order, bottom list = the other 256-d cells, scanned from the far end):
  NA = strict fillings of the prefix, NB = strict fillings of the complement, C = admissible used sets at the cut,
  K = log2(NA*NB/E[Z]) = bits of the exact join condition (seam + used set) in the model.
Run: python3 -I g2_real_tradeoff.py <official_eternity2.csv>"""
import sys, os
from collections import Counter
from math import log10, log2, lgamma, log, exp
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "..", "surveys", "DERIVATIONS-recheck.scripts"))
from m12_model import interior_match_logp, frame_match_logp, profile
N = 16; B = log2(10)
def lchoose(n, k): return (lgamma(n + 1) - lgamma(k + 1) - lgamma(n - k + 1)) / log(2)
rows = [l.strip() for l in open(sys.argv[1]) if l.strip()]
pieces, hints = [], []
for i, l in enumerate(rows[1:]):
    c = l.split(",")
    pieces.append([0 if int(x, 2) == 65535 else int(x, 2) for x in c[:4]])
    x, y, rot = int(c[4]), int(c[5]), int(c[6])
    if x or y or rot: hints.append((y, x, i))
fc, ic = Counter(), Counter()
for p in pieces:
    z = [i for i, v in enumerate(p) if v == 0]
    if len(z) == 0:
        for v in p: ic[v] += 1
    elif len(z) == 1:
        i = z[0]; ic[p[(i + 2) % 4]] += 1; fc[p[(i + 1) % 4]] += 1; fc[p[(i + 3) % 4]] += 1
    else:
        for v in p:
            if v: fc[v] += 1
lpi = interior_match_logp(list(ic.values()), 420); lpf = frame_match_logp([v // 2 for v in fc.values()], 60)
clue = {(h[0], h[1]) for h in hints}
order = [(r, c) for r in reversed(range(N)) for c in range(N)]
pools = {"corner": 4, "edge": 56, "interior": 196 - len(clue)}
A = [x[0] * B for x in profile(N, order, pools, lpi, lpf, clue)]            # log2 NA(d), d = index+1
Bq = [x[0] * B for x in profile(N, order[::-1], pools, lpi, lpf, clue)]     # log2 NB of the last j cells, j = index+1
EZ = A[-1]
def kind(r, c): return (r in (0, N - 1)) + (c in (0, N - 1))
def cut(d):
    used = [0, 0, 0]
    for (r, c) in order[:d]:
        if (r, c) not in clue: used[kind(r, c)] += 1
    C = lchoose(191, used[0]) + lchoose(56, used[1]) + lchoose(4, used[2])
    return A[d - 1], Bq[256 - d - 1], C
TOT = log2(sum(2 ** (a - 100) for a in A)) + 100
print("model: total nodes 2^%.1f, peak 2^%.1f at depth %d, E[Z] (five clues, unplanted) 2^%.1f" % (TOT, max(A), A.index(max(A)) + 1, EZ))
def seam_bits(d):
    """Uniform-colour width of the seam between the first d cells and the rest: log2(17) per interior edge, log2(5) per frame edge."""
    top = set(order[:d]); ni = nf = 0
    for (r, c) in top:
        for (rr, cc) in ((r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)):
            if 0 <= rr < N and 0 <= cc < N and (rr, cc) not in top:
                if (r == rr and r in (0, N - 1)) or (c == cc and c in (0, N - 1)): nf += 1
                else: ni += 1
    return ni * log2(17) + nf * log2(5)
print("\n== 1. two-list join, cut by cut (bits). F = seam width with uniform colours; Fm = seam width implied by the fixed-census model (NA+NB-E[Z]-C) ==")
print("   d rows |    NA     NB  C(used sets)     F    Fm | widest level: top  bottom | b for <=1 false join: with F, with Fm | used sets per seam (F): top bottom")
def row(d):
    a, b, C = cut(d); F = seam_bits(d); Fm = a + b - EZ - C
    return a, b, C, F, Fm, max(A[:d]), max(Bq[:256 - d])
for d in list(range(16, 256, 16)) + [137]:
    a, b, C, F, Fm, pa, pb = row(d)
    print("%4d %4.1f | %6.1f %6.1f %8.1f %6.1f %5.1f | %6.1f %6.1f | %6.1f %6.1f | %6.1f %6.1f" % (d, d / 16, a, b, C, F, Fm, pa, pb, a + b - F, a + b - Fm, a - F, b - F))
lst = min((max(row(d)[0], row(d)[1]), d) for d in range(80, 192, 16))
gen = min((max(row(d)[5], row(d)[6]), d) for d in range(1, 256))
genr = min((max(row(d)[5], row(d)[6]), d) for d in range(16, 256, 16))
print("max of the two list sizes over row cuts 5 to 11: smallest 2^%.1f at d=%d ; at rows 7 and 9 it is 2^126.8 (the 10^38.16 quoted as the join floor). A list size is not a cost floor: it keeps falling toward the ends." % lst)
print("floor by widest level of each side's own row scan, row cuts only: 2^%.1f at d=%d ; all 255 cuts: 2^%.1f at d=%d ; gain against the model tree %.1f bits" % (genr[0], genr[1], gen[0], gen[1], TOT - gen[0]))

print("\n== 2. the trade-off at a cut: b-bit sketch of the used set; best possible pass rate of a wrong pair is 2^-b (hash) ==")
for d in (112, gen[1], 144):
    a, b, C, F, Fm, pa, pb = row(d)
    print(" cut d=%d: NA 2^%.1f, NB 2^%.1f, seam F %.1f bits, used-set condition %.1f bits" % (d, a, b, F, C))
    print("    b | candidate joins log2 | table cells F+b | bottom entries per cell log2 | pass rate of a wrong top entry | occupied cells log2")
    for bb in (0, 16, 32, 48, 58, 64, 72, 80, 96, 128, int(a + b - F) + 1, int(C) + 1):
        lam = b - F - bb
        pas = 1 - exp(-2 ** lam) if lam < 30 else 1.0
        occ = (F + bb) + (log2(1 - exp(-2 ** lam)) if lam > -40 else lam) if lam < 30 else F + bb
        print("  %4d | %7.1f | %6.1f | %7.1f | %.3g | %.1f" % (bb, a + b - F - bb, F + bb, lam, pas, occ))

print("\n== 3. a filter placed at depth d of the row scan (top = strict search, bottom = any sound oblivious table) ==")
print("   d | nodes already paid above d (widest level so far) | bottom fillings NB | sketch bits before the table prunes (NB - F) | used sets per seam in the tree (NA - F) | admissible sets C")
for d in (64, 96, 112, 128, 137, 144, 161, 176, 192, 208):
    a, b, C, F, Fm, pa, pb = row(d)
    print("%4d | %6.1f | %6.1f | %6.1f | %6.1f | %6.1f" % (d, pa, b, b - F, a - F, C))


# (the price of a sketch DP for a bottom region is in g2_region_sketch_price.py)

print("\n== 4. named sketches, pseudo-solutions per true solution (whole board, 256 tiles) ==")
def l2fact(n): return lgamma(n + 1) / log(2)
print(" colour-only (no sketch): 256^256/256! = 2^%.1f" % (256 * 8 - l2fact(256)))
for k in (8, 13, 16, 32, 64, 128):
    s = 256 // k
    print(" %3d classes of %3d tiles, count per class (state up to %5.1f bits): (s^s/s!)^k = 2^%.1f" % (k, s, k * log2(s + 1), k * (s * log2(s) - l2fact(s))))
for t in (32, 58, 64, 128, 184, 220):
    u = 256 - t
    # tracked t tiles exactly, u untracked reusable: sequences = sum_j C(256,j) [j tracked cells] t!/(t-j)! u^(256-j); the relaxation has free choice u per cell
    best = max(lchoose(256, j) + (l2fact(t) - l2fact(t - j)) + (256 - j) * log2(u) for j in range(0, t + 1))
    print(" tracked subset of %3d tiles (state up to %3d bits), %3d reusable: sequences/256! >= 2^%.1f ; choices per cell from reusable tiles alone %.2f x 4/289 = %.2f" % (t, t, u, best - l2fact(256), u, u * 4 / 289))
for bb in (16, 58, 128, 256):
    print(" random linear sketch of %3d bits: 2^(%.1f - %d) = 2^%.1f" % (bb, 256 * 8 - l2fact(256), bb, 256 * 8 - l2fact(256) - bb))

print("\n== 5. unsound sketches: random split of the tiles between the two halves (divide and colour) ==")
for d in (112, 128, 144):
    a, b, C = cut(d)
    used = [0, 0, 0]
    for (r, c) in order[:d]:
        if (r, c) not in clue: used[kind(r, c)] += 1
    pa = {"interior": used[0], "edge": used[1], "corner": used[2]}
    pb = {"interior": 191 - used[0], "edge": 56 - used[1], "corner": 4 - used[2]}
    ta = [x[0] * B for x in profile(N, order[:d], pa, lpi, lpf, clue)]
    tb = [x[0] * B for x in profile(N, order[::-1][:256 - d], pb, lpi, lpf, clue)]
    print(" cut d=%d: success per split 2^-%.1f ; top tree with its own pool: peak 2^%.1f ; bottom: peak 2^%.1f ; total 2^%.1f" % (d, C, max(ta), max(tb), C + max(max(ta), max(tb))))

print("\n== 6. memory of a Bloom filter over the smaller list at the floor cut ==")
a, b, C = cut(137)
for eps in (1e-3, 1e-6, 1e-12):
    print(" false-positive rate %.0e: %.1f bits per entry, 2^%.1f bits in all for 2^%.1f entries" % (eps, 1.44 * log2(1 / eps), min(a, b) + log2(1.44 * log2(1 / eps)), min(a, b)))
