"""Gap 3, board files for g3tree.c.

Writes one text file per (board, scan direction). Every direction is turned into
"rows top-down, left to right" by reflecting the board, so the C program has one scan.

  python3 -I g3_prep.py real  <official.csv> <outdir>            -> real-<dir>.txt (no solution)
  python3 -I g3_prep.py paint <official.csv> <outdir> <seed0> <count>
        -> paint-<seed>-<dir>.txt: TOY boards painted with the official colour census
           (60 frame joints, 420 inner joints, colours shuffled over joints), planted
           solution kept, the five official clue cells pinned to the planted tiles.

File format: n / n*n lines "t r b l" (free orientation) / F then F lines "pos pid t r b l"
(pinned) / S then S lines "pos pid t r b l" (planted solution, S = 0 if unknown).
Directions: TD rows top-down, BU rows bottom-up (Track 10's scan), LR columns left to
right, RL columns right to left.
"""
import random
import sys
from collections import Counter

N = 16


def flip_v(y, x, e):
    return N - 1 - y, x, (e[2], e[1], e[0], e[3])


def flip_h(y, x, e):
    return y, N - 1 - x, (e[0], e[3], e[2], e[1])


def transpose(y, x, e):
    return x, y, (e[3], e[2], e[1], e[0])


def compose(*fs):
    def g(y, x, e):
        for f in fs:
            y, x, e = f(y, x, e)
        return y, x, e
    return g


DIRS = {
    "TD": compose(),
    "BU": compose(flip_v),
    "LR": compose(transpose),
    "RL": compose(flip_h, transpose),
}


def rot(e, r):
    t, ri, b, l = e
    return [(t, ri, b, l), (l, t, ri, b), (b, l, t, ri), (ri, b, l, t)][r & 3]


def load(fn):
    rows = [l.strip() for l in open(fn) if l.strip()]
    pieces, hints = [], []
    for i, l in enumerate(rows[1:]):
        c = l.split(",")
        e = tuple(0 if int(v, 2) == 65535 else int(v, 2) for v in c[:4])
        pieces.append(e)
        x, y, r = int(c[4]), int(c[5]), int(c[6])
        if x or y or r:
            hints.append((y, x, i, rot(e, r)))
    return pieces, hints


def write(fn, pieces, fixed, sol, tf):
    with open(fn, "w") as f:
        f.write("%d\n" % N)
        for e in pieces:
            f.write("%d %d %d %d\n" % tf(0, 0, e)[2])
        f.write("%d\n" % len(fixed))
        for (y, x, pid, e) in fixed:
            yy, xx, ee = tf(y, x, e)
            f.write("%d %d %d %d %d %d\n" % ((yy * N + xx, pid) + ee))
        f.write("%d\n" % len(sol))
        for (y, x, pid, e) in sol:
            yy, xx, ee = tf(y, x, e)
            f.write("%d %d %d %d %d %d\n" % ((yy * N + xx, pid) + ee))


def census(pieces):
    fc, ic = Counter(), Counter()
    for p in pieces:
        z = [i for i, v in enumerate(p) if v == 0]
        if len(z) == 0:
            for v in p:
                ic[v] += 1
        elif len(z) == 1:
            i = z[0]
            ic[p[(i + 2) % 4]] += 1
            fc[p[(i + 1) % 4]] += 1
            fc[p[(i + 3) % 4]] += 1
        else:
            for v in p:
                if v:
                    fc[v] += 1
    return fc, ic


def paint(fc, ic, rng):
    fj = [c for c, k in sorted(fc.items()) for _ in range(k // 2)]
    ij = [c for c, k in sorted(ic.items()) for _ in range(k // 2)]
    assert len(fj) == 60 and len(ij) == 420, (len(fj), len(ij))
    while True:
        rng.shuffle(fj)
        rng.shuffle(ij)
        a = b = 0
        H = [[0] * (N + 1) for _ in range(N)]
        V = [[0] * N for _ in range(N + 1)]
        for r in range(N):
            for c in range(1, N):
                if r in (0, N - 1):
                    H[r][c] = fj[a]; a += 1
                else:
                    H[r][c] = ij[b]; b += 1
        for r in range(1, N):
            for c in range(N):
                if c in (0, N - 1):
                    V[r][c] = fj[a]; a += 1
                else:
                    V[r][c] = ij[b]; b += 1
        cells = [(V[r][c], H[r][c + 1], V[r + 1][c], H[r][c]) for r in range(N) for c in range(N)]
        canon = set()
        ok = True
        for p in cells:
            rots = [rot(p, k) for k in range(4)]
            if len(set(rots)) < 4 or min(rots) in canon:
                ok = False
                break
            canon.add(min(rots))
        if ok:
            return cells


def main():
    mode, csv, out = sys.argv[1:4]
    pieces, hints = load(csv)
    if mode == "real":
        for d, tf in DIRS.items():
            write("%s/real-%s.txt" % (out, d), pieces, hints, [], tf)
        fc, ic = census(pieces)
        print("frame colours", sorted(fc.values()), "inner colours", sorted(ic.values()))
        return
    seed0, count = int(sys.argv[4]), int(sys.argv[5])
    fc, ic = census(pieces)
    clue_cells = [(h[0], h[1]) for h in hints]
    for s in range(seed0, seed0 + count):
        rng = random.Random(1113000 + s)
        cells = paint(fc, ic, rng)
        ids = list(range(N * N))
        rng.shuffle(ids)                      # piece id of the tile planted on each cell
        pcs = [None] * (N * N)
        sol = []
        for pos, e in enumerate(cells):
            pcs[ids[pos]] = rot(e, rng.randrange(4))
            sol.append((pos // N, pos % N, ids[pos], e))
        fixed = [sol[y * N + x] for (y, x) in clue_cells]
        for d, tf in DIRS.items():
            write("%s/paint-%d-%s.txt" % (out, s, d), pcs, fixed, sol, tf)


main()
