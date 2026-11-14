"""Smallest unsatisfiable core of a clue corner block over its four touching
edge cells, exact on the official pieces, all five clue pieces out of the pool.
python3 -I 02_corner_cores.py"""
import sys, time
from math import log2
from collections import Counter
LIB = "research/program/night/2026-11-12/notes/price-and-ledger-review"
sys.path.insert(0, LIB)
from rv_lib import *
t0 = time.time()
pieces, hints = load_official()
N = 16
en = Enum(N, pieces)
OPP = (2, 3, 0, 1)
NB = ((-1, 0), (0, 1), (1, 0), (0, -1))
lines = [l.strip() for l in open(CSV) if l.strip()][1:]
clue = {}
for i, l in enumerate(lines):
    c = l.split(",")
    if int(c[4]) or int(c[5]) or int(c[6]):
        clue[(int(c[5]), int(c[4]))] = (i, rotations(pieces[i])[int(c[6])])
cluep = {v[0] for v in clue.values()}
interior = {i for i, p in enumerate(pieces) if 0 not in p}

def chains(cells):
    board, used = {}, set()
    def go(d):
        if d == len(cells):
            yield dict(board); return
        r, c = cells[d]
        key = []
        for s in range(4):
            rr, cc = r + NB[s][0], c + NB[s][1]
            if rr < 0 or rr >= N or cc < 0 or cc >= N:
                key.append(0)
            else:
                q = board.get((rr, cc))
                key.append(q[1][OPP[s]] if q is not None else -1)
        for i, q in en.cands(tuple(key)):
            if i in used: continue
            used.add(i); board[(r, c)] = (i, q)
            yield from go(d + 1)
            used.discard(i); del board[(r, c)]
    yield from go(0)

MASKS = sorted(range(1, 16), key=lambda m: bin(m).count("1"))
alive_all = 1.0
tot = Counter()
for (R, C) in ((0, 0), (0, 15), (15, 0), (15, 15)):
    dr = 1 if R == 0 else -1
    dc = 1 if C == 0 else -1
    fr5 = [(R, C), (R, C + dc), (R, C + 2 * dc), (R + dr, C), (R + 2 * dr, C)]
    cl = (R + 2 * dr, C + 2 * dc)
    order = [(R + dr, C + 2 * dc), (R + 2 * dr, C + dc), (R + dr, C + dc)]
    allowed = (interior - cluep) | {clue[cl][0]}
    vside = 2 if R == 0 else 0
    hside = 1 if C == 0 else 3
    sides = (vside, vside, hside, hside)
    cache = {}
    allch = list(chains(fr5))
    nch = len(allch)
    # counts of chains sharing pieces / colours on each mask
    cp = {m: Counter() for m in MASKS}
    cc = {m: Counter() for m in MASKS}
    def keys(bd, m):
        kp = tuple(bd[fr5[1 + j]][0] for j in range(4) if m >> j & 1)
        kc = tuple(bd[fr5[1 + j]][1][sides[j]] for j in range(4) if m >> j & 1)
        return kp, kc
    for bd in allch:
        for m in MASKS:
            kp, kc = keys(bd, m)
            cp[m][kp] += 1; cc[m][kc] += 1
    size = Counter(); bp = Counter(); bc = Counter(); nd = Counter(); dead = 0
    for bd in allch:
        found = None
        for m in MASKS:
            kp, kc = keys(bd, m)
            v = cache.get((m, kc))
            if v is None:
                pre = {fr5[1 + j]: bd[fr5[1 + j]] for j in range(4) if m >> j & 1}
                pre[cl] = clue[cl]
                res = en.run(order, preset=pre, allowed=allowed)["per"]
                v = (res[-1], sum(res[1:]))
                cache[(m, kc)] = v
            if v[0] == 0:
                found = (m, v[1], kp, kc); break
        if found:
            dead += 1
            m, nodes, kp, kc = found
            s = bin(m).count("1")
            size[s] += 1
            bp[s] += log2(nch / cp[m][kp]); bc[s] += log2(nch / cc[m][kc]); nd[s] += nodes
    alive_all *= 1 - dead / nch
    print("corner (%2d,%2d): chains %d, block dead %d (%.4f), alive %.4f = %.2f bits" % (R, C, nch, dead, dead / nch, 1 - dead / nch, -log2(1 - dead / nch)))
    for s in sorted(size):
        print("    smallest core %d edge cells: %5.1f%% of dead chains | mean bits, placements %.2f, inward colours %.2f | mean nodes of the proof %.1f"
              % (s, 100 * size[s] / dead, bp[s] / size[s], bc[s] / size[s], nd[s] / size[s]))
        tot[s] += size[s] / dead / 4
print("\nall four blocks alive: %.3e = 2^-%.2f of chains (product of the four corners)" % (alive_all, -log2(alive_all)))
print("mean share by smallest core size over corners:", {s: round(v, 3) for s, v in sorted(tot.items())})
print("[%.1fs]" % (time.time() - t0))
