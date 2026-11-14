"""Second analysis: does the median degenerate again INSIDE H? For frames of H (four 2x2 clue blocks pass),
test the 3x3 block around each corner clue (8 free cells, 6 edge cells touching). python3 -I r4_h_3x3.py REPS NP"""
import sys, random, time
from math import log2, log10
from collections import Counter
from itertools import permutations
src = open(__file__.replace("r4_h_3x3.py", "r1b_smc.py")).read().split("t0 = time.time()")[0]
src = src.replace("r1b_smc.py", "r4_h_3x3.py").replace("REPS = int(sys.argv[1]); NP = int(sys.argv[2])", "")
REPS = int(sys.argv[1]); NP = int(sys.argv[2])
exec(src)
t0 = time.time()
def block3(co, fr):
    """(fillable?, nodes) of the 3x3 block at corner co under frame fr; distinct pieces, five clue pieces out."""
    R, C = co.R, co.C
    dr = 1 if R == 0 else -1; dc = 1 if C == 0 else -1
    cl = co.cl
    cells = [(R + dr * a, C + dc * b) for a in (1, 2, 3) for b in (1, 2, 3) if (R + dr * a, C + dc * b) != cl]
    # order: clue-adjacent and frame-adjacent first
    cells.sort(key=lambda c: (abs(c[0] - cl[0]) + abs(c[1] - cl[1]), c))
    order = [(R + dr, C + dc), (R + dr, C + 2 * dc), (R + 2 * dr, C + dc), (R + dr, C + 3 * dc), (R + 3 * dr, C + dc), (R + 2 * dr, C + 3 * dc), (R + 3 * dr, C + 2 * dc), (R + 3 * dr, C + 3 * dc)]
    assert sorted(order) == sorted(cells)
    fixed = {cl: co.clq}
    ecol = {}
    for t in (1, 2, 3):
        for cell in ((R, C + dc * t), (R + dr * t, C)):
            ecol[cell] = desc[fr[RIDX[cell]]][2]
    used = set(); nodes = 0
    def pat(cell):
        o = []
        for s in range(4):
            nb = (cell[0] + NB[s][0], cell[1] + NB[s][1])
            o.append(ecol[nb] if nb in ecol else (fixed[nb][OPP[s]] if nb in fixed else -1))
        return tuple(o)
    def go(d):
        nonlocal nodes
        if d == 8: return True
        cell = order[d]
        for i, q in cands(pat(cell)):
            if i in used: continue
            nodes += 1
            used.add(i); fixed[cell] = q
            ok = go(d + 1)
            used.discard(i); del fixed[cell]
            if ok: return True
        return False
    return go(0), nodes
rng = random.Random(99)
O1 = make_order(True)
n = 0; dead_any = 0; per = [0] * 4; nd = []; ndead = Counter(); reps_stat = []
cache = {}
for rep in range(REPS):
    lz, parts = smc(rng, (0, 1, 2, 3), O1, keep=True)
    if parts is None: continue
    fs = {tuple(p[0]) for p in parts}
    r_n = 0; r_d = 0
    for fr in fs:
        k = 0
        for j, co in enumerate(CS):
            key = (j,) + tuple(fr[RIDX[c]] for c in [(co.R, co.C + (1 if co.C == 0 else -1) * t) for t in (1, 2, 3)] + [(co.R + (1 if co.R == 0 else -1) * t, co.C) for t in (1, 2, 3)])
            v = cache.get(key)
            if v is None:
                v = block3(co, fr); cache[key] = v
            if not v[0]:
                per[j] += 1; k += 1; nd.append(v[1])
        n += 1; ndead[k] += 1; r_n += 1; r_d += (k > 0)
        if k: dead_any += 1
    reps_stat.append(r_d / r_n)
m = sum(reps_stat) / len(reps_stat)
se = (sum((x - m) ** 2 for x in reps_stat) / (len(reps_stat) - 1) / len(reps_stat)) ** 0.5
print("H frames (distinct final particles over %d reps x %d): %d ; distinct corner windows of 6 edge cells tested: %d" % (REPS, NP, n, len(cache)))
print("share of H frames with at least one unfillable 3x3 clue block: %.3f pooled ; mean of per-rep shares %.3f (s.e. %.3f)" % (dead_any / n, m, se))
print("per corner dead share:", [round(x / n, 3) for x in per], "; number of dead 3x3 blocks per frame:", {k: round(v / n, 3) for k, v in sorted(ndead.items())})
if nd:
    nd.sort()
    print("nodes of the 3x3 refutation (this fixed order): median %d, mean %.0f, max %d" % (nd[len(nd) // 2], sum(nd) / len(nd), nd[-1]))
print("[%.0fs]" % (time.time() - t0))
