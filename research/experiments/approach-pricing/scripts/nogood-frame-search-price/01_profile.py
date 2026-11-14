"""Row-scan profile split into border arc x interior fills, and window counts.
Model: re-check's first-moment library (read only). python3 -I 01_profile.py"""
import sys
from math import log2, log10
LIB = "research/program/night/2026-11-12/notes/price-and-ledger-review"
sys.path.insert(0, LIB)
from rv_lib import *

L10 = log2(10)
pieces, hints = load_official()
print("clues (row, col):", sorted(hints))
cen = census(pieces)
m = Model(16, cen)
order = rows_from(16, top=False)
W = m.profile(order, hints, mode="class")
free = [c for c in order if c not in set(hints)]
assert len(free) == len(W) == 251
tot = logsum(W)
print("row scan from row 15, five clues, class-aware: total 10^%.3f = 2^%.2f ; peak 10^%.2f at free depth %d"
      % (tot, tot * L10, max(W), W.index(max(W)) + 1))
border = [c for c in free if kind(16, *c) > 0]
NF = m.profile(border, (), mode="class")
print("frames, model: 10^%.3f = 2^%.2f (exact committed 10^37.608 = 2^%.2f)" % (NF[-1], NF[-1] * L10, 37.608 * L10))
# suffix sums
suf = [0.0] * (len(W) + 1)
suf[len(W)] = float("-inf")
for i in range(len(W) - 1, -1, -1):
    suf[i] = logsum([W[i], suf[i + 1]]) if suf[i + 1] != float("-inf") else W[i]
print("\nend of each scanned row k (rows 15 .. 16-k): border arc B_k = bottom side + (k-1) cells up each side")
print(" k  free  arc cells  log2 N_k   log10 W   log10 mu_k  log2 T_>k  log2 kappa_k  share of T below")
nb = 0
d = 0
rows = {}
for idx, c in enumerate(free):
    if kind(16, *c) > 0:
        nb += 1
    last_of_row = (idx + 1 == len(free)) or (free[idx + 1][0] != c[0])
    if last_of_row:
        k = 16 - c[0]
        d = idx + 1
        n = NF[nb - 1]
        rest = suf[d] if d < len(W) else float("-inf")
        rows[k] = (d, nb, n, W[idx], rest)
        print("%2d  %4d  %3d  %8.2f  %8.2f  %8.2f  %8.2f  %8.2f  %.3g"
              % (k, d, nb, n * L10, W[idx], W[idx] - n, rest * L10, (rest - n) * L10, 10 ** (rest - tot) if rest > -1e9 else 0))
# crossing of mu=1 by free depth
print("\nfirst free depth with mu < 1 after the peak:")
nb = 0
prev = None
for idx, c in enumerate(free):
    if kind(16, *c) > 0:
        nb += 1
    mu = W[idx] - NF[nb - 1]
    if idx > 150 and mu < 0:
        print("  free depth %d, cell %s, arc cells %d, log2 N = %.2f, log10 W = %.2f, share of total spent before = %.6f"
              % (idx + 1, c, nb, NF[nb - 1] * L10, W[idx], 10 ** (logsum(W[:idx]) - tot)))
        break
# mass quantiles
acc = 0.0
tgt = [0.01, 0.1, 0.5, 0.9, 0.99]
ti = 0
nb = 0
print("\nwhere the row-scan nodes sit (cumulative share by free depth):")
for idx, c in enumerate(free):
    if kind(16, *c) > 0:
        nb += 1
    acc += 10 ** (W[idx] - tot)
    while ti < len(tgt) and acc >= tgt[ti]:
        print("  %.0f%% of nodes by free depth %d (row %d, arc cells %d, log2 N = %.1f, log10 mu = %.2f)"
              % (100 * tgt[ti], idx + 1, c[0], nb, NF[nb - 1] * L10, W[idx] - NF[nb - 1]))
        ti += 1
# windows of L consecutive ring cells
rg = ring(16)
print("\nvalid windows of L consecutive border cells (model), log2 count; straight = inside one side, corner = centred on a corner")
print("  L  straight  corner-centred  per-cell bits (straight)")
for L in (2, 4, 5, 6, 8, 10, 12, 14, 16, 20, 24, 30, 36, 44, 52, 60):
    out = []
    for start in (1, (60 - L // 2) % 60):
        cells = [rg[(start + t) % 60] for t in range(L)]
        if start == 1 and L > 14:
            out.append(float("nan"))
            continue
        out.append(m.profile(cells, (), mode="class")[-1] * L10)
    print(" %2d  %8.2f  %8.2f  %6.2f" % (L, out[0], out[1], out[0] / L if out[0] == out[0] else out[1] / L))
# colour census
seam = cen["seam"]
S = sum(seam.values())
H = -sum(v / S * log2(v / S) for v in seam.values())
print("\nseam (inward) colours of the 56 edge pieces: %d colours, counts %s" % (len(seam), sorted(seam.values())))
print("entropy per inward letter %.3f bits ; log2(56) = %.3f ; log2 of distinct words 56!/prod m_c! = %.1f"
      % (H, log2(56), (ilog10(factorial(56)) - sum(ilog10(factorial(v)) for v in seam.values())) * L10))
fr = cen["frame"]
print("frame (lateral) colours: %s" % sorted(fr.values()))
