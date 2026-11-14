"""Gap 6b: on the official tile set, how much does knowing s sides of an interior cell
narrow its tile domain? Checks the convexity table of ledger16.py against the real tiles.
Usage: python3 -I real_domains.py <size_16_official_eternity.csv>
"""
import sys, itertools
from math import log2
from collections import Counter
rows = [l.strip().split(",") for l in open(sys.argv[1]) if "," in l]
tiles = [tuple(int(x, 2) for x in r[:4]) for r in rows]
RIM = max(c for t in tiles for c in t)
inner = [t for t in tiles if RIM not in t]
cols = sorted({c for t in inner for c in t})
print(f"tiles {len(tiles)}, interior {len(inner)}, interior colours seen on interior tiles {len(cols)}")
census = Counter(c for t in inner for c in t)
tot = sum(census.values())
q = sum((v / tot) ** 2 for v in census.values())
print(f"sum p_c^2 on interior tiles {q:.5f} (uniform 17: {1/17:.5f}); effective colours {1/q:.2f}")
place = {(i, t[k:] + t[:k]) for i, t in enumerate(inner) for k in range(4)}
print(f"oriented interior placements {len(place)} (784 if no tile is rotation-symmetric)")
full = Counter(p for _, p in place)
print(f"4-side patterns owned by more than one tile: {sum(1 for v in full.values() if v > 1)}")
for name, idx in (("1 side", (0,)), ("2 adjacent sides", (0, 1)), ("2 opposite sides", (0, 2)), ("3 sides", (0, 1, 2)), ("4 sides", (0, 1, 2, 3))):
    cnt = Counter()
    for i, p in place:
        cnt[tuple(p[j] for j in idx)] += 1
    npat = len(cols) ** len(idx)
    nonempty = len(cnt)
    # exact: probability that a pattern drawn with the census colour frequencies has a non-empty domain
    pw = 0.0
    for pat in cnt:
        w = 1.0
        for c in pat: w *= census[c] / tot
        pw += w
    print(f"{name:18s} patterns {npat:6d}  non-empty {nonempty:5d}  P(non-empty), census-weighted {pw:.4f} = {-log2(pw):5.2f} bits  mean domain when non-empty {len(place)/nonempty:6.2f}")
