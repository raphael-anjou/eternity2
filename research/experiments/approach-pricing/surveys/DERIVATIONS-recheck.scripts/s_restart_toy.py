"""Item 7: is the row-scan tree 'balanced' in the sense of Chen, Gomes, Selman? Toy planted boards with one solution (first corner pinned).
Exact subtree size of every sibling along the planted path; distribution of the time to the solution under random value
order; Luby's l(t) = E[min(T,t)]/P(T<=t) for every cutoff. Balanced tree: T uniform, E[T] = Ttot/2, best restart gains 0,
short cutoffs lose 1 bit. Run: python3 -I s_restart_toy.py"""
import random, sys, functools
from math import log2
from collections import defaultdict
print = functools.partial(print, flush=True)
sys.setrecursionlimit(10000)

def gen(n, cb, ci, rng):
    while True:
        H = [[0] * (n + 1) for _ in range(n)]; V = [[0] * n for _ in range(n + 1)]
        for r in range(n):
            for c in range(1, n):
                H[r][c] = rng.randrange(1, cb + 1) if r in (0, n - 1) else rng.randrange(cb + 1, cb + ci + 1)
        for r in range(1, n):
            for c in range(n):
                V[r][c] = rng.randrange(1, cb + 1) if c in (0, n - 1) else rng.randrange(cb + 1, cb + ci + 1)
        pcs = [(V[r][c], H[r][c + 1], V[r + 1][c], H[r][c]) for r in range(n) for c in range(n)]
        canon = set(); ok = True
        for p in pcs:
            rots = [p[i:] + p[:i] for i in range(4)]
            if len(set(rots)) < 4 or min(rots) in canon: ok = False; break
            canon.add(min(rots))
        if ok: return pcs      # piece i sits at cell i, rotation 0

def analyse(n, pcs):
    t = defaultdict(list)
    for i, p in enumerate(pcs):
        for k in range(4):
            N, E, S, W = p[k:] + p[:k]
            t[(N, W, E == 0, S == 0)].append((i, k, E, S))
    south = [0] * n; nsol = [0]
    def size(pos, used, east):
        if pos == n * n: nsol[0] += 1; return 1
        r, c = divmod(pos, n); tot = 1
        for (i, k, E, S) in t.get((south[c], 0 if c == 0 else east, c == n - 1, r == n - 1), ()):
            if used >> i & 1: continue
            old = south[c]; south[c] = S
            tot += size(pos + 1, used | 1 << i, E)
            south[c] = old
        return tot
    sib = []; used = 0; east = 0
    for pos in range(n * n):
        r, c = divmod(pos, n); here = []
        for (i, k, E, S) in t.get((south[c], 0 if c == 0 else east, c == n - 1, r == n - 1), ()):
            if used >> i & 1 or (i == pos and k == 0) or pos == 0: continue   # the first corner is pinned (a clue): removes the board rotations
            old = south[c]; south[c] = S
            here.append(size(pos + 1, used | 1 << i, E))
            south[c] = old
        sib.append(here)
        p = pcs[pos]; south[c] = p[2]; east = p[1]; used |= 1 << pos
    return sib, nsol[0]

def main():
    rng = random.Random(11)
    print("board | tree nodes | E[T]/tree (balanced 0.5) | best cutoff: gain in bits, cutoff/tree, P(T<=cutoff) | gain at cutoff = tree/100, tree/10 (balanced: -1.0, -0.9) | sum of squared CV of sibling subtree sizes on the planted path")
    res = defaultdict(list)
    for (n, cb, ci, want) in ((5, 4, 5, 30), (6, 4, 6, 12), (6, 5, 7, 12)):
        done = 0
        while done < want:
            pcs = gen(n, cb, ci, rng)
            sib, nsol = analyse(n, pcs)
            if nsol: continue                      # another solution in a sibling subtree: skip, keep single-leaf boards
            tot = n * n + sum(sum(h) for h in sib)
            r2 = random.Random(done); T = []
            for _ in range(6000):
                x = 0
                for d, h in enumerate(sib):
                    if h:
                        k = r2.randrange(len(h) + 1)       # planted child's position among len(h)+1 children
                        if k: x += sum(r2.sample(h, k))
                T.append(x + n * n)
            T.sort(); ET = sum(T) / len(T)
            best = None; pre = 0.0; at = {}
            for j, tt in enumerate(T):
                pre += tt
                l = (pre + (len(T) - j - 1) * tt) / (j + 1)   # E[min(T,t)]/q(t)
                if best is None or l < best[0]: best = (l, tt, (j + 1) / len(T))
                for frac in (0.01, 0.1):
                    if frac not in at and tt >= frac * tot: at[frac] = l
            cv = 0.0
            for h in sib:
                if len(h) >= 2:
                    m = sum(h) / len(h); cv += sum((y - m) ** 2 for y in h) / len(h) / m / m
            g = log2(ET / best[0]); res[(n, cb, ci)].append((g, ET / tot, log2(ET / at.get(0.01, ET)), log2(ET / at.get(0.1, ET))))
            print("%dx%d c%d+%d #%02d | %9d | %.3f | %5.2f  %.4f  %.4f | %5.2f %5.2f | %.1f" % (n, n, cb, ci, done, tot, ET / tot, g, best[1] / tot, best[2], log2(ET / at.get(0.01, ET)), log2(ET / at.get(0.1, ET)), cv))
            done += 1
    print()
    for k, v in res.items():
        gs = sorted(x[0] for x in v)
        print("%s: %d boards | gain of the best restart cutoff, bits: median %.2f mean %.2f max %.2f ; share of boards with gain > 1 bit %.2f ; mean E[T]/tree %.3f ; mean gain at cutoff tree/100: %.2f, tree/10: %.2f"
              % (k, len(v), gs[len(gs) // 2], sum(gs) / len(gs), gs[-1], sum(1 for g in gs if g > 1) / len(gs), sum(x[1] for x in v) / len(v), sum(x[2] for x in v) / len(v), sum(x[3] for x in v) / len(v)))
main()
