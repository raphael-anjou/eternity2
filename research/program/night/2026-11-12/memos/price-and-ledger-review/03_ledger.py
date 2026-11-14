"""Claim C. The conditional ledger of path 3 and the price of frame-first search.
Part 1: official 16x16, model only. Part 2: exact enumeration on small planted
boards of every valid frame and of the interior tree under each.
Run: python3 -I 03_ledger.py"""
import os
import random
import statistics
import sys
import time
from math import log10, comb

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rv_lib import *

t0 = time.time()
pieces, hints = load_official()
cen = census(pieces)
M = Model(16, cen)
N = 16
centre = [h for h in hints if 5 <= h[0] <= 10 and 5 <= h[1] <= 10]
td, bu = rows_from(N, True), rows_from(N, False)
fr = ring(N)
int_td = [(r, c) for r in range(1, N - 1) for c in range(1, N - 1)]
int_bu = [(r, c) for r in range(N - 2, 0, -1) for c in range(1, N - 1)]

print("PART 1. Official set, pooled exact model.")
print("1a. path 3 closes the joints of a clue only when the scan reaches the clue cell (model.py profile()).")
print("    A solver has the clue on the board from the start. Both conventions, five clues:")
for name, o in (("r=0 first (path 3 'top-down')", td), ("r=15 first (path 3 'bottom-up')", bu)):
    a = M.profile(o, hints, defer=True)
    b = M.profile(o, hints, count_hint_levels=True)
    print("    %-32s deferred: total 10^%.2f peak 10^%.2f at %d | clue on board: total 10^%.2f peak 10^%.2f at %d"
          % (name, logsum(a), max(a), a.index(max(a)) + 1, logsum(b), max(b), b.index(max(b)) + 1))
a = M.profile(td, hints, defer=True)
b = M.profile(td, hints, count_hint_levels=True)
print("    log10 N after each full row, r=0 first, deferred :", " ".join("%.1f" % a[16 * r + 15] for r in range(N)))
print("    log10 N after each full row, r=0 first, on board :", " ".join("%.1f" % b[16 * r + 15] for r in range(N)))

print("\n1b. frame-first search: all 60 frame cells, then the interior. Per-frame figures are means over all valid frames.")
for cname, hs in (("five clues", hints), ("centre clue", centre)):
    for iname, io in (("interior r=1 first", int_td), ("interior r=14 first", int_bu)):
        for conv in ("deferred", "on board"):
            pr = M.profile(fr + io, hs, defer=(conv == "deferred"))
            nf = pr[59]
            rest = pr[60:]
            print("    %-11s %-19s %-8s log10 frames %.2f | interior per frame: peak 10^%.2f, sum 10^%.2f | frame-first total 10^%.2f | completions per frame 10^%.2f"
                  % (cname, iname, conv, nf, max(rest) - nf, logsum(rest) - nf, logsum(pr), pr[-1] - nf))
exact_frames = log10(40519056449429419697492635191544381440)
best = M.profile(fr + int_bu, hints)
per_frame = logsum(best[60:]) - best[59]
print("    committed exact frame count 10^%.3f; model 10^%.3f (ratio %.2f)" % (exact_frames, best[59], 10 ** (exact_frames - best[59])))
print("    frames x mean refutation cost, exact frame count: 10^%.2f ; row scan: 10^%.2f ; frame-first is 10^%.1f times dearer"
      % (exact_frames + per_frame, logsum(M.profile(bu, hints)), exact_frames + per_frame - logsum(M.profile(bu, hints))))
print("    break-even: frame-first matches row scan only if the mean cost per frame is 10^%.2f nodes" % (logsum(M.profile(bu, hints)) - exact_frames))

print("\n1c. where the row-scan width sits: partial frames times interior fills per partial frame (five clues, r=15 first)")
pr = M.profile(bu, hints)
free = [c for c in bu if c not in set(hints)]
for k in (16, 64, 112, 160, 208, 251):
    cells = free[:k]
    fcells = [c for c in cells if kind(N, *c)]
    # valid partial frames on those frame cells alone
    pf = M.profile(fcells, ())[-1]
    print("    after %3d free cells: width 10^%.2f = partial frames (%d cells) 10^%.2f x interior fills per partial frame 10^%.2f"
          % (k, pr[k - 1], len(fcells), pf, pr[k - 1] - pf))

print("\n1d. top/bottom split given (five clues): top 8 rows, pool = the pieces of the true top half")
top = [(r, c) for r in range(8) for c in range(N)]
th = [h for h in hints if h[0] < 8]
n_int_top = 7 * 14 - len(th)
full = M.profile(top, hints)
half = M.profile(top, hints, pools={2: 2, 1: 28, 0: n_int_top})
print("    free pool: peak 10^%.2f end 10^%.2f | exact pool: peak 10^%.2f, sum 10^%.2f, end 10^%.2f | splits: 10^%.2f"
      % (max(full), full[-1], max(half), logsum(half), half[-1], log10(comb(191, n_int_top)) + log10(comb(56, 28)) + log10(comb(4, 2))))

print("\n1e. seam join with the clue on the board (five clues). tops: rows 0..r-1 scanned r=0 first; bottoms: rows r..15 scanned r=15 first")
T = M.profile(td, hints, count_hint_levels=True)
B = M.profile(bu, hints, count_hint_levels=True)
dfs = logsum(M.profile(bu, hints))
for r in range(5, 11):
    ct, cb = logsum(T[:16 * r]), logsum(B[:16 * (N - r)])
    print("    seam r=%2d: tops 10^%.1f bottoms 10^%.1f | generation 10^%.2f | row scan 10^%.2f | gain %.2f digits"
          % (r, T[16 * r - 1], B[16 * (N - r) - 1], logsum([ct, cb]), dfs, dfs - logsum([ct, cb])))

print("\n1f. piece-set statistic behind the '36% of perfect frames cannot start the interior' report")
adj = set()
for p in pieces:
    if 0 not in p:
        for i in range(4):
            adj.add((p[i], p[(i + 1) % 4]))
w = cen["seam"]
tot = sum(w.values())
dead = sum(w[a] * w[b] for a in w for b in w if (a, b) not in adj) / tot ** 2
print("    ordered colour pairs carried by two clockwise-adjacent sides of some interior piece: %d of 289" % len(adj))
print("    P(an inner-corner cell has no candidate | independent inward colours) = %.4f ; P(at least one of four) = %.3f ; Poisson with mean 784/289: %.4f"
      % (dead, 1 - (1 - dead) ** 4, 2.718281828 ** (-784 / 289)))
print("[%.0fs]" % (time.time() - t0))

print("\nPART 2. Small planted boards, exact. Every valid frame is enumerated and the interior tree under each is measured.")
for (n, cf, ci, seeds) in ((6, 4, 6, (1, 2, 3)), (6, 4, 5, (1, 2))):
    td = rows_from(n, True)
    frn = ring(n)
    inn = [(r, c) for r in range(1, n - 1) for c in range(1, n - 1)]
    nf = len(frn)
    for seed in seeds:
        rng = random.Random(1000 * n + 100 * cf + 10 * ci + seed)
        ps, sol = gen_planted(n, cf, ci, rng)
        Mm = Model(n, census(ps))
        mrow = Mm.profile(td)
        mff = Mm.profile(frn + inn)
        en = Enum(n, ps)
        try:
            ff = en.run(frn + inn, watch=nf, cap=40_000_000)
        except OverflowError:
            print("  %dx%d (%d/%d) seed %d: frame-first above cap, skipped" % (n, n, cf, ci, seed))
            continue
        sub = ff["sub"]
        F = ff["per"][nf]
        tot_ff = sum(ff["per"])
        true = Enum(n, ps).run(inn, preset={c: sol[c] for c in frn})
        t_true = sum(true["per"])
        try:
            row = sum(Enum(n, ps).run(td, cap=8_000_000)["per"])
            rowtxt = "%d" % row
        except OverflowError:
            row = None
            rowtxt = "above 8e6 (model 10^%.2f)" % logsum(mrow)
        sub.sort()
        zero = sum(1 for x in sub if x == 0)
        print("  %dx%d (%d/%d) seed %d: frames %d (model %.3g) | interior tree per frame: mean %.1f (model %.1f), median %d, 90th pct %d, max %d, zero-node frames %.0f%% | true frame %d (solutions under it %d)"
              % (n, n, cf, ci, seed, F, 10 ** mff[nf - 1], statistics.mean(sub), 10 ** (logsum(mff[nf:]) - mff[nf - 1]), sub[len(sub) // 2], sub[int(0.9 * len(sub))], sub[-1], 100 * zero / len(sub), t_true, true["per"][-1]))
        top1 = sum(sub[-max(1, len(sub) // 100):]) / max(1, sum(sub))
        print("        frame-first total %d nodes (frame part %d, interior part %d; model 10^%.2f) | row scan %s (model 10^%.2f) | share of interior nodes under the heaviest 1%% of frames %.0f%%   [%.0fs]"
              % (tot_ff, sum(ff["per"][:nf + 1]), sum(sub), logsum(mff), rowtxt, logsum(mrow), 100 * top1, time.time() - t0))
