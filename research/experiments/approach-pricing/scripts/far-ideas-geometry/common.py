"""TOY BOARDS ONLY (generated; never Eternity II progress).

Painter copied from gap-6a-linearisation/boards.py gen() (itself from gap-6b), so
(n, cf, ci, seed) gives the same tile list as those scripts. Sides are (N, E, S, W), colour 0
on the rim. Pin: the corner tile with the smallest index is fixed at cell 0 in its only
rotation with grey on N and W (kills the four global rotations).

enumerate_all(): plain row-scan DFS, strict matching, every solution, level widths, and
optionally the list of live partial boards at one depth with the number of solutions below.
"""
import random


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
            if len(set(rots)) < 4 and 0 not in t:
                ok = False
            m = min(rots)
            if m in canon:
                ok = False
            canon.add(m)
        if ok:
            rng.shuffle(tiles)
            return tiles
    return None


def rot(t, k):
    """Rotation k of tile t as sides (N,E,S,W): t[k:]+t[:k] (same convention as boards.py)."""
    return t[k:] + t[:k]


def rim(n, x):
    r, c = divmod(x, n)
    return (r == 0, c == n - 1, r == n - 1, c == 0)


def oriented(n, tiles):
    """per cell: list of (tile, k, sides) consistent with the rim."""
    out = []
    for x in range(n * n):
        rm = rim(n, x)
        lst = []
        for i, t in enumerate(tiles):
            seen = set()
            for k in range(4):
                p = rot(t, k)
                if p in seen:
                    continue
                seen.add(p)
                if all((s == 0) == q for s, q in zip(p, rm)):
                    lst.append((i, k, p))
        out.append(lst)
    return out


def pin_of(n, tiles):
    for i, t in enumerate(tiles):
        if t.count(0) == 2:
            for k in range(4):
                p = rot(t, k)
                if p[0] == 0 and p[3] == 0:
                    return (i, k, p)
    raise ValueError


def enumerate_all(n, tiles, keep_depth=None, max_nodes=10**9):
    """Row-scan DFS. Returns dict: sols (list of tuples of (tile,k) per cell), widths, live (list of
    (partial assignment tuple, n_solutions_below)) at keep_depth, nodes."""
    orient = oriented(n, tiles)
    # bucket by (N colour, W colour) per cell
    buckets = []
    for x in range(n * n):
        b = {}
        for (i, k, p) in orient[x]:
            b.setdefault((p[0], p[3]), []).append((i, k, p))
        buckets.append(b)
    pin = pin_of(n, tiles)
    N = n * n
    used = [False] * len(tiles)
    assign = [None] * N
    sides = [None] * N
    widths = [0] * (N + 1)
    sols = []
    live = []
    nodes = [0]

    def rec(x):
        nodes[0] += 1
        widths[x] += 1
        if x == N:
            sols.append(tuple(assign))
            return 1
        r, c = divmod(x, n)
        north = sides[x - n][2] if r > 0 else 0
        west = sides[x - 1][1] if c > 0 else 0
        cnt = 0
        for (i, k, p) in buckets[x].get((north, west), ()):
            if used[i]:
                continue
            used[i] = True
            assign[x] = (i, k)
            sides[x] = p
            if keep_depth is not None and x + 1 == keep_depth:
                below = rec(x + 1)
                live.append((tuple(assign[:x + 1]), below))
                cnt += below
            else:
                cnt += rec(x + 1)
            used[i] = False
        return cnt

    i, k, p = pin
    used[i] = True
    assign[0] = (i, k)
    sides[0] = p
    widths[0] = 1
    nodes[0] = 1
    rec(1)
    return dict(sols=sols, widths=widths, live=live, nodes=nodes[0], orient=orient, pin=pin)
