"""Gap 6b toy check (TOY BOARDS, generated; not Eternity II progress).

Counts the nodes of exhaustive searches (all solutions, no early stop) on small
framed boards with pairwise distinct tiles, under different search spaces:

  T     tile placement, row scan, test = edges match + each tile once
  TM    same, plus the global test M below at every node
  E/R   edge colouring, edges in row-scan order (east then south edge of each cell), test M
  E/I   edge colouring, row interface first (south edges of a row, then its east edges), test M
  E/H   edge colouring, all left-right edges first, then all up-down edges, test M
  E/C   edge colouring, edges around the black cells of a checkerboard first, test M
  COARSE  interior colours merged into two classes; share of coarse colourings of the
          non-ring edges that pass M (sampled), against the number that can be refined

Test M: for every cell, the tiles compatible with its known sides (rim included)
form its domain; M passes iff a perfect matching cells -> distinct tiles exists.
Usage: python3 -I toy_reformulations.py n cf ci seed [cap]
"""
import sys, random
from math import log2

n, cf, ci, seed = map(int, sys.argv[1:5])
cap = int(sys.argv[5]) if len(sys.argv) > 5 else 3_000_000
rng = random.Random(seed)
N = n * n

def gen():
    while True:
        hor = [[0] * (n + 1) for _ in range(n)]   # hor[r][c]: edge on the west side of (r,c)
        ver = [[0] * n for _ in range(n + 1)]     # ver[r][c]: edge on the north side of (r,c)
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

tiles = gen()
place = [(i, t[k:] + t[:k]) for i, t in enumerate(tiles) for k in range(4)]
place = list({(i, p) for i, p in place})

# edges: index them. sides of cell: 0 N, 1 E, 2 S, 3 W. rim sides have edge id -1 (colour 0).
eid = {}
cell_edges = [[-1] * 4 for _ in range(N)]
edges = []        # (cellA, sideA, cellB, sideB, ring)
for r in range(n):
    for c in range(n):
        x = r * n + c
        if c + 1 < n:
            e = len(edges); edges.append((x, 1, x + 1, 3, r in (0, n - 1)))
            cell_edges[x][1] = e; cell_edges[x + 1][3] = e
        if r + 1 < n:
            e = len(edges); edges.append((x, 2, x + n, 0, c in (0, n - 1)))
            cell_edges[x][2] = e; cell_edges[x + n][0] = e
NE = len(edges)
palette = [list(range(1, cf + 1)) if e[4] else list(range(cf + 1, cf + ci + 1)) for e in edges]

dom_cache = {}
def dom(pattern):
    """bitmask of tiles having a rotation consistent with pattern (None = unknown)."""
    m = dom_cache.get(pattern)
    if m is None:
        m = 0
        for i, p in place:
            if all(q is None or q == s for q, s in zip(pattern, p)):
                m |= 1 << i
        # unknown sides must not be rim (0): handled because rim sides are always known = 0,
        # and inner sides never take colour 0 in the palette; exclude placements with 0 on unknown side
        m2 = 0
        for i, p in place:
            if all((q is None and s != 0) or q == s for q, s in zip(pattern, p)):
                m2 |= 1 << i
        m = m2
        dom_cache[pattern] = m
    return m

def has_matching(doms):
    owner = {}
    order = sorted(range(N), key=lambda x: bin(doms[x]).count("1"))
    def aug(x, seen):
        d = doms[x] & ~seen[0]
        while d:
            b = d & -d; d ^= b
            seen[0] |= b
            y = owner.get(b)
            if y is None or aug(y, seen):
                owner[b] = x
                return True
        return False
    for x in order:
        if not aug(x, [0]):
            return False
    return True

col = [None] * NE
def pattern(x):
    return tuple(0 if e < 0 else col[e] for e in cell_edges[x])
def test_M():
    doms = [dom(pattern(x)) for x in range(N)]
    if 0 in doms: return False
    return has_matching(doms)

class Cap(Exception): pass

def edge_search(order):
    lev = [0] * (NE + 1)
    tot = [0]
    def rec(k):
        lev[k] += 1; tot[0] += 1
        if tot[0] > cap: raise Cap
        if k == NE: return
        e = order[k]
        for v in palette[e]:
            col[e] = v
            if test_M(): rec(k + 1)
        col[e] = None
    capped = False
    try: rec(0)
    except Cap:
        capped = True
        for e in range(NE): col[e] = None
    return lev, tot[0], capped

def tile_search(with_M):
    lev = [0] * (N + 1)
    used = [False] * len(tiles)
    board = [None] * N
    byNW = {}
    for i, p in place: byNW.setdefault((p[0], p[3]), []).append((i, p))
    def rec(x):
        lev[x] += 1
        if x == N: return
        r, c = divmod(x, n)
        north = board[x - n][2] if r else 0
        west = board[x - 1][1] if c else 0
        for i, p in byNW.get((north, west), ()):
            if used[i]: continue
            if (p[1] == 0) != (c == n - 1) or (p[2] == 0) != (r == n - 1): continue
            if with_M:
                for s in (1, 2):
                    e = cell_edges[x][s]
                    if e >= 0: col[e] = p[s]
                ok = test_M()
            else: ok = True
            if ok:
                used[i] = True; board[x] = p
                rec(x + 1)
                used[i] = False
            if with_M:
                for s in (1, 2):
                    e = cell_edges[x][s]
                    if e >= 0: col[e] = None
        board[x] = None
    rec(0)
    return lev

def fmt(lev): return f"total {sum(lev):>9d}  peak {max(lev):>8d} ({log2(max(lev)):5.1f} bits) at level {lev.index(max(lev))}"

print(f"TOY n={n} frame colours={cf} interior colours={ci} seed={seed}  tiles={len(tiles)} inner edges={NE}")
T = tile_search(False); Z = T[N]
print(f"T     {fmt(T)}   solutions {Z}")
TM = tile_search(True)
print(f"TM    {fmt(TM)}   solutions {TM[N]}")

oR = []; oI = []
for r in range(n):
    row_e = [cell_edges[r * n + c][1] for c in range(n - 1)]
    row_s = [cell_edges[r * n + c][2] for c in range(n)] if r + 1 < n else []
    for c in range(n):
        for s in (1, 2):
            e = cell_edges[r * n + c][s]
            if e >= 0: oR.append(e)
    oI += row_s + row_e
oH = [e for e in range(NE) if edges[e][1] == 1] + [e for e in range(NE) if edges[e][1] == 2]
oC = []
for x in range(N):
    if sum(divmod(x, n)) % 2 == 0:
        for e in cell_edges[x]:
            if e >= 0 and e not in oC: oC.append(e)
oC += [e for e in range(NE) if e not in oC]
for name, o in (("E/R", oR), ("E/I", oI), ("E/H", oH), ("E/C", oC)):
    lev, tot, capped = edge_search(o)
    extra = ""
    if name == "E/R" and not capped:
        # levels at which cells 0..j are complete
        done = []; k = 0
        for x in range(N):
            k += sum(1 for s in (1, 2) if cell_edges[x][s] >= 0)
            done.append(lev[k])
        same = all(a == b for a, b in zip(done, TM[1:]))
        extra = f"   cell-complete levels equal TM levels: {same}"
    print(f"{name}   {fmt(lev)}{'  CAPPED (lower bound)' if capped else ''}{extra}")

# coarse two-class colouring of the non-ring inner edges
cls = {v: 'ab'[(v - cf - 1) % 2] for v in range(cf + 1, cf + ci + 1)}
free = [e for e in range(NE) if not edges[e][4]]
cdom_cache = {}
def cdom(pat):   # pat entries: 0 rim, 'r' ring unknown, 'a'/'b' class
    m = cdom_cache.get(pat)
    if m is None:
        m = 0
        for i, p in place:
            ok = True
            for q, s in zip(pat, p):
                if q == 0: ok &= s == 0
                elif q == 'r': ok &= 1 <= s <= cf
                else: ok &= s > cf and cls[s] == q
            if ok: m |= 1 << i
        cdom_cache[pat] = m
    return m
def coarse_pass(assign):
    doms = []
    for x in range(N):
        pat = tuple(0 if e < 0 else ('r' if edges[e][4] else assign[e]) for e in cell_edges[x])
        d = cdom(pat)
        if d == 0: return False
        doms.append(d)
    return has_matching(doms)
S = 20000; ok = 0
r2 = random.Random(seed + 1)
for _ in range(S):
    a = {e: r2.choice('ab') for e in free}
    ok += coarse_pass(a)
print(f"COARSE  non-ring inner edges {len(free)} -> {len(free)} bits committed; pass share {ok}/{S}"
      f" -> survivors about 2^{len(free) + (log2(ok / S) if ok else float('-inf')):.1f}; refinable at most {Z} (2^{log2(max(Z,1)):.1f})")
