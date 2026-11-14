"""Width profile of the row scan (official 16x16), one and five clues, and budget depths."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from bulge import *
n, pieces, hints = load_official()
bt, mt = histograms(pieces)
allh = [(h[0], h[1]) for h in hints]
for name, mdl, hs in (("1 clue", Model(16, 16, bt, mt, 1), [(8, 7)]), ("5 clues", Model(16, 16, bt, mt, 5), allh)):
    w = mdl.profile(rowscan(16, 16), hs)
    lw = [L(x) for x in w]
    tot = sum(w)
    print(name, "total 10^%.2f" % L(tot), "final", float(w[-1]))
    print("  free cells placed -> log10 width:", {d: round(lw[d - 1], 1) for d in (16, 32, 48, 64, 80, 96, 112, 128, 144, 160, 176, 192, 208, 224, 240, len(w))})
    for B in (18, 20, 21, 23, 26, 28):
        d = next(i for i, x in enumerate(lw) if x > B) + 1
        print(f"  width first exceeds 10^{B} after {d} free cells; peak/that = 10^{max(lw) - B:.1f}")
    c = 0
    lo = hi = None
    for i, x in enumerate(w):
        c += x
        if lo is None and c > tot / 100: lo = i + 1
        if hi is None and c > tot * 99 / 100: hi = i + 1
    print(f"  98% of the nodes lie between free-cell depths {lo} and {hi}")
