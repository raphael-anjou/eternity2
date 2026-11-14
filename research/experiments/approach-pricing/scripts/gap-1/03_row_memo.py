"""Gap 1, real pieces: what does a learner that keeps its reasons while the search
stays under one row-start state gain? (The reuse a persistent solver gets between
neighbours of a depth-first search, in its most favourable form.)

For row-start states sampled by SMC (same sampler and gate as 02), enumerate the whole
subtree inside the row (16 cells) three ways:

  plain   every valid partial row
  memo    the same search with a table (cell, left colour) -> reasons. A reason is the
          set of tiles of the CURRENT row whose being used was needed for the death of
          the subtree inside the row (the frontier below and the used tiles of lower
          rows are the same for the whole row, so they cost nothing). A node is cut,
          at the price of one node, when a stored reason for its (cell, left colour)
          is contained in the tiles now placed in the row. Sound.
  oracle  only the partial rows that extend to a complete row (free, perfect).

usage: python3 -I 03_row_memo.py NP NS SEED
"""
import random
import sys
import time
from math import log2, log10

sys.path.insert(0, __import__("os").path.dirname(__file__))
import e2real as E

NP, NS, SEED = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3])
cands, context, CELLS, NFREE = E.cands, E.context, E.CELLS, E.NFREE
ROWSTART = [p for p in range(NFREE) if CELLS[p][1] == 0 and 105 <= p <= 180]
CAP = 300_000


def row_end(p0):
    r = CELLS[p0][0]
    q = p0
    while q < NFREE and CELLS[q][0] == r:
        q += 1
    return q


def run_row(p0, used0, front, lastright0):
    end = row_end(p0)
    stats = {"plain": 0, "memo": 0, "oracle": 0, "rows": 0, "cuts": 0, "over": False}
    memo = {}

    def rec(pos, used, lastright, rowmask, counting_memo):
        """returns (alive, plain nodes under and including this node, R)"""
        if pos == end:
            return True, 0, 0
        if stats["plain"] > CAP:
            stats["over"] = True
            return True, 0, 0
        left, down = context(pos, front, lastright)
        cut = False
        if counting_memo:
            for rl in memo.get((pos, left), ()):
                if rl & rowmask == rl:
                    cut = True
                    stats["cuts"] += 1
                    break
        alive, R, kids_alive = False, 0, 0
        for t, up, right, bit in cands(pos, left, down):
            if used & bit:
                R |= bit
                continue
            stats["plain"] += 1
            cm = counting_memo and not cut
            if cm:
                stats["memo"] += 1
            a, _, cR = rec(pos + 1, used | bit, right, rowmask | bit, cm)
            if a:
                alive = True
                kids_alive += 1
            R |= cR & ~bit
        stats["oracle"] += kids_alive
        if not alive:
            if counting_memo and not cut:
                memo.setdefault((pos, left), []).append(R & rowmask)
        return alive, 0, R

    rec(p0, used0, lastright0, 0, True)
    return stats


def main():
    rng = random.Random(SEED)
    meas = E.measured_widths()
    parts = [(0, (0,) * 16, 0)] * NP
    logZ = 0.0
    print("# NP %d NS %d seed %d" % (NP, NS, SEED))
    print("row-start depth (board row) | gate diff dex | row-starts | plain nodes per row-start | gain bits: memo, oracle | share of memo lookups that cut | truncated")
    for pos in range(max(ROWSTART) + 1):
        if pos in ROWSTART:
            idx = list(range(NP))
            rng.shuffle(idx)
            seen, tot = set(), {"plain": 0, "memo": 0, "oracle": 0, "rows": 0, "cuts": 0}
            over = 0
            for i in idx:
                st = parts[i]
                if st in seen:
                    continue
                seen.add(st)
                s = run_row(pos, st[0], list(st[1]), st[2])
                over += s["over"]
                for k in tot:
                    tot[k] += s[k]
                if len(seen) >= NS:
                    break
            n = len(seen)
            # complete rows = oracle nodes at the last level; recount cheaply: oracle counts alive children at every level
            print("%3d (row %2d) | %+.3f | %d | %.1f | %.2f, %.2f | %.3f | %d" % (
                pos, CELLS[pos][0], logZ - meas[pos], n, tot["plain"] / n,
                log2(tot["plain"] / max(tot["memo"], 1)), log2(tot["plain"] / max(tot["oracle"], 1)),
                tot["cuts"] / max(tot["memo"], 1), over), flush=True)
        new, ws = [], []
        col = CELLS[pos][1]
        for used, front, lastright in parts:
            left, down = context(pos, front, lastright)
            cs = [x for x in cands(pos, left, down) if not used & x[3]]
            if cs:
                t, up, right, bit = cs[rng.randrange(len(cs))]
                new.append((used | bit, front[:col] + (up,) + front[col + 1:], right))
                ws.append(len(cs))
        tot_w = sum(ws)
        if not tot_w:
            break
        logZ += log10(tot_w / len(parts))
        stepw = tot_w / NP
        u = rng.random() * stepw
        res, acc = [], 0.0
        for p, w in zip(new, ws):
            acc += w
            while u < acc and len(res) < NP:
                res.append(p)
                u += stepw
        while len(res) < NP:
            res.append(new[-1])
        parts = res


main()
