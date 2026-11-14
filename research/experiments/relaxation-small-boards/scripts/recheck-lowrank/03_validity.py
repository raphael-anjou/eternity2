"""TOY. re-check check of what "solved" means in the Rust crate, without Board.py or the stored
enumeration as the authority.
 1. tiles regenerated from the painter (common.gen) and, for the broken board, the same recolouring;
 2. every row of place.txt is a rotation of its tile, sits in a rim-consistent cell, and the
    placement list is complete (count of all rim-consistent (cell, tile, rotation) with the pin);
 3. nb.txt is the grid;
 4. my own DFS on the tiles (written here, not common.enumerate_all) counts the solutions;
 5. every line of sols.txt passes an edge-by-edge check written here; the solution SETS are equal;
 6. parity of the colour counts of the broken tile set (odd count => unsolvable, no search).
Usage: python3 -I 03_validity.py"""
import os, sys
H = os.path.dirname(os.path.abspath(__file__))
L = os.path.join(H, "..", "lowrank-rust")
sys.path.insert(0, os.path.join(L, "py"))
from common import gen   # painter only

def rows(p): return [[int(a) for a in l.split()] for l in open(p) if l.strip()]

def broken(n, cf, ci, seed):
    tiles = gen(n, cf, ci, seed)
    for i, t in enumerate(tiles):
        if 0 not in t:
            nt = list(t); nt[0] = cf + 1 if t[0] != cf + 1 else cf + 2; nt = tuple(nt)
            canon = {min(u[k:] + u[:k] for k in range(4)) for j, u in enumerate(tiles) if j != i}
            if min(nt[k:] + nt[:k] for k in range(4)) in canon: continue
            tt = tiles[:i] + [nt] + tiles[i + 1:]
            if count_solutions(n, tt)[0] == 0: return tt, i, t, nt
    raise RuntimeError

def rimmask(n, x):
    r, c = divmod(x, n); return (r == 0, c == n - 1, r == n - 1, c == 0)

def count_solutions(n, tiles):
    """own DFS, snake-free row scan, cell 0 pinned to the first corner tile (grey N and W)."""
    N = n * n
    pin = next(i for i, t in enumerate(tiles) if t.count(0) == 2)
    opts = []
    for x in range(N):
        rm = rimmask(n, x); o = []
        for i, t in enumerate(tiles):
            for s in {t[k:] + t[:k] for k in range(4)}:
                if all((a == 0) == b for a, b in zip(s, rm)): o.append((i, s))
        opts.append(o)
    opts[0] = [(i, s) for i, s in opts[0] if i == pin]
    used = [False] * len(tiles); sides = [None] * N; who = [None] * N; sols = []; nodes = [0]
    def rec(x):
        nodes[0] += 1
        if x == N: sols.append(tuple(zip(who, sides))); return
        r, c = divmod(x, n)
        for i, s in opts[x]:
            if used[i] or (x and i == pin): continue
            if r and sides[x - n][2] != s[0]: continue
            if c and sides[x - 1][1] != s[3]: continue
            used[i] = True; sides[x] = s; who[x] = i
            rec(x + 1); used[i] = False
    rec(0)
    return len(sols), set(sols), sum(len(o) for o in opts) - sum(1 for x in range(1, N) for i, s in opts[x] if i == pin), nodes[0]

for name in ["b5_s0", "b5_s1", "b5_s2", "b5_s3", "b5_s4", "b5_broken"]:
    d = os.path.join(L, "boards", name)
    n, cf, ci, seed, P, N, T, K, F, ns = rows(os.path.join(d, "meta.txt"))[0]
    if name.endswith("broken"):
        tiles, bi, before, after = broken(n, cf, ci, seed)
    else:
        tiles = gen(n, cf, ci, seed)
    place = rows(os.path.join(d, "place.txt")); nb = rows(os.path.join(d, "nb.txt"))
    assert len(place) == P and len(tiles) == T == N
    for c, t, k, *s in place:
        s = tuple(s)
        assert s in {tiles[t][j:] + tiles[t][:j] for j in range(4)}, "not a rotation"
        assert all((a == 0) == b for a, b in zip(s, rimmask(n, c))), "rim"
    assert len({(c, t, tuple(s)) for c, t, k, *s in place}) == P, "duplicate placements"
    for x in range(N):
        r, c = divmod(x, n)
        assert nb[x] == [x - n if r else -1, x + 1 if c < n - 1 else -1, x + n if r < n - 1 else -1, x - 1 if c else -1]
    cnt, mine, pcount, nodes = count_solutions(n, tiles)
    stored = set()
    if ns:
        for js in rows(os.path.join(d, "sols.txt")):
            assert len(js) == N and [place[p][0] for p in js] == list(range(N))
            assert sorted(place[p][1] for p in js) == list(range(T)), "tile used twice"
            for x in range(N):
                s = place[js[x]][3:]
                for dd in range(4):
                    y = nb[x][dd]
                    assert (s[dd] == 0) if y < 0 else (s[dd] == place[js[y]][3:][(dd + 2) % 4]), "edge"
            stored.add(tuple((place[p][1], tuple(place[p][3:])) for p in js))
    col = {}
    for t in tiles:
        for a in t:
            if a: col[a] = col.get(a, 0) + 1
    odd = sorted(a for a, v in col.items() if v % 2)
    print(f"{name}: P {P} (my count of rim-consistent placements with the pin: {pcount}), stored solutions {ns}, my DFS {cnt} ({nodes} nodes), "
          f"sets equal {stored == mine}, colours with an odd side count {odd}" + (f", broken tile {bi}: {before} -> {after}" if name.endswith('broken') else ""))
