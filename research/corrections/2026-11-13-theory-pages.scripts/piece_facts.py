"""Bag facts behind the 2026-11-13 theory-page corrections.

Reads engine/data/official_eternity2.csv (top, right, bottom, left; 65535 = grey).
Stdlib only, single thread:  python3 -I piece_facts.py

Prints:
  1. near-twin pairs counted in the stored orientation (the old 114) and up to
     rotation, by kind (interior, edge, corner);
  2. in-place turns of an interior piece that keep exactly two sides, as tiles
     and as (tile, rotation) moves;
  3. how the file stores pieces (is every interior piece's smallest colour on top?);
  4. the fixed-orientation cap sum_c min(E,W) + min(N,S) for the stored file,
     for random orientations, and for a valid orientation of a solution.
"""
import os
import random
import statistics
from itertools import combinations

HERE = os.path.dirname(os.path.abspath(__file__))
CSV = os.path.join(HERE, "..", "..", "..", "engine", "data", "official_eternity2.csv")


def load():
    rows = [l.strip() for l in open(CSV) if l.strip()][1:]
    out = []
    for l in rows:
        c = l.split(",")
        out.append(tuple(0 if int(x, 2) == 65535 else int(x, 2) for x in c[:4]))
    return out


def rot(p, k):
    """quarter turns clockwise: the side at index i moves to index (i + k) % 4"""
    return tuple(p[(i - k) % 4] for i in range(4))


P = load()
kind = [p.count(0) for p in P]  # 0 interior, 1 edge, 2 corner
assert len(P) == 256 and kind.count(0) == 196 and kind.count(1) == 56 and kind.count(2) == 4

# 1. near-twins
stored_all = 0
by_kind = {0: 0, 1: 0, 2: 0}
for a, b in combinations(range(256), 2):
    pa, pb = P[a], P[b]
    if sum(x == y for x, y in zip(pa, pb)) == 3:
        stored_all += 1
    if kind[a] != kind[b]:
        continue
    for k in range(4):
        q = rot(pb, k)
        # grey must sit on grey (a legal swap keeps the rim grey)
        if all((x == 0) == (y == 0) for x, y in zip(pa, q)) and sum(x == y for x, y in zip(pa, q)) == 3:
            by_kind[kind[a]] += 1
            break
print("near-twin pairs, stored orientation (3 of 4 same-position sides equal):", stored_all)
print("near-twin pairs up to rotation, same kind, grey on grey: interior %d, edge %d, corner %d, total %d"
      % (by_kind[0], by_kind[1], by_kind[2], sum(by_kind.values())))

# 2. turns in place keeping two sides
tiles, moves = set(), 0
for i, p in enumerate(P):
    if kind[i]:
        continue
    for k in (1, 2, 3):
        if sum(x == y for x, y in zip(p, rot(p, k))) == 2:
            tiles.add(i)
            moves += 1
print("interior tiles with a two-side-keeping turn: %d tiles, %d (tile, rotation) moves" % (len(tiles), moves))
print("single-move boards at score 478 around any solution, floor from the bag: %d + %d = %d"
      % (sum(by_kind.values()), moves, sum(by_kind.values()) + moves))

# 3. storage convention
smallest_on_top = sum(1 for i, p in enumerate(P) if kind[i] == 0 and p[0] == min(p))
edge_grey_top = sum(1 for i, p in enumerate(P) if kind[i] == 1 and p[0] == 0)
print("interior pieces stored with their smallest colour on top: %d of 196" % smallest_on_top)
print("edge pieces stored with grey on top: %d of 56" % edge_grey_top)


# 4. fixed-orientation cap
def cap(pieces):
    from collections import Counter
    n, e, s, w = Counter(), Counter(), Counter(), Counter()
    for t, r, b, l in pieces:
        n[t] += 1
        e[r] += 1
        s[b] += 1
        w[l] += 1
    cols = (set(n) | set(e) | set(s) | set(w)) - {0}
    return sum(min(e[c], w[c]) + min(n[c], s[c]) for c in cols)


print("fixed-orientation cap, stored file:", cap(P))
rng = random.Random(20261113)
caps_int = [cap([rot(p, rng.randrange(4)) if kind[i] == 0 else p for i, p in enumerate(P)]) for _ in range(2000)]
caps_all = [cap([rot(p, rng.randrange(4)) for p in P]) for _ in range(2000)]
print("random orientation of the 196 interior pieces, border as stored (2000 draws): mean %.1f sd %.1f min %d max %d"
      % (statistics.mean(caps_int), statistics.pstdev(caps_int), min(caps_int), max(caps_int)))
print("random orientation of all 256 pieces (2000 draws): mean %.1f sd %.1f min %d max %d"
      % (statistics.mean(caps_all), statistics.pstdev(caps_all), min(caps_all), max(caps_all)))
print("a solution's orientations: every one of the 480 joints matches, so the cap is 480")
