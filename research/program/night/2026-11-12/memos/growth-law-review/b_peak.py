"""Claim B, side check: where the first-solution cost of row-major DFS peaks in c on small boards,
and the balanced frame counts at ladder sizes.

python3 -I b_peak.py
"""
import math, os, random, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rvlib as rv

rng = random.Random(77)
print("valid frames, 5 frame colours: n, naive (memo F_n), balanced")
for n in (6, 7, 8, 9, 10, 16):
    print("  %d  %.3g  %.3g" % (n, math.exp(rv.lnF_naive(n)), math.exp(rv.lnF_balanced(n))))
print("first-solution nodes of row-major DFS by colour count (review generator, distinct pieces)")
print("  n c boards cap | solved | KM median | lnS naive | lnS balanced")
for n, cs, reps, cap in ((5, (3, 4, 5, 6, 7), 300, 10**8), (6, (3, 4, 5, 6), 24, 10**8)):
    for c in cs:
        boards = []
        while len(boards) < reps:
            p, _ = rv.planted_board(n, c, 5, rng)
            if p is None:
                break
            boards.append(("b%d" % len(boards), n, p))
        if not boards:
            print("  %d %d: no board with distinct pieces found" % (n, c))
            continue
        res = rv.run_enum("first", cap, boards)
        v = [math.inf if r["capped"] else r["nodes"] for r in res]
        med = rv.km_median(v)
        print("  %d %d %d %.0e | %d | %s | %.1f | %.1f" % (n, c, len(v), cap, sum(x < math.inf for x in v),
              "above cap" if med == math.inf else "%d" % med, rv.lnS_naive(n, c), rv.lnS_balanced(n, c)))
        sys.stdout.flush()
