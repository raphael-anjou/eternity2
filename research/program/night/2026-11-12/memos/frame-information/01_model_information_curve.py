"""Frame information, part 1: the first-moment model on the official set.

Question: under a fixed full frame, how many valid partial interiors exist on
average after d interior cells (mu_d), for several interior orders? A depth-d
test can only tell frames apart once mu_d is below 1, so the curve mu_d against
the nodes spent to reach depth d is the information-against-cost curve of any
refuter that works by filling interior cells.

Uses the referee's model (read only): ../price-and-ledger-review/rv_lib.py
Run: python3 -I 01_model_information_curve.py
"""
import os
import sys
import time
from math import log10, log2

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "price-and-ledger-review"))
from rv_lib import *

t0 = time.time()
pieces, hints = load_official()
cen = census(pieces)
M = Model(16, cen)
N = 16
fr = ring(N)
hs = set(hints)
EXACT_FRAMES = log10(40519056449429419697492635191544381440)
ROW = logsum(M.profile(rows_from(N, False), hints))
print("row scan, five clues, r=15 first: 10^%.2f nodes ; exact frame count 10^%.3f ; break-even mean per frame 10^%.2f"
      % (ROW, EXACT_FRAMES, ROW - EXACT_FRAMES))
print("hints (row, col):", sorted(hints))


def per_frame(order, mode="pooled", hints_=hints):
    """log10 mu_d for d = 1..len(order): mean valid partial interiors per valid frame."""
    pr = M.profile(fr + order, hints_, mode=mode)
    return [x - pr[59] for x in pr[60:]]


def spiral():
    o, lo, hi = [], 1, N - 2
    while lo <= hi:
        rc = [(lo, c) for c in range(lo, hi + 1)] + [(r, hi) for r in range(lo + 1, hi + 1)]
        rc += [(hi, c) for c in range(hi - 1, lo - 1, -1)] + [(r, lo) for r in range(hi - 1, lo, -1)]
        o += rc
        lo += 1
        hi -= 1
    seen, out = set(), []
    for x in o:
        if x not in seen and x not in hs:
            seen.add(x)
            out.append(x)
    return out


SP = spiral()
# the three free cells of each corner block that holds a clue at its far cell
blocks = []
for (hr, hc) in sorted(hints):
    if hr in (2, 13) and hc in (2, 13):
        r1 = 1 if hr == 2 else 14
        c1 = 1 if hc == 2 else 14
        blocks.append([(r1, hc), (hr, c1), (r1, c1)])
flat = [x for b in blocks for x in b]
CF = flat + [x for x in SP if x not in set(flat)]

print("\nA. mu_d (mean valid partial interiors per frame), five clues, pooled census")
for name, o in (("spiral inward", SP), ("four clue corner blocks first, then spiral", CF)):
    w = per_frame(o)
    cum = [logsum(w[:i + 1]) for i in range(len(w))]
    pk = max(range(len(w)), key=lambda i: w[i])
    below = next((i for i in range(pk, len(w)) if w[i] < 0), None)
    first_below = next((i for i in range(len(w)) if w[i] < 0), None)
    print("  %s: sum 10^%.2f, peak 10^%.2f at interior cell %d; mu falls below 1 for good at cell %d of %d (first time below 1: cell %s)"
          % (name, cum[-1], w[pk], pk + 1, below + 1, len(w), first_below + 1 if first_below is not None else "never"))
    print("    x exact frame count: 10^%.2f, against row scan %+.2f digits" % (cum[-1] + EXACT_FRAMES, cum[-1] + EXACT_FRAMES - ROW))
    print("    d : log10 mu_d : log10 nodes spent per frame up to d : bits a depth-d test can give = max(0, -log2 mu_d)")
    for d in (1, 2, 3, 6, 9, 12, 20, 30, 40, 52, 60, 80, 96, 100, 110, 120, 130, 140, 150, 160, 170, 180, 191):
        print("    %3d : %7.2f : %6.2f : %6.1f" % (d, w[d - 1], cum[d - 1], max(0.0, -w[d - 1] * log2(10))))

print("\nB. the clue corner blocks alone (3 free cells, 8 joints each)")
w = per_frame(CF)
for b in range(4):
    print("  after block %d: log10 mu = %.3f (survivor fraction if at most one fill each: %.4f, bits %.2f)"
          % (b + 1, w[3 * b + 2], 10 ** w[3 * b + 2], -w[3 * b + 2] * log2(10)))
wc = per_frame(CF, mode="class")
print("  class-aware census (seam joints their own class), after four blocks: log10 mu = %.3f, bits %.2f" % (wc[11], -wc[11] * log2(10)))

print("\nC. how small can a region be and still refute in the mean? mu of corner-anchored k x k interior blocks and of full rings")
for k in range(1, 15):
    cells = [(r, c) for s in range(2, 2 * k + 1) for r in range(1, k + 1) for c in range(1, k + 1) if r + c == s and (r, c) not in hs]
    w1 = per_frame(cells)
    w0 = per_frame(cells, hints_=[h for h in hints if 5 <= h[0] <= 10])
    print("  k=%2d (%3d cells): five clues log10 mu = %7.2f | centre clue only %7.2f" % (k, k * k, w1[-1], w0[-1]))
w = per_frame(SP)
done = 0
for ringi, cnt in enumerate((52, 44 - 4, 36, 28, 20, 12, 4 - 1)):
    done += cnt
    print("  after spiral ring %d (%3d free interior cells): log10 mu = %7.2f" % (ringi + 1, done, w[done - 1]))

print("\nD. the other side of the join: interiors without a frame (five clues on the board)")
inn_rows = [(r, c) for r in range(14, 0, -1) for c in range(1, 15)]
pi = M.profile(inn_rows, hints)
print("  valid 14x14 interiors, frame ignored: 10^%.2f ; row-scan tree to list them: sum 10^%.2f, peak 10^%.2f" % (pi[-1], logsum(pi), max(pi)))
pi2 = M.profile(SP, hints)
print("  same by spiral: sum 10^%.2f, peak 10^%.2f" % (logsum(pi2), max(pi2)))
print("  frames 10^%.2f ; so the frame list is the %s list" % (EXACT_FRAMES, "larger" if EXACT_FRAMES > pi[-1] else "smaller"))

print("\nE. oracle sizes: bits about the frame needed for frame-first search to cost a given budget")
best = logsum(per_frame(SP))
for name, budget in (("row scan (10^%.2f)" % ROW, ROW), ("1,000 core-years (10^18.5)", 18.5), ("1e5 core-years (10^20.5)", 20.5)):
    need = EXACT_FRAMES + best - budget
    print("  %-28s rank of the true frame must be within 10^%.1f of 10^%.1f: %.1f bits" % (name, budget - best, EXACT_FRAMES, need * log2(10)))
print("[%.1fs]" % (time.time() - t0))
