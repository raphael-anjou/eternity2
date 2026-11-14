"""Block lemma, numbers. Run: python3 -I analyze_profile.py profile_PLAIN.txt profile_FC.txt profile_AC.txt
Input: per-level node counts of ROW on the official board turned by a half turn (lines "P rep level decided log2_n").
Lemma (see the review): a rule that completes each block of k rows before it touches the next block, in any order inside
the block and with any dependence on the node, has exactly row scan's node count at every block end, under PLAIN, FC or AC.
So its tree is at least the sum of row scan's counts at block ends, and its gain over row scan is at most
log2(sum over all levels / sum over block-end levels). Printed for k = 1, 2, 4, 8 and every alignment of the blocks."""
import sys
from math import log2
from collections import defaultdict
clue = {255 - c for c in (8 * 16 + 7, 13 * 16 + 2, 2 * 16 + 2, 13 * 16 + 13, 2 * 16 + 13)}
order = [c for c in range(256) if c not in clue]
rowend = {}   # level (1-based) -> row index completed
for lvl, c in enumerate(order, 1):
    if all(o // 16 != c // 16 or o <= c for o in order): rowend[lvl] = c // 16
for f in sys.argv[1:]:
    prof = defaultdict(dict)
    for l in open(f):
        p = l.split()
        if p and p[0] == "P": prof[int(p[1])][int(p[2])] = float(p[4])
    for rep, d in sorted(prof.items()):
        tot = sum(2.0 ** (v - 100) for v in d.values()); pk = max(d, key=d.get)
        out = []
        for k in (1, 2, 4, 8):
            bs = []
            for off in range(k):
                s = sum(2.0 ** (d[l] - 100) for l, r in rowend.items() if l in d and (r + 1 - off) % k == 0)
                bs.append(log2(tot / s) if s > 0 else float("inf"))
            out.append("k=%d: %.2f to %.2f" % (k, min(bs), max(bs)))
        print("%s rep %d: log2 total %.2f, widest level %d (2^%.2f), levels seen %d | gain bound in bits, by block alignment: %s" % (f, rep, log2(tot) + 100, pk, d[pk], len(d), " ; ".join(out)))
