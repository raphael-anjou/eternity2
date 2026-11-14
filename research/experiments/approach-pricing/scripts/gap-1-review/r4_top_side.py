"""Second analysis script 4 (gap 1 review). Real border pieces.

How many valid frames share one 44-cell bottom arc (bottom row and both side
columns)? The 16 cells left are the top row: 2 corners and 14 edge pieces, 17 frame
joints. Monte Carlo: draw the 2 corners and 14 edge pieces left over at random, draw
the two colours the side columns present to the top corners from the frame colours
in proportion to their use, count the arrangements of the top row whose joints all
match. A uniform draw of the leftover set is an approximation (a real arc leaves a
set that is not uniform), stated in the review. The mean is carried by rare arcs:
the top row is a path that uses every leftover edge piece, so most leftover sets
admit none and the few that admit one admit many.

usage: python3 -I r4_top_side.py DRAWS SEED
"""
import random
import sys

CSV = "engine/data/official_eternity2.csv"
DRAWS, SEED = int(sys.argv[1]), int(sys.argv[2])
pieces = []
for l in [l.strip() for l in open(CSV) if l.strip()][1:]:
    c = l.split(",")
    pieces.append(tuple(0 if int(x, 2) == 65535 else int(x, 2) for x in c[:4]))


def as_top(e):
    """(left colour, right colour) of a border piece seated in the top row, grey up.
    For a corner, the two non-grey sides in clockwise order after the greys."""
    for r in range(4):
        o = tuple(e[(i + 4 - r) % 4] for i in range(4))  # top right bottom left
        if sum(1 for v in e if v == 0) == 1 and o[0] == 0:
            return o[3], o[1]
    return None


edges = [as_top(e) for e in pieces if sum(1 for v in e if v == 0) == 1]
corners = []
for e in pieces:
    if sum(1 for v in e if v == 0) == 2:
        for r in range(4):
            o = tuple(e[(i + 4 - r) % 4] for i in range(4))
            if o[0] == 0 and o[3] == 0:
                corners.append((o[2], o[1]))  # top-left seat: (down, right); top-right seat by symmetry: (left, down) = (right, down) of the mirror
frame_cols = [c for l, r in edges for c in (l, r)]
rng = random.Random(SEED)
assert len(edges) == 56 and len(corners) == 4


def count(tl, tr, pool, down_l, down_r):
    """tl seated top-left: (down, right). tr seated top-right has (left, down): a corner
    stored as (down, right) for the top-left seat sits top-right, turned a quarter, as
    left = its 'down', down = its 'right'."""
    if tl[0] != down_l or tr[1] != down_r:
        return 0
    total = 0

    def rec(col, used, k):
        nonlocal total
        if k == len(pool):
            total += col == tr[0]
            return
        for i, (l, r) in enumerate(pool):
            if not used >> i & 1 and l == col:
                rec(r, used | 1 << i, k + 1)

    rec(tl[1], 0, 0)
    return total


tot, nz, pos = 0, 0, []
for _ in range(DRAWS):
    cs = rng.sample(corners, 2)
    pool = rng.sample(edges, 14)
    dl, dr = rng.choice(frame_cols), rng.choice(frame_cols)
    c = count(cs[0], cs[1], pool, dl, dr) + count(cs[1], cs[0], pool, dl, dr)
    tot += c
    nz += c > 0
    if c:
        pos.append(c)
pos.sort()
print("draws %d: mean number of valid top rows per arc %.3f, arcs with at least one %d (share %.5f)" % (DRAWS, tot / DRAWS, nz, nz / DRAWS))
if pos:
    print("among arcs with at least one: median %d, mean %.0f, smallest %d, largest %d" % (pos[len(pos) // 2], sum(pos) / len(pos), pos[0], pos[-1]))
print("uniform arithmetic for comparison: 14! * 2 / 5^17 = %.3f" % (87178291200 * 2 / 5 ** 17))
