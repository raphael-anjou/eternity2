"""Calibration the solution-counting page asks for: census-exact first moment of the Clue 3 puzzle (6x6)
against the exact count 2,195,647,488 quoted from groups.io msg 8168. Also Clue 1 for reference."""
import json, math, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from collections import Counter
from e2lib import ROOT
def df_log(m): return sum(math.log(k) for k in range(m, 0, -2))
rot = lambda p, r: tuple(p[(i - r) % 4] for i in range(4))
for name in ("clue3", "clue1"):
    d = json.load(open(os.path.join(ROOT, "research", "topics", "clue-puzzle-pieces", "results", f"{name}-solution.json")))
    P = [tuple(p) for p in d["solutionURDL"]]; W = d["width"]; Hh = d["height"]
    corners = [p for p in P if p.count(0) == 2]; edges = [p for p in P if p.count(0) == 1]; inner = [p for p in P if 0 not in p]
    out, inn, interior = Counter(), Counter(), Counter()
    for p in edges:
        g = p.index(0); out[p[(g + 1) % 4]] += 1; inn[p[(g - 1) % 4]] += 1; interior[p[(g + 2) % 4]] += 1
    for p in corners:
        g = [i for i in range(4) if p[i] == 0 and p[(i + 1) % 4] == 0][0]; out[p[(g + 2) % 4]] += 1; inn[p[(g - 1) % 4]] += 1
    for p in inner:
        for c in p: interior[c] += 1
    assert out == inn, (out, inn)
    ring = sum(out.values()); half = sum(interior.values())
    logE = (math.lgamma(len(corners) + 1) + math.lgamma(len(edges) + 1) + math.lgamma(len(inner) + 1) + len(inner) * math.log(4)
            + sum(math.lgamma(k + 1) for k in out.values()) - math.lgamma(ring + 1)
            + sum(df_log(k - 1) for k in interior.values()) - df_log(half - 1))
    canon = lambda p: min(rot(p, r) for r in range(4))
    dup = Counter(canon(p) for p in P)
    sym = sum(math.log(sum(rot(p, r) == p for r in range(4))) for p in inner)     # orientations that give the same board
    dupl = sum(math.lgamma(v + 1) for v in dup.values())
    print(f"{name}: {W}x{Hh}, corners {len(corners)}, edges {len(edges)}, interior {len(inner)}; ring colours {dict(out)}; interior half-edges {dict(sorted(interior.items()))}")
    print(f"   labelled first moment: {math.exp(logE):.4g};  duplicate classes {sorted(v for v in dup.values() if v > 1)}, log symmetry {sym:.2f}")
    print(f"   first moment counting distinct boards (divide by duplicate permutations and self-symmetric rotations): {math.exp(logE - dupl - sym):.4g}")
    if name == "clue3":
        ex = 2195647488
        print(f"   exact count quoted on the page: {ex:.4g};  ratio model/exact: {math.exp(logE - dupl - sym) / ex:.3f}  (labelled: {math.exp(logE) / ex:.3g}); model/4 (if the exact count fixes the board orientation): {math.exp(logE - dupl - sym) / 4 / ex:.3f}")
