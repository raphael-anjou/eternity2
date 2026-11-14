"""Bulge-model price of the official 16x16 under several fill orders."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from bulge import *

n, pieces, hints = load_official()
bt, mt = histograms(pieces)
print("bt", bt, "mt", mt)
print("hints (row,col,piece,rot):", hints)
centre = [(h[0], h[1]) for h in hints if (h[0], h[1]) in ((8, 7), (7, 8), (7, 7), (8, 8))]
allh = [(h[0], h[1]) for h in hints]
print("centre", centre, "all", allh)


def report(name, model, order, hs):
    w = model.profile(order, hs)
    tot = sum(w)
    peak = max(w)
    d = w.index(peak) + 1 + len(hs)
    print(f"{name:34s} sols {float(w[-1]):11.4g}  total 10^{L(tot):6.2f}  peak 10^{L(peak):6.2f} at depth {d}  per-sol 10^{L(tot / w[-1]) if w[-1] > 0 else float('nan'):6.2f}")
    return w, tot


m1 = Model(16, 16, bt, mt, 1)
m5 = Model(16, 16, bt, mt, 5)
w, t = report("1 clue, row scan from P1", m1, rowscan(16, 16), centre)
# McGavin's published checkpoints (README of topics/complex-theory)
for dpt, ref in ((2, 4.0), (3, 44.8), (5, 5341), (8, 6.089e6), (16, 3.689e14), (157, 5.045e45), (256, 14702)):
    print(f"   depth {dpt}: {float(w[dpt - 2]):.4g} (published {ref:g})")
res = {}
for nm_, mdl, hs in (("1 clue", m1, centre), ("5 clues", m5, allh)):
    for oname, o in (("row from P (bottom)", rowscan(16, 16, True)), ("row from A (top)", rowscan(16, 16, False)),
                     ("col from left", colscan(16, 16, True)), ("col from right", colscan(16, 16, False))):
        res[(nm_, oname)] = report(f"{nm_}, {oname}", mdl, o, hs)
    best = None
    for start in [None, (15, 0), (0, 0), (15, 15), (0, 15)]:
        o = mdl.greedy(hs, start)
        ww, tt = report(f"{nm_}, greedy start {start}", mdl, o, hs)
        if best is None or tt < best[1]:
            best = (o, tt, ww)
    res[(nm_, "greedy")] = best
    o, tt, ww = best
    print("   best greedy order, first 40 cells:", o[:40])
    # per-depth profile, every 16
    print("   log10 width every 16 placed:", [round(L(x), 1) for x in ww[15::16]])
# no-clue ensemble for reference
m0 = Model(16, 16, bt, mt, 0)
report("0 clue, row scan", m0, rowscan(16, 16), [])
