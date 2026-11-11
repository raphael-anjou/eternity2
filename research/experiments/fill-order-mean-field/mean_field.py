#!/usr/bin/env python3
"""Mean-field candidate law vs exact enumeration on the official piece set,
plus k-profiles (placed neighbours at visit time) of row scan and spiral on a
bordered 14x14 interior. Deterministic (seeded); runs in a few minutes.

Usage (repo root): python3 research/experiments/fill-order-mean-field/mean_field.py
"""
import json, random
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
OUT = Path(__file__).resolve().parent / "results" / "mean-field.json"
GRAY = "1111111111111111"

rows = (ROOT / "engine/data/official_eternity2.csv").read_text().split("\n")[1:]
pieces = [r.split(",")[:4] for r in rows if r.strip()]
interior = [p for p in pieces if GRAY not in p]

def sum_f2(sides):
    c = Counter(sides); t = sum(c.values())
    return len(c), sum((v / t) ** 2 for v in c.values())

n_all, p_all = sum_f2([s for p in pieces for s in p if s != GRAY])
n_int, p_int = sum_f2([s for p in interior for s in p])
tuples = {tuple(p[r:] + p[:r]) for p in interior for r in range(4)}
R = len(interior)

def trial(rng, k):
    """Exact candidate count for a cell with k placed neighbours at the start of
    interior fill: each neighbour is a random interior piece in a random rotation."""
    used = rng.sample(range(R), k)
    req = {}
    for side, u in zip(range(k), used):
        p = interior[u]; r = rng.randrange(4); rp = p[r:] + p[:r]
        req[side] = rp[(side + 2) % 4]
    cnt = 0
    for i, p in enumerate(interior):
        if i in used:
            continue
        for r in range(4):
            rp = p[r:] + p[:r]
            if all(rp[s] == c for s, c in req.items()):
                cnt += 1
    return cnt

rng = random.Random(1)
law = []
for k in (1, 2, 3, 4):
    n = 20000 if k < 3 else 100000
    counts = [trial(rng, k) for _ in range(n)]
    law.append({
        "k": k, "trials": n,
        "exact_mean": sum(counts) / n,
        "exact_dead_fraction": sum(c == 0 for c in counts) / n,
        "pred_p_interior": 4 * R * p_int ** k,
        "pred_p_all22": 4 * R * p_all ** k,
    })

def k_profile(order, size, bordered):
    placed = set(); hist = Counter()
    for (r, c) in order:
        k = 0
        for dr, dc in ((-1, 0), (1, 0), (0, -1), (0, 1)):
            rr, cc = r + dr, c + dc
            inside = 0 <= rr < size and 0 <= cc < size
            if (inside and (rr, cc) in placed) or (bordered and not inside):
                k += 1
        hist[k] += 1; placed.add((r, c))
    return {str(k): hist[k] for k in range(5)}

def spiral(size):
    out, top, left, bot, right = [], 0, 0, size - 1, size - 1
    while top <= bot and left <= right:
        out += [(top, c) for c in range(left, right + 1)]
        out += [(r, right) for r in range(top + 1, bot + 1)]
        if top < bot:
            out += [(bot, c) for c in range(right - 1, left - 1, -1)]
        if left < right:
            out += [(r, left) for r in range(bot - 1, top, -1)]
        top, left, bot, right = top + 1, left + 1, bot - 1, right - 1
    return out

row_scan = [(r, c) for r in range(14) for c in range(14)]
profiles = {
    "bordered14_row_scan": k_profile(row_scan, 14, True),
    "bordered14_spiral_in": k_profile(spiral(14), 14, True),
    "free16_row_scan": k_profile([(r, c) for r in range(16) for c in range(16)], 16, False),
}

res = {
    "interior_pieces": R, "interior_rotation_tuples": len(tuples),
    "colours_all": n_all, "p_all22": p_all,
    "colours_interior": n_int, "p_interior": p_int,
    "law": law, "k_profiles": profiles, "seed": 1,
}
OUT.write_text(json.dumps(res, indent=2) + "\n")
print(json.dumps(res, indent=2))
