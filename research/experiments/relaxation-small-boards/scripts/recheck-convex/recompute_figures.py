"""re-check recomputation of the derived figures of relaxation-theory.md and relaxation-engineering.md.
Stdlib only, no board is built: formulas of the theory memo's Appendix A, the engineering memo's
Appendix F inputs (typed in here from the memo text, not read from its raw files), and the
iteration counts quoted in the two memos.

Usage: python3 -I recompute_figures.py
"""
import math

GIB, GB = 2.0 ** 30, 1e9
YEAR = 365.25 * 86400


def board(n, cf, ci):
    E, I = 4 * (n - 2), (n - 2) ** 2
    N = n * n
    P = 16 + E * E + 4 * I * I
    c2 = lambda a: a * (a - 1) // 2
    same_cell = 4 * c2(4) + E * c2(E) + I * c2(4 * I)
    same_tile = 4 * c2(4) + E * c2(E) + I * (c2(4 * I) - 6 * I)
    adj = (2 * (n - 2) * (n - 3) * (16 * I * I - 16 * I) * (1 - 1 / ci) + 4 * (n - 2) * (4 * E * I) * (1 - 1 / ci)
           + 4 * (n - 3) * (E * E - E) * (1 - 1 / cf) + 32 * E * (1 - 1 / cf))
    Z = same_cell + same_tile + adj
    pairs = c2(P)
    rho = Z / pairs
    return dict(n=n, N=N, P=P, Z=Z, F=pairs - Z, rho=rho, R=math.sqrt(P * (1 - rho) / rho) / N,
                tile_share=same_tile / Z, I=I)


print("== theory memo, section 1 table and section 4")
for key in [(4, 2, 4), (5, 3, 4), (6, 4, 5), (7, 4, 7), (8, 5, 8), (10, 5, 10), (12, 5, 12), (16, 5, 17)]:
    b = board(*key)
    print("%2dx%-2d (%d,%2d)  P %7d  P/N %5.1f  zeros %.4g  share %5.1f%%  6/N %5.1f%%  R %.2f  tile-rule share of zeros %.2f"
          % (key[0], key[0], key[1], key[2], b["P"], b["P"] / b["N"], b["Z"], 100 * b["rho"], 600 / b["N"], b["R"], b["tile_share"]))
b = board(16, 5, 17)
P, Z, F = b["P"], b["Z"], b["F"]
m_eq = Z + P + 1
r_bp = math.ceil((math.sqrt(8 * m_eq + 1) - 1) / 2)
print("16x16: P %d, Z %.4g, F %.4g" % (P, Z, F))
print("  dense, 8 P^2 bytes            : %.1f GiB = %.1f GB" % (8 * P * P / GIB, 8 * P * P / GB))
print("  equalities of the theta form  : m = Z + P + 1 = %.4g, least r with r(r+1)/2 >= m: %d" % (m_eq, r_bp))
print("  factor at that rank, 8 P r    : %.1f GiB = %.1f GB" % (8 * P * r_bp / GIB, 8 * P * r_bp / GB))
print("  multipliers for Y >= 0, 8 F   : %.1f GiB = %.1f GB" % (8 * F / GIB, 8 * F / GB))
print("  factor + multipliers          : %.1f GiB = %.1f GB" % ((8 * P * r_bp + 8 * F) / GIB, (8 * P * r_bp + 8 * F) / GB))
m_all = m_eq + F
r_all = math.ceil((math.sqrt(8 * m_all + 1) - 1) / 2)
print("  if every free pair counts as a constraint: m = %.4g, r = %d (P = %d), factor %.1f GiB"
      % (m_all, r_all, P, 8 * P * min(r_all, P) / GIB))
print("  SCS as coded, 72 F bytes      : %.2f TB" % (72 * F / 1e12))
print("  splitting, 5 dense matrices   : 40 (P+1)^2 = %.0f GiB = %.0f GB" % (40 * (P + 1) ** 2 / GIB, 40 * (P + 1) ** 2 / GB))
print("  10 P^3 flops                  : %.2g" % (10 * P ** 3))
print("  chordal block 4 (I^2/2 + I/2) : %d of %d" % (4 * (b["I"] ** 2 // 2 + b["I"] // 2), P))
print("  edge-colour order 480 x 17    : %d, ratio %.1f, cubed %.0f" % (480 * 17, P / (480 * 17), (P / (480 * 17)) ** 3))
print("  6x6 dense copy, 8 x 1297^2    : %.1f MB" % (8 * 1297 ** 2 / 1e6))

print()
print("== engineering memo, iteration growth from 4x4 (P 138) to 5x5 (P 478), one size step")
step = math.log(478 / 138)
pairs = [
    ("splitting, memo's pairing: <= 500 (first log line) to 3,500", 500, 3500),
    ("splitting, memo's pairing: <= 500 to 6,000", 500, 6000),
    ("SCS, memo's pairing: 175 to 3,025", 175, 3025),
    ("SCS, memo's pairing: 175 to 8,200 (unconverged)", 175, 8200),
    ("SCS, the memo's other 4x4 figure: 475 to 3,025", 475, 3025),
    ("SCS: 475 to 8,200", 475, 8200),
    ("splitting to residual 1e-7, l2: 1,387 (4x4 seed 3, rho 30) to 35,602 (5x5 seed 1, rho 300)", 1387, 35602),
    ("splitting, mass under 1e-5, 4x4 seed 0 at rho 1 (14,500) to 5x5 at best rho (3,500)", 14500, 3500),
    ("same, to 6,000", 14500, 6000),
    ("if the true 4x4 count were 100 (the log cannot exclude it): 100 to 6,000", 100, 6000),
]
for name, a, c in pairs:
    print("  exponent %5.2f   ratio %6.1f   %s" % (math.log(c / a) / step, c / a, name))

print()
print("== engineering memo, section 4: the scaling table from the Appendix F inputs")
eig = {479: 0.026, 1291: 0.40, 2917: 4.7}          # seconds per eigendecomposition, one core (memo, Appendix F)
TREE = {5: 16.0, 6: 21.7, 7: 25.9, 8: 32.8, 10: 60.4, 12: 96.5, 16: 134.4}   # log2 row-scan nodes (memo table)


def t_core(D):
    ks = sorted(eig)
    if D > ks[-1]:
        return eig[ks[-1]] * (D / ks[-1]) ** 3
    base = min(ks, key=lambda k: abs(k - D))
    return eig[base] * (D / base) ** 3


def fmt(s):
    for u, k in (("s", 1), ("min", 60), ("h", 3600), ("d", 86400)):
        if s < 100 * k:
            return "%.2g %s" % (s / k, u)
    return "%.2g y" % (s / YEAR)


rows = {}
for key in [(5, 3, 4), (6, 4, 5), (7, 4, 7), (8, 5, 8), (10, 5, 10), (12, 5, 12), (16, 5, 17)]:
    n = key[0]
    b = board(*key)
    D = b["P"] + 1 - (2 * n * n - 1)
    t1 = t_core(D)
    tg = max(0.02, 1.5 * 80.4 * (D / 50000) ** 3)
    rs14 = 2 ** TREE[n] / 1.6e7 / 14
    its = {a: 4500 * (b["P"] / 478) ** a for a in (1, 3)}
    rows[n] = dict(P=b["P"], D=D, t1=t1, tg=tg, rs14=rs14)
    print("%2dx%-2d D %6d  s/it %.3g  its %.3g / %.3g | 1 core %s / %s | 14 cores (1 core / 6) %s / %s | card %s / %s | row scan 14 cores %s | 5 matrices %.0f GiB"
          % (n, n, D, t1, its[1], its[3], fmt(its[1] * t1), fmt(its[3] * t1), fmt(its[1] * t1 / 6), fmt(its[3] * t1 / 6),
             fmt(its[1] * tg), fmt(its[3] * tg), fmt(rs14), 40 * (b["P"] + 1) ** 2 / GIB))

print()
print("== crossover: largest iteration exponent a for which ONE lift solve on 14 cores still beats row scan on 14 cores")
for n in (8, 10, 12):
    r = rows[n]
    # 4500 (P/478)^a t1/6 = rs14
    a = math.log(r["rs14"] * 6 / (4500 * r["t1"])) / math.log(r["P"] / 478)
    print("  %2dx%-2d: a < %.2f  (and a < %.2f if the row-scan tree is 2^5 larger, a < %.2f if 2^5 smaller)"
          % (n, n, a, a + 5 * math.log(2) / math.log(r["P"] / 478), a - 5 * math.log(2) / math.log(r["P"] / 478)))
print("  so along THIS colour ladder the crossing size moves little with a; it is set by the tree model (28 bits between 8x8 and 10x10)")
print("  8x8 with one colour fewer (5,7): tree 2^41.6 (memo): row scan on 14 cores %s, against the card's best case %s"
      % (fmt(2 ** 41.6 / 1.6e7 / 14), fmt(4500 * (rows[8]["P"] / 478) * rows[8]["tg"])))
