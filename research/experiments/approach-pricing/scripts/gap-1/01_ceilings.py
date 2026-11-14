"""Gap 1: turn the per-depth shares of 02_real_reasons.py into bits on the whole tree,
with the measured level widths of Track 10 (five clues, N = 1e6) as weights.

usage: python3 -I 01_ceilings.py 02_real_reasons.all.out
"""
import sys
from math import log2, log10, lgamma, log

sys.path.insert(0, __import__("os").path.dirname(__file__))
import e2real as E

HS = (1, 2, 4, 8, 12, 16, 24, 32)
BS = (20, 30, 40, 50, 61, 80)
L10 = log2(10)
w = E.measured_widths()
N = [10 ** (x - 30) for x in w]  # scaled
total = sum(N)
print("levels with a measured width: 0 to %d" % (len(w) - 1))
print("total  log10 %.3f  = 2^%.2f   widest level: free depth %d, log10 %.2f" % (log10(total) + 30, (log10(total) + 30) * L10, max(range(len(w)), key=lambda d: w[d]), max(w)))
print()
print("A. Ceiling of a free, perfect cut at depth d (everything deeper removed): log2(total / sum to d)")
cum, cums = 0.0, []
for d in range(len(N)):
    cum += N[d]
    cums.append(cum)
for d in (48, 64, 80, 96, 112, 128, 144, 158, 160):
    print("   d = %3d   %.1f bits" % (d, log2(total / cums[d])))
for g in (3, 5, 10, 20, 30, 56, 73):
    d = max(d for d in range(len(N)) if log2(total / cums[d]) >= g)
    print("   a gain of %2d bits needs nearly every node cut at free depth %d or before (row %d of the scan)" % (g, d, E.CELLS[d][0] if d < E.NFREE else -1))
print()
print("B. First-moment LOWER bound on the gain of a free perfect oracle of height H: log2(total / sum_d min(N_d, N_(d+H)))")
for H in HS + (48, 64):
    rem = sum(min(N[d], N[d + H] if d + H < len(N) else 0.0) for d in range(len(N)))
    print("   H = %2d   %.2f bits" % (H, log2(total / rem)))
print()
rows = {}
for l in open(sys.argv[1]):
    if l.startswith("ROW "):
        v = l.split()
        rows[int(v[1])] = [float(x) for x in v[2:]]
ds = sorted(rows)
print("C. Measured on the real pieces (%d depths, %d to %d). Depths before the first are counted as untouched, depths after the last as fully removed." % (len(ds), ds[0], ds[-1]))
covered = sum(N[d] for d in ds) / total
print("   share of the tree at the sampled depths: %.4f" % covered)


def gain(col):
    rem = 0.0
    for d in range(len(N)):
        if d < ds[0]:
            rem += N[d]
        elif d in rows:
            rem += N[d] * rows[d][col]
    return log2(total / rem)


for i, H in enumerate(HS):
    print("   free perfect oracle, height <= %2d : ceiling %.2f bits of nodes" % (H, gain(i)))
for i, B in enumerate(BS):
    print("   free perfect store, reasons <= %2d bits : ceiling %.2f bits of nodes" % (B, gain(len(HS) + i)))
print()
print("D. Reason arithmetic")
print("   bits per frontier colour: inner log2(17) = %.2f, frame log2(5) = %.2f" % (log2(17), log2(5)))
for d in (112, 128, 144, 158, 170):
    r, c = E.CELLS[d]
    # interior tiles used at depth d: free cells placed minus border cells placed
    placed = E.CELLS[:d]
    ui = sum(1 for (rr, cc) in placed if 0 < rr < 15 and 0 < cc < 15)
    print("   depth %d (row %d col %d): interior tiles used %d of %d, bits per blocked tile %.2f (first), %.2f (40th)" % (
        d, r, c, ui, E.INTERIOR_POOL, log2(E.INTERIOR_POOL / ui), log2((E.INTERIOR_POOL - 39) / (ui - 39)) if ui > 39 else float("nan")))
print("   window of a refutation of height h: h + 2 colours; in bits %.1f at h = 5, %.1f at h = 8, %.1f at h = 12, %.1f at h = 15" % tuple(log2(17) * (h + 2) for h in (5, 8, 12, 15)))
print("   largest height whose window alone fits: 40 bits -> h = %d, 50 -> %d, 61 -> %d" % tuple(int(B / log2(17)) - 2 for B in (40, 50, 61)))
print()
print("E. Width-size relation, numbers only (form S >= exp((w - k)^2 / (c n)), c of order 1 to 16; theorem not opened in the original)")
for n, name in ((480 * 17, "seam-colour variables only"), (157000, "direct placement variables")):
    for bits in (50, 61):
        print("   n = %6d (%s): a bound of 2^%d needs refutation width above k + %.0f literals (c = 1)" % (n, name, bits, (n * bits * log(2)) ** 0.5))
print("   widest clause a row-scan state can need: 17 colours x 17 values + 256 tile literals = %d literals" % (17 * 17 + 256))
