"""Piece-level facts quoted on the theory pages, recomputed from the CSV."""
import math, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from fractions import Fraction
from collections import Counter, defaultdict
from itertools import combinations
from e2lib import load, kind
n, P, H = load()
rot = lambda p, r: tuple(p[(i - r) % 4] for i in range(4))  # r quarter turns clockwise: top->right
INT = [p for p in P if kind(p) == "interior"]
EDG = [p for p in P if kind(p) == "edge"]
COR = [p for p in P if kind(p) == "corner"]

print("== symmetry ==")
print("pieces fixed by some non-trivial rotation:", sum(any(rot(p, r) == p for r in (1, 2, 3)) for p in P))
print("pieces with top==bottom and left==right:", sum(p[0] == p[2] and p[1] == p[3] for p in P))
canon = lambda p: min(rot(p, r) for r in range(4))
print("rotation-duplicate pairs:", sum(c * (c - 1) // 2 for c in Counter(canon(p) for p in P).values()))
ms = Counter(tuple(sorted(p)) for p in P)
print("same-multiset pairs (twins):", sum(c * (c - 1) // 2 for c in ms.values()), "distinct multisets", len(ms))

print("== near twins: pairs agreeing on exactly 3 of 4 sides in some relative rotation ==")
def agree(p, q):
    return max(sum(a == b for a, b in zip(p, rot(q, r))) for r in range(4))
def n_alignments(p, q, k):
    return sum(sum(a == b for a, b in zip(p, rot(q, r))) == k for r in range(4))
for name, S in (("all 256", P), ("interior", INT), ("edge", EDG), ("corner", COR)):
    pairs = [(p, q) for p, q in combinations(S, 2) if agree(p, q) == 3]
    al = sum(n_alignments(p, q, 3) for p, q in pairs)
    print(f"  {name}: pairs {len(pairs)}, (pair, rotation) alignments {al}")
mixed = sum(1 for p, q in combinations(P, 2) if kind(p) != kind(q) and agree(p, q) == 3)
print("  mixed-kind pairs:", mixed)
# border pieces: a swap of two edge pieces breaks exactly one joint each when they agree on 2 of their 3 coloured
# sides with grey aligned (that is 3 of 4 with grey), a corner pair when they agree on 1 of 2 coloured sides.
def edge_norm(p):
    g = p.index(0); return rot(p, (4 - g) % 4)  # grey on top
ee = sum(1 for p, q in combinations(EDG, 2) if sum(a == b for a, b in zip(edge_norm(p), edge_norm(q))) == 3)
def cor_norm(p):
    g = [i for i in range(4) if p[i] == 0 and p[(i + 1) % 4] == 0][0]; return rot(p, (4 - g) % 4)
cc = sum(1 for p, q in combinations(COR, 2) if sum(a == b for a, b in zip(cor_norm(p), cor_norm(q))) == 3)
print("  edge pairs differing in exactly one coloured side (grey aligned):", ee, "; corner pairs:", cc)

print("== in-place turns keeping exactly 2 of 4 sides (interior) ==")
half = [p for p in INT if sum(a == b for a, b in zip(p, rot(p, 2))) == 2]
q1 = [p for p in INT if sum(a == b for a, b in zip(p, rot(p, 1))) == 2]
q3 = [p for p in INT if sum(a == b for a, b in zip(p, rot(p, 3))) == 2]
print("  half-turn tiles:", len(half), " quarter-turn cw:", len(q1), " ccw:", len(q3), " tiles with any quarter:", len(set(q1) | set(q3)))
moves = sum(sum(a == b for a, b in zip(p, rot(p, r))) == 2 for p in INT for r in (1, 2, 3))
print("  distinct (tile, non-trivial rotation) moves keeping exactly 2 sides:", moves)
print("  quarter-turn tiles:", sorted(set(q1) | set(q3)))

print("== ring multigraph ==")
arcs = []
for p in EDG:
    g = p.index(0); arcs.append((p[(g - 1) % 4], p[(g + 1) % 4]))
for p in COR:
    g = [i for i in range(4) if p[i] == 0 and p[(i + 1) % 4] == 0][0]; arcs.append((p[(g - 1) % 4], p[(g + 2) % 4]))
A = Counter(arcs)
print("  self loops:", sum(v for (a, b), v in A.items() if a == b), " distinct ordered pairs:", len(A),
      " unordered pairs+loops:", len({tuple(sorted(k)) for k in A}))
cols = sorted({a for a, b in arcs})
L = [[Fraction((sum(A[(a, x)] for x in cols if x != a)) if a == b else -A[(a, b)]) for b in cols] for a in cols]
def det(M):
    M = [r[:] for r in M]; d = Fraction(1); m = len(M)
    for i in range(m):
        piv = next((r for r in range(i, m) if M[r][i] != 0), None)
        if piv is None: return Fraction(0)
        if piv != i: M[i], M[piv] = M[piv], M[i]; d = -d
        d *= M[i][i]
        for r in range(i + 1, m):
            f = M[r][i] / M[i][i]
            for c in range(i, m): M[r][c] -= f * M[i][c]
    return d
arb = det([r[1:] for r in L[1:]])
print("  arborescences:", arb, " BEST circuits log10:", math.log10(arb) + 5 * math.log10(math.factorial(11)),
      "=", float(arb) * math.factorial(11) ** 5)
print("  incident pieces per colour:", sorted(sum(1 for a, b in arcs if c in (a, b)) for c in cols))
print("  P(4 labelled arcs equally spaced among 60 cyclic slots) =", 15 * 24 / (60 * 59 * 58 * 57))
mult = sorted(A.values(), reverse=True)
print("  arc-type multiplicities (edge+corner):", mult)
Ae = Counter(arcs[:56])
poly = [1]
for m in Ae.values():
    new = [0] * (len(poly) + m)
    for i, c in enumerate(poly):
        for j in range(m + 1): new[i + j] += c
    poly = new
print("  edge-piece arc types:", len(Ae), " sub-multisets of any size:", sum(poly), " of size 14:", poly[14], " of size 28:", poly[28])
print("  inward demand colours 6..22:", [sum(1 for p in EDG if p[(p.index(0) + 2) % 4] == c) for c in range(6, 23)])

print("== piece theft: (north, west) demands on interior pieces ==")
dem = defaultdict(set); demrot = Counter()
for i, p in enumerate(INT):
    for r in range(4):
        q = rot(p, r); dem[(q[0], q[3])].add(i); demrot[(q[0], q[3])] += 1
print("  occurring demands:", len(dem), " single-server:", sum(len(v) == 1 for v in dem.values()),
      " mean distinct servers:", sum(len(v) for v in dem.values()) / len(dem), " mean oriented:", 784 / len(dem),
      " max:", max(len(v) for v in dem.values()), " histogram:", sorted(Counter(len(v) for v in dem.values()).items()))
print("  (N,W) pairs with zero oriented pieces:", 289 - len(dem), " with >=2 oriented:", sum(v >= 2 for v in demrot.values()))

print("== collision probabilities ==")
ci = Counter(c for p in INT for c in p)
print("  interior-tile collision:", sum(v * v for v in ci.values()) / 784 ** 2, " mu:", 784 * (sum(v * v for v in ci.values()) / 784 ** 2) ** 4)
lg10f = lambda k: math.lgamma(k + 1) / math.log(10)
print("  log10 W_geom:", math.log10(24) + lg10f(56) + lg10f(196) + 196 * math.log10(4), " /784 ->", 10 ** ((math.log10(24) + lg10f(56) + lg10f(196) + 196 * math.log10(4) - math.log10(784)) % 1), "e", int(math.log10(24) + lg10f(56) + lg10f(196) + 196 * math.log10(4) - math.log10(784)))
print("  group order log10 (196! 56! 4!/2):", lg10f(196) + lg10f(56) + math.log10(12))
print("  Owen I(16)=", math.exp((math.lgamma(197) + 196 * math.log(4)) / 392), " I(8)=", math.exp((math.lgamma(37) + 36 * math.log(4)) / 72))
for Lr in range(16, 23):
    print(f"  strip 2x{Lr}: log10 E =", lg10f(2 * Lr) + 2 * Lr * math.log10(4) - (3 * Lr - 2) * math.log10(17))
