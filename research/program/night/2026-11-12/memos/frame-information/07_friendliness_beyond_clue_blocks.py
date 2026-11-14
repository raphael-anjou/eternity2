"""Frame information, part 7: is anything left once the clue corner blocks are used?

Generated 16x16 planted boards (5 frame colours, 17 interior colours) with four
clues read off the planted solution at the official corner clue cells (2,2),
(2,13), (13,2), (13,13). Statistic: the count of fills of the first interior
ring, pieces reusable, clue joints enforced (exact trace of 52 matrices).
Null: the same 56 inward letters shuffled, then, corner by corner, the four
letters that the clue block sees are redrawn from the rest of the word until
the block is fillable. So every null word passes the four sound corner tests,
as the true word does. This null is close to, not equal to, the uniform law
over passing words.

Question: how far above the null does the true word sit? That is the ranking
information that remains after the 17 bits of the sound filter.

Run: python3 -I 07_friendliness_beyond_clue_blocks.py
"""
import os
import random
import statistics
import sys
import time
from math import log, log2, sqrt

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "price-and-ledger-review"))
from rv_lib import *

t0 = time.time()
N = 16
NB = ((-1, 0), (0, 1), (1, 0), (0, -1))
OPP = (2, 3, 0, 1)
CL = [(2, 2), (2, 13), (13, 2), (13, 13)]
ring1 = [(1, c) for c in range(1, 15)] + [(r, 14) for r in range(2, 15)] + [(14, c) for c in range(13, 0, -1)] + [(r, 1) for r in range(13, 1, -1)]
L = len(ring1)
frame_edge = [(0, c) for c in range(1, 15)] + [(r, 15) for r in range(1, 15)] + [(15, c) for c in range(14, 0, -1)] + [(r, 0) for r in range(14, 0, -1)]
FIDX = {c: i for i, c in enumerate(frame_edge)}
INWARD = [2] * 14 + [3] * 14 + [0] * 14 + [1] * 14
spec = []
for k, (r, c) in enumerate(ring1):
    prev, nxt = ring1[k - 1], ring1[(k + 1) % L]
    row = []
    for s in range(4):
        x = (r + NB[s][0], c + NB[s][1])
        if x == prev:
            row.append("E")
        elif x == nxt:
            row.append("X")
        elif x in CL:
            row.append(("C", x, OPP[s]))
        elif x[0] in (0, 15) or x[1] in (0, 15):
            row.append(("F", FIDX[x]))
        else:
            row.append(None)
    spec.append(row)
CORNER_K = [0, 13, 26, 39]
BLOCK = {k: [(k - 1) % L, k, (k + 1) % L] for k in CORNER_K}
BPOS = {k: sorted({a[1] for j in BLOCK[k] for a in spec[j] if isinstance(a, tuple) and a[0] == "F"}) for k in CORNER_K}
CORNER_POS = {p for k in CORNER_K for p in BPOS[k]}
FREE_POS = [i for i in range(56) if i not in CORNER_POS]
assert all(len(v) == 4 for v in BPOS.values())


def study(seed, K):
    rng = random.Random(7000 + seed)
    ps, sol = gen_planted(N, 5, 17, rng)
    clue_ids = {sol[c][0] for c in CL}
    ORI = [q for i, p in enumerate(ps) if 0 not in p and i not in clue_ids for q in rotations(p)]
    cols = sorted({v for q in ORI for v in q})
    mc = {}

    def matrix(k, w):
        row = spec[k]
        key = []
        for s, a in enumerate(row):
            if isinstance(a, tuple):
                key.append((s, w[a[1]] if a[0] == "F" else sol[a[1]][1][a[2]]))
        key = (row.index("E"), row.index("X"), tuple(key))
        m = mc.get(key)
        if m is None:
            m = {}
            es, xs, fixed = key
            for q in ORI:
                if all(q[s] == v for s, v in fixed):
                    d = m.setdefault(q[es], {})
                    d[q[xs]] = d.get(q[xs], 0) + 1
            mc[key] = m
        return m

    def step(v, m):
        nv = {}
        for x, n in v.items():
            d = m.get(x)
            if d:
                for b, c in d.items():
                    nv[b] = nv.get(b, 0) + n * c
        return nv

    def block_ok(k, w):
        ks = BLOCK[k]
        v = {}
        for a, d in matrix(ks[0], w).items():
            for b, c in d.items():
                v[b] = v.get(b, 0) + c
        for j in ks[1:]:
            v = step(v, matrix(j, w))
            if not v:
                return False
        return True

    def z_ring(w):
        mats = [matrix(k, w) for k in range(L)]
        z = 0
        for a in cols:
            v = {a: 1}
            for m in mats:
                v = step(v, m)
                if not v:
                    break
            z += v.get(a, 0)
        return z

    true = [sol[c][1][INWARD[i]] for i, c in enumerate(frame_edge)]
    assert all(block_ok(k, true) for k in CORNER_K)
    zt = z_ring(true)
    assert zt > 0
    nul = []
    tries = 0
    dead = 0
    for _ in range(K):
        w = true[:]
        rng.shuffle(w)
        for k in CORNER_K:
            while not block_ok(k, w):
                tries += 1
                for p, q in zip(BPOS[k], rng.sample(FREE_POS, 4)):
                    w[p], w[q] = w[q], w[p]
        z = z_ring(w)
        if z > 0:
            nul.append(log(z))
        else:
            dead += 1
    return log(zt), nul, dead, tries


B, K = 30, 60
rows = []
for seed in range(B):
    lt, nul, dead, tries = study(seed, K)
    mu, sd = statistics.mean(nul), statistics.pstdev(nul)
    ge = sum(1 for x in nul if x >= lt)
    rows.append({"z": (lt - mu) / sd, "sd": sd, "dead": dead / K, "rank": (ge + 1) / (len(nul) + 1), "tries": tries / (4 * K)})
zs = [r["z"] for r in rows]
print("%d planted 16x16 boards with four corner clues, %d null words each, every null word passing the four clue blocks" % (B, K))
print("  redraws needed per corner block: %.1f (so a block passes with probability about %.3f, %.2f bits per corner)"
      % (statistics.mean(r["tries"] for r in rows), 1 / (1 + statistics.mean(r["tries"] for r in rows)), log2(1 + statistics.mean(r["tries"] for r in rows))))
print("  null words that pass the blocks but have no ring fill: %.3f" % statistics.mean(r["dead"] for r in rows))
print("  sd of ln count over null words: %.2f" % statistics.mean(r["sd"] for r in rows))
print("  z of the true word: mean %.2f (s.e. %.2f), median %.2f" % (statistics.mean(zs), statistics.pstdev(zs) / sqrt(B), statistics.median(zs)))
print("  mean -log2(rank fraction) of the true word: %.2f bits, against 1.44 bits with no signal: ranking is worth about %.2f bits"
      % (statistics.mean(-log2(r["rank"]) for r in rows), statistics.mean(-log2(r["rank"]) for r in rows) - 1 / log(2)))
print("[%.0fs]" % (time.time() - t0))
