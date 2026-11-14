"""First-moment model of the number of complete frames of the official set
(no clue: the frame holds none) against the exact count of Track 10's gate (b),
40519056449429419697492635191544381440. Also the 5x5 (left, right) table of
the edge pieces and the count of same-colour pieces, for the record.
python3 -I 08_frame_count_model.py
"""
import math, sys, collections
sys.path.insert(0, "research/experiments/orders-and-pins/scripts")
from model import Model
import orders as O
P = O.load_official()
M = Model(16, P)
ring = [(15, c) for c in range(16)] + [(r, 15) for r in range(14, -1, -1)] + [(0, c) for c in range(14, -1, -1)] + [(r, 0) for r in range(1, 15)]
prof = M.profile(ring, ())
exact = 40519056449429419697492635191544381440
print("model log10 frames %.4f   exact log10 %.4f   exact - model %+.4f dex" % (prof[-1], math.log10(exact), math.log10(exact) - prof[-1]))
tab = collections.Counter(); same = 0
for p in P:
    g = [i for i in range(4) if p[i] == 0]
    if len(g) == 1:
        k = g[0]; a, b = p[(k + 1) % 4], p[(k + 3) % 4]
        tab[(a, b)] += 1; same += a == b
cols = sorted(set(a for a, _ in tab) | set(b for _, b in tab))
print("edge pieces by (side after grey clockwise, side before): rows = first, cols = second")
for a in cols: print("  ", a, [tab[(a, b)] for b in cols], "row sum", sum(tab[(a, b)] for b in cols))
print("   col sums", [sum(tab[(a, b)] for a in cols) for b in cols], " same-colour pieces", same)
print("sum over colours of rowsum*colsum (ordered compatible edge-edge pairs, self-pairs included): ", sum(sum(tab[(c, b)] for b in cols) * sum(tab[(a, c)] for a in cols) for c in cols), " balanced value 5*11.2^2 = %.1f" % (5 * 11.2 ** 2))
