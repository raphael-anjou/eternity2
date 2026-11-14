"""Second analysis, claim 1. Independent of rv_lib. python3 -I r1_corners.py [reps] [N]
Part A: exact chain enumeration per corner, block alive, minimal core sizes.
Part B: SMC over real valid frames (gate: exact frame count), frame-weighted
        alive fractions, pairwise dependence, |H|/|Omega| computed jointly,
        cross-block piece conflicts, per-frame smallest core."""
import sys, random, time
from math import log2, log10
from collections import Counter
from itertools import permutations

CSV = "engine/data/official_eternity2.csv"
EXACT_FRAMES = 40519056449429419697492635191544381440
t0 = time.time()
lines = [l.strip() for l in open(CSV) if l.strip()][1:]
P = []
clue = {}
def rot(p, k):
    return tuple(p[(i + 4 - k) % 4] for i in range(4))
for i, l in enumerate(lines):
    c = l.split(",")
    p = tuple(0 if int(x, 2) == 65535 else int(x, 2) for x in c[:4])
    P.append(p)
    if int(c[4]) or int(c[5]) or int(c[6]):
        clue[(int(c[5]), int(c[4]))] = (i, rot(p, int(c[6])))
assert len(P) == 256 and len(clue) == 5
cluep = {v[0] for v in clue.values()}
corners = [i for i, p in enumerate(P) if p.count(0) == 2]
edges = [i for i, p in enumerate(P) if p.count(0) == 1]
inter = [i for i, p in enumerate(P) if p.count(0) == 0]
assert (len(corners), len(edges), len(inter)) == (4, 56, 196)
# ring description of border pieces: (inc, out, inward)
desc = {}
for i in edges:
    p = P[i]
    k = (4 - p.index(0)) % 4
    q = rot(p, k)
    assert q[0] == 0
    desc[i] = (q[3], q[1], q[2])
for i in corners:
    p = P[i]
    for k in range(4):
        q = rot(p, k)
        if q[0] == 0 and q[3] == 0:
            desc[i] = (q[2], q[1], None)
seam = Counter(desc[i][2] for i in edges)
print("pieces: 4 corners, 56 edges, 196 interior; clues at", sorted(clue), "; seam colours", len(seam))

# interior orientations available for block cells (all five clue pieces out)
ORI = [(i, rot(P[i], k)) for i in inter if i not in cluep for k in range(4)]
pat_cache = {}
def cands(pat):
    v = pat_cache.get(pat)
    if v is None:
        v = [(i, q) for i, q in ORI if all(w == -1 or q[s] == w for s, w in enumerate(pat))]
        pat_cache[pat] = v
    return v

NB = ((-1, 0), (0, 1), (1, 0), (0, -1))
OPP = (2, 3, 0, 1)
RING = [(0, c) for c in range(16)] + [(r, 15) for r in range(1, 16)] + [(15, c) for c in range(14, -1, -1)] + [(r, 0) for r in range(14, 0, -1)]
RIDX = {c: i for i, c in enumerate(RING)}
def inward_side(cell):
    r, c = cell
    if r == 0: return 2
    if r == 15: return 0
    if c == 0: return 1
    return 3

class Corner:
    def __init__(self, R, C):
        dr = 1 if R == 0 else -1
        dc = 1 if C == 0 else -1
        self.R, self.C = R, C
        self.win = [(R, C + dc), (R, C + 2 * dc), (R + dr, C), (R + 2 * dr, C)]  # h1 h2 v1 v2
        self.blk = [(R + dr, C + dc), (R + dr, C + 2 * dc), (R + 2 * dr, C + dc)]  # A B Cc
        self.cl = (R + 2 * dr, C + 2 * dc)
        self.clq = clue[self.cl][1]
        self.ridx = [RIDX[c] for c in self.win]
        self.cache = {}
    def solve(self, cols, order=(1, 2, 0), want="alive"):
        """cols: 4-tuple of inward colours of win cells, -1 = free.
        returns (nfills, nodes, set of free-side colour tuples of fills)"""
        fixed = {self.cl: self.clq}
        ecol = {w: c for w, c in zip(self.win, cols)}
        cells = [self.blk[j] for j in order]
        used = set()
        nodes = 0
        fills = 0
        need = set()
        def pat(cell):
            out = []
            for s in range(4):
                nb = (cell[0] + NB[s][0], cell[1] + NB[s][1])
                if nb in ecol:
                    out.append(ecol[nb])
                elif nb in fixed:
                    out.append(fixed[nb][OPP[s]])
                else:
                    out.append(-1)
            return tuple(out)
        def go(d):
            nonlocal nodes, fills
            if d == 3:
                fills += 1
                t = []
                for w, c in zip(self.win, cols):
                    if c == -1:
                        s = inward_side(w)
                        nbc = (w[0] + NB[s][0], w[1] + NB[s][1])
                        t.append(fixed[nbc][OPP[s]])
                need.add(tuple(t))
                return
            cell = cells[d]
            for i, q in cands(pat(cell)):
                if i in used: continue
                nodes += 1
                used.add(i); fixed[cell] = q
                go(d + 1)
                used.discard(i); del fixed[cell]
        go(0)
        return fills, nodes, need
    def get(self, mask, cols4):
        key = tuple(cols4[j] if mask >> j & 1 else -1 for j in range(4))
        v = self.cache.get(key)
        if v is None:
            f, n, need = self.solve(key)
            nmin = n
            if f == 0:
                nmin = min(self.solve(key, order=o)[1] for o in permutations(range(3)))
            v = (f, n, nmin, need)
            self.cache[key] = v
        return v
    def fills_list(self, cols4):
        """explicit fills (piece triples) for a full window, for the cross-block check"""
        fixed = {self.cl: self.clq}
        ecol = {w: c for w, c in zip(self.win, cols4)}
        cells = [self.blk[j] for j in (1, 2, 0)]
        out = []
        used = []
        def pat(cell):
            o = []
            for s in range(4):
                nb = (cell[0] + NB[s][0], cell[1] + NB[s][1])
                o.append(ecol[nb] if nb in ecol else (fixed[nb][OPP[s]] if nb in fixed else -1))
            return tuple(o)
        def go(d):
            if d == 3:
                out.append(frozenset(used)); return
            cell = cells[d]
            for i, q in cands(pat(cell)):
                if i in used: continue
                used.append(i); fixed[cell] = q
                go(d + 1)
                used.pop(); del fixed[cell]
        go(0)
        return out

CS = [Corner(0, 0), Corner(0, 15), Corner(15, 0), Corner(15, 15)]
MASKS = sorted(range(1, 16), key=lambda m: (bin(m).count("1"), m))

def min_core(co, cols4, pcs4=None):
    """smallest mask (by size) whose restriction has no fill. Relaxed: a free
    window cell accepts any colour. Returns (size, mask, nodes, nodes_min) or None if alive."""
    for m in MASKS:
        f, n, nmin, need = co.get(m, cols4)
        if f == 0:
            return bin(m).count("1"), m, n, nmin
    return None

def min_core_exact(co, cols4, pcs4):
    """as min_core, but a free window cell must be supplied by some edge piece
    not among the core's own pieces (colour availability; lateral joints ignored)."""
    for m in MASKS:
        f, n, nmin, need = co.get(m, cols4)
        if f == 0:
            return bin(m).count("1")
        avail = Counter(seam)
        for j in range(4):
            if m >> j & 1:
                avail[cols4[j]] -= 1
        ok = False
        for t in need:
            cnt = Counter(t)
            if all(avail[c] >= k for c, k in cnt.items()):
                ok = True; break
        if not ok:
            return bin(m).count("1")
    return None

# ---------------- Part A: chains
by_inc = {}
for i in edges:
    by_inc.setdefault(desc[i][0], []).append(i)
by_out = {}
for i in edges:
    by_out.setdefault(desc[i][1], []).append(i)
print("\nPART A. exact, 5-piece chains per corner (ring order: 2 edges, corner, 2 edges)")
alive_prod = 1.0
agg = Counter(); aggx = Counter()
for co in CS:
    k0 = RIDX[(co.R, co.C)]
    # ring positions k0-2 .. k0+2
    pos = [(k0 + d) % 60 for d in (-2, -1, 0, 1, 2)]
    cellpos = {RING[p]: j for j, p in enumerate(pos)}
    widx = [cellpos[w] for w in co.win]  # index in chain for each window cell
    chains = []
    for cp in corners:
        cin, cout, _ = desc[cp]
        for a in by_out.get(cin, []):
            for b in by_inc.get(cout, []):
                if b == a: continue
                for a2 in by_out.get(desc[a][0], []):
                    if a2 in (a, b): continue
                    for b2 in by_inc.get(desc[b][1], []):
                        if b2 in (a, b, a2): continue
                        chains.append((a2, a, cp, b, b2))
    n = len(chains)
    cntp = {m: Counter() for m in MASKS}
    for ch in chains:
        pcs = [ch[j] for j in widx]
        for m in MASKS:
            cntp[m][tuple(pcs[j] for j in range(4) if m >> j & 1)] += 1
    dead = 0
    size = Counter(); sizex = Counter(); bits = Counter(); nodes = Counter(); nodes_min = Counter()
    alive_n = 0
    for ch in chains:
        pcs = [ch[j] for j in widx]
        cols = tuple(desc[p][2] for p in pcs)
        r = min_core(co, cols)
        if r is None:
            alive_n += 1; continue
        dead += 1
        s, m, nd, ndm = r
        size[s] += 1; nodes[s] += nd; nodes_min[s] += ndm
        bits[s] += log2(n / cntp[m][tuple(pcs[j] for j in range(4) if m >> j & 1)])
        sizex[min_core_exact(co, cols, pcs)] += 1
    alive_prod *= alive_n / n
    print("corner (%2d,%2d): chains %d, alive %d (%.4f = %.2f bits)" % (co.R, co.C, n, alive_n, alive_n / n, -log2(alive_n / n)))
    for s in sorted(size):
        print("   min core %d cells: %5.1f%% of dead (colour-availability variant %5.1f%%) | mean placement bits %.2f | mean proof nodes: fixed order %.1f, best order %.1f"
              % (s, 100 * size[s] / dead, 100 * sizex[s] / dead, bits[s] / size[s], nodes[s] / size[s], nodes_min[s] / size[s]))
        agg[s] += size[s] / dead / 4; aggx[s] += sizex[s] / dead / 4
print("product of the four chain fractions: 2^-%.2f" % -log2(alive_prod))
print("mean share by min core size (per dead block):", {s: round(v, 3) for s, v in sorted(agg.items())}, " colour-availability variant:", {s: round(v, 3) for s, v in sorted(aggx.items())})
print("[%.1fs]" % (time.time() - t0)); sys.stdout.flush()
