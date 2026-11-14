"""Gap 3, exact check on TOY boards (5x5 and 6x6, one solution, first corner pinned).

The generator and the tree are those of the derivations re-check's s_restart_toy.py (same
seed, same boards). For every depth d the script lists the subtree size S(x) of every node
of the level (nodes strictly below x) and computes, exactly:

  Q_d   = share of the level's work that sits in subtrees no larger than the planted one,
          sum(S; S <= S*) / sum(S)
  gU_d  = log2( mean(S) / (2 mean(min(S, S*))) ): gain in bits of "draw a uniform node of
          level d at no cost, search below it, restart when exhausted, give up at S*"
          over plain depth-first search (which pays half the tree)
  kraft = log2 N_d - log2 W*_d, W* the product of the numbers of children along the planted
          path: how much likelier than 1/N_d a uniform-child probe is to land on the planted
          prefix
and the exact best fixed cutoff of depth-first search with random value order (the
re-check's quantity), for comparison with max over d of the bounds.

Run: python3 -I g3_toy_rank.py
"""
import functools
import random
import sys
from collections import defaultdict
from math import log2

print = functools.partial(print, flush=True)
sys.setrecursionlimit(10000)


def gen(n, cb, ci, rng):
    while True:
        H = [[0] * (n + 1) for _ in range(n)]
        V = [[0] * n for _ in range(n + 1)]
        for r in range(n):
            for c in range(1, n):
                H[r][c] = rng.randrange(1, cb + 1) if r in (0, n - 1) else rng.randrange(cb + 1, cb + ci + 1)
        for r in range(1, n):
            for c in range(n):
                V[r][c] = rng.randrange(1, cb + 1) if c in (0, n - 1) else rng.randrange(cb + 1, cb + ci + 1)
        pcs = [(V[r][c], H[r][c + 1], V[r + 1][c], H[r][c]) for r in range(n) for c in range(n)]
        canon = set()
        ok = True
        for p in pcs:
            rots = [p[i:] + p[:i] for i in range(4)]
            if len(set(rots)) < 4 or min(rots) in canon:
                ok = False
                break
            canon.add(min(rots))
        if ok:
            return pcs


def analyse(n, pcs):
    t = defaultdict(list)
    for i, p in enumerate(pcs):
        for k in range(4):
            N, E, S, W = p[k:] + p[:k]
            t[(N, W, E == 0, S == 0)].append((i, k, E, S))
    south = [0] * n
    nsol = [0]
    lev = [[] for _ in range(n * n + 1)]       # lev[d] = subtree sizes (nodes below) of level-d nodes

    def size(pos, used, east):                 # nodes strictly below the node with pos cells placed
        if pos == n * n:
            nsol[0] += 1
            lev[pos].append(0)
            return 0
        r, c = divmod(pos, n)
        tot = 0
        for (i, k, E, S) in t.get((south[c], 0 if c == 0 else east, c == n - 1, r == n - 1), ()):
            if used >> i & 1 or (pos == 0 and not (i == 0 and k == 0)):
                continue                       # first corner pinned
            old = south[c]
            south[c] = S
            tot += 1 + size(pos + 1, used | 1 << i, E)
            south[c] = old
        lev[pos].append(tot)
        return tot

    total = size(0, 0, 0)
    # planted path: children counts and sibling subtree sizes
    south = [0] * n
    used = 0
    east = 0
    sib, kids = [], []
    lev2 = lev
    lev = [[] for _ in range(n * n + 1)]       # scratch for the second pass
    for pos in range(n * n):
        r, c = divmod(pos, n)
        here = []
        cnt = 0
        for (i, k, E, S) in t.get((south[c], 0 if c == 0 else east, c == n - 1, r == n - 1), ()):
            if used >> i & 1 or (pos == 0 and not (i == 0 and k == 0)):
                continue
            cnt += 1
            if i == pos and k == 0:
                continue
            old = south[c]
            south[c] = S
            here.append(1 + size(pos + 1, used | 1 << i, E))
            south[c] = old
        sib.append(here)
        kids.append(cnt)
        p = pcs[pos]
        south[c] = p[2]
        east = p[1]
        used |= 1 << pos
    return total, nsol[0], lev2, sib, kids


def main():
    rng = random.Random(11)
    print("board | tree | best cutoff gain (random value order, exact) | max_d gU_d at d | min_d Q_d at d | max_d kraft at d | Q at mid depth")
    agg = defaultdict(list)
    for (n, cb, ci, want) in ((5, 4, 5, 30), (6, 4, 6, 12), (6, 5, 7, 12)):
        done = 0
        while done < want:
            pcs = gen(n, cb, ci, rng)
            total, nsol, lev, sib, kids = analyse(n, pcs)
            if nsol != 1:
                continue
            cells = n * n
            # planted subtree size below depth d
            sstar = [0] * (cells + 1)
            for d in range(cells - 1, -1, -1):
                sstar[d] = sstar[d + 1] + 1 + sum(sib[d])
            assert sstar[0] == total, (sstar[0], total)
            best_g, best_q, best_k = (-99, 0), (2.0, 0), (-99, 0)
            l2w = 0.0
            qmid = None
            for d in range(1, cells):
                l2w += log2(kids[d - 1])
                S = lev[d]
                tot = sum(S)
                if tot == 0:
                    continue
                s = sstar[d]
                q = sum(x for x in S if x <= s) / tot
                g = log2(tot / (2 * sum(min(x, s) for x in S)))
                kr = log2(len(S)) - l2w
                if g > best_g[0]: best_g = (g, d)
                if q < best_q[0]: best_q = (q, d)
                if kr > best_k[0]: best_k = (kr, d)
                if d == cells // 2: qmid = q
            # exact-in-distribution best cutoff of randomised DFS (re-check's computation)
            r2 = random.Random(done)
            T = []
            for _ in range(6000):
                x = 0
                for h in sib:
                    if h:
                        k = r2.randrange(len(h) + 1)
                        if k:
                            x += sum(r2.sample(h, k))
                T.append(x + cells)
            T.sort()
            ET = sum(T) / len(T)
            best = None
            pre = 0.0
            for j, tt in enumerate(T):
                pre += tt
                l = (pre + (len(T) - j - 1) * tt) / (j + 1)
                if best is None or l < best:
                    best = l
            cut = log2(ET / best)
            agg[(n, cb, ci)].append((cut, best_g[0], best_q[0], best_k[0], qmid))
            print("%dx%d c%d+%d #%02d | %8d | %5.2f | %5.2f @%2d | %.3f @%2d | %5.2f @%2d | %.3f"
                  % (n, n, cb, ci, done, total, cut, best_g[0], best_g[1], best_q[0], best_q[1], best_k[0], best_k[1], qmid))
            done += 1
    print()
    for k, v in agg.items():
        def med(i):
            xs = sorted(x[i] for x in v)
            return xs[len(xs) // 2], xs[0], xs[-1]
        print("%s: %d boards | best cutoff gain median %.2f min %.2f max %.2f | max_d gU median %.2f min %.2f max %.2f | min_d Q median %.3f min %.3f max %.3f | max_d kraft median %.2f min %.2f max %.2f | Q mid depth median %.3f min %.3f max %.3f"
              % ((k, len(v)) + med(0) + med(1) + med(2) + med(3) + med(4)))


main()
