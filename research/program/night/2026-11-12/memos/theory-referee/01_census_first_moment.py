"""Colour census of the official set and the first-moment solution counts."""
import math, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from collections import Counter
from e2lib import load, kind, census
n, P, H = load()
print("n", n, "pieces", len(P), "hints", H)
print("kinds", Counter(kind(p) for p in P))
lat, inn = census(P)
print("lateral (frame-ring) half-edges by colour:", dict(sorted(lat.items())), "total", sum(lat.values()))
print("interior half-edges by colour:", dict(sorted(inn.items())), "total", sum(inn.values()))
# clockwise/anticlockwise split of lateral colours
cw, ccw = Counter(), Counter()
for p in P:
    k = kind(p)
    if k == "edge":
        g = p.index(0)
        cw[p[(g + 1) % 4]] += 1; ccw[p[(g - 1) % 4]] += 1
    elif k == "corner":
        # grey sides are adjacent: g and g+1
        g = [i for i in range(4) if p[i] == 0 and p[(i + 1) % 4] == 0][0]
        cw[p[(g + 2) % 4]] += 1; ccw[p[(g - 1) % 4]] += 1
print("cw", dict(sorted(cw.items())), "ccw", dict(sorted(ccw.items())))
frame_cols = set(lat); int_cols = set(inn)
print("frame colours", len(frame_cols), "interior colours", len(int_cols), "overlap", frame_cols & int_cols)
lg = lambda x: math.lgamma(x + 1)
def ldf(m):  # log (m)!! for odd m
    return sum(math.log(k) for k in range(m, 0, -2))
L10 = math.log(10)
arr = lg(4) + lg(56) + lg(195) + 195 * math.log(4)
naive = arr - 60 * math.log(5) - 420 * math.log(17)
print("naive product (starter pinned):", math.exp(naive))
print("naive * 2^20:", math.exp(naive) * 2 ** 20, " ratio 14702.3/naive = 2^", math.log2(14702.3 / math.exp(naive)))
ring = sum(lg(cw[c]) for c in cw) - lg(60)
interior = sum(ldf(inn[c] - 1) for c in inn) - ldf(839)
E1 = arr + ring + interior
print("exact-census first moment, starter pinned:", math.exp(E1))
ring_free = sum(ldf(lat[c] - 1) for c in lat) - ldf(119)
print("with frame half-edges paired freely:", math.exp(arr + ring_free + interior))
E0 = E1 + math.log(196 * 4)
print("no starter:", math.exp(E0), " ratio", math.exp(E0 - E1))
# all five clues: crude version, pin 5 interior pieces => 191! 4^191
E5 = lg(4) + lg(56) + lg(191) + 191 * math.log(4) + ring + interior
print("five clues pinned (same matching probability, not conditional):", math.exp(E5), "ratio to starter", math.exp(E5 - E1))
# frame count first moment: ring edges only
fr = lg(4) + lg(56) + ring
print("frame-ring first moment (60 ring edges only): 10^", fr / L10)
# with rotations/symmetry? 4 corners fixed positions.
# 3x3 toy
for c in (2, 3): print("toy 3x3 c=", c, 2304 / c ** 12)
print("poisson P(X<=2 | 3.2) =", math.exp(-3.2) * (1 + 3.2 + 3.2 ** 2 / 2))
print("planted: 1+Poisson(2.2) <=2 :", math.exp(-2.2) * (1 + 2.2))
print("log2 of 7e5", math.log2(14702.3 / math.exp(naive)))
print("2.2e4 / 14702 =", math.exp(naive) * 2 ** 20 / 14702.3)
print("9.2766e42/9.2751e42", 9.2766e42 / 9.2751e42, "1.1527e7/14702", 1.1527e7 / 14702)
