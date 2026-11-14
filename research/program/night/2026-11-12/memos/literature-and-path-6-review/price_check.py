"""Is path 6's 10^47.72 the reviewed price under another convention?

Run: python3 -I price_check.py
Uses the price referee's library (../price-and-ledger-review/rv_lib.py): exact
probability of the joints closed so far, against the same probability spread
evenly over all joints, which is what solve-path-6/profile.py does.
"""
import os, sys, time
from math import log10
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "price-and-ledger-review"))
from rv_lib import load_official, census, Model, rows_from, logsum

t0 = time.time()
pieces, hints = load_official()
M = Model(16, census(pieces))
centre = [h for h in hints if 5 <= h[0] <= 10 and 5 <= h[1] <= 10]
print("clues   order       model          log10 tree  log10 sols  log10 nodes per solution")
for cname, hs in (("none", []), ("one", centre), ("five", hints)):
    for oname, top in (("row 0 first", True), ("row 15 first", False)):
        for mode in ("spread-pooled", "pooled"):
            pr = M.profile(rows_from(16, top), hs, mode)
            print("%-7s %-12s %-13s %9.2f %11.2f %12.2f" % (cname, oname, mode, logsum(pr), pr[-1], logsum(pr) - pr[-1]))
print("profile.py (path 6): tree 10^54.79, boards 10^7.06, nodes per solution 10^47.72  [no clue, row 0 first, spread]")
print("[%.0fs]" % (time.time() - t0))
