"""Second analysis replacement for Lemma B, checked on a toy and on the real tiles. Stdlib only.
Run: python3 -I r_lemmaB.py <official.json>
Statement B': write the colour-compatibility tensor G(placement of every cell) = prod over internal edges
[the two sides match] as a sum of m terms, each a product of one vector per cell, over any field, signs and
cancellation allowed. Then m >= L, the number of edge colourings for which every cell has a fitting placement.
Proof: flatten white cells against black cells of the checkerboard. Every edge joins a white and a black cell,
so a white placement fixes the whole colouring: the matrix is block diagonal with all-ones blocks, rank L.
The toy below computes that rank directly. TOY BOARD, generated, not Eternity II.
The hole that remains: G only has to be right on injective placements (the determinant never reads the rest).
Restricted to injective placements the same flattening has rank at most the number of solutions."""
import json, sys, random
from itertools import product

def rank_mod(rows, P=1000003):
    rows = [r[:] for r in rows]; rk = 0
    for c in range(len(rows[0])):
        piv = next((i for i in range(rk, len(rows)) if rows[i][c]), None)
        if piv is None: continue
        rows[rk], rows[piv] = rows[piv], rows[rk]
        inv = pow(rows[rk][c], P - 2, P); rows[rk] = [x * inv % P for x in rows[rk]]
        for i in range(len(rows)):
            if i != rk and rows[i][c]:
                f = rows[i][c]; rows[i] = [(x - f * y) % P for x, y in zip(rows[i], rows[rk])]
        rk += 1
    return rk

print("toy 2x2 block, no rim, 3 colours, pools of 4 distinct tiles (generated)")
for seed in range(4):
    random.seed(seed); C = 3
    tiles = set()
    while len(tiles) < 4:
        t = tuple(random.randrange(1, C + 1) for _ in range(4))
        if all(t[k:] + t[:k] not in tiles for k in range(4)): tiles.add(t)
    tiles = sorted(tiles)
    pl = [(i, t[4 - k:] + t[:4 - k]) for i, t in enumerate(tiles) for k in range(4)]     # URDL
    # cells: a=(0,0) white, b=(0,1) black, c=(1,0) black, d=(1,1) white. Edges: a.R-b.L, a.D-c.U, b.D-d.U, c.R-d.L
    def ok(a, b, c, d): return a[1][1] == b[1][3] and a[1][2] == c[1][0] and b[1][2] == d[1][0] and c[1][1] == d[1][3]
    W = list(product(pl, pl)); B = list(product(pl, pl))
    Mx = [[1 if ok(a, b, c, d) else 0 for (b, c) in B] for (a, d) in W]
    L = 0
    for x in product(range(1, C + 1), repeat=4):       # colours of the four edges ab, ac, bd, cd
        fit = (any(p[1][1] == x[0] and p[1][2] == x[1] for p in pl) and any(p[1][3] == x[0] and p[1][2] == x[2] for p in pl)
               and any(p[1][0] == x[1] and p[1][1] == x[3] for p in pl) and any(p[1][0] == x[2] and p[1][3] == x[3] for p in pl))
        L += fit
    # the same matrix with the entries the determinant never reads set aside: block (white tile set A) x (black set = rest)
    best = 0; sols = 0
    blocks = {}
    for wi, (a, d) in enumerate(W):
        if a[0] == d[0]: continue
        for bi, (b, c) in enumerate(B):
            if {b[0], c[0]} == set(range(4)) - {a[0], d[0]}:
                blocks.setdefault(frozenset((a[0], d[0])), {}).setdefault(wi, {})[bi] = Mx[wi][bi]; sols += Mx[wi][bi]
    for A, rws in blocks.items():
        cols = sorted({bi for r in rws.values() for bi in r})
        best = max(best, rank_mod([[r.get(bi, 0) for bi in cols] for r in rws.values()]))
    print("  seed %d: flattening rank %d, colourings with a fitting placement in every cell L = %d, all colourings %d ; "
          "strict fillings %d, largest rank of a specified block once repeats are set aside: %d"
          % (seed, rank_mod(Mx), L, C ** 4, sols, best))

T = [tuple(p) for p in json.load(open(sys.argv[1]))["pieces"]]
inner = [t for t in T if 0 not in t]
half1 = {c for t in inner[0::2] for c in t}; half2 = {c for t in inner[1::2] for c in t}
print("real tiles: colours shown by every other interior tile: %d, by the rest: %d, common: %d" % (len(half1), len(half2), len(half1 & half2)))
print("  so the edge matrix restricted to rows from one half and columns from the other (no same-tile entry) has rank %d:"
      " leaving same-tile pairs unspecified does not lower the 17 indices per interior edge" % len(half1 & half2))
