"""Second analysis, claim 1, part B. SMC over real valid frames, gate = exact frame count.
python3 -I r1b_smc.py REPS NP
Counts |Omega| (gate), |Omega_j| (block j alive), two pairs, |H| (all four), by
sequential Monte Carlo with the block test applied as a hard constraint when its
window completes. Corner regions are placed first so the tests act on diverse particles."""
import sys, random, time
from math import log2, log10
from collections import Counter
from itertools import permutations
pre = open(__file__.replace("r1b_smc.py", "r1_corners.py")).read().split("# ---------------- Part A: chains")[0]
exec("\n".join(l for l in pre.split("\n") if not l.startswith('print("pieces')))
REPS = int(sys.argv[1]); NP = int(sys.argv[2])
CORNER_IDX = {0, 15, 30, 45}
idx = {}
for kind, lst in (("e", edges), ("c", corners)):
    for i in lst:
        a, b, _ = desc[i]
        for key in ((kind, a, -1), (kind, -1, b), (kind, a, b), (kind, -1, -1)):
            idx.setdefault(key, []).append(i)
def make_order(first=True):
    region = []
    for co in CS:
        k0 = RIDX[(co.R, co.C)]
        region.append([(k0 + d) % 60 for d in (0, 1, 2, -1, -2)])
    flat = {k for r in region for k in r}
    mid = [k for k in range(60) if k not in flat]
    reg = [k for r in region for k in r]
    return reg + mid if first else mid + reg
def smc(rng, tests, order, keep=False):
    posstep = {k: s for s, k in enumerate(order)}
    done = {}
    for j in tests:
        done.setdefault(max(posstep[k] for k in CS[j].ridx), []).append(j)
    parts = [([None] * 60, 0)] * NP
    logZ = 0.0
    for step, k in enumerate(order):
        kind = "c" if k in CORNER_IDX else "e"
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
                    ok = True
                    for j in chk:
                        co = CS[j]
                        if co.get(15, tuple(desc[fr[r]][2] for r in co.ridx))[0] == 0:
                            ok = False; break
                    if ok: good.append(i)
                fr[k] = None
                cs = good
            w = len(cs)
            if w:
                i = cs[rng.randrange(w)]
                f2 = list(fr); f2[k] = i
                new.append((f2, used | 1 << i)); ws.append(w)
        if not ws:
            return None, None
        tot = sum(ws)
        logZ += log10(tot / len(parts))
        stepw = tot / NP
        u = rng.random() * stepw
        out = []; acc = 0.0
        for p, w in zip(new, ws):
            acc += w
            while u < acc and len(out) < NP:
                out.append(p); u += stepw
        while len(out) < NP:
            out.append(new[-1])
        parts = out
    return logZ, (parts if keep else None)
def est(name, tests, rng, order):
    zs = []
    for _ in range(REPS):
        lz, _p = smc(rng, tests, order)
        zs.append(0.0 if lz is None else 10 ** (lz - 30))
    m = sum(zs) / len(zs)
    se = (sum((z - m) ** 2 for z in zs) / (len(zs) - 1) / len(zs)) ** 0.5
    return m * 1e30, se * 1e30
t0 = time.time()
rng = random.Random(20261113)
O1 = make_order(True)
res = {}
for name, tests in (("Omega", ()), ("c0 (0,0)", (0,)), ("c1 (0,15)", (1,)), ("c2 (15,0)", (2,)), ("c3 (15,15)", (3,)),
                    ("bottom pair c2,c3", (2, 3)), ("top pair c0,c1", (0, 1)), ("left pair c0,c2", (0, 2)), ("H: all four", (0, 1, 2, 3))):
    m, se = est(name, tests, rng, O1)
    res[name] = (m, se)
    if name == "Omega":
        print("gate: |Omega| estimate %.4e (rel s.e. %.3f), exact %.4e, ratio %.4f" % (m, se / m, EXACT_FRAMES, m / EXACT_FRAMES))
    else:
        print("%-20s count %.4e (rel s.e. %.3f)  share of exact |Omega| = 2^-%.2f  (1 s.e.: %.2f to %.2f)"
              % (name, m, se / m, -log2(m / EXACT_FRAMES), -log2((m + se) / EXACT_FRAMES), -log2(max(m - se, 1e-300) / EXACT_FRAMES)))
    print("   [%.0fs]" % (time.time() - t0)); sys.stdout.flush()
f = {k: res[k][0] / EXACT_FRAMES for k in res}
s4 = sum(-log2(f[k]) for k in ("c0 (0,0)", "c1 (0,15)", "c2 (15,0)", "c3 (15,15)"))
print("sum of the four frame-weighted single-corner bits: %.2f ; joint (H): %.2f ; chain product (memo): 17.45" % (s4, -log2(f["H: all four"])))
for nm, a, b in (("bottom pair c2,c3", "c2 (15,0)", "c3 (15,15)"), ("top pair c0,c1", "c0 (0,0)", "c1 (0,15)"), ("left pair c0,c2", "c0 (0,0)", "c2 (15,0)")):
    print("%s: joint 2^-%.2f against product of singles 2^-%.2f" % (nm, -log2(f[nm]), -log2(f[a] * f[b])))
# rough per-frame distribution of the smallest corner core, uniform frames (regions last for diversity there)
O2 = make_order(False)
mins = Counter(); n = 0; kal = Counter()
for _ in range(REPS * 3):
    lz, parts = smc(rng, (), O2, keep=True)
    if parts is None: continue
    for fr in {tuple(p[0]) for p in parts}:
        n += 1
        best = 9; al = 0
        for co in CS:
            r = min_core(co, tuple(desc[fr[r_]][2] for r_ in co.ridx))
            if r is None: al += 1
            else: best = min(best, r[0])
        mins[best] += 1; kal[al] += 1
print("uniform frames, distinct final particles %d: smallest corner core per frame %s ; alive corners per frame %s"
      % (n, {k: round(v / n, 3) for k, v in sorted(mins.items())}, {k: round(v / n, 4) for k, v in sorted(kal.items())}))
print("[%.0fs]" % (time.time() - t0))
