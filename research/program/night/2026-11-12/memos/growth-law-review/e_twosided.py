"""Claim E: two-sided search in the first-moment model, checked numerically.

python3 -I e_twosided.py
1. Interior-only exact product against the continuum rates g(x) and (ln 2 - 1 - ln x)/2.
2. Whole board with the frame (row-major top half against the row-major peak), naive profile.
3. Measured: consistent top half-boards against the DFS peak and the whole tree on enumerated 6x6 boards.
"""
import math, os, random, statistics as st, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rvlib as rv
from a_dfs_model import profile


def ln_profile(n, c, cf=5):
    out = [0.0]
    for m, a, f in profile(n, c, cf):
        out.append(out[-1] + math.log(m) - a * math.log(c) - f * math.log(cf))
    return out

print("1. interior only, P pieces, two closed seams per cell: ln A(m) = sum_{i<m} ln(4 (P - i) / c^2)")
print("   x | P | peak rate ln A*/P (continuum g(x)) | half rate ln A(P/2)/P (continuum (ln2 - 1 - ln x)/2) | gain in nats")
for x in (1 / math.e, 0.30, 0.45, 0.50, 0.55, 0.70):
    for P in (36, 196, 1024, 16384, 262144):
        c2 = 4 * P * x
        ln = [0.0]
        for i in range(P):
            ln.append(ln[-1] + math.log(4 * (P - i) / c2))
        pk = max(ln)
        half = max(ln[: P // 2 + 1])
        print("   %.3f | %d | %.4f (%.4f) | %.4f (%.4f) | %.2f" % (x, P, pk / P, x - 1 - math.log(x), half / P, max(0.0, 0.5 * (math.log(2) - 1 - math.log(x))) if x < 0.5 else x - 1 - math.log(x), pk - half))

print()
print("2. whole board with the 5-colour frame, row-major: ln A at the peak, ln of the largest count up to half depth, difference")
print("   n c | ln A* | max over d <= N/2 of ln A(d) | gain in nats | bulk prediction P (g - (ln2 - 1 - ln x)/2)")
for n, c in ((6, 4), (6, 5), (8, 7), (8, 8), (10, 10), (12, 12), (16, 17), (16, 18), (24, 27), (32, 37), (64, 77)):
    lnA = ln_profile(n, c)
    N = n * n
    pk = max(lnA)
    half = max(lnA[: N // 2 + 1])
    P = (n - 2) ** 2
    x = c * c / (4.0 * P)
    pred = P * ((x - 1 - math.log(x)) - 0.5 * (math.log(2) - 1 - math.log(x))) if x < 0.5 else 0.0
    print("   %d %d | %.1f | %.1f | %.1f | %.1f" % (n, c, pk, half, pk - half, pred))

print()
print("3. measured on enumerated 6x6 boards: consistent top halves (depth 18) against the row-major peak and the whole tree")
rng = random.Random(5)
gen = []
while len(gen) < 8:
    p, _ = rv.planted_board(6, 4, 5, rng)
    gen.append(("g%d" % len(gen), 6, p))
for label, boards in (("6x6 c=4 (review generator, 8 boards)", gen),
                      ("6x6 c=5 (committed primary, 30)", [(a, b, p) for a, b, p, _ in rv.ladder_instances("primary", 6)])):
    res = rv.run_enum("full", 10**10, boards)
    Zm = [st.mean(r["Z"][d] for r in res) for d in range(37)]
    pk = max(range(37), key=lambda d: Zm[d])
    up_to_half = sum(Zm[1:19])
    print("   %s: mean Z(18) = %.4g, peak Z(%d) = %.4g, nodes to list all top halves = %.4g, whole tree = %.4g, ratio whole / (2 x halves) = %.2f" % (
        label, Zm[18], pk, Zm[pk], up_to_half, sum(Zm[1:]), sum(Zm[1:]) / (2 * up_to_half)))
