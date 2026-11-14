"""Tiny-board checks for the counting/algebra memo. Stdlib only: python3 -I toy.py
One frontier dynamic program, three modes:
  free    : colour-only count, pieces reusable (the plain tensor contraction)
  exact   : state carries the used set, reuse pruned (the exact transfer matrix)
  parity  : state carries the usage parity of every piece (the +-1 inclusion-exclusion in Walsh form)
  track(Q): pieces in Q must be used exactly once, the others are reusable (the tightening ladder)
"""
import random, sys, time, functools
print = functools.partial(print, flush=True)
from math import lgamma, log, log10
from collections import defaultdict

def gen(n, cb, ci, rng):
    """Random planted n x n puzzle: cb frame colours, ci interior colours, all pieces distinct under rotation."""
    while True:
        H = [[0] * (n + 1) for _ in range(n)]      # H[r][c] = colour on the west side of cell (r,c)
        V = [[0] * n for _ in range(n + 1)]        # V[r][c] = colour on the north side of cell (r,c)
        for r in range(n):
            for c in range(1, n):
                H[r][c] = rng.randrange(1, cb + 1) if r in (0, n - 1) else rng.randrange(cb + 1, cb + ci + 1)
        for r in range(1, n):
            for c in range(n):
                V[r][c] = rng.randrange(1, cb + 1) if c in (0, n - 1) else rng.randrange(cb + 1, cb + ci + 1)
        pcs = [(V[r][c], H[r][c + 1], V[r + 1][c], H[r][c]) for r in range(n) for c in range(n)]  # N,E,S,W
        canon = set()
        ok = True
        for p in pcs:
            rots = [p[i:] + p[:i] for i in range(4)]
            if len(set(rots)) < 4 or min(rots) in canon: ok = False; break
            canon.add(min(rots))
        if ok:
            rng.shuffle(pcs)
            return pcs

def table(n, pcs):
    """(north, west, east_is_rim, south_is_rim) -> list of (piece, east, south)."""
    t = defaultdict(list)
    for i, p in enumerate(pcs):
        for k in range(4):
            N, E, S, W = p[k:] + p[:k]
            t[(N, W, E == 0, S == 0)].append((i, E, S))
    return t

def sweep(n, pcs, mode, Q=()):
    """Row-major frontier DP. Returns (final dict mask->count, per-depth (sum of counts, number of states))."""
    t = table(n, pcs)
    qbit = {p: 1 << j for j, p in enumerate(Q)}
    cur = {((0,) * n, 0, 0): 1}                    # (south colours of the row above/current, east colour, mask)
    prof = []
    for r in range(n):
        for c in range(n):
            nxt = defaultdict(int)
            for (fr, east, mask), cnt in cur.items():
                west = 0 if c == 0 else east
                for (i, E, S) in t.get((fr[c], west, c == n - 1, r == n - 1), ()):
                    if mode == "free": m = 0
                    elif mode == "exact":
                        if mask >> i & 1: continue
                        m = mask | 1 << i
                    elif mode == "parity": m = mask ^ 1 << i
                    else:
                        b = qbit.get(i, 0)
                        if mask & b: continue
                        m = mask | b
                    nxt[(fr[:c] + (S,) + fr[c + 1:], E, m)] += cnt
            cur = nxt
            if len(cur) > CAP: raise OverflowError("state cap")
            prof.append((sum(cur.values()), len(cur)))
    fin = defaultdict(int)
    for (fr, east, mask), cnt in cur.items(): fin[mask] += cnt
    return fin, prof

CAP = 2500000

def lf(k): return lgamma(k + 1) / log(10)
def fake_pred(n):
    out = 0.0
    for k in (4, 4 * (n - 2), (n - 2) ** 2):
        if k: out += k * log10(k) - lf(k)
    return out

def main():
    t0 = time.time()
    print("A. parity identity and its Monte Carlo cost (N = number of true solutions)")
    print("   n cb ci seed |      N  odd-all  | colour-only Z | E[Z(x)^2]/N^2 = samples for unit relative error")
    for (n, cb, ci, seeds) in ((3, 2, 2, (1, 2, 3)), (3, 2, 3, (1, 2)), (3, 3, 4, (1, 2)), (4, 2, 3, (1, 2)), (4, 3, 4, (1,))):
        for s in seeds:
            pcs = gen(n, cb, ci, random.Random(s))
            full = (1 << n * n) - 1
            ex, _ = sweep(n, pcs, "exact"); N = ex[full]
            par, _ = sweep(n, pcs, "parity")
            Z = sum(par.values()); m2 = sum(v * v for v in par.values())
            print("   %d %d  %d  %3d  | %6d  %6d   | %12d  | %.3g   %s" % (n, cb, ci, s, N, par[full], Z, m2 / N / N, "OK" if N == par[full] else "MISMATCH"))
    print("   time %.1fs" % (time.time() - t0)); t0 = time.time()

    print("\nB. colour-only overcount Z/N against the first-moment prediction prod_t n_t^n_t/n_t!")
    for (n, cb, ci, seeds) in ((3, 2, 2, range(1, 9)), (4, 2, 3, range(1, 9)), (5, 3, 4, range(1, 4)), (6, 3, 6, range(1, 2))):
        ls = []
        for s in seeds:
            pcs = gen(n, cb, ci, random.Random(100 + s))
            try:
                ex, prof = sweep(n, pcs, "exact"); N = ex[(1 << n * n) - 1]
            except OverflowError:
                print("   n=%d seed %d: exact sweep passed the state cap, skipped" % (n, s)); continue
            fr, _ = sweep(n, pcs, "free"); Z = fr[0]
            ls.append((log10(Z / N), N, max(p[1] for p in prof)))
        if not ls: continue
        print("   n=%d cb=%d ci=%d  log10(Z/N): mean %.2f  min %.2f max %.2f | predicted %.2f | N range %d..%d | peak exact states %d"
              % (n, cb, ci, sum(l[0] for l in ls) / len(ls), min(l[0] for l in ls), max(l[0] for l in ls), fake_pred(n),
                 min(l[1] for l in ls), max(l[1] for l in ls), max(l[2] for l in ls)))
    print("   time %.1fs" % (time.time() - t0)); t0 = time.time()

    print("\nC. exact transfer matrix: partial boards per distinct (frontier, used set) state, by depth")
    for (n, cb, ci, s) in ((4, 2, 3, 101), (5, 3, 4, 101), (5, 3, 5, 102)):
        pcs = gen(n, cb, ci, random.Random(s))
        ex, prof = sweep(n, pcs, "exact")
        tot_b = sum(p[0] for p in prof); tot_s = sum(p[1] for p in prof)
        pk = max(range(len(prof)), key=lambda i: prof[i][1])
        worst = prof[pk][0] / prof[pk][1]
        print("   n=%d ci=%d seed %d: N=%d, partial boards %d, DP states %d, overall ratio %.3f, ratio at the peak depth %.3f, peak states %d"
              % (n, ci, s, ex[(1 << n * n) - 1], tot_b, tot_s, tot_b / tot_s, worst, max(p[1] for p in prof)))
    print("   time %.1fs" % (time.time() - t0)); t0 = time.time()

    print("\nD. tightening ladder: Z_k = colour-only boards in which k tracked pieces are each used exactly once")
    for (n, cb, ci, s) in ((3, 2, 2, 1), (3, 2, 3, 2), (4, 2, 3, 1)):
        pcs = gen(n, cb, ci, random.Random(s))
        order = list(range(n * n)); random.Random(7).shuffle(order)
        zs = []
        for k in range(0, n * n + 1):
            try:
                fin, prof = sweep(n, pcs, "track", tuple(order[:k]))
            except OverflowError:
                zs.append((0, CAP)); continue
            zs.append((fin[(1 << k) - 1], max(p[1] for p in prof)))
        print("   n=%d seed %d  Z_k: %s" % (n, s, " ".join(str(z[0]) for z in zs)))
        print("        ratios Z_k+1/Z_k: %s   (1/e = 0.368)" % " ".join("%.2f" % (zs[k + 1][0] / zs[k][0]) for k in range(n * n) if zs[k][0] and zs[k + 1][0]))
        print("        peak DP states:   %s" % " ".join(str(z[1]) for z in zs))
    print("   time %.1fs" % (time.time() - t0))

if __name__ == "__main__":
    main()
