"""First-moment row-scan profile of the official 16x16, exact-census seam weights.

Run: python3 -I profile.py
Ring seams pair a clockwise lateral side with a counter-clockwise one: the
matched fraction is (12!)^5 / 60!. Other seams pair any two of the 840
half-edges (12 colours x 50, 5 colours x 48): prod (h-1)!! / 839!!.
Each family's log-probability is spread evenly over its seams.
Census source: memos/theory-referee/01_census_first_moment.out.
"""
import math
lg = lambda x: math.lgamma(x + 1) / math.log(10)          # log10 x!
dfact = lambda h: lg(h) - (h // 2) * math.log10(2) - lg(h // 2)  # log10 (h-1)!!
ring = (5 * lg(12) - lg(60)) / 60
inner = (12 * dfact(50) + 5 * dfact(48) - dfact(840)) / 420
n = 16
c = e = i = 0
s = 0.0
prof = []
for r in range(n):
    for j in range(n):
        br, bc = r in (0, n - 1), j in (0, n - 1)
        if br and bc:
            s += math.log10(4 - c); c += 1
        elif br or bc:
            s += math.log10(56 - e); e += 1
        else:
            s += math.log10((196 - i) * 4); i += 1
        for rr, jj in ((r, j - 1), (r - 1, j)):
            if rr < 0 or jj < 0:
                continue
            b1 = r in (0, n - 1) or j in (0, n - 1)
            b2 = rr in (0, n - 1) or jj in (0, n - 1)
            s += ring if (b1 and b2) else inner
        prof.append(s)
tot = math.log10(sum(10 ** x for x in prof))
print(f"per-seam log10 weight: ring {ring:.4f} (1/{10**-ring:.2f}), other {inner:.4f} (1/{10**-inner:.2f})")
print(f"boards, no clue: 10^{prof[-1]:.2f}; with the starter pinned: {10**prof[-1]/784:.0f}")
print(f"peak 10^{max(prof):.2f} at cell {prof.index(max(prof))}; tree 10^{tot:.2f}; nodes per solution 10^{tot-prof[-1]:.2f}")
# interior-only hump: product of (r*4*p) while > 1
p = 10 ** inner
h = sum(math.log10(r * 4 * p * p) for r in range(1, 197) if r * 4 * p * p > 1)
print(f"interior hump (branching r*4*p^2 > 1 for r > {1/(4*p*p):.1f}): 10^{h:.2f}")
print(f"relaxed interior tilings per frame (repeats allowed): 10^{196*math.log10(784)+420*inner:.1f}; "
      f"all-different fraction 196!/196^196 = 10^{lg(196)-196*math.log10(196):.1f}; 2^196 = 10^{196*math.log10(2):.1f}")
