"""Claim C follow-up: how cheap can the interior be made under a given frame,
in the model (five clues on the board)? Run: python3 -I 06_interior_orders.py"""
import os
import sys
import time
from math import log10

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rv_lib import *

t0 = time.time()
pieces, hints = load_official()
M = Model(16, census(pieces))
N = 16
fr = ring(N)
hs = set(hints)
inner = [(r, c) for r in range(1, N - 1) for c in range(1, N - 1) if (r, c) not in hs]
lp = M.lp_int
base = M.profile(fr, hints)[-1]


def run(order):
    """log10 widths relative to the number of frames; all frame joints closed."""
    placed = set(fr) | hs
    e = 0
    # joints already closed by frame + hints: frame-internal only (no hint touches the frame)
    e0 = None
    left = 196 - len(hs)
    la = 0.0
    out = []
    for cell in order:
        la += log10(left) + log10(4)
        left -= 1
        r, c = cell
        e += sum(((r + a, c + b) in placed) for a, b in ((1, 0), (-1, 0), (0, 1), (0, -1)))
        placed.add(cell)
        out.append(la + lp[e])
    return out


def greedy_width():
    placed = set(fr) | hs
    left = list(inner)
    order = []
    while left:
        # the next width depends only on how many joints the new cell closes: take the max
        best = max(left, key=lambda x: sum(((x[0] + a, x[1] + b) in placed) for a, b in ((1, 0), (-1, 0), (0, 1), (0, -1))))
        k = sum(((best[0] + a, best[1] + b) in placed) for a, b in ((1, 0), (-1, 0), (0, 1), (0, -1)))
        # tie-break: among cells closing k joints prefer one adjacent to the last placed cell, else nearest to a hint
        ties = [x for x in left if sum(((x[0] + a, x[1] + b) in placed) for a, b in ((1, 0), (-1, 0), (0, 1), (0, -1))) == k]
        if order:
            lr, lc = order[-1]
            ties.sort(key=lambda x: (abs(x[0] - lr) + abs(x[1] - lc), min(abs(x[0] - h[0]) + abs(x[1] - h[1]) for h in hints)))
        else:
            ties.sort(key=lambda x: min(abs(x[0] - h[0]) + abs(x[1] - h[1]) for h in hints))
        best = ties[0]
        order.append(best)
        placed.add(best)
        left.remove(best)
    return order


def spiral(start_corner=0, clockwise=True):
    o, lo, hi = [], 1, N - 2
    while lo <= hi:
        ringc = [(lo, c) for c in range(lo, hi + 1)] + [(r, hi) for r in range(lo + 1, hi + 1)]
        ringc += [(hi, c) for c in range(hi - 1, lo - 1, -1)] + [(r, lo) for r in range(hi - 1, lo, -1)]
        o += ringc
        lo += 1
        hi -= 1
    seen, out = set(), []
    for x in o:
        if x not in seen and x not in hs:
            seen.add(x)
            out.append(x)
    return out


cands = {"rows r=14 first": sorted(inner, key=lambda x: (-x[0], x[1])),
         "spiral inward": spiral(),
         "greedy: most closed joints, stay local": greedy_width()}
# model hill climb on the spiral: move one cell, 20 s
import random
rng = random.Random(1)
cur = spiral()
ct = logsum(run(cur))
it = 0
while time.time() - t0 < 25:
    it += 1
    i = rng.randrange(len(cur))
    j = max(0, min(len(cur) - 1, i + rng.randint(-30, 30)))
    if i == j:
        continue
    o = cur[:]
    o.insert(j, o.pop(i))
    t = logsum(run(o))
    if t < ct - 1e-9:
        cur, ct = o, t
cands["spiral + hill climb (%d proposals)" % it] = cur
exact_frames = log10(40519056449429419697492635191544381440)
row = 40.49
for name, o in cands.items():
    w = run(o)
    s = logsum(w)
    print("%-42s mean interior tree per frame: peak 10^%.2f sum 10^%.2f | x exact frame count = 10^%.2f | against row scan 10^%.2f: %+.1f digits"
          % (name, max(w), s, exact_frames + s, row, exact_frames + s - row))
print("model frame count 10^%.2f; completions per frame 10^%.2f   [%.0fs]" % (base, run(cands["spiral inward"])[-1], time.time() - t0))
