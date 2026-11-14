"""Second analysis recomputation of the prices in gap-6a-reformulation-algebra.md. Stdlib only, exact integers
where possible (no lgamma). Run: python3 -I r_numbers.py <official.json>"""
import json, sys
from math import factorial, comb, log2
from fractions import Fraction
from collections import Counter

T = [tuple(p) for p in json.load(open(sys.argv[1]))["pieces"]]   # U,R,D,L ; 0 = grey
L2 = lambda x: log2(x) if x < 1 << 1000 else (x.bit_length() - 1) + log2(x >> (x.bit_length() - 53)) - 52
corner = [t for t in T if t.count(0) == 2]; edge = [t for t in T if t.count(0) == 1]; inner = [t for t in T if 0 not in t]
print("census", len(corner), len(edge), len(inner))

print("\n-- section 3 baseline")
print("budget of terms: 134.4 - 75.4 = %.1f" % (134.4 - 75.4))
fk = lambda n: L2(n ** n) - L2(factorial(n))
tot = fk(4) + fk(56) + fk(196)
print("prod n^n/n! = 2^%.1f (digits %.2f) ; types: %.1f %.1f %.1f" % (tot, tot * log2(2) / log2(10), fk(4), fk(56), fk(196)))
print("  path 1 says 'between 10^102 and 10^108': 10^102 = 2^%.1f" % (102 * log2(10)))
print("  measured toy correction 10^1.1..10^1.9 would give 2^%.1f .. 2^%.1f" % (tot - 1.9 * log2(10), tot - 1.1 * log2(10)))
print("  sign samples Z/N^2 with N=14702: 2^%.1f ; with N=1: 2^%.1f" % (tot + log2(14702) - 2 * log2(14702), tot))

print("\n-- rows 2, 3, 6, 7 (oblivious)")
for n in (251, 256):
    print("n=%d: 2^n; C(n,n//2)=2^%.2f; sum_{i<=n/2} C(n,i)=2^%.2f; 2^n/n=2^%.2f; 2^n/sqrt(n)=2^%.2f; Waring 2^(n-1)"
          % (n, L2(comb(n, n // 2)), L2(sum(comb(n, i) for i in range(n // 2 + 1))), n - log2(n), n - log2(n) / 2))
print("row 2: 251+75.4 = %.1f ; layered (Nisan) %.1f" % (251 + 75.4, L2(comb(251, 125)) + 75.4))
print("row 6: 251 - log2(251) + 75.4 = %.1f" % (251 - log2(251) + 75.4))

print("\n-- row 8: Bell(256)")
row = [1]
for _ in range(255):
    new = [row[-1]]
    for x in row: new.append(new[-1] + x)
    row = new
print("Bell(256) = 2^%.1f" % L2(row[-1]))
# refinements of the kernel of a typical colour-only board: uniform random map 256 cells -> 256 tiles
bell = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147]
from math import exp
lam = 1.0; extra = 0.0
for k in range(2, 10):
    extra += 256 * exp(-lam) * lam ** k / factorial(k) * log2(bell[k])
print("row 8: partitions refining the kernel of one typical colour-only board (Poisson(1) loads): 2^%.0f per board" % extra)

print("\n-- row 11: spaces")
place = L2(factorial(4)) + L2(factorial(56)) + L2(factorial(196)) + 2 * 196
half = Counter(c for t in T for c in t if c)
dfact = lambda m: factorial(m) // (2 ** (m // 2) * factorial(m // 2))
pair = sum(L2(dfact(v)) for v in half.values())
print("placements 2^%.1f ; half-edge pairings 2^%.1f ; edge colourings 2^%.1f ; half-edges %d"
      % (place, pair, 60 * log2(5) + 420 * log2(17), sum(half.values())))
print("half-edge census:", sorted(half.items()))

print("\n-- row 12: row systems (first moment, exact rational collision probabilities)")
ic = Counter(c for t in inner for c in t); S = sum(ic.values())
p_ii = sum(Fraction(v, S) ** 2 for v in ic.values())
inw = Counter(t[(t.index(0) + 2) % 4] for t in edge)
p_fi = sum(Fraction(inw[c], 56) * Fraction(ic[c], S) for c in ic)
# frame-frame: the two frame sides of edge pieces and corners
fc = Counter()
for t in edge:
    z = t.index(0); fc[t[(z + 1) % 4]] += 1; fc[t[(z - 1) % 4]] += 1
for t in corner:
    for c in t:
        if c: fc[c] += 1
SF = sum(fc.values()); p_ff = sum(Fraction(v, SF) ** 2 for v in fc.values())
lii, lfi, lff = -log2(p_ii), -log2(p_fi), -log2(p_ff)
print("match cost bits: ii %.3f  fi %.3f  ff %.3f (memo uses log2 5 = %.3f for ff)" % (lii, lfi, lff, log2(5)))
one = log2(56 * 55) + L2(factorial(196) // factorial(182)) + 28 - 13 * lii - 2 * lfi
print("one interior row candidates 2^%.1f" % one)
print("horizontal-only boards 2^%.1f (memo ff) / 2^%.1f (census ff); vertical must remove %.1f"
      % (place - 30 * log2(5) - 28 * lfi - 182 * lii, place - 30 * lff - 28 * lfi - 182 * lii,
         30 * log2(5) + 28 * lfi + 182 * lii))

print("\n-- row 13: stars")
print("stars per deep vertex 2^%.1f ; corners %d = 225*4 + %d on the rim ; k-dim matching 2^((k-2)/k * n) with k=4, n=1024: 2^%d"
      % (4 * log2(784) - 4 * lii, 1024, 1024 - 900, 512))

print("\n-- row 15: XL monomials")
V = 4 * 4 + 56 * 56 + 196 * 196 * 4
print("V =", V)
for D in range(1, 10):
    b = L2(comb(V, D))
    print("  D=%d  C(V,D)=2^%.1f  x2: %.1f  x2.37: %.1f   %s" % (D, b, 2 * b, 2.37 * b,
          "under 134.4 at omega=2" if 2 * b < 134.4 else "over"))

print("\n-- row 5: checkerboard, white strict, black colour-only on the tiles white left")
print("live terms per solution (first moment): 2^%.1f (types %.1f %.1f %.1f); black free over all tiles: 2^%.1f"
      % (fk(2) + fk(28) + fk(98), fk(2), fk(28), fk(98),
         (2 * 2 + 28 * log2(56) + 98 * log2(196)) - L2(factorial(2) * factorial(28) * factorial(98))))

print("\n-- row 10: oracle calls  2^%.1f .. 2^%.1f" % (log2(2560), log2(256 * 784)))
print("-- half turn: 17^-4 = %.2e ; x 4,059,952 = %.1f" % (17 ** -4, 4059952 / 17 ** 4))
