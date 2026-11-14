"""Prices for the gap-6a memo (algebra and counting reformulations). Stdlib only.
Run: python3 -I prices.py <path to research/starter-kit/data/official.json>
Every figure is a first-moment (model) figure on the real colour census unless marked exact."""
import json, sys
from math import lgamma, log, log2, comb
from collections import Counter

P = json.load(open(sys.argv[1]))["pieces"]          # URDL, 0 = grey rim
LN2 = log(2)
def lf2(n): return lgamma(n + 1) / LN2               # log2 n!
def ldf2(m): return lf2(m) - (m // 2) * 1.0 - lf2(m // 2)   # log2 (m-1)!! for even m = m!/(2^(m/2) (m/2)!)

corners = [p for p in P if p.count(0) == 2]
edges = [p for p in P if p.count(0) == 1]
inner = [p for p in P if p.count(0) == 0]
print("exact census: corners %d, edge pieces %d, interior %d" % (len(corners), len(edges), len(inner)))

# half-edge census (exact): every non-grey side of every tile is a half-edge
half = Counter(c for p in P for c in p if c)
frame_cols = sorted({c for p in edges + corners for i, c in enumerate(p) if c and
                     (p[(i + 1) % 4] == 0 or p[(i - 1) % 4] == 0)})
print("half-edges per colour (exact):", dict(sorted(half.items())))
assert all(v % 2 == 0 for v in half.values()), "odd colour class"
print("sum of half-edges %d = 2 x %d internal edges" % (sum(half.values()), sum(half.values()) // 2))

print("\n[A] search-space sizes of the dual formulations, bits")
place = lf2(4) + lf2(56) + lf2(196) + 2 * 196
print("  placements (cells -> tiles, rotations): %.1f" % place)
pair = sum(ldf2(v) for v in half.values())
print("  half-edge pairings inside colour classes (tiles as vertices, colours as matchings): %.1f" % pair)
print("    of which frame colours %s: %.1f" % (frame_cols, sum(ldf2(half[c]) for c in frame_cols)))
print("  colourings of the 480 internal edges (chi): %.1f" % (60 * log2(5) + 420 * log2(17)))

# colour-only boards (tiles reusable), path-1 law n^n/n! per type times solutions
def fake(n): return n * log2(n) - lf2(n)
fr = fake(4) + fake(56) + fake(196)
print("\n[B] colour-only boards per solution (model): 2^%.1f ; with 14,702 solutions 2^%.1f ; with ~1: 2^%.1f"
      % (fr, fr + log2(14702), fr))
print("    path-1 toy correction (law high by 10^1.1 to 10^1.9 on toys): floor 2^%.1f" % (fr - 5.65 * log2(10)))

n = 251
print("\n[C] oblivious algebra prices, bits (251 free tiles, contraction 2^75.4)")
print("  exterior algebra / any module where the full product survives: dim 2^%d ; x contraction = %.1f" % (n, n + 75.4))
print("  Clifford (Majorana) spinor module: dim 2^%.1f ; trace over a basis x state x contraction = %.1f"
      % (n / 2, n / 2 + n / 2 + 75.4))
print("  sum of determinants over colour-only-feasible edge colourings: about %.0f to %.0f terms (bits)"
      % (fr - 5.65 * log2(10), fr + log2(14702)))
print("  central binomial C(256,128) (Nisan width of the symmetrised monomial): 2^%.1f" % log2(comb(256, 128)))
# Bell number B_256 via Dobinski-free recurrence (exact big ints)
row = [1]
for _ in range(255):
    new = [row[-1]]
    for x in row: new.append(new[-1] + x)
    row = new
print("  set partitions of 256 cells (index set of the homomorphism-basis sieve): 2^%.1f" % log2(row[-1]))

print("\n[D] row systems (horizontal matching only), model")
p_ii = sum((v / sum(Counter(c for p in inner for c in p).values())) ** 2
           for v in Counter(c for p in inner for c in p).values())
l_ii = -log2(p_ii)
inw = Counter(p[(p.index(0) + 2) % 4] for p in edges)
inn = Counter(c for p in inner for c in p)
p_fi = sum(inw[c] / 56 * inn[c] / sum(inn.values()) for c in inn); l_fi = -log2(p_fi)
print("  match cost: interior-interior %.3f bits, edge piece inward to interior %.3f bits, frame-frame %.3f"
      % (l_ii, l_fi, log2(5)))
one_row = log2(56 * 55) + (lf2(196) - lf2(182)) + 2 * 14 - 13 * l_ii - 2 * l_fi
print("  candidates for ONE interior row (edge, 14 interior, edge; horizontal matches only): 2^%.1f" % one_row)
# horizontal edges: rows 0 and 15 have 15 frame-colour edges each; each interior row has
# 2 edge-piece-inward to interior edges and 13 interior-interior edges
hsys = place - 30 * log2(5) - 28 * l_fi - 182 * l_ii
print("  boards with all 240 horizontal edges matched (each tile once, verticals ignored): 2^%.1f" % hsys)
vleft = 30 * log2(5) + 28 * l_fi + 182 * l_ii
print("  bits the 240 vertical edges must then remove: %.1f" % vleft)

print("\n[E] vertex stars (four tile corners around one interior board vertex, 4 matched edges), deep interior")
star = 4 * log2(196 * 4) - 4 * l_ii
print("  candidate stars per vertex: 2^%.1f ; 169 deep-interior vertices" % star)

print("\n[F] linear algebra on lifted variables (XL / Nullstellensatz / polynomial calculus), monomial counts")
V = 4 * 4 + 56 * 56 + 196 * 196 * 4      # cell x tile x legal rotation: corners 1 rot, edges 1 rot, interior 4
print("  0/1 variables (cell, tile, rotation) with rim respected: %d" % V)
for D in range(4, 13):
    b = lf2(V) - lf2(V - D) - lf2(D)
    print("    degree %2d: C(V,D) = 2^%.1f, squared (dense elimination, omega=2) 2^%.1f" % (D, b, 2 * b))

print("\n[G] self-reduction with a decision or counting oracle")
print("  calls, naive (every cell, every tile-rotation): at most 2^%.1f ; with the count oracle and bisection"
      " on a sorted candidate list: 256 x about 10 = 2^%.1f" % (log2(256 * 784), log2(2560)))
print("\n[H] budget: 2^134.4 tree, 2^75.4 contraction => a sieve must have fewer than 2^%.1f terms" % (134.4 - 75.4))
