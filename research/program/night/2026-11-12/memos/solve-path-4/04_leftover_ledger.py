"""Is the piece set left for the mismatch band of a record board unusual in its colour ledger?

For each public record board the mismatches sit in rows 0..5 (03_record_boards.out). Take that
band as the "tail": its 96 pieces L and the seam demand beta (the colours row 6 shows upward).
Statistic F1 = sum over colours of log10(number of perfect pairings of T_c half-edges),
T_c = n_c(L) + beta_c: the first-order (configuration-model) log-count of ways to pair the tail's
half-edges colour by colour. Larger F1 = a more concentrated ledger = more pairings.
Null: 4000 random tails with the same number of corner, border and interior pieces, same beta.
Also the number of colours with odd T_c (each pair of odd colours forces one mismatch).

Control added after the first run: that null is the wrong one. Any region that is tiled with
few mismatches has even T_c and an overdispersed ledger, because half-edges enter in same-colour
pairs. So the same statistic is computed for the mirror band of each board (rows 10..15, which
holds no mismatch, seam demand from row 9), a perfectly tiled region that no search treated as
its tail. If the mismatch band and the perfect band score alike, the z-score above measures
"being tiled", not "having good leftovers".
Stdlib only: python3 -I 04_leftover_ledger.py
"""
import math, os, random, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib4 import *
from collections import Counter


def lg_pairings(T):
    T -= T % 2
    return lg_fact(T) - (T // 2) * math.log10(2) - lg_fact(T // 2)


def stat(tail_pieces, beta):
    T = Counter(beta)
    for t in tail_pieces:
        for col in t:
            if col:
                T[col] += 1
    return sum(lg_pairings(v) for v in T.values()), sum(1 for v in T.values() if v % 2)


rng = random.Random(4)
boards = load_boards()
for name, g in boards.items():
    rows = range(0, 6)
    tail = [g[r][c] for r in rows for c in range(N)]
    rest = [g[r][c] for r in range(6, N) for c in range(N)]
    beta = [g[6][c][0] for c in range(N)]
    f, odd = stat(tail, beta)
    allp = tail + rest
    by = {0: [], 1: [], 2: []}
    for t in allp:
        by[t.count(0)].append(t)
    need = Counter(t.count(0) for t in tail)
    fs, odds = [], []
    for _ in range(4000):
        samp = []
        for k in (0, 1, 2):
            samp += rng.sample(by[k], need[k])
        a, b = stat(samp, beta)
        fs.append(a); odds.append(b)
    m = sum(fs) / len(fs)
    sd = math.sqrt(sum((x - m) ** 2 for x in fs) / len(fs))
    mo = sum(odds) / len(odds)
    print("%-26s tail F1 = %.2f | null mean %.2f sd %.2f decades | z = %+.2f | odd colours %d (null mean %.1f, null share with <= this many: %.3f)" % (
        name, f, m, sd, (f - m) / sd, odd, mo, sum(1 for x in odds if x <= odd) / len(odds)))


print()
print("control: mismatch band (rows 0..5) against the mismatch-free mirror band (rows 10..15), same statistic")
for name, g in boards.items():
    out = []
    for rows, seam_row, side in ((range(0, 6), 6, 0), (range(10, N), 9, 2)):
        tail = [g[r][c] for r in rows for c in range(N)]
        beta = [g[seam_row][c][side] for c in range(N)]
        out.append(stat(tail, beta))
    print("%-26s mismatch band F1 = %.2f (odd %d) | perfect mirror band F1 = %.2f (odd %d)" % (name, out[0][0], out[0][1], out[1][0], out[1][1]))
