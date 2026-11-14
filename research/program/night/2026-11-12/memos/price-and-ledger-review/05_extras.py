"""Small follow-ups. (1) interior orders under a given frame; (2) path 2's
18-hint lattices and greedy placements with exact probabilities; (3) labels of
the peak depth. Run: python3 -I 05_extras.py"""
import os
import sys
from math import log10

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rv_lib import *

pieces, hints = load_official()
M = Model(16, census(pieces))
N = 16
fr = ring(N)
inner = [(r, c) for r in range(1, N - 1) for c in range(1, N - 1)]


def spiral():
    o, lo, hi = [], 1, N - 2
    while lo <= hi:
        o += [(lo, c) for c in range(lo, hi + 1)] + [(r, hi) for r in range(lo + 1, hi + 1)]
        o += [(hi, c) for c in range(hi - 1, lo - 1, -1)] + [(r, lo) for r in range(hi - 1, lo, -1)]
        lo += 1
        hi -= 1
    seen, out = set(), []
    for x in o:
        if x not in seen:
            seen.add(x)
            out.append(x)
    return out


def greedy():
    placed = set(fr) | set(hints)
    left = [c for c in inner if c not in placed]
    out = []
    while left:
        best = max(left, key=lambda x: (sum(((x[0] + a, x[1] + b) in placed) for a, b in ((1, 0), (-1, 0), (0, 1), (0, -1))), x[0], -x[1]))
        out.append(best)
        placed.add(best)
        left.remove(best)
    return out


print("1. mean interior tree per valid frame, five clues on the board, by interior order")
for name, io in (("rows r=1 first", inner), ("rows r=14 first", inner[::-1]), ("columns c=1 first", sorted(inner, key=lambda x: (x[1], x[0]))),
                 ("spiral inward", spiral()), ("most placed neighbours first (ties: lowest row)", greedy())):
    pr = M.profile(fr + io, hints)
    print("   %-48s peak 10^%.2f sum 10^%.2f" % (name, max(pr[60:]) - pr[59], logsum(pr[60:]) - pr[59]))

print("\n2. path 2 section 2.2 redone with the exact probability of the joints closed (class-aware census, r=0 first)")
td = rows_from(N, True)
base = logsum(M.profile(td, hints, "class"))
print("   five clues: 2^%.1f ; three desktops (2^57.1) short by %.1f bits ; 1,000 core-years (2^61.5) short by %.1f bits" % (base / log10(2), base / log10(2) - 57.1, base / log10(2) - 61.5))
hs = list(hints)
for extra in range(1, 13):
    best = None
    for c in inner:
        if c in hs or any(abs(c[0] - a) + abs(c[1] - b) == 1 for a, b in hs):
            continue
        s = logsum(M.profile(td, hs + [c], "class"))
        if best is None or s < best[0]:
            best = (s, c)
    hs.append(best[1])
    print("   +%2d correct placements (greedy best cell %s): 2^%.1f" % (extra, best[1], best[0] / log10(2)))
lat = {"rows 1,3,5 x cols 2,5,8,11,14 + (7,2),(7,8),(7,14)": [(r, c) for r in (1, 3, 5) for c in (2, 5, 8, 11, 14)] + [(7, 2), (7, 8), (7, 14)],
       "rows 1,3,5 x cols 1,4,7,10,13 + (7,4),(7,7),(7,10)": [(r, c) for r in (1, 3, 5) for c in (1, 4, 7, 10, 13)] + [(7, 4), (7, 7), (7, 10)],
       "rows 1,5,9,13 x cols 2,5,8,11,14 (first 18)": [(r, c) for r in (1, 5, 9, 13) for c in (2, 5, 8, 11, 14)][:18]}
for name, l in lat.items():
    print("   18-hint lattice %-52s exact 2^%.1f (path 2, spread: 2^%.1f)" % (name, logsum(M.profile(td, l, "class")) / log10(2), logsum(M.profile(td, l, "spread-class")) / log10(2)))

print("\n3. peak label, five clues, r=15 first")
bu = rows_from(N, False)
pr = M.profile(bu, hints)
k = pr.index(max(pr)) + 1
free = [c for c in bu if c not in set(hints)]
print("   peak after %d free cells = board cell number %d in the scan (cell %s); path 5 prints %d" % (k, bu.index(free[k - 1]) + 1, free[k - 1], k + len(hints)))
