"""Item 1: numerical test of barrier B1 (partial sieve) on toy boards, any cell order.
For a split of the tiles into tracked T and sieved U, and every S subset of U, run the frontier DP whose state is
(exposed colours, used & T), tiles of S reusable, tiles of U-S forbidden. Count table entries per depth, sum over S.
Compare with N_k (strict partial boards) and D_k (distinct (exposed colours, full used set)) of the same order.
Also the single-run parity variant: state (exposed colours, used & T, parity on U).
Run: python3 -I s_b1_toy.py"""
import random, sys, functools
from collections import defaultdict
from itertools import combinations
print = functools.partial(print, flush=True)
DIRS = ((-1, 0), (0, 1), (1, 0), (0, -1))  # N E S W

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
        if ok:
            rng.shuffle(pcs); return pcs

def plan(n, order):
    """Per step: for each direction, ('rim',) | ('need', index in exposed list) | ('free',); and the new exposed list."""
    filled = set(); exp = []; steps = []
    for (r, c) in order:
        req = []; newexp = [e for e in exp if (e[0][0] + DIRS[e[1]][0], e[0][1] + DIRS[e[1]][1]) != (r, c)]
        keep = [i for i, e in enumerate(exp) if (e[0][0] + DIRS[e[1]][0], e[0][1] + DIRS[e[1]][1]) != (r, c)]
        for d, (dr, dc) in enumerate(DIRS):
            y = (r + dr, c + dc)
            if not (0 <= y[0] < n and 0 <= y[1] < n): req.append(("rim", None))
            elif y in filled: req.append(("need", exp.index((y, (d + 2) % 4))))
            else: req.append(("free", None)); newexp.append(((r, c), d))
        steps.append((req, keep)); filled.add((r, c)); exp = newexp
    return steps

def run(n, pcs, steps, tracked, allowed, parity=()):
    """tracked: tiles used at most once (mask in state). allowed: reusable tiles. parity: reusable, parity bit in state.
    Returns per depth (sum of counts, number of states) and the final dict."""
    tb = {p: 1 << j for j, p in enumerate(tracked)}; pb = {p: 1 << j for j, p in enumerate(parity)}
    rots = [(i, p[k:] + p[:k]) for i, p in enumerate(pcs) if i in tb or i in pb or i in allowed for k in range(4)]
    cur = {((), 0, 0): 1}; prof = []
    for req, keep in steps:
        nxt = defaultdict(int)
        for (fr, m, par), cnt in cur.items():
            base = tuple(fr[i] for i in keep)
            for i, t in rots:
                ok = True; add = []
                for d in range(4):
                    kind, idx = req[d]
                    if kind == "rim":
                        if t[d] != 0: ok = False; break
                    elif kind == "need":
                        if t[d] != fr[idx]: ok = False; break
                    else:
                        if t[d] == 0: ok = False; break
                        add.append(t[d])
                if not ok: continue
                m2, p2 = m, par
                if i in tb:
                    if m & tb[i]: continue
                    m2 = m | tb[i]
                elif i in pb: p2 = par ^ pb[i]
                nxt[(base + tuple(add), m2, p2)] += cnt
        cur = nxt; prof.append((sum(cur.values()), len(cur)))
    return prof, cur

def orders(n, rng):
    rm = [(r, c) for r in range(n) for c in range(n)]
    ring = [(0, c) for c in range(n)] + [(r, n - 1) for r in range(1, n)] + [(n - 1, c) for c in range(n - 2, -1, -1)] + [(r, 0) for r in range(n - 2, 0, -1)]
    fr = ring + [x for x in rm if x not in ring]
    rnd = rm[:]; rng.shuffle(rnd)
    chk = [x for x in rm if (x[0] + x[1]) % 2 == 0] + [x for x in rm if (x[0] + x[1]) % 2 == 1]
    two = [(r, c) for r in ([0, n - 1] + list(range(1, n - 1))) for c in range(n)]   # top row, bottom row, then the middle: two frontiers that meet
    if n >= 4: return (("row-major", rm), ("frame first", fr), ("two-sided", two))   # random and checkerboard orders are too wide at 4x4 for a 2^u sweep
    return (("row-major", rm), ("frame first", fr), ("random", rnd), ("checkerboard", chk))

def main():
    worst = 9e9; worstp = 9e9; n_cases = 0
    print("board order split | N_sol IE-check | min_k SumS/D_k  min_k parity/D_k | at the widest strict level: N_k D_k maxmult SumS parity")
    for (n, cb, ci, seeds, umax) in ((3, 2, 2, (1, 2, 3), 9), (3, 2, 3, (4, 5), 9), (4, 2, 3, (1,), 6), (4, 3, 4, (3,), 6)):
        for s in seeds:
            rng = random.Random(s); pcs = gen(n, cb, ci, rng); nt = n * n; alltiles = list(range(nt))
            for oname, order in orders(n, random.Random(100 + s)):
                steps = plan(n, order)
                sprof, sfin = run(n, pcs, steps, alltiles, ())
                nsol = sum(sfin.values())
                # max multiplicity per (frontier, used set) is max count in the strict DP; recompute at the widest level
                kw = max(range(nt), key=lambda k: sprof[k][0])
                framep = [i for i, p in enumerate(pcs) if 0 in p]
                r2 = random.Random(7 * s + n)
                splits = [("U=frame tiles", framep), ("U=random half", r2.sample(alltiles, nt // 2)), ("U=all", alltiles), ("U=one tile", [r2.randrange(nt)]), ("U=interior", [i for i in alltiles if i not in framep])]
                for sname, U in splits:
                    U = U[:umax]; T = [i for i in alltiles if i not in U]; u = len(U)
                    tot = [0] * nt; ie = 0
                    for k in range(u + 1):
                        for S in combinations(U, k):
                            prof, fin = run(n, pcs, steps, T, set(S))
                            for d in range(nt): tot[d] += prof[d][1]
                            ie += (-1) ** (u - k) * sum(v for (f, m, p), v in fin.items() if m == (1 << len(T)) - 1)
                    pprof, pfin = run(n, pcs, steps, T, (), tuple(U))
                    r1 = min(tot[d] / sprof[d][1] for d in range(nt) if sprof[d][1]); r2_ = min(pprof[d][1] / sprof[d][1] for d in range(nt) if sprof[d][1])
                    worst = min(worst, r1); worstp = min(worstp, r2_); n_cases += 1
                    print("%dx%d c%d+%d s%d %-12s %-14s | %5d %s | %8.2f %8.2f | %d %d %s %d %d" % (n, n, cb, ci, s, oname, sname + "(%d)" % u, nsol, "ok" if ie == nsol else "MISMATCH %d" % ie, r1, r2_,
                          sprof[kw][0], sprof[kw][1], "", tot[kw], pprof[kw][1]))
    print("\ncases %d ; smallest ratio (sum over S of table entries)/D_k over all depths, orders, splits: %.3f ; parity form: %.3f" % (n_cases, worst, worstp))
    print("B1 in its strong form (>= D_k, no m needed) predicts both >= 1.")
main()
