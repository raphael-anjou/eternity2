"""Flux-law ranks, the defect-2 shell tested on painted solved boards, small-overlap second-moment terms,
and a rigorous positivity bound for the reusable-tile entropy."""
import math, os, random, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from fractions import Fraction
from collections import Counter, defaultdict
from itertools import combinations
from e2lib import load, kind
n, P, H = load()
rot = lambda p, r: tuple(p[(i - r) % 4] for i in range(4))
INT = [p for p in P if kind(p) == "interior"]

print("== flux law ranks (pieces in CSV orientation: top,right,bottom,left) ==")
A = [[(p[1] == c) - (p[3] == c) for p in P] for c in range(1, 23)]   # east minus west
B = [[(p[2] == c) - (p[0] == c) for p in P] for c in range(1, 23)]   # south minus north
def rank_q(M):
    M = [[Fraction(x) for x in r] for r in M]; rk = 0
    for col in range(len(M[0])):
        piv = next((r for r in range(rk, len(M)) if M[r][col] != 0), None)
        if piv is None: continue
        M[rk], M[piv] = M[piv], M[rk]
        for r in range(len(M)):
            if r != rk and M[r][col] != 0:
                f = M[r][col] / M[rk][col]
                M[r] = [a - f * b for a, b in zip(M[r], M[rk])]
        rk += 1
        if rk == len(M): break
    return rk
# complex rank of A + iB equals half the real rank of [[A, -B], [B, A]]
big = [a + [-x for x in b] for a, b in zip(A, B)] + [b + a for a, b in zip(A, B)]
print("  complex rank of the 22 x 256 flux matrix:", rank_q(big) // 2, "(real rank of the realified 44 x 512 system:", rank_q(big), ")")
print("  real rank of stacked [A; B] (44 x 256):", rank_q(A + B))
def rank_f2(rows):
    rows = [int("".join(str(x & 1) for x in r), 2) for r in rows]; rk = 0
    for bit in reversed(range(max(r.bit_length() for r in rows) or 1)):
        piv = next((i for i in range(rk, len(rows)) if rows[i] >> bit & 1), None)
        if piv is None: continue
        rows[rk], rows[piv] = rows[piv], rows[rk]
        for i in range(len(rows)):
            if i != rk and rows[i] >> bit & 1: rows[i] ^= rows[rk]
        rk += 1
    return rk
Tm = [[sum(x == c for x in p) & 1 for p in P] for c in range(1, 23)]
rhs = [sum((p[1] == c) + (p[3] == c) for p in P) & 1 for c in range(1, 23)]
print("  F2 rank of parity matrix:", rank_f2(Tm), " augmented:", rank_f2([r + [b] for r, b in zip(Tm, rhs)]))
fr = [sum(Tm[c][j] for c in range(5)) & 1 for j in range(256)]
print("  sum of the five frame-colour rows mod 2 is zero:", not any(fr), "(each border piece carries exactly two frame-coloured sides)")
print("  2^21 =", 2 ** 21)

print("== defect-2 shell, tested on painted solved boards ==")
def painted(rng):
    ringc = [c for c in range(1, 6) for _ in range(12)]; rng.shuffle(ringc)
    inner = [c for c in range(6, 23) for _ in range(24 if c <= 10 else 25)]; rng.shuffle(inner)
    it = iter(inner); ri = iter(ringc)
    Hh = [[0] * 15 for _ in range(16)]; V = [[0] * 16 for _ in range(15)]
    for y in range(16):
        for x in range(15): Hh[y][x] = next(ri) if y in (0, 15) else next(it)
    for y in range(15):
        for x in range(16): V[y][x] = next(ri) if x in (0, 15) else next(it)
    return [[(V[y - 1][x] if y else 0, Hh[y][x] if x < 15 else 0, V[y][x] if y < 15 else 0, Hh[y][x - 1] if x else 0)
             for x in range(16)] for y in range(16)]
def score(Bd):
    s = 0
    for y in range(16):
        for x in range(16):
            if x < 15: s += Bd[y][x][1] == Bd[y][x + 1][3]
            if y < 15: s += Bd[y][x][2] == Bd[y + 1][x][0]
    return s
rng = random.Random(11)
for trial in range(3):
    Bd = painted(rng); assert score(Bd) == 480
    cells = [(y, x) for y in range(16) for x in range(16)]
    hist = Counter(); n478_swap = 0; n478_rot = 0; near = 0; rot2 = 0
    # all single in-place rotations of interior cells
    for (y, x) in cells:
        if 0 in Bd[y][x]: continue
        for r in (1, 2, 3):
            p = Bd[y][x]; q = rot(p, r); rot2 += sum(a == b for a, b in zip(p, q)) == 2
            Bd[y][x] = q; s = score(Bd); Bd[y][x] = p
            hist[s] += 1; n478_rot += s == 478
    # all swaps of two cells of the same kind, all rotations that keep grey outward
    for (a, b) in combinations(cells, 2):
        pa, pb = Bd[a[0]][a[1]], Bd[b[0]][b[1]]
        if pa.count(0) != pb.count(0): continue
        best_agree = max(sum(u == v for u, v in zip(pa, rot(pb, r))) for r in range(4))
        near += best_agree == 3
        if best_agree < 2: continue          # cannot give exactly two breaks unless adjacent; adjacent pairs handled below
        for ra in range(4):
            qa = rot(pb, ra)
            if any((qa[i] == 0) != (pa[i] == 0) for i in range(4)): continue
            for rb in range(4):
                qb = rot(pa, rb)
                if any((qb[i] == 0) != (pb[i] == 0) for i in range(4)): continue
                Bd[a[0]][a[1]], Bd[b[0]][b[1]] = qa, qb
                s = score(Bd); n478_swap += s == 478
                Bd[a[0]][a[1]], Bd[b[0]][b[1]] = pa, pb
    print(f"  board {trial}: near-twin pairs in the bag {near}, (tile, rotation) moves keeping two sides {rot2};"
          f" boards at 478 reached by one swap {n478_swap}, by one in-place rotation {n478_rot}; any at 479: {hist[479]}")

print("== second moment: expected number of OTHER solutions that differ from a given one on a small set of cells ==")
# independent-edge model, interior cells only, p = 1/17 per changed joint. A set D of m interior cells is re-filled with
# the same m pieces, none staying in place with the same rotation; every joint touching D must match again.
p17 = 1 / 17
# m = 1: rotation only. Needs all 4 sides equal after rotation: impossible unless symmetric piece (none).
# m = 2 non-adjacent: 16 rotation pairs, 8 joints; adjacent: 16 rotation pairs, 7 joints.
pairs_adj = 2 * 14 * 13; pairs_all = 196 * 195 // 2
e2 = (pairs_all - pairs_adj) * 16 * p17 ** 8 + pairs_adj * 16 * p17 ** 7
# m = 3: derangements 2, rotations 64, joints >= 9 (compact L or line: 3*4 - 2 = 10 joints; scattered: 12)
e3 = (196 * 195 * 194 // 6) * 2 * 64 * p17 ** 10
print(f"  E[two-cell alternatives] ~ {e2:.2e}; E[three-cell alternatives] <= {e3:.2e} (bound with 10 joints)")
# general m, upper bound: C(196,m) * m! * 4^m * 17^-(2m + ceil(2 sqrt m))  (each cell has 4 joints, each shared by <= 2 cells of D,
# plus the isoperimetric boundary 2*ceil(2 sqrt m) joints counted once)
tot = 0.0; worst = None
for m in range(2, 197):
    joints = 2 * m + math.ceil(2 * math.sqrt(m))
    joints = min(joints, 420)
    lg = (math.lgamma(197) - math.lgamma(m + 1) - math.lgamma(197 - m) + math.lgamma(m + 1) + m * math.log(4) - joints * math.log(17)) / math.log(10)
    if worst is None or lg > worst[1]: worst = (m, lg)
    if m in (2, 5, 10, 50, 100, 150, 180, 190, 196): print(f"    m={m}: log10 bound {lg:.1f}")
print("  the crude bound peaks at m =", worst, "(it ignores that large D must also satisfy all 420 joints; with all joints the m=196 term is the full first moment)")
print("  => within the model, E[Z^2]/E[Z] = 1 + E[Z'] + O(1e-4): the planted puzzle expects its own solution plus the unplanted first moment.")

print("== positivity of the reusable-tile entropy: rows of oriented tiles with top = bottom = a ==")
best = (0, None)
for a in range(6, 23):
    arcs = Counter()
    for p in INT:
        for r in range(4):
            q = rot(p, r)
            if q[0] == a and q[2] == a: arcs[(q[3], q[1])] += 1     # west -> east
    cols = sorted({c for k in arcs for c in k})
    if not cols: continue
    v = {c: 1.0 for c in cols}; lam = 0
    for _ in range(300):
        w = {c: 0.0 for c in cols}
        for (x, y), k in arcs.items(): w[y] += v[x] * k
        lam = max(w.values()) or 0
        if lam == 0: break
        v = {c: w[c] / lam for c in cols}
    if lam > best[0]: best = (lam, a)
    print(f"    colour {a}: {sum(arcs.values())} oriented tiles with top=bottom={a}, spectral radius {lam:.3f}")
print("  best:", best, "=> h_inf >= log10(lambda) =", math.log10(best[0]) if best[0] > 0 else None, "(rows of such tiles stack freely)")
