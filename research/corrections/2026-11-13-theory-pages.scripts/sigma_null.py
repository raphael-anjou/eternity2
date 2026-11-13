"""Largest cycle between record pairs against a random-permutation null.

Stdlib only:  python3 -I sigma_null.py

Corpus: research/topics/sigma-cycles/results/sigma-cycles.json (246 pairs).
Null: a uniform permutation that keeps corners on corners, edges on edges and
interior on interior (4, 56, 196), the only structure any legal pair of
boards must share. 20,000 draws, seeded.
"""
import json
import os
import random
import statistics

HERE = os.path.dirname(os.path.abspath(__file__))
J = os.path.join(HERE, "..", "..", "topics", "sigma-cycles", "results", "sigma-cycles.json")
d = json.load(open(J))
L = sorted(p["largestCycle"] for p in d["pairs"])
print("corpus: %d pairs, largest cycle min %d, median %s, max %d"
      % (len(L), L[0], statistics.median(L), L[-1]))
big = sorted(p["largestCycle"] for p in d["pairs"] if p["movedCells"] >= 240)
print("corpus pairs moving at least 240 cells: %d, median largest cycle %s" % (len(big), statistics.median(big)))

rng = random.Random(11)


def largest(n):
    p = list(range(n))
    rng.shuffle(p)
    seen = [False] * n
    best = 0
    for i in range(n):
        if not seen[i]:
            k, j = 0, i
            while not seen[j]:
                seen[j] = True
                j = p[j]
                k += 1
            best = max(best, k)
    return best


xs = sorted(max(largest(196), largest(56), largest(4)) for _ in range(20000))
print("null (20,000 draws): median %d, 5%% %d, 95%% %d, mean %.1f"
      % (xs[10000], xs[1000], xs[19000], statistics.mean(xs)))
