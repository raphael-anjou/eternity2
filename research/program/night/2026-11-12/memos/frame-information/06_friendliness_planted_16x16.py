"""Frame information, part 6: does a cheap count rank the TRUE border word high?

Generated 16x16 planted boards with the official shape (5 frame colours, 17
interior colours, no clues), where the true frame is known. Statistic of a
border word (the 56 inward colours in ring order): the number of fills of
interior cells next to the frame, pieces reusable, by transfer matrix.
  ring1   the whole first interior ring, 52 cells, corners included (exact trace)
  side1   four side strips of depth 1 (12 cells each, corners left out)
  side2   four side strips of depth 2 (24 cells each)
Null: the same 56 letters in uniformly random order. That ignores the lateral
frame colours, which the generator draws independently of the inward ones.

If the true word is like a word drawn in proportion to its count (size bias),
its log count sits about one null standard deviation above the null mean per
unit of sigma (z = sigma in natural-log units for a lognormal count).

Run: python3 -I 06_friendliness_planted_16x16.py
"""
import os
import random
import statistics
import sys
import time
from math import log, log2, erfc, sqrt

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "price-and-ledger-review"))
from rv_lib import *

t0 = time.time()
N = 16
NB = ((-1, 0), (0, 1), (1, 0), (0, -1))
OPP = (2, 3, 0, 1)
ring1 = [(1, c) for c in range(1, 15)] + [(r, 14) for r in range(2, 15)] + [(14, c) for c in range(13, 0, -1)] + [(r, 1) for r in range(13, 1, -1)]
L = len(ring1)
frame_edge = [(0, c) for c in range(1, 15)] + [(r, 15) for r in range(1, 15)] + [(15, c) for c in range(14, 0, -1)] + [(r, 0) for r in range(14, 0, -1)]
INWARD = [2] * 14 + [3] * 14 + [0] * 14 + [1] * 14   # side of the edge piece that faces the interior
spec = []
for k, (r, c) in enumerate(ring1):
    prev, nxt = ring1[k - 1], ring1[(k + 1) % L]
    row = []
    for s in range(4):
        x = (r + NB[s][0], c + NB[s][1])
        row.append("E" if x == prev else "X" if x == nxt else x if (x[0] in (0, 15) or x[1] in (0, 15)) else None)
    spec.append(row)


def study(seed, K1, K2):
    rng = random.Random(9000 + seed)
    ps, sol = gen_planted(N, 5, 17, rng)
    ORI = [q for p in ps if 0 not in p for q in rotations(p)]
    cols = sorted({v for q in ORI for v in q})
    mc = {}

    def matrix(k, word):
        row = spec[k]
        key = tuple((s, a if isinstance(a, str) else word[a]) for s, a in enumerate(row) if a is not None)
        m = mc.get(key)
        if m is None:
            m = {}
            es, xs = row.index("E"), row.index("X")
            fixed = [(s, v) for s, v in key if not isinstance(v, str)]
            for q in ORI:
                if all(q[s] == v for s, v in fixed):
                    d = m.setdefault(q[es], {})
                    d[q[xs]] = d.get(q[xs], 0) + 1
            mc[key] = m
        return m

    def z_ring(word):
        z = 0
        mats = [matrix(k, word) for k in range(L)]
        for a in cols:
            v = {a: 1}
            for m in mats:
                nv = {}
                for x, n in v.items():
                    d = m.get(x)
                    if d:
                        for b, c in d.items():
                            nv[b] = nv.get(b, 0) + n * c
                v = nv
                if not v:
                    break
            z += v.get(a, 0)
        return z

    # side strips: all sides are the top side after a rotation, letters read clockwise
    bytop = {}
    for q in ORI:
        bytop.setdefault(q[0], []).append(q)
    M1, M2 = {}, {}
    for t in cols:
        m1, m2 = {}, {}
        for q1 in bytop.get(t, ()):
            d = m1.setdefault(q1[3], {})
            d[q1[1]] = d.get(q1[1], 0) + 1
            for q2 in bytop.get(q1[2], ()):
                d = m2.setdefault((q1[3], q2[3]), {})
                kk = (q1[1], q2[1])
                d[kk] = d.get(kk, 0) + 1
        M1[t], M2[t] = m1, m2

    def z_sides(letters, MM):
        tot = 0.0
        for side in range(4):
            ls = letters[14 * side + 1: 14 * side + 13]
            v = None
            for t in ls:
                m = MM[t]
                nv = {}
                if v is None:
                    for a, d in m.items():
                        for b, c in d.items():
                            nv[b] = nv.get(b, 0) + c
                else:
                    for x, n in v.items():
                        d = m.get(x)
                        if d:
                            for b, c in d.items():
                                nv[b] = nv.get(b, 0) + n * c
                v = nv
                if not v:
                    return None
            tot += log(sum(v.values()))
        return tot

    true = [sol[c][1][INWARD[i]] for i, c in enumerate(frame_edge)]
    out = {}
    zt = z_ring(dict(zip(frame_edge, true)))
    assert zt > 0
    nul = []
    for _ in range(K1):
        w = true[:]
        rng.shuffle(w)
        nul.append(z_ring(dict(zip(frame_edge, w))))
    pos = [log(z) for z in nul if z > 0]
    out["ring1"] = (log(zt), pos, K1)
    t1, t2 = z_sides(true, M1), z_sides(true, M2)
    n1, n2 = [], []
    for _ in range(K2):
        w = true[:]
        rng.shuffle(w)
        a, b = z_sides(w, M1), z_sides(w, M2)
        if a is not None:
            n1.append(a)
        if b is not None:
            n2.append(b)
    out["side1"] = (t1, n1, K2)
    out["side2"] = (t2, n2, K2)
    return out


B, K1, K2 = 40, 200, 60
acc = {"ring1": [], "side1": [], "side2": []}
for seed in range(B):
    o = study(seed, K1, K2)
    for name, (lt, pos, K) in o.items():
        mu, sd = statistics.mean(pos), statistics.pstdev(pos)
        ge = sum(1 for x in pos if x >= lt)
        acc[name].append({"z": (lt - mu) / sd, "sd": sd, "alive": len(pos) / K, "rank": (ge + 1) / (K + 1), "rank_alive": (ge + 1) / (len(pos) + 1)})
print("%d planted 16x16 boards (5 frame colours, 17 interior colours, uniform random colours, no clues)   [%.0fs]" % (B, time.time() - t0))
print("statistic | null words alive | sd of ln count over alive null words | z of the true word: mean (s.e.), median | size-bias prediction z = sd | mean bits: sure, ranking among alive, total")
for name in ("ring1", "side1", "side2"):
    a = acc[name]
    zs = [x["z"] for x in a]
    sds = [x["sd"] for x in a]
    sure = statistics.mean(-log2(x["alive"]) for x in a)
    soft = statistics.mean(-log2(x["rank_alive"]) for x in a)
    tot = statistics.mean(-log2(x["rank"]) for x in a)
    print("  %s | %.3f | %.2f | %.2f (%.2f), %.2f | %.2f | %.2f, %.2f, %.2f"
          % (name, statistics.mean(x["alive"] for x in a), statistics.mean(sds), statistics.mean(zs), statistics.pstdev(zs) / sqrt(len(zs)), statistics.median(zs),
             statistics.mean(sds), sure, soft, tot))
    mz = statistics.mean(zs)
    print("      a Gaussian null word beats the mean true word with probability %.3f; ranking bits above the no-signal floor of 1.44: %.2f"
          % (0.5 * erfc(mz / sqrt(2)), soft - 1 / log(2)))
print("  note: with no signal, -log2 of a uniform rank averages 1/ln 2 = 1.44 bits; ranking bits must be read against that floor")
print("[%.0fs]" % (time.time() - t0))
