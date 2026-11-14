"""Claim B. The only exact counts on the OFFICIAL pieces that the repo holds:
small sub-grids (research/topics/subgrid-placement-counts/results/reference-table.json,
column 'vide' = empty board, no hint) and the full frame
(research/experiments/frame-count-corners). Model against exact.
Run: python3 -I 07_real_set_small_regions.py"""
import json
import os
import sys
from math import log10

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rv_lib import *

pieces, hints = load_official()
M = Model(16, census(pieces))
tab = json.load(open(os.path.join(REPO, "research", "topics", "subgrid-placement-counts", "results", "reference-table.json")))
exact = {r["key"]: r["vide"] for r in tab["rows"]}
regions = {
    "2x2-tl": [(r, c) for r in range(2) for c in range(2)],
    "2x2-side": [(r, c) for r in range(2) for c in range(5, 7)],
    "2x2-middle": [(r, c) for r in range(5, 7) for c in range(5, 7)],
    "3x3-tl": [(r, c) for r in range(3) for c in range(3)],
    "3x3-side": [(r, c) for r in range(3) for c in range(5, 8)],
}
print("region        exact (official set)   model pooled   exact/model   model class-aware   exact/model")
for k, cells in regions.items():
    a = M.profile(cells, ())[-1]
    b = M.profile(cells, (), "class")[-1]
    print("%-12s %20d %14.4g %12.3f %18.4g %12.3f" % (k, exact[k], 10 ** a, exact[k] / 10 ** a, 10 ** b, exact[k] / 10 ** b))
fr = M.profile(ring(16), ())[-1]
ex = 40519056449429419697492635191544381440
print("%-12s %20.6g %14.4g %12.3f   (frame joints only, same in both censuses)" % ("full frame", ex, 10 ** fr, ex / 10 ** fr))
