"""Second analysis, claim 3: number of valid bottom arcs on the REAL pieces by SMC (row-scan border order),
with and without the two bottom clue-block tests, against the model's beta_k. python3 -I r3_arcs.py REPS NP"""
import sys, random, time
from math import log2, log10
from collections import Counter
from itertools import permutations
pre = open(__file__.replace("r3_arcs.py", "r1_corners.py")).read().split("# ---------------- Part A: chains")[0]
exec("\n".join(l for l in pre.split("\n") if not l.startswith('print("pieces')))
REPS = int(sys.argv[1]); NP = int(sys.argv[2])
idx = {}
for kind, lst in (("e", edges), ("c", corners)):
    for i in lst:
        a, b, _ = desc[i]
        for key in ((kind, a, -1), (kind, -1, b), (kind, a, b), (kind, -1, -1)):
            idx.setdefault(key, []).append(i)
cells = [(15, c) for c in range(16)]
for r in range(14, 0, -1):
    cells += [(r, 0), (r, 15)]
cells += [(0, c) for c in range(16)]
order = [RIDX[c] for c in cells]
def run(rng, tests):
    posstep = {k: s for s, k in enumerate(order)}
    done = {}
    for j in tests:
        done.setdefault(max(posstep[k] for k in CS[j].ridx), []).append(j)
    parts = [([None] * 60, 0)] * NP
    logZ = 0.0; out = []
    for step, k in enumerate(order):
        kind = "c" if k in (0, 15, 30, 45) else "e"
        pk, nk = (k - 1) % 60, (k + 1) % 60
        new = []; ws = []
        chk = done.get(step, ())
        for fr, used in parts:
            a = desc[fr[pk]][1] if fr[pk] is not None else -1
            b = desc[fr[nk]][0] if fr[nk] is not None else -1
            cs = [i for i in idx.get((kind, a, b), ()) if not used >> i & 1]
            if chk:
                good = []
                for i in cs:
                    fr[k] = i
                    if all(CS[j].get(15, tuple(desc[fr[r]][2] for r in CS[j].ridx))[0] > 0 for j in chk):
                        good.append(i)
                fr[k] = None
                cs = good
            if cs:
                i = cs[rng.randrange(len(cs))]
                f2 = list(fr); f2[k] = i
                new.append((f2, used | 1 << i)); ws.append(len(cs))
        tot = sum(ws)
        logZ += log10(tot / len(parts))
        out.append(logZ)
        stepw = tot / NP; u = rng.random() * stepw; res = []; acc = 0.0
        for p, w in zip(new, ws):
            acc += w
            while u < acc and len(res) < NP:
                res.append(p); u += stepw
        while len(res) < NP: res.append(new[-1])
        parts = res
    return out
rng = random.Random(7)
def mean_log(tests):
    acc = [0.0] * 60
    for _ in range(REPS):
        o = run(rng, tests)
        for i, x in enumerate(o): acc[i] += 10 ** (x - 20 - i * 0.3)   # scaled to stay in float range
    return [log10(a / REPS) + 20 + i * 0.3 for i, a in enumerate(acc)]
L10 = log2(10)
U = mean_log(()); B = mean_log((2, 3)); A4 = mean_log((0, 1, 2, 3))
MODEL = {16: 47.68, 18: 53.84, 20: 59.86, 22: 65.75, 26: 77.06, 30: 87.71, 34: 97.60, 36: 102.22, 38: 106.61, 40: 110.74, 42: 114.58, 44: 118.10, 60: 125.28}
print("arc cells | real pieces log2 N (SMC) | model beta | real, two bottom blocks on | block share bits | budget log2 C = 134.40 - that | core-s at 1e8/s | at 3e7/s")
for n in (16, 18, 20, 22, 26, 30, 34, 36, 38, 40, 42, 44, 60):
    u = U[n - 1] * L10; b = B[n - 1] * L10
    be = 134.40 - b
    print("%3d | %7.2f | %7.2f | %7.2f | %5.2f | %6.2f | %10.3g | %10.3g" % (n, u, MODEL[n], b, u - b, be, 2 ** be / 1e8, 2 ** be / 3e7))
print("full frame, real: no test 2^%.2f (exact 2^124.93), two bottom blocks 2^%.2f, four blocks 2^%.2f" % (U[59] * L10, B[59] * L10, A4[59] * L10))
def lsum(xs):
    m = max(xs); return m + log10(sum(10 ** (x - m) for x in xs))
print("walk of all border prefixes in this order, log2 sum_j N_j: no test %.2f, four blocks %.2f (widest level: cell %d, 2^%.2f)"
      % (lsum(U) * L10, lsum(A4) * L10, A4.index(max(A4)) + 1, max(A4) * L10))
for n in (38, 40, 42, 44):
    print("   walk to %d cells with the two bottom blocks: log2 sum_{j<=%d} N_j = %.2f (N at %d: %.2f)" % (n, n, lsum(B[:n]) * L10, n, B[n - 1] * L10))
