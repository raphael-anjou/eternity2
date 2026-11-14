"""Second analysis checks of Lemma A. Stdlib only, a few seconds. Run: python3 -I r_lemmaA.py
(a) The lemma itself with signs: in the exterior algebra over GF(3) (n = 3, exhaustive) and over the
    integers (n = 6, random sparse elements), every nonzero a has a monomial e_B with e_B a = +-c w.
(b) Its hypothesis is not the general one: linear operators that kill every repeat and keep every full
    product need not anticommute. Smallest case n = 2: a device of dimension 2, not 4.
(c) The bound that does hold for any such device: the matrix (ordered prefix, ordered suffix) -> [letters
    all distinct] has rank C(n, k) at layer k, and at least C(n, k) when the allowed entries carry any
    nonzero values. The layered used-set table reaches it."""
import random, sys
from itertools import permutations, product
from math import comb

def sign_mul(B, A):            # e_B ^ e_A for disjoint bitmasks: sign of sorting B then A
    s = 0
    for b in range(B.bit_length()):
        if B >> b & 1: s += bin(A & ((1 << b) - 1)).count("1")
    return -1 if s & 1 else 1
def mul_mono(B, a, mod=None):  # a: dict mask -> coeff
    out = {}
    for A, c in a.items():
        if A & B == 0:
            v = out.get(A | B, 0) + sign_mul(B, A) * c
            if mod: v %= mod
            if v: out[A | B] = v
            else: out.pop(A | B, None)
    return out

n = 3; full = (1 << n) - 1; bad = 0; cnt = 0
for coeffs in product(range(3), repeat=1 << n):
    a = {A: c for A, c in enumerate(coeffs) if c}
    if not a: continue
    cnt += 1
    if not any(set(mul_mono(B, a, 3)) == {full} for B in range(1 << n)): bad += 1
print("(a) GF(3), n = 3: %d nonzero elements, without a monomial multiple equal to +-c w: %d" % (cnt, bad))
random.seed(1); n = 6; full = 63; bad = 0
for _ in range(20000):
    a = {random.randrange(64): random.choice((-2, -1, 1, 2, 3)) for _ in range(random.randint(1, 12))}
    low = min(a, key=lambda A: bin(A).count("1"))
    r = mul_mono(full ^ low, a)
    if set(r) != {full}: bad += 1
print("    integers, n = 6: 20000 random elements, complement of a least-degree monomial fails: %d" % bad)

# (b) n = 2, dimension 2. Basis (p, q). X1: q -> p, p -> 0. X2: p -> q, q -> 0. v = p + q, readout l = sum.
X = {1: ((0, 1), (0, 0)), 2: ((0, 0), (1, 0))}
def app(Mx, v): return tuple(sum(Mx[i][j] * v[j] for j in range(2)) for i in range(2))
v = (1, 1)
print("(b) n = 2 device of dimension 2:")
for w in product((1, 2), repeat=2):
    x = v
    for t in w: x = app(X[t], x)          # first letter acts first
    print("    word %s -> vector %s readout %d" % (w, x, sum(x)))
print("    X1X2 + X2X1 = identity, so the operators do not anticommute and the space is not a module over the exterior algebra")

# (c) rank at layer k of the prefix-by-suffix matrix, mod a prime
P = 1000003
def rank(rows):
    rows = [r[:] for r in rows]; rk = 0; ncol = len(rows[0])
    for c in range(ncol):
        piv = next((i for i in range(rk, len(rows)) if rows[i][c] % P), None)
        if piv is None: continue
        rows[rk], rows[piv] = rows[piv], rows[rk]
        inv = pow(rows[rk][c], P - 2, P)
        rows[rk] = [x * inv % P for x in rows[rk]]
        for i in range(len(rows)):
            if i != rk and rows[i][c] % P:
                f = rows[i][c]; rows[i] = [(x - f * y) % P for x, y in zip(rows[i], rows[rk])]
        rk += 1
    return rk
for n in (4, 5, 6):
    out = []
    for k in range(n + 1):
        pre = list(permutations(range(n), k)); suf = list(permutations(range(n), n - k))
        if len(pre) * len(suf) > 200000: out.append("k=%d skipped" % k); continue
        random.seed(k)
        # generic nonzero values on the allowed entries: the bound does not need them equal
        Mx = [[(random.randrange(1, P) if set(p).isdisjoint(s) else 0) for s in suf] for p in pre]
        Mo = [[(1 if set(p).isdisjoint(s) else 0) for s in suf] for p in pre]
        out.append("k=%d: 0/1 rank %d, generic-values rank %d, C(n,k) %d" % (k, rank(Mo), rank(Mx), comb(n, k)))
    print("(c) n = %d: %s" % (n, "; ".join(out)))
print("    sum over layers = 2^n; the widest layer is C(n, n/2). For n = 251: 2^246.7 against 2^251.")
