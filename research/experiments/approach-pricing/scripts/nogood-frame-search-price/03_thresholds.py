"""Break-even budgets per bottom arc and the gain ceiling per row.
python3 -I 03_thresholds.py"""
import sys
from math import log2, log10
LIB = "research/program/night/2026-11-12/notes/price-and-ledger-review"
sys.path.insert(0, LIB)
from rv_lib import *
L10 = log2(10)
pieces, hints = load_official()
m = Model(16, census(pieces))
order = rows_from(16, top=False)
free = [c for c in order if c not in set(hints)]
border = [c for c in free if kind(16, *c) > 0]
NF = m.profile(border, (), mode="class")
T_MEAS = 40.457 * L10          # Track 10, measured
PB = log2(2669 / 57131 * 2508 / 57131)   # both bottom clue blocks alive (chains)
print("measured price 2^%.2f ; both bottom blocks alive 2^%.2f" % (T_MEAS, PB))
for mode in ("class", "pooled"):
    W = m.profile(order, hints, mode=mode)
    tot = logsum(W)
    print("\nmode %s: model total 2^%.2f (10^%.3f)" % (mode, tot * L10, tot))
    print(" k  arc  beta_k  log2 T<=k  ceiling of gain (bits)  break-even log2 C per block-surviving arc  = core-seconds at 1e8 nodes/s  | log2 fills per surviving arc")
    nb = 0
    for idx, c in enumerate(free):
        if kind(16, *c) > 0:
            nb += 1
        if idx + 1 == len(free) or free[idx + 1][0] != c[0]:
            k = 16 - c[0]
            beta = NF[nb - 1] * L10 if k < 16 else 37.608 * L10
            up = logsum(W[: idx + 1]) * L10
            be = T_MEAS - (beta + (PB if k >= 3 else 0))
            print("%2d  %3d  %7.2f  %8.2f  %6.2f  %7.2f  %10.3g  | %6.2f"
                  % (k, nb, beta, up, tot * L10 - up, be, 2 ** be / 1e8, (W[idx] - NF[nb - 1]) * L10 - (PB if k >= 3 else 0)))
