"""Merge factor of small regions on the REAL Eternity II tiles (exact enumeration). Stdlib only.
Run: python3 -I merge_real.py <official.json> [region ...]
A 'merge' is two distinct strict fillings of the same region with the same tile set and the same
outline colours. The used-set transfer matrix, the exterior-algebra sweep and counting modulo a prime
can only save on merges, so this measures what those devices can save locally on the real set.
Regions: all cells are filled with distinct tiles, internal edges match, rim sides are grey (0),
non-rim sides are never grey. Tiles are URDL."""
import json, sys, time
from collections import defaultdict

P = [tuple(p) for p in json.load(open(sys.argv[1]))["pieces"]]
ROT = []                                   # (tile, rotation, (U,R,D,L)); identical-looking rotations kept once
sym = 0
for i, p in enumerate(P):
    seen = set()
    for k in range(4):
        q = p[k:] + p[:k]
        if q in seen: sym += 1; continue
        seen.add(q); ROT.append((i, k, q))
print("tiles %d, oriented tiles %d, rotations dropped as identical (symmetric tiles): %d" % (len(P), len(ROT), sym))
canon = defaultdict(list)
for i, p in enumerate(P): canon[min(p[k:] + p[:k] for k in range(4))].append(i)
print("tiles equal to another tile up to rotation: %d" % sum(len(v) for v in canon.values() if len(v) > 1))

REGIONS = {  # name: (cells in row-major order, set of (cell, side) that are rim)
    "interior 1x2": ([(0, 0), (0, 1)], set()),
    "interior 1x3": ([(0, 0), (0, 1), (0, 2)], set()),
    "interior L3": ([(0, 0), (0, 1), (1, 0)], set()),
    "interior 2x2": ([(0, 0), (0, 1), (1, 0), (1, 1)], set()),
    "bottom-rim 1x2": ([(0, 0), (0, 1)], {((0, 0), 2), ((0, 1), 2)}),
    "bottom-rim 1x3": ([(0, 0), (0, 1), (0, 2)], {((0, 0), 2), ((0, 1), 2), ((0, 2), 2)}),
    "bottom-rim 1x4": ([(0, j) for j in range(4)], {((0, j), 2) for j in range(4)}),
    "bottom-rim 2x2": ([(0, 0), (0, 1), (1, 0), (1, 1)], {((1, 0), 2), ((1, 1), 2)}),
    "bottom-rim 2x3": ([(i, j) for i in range(2) for j in range(3)], {((1, j), 2) for j in range(3)}),
    "corner 2x2": ([(0, 0), (0, 1), (1, 0), (1, 1)], {((1, 0), 2), ((1, 1), 2), ((0, 0), 3), ((1, 0), 3)}),
    "corner 2x3": ([(i, j) for i in range(2) for j in range(3)],
                   {((1, j), 2) for j in range(3)} | {((0, 0), 3), ((1, 0), 3)}),
    "corner 3x3": ([(i, j) for i in range(3) for j in range(3)],
                   {((2, j), 2) for j in range(3)} | {((i, 0), 3) for i in range(3)}),
}
DR = [(-1, 0), (0, 1), (1, 0), (0, -1)]

def run(name):
    cells, rim = REGIONS[name]
    idx = {c: n for n, c in enumerate(cells)}
    # per cell: zero pattern, earlier-neighbour constraints, outline sides
    spec = []
    for n, c in enumerate(cells):
        zero = tuple((c, s) in rim for s in range(4))
        cons = []
        for s in range(4):
            nb = (c[0] + DR[s][0], c[1] + DR[s][1])
            if nb in idx and idx[nb] < n: cons.append((s, idx[nb], (s + 2) % 4))
        out = [s for s in range(4) if not zero[s] and (c[0] + DR[s][0], c[1] + DR[s][1]) not in idx]
        cand = [(i, q) for (i, k, q) in ROT if all((q[s] == 0) == zero[s] for s in range(4))]
        # index candidates by the colours on constrained sides
        tab = defaultdict(list)
        for i, q in cand: tab[tuple(q[s] for s, _, _ in cons)].append((i, q))
        spec.append((cons, out, tab))
    states = {}; examples = []; first = {}; keep_first = len(cells) <= 3 or bool(rim)
    t0 = time.time(); total = 0
    placed = [None] * len(cells)
    def rec(n, used):
        nonlocal total
        if n == len(cells):
            total += 1
            k = 0                               # exact injective integer key: sorted tile ids, then outline
            for i in sorted(i for i, _ in placed): k = k * 256 + i
            for m in range(len(cells)):
                for s_ in spec[m][1]: k = k * 23 + placed[m][1][s_]
            if k in states:
                states[k] += 1
                if len(examples) < 3: examples.append((k, tuple(placed)))
            else:
                states[k] = 1
                if keep_first: first[k] = tuple(placed)
            return
        cons, out, tab = spec[n]
        for i, q in tab.get(tuple(placed[m][1][t] for _, m, t in cons), ()):
            if i in used: continue
            placed[n] = (i, q); used.add(i); rec(n + 1, used); used.discard(i)
    rec(0, set())
    multi = sum(v for v in states.values() if v > 1)
    even = sum(1 for v in states.values() if v % 2 == 0)
    print("%-16s fillings %11d  distinct (tile set, outline) %11d  merge factor %.9f  fillings in merged states %d"
          "  states vanishing mod 2: %d  (%.0f s)" % (name, total, len(states), total / max(1, len(states)),
                                                      multi, even, time.time() - t0), flush=True)
    for k, pl in examples:
        print("     example: %s  and  %s" % (first.get(k, "(first filling not kept)"), pl))
names = sys.argv[2:] or list(REGIONS)
for nm in names:
    run(nm)
