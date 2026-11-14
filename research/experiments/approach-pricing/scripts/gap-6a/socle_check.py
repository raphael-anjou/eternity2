"""Brute-force check of the lemma used in the gap-6a memo, over GF(2), n = 4 generators.
Lemma: in the exterior algebra on n generators every nonzero element a has a monomial multiple
equal to the top form w = e_1 ^ ... ^ e_n. Hence a module vector m with w.m != 0 has a.m != 0 for
every nonzero a, so the map a -> a.m is injective and the module has dimension at least 2^n.
An element is a set of subsets (bitmask over the 2^n basis monomials). Stdlib only, under a second."""
n = 4; FULL = (1 << n) - 1
def mul_mono(B, a):                      # e_B ^ a over GF(2): keep A disjoint from B, map to A|B
    out = 0
    for A in range(1 << n):
        if a >> A & 1 and A & B == 0: out ^= 1 << (A | B)
    return out
bad = 0
for a in range(1, 1 << (1 << n)):
    if not any(mul_mono(B, a) == 1 << FULL for B in range(1 << n)): bad += 1
print("n = %d: nonzero elements %d, elements with no monomial multiple equal to the top form: %d"
      % (n, (1 << (1 << n)) - 1, bad))
