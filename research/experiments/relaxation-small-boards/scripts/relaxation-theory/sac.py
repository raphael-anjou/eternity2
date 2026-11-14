"""TOY BOARDS ONLY. Local-consistency ladder on generated boards, against the true solution set.

Usage: python3 -I sac.py n cf ci seed [max_dfs_nodes]
Rungs (each a polynomial-time combinatorial rule, no search):
  AC    arc consistency on cells (adjacent: colours match and tiles differ; one-tile cells remove the
        tile elsewhere), as board.py of the lift scripts.
  ACM   AC plus: a perfect matching of cells to tiles must exist in the live (cell, tile) graph.
  SAC   singleton AC: pin p, run AC, a wipe-out kills p; to a fixpoint.
  SACM  the same with ACM as the inner test.
Reports, per rung, how many placements stay alive and how many of those are in no solution.
Board, placements and pin are the lift's (common.py of far-ideas-geometry, read only).
"""
import os, sys, json, time
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "far-ideas-geometry"))
from common import gen, enumerate_all, oriented, pin_of

n, cf, ci, seed = map(int, sys.argv[1:5])
maxn = int(sys.argv[5]) if len(sys.argv) > 5 else 3 * 10**8
tiles = gen(n, cf, ci, seed)
N = n * n
pin = pin_of(n, tiles)
orient = oriented(n, tiles)
cellOf, tileOf, cols, key = [], [], [], {}
for x in range(N):
    for (i, k, p) in orient[x]:
        if x == 0 and (i, k) != (pin[0], pin[1]):
            continue
        if x != 0 and i == pin[0]:
            continue
        key[(x, i, k)] = len(cellOf)
        cellOf.append(x); tileOf.append(i); cols.append(p)
P = len(cellOf)
cellmask = [0] * N
tilemask = [0] * len(tiles)
for j in range(P):
    cellmask[cellOf[j]] |= 1 << j
    tilemask[tileOf[j]] |= 1 << j
nb = []
for x in range(N):
    r, c = divmod(x, n)
    nb.append([x - n if r > 0 else -1, x + 1 if c < n - 1 else -1, x + n if r < n - 1 else -1, x - 1 if c > 0 else -1])
# support masks
bycol = {}
for j in range(P):
    for d in range(4):
        bycol.setdefault((cellOf[j], d, cols[j][d]), 0)
        bycol[(cellOf[j], d, cols[j][d])] |= 1 << j
supp = []
for j in range(P):
    s = []
    for d in range(4):
        y = nb[cellOf[j]][d]
        s.append(-1 if y < 0 else bycol.get((y, (d + 2) % 4, cols[j][d]), 0) & ~tilemask[tileOf[j]])
    supp.append(s)
ALL = (1 << P) - 1


def bits(m):
    out = []
    while m:
        b = m & -m
        out.append(b.bit_length() - 1)
        m ^= b
    return out


def matching_ok(alive):
    adj = []
    for x in range(N):
        adj.append(sorted({tileOf[j] for j in bits(alive & cellmask[x])}))
    match = {}

    def aug(x, seen):
        for t in adj[x]:
            if t in seen:
                continue
            seen.add(t)
            if t not in match or aug(match[t], seen):
                match[t] = x
                return True
        return False
    return all(aug(x, set()) for x in range(N))


def ac(alive, use_matching):
    """returns the fixpoint mask, or 0 on a wipe-out."""
    while True:
        changed = False
        for x in range(N):
            dom = alive & cellmask[x]
            if not dom:
                return 0
            for j in bits(dom):
                for d in range(4):
                    s = supp[j][d]
                    if s != -1 and not (s & alive):
                        alive &= ~(1 << j)
                        changed = True
                        break
        for x in range(N):
            dom = alive & cellmask[x]
            if not dom:
                return 0
            t0 = tileOf[dom.bit_length() - 1]
            if not (dom & ~tilemask[t0]):
                kill = alive & tilemask[t0] & ~cellmask[x]
                if kill:
                    alive &= ~kill
                    changed = True
        if not changed:
            break
    if use_matching and not matching_ok(alive):
        return 0
    return alive


def sac(alive, use_matching):
    alive = ac(alive, use_matching)
    while True:
        changed = False
        for j in bits(alive):
            if not (alive >> j) & 1:
                continue
            x = cellOf[j]
            trial = (alive & ~cellmask[x] & ~tilemask[tileOf[j]]) | (1 << j)
            if not ac(trial, use_matching):
                alive &= ~(1 << j)
                changed = True
        a2 = ac(alive, use_matching)
        if a2 != alive:
            alive = a2
            changed = True
        if not changed or not alive:
            return alive


t0 = time.time()
base = enumerate_all(n, tiles, max_nodes=maxn)
solmask = 0
for s in base["sols"]:
    for x, a in enumerate(s):
        solmask |= 1 << key[(x, a[0], a[1])]
out = dict(board=[n, cf, ci, seed], P=P, n_sols=len(base["sols"]), dfs_nodes=base["nodes"],
           in_some_solution=bin(solmask).count("1"), enum_seconds=round(time.time() - t0, 1))
for name, f in (("AC", lambda: ac(ALL, False)), ("ACM", lambda: ac(ALL, True)),
                ("SAC", lambda: sac(ALL, False)), ("SACM", lambda: sac(ALL, True))):
    t1 = time.time()
    m = f()
    assert m & solmask == solmask, "a rung killed a solution placement"
    out[name] = dict(alive=bin(m).count("1"), alive_in_no_solution=bin(m & ~solmask).count("1"),
                     seconds=round(time.time() - t1, 1))
    if name == "SACM":
        out["sacm_mask_hex"] = hex(m)
        out["sol_mask_hex"] = hex(solmask)
print(json.dumps(out))
