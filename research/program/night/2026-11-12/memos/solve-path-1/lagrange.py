"""Reconstruction of the 'Lagrangian-patched contraction' on tiny boards. Stdlib only: python3 -I lagrange.py
Max-sum (tropical) contraction of the colour-only network with one price per piece. Each round returns the
edge-matched board of largest total price, then prices move against over-used pieces (subgradient step).
Stops when the returned board uses every piece once. Compared with blind sampling, whose expected number of
draws is Z/N (colour-only boards per true solution, from toy.py)."""
import random, time, functools, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))   # -I drops the script directory
from toy import gen, table, sweep
print = functools.partial(print, flush=True)

def best_board(n, t, lam):
    cur = {((0,) * n, 0): (0.0, None)}
    back = []
    for r in range(n):
        for c in range(n):
            nxt = {}
            for (fr, east), (val, _) in cur.items():
                west = 0 if c == 0 else east
                for (i, E, S) in t.get((fr[c], west, c == n - 1, r == n - 1), ()):
                    key = (fr[:c] + (S,) + fr[c + 1:], E)
                    v = val + lam[i]
                    o = nxt.get(key)
                    if o is None or v > o[0]: nxt[key] = (v, ((fr, east), i))
            back.append(nxt); cur = nxt
    key = max(cur, key=lambda k: cur[k][0])
    val = cur[key][0]; used = []
    for lvl in reversed(back):
        prev, i = lvl[key][1]; used.append(i); key = prev
    return val, used, max(len(l) for l in back)

def solve(n, pcs, cap, rng, deadline):
    t = table(n, pcs); m = n * n
    lam = [rng.random() * 1e-3 for _ in range(m)]
    best = 0
    for it in range(1, cap + 1):
        if time.time() > deadline: return None, it, best, 0
        val, used, states = best_board(n, t, lam)
        cnt = [0] * m
        for i in used: cnt[i] += 1
        distinct = sum(1 for x in cnt if x)
        best = max(best, distinct)
        if distinct == m: return it, it, best, states
        step = 1.0 / (1 + it / 100.0)
        for i in range(m): lam[i] -= step * (cnt[i] - 1) + rng.random() * 1e-4
    return None, cap, best, states

T0 = time.time()
print(" n cb ci | solved/seeds | rounds to a true solution (solved seeds) | best distinct pieces on unsolved | blind sampling Z/N | states | s/round")
for (n, cb, ci, seeds, cap, budget) in ((3, 2, 2, 8, 2000, 5), (4, 2, 3, 8, 3000, 25), (5, 3, 4, 6, 1500, 55), (6, 3, 6, 3, 300, 70)):
    deadline = time.time() + budget
    its, fails, zs, st, rounds, t1 = [], [], [], 0, 0, time.time()
    for s in range(1, seeds + 1):
        pcs = gen(n, cb, ci, random.Random(100 + s))
        if time.time() > deadline: break
        r, done, best, states = solve(n, pcs, cap, random.Random(s), deadline)
        rounds += done; st = max(st, states)
        if r is None: fails.append(best)
        else: its.append(r)
        if n <= 5:
            N = sweep(n, pcs, "exact")[0][(1 << n * n) - 1]; Z = sweep(n, pcs, "free")[0][0]; zs.append(Z / N)
    dt = time.time() - t1
    its.sort(); zs.sort()
    print(" %d  %d  %d | %d/%d | %s | %s | median %s | %d | %.3f"
          % (n, cb, ci, len(its), len(its) + len(fails), its, fails, ("%.3g" % zs[len(zs) // 2]) if zs else "about 1e11 (toy.py, one seed)", st, dt / max(rounds, 1)))
print("total %.0fs" % (time.time() - T0))
