"""Shared loader for the referee checks. Stdlib only."""
import os
from collections import Counter
ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), *[".."] * 6))
CSV = os.path.join(ROOT, "engine", "data", "official_eternity2.csv")

def load():
    lines = [l.strip() for l in open(CSV) if l.strip()]
    n = int(lines[0])
    pieces, hints = [], []
    for i, l in enumerate(lines[1:]):
        c = l.split(",")
        e = tuple(0 if int(x, 2) == 65535 else int(x, 2) for x in c[:4])  # top,right,bottom,left
        pieces.append(e)
        if len(c) >= 7:
            x, y, r = int(c[4]), int(c[5]), int(c[6])
            if x or y or r:
                hints.append((y * n + x, i, r))
    return n, pieces, hints

def kind(p):
    g = p.count(0)
    return {2: "corner", 1: "edge", 0: "interior"}[g]

def census(pieces):
    """Return (frame-ring half-edge counts on the lateral sides of border pieces,
    interior half-edge counts, including the inward side of each edge piece)."""
    lateral, inner = Counter(), Counter()
    for p in pieces:
        k = kind(p)
        if k == "interior":
            for c in p: inner[c] += 1
        elif k == "edge":
            g = p.index(0)
            inner[p[(g + 2) % 4]] += 1
            lateral[p[(g + 1) % 4]] += 1
            lateral[p[(g - 1) % 4]] += 1
        else:
            for c in p:
                if c: lateral[c] += 1
    return lateral, inner
