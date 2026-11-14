"""Second analysis script 1. The abelian invariant group, recomputed from the definition, with positive
controls of the re-check's own making.

Definition used. Generators: h_c and v_c for each of the 23 colours c (0 = grey): 46 in all.
A tile (t, r, b, l) placed in a cell has boundary word h_b v_r h_t^-1 v_l^-1 (walk the cell
anticlockwise from its bottom-left corner); its abelian image is h_b - h_t + v_r - v_l.
ROTATIONS ARE FREE: every tile contributes all its four rotations. L = lattice spanned.
Invariant group = Z^46 / L. A weighting with values in an abelian group A that vanishes on
every tile in every rotation is an element of Hom(Z^46 / L, A).
Run: python3 -I r1_lattice.py
"""
import random, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from collections import Counter
from rcommon import load, rots, smith_diagonal, rank_mod, tiles_from_joints, N

NC = 23


def gens(tiles, nc=NC, free_rot=True):
    out = set()
    for t in tiles:
        for (top, right, bot, left) in (rots(t) if free_rot else [t]):
            v = [0] * (2 * nc)
            v[bot] += 1; v[top] -= 1; v[nc + right] += 1; v[nc + left] -= 1
            if any(v):
                out.add(tuple(v))
    return sorted(out)


def group(tiles, nc=NC, free_rot=True):
    G = gens(tiles, nc, free_rot)
    d = smith_diagonal(G)
    used = len({c for t in tiles for c in t})
    free = 2 * nc - len(d)
    tors = sorted(x for x in d if x != 1)
    return free, tors, len(d)


def describe(free, tors):
    s = f"Z^{free}"
    for t in tors:
        s += f" + Z/{t}"
    return s


def weight_check(tiles, wh, wv, mod=None):
    """Does the weighting vanish on every tile in every rotation?"""
    for t in tiles:
        for (top, right, bot, left) in rots(t):
            s = wh[bot] - wh[top] + wv[right] - wv[left]
            if (s % mod if mod else s) != 0:
                return False
    return True


if __name__ == "__main__":
    P, clues = load()
    print("== official 256 tiles, rotations free ==")
    free, tors, rk = group(P)
    print(f"generators 46, rank of L {rk}, Z^46/L = {describe(free, tors)}")
    for p in (2, 3, 5, 7, 11, 13, 17, 19, 23):
        print(f"  rank of L mod {p}: {rank_mod(gens(P), p)}", end=";")
    print()
    ones = [1] * NC; zero = [0] * NC
    frame = [1 if 1 <= c <= 5 else 0 for c in range(NC)]
    print("all-ones on h vanishes:", weight_check(P, ones, zero),
          "| all-ones on v vanishes:", weight_check(P, zero, ones),
          "| frame colours (h and v) vanish mod 2:", weight_check(P, frame, frame, 2),
          "| frame colours over Z:", weight_check(P, frame, frame))
    print("variants, for the record:")
    f2, t2, r2 = group(P, free_rot=False)
    print(f"  stored orientation only (not the puzzle): rank {r2}, {describe(f2, t2)}")
    inner = [t for t in P if 0 not in t]
    f3, t3, r3 = group(inner)
    print(f"  196 interior tiles alone: rank {r3}, {describe(f3, t3)} "
          f"(12 coordinates of grey and frame colours are unused there)")
    border = [t for t in P if 0 in t]
    f4, t4, r4 = group(border)
    print(f"  60 border tiles alone: rank {r4}, {describe(f4, t4)}")
    cl = [P[i] for i, _, _, _ in clues]
    rest = [t for i, t in enumerate(P) if i not in {c[0] for c in clues}]
    f5, t5, r5 = group(rest)
    print(f"  251 tiles without the five clue tiles: rank {r5}, {describe(f5, t5)}")

    print("== positive controls (generated tile sets, not Eternity II) ==")
    rng = random.Random(20261113)
    # Control 1: a second closed ring. 16x16 board painted like the official one (5 ring colours,
    # 17 inner colours) except that the joints of the SECOND ring of cells (cells at distance 1
    # from the rim, joined to each other along the ring) take 3 dedicated colours 23,24,25.
    # Every tile then has 0 or 2 sides in {23,24,25}: a planted Z/2 invariant.
    def board(second_ring, nc):
        H = [[0] * (N - 1) for _ in range(N)]; V = [[0] * N for _ in range(N - 1)]
        for r in range(N):
            for c in range(N - 1):
                if r in (0, N - 1): H[r][c] = rng.randint(1, 5)
                elif second_ring and r in (1, N - 2) and 1 <= c < N - 2: H[r][c] = rng.randint(23, 25)
                else: H[r][c] = rng.randint(6, 22)
        for r in range(N - 1):
            for c in range(N):
                if c in (0, N - 1): V[r][c] = rng.randint(1, 5)
                elif second_ring and c in (1, N - 2) and 1 <= r < N - 2: V[r][c] = rng.randint(23, 25)
                else: V[r][c] = rng.randint(6, 22)
        return tiles_from_joints(N, H, V)
    res = Counter()
    for _ in range(5):
        T = board(False, 23)
        res[("plain painted board", describe(*group(T, 23)[:2]))] += 1
        T = board(True, 26)
        res[("second ring planted", describe(*group(T, 26)[:2]))] += 1
    for k, v in sorted(res.items()):
        print(f"  {k[0]}: {k[1]}  ({v} of 5 boards)")
    # Control 2: a planted Z/3. 12x12 TORUS board (no rim), 12 colours. Each colour has a charge
    # q(c) in Z/3; horizontal joints carry any colour, and every tile is forced to have
    # q(top)+q(right)+q(bottom)+q(left) = 0 mod 3 ... which is not rotation-proof for the signed
    # form, so instead plant it in the signed form directly: weighting wh = wv = q, and tiles
    # built with q(top) = q(bottom) and q(left) = q(right) mod 3 (colours of equal charge on
    # opposite sides). This plants FREE invariants (one per charge class and direction).
    n = 12
    q = {c: (c - 1) % 3 for c in range(1, 13)}
    byq = {k: [c for c in q if q[c] == k] for k in range(3)}
    colq = [rng.randrange(3) for _ in range(n)]   # charge of the vertical joints in each column
    rowq = [rng.randrange(3) for _ in range(n)]   # charge of the horizontal joints in each row
    tiles = []
    Hj = [[rng.choice(byq[rowq[r]]) for c in range(n)] for r in range(n)]  # joint right of (r,c)
    Vj = [[rng.choice(byq[colq[c]]) for c in range(n)] for r in range(n)]  # joint below (r,c)
    for r in range(n):
        for c in range(n):
            tiles.append((Vj[(r - 1) % n][c], Hj[r][c], Vj[r][c], Hj[r][(c - 1) % n]))
    f, t, rk = group(tiles, 13)
    print(f"  torus board, opposite sides of equal charge class (3 classes): rank {rk} of 24 used "
          f"coordinates, Z^26/L = {describe(f, t)}; expected free rank: 2 unused grey coordinates + "
          f"2 per charge class = 8")
    # Control 3: planted odd torsion. Tiles (a, a, a, x): three equal sides. The weighting
    # wh = wv = 1 on colour a gives 1-1+1 = +-1 ... not an invariant. Use instead the classical
    # one: a set in which every tile has the SAME colour on all four sides except one tile type
    # (a,a,a,b) for a fixed pair: computed, whatever it is, and checked against brute force.
    small = [(1, 1, 2, 2), (1, 2, 1, 2), (3, 3, 3, 3), (1, 1, 1, 3)]
    f, t, rk = group(small, 4)
    # brute force Hom(Z^8/L, Z/m) for m = 2,3,4 by enumeration, compare with the Smith form
    import itertools
    for m in (2, 3, 4):
        cnt = 0
        for w in itertools.product(range(m), repeat=8):
            if all(sum(a * b for a, b in zip(w, g)) % m == 0 for g in gens(small, 4)):
                cnt += 1
        from math import gcd
        pred = m ** f
        for d in t:
            pred *= gcd(d, m)
        print(f"  4-tile toy: Smith gives {describe(f, t)}; weightings mod {m}: brute force {cnt}, "
              f"predicted from Smith {pred}")
