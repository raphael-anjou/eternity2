"""Toy boards for the gap-6a linearisation check (TOY BOARDS, generated; not Eternity II progress).

Painter: copied from gap-6b/toy_reformulations.py gen() (same random call order, so
(n, cf, ci, seed) gives the same tile list as that script): cf colours on the edges between
two rim cells along the rim ("ring" edges), ci colours on every other inner edge, colour 0 on
the rim; boards with a repeated tile or a rotation-symmetric tile are redrawn.

Pin: the corner tile with the smallest index in the shuffled tile list is fixed in cell 0
(row 0, column 0) in its only rotation with grey on the north and west sides. Solutions are
counted with that pin, so the four rotations of a whole board count once.

Sides of a tile: (N, E, S, W). A placement is (cell, tile, sides after rotation).
"""
import random, sys, json

def gen(n, cf, ci, seed, max_tries=20000):
    rng = random.Random(seed)
    for _ in range(max_tries):
        hor = [[0] * (n + 1) for _ in range(n)]
        ver = [[0] * n for _ in range(n + 1)]
        for r in range(n):
            for c in range(1, n):
                ring = r in (0, n - 1)
                hor[r][c] = rng.randint(1, cf) if ring else cf + rng.randint(1, ci)
        for r in range(1, n):
            for c in range(n):
                ring = c in (0, n - 1)
                ver[r][c] = rng.randint(1, cf) if ring else cf + rng.randint(1, ci)
        tiles = [(ver[r][c], hor[r][c + 1], ver[r + 1][c], hor[r][c]) for r in range(n) for c in range(n)]
        canon = set()
        ok = True
        for t in tiles:
            rots = [t[k:] + t[:k] for k in range(4)]
            if len(set(rots)) < 4 and 0 not in t: ok = False
            m = min(rots)
            if m in canon: ok = False
            canon.add(m)
        if ok:
            rng.shuffle(tiles)
            return tiles
    return None     # the painter found no board with distinct, asymmetric tiles

def rim_pattern(n, x):
    r, c = divmod(x, n)
    return (r == 0, c == n - 1, r == n - 1, c == 0)   # which sides are rim: N E S W

def placements(n, tiles):
    """All rim-consistent placements: grey exactly on the rim sides. List of (cell, tile, sides)."""
    out = []
    for x in range(n * n):
        rim = rim_pattern(n, x)
        for i, t in enumerate(tiles):
            seen = set()
            for k in range(4):
                p = t[k:] + t[:k]
                if p in seen: continue
                seen.add(p)
                if all((s == 0) == rm for s, rm in zip(p, rim)):
                    out.append((x, i, p))
    return out

def pin_tile(tiles):
    return min(i for i, t in enumerate(tiles) if sum(1 for s in t if s == 0) == 2)

def solve(n, tiles, cap=100000):
    """All solutions with the pin, row scan. Returns list of solutions; each a list of (tile, sides) per cell."""
    N = n * n
    pl = placements(n, tiles)
    by_cell = [[] for _ in range(N)]
    for x, i, p in pl: by_cell[x].append((i, p))
    t0 = pin_tile(tiles)
    by_cell[0] = [(i, p) for i, p in by_cell[0] if i == t0]
    used = [False] * N
    cur = [None] * N
    sols = []
    def rec(x):
        if len(sols) >= cap: return
        if x == N:
            sols.append(list(cur)); return
        r, c = divmod(x, n)
        for i, p in by_cell[x]:
            if used[i]: continue
            if c > 0 and cur[x - 1][1][1] != p[3]: continue
            if r > 0 and cur[x - n][1][2] != p[0]: continue
            used[i] = True; cur[x] = (i, p)
            rec(x + 1)
            used[i] = False
        cur[x] = None
    rec(0)
    return sols

if __name__ == "__main__":
    # scan: python3 -I boards.py scan n nseeds cf_list ci_list
    #   -> share of seeds with exactly one pinned solution per (cf, ci)
    if sys.argv[1] == "scan":
        n, ns = int(sys.argv[2]), int(sys.argv[3])
        cfs = [int(v) for v in sys.argv[4].split(",")]; cis = [int(v) for v in sys.argv[5].split(",")]
        print(f"n={n} seeds 0..{ns-1}: boards with exactly 1 pinned solution, median count, max count (cap 2000)")
        for cf in cfs:
            for ci in cis:
                cnt = []
                for s in range(ns):
                    t = gen(n, cf, ci, s)
                    if t is None: break
                    cnt.append(len(solve(n, t, cap=2000)))
                if len(cnt) < ns:
                    print(f"cf={cf} ci={ci}: painter fails (no board with distinct tiles)", flush=True); continue
                cs = sorted(cnt)
                print(f"cf={cf} ci={ci} total={cf+ci}: unique {sum(1 for v in cnt if v == 1)}/{ns}  median {cs[len(cs)//2]}  max {cs[-1]}", flush=True)
