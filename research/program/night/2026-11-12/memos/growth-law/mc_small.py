"""Monte Carlo check of Z_planted(d) on fresh small boards.

Stdlib only. Run: python3 -I mc_small.py
Boards are drawn by a Python copy of the generator's painter (colours cycled
over the seams of each band, then shuffled; frame band = seams joining two rim
cells along the rim). The rotation-uniqueness repair of the Rust generator is
not reproduced, and all four orientations of a piece are counted (the model
counts four). The whole row-major tree is enumerated for every board.
"""
import math, os, random, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from model import Profile, edge_kind
import dfs_check


def planted(n, c, cf, rng):
    seams = []
    for y in range(n):
        for x in range(n):
            if x + 1 < n:
                seams.append(((x, y), (x + 1, y)))
            if y + 1 < n:
                seams.append(((x, y), (x, y + 1)))
    F = [s for s in seams if edge_kind(n, *s) == "F"]
    I = [s for s in seams if edge_kind(n, *s) == "I"]
    col = {}
    for band, base, k in ((F, 0, cf), (I, cf, c)):
        cols = [base + (i % k) + 1 for i in range(len(band))]
        rng.shuffle(cols)
        for s, v in zip(band, cols):
            col[s] = v
    pieces = []
    for y in range(n):
        for x in range(n):
            up = col.get(((x, y - 1), (x, y)), 0)
            down = col.get(((x, y), (x, y + 1)), 0)
            left = col.get(((x - 1, y), (x, y)), 0)
            right = col.get(((x, y), (x + 1, y)), 0)
            pieces.append((up, right, down, left))
    return pieces


def build_all(pieces):
    table = {}
    for p, e in enumerate(pieces):
        for r in range(4):
            t, rt, b, l = dfs_check.rotated(e, r)
            table.setdefault((t, l, rt == 0, b == 0), []).append((p, rt, b))
    return table


def main():
    dfs_check.build = build_all
    rng = random.Random(20261112)
    for n, c, reps in ((4, 2, 3000), (4, 3, 3000), (5, 3, 600), (5, 4, 1500), (5, 6, 3000)):
        N = n * n
        tot = [0.0] * (N + 1)
        sq = [0.0] * (N + 1)
        done = 0
        for _ in range(reps):
            nodes, Z, sols, capped = dfs_check.search(n, planted(n, c, 5, rng), False, 400000)
            if capped:
                continue
            done += 1
            for d in range(N + 1):
                tot[d] += Z[d]
                sq[d] += Z[d] * Z[d]
        p = Profile(n, c, "rowmajor")
        full_m = sum(tot[1:]) / done
        print("n=%d c=%d boards=%d (capped %d): full tree measured %.1f, model %.1f, ratio %.3f; solutions measured %.2f, model %.2f" % (
            n, c, done, reps - done, full_m, p.full_tree(), full_m / p.full_tree(), tot[N] / done, p.solutions_planted()))
        line = []
        for d in range(1, N + 1):
            m = tot[d] / done
            se = math.sqrt(max(sq[d] / done - m * m, 0) / done)
            line.append("%d:%.1f/%.1f(se %.1f)" % (d, m, p.Z_planted(d), se))
        print("   d:measured/model  " + "  ".join(line))


if __name__ == "__main__":
    main()
