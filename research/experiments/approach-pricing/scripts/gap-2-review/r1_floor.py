"""Review item 3: what are 2^126.6 (O1), 2^128.7 (memo) and 2^131.9 (survey) the price of?
For every cut of the bottom-up row scan: list sizes, widest level and NODE SUM of each side's generation,
with three generation orders per side: row scan from its rim, column scan inside the band, greedy (next cell = smallest next level)."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from r_common import *
A = levels(ROWS_BU); Bq = levels(ROWS_BU[::-1])
TOT = lsum(A)
print("model tree, rows bottom-up, five clues: node sum 2^%.2f, widest level 2^%.2f at depth %d ; sum/peak = %.2f bits" % (TOT, max(A), A.index(max(A)) + 1, TOT - max(A)))
print("the same tree scanned from the other rim: node sum 2^%.2f, widest level 2^%.2f at depth %d" % (lsum(Bq), max(Bq), Bq.index(max(Bq)) + 1))
print("\n== A. row-scan generation of both sides, per cut d (bits) ==")
print("   d | list top, bottom | widest top, bottom | node sum top, bottom | max list | max widest | node sum both")
best = {}
for d in range(1, 256):
    a, b = A[d - 1], Bq[255 - d]; wa, wb = max(A[:d]), max(Bq[:256 - d]); sa, sb = lsum(A[:d]), lsum(Bq[:256 - d])
    rec = (max(a, b), max(wa, wb), lsum([sa, sb]))
    for i, k in enumerate(("list", "widest", "sum")):
        if k not in best or rec[i] < best[k][0]: best[k] = (rec[i], d)
    if d % 16 == 0 or d in (137,):
        print("%4d | %6.1f %6.1f | %6.1f %6.1f | %6.1f %6.1f | %6.1f | %6.1f | %6.1f" % (d, a, b, wa, wb, sa, sb, *rec))
print("min over all 255 cuts: max list 2^%.2f (d=%d) ; max widest level 2^%.2f (d=%d) ; node sum 2^%.2f (d=%d)" % (*best["list"], *best["widest"], *best["sum"]))
rowcuts = [(max(A[d - 1], Bq[255 - d]), d) for d in range(80, 192, 16)]
print("row cuts 5..11, max list: " + " ".join("d=%d:%.2f(10^%.2f)" % (d, v, v / B) for v, d in rowcuts))
print("gain of the row-scan two-list join against the tree, like with like: node sum %.2f bits ; widest level %.2f bits" % (TOT - best["sum"][0], max(A) - best["widest"][0]))

print("\n== B. other generation orders inside each side (appendix D item 2 of the memo) ==")
def colscan(cells):
    return sorted(cells, key=lambda x: (x[1], x[0]))
def greedy(cells, start_rim_row):
    """Greedy order: start in the rim corner, always add the cell (adjacent to the placed set) that gives the smallest next level."""
    cells = set(cells); placed = []; ps = set()
    left = dict(POOLS); ei = ef = 0; la = 0.0
    def kind(r, c): return ("interior", "edge", "corner")[(r in (0, N - 1)) + (c in (0, N - 1))]
    def delta(r, c):
        k = kind(r, c); g = 0.0
        if (r, c) not in clue: g = log2(left[k]) + (2 if k == "interior" else 0)
        di = df = 0
        for (rr, cc) in ((r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)):
            if (rr, cc) in ps:
                if (r == rr and r in (0, N - 1)) or (c == cc and c in (0, N - 1)): df += 1
                else: di += 1
        return g + LPI[ei + di] - LPI[ei] + LPF[ef + df] - LPF[ef], di, df
    cur = (start_rim_row, 0)
    while True:
        dl, di, df = delta(*cur); k = kind(*cur)
        if cur not in clue: left[k] -= 1
        ei += di; ef += df; ps.add(cur); placed.append(cur)
        if len(ps) == len(cells): break
        cand = [x for x in cells - ps if any(y in ps for y in ((x[0] - 1, x[1]), (x[0] + 1, x[1]), (x[0], x[1] - 1), (x[0], x[1] + 1)))]
        cur = min(cand, key=lambda x: (delta(*x)[0], x))
    return placed
print("   d | max list | row scan: widest, sum | column scan in band: widest, sum | greedy: widest, sum   (each = max / sum over the two sides)")
bestB = (1e9, None)
for d in list(range(64, 208, 16)) + [137, 120, 121, 136]:
    top = ROWS_BU[:d]; bot = ROWS_BU[d:]
    out = []
    for name, ot, ob in (("row", top, bot[::-1]), ("col", colscan(top), colscan(bot)), ("greedy", greedy(top, N - 1), greedy(bot, 0))):
        la, lb = levels(ot), levels(ob)
        assert abs(la[-1] - A[d - 1]) < 1e-6 and abs(lb[-1] - Bq[255 - d]) < 1e-6
        out += [max(max(la), max(lb)), lsum(la + lb)]
        if lsum(la + lb) < bestB[0]: bestB = (lsum(la + lb), (d, name))
    print("%4d | %6.1f | %6.1f %6.1f | %6.1f %6.1f | %6.1f %6.1f" % (d, max(A[d - 1], Bq[255 - d]), *out))
print("cheapest two-list generation found: node sum 2^%.2f at %s ; gain against the tree %.2f bits" % (bestB[0], bestB[1], TOT - bestB[0]))
print("\n== C. left/right cut (columns), for the 4-list geometry ==")
COLS = [(r, c) for c in range(N) for r in range(N)]
Lc = levels(COLS); Rc = levels(COLS[::-1])
for d in (112, 128, 144):
    print(" column cut d=%d: left list 2^%.1f right list 2^%.1f ; widest %.1f %.1f ; node sum both 2^%.1f" % (d, Lc[d - 1], Rc[255 - d], max(Lc[:d]), max(Rc[:256 - d]), lsum(Lc[:d] + Rc[:256 - d])))
