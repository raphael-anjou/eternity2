"""Review of gap 6b, script 4: lemma L4.
(a) Exhaustive check of the inequality on the 4x4 and 5x5 grids, and of how many
    minimisers are not bands.
(b) What the lemma is used for: is a band the region of k cells with the fewest
    strict partial boards N(S)? N(S) depends on the corner/edge/interior make-up
    and on which closed edges are frame-frame, not only on the number of open edges.
    Simulated annealing over k-subsets of the 16x16 (five clues, real-count ledger),
    compared with the row-scan prefix of the same size.
Usage: python3 -I r4_isoperimetry.py [seed]
"""
import sys, os, random
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rlib import *

def exhaustive(n):
    hm = 0; vm = 0
    for r in range(n):
        for c in range(n):
            i = r * n + c
            if c < n - 1: hm |= 1 << i
            if r < n - 1: vm |= 1 << i
    lo, hi = -(-n * n // 4), 3 * n * n // 4
    worst = {}
    for s in range(1 << (n * n)):
        k = s.bit_count()
        if k < lo or k > hi: continue
        b = ((s ^ (s >> 1)) & hm).bit_count() + ((s ^ (s >> n)) & vm).bit_count()
        w = worst.get(k)
        if w is None or b < w[0]: worst[k] = [b, 1]
        elif b == w[0]: w[1] += 1
    return lo, hi, worst

for n in (4, 5):
    lo, hi, worst = exhaustive(n)
    print(f"n={n}: sizes {lo}..{hi}; min open edges by size " + " ".join(f"{k}:{worst[k][0]}(x{worst[k][1]})" for k in sorted(worst)) + f"; bound n={n} holds: {all(v[0] >= n for v in worst.values())}")

PI, PF, PIu, PFu = tables()
cells = [(r, c) for r in range(16) for c in range(16)]
nbr = {x: [(x[0] + dr, x[1] + dc) for dr, dc in ((1, 0), (-1, 0), (0, 1), (0, -1)) if 0 <= x[0] + dr < 16 and 0 <= x[1] + dc < 16] for x in cells}
def is_ff(a, b): return kind(*a) > 0 and kind(*b) > 0

def value(S, PI=PI, PF=PF, clues=CLUES):
    nc = sum(1 for x in S if kind(*x) == 2); ne = sum(1 for x in S if kind(*x) == 1)
    ni = sum(1 for x in S if kind(*x) == 0 and x not in clues)
    eff = ein = 0
    for x in S:
        for y in nbr[x]:
            if y in S and x < y:
                if is_ff(x, y): eff += 1
                else: ein += 1
    navail = 196 - len(clues)
    return lfall(4, nc) + lfall(56, ne) + lfall(navail, ni) + 2 * ni + PI[ein] + PF[eff], (nc, ne, ni, eff, ein)

def openedges(S): return sum(1 for x in S for y in nbr[x] if y not in S)

def anneal(k, rng, start, steps=120000, T0=3.0):
    S = set(start); out = [x for x in cells if x not in S]; inn = list(S)
    cur, _ = value(S); best = cur; bestS = set(S)
    for t in range(steps):
        T = T0 * (1 - t / steps) + 0.02
        i = rng.randrange(len(inn)); j = rng.randrange(len(out))
        a, b = inn[i], out[j]
        S.remove(a); S.add(b)
        v, _ = value_fast(S)
        if v <= cur or rng.random() < 2 ** ((cur - v) / T):
            cur = v; inn[i], out[j] = b, a
            if v < best: best = v; bestS = set(S)
        else:
            S.remove(b); S.add(a)
    return best, bestS

# faster value: recomputed fully each time is too slow for many steps, so keep it simple but lean
def value_fast(S):
    nc = ne = ni = eff = ein = 0
    for x in S:
        k = KIND[x]
        if k == 2: nc += 1
        elif k == 1: ne += 1
        elif x not in CLUES: ni += 1
        for y, ff in NB2[x]:
            if y in S:
                if ff: eff += 1
                else: ein += 1
    return LF4[nc] + LF56[ne] + LF191[ni] + 2 * ni + PI[ein] + PF[eff], None
KIND = {x: kind(*x) for x in cells}
NB2 = {x: [(y, is_ff(x, y)) for y in nbr[x] if x < y] for x in cells}
LF4 = [lfall(4, i) for i in range(5)]; LF56 = [lfall(56, i) for i in range(57)]; LF191 = [lfall(191, i) for i in range(192)]

if __name__ == "__main__":
    seed = int(sys.argv[1]) if len(sys.argv) > 1 else 1
    rng = random.Random(seed)
    rowscan = cells
    prof = [value(set(rowscan[:d]))[0] for d in range(1, 257)]
    pk = max(prof); print(f"\nrow scan, five clues, real counts: peak level 2^{pk:.2f} at depth {prof.index(pk)+1}")
    print("k   row-scan prefix (open edges)   best set found by annealing (open edges; corners, edge cells, interior non-clue cells; frame-frame, other closed edges)")
    res = {}
    for k in (64, 96, 112, 128, 144, 152, 161, 168, 176, 192):
        pre = set(rowscan[:k]); v0 = prof[k - 1]
        best = None
        starts = [pre, set(rng.sample(cells, k)), set(sorted(cells, key=lambda x: (min(x[0], 15 - x[0], x[1], 15 - x[1]), x))[:k])]
        for st in starts:
            b, bs = anneal(k, rng, st, steps=40000)
            if best is None or b < best[0]: best = (b, bs)
        b, bs = best; _, comp = value(bs)
        res[k] = (v0, b, bs)
        print(f"{k:3d}  {v0:7.2f} ({openedges(pre):2d})     {b:7.2f} ({openedges(bs):2d}; {comp[0]}, {comp[1]}, {comp[2]}; {comp[3]}, {comp[4]})   gain {v0-b:+.2f} bits")
    print(f"\nlower bound on the peak of ANY static cell order from these sets: max over k of the best value found = 2^{max(v[1] for v in res.values()):.2f} (an estimate from annealing, not a certificate), against the row-scan peak 2^{pk:.2f}")
    for k in (128, 161):
        print(f"\nbest set found at k={k}:")
        bs = res[k][2]
        for r in range(16): print("  " + "".join("#" if (r, c) in bs else ("c" if (r, c) in CLUES else ".") for c in range(16)))
