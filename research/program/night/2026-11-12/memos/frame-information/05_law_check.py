"""Frame information, part 5: when the information arrives, and the size-bias law.

Size-bias law tested. If the planted frame is, to an observer who only counts
depth-d partial interiors, a frame drawn in proportion to W_d, then the mean of
W_d(planted) / mu_d is E[W_d^2] / E[W_d]^2 over frames. Weighting frames by W_d
is then worth about log2 of that ratio ("size bias" column of script 03).

Reads 03_small_boards_bits_per_depth.json, then prints the official set's
model curve (script 01) at the depths that matter.
Run: python3 -I 05_law_check.py
"""
import json
import os
import sys
from math import log2, log10

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "price-and-ledger-review"))
from rv_lib import *

res = json.load(open(os.path.join(HERE, "03_small_boards_bits_per_depth.json")))
print("small boards: W_d(planted)/mu_d against E[W^2]/E[W]^2, depths with mu_d >= 0.05 (ratios, then the mean over boards)")
acc = {}
for o in res:
    line = []
    for r in o["rows"]:
        if r["mu"] >= 0.05:
            ratio, pred = 2 ** r["post"], 2 ** r["sizebias"]
            acc.setdefault(r["d"], []).append((ratio, pred, 2 ** r["sure"]))
            line.append("%d: %.1f/%.1f" % (r["d"], ratio, pred))
    print("  %dx%d (%d/%d) s%d  d: planted/size-bias  %s" % (o["n"], o["n"], o["cf"], o["ci"], o["seed"], "  ".join(line)))
allp = [x for v in acc.values() for x in v]
n = len(allp)
mr = sum(a for a, _, _ in allp) / n
mp = sum(b for _, b, _ in allp) / n
ms = sum(c for _, _, c in allp) / n
print("  %d (board, depth) pairs: mean planted ratio %.2f (%.2f bits), mean size-bias prediction %.2f (%.2f bits), mean 1/(surviving fraction) %.2f (%.2f bits)"
      % (n, mr, log2(mr), mp, log2(mp), ms, log2(ms)))
print("  so of the %.2f bits the planted frame shows, %.2f are the sound filter and %.2f are ranking among survivors" % (log2(mr), log2(ms), log2(mr) - log2(ms)))

print("\nwhen does the information arrive? bits held when a share of the mean per-frame cost has been spent")
print("  board | total bits | at 50%% of cost: sure, soft, post | at 90%% of cost: sure, soft, post | cost share spent when sure bits reach half of the total")
for o in res:
    rows = o["rows"]
    tot = rows[-1]["cum"]
    total_bits = rows[-1]["sure"]
    def at(share):
        b = (0.0, 0.0, 0.0)
        for r in rows:
            if r["cum"] <= share * tot:
                b = (r["sure"], r["soft"], r["post"])
        return b
    half = next((r["cum"] / tot for r in rows if r["sure"] >= total_bits / 2), 1.0)
    a, b = at(0.5), at(0.9)
    print("  %dx%d (%d/%d) s%d | %.1f | %.2f %.2f %.2f | %.2f %.2f %.2f | %.3f"
          % (o["n"], o["n"], o["cf"], o["ci"], o["seed"], total_bits, a[0], a[1], a[2], b[0], b[1], b[2], half))

print("\nofficial set, model curve, five clues, clue corner blocks first then inward spiral")
pieces, hints = load_official()
M = Model(16, census(pieces))
N = 16
fr = ring(N)
hs = set(hints)
o, lo, hi = [], 1, N - 2
while lo <= hi:
    rc = [(lo, c) for c in range(lo, hi + 1)] + [(r, hi) for r in range(lo + 1, hi + 1)]
    rc += [(hi, c) for c in range(hi - 1, lo - 1, -1)] + [(r, lo) for r in range(hi - 1, lo, -1)]
    o += rc
    lo += 1
    hi -= 1
SP = [x for x in dict.fromkeys(o) if x not in hs]
flat = []
for (hr, hc) in sorted(hints):
    if hr in (2, 13) and hc in (2, 13):
        r1 = 1 if hr == 2 else 14
        c1 = 1 if hc == 2 else 14
        flat += [(r1, hc), (hr, c1), (r1, c1)]
CF = flat + [x for x in SP if x not in set(flat)]
pr = M.profile(fr + CF, hints)
w = [x - pr[59] for x in pr[60:]]
cum = []
low = 0.0
for d in range(len(w)):
    cum.append(logsum(w[:d + 1]))
    low = min(low, w[d])
    if d + 1 in (3, 12, 13, 20, 40, 60, 91, 120, 140, 145, 150, 160, 191):
        print("  d=%3d  log10 mu %7.2f  nodes per frame so far 10^%.2f  first-moment bound on sure bits, -log2 of the lowest mu so far: %6.1f"
              % (d + 1, w[d], cum[-1], -low * log2(10)))
print("\nreuse-allowed relaxation at 16x16 (arithmetic): fills per frame if a piece may be used many times")
print("  191 cells x log10(764 oriented pieces) - 420 joints x log10(17.34) = %.1f digits: the relaxation refutes no frame in the mean"
      % (191 * log10(764) - 420 * log10(17.34)))
