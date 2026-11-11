#!/usr/bin/env python3
"""Null model for the piece-set axes (stdlib only, seeded, deterministic).

Paint a solved 16x16 board at random with the official joint-colour counts
(the 60 frame-ring joints carry 5 colours x 12; the 420 interior joints carry
5 colours x 24 and 12 colours x 25), shuffled uniformly, cut it into pieces,
and measure the same axes as the committed invariants on the real set.

Usage: python3 null_model.py [trials] > ../results/null_model.json
"""
import json, sys, random
from collections import Counter

N = 16
GRAY = 0


def paint(rng):
    # Joint lists: horizontal joints h[r][c] between (r,c) and (r,c+1);
    # vertical joints v[r][c] between (r,c) and (r+1,c).
    ring, inner = [], []
    for r in range(N):
        for c in range(N - 1):
            (ring if r in (0, N - 1) else inner).append(("h", r, c))
    for r in range(N - 1):
        for c in range(N):
            (ring if c in (0, N - 1) else inner).append(("v", r, c))
    assert len(ring) == 60 and len(inner) == 420
    ring_cols = [c for c in range(1, 6) for _ in range(12)]
    inner_cols = [c for c in range(6, 11) for _ in range(24)] + [c for c in range(11, 23) for _ in range(25)]
    rng.shuffle(ring_cols)
    rng.shuffle(inner_cols)
    col = dict(zip(ring, ring_cols))
    col.update(zip(inner, inner_cols))
    pieces = []
    for r in range(N):
        for c in range(N):
            n = col[("v", r - 1, c)] if r > 0 else GRAY
            e = col[("h", r, c)] if c < N - 1 else GRAY
            s = col[("v", r, c)] if r < N - 1 else GRAY
            w = col[("h", r, c - 1)] if c > 0 else GRAY
            pieces.append((n, e, s, w))
    return pieces


def canon(p):
    return min(tuple(p[(i + k) % 4] for i in range(4)) for k in range(4))


def axes(pieces):
    sym = sum(1 for p in pieces if p[0] == p[2] and p[1] == p[3])
    rot = Counter(canon(p) for p in pieces)
    rot_dup_pairs = sum(k * (k - 1) // 2 for k in rot.values())
    ms = Counter(tuple(sorted(p)) for p in pieces)
    ms_pairs = sum(k * (k - 1) // 2 for k in ms.values())
    return sym, rot_dup_pairs, ms_pairs - rot_dup_pairs


def summarize(xs, observed, tail):
    n = len(xs)
    mean = sum(xs) / n
    if tail == "le":
        p = sum(1 for x in xs if x <= observed) / n
    else:
        p = sum(1 for x in xs if x >= observed) / n
    return {"observed_official": observed, "null_mean": mean,
            "null_fraction_" + ("at_or_below" if tail == "le" else "at_or_above") + "_observed": p}


def main():
    trials = int(sys.argv[1]) if len(sys.argv) > 1 else 20000
    rng = random.Random(20261111)
    sym, rot, twin = [], [], []
    for _ in range(trials):
        a, b, c = axes(paint(rng))
        sym.append(a)
        rot.append(b)
        twin.append(c)
    print(json.dumps({
        "model": "random solved 16x16 painted with the official joint-colour counts, shuffled uniformly",
        "trials": trials,
        "seed": 20261111,
        "rotation_symmetric_pieces": summarize(sym, 0, "le"),
        "rotation_duplicate_pairs": summarize(rot, 0, "le"),
        "twin_pairs_same_multiset_not_rotation_equal": summarize(twin, 5, "le"),
        "automatic_in_every_trial": [
            "every colour has an even side count",
            "pairing capacity sums to exactly 480 (zero slack)",
            "frame colours never appear on interior pieces",
        ],
    }, indent=2))


if __name__ == "__main__":
    main()
