"""Claim A. Expected number of valid partial boards per depth, official set,
row scan, and what each memo's number measures. Run: python3 -I 01_price.py"""
import itertools
import random
import time
import os
import sys
from math import log10
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rv_lib import *

t0 = time.time()

# --- 0. formula check by brute force on a tiny census -----------------------
def brute_int(counts, e):
    slots = [c for c, n in enumerate(counts) for _ in range(n)]
    ok = tot = 0
    for perm in set(itertools.permutations(slots)):
        tot += 1
        ok += all(perm[2 * i] == perm[2 * i + 1] for i in range(e))
    return ok / tot

def brute_frame(counts, f):
    slots = [c for c, n in enumerate(counts) for _ in range(n)]
    perms = set(itertools.permutations(slots))
    ok = tot = 0
    for a in perms:
        for b in perms:
            tot += 1
            ok += all(a[i] == b[i] for i in range(f))
    return ok / tot

print("0. brute-force check of the match probabilities (tiny census)")
for counts in ([2, 2, 4], [4, 2], [2, 2, 2, 2]):
    tab = p_int_table(counts)
    print("   P_int  ", counts, ["%.6f/%.6f" % (10 ** tab[e], brute_int(counts, e)) for e in range(sum(counts) // 2 + 1)])
for counts in ([2, 1, 1], [2, 2]):
    tab = p_frame_table(counts)
    print("   P_frame", counts, ["%.6f/%.6f" % (10 ** tab[f], brute_frame(counts, f)) for f in range(sum(counts) + 1)])
# P2 against Monte Carlo on a small two-class census
seam, inner = {1: 2, 2: 1}, {1: 4, 2: 5, 3: 3}
tab = p2_table(seam, inner)
rng = random.Random(1)
sl_s = [c for c, n in seam.items() for _ in range(n)]
sl_i = [c for c, n in inner.items() for _ in range(n)]
hits = {(1, 1): 0, (2, 2): 0, (3, 3): 0, (0, 4): 0}
T = 200000
for _ in range(T):
    rng.shuffle(sl_s); rng.shuffle(sl_i)
    for (s, e) in hits:
        if all(sl_s[i] == sl_i[i] for i in range(s)) and all(sl_i[s + 2 * i] == sl_i[s + 2 * i + 1] for i in range(e)):
            hits[(s, e)] += 1
print("   P2 exact / Monte Carlo:", {k: "%.5f/%.5f" % (10 ** tab[k[0]][k[1]], v / T) for k, v in hits.items()})

# --- 1. official set ----------------------------------------------------------
pieces, hints = load_official()
cen = census(pieces)
print("\n1. census: kinds (interior, edge, corner) =", cen["kinds"], "hints (row, col) =", hints)
print("   frame joints per colour:", sorted(cen["frame"].values()), " inner sides per colour:", sorted(cen["inner"].values()))
print("   seam (inward) sides per colour:", sorted(cen["seam"].values()))
M = Model(16, cen)
p2 = M.p2()
print("   log10 P_int(420) = %.3f, log10 P_frame(60) = %.3f, log10 P2(56,364) = %.3f   [%.0fs]" % (M.lp_int[420], M.lp_fr[60], p2[56][364], time.time() - t0))
print("   per-joint cost in digits, pooled interior, joint number 1, 100, 200, 300, 400, 415, 420:",
      ["%.3f" % (M.lp_int[e - 1] - M.lp_int[e]) for e in (1, 100, 200, 300, 400, 415, 420)])
print("   per-joint cost in digits, frame, joint number 1, 15, 30, 45, 55, 60:",
      ["%.3f" % (M.lp_fr[f - 1] - M.lp_fr[f]) for f in (1, 15, 30, 45, 55, 60)])
print("   spread values: interior %.4f digits per joint (1/%.2f), frame %.4f (1/%.2f)"
      % (-M.lp_int[420] / 420, 10 ** (-M.lp_int[420] / 420), -M.lp_fr[60] / 60, 10 ** (-M.lp_fr[60] / 60)))

centre = [h for h in hints if 5 <= h[0] <= 10 and 5 <= h[1] <= 10]
orders = {"r=0 first": rows_from(16, True), "r=15 first": rows_from(16, False)}
print("\n2. row scan, expected valid partial boards. total = sum over free-cell levels; '+hint levels' also counts the 1 or 5 levels at hint cells")
print("   %-12s %-11s %-13s %9s %9s %6s %9s %10s" % ("clues", "order", "model", "log10 tot", "peak", "depth", "+hint lv", "log10 sols"))
res = {}
for cname, hs in (("centre", centre), ("five", hints)):
    for oname, o in orders.items():
        for mode in ("pooled", "class", "spread-pooled", "spread-class"):
            pr = M.profile(o, hs, mode)
            pr2 = M.profile(o, hs, mode, count_hint_levels=True)
            pk = max(pr)
            # depth in board cells (hints included) at the peak
            free = [c for c in o if c not in set(hs)]
            cellpk = free[pr.index(pk)]
            dpk = o.index(cellpk) + 1
            res[(cname, oname, mode)] = pr
            print("   %-12s %-11s %-13s %9.2f %9.2f %6d %9.2f %10.3f" % (cname, oname, mode, logsum(pr), pk, dpk, logsum(pr2), pr[-1]))

print("\n3. decomposition of path 2's 10^45.3 (five clues). Each line changes one thing.")
a = logsum(res[("five", "r=0 first", "spread-class")])
b = logsum(res[("five", "r=0 first", "class")])
c = logsum(res[("five", "r=0 first", "pooled")])
d = logsum(res[("five", "r=15 first", "pooled")])
print("   path 2 as written (class-aware census, match probability spread evenly, r=0 first): 10^%.2f = 2^%.1f" % (a, a / log10(2)))
print("   same census and order, exact probability of the joints actually closed:               10^%.2f   (spreading error %+.2f digits)" % (b, a - b))
print("   pooled census (Owen), same order:                                                     10^%.2f   (census choice %+.2f digits)" % (c, b - c))
print("   pooled census, r=15 first (the order of paths 5 and 3):                               10^%.2f   (scan direction %+.2f digits)" % (d, c - d))
print("   class-aware census, exact, r=15 first:                                                10^%.2f" % logsum(res[("five", "r=15 first", "class")]))
pr_s, pr_e = res[("five", "r=0 first", "spread-class")], res[("five", "r=0 first", "class")]
print("   spread minus exact (digits) after free cells 16, 48, 96, 128, 160, 192, 224, 240, 251:",
      ["%.2f" % (pr_s[k - 1] - pr_e[k - 1]) for k in (16, 48, 96, 128, 160, 192, 224, 240, 251)])

print("\n4. per-depth profile, five clues, r=15 first, pooled exact (log10 width after k free cells)")
pr = res[("five", "r=15 first", "pooled")]
print("   ", {k: round(pr[k - 1], 2) for k in (16, 32, 48, 64, 80, 96, 112, 128, 144, 160, 176, 192, 208, 224, 240, 251)})
tot = logsum(pr)
cum = 0.0
lo = hi = None
for i, x in enumerate(pr):
    cum += 10 ** (x - tot)
    if lo is None and cum > 0.01: lo = i + 1
    if hi is None and cum > 0.99: hi = i + 1
print("    98%% of nodes between free-cell depths %d and %d; first solution (half the tree) 10^%.2f" % (lo, hi, tot - log10(2)))
for mode in ("pooled", "class"):
    p1 = res[("centre", "r=15 first", mode)]
    p5 = res[("five", "r=15 first", mode)]
    print("    %-7s one clue: total 10^%.2f, solutions %.0f, nodes per solution 10^%.2f ; five clues: total 10^%.2f, other solutions %.2e"
          % (mode, logsum(p1), 10 ** p1[-1], logsum(p1) - p1[-1], logsum(p5), 10 ** p5[-1]))
print("    bits: five-clue tree 2^%.1f ; 48 core-years at 1e8/s = 2^%.1f ; 1,000 core-years = 2^%.1f"
      % (tot / log10(2), log10(48 * 3.15576e15) / log10(2), log10(1000 * 3.15576e15) / log10(2)))
print("[%.0fs]" % (time.time() - t0))
