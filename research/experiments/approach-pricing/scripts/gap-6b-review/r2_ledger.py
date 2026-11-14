"""Review of gap 6b, script 2: the memo's uniform ledger against a ledger with the
REAL colour counts (12 interior colours on 50 half-edges, 5 on 48, 5 frame colours
on 24 each), drawn without replacement.

Ensemble: the multiset of colours is dealt at random onto the half-edge slots.
P(e given slot pairs are all monochromatic) = e! [x^e] prod_c g_c(x) / N^(2e falling),
g_c(x) = sum_k n_c^(2k falling) x^k / k!.
Uniform ledger (the memo's): P = C^-e.

Outputs: totals, the row-scan level profile with 0, 1 and 5 clues against the
measured 10^50.04, 10^47.14, 10^40.457, and the memo's prices redone.
Usage: python3 -I r2_ledger.py
"""
from fractions import Fraction
from math import lgamma, log, log2, comb, factorial
LN2 = log(2)
lf = lambda n: lgamma(n + 1) / LN2
def lfall(n, k): return lf(n) - lf(n - k)
def l2(fr): return log2(fr.numerator) - log2(fr.denominator) if fr > 0 else float("-inf")

def match_table(counts, emax):
    """log2 P(e pairs monochromatic), e = 0..emax, sampling without replacement."""
    N = sum(counts)
    poly = [Fraction(1)]
    for n in counts:
        g = []
        ff = 1
        for k in range(0, n // 2 + 1):
            if k > 0: ff *= (n - 2 * k + 2) * (n - 2 * k + 1)
            g.append(Fraction(ff, factorial(k)))
        new = [Fraction(0)] * min(emax + 1, len(poly) + len(g) - 1)
        for i, a in enumerate(poly):
            for j, b in enumerate(g):
                if i + j < len(new): new[i + j] += a * b
        poly = new
    out = []
    den = 1
    for e in range(0, emax + 1):
        if e > 0: den *= (N - 2 * e + 2) * (N - 2 * e + 1)
        out.append(l2(poly[e] * factorial(e) / den) if e < len(poly) else float("-inf"))
    return out

INT = [50] * 12 + [48] * 5
FRM = [24] * 5
PI_real = match_table(INT, 420)
PF_real = match_table(FRM, 60)
PI_unif = [-e * log2(17) for e in range(421)]
PF_unif = [-e * log2(5) for e in range(61)]

def lsum(xs):
    m = max(xs)
    return m + log2(sum(2 ** (x - m) for x in xs))

print("== 1. full board")
H = lf(196) + 392 + lf(56) + lf(4)
for name, PI, PF in (("uniform (memo)", PI_unif, PF_unif), ("real counts", PI_real, PF_real)):
    print(f"{name:16s} constraint bits {-(PI[420]+PF[60]):8.2f}  (interior {-PI[420]:.2f}, frame {-PF[60]:.2f})   log2 E[Z] no clue {H+PI[420]+PF[60]:7.2f}   one clue {H+PI[420]+PF[60]-log2(784):7.2f}   five clues {H+PI[420]+PF[60]-sum(log2(4*(196-i)) for i in range(5)):7.2f}")
print(f"canonical (ROUTES row 65): no clue log2(1.153e7) = {log2(1.153e7):.2f}, one clue log2(14702.27) = {log2(14702.27):.2f}, five log2(4.1e-8) = {log2(4.1e-8):.2f}")
print(f"frames: uniform 4! 56! / 5^60 = {lf(56)+lf(4)+PF_unif[60]:.2f}; real counts {lf(56)+lf(4)+PF_real[60]:.2f}; exact count on file 124.93")

def kind(r, c):
    b = (r in (0, 15)) + (c in (0, 15))
    return b  # 2 corner, 1 edge, 0 interior

def profile(order, PI, PF, clues=()):
    """log2 level sizes for a static cell order. clues: set of cells holding a fixed tile."""
    filled = set(); ncor = nedg = nint = 0; eff = ein = 0
    lg = 0.0; out = []
    nclue_left = len(clues)
    for (r, c) in order:
        k = kind(r, c)
        if (r, c) in clues:
            nclue_left -= 1          # no choice here
        elif k == 2: lg += log2(4 - ncor)
        elif k == 1: lg += log2(56 - nedg)
        else: lg += log2(4 * (196 - nint - nclue_left))   # tiles reserved for later clues are not available
        if k == 2: ncor += 1
        elif k == 1: nedg += 1
        else: nint += 1
        for (dr, dc) in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            q = (r + dr, c + dc)
            if q in filled:
                if kind(*q) > 0 and k > 0: eff += 1
                else: ein += 1
        filled.add((r, c))
        out.append(lg + PI[ein] + PF[eff])
    return out

rowscan = [(r, c) for r in range(16) for c in range(16)]
print("\n== 2. row scan, first-moment level profile (interior clue tiles: no choice at the cell, tile withheld before)")
meas = {0: 50.04, 1: 47.14, 5: 40.457}
print('the centre clue is cell 120 of the scan (row 7, column 8 in scan coordinates) when the widest level sits near depth 158, as measured; the other placement is shown for comparison')
for nm, clues in (("no clue", ()), ("one clue (8,7)", {(8, 7)}), ("one clue (7,8)", {(7, 8)}), ("five clues, centre at (8,7)", {(2, 2), (2, 13), (13, 2), (13, 13), (8, 7)}), ("five clues, centre at (7,8)", {(2, 2), (2, 13), (13, 2), (13, 13), (7, 8)})):
    for name, PI, PF in (("uniform", PI_unif, PF_unif), ("real", PI_real, PF_real)):
        p = profile(rowscan, PI, PF, set(clues))
        pk = max(p); d = p.index(pk) + 1
        tot = lsum(p)
        m = meas[len(clues)]
        print(f"{nm:28s} {name:8s} peak level 2^{pk:6.2f} at depth {d:3d}; total 2^{tot:6.2f} = 10^{tot*log(2)/log(10):5.2f}  (measured 10^{m}; difference {tot - m*log(10)/log(2):+5.2f} bits); last level {p[-1]:6.2f}")

print("\n== 3. where the two ledgers part along the row scan (no clue): real minus uniform, bits")
pu = profile(rowscan, PI_unif, PF_unif); pr = profile(rowscan, PI_real, PF_real)
print("  " + "  ".join(f"d{d}:{pr[d-1]-pu[d-1]:+.2f}" for d in (16, 32, 64, 96, 128, 158, 192, 224, 240, 256)))

print("\n== 4. memo prices redone with real counts")
for (w, h) in ((7, 7), (14, 7), (14, 14)):
    s = w * h; e = 2 * w * h - w - h
    a = lfall(196, s) + 2 * s
    print(f"free-boundary {w}x{h} interior block: fillings uniform {a+PI_unif[e]:7.2f}, real {a+PI_real[e]:7.2f}")
Hrel = 196 * log2(784) + 56 * log2(56) + 4 * log2(4)
print(f"colour-only (relaxed) solutions: uniform {Hrel+PI_unif[420]+PF_unif[60]:.2f}, real {Hrel+PI_real[420]+PF_real[60]:.2f}; P(strict | relaxed) 2^-{Hrel-H:.1f} in both")
print("coarse puzzles (interior colours merged), log2 E[number of arrangements that match], frame at full resolution:")
arr = lf(196) + 392
for nm, cnts in (("17 classes", INT), ("one pair merged, 50+50", [100] + [50] * 10 + [48] * 5), ("one pair merged, 48+48", [96] + [50] * 12 + [48] * 3),
                 ("15 classes", [100, 100] + [50] * 8 + [48] * 5), ("6 classes (3,3,3,3,3,2)", [150, 150, 150, 150, 144, 96]), ("2 classes (9,8)", [50 * 9 - 6, 50 * 8 - 4 + 0]) ):
    if sum(cnts) != 840: cnts = [sum(INT[:9]), sum(INT[9:])]
    t = match_table(cnts, 420)[420]
    q = sum((x / 840) ** 2 for x in cnts)
    print(f"  {nm:26s} match bits real {-t:7.2f} (uniform-style 420 log2 1/q = {-420*log2(q):7.2f});  arrangements x P = {arr + t + lf(56)+lf(4)+PF_real[60]:7.2f}  [upper bound on distinct coarse solutions; see r3 for the 2-class entropy cap]")
print(f"residual at the peak (memo exit 2): surplus at depth 158 + log2 E[Z | five clues]: uniform ledger {max(profile(rowscan, PI_unif, PF_unif, {(2,2),(2,13),(13,2),(13,13),(8,7)})):.1f} and canonical -24.5")
