"""Review check 5: is the "friendly border word" signal an artefact of its null?

The memo ranks the true border word of a planted 16x16 board against words
made by shuffling its 56 inward letters. A real competitor of the true frame
is not a shuffled word, it is another VALID FRAME of the same pieces. Here
both nulls are run side by side on the same boards with the same statistic
(count of first-ring fills, pieces reusable, no clues), written from scratch:

  shuffled   the 56 inward letters in random order (the memo's null)
  frames     words read off random valid frames of the same piece set, found
             by a randomised depth-first search around the ring (valid, not
             uniform)

Also a cheaper statistic, to see where the signal lives: the product of the
candidate counts of the four corner cells of the interior.

And, for check 4 part B at a larger sample: arc consistency on the first ring
only against arc consistency on the whole interior (official pieces).

Run: uv run --with numpy python 05_friendliness_null.py
"""
import os
import random
import statistics
import sys
import time
from math import log, log2, sqrt

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "price-and-ledger-review"))
import rv_lib

sys.setrecursionlimit(10000)
T0 = time.time()
N = 16
NB = ((-1, 0), (0, 1), (1, 0), (0, -1))
OPP = (2, 3, 0, 1)
RING0 = rv_lib.ring(N)  # 60 frame cells, cyclic
ring1 = [(1, c) for c in range(1, 15)] + [(r, 14) for r in range(2, 15)] + [(14, c) for c in range(13, 0, -1)] + [(r, 1) for r in range(13, 1, -1)]
L = len(ring1)
SPEC = []  # per ring-1 cell: (entry side, exit side, [(side, frame cell)])
for k, x in enumerate(ring1):
    prev, nxt = ring1[k - 1], ring1[(k + 1) % L]
    es = xs = None
    seam = []
    for s in range(4):
        y = (x[0] + NB[s][0], x[1] + NB[s][1])
        if y == prev:
            es = s
        elif y == nxt:
            xs = s
        elif y[0] in (0, 15) or y[1] in (0, 15):
            seam.append((s, y))
    SPEC.append((es, xs, seam))


def inward_side(cell):
    r, c = cell
    return 2 if r == 0 else 0 if r == 15 else 1 if c == 0 else 3


EDGE_CELLS = [x for x in RING0 if not (x[0] in (0, 15) and x[1] in (0, 15))]


def study(seed, K):
    rng = random.Random(9000 + seed)
    ps, sol = rv_lib.gen_planted(N, 5, 17, rng)
    ORI = [q for p in ps if 0 not in p for q in rv_lib.rotations(p)]
    cols = sorted({v for q in ORI for v in q})
    ci = {v: k for k, v in enumerate(cols)}
    C = len(cols)
    cache = {}

    def mat(k, word):
        es, xs, seam = SPEC[k]
        key = (es, xs, tuple((s, word[y]) for s, y in seam))
        m = cache.get(key)
        if m is None:
            m = np.zeros((C, C))
            for q in ORI:
                if all(q[s] == v for s, v in key[2]):
                    m[ci[q[es]], ci[q[xs]]] += 1
            cache[key] = m
        return m

    def z1(word):
        P = mat(0, word)
        for k in range(1, L):
            P = P @ mat(k, word)
            if not P.any():
                return 0.0
        return float(np.trace(P))

    def corner_stat(word):
        v = 1.0
        for k in (0, 13, 26, 39):
            v *= mat(k, word).sum()
        return v

    true = {x: sol[x][1][inward_side(x)] for x in EDGE_CELLS}
    # null 1: shuffled letters
    lets = [true[x] for x in EDGE_CELLS]
    shuf = []
    for _ in range(K):
        w = lets[:]
        rng.shuffle(w)
        shuf.append(dict(zip(EDGE_CELLS, w)))
    # null 2: random valid frames
    en = rv_lib.Enum(N, ps)
    frames = []
    restarts = 0
    while len(frames) < K:
        board, used = {}, set()
        nodes = [0]

        def go(d):
            if d == len(RING0):
                return True
            r, c = RING0[d]
            key = []
            for s in range(4):
                rr, cc = r + NB[s][0], c + NB[s][1]
                if rr < 0 or rr >= N or cc < 0 or cc >= N:
                    key.append(0)
                else:
                    q = board.get((rr, cc))
                    key.append(q[OPP[s]] if q is not None else -1)
            cand = [t for t in en.cands(tuple(key)) if t[0] not in used]
            rng.shuffle(cand)
            for i, q in cand:
                nodes[0] += 1
                if nodes[0] > 4000:
                    raise OverflowError
                board[(r, c)] = q
                used.add(i)
                if go(d + 1):
                    return True
                used.discard(i)
                del board[(r, c)]
            return False

        try:
            ok = go(0)
        except OverflowError:
            ok = False
        if ok:
            frames.append({x: board[x][inward_side(x)] for x in EDGE_CELLS})
        else:
            restarts += 1
    out = {}
    for stat_name, f in (("ring", z1), ("corner cells", corner_stat)):
        vt = f(true)
        assert vt > 0
        for null_name, words in (("shuffled", shuf), ("frames", frames)):
            vals = [f(w) for w in words]
            alive = [log(v) for v in vals if v > 0]
            mu, sd = statistics.mean(alive), statistics.pstdev(alive)
            ge = sum(1 for v in alive if v >= log(vt))
            out[(stat_name, null_name)] = {"alive": len(alive) / len(vals), "mean": mu, "sd": sd, "z": (log(vt) - mu) / sd,
                                           "rank": (ge + 1) / (len(alive) + 1)}
    return out, restarts


B, K = 24, 120
acc = {}
rs = 0
for seed in range(B):
    o, r = study(seed, K)
    rs += r
    for k, v in o.items():
        acc.setdefault(k, []).append(v)
print("%d planted 16x16 boards (5 frame, 17 interior colours, no clues), %d null words of each kind per board ; frame search restarts per board %.1f" % (B, K, rs / B))
print("statistic | null | null words alive | mean ln count (alive) | sd | z of the true word: mean (s.e.) | mean rank fraction among alive (0.5 = no signal) | mean -log2 rank minus 1.44")
for k in sorted(acc):
    v = acc[k]
    zs = [x["z"] for x in v]
    print("  %-12s | %-8s | %.3f | %.2f | %.2f | %.2f (%.2f) | %.3f | %.2f"
          % (k[0], k[1], statistics.mean(x["alive"] for x in v), statistics.mean(x["mean"] for x in v), statistics.mean(x["sd"] for x in v),
             statistics.mean(zs), statistics.pstdev(zs) / sqrt(B), statistics.mean(x["rank"] for x in v),
             statistics.mean(-log2(x["rank"]) for x in v) - 1 / log(2)))
print("[%.0fs]" % (time.time() - T0))
sys.stdout.flush()

# ------------------------------------------------------------ check 4 B, larger
print("\n== official pieces, random border words: arc consistency on the first ring only against the whole interior ==")
pieces, hints = rv_lib.load_official()
int_or = [(i, rv_lib.rotations(p)[k]) for i, p in enumerate(pieces) if 0 not in p for k in range(4)]
letters = []
for p in pieces:
    if sum(1 for v in p if v == 0) == 1:
        letters.append(p[(p.index(0) + 2) % 4])
cellsB = [(r, c) for r in range(1, 15) for c in range(1, 15)]
ring_set = set(ring1)
seam_slots = [((1, c), 0) for c in range(1, 15)] + [((r, 14), 1) for r in range(1, 15)] + [((14, c), 2) for c in range(1, 15)] + [((r, 1), 3) for r in range(1, 15)]
by_side = [{} for _ in range(4)]
for t in int_or:
    for s in range(4):
        by_side[s].setdefault(t[1][s], set()).add(t)
ALL = set(int_or)
ncol = [len(by_side[s]) for s in range(4)]


def ac(word, region):
    dom = {}
    for (x, s), v in zip(seam_slots, word):
        dom[x] = (dom[x] if x in dom else ALL) & by_side[s].get(v, set())
    if any(not v for v in dom.values()):
        return False
    queue = list(dom)
    inq = set(queue)
    while queue:
        x = queue.pop()
        inq.discard(x)
        dx = dom.get(x, ALL)
        for s in range(4):
            y = (x[0] + NB[s][0], x[1] + NB[s][1])
            if y not in region:
                continue
            offer = {q[s] for _, q in dx}
            if len(offer) == ncol[s]:
                continue
            dy = dom.get(y, ALL)
            new = {t for t in dy if t[1][OPP[s]] in offer}
            if len(new) != len(dy):
                dom[y] = new
                if not new:
                    return False
                if y not in inq:
                    queue.append(y)
                    inq.add(y)
    return True


rng = random.Random(10)
W = 1500
c0 = c1 = c2 = 0
allc = set(cellsB)
for _ in range(W):
    w = letters[:]
    rng.shuffle(w)
    a = ac(w, set())
    b = ac(w, ring_set)
    c = ac(w, allc)
    c0 += not a
    c1 += not b
    c2 += not c
print("  %d words: dead at a seam cell %.4f (%.2f bits) | arc consistency, first ring only %.4f (%.2f bits) | arc consistency, all 196 cells %.4f (%.2f bits)"
      % (W, c0 / W, -log2(1 - c0 / W), c1 / W, -log2(1 - c1 / W), c2 / W, -log2(1 - c2 / W)))
print("  memo (4,000 words): corner cells 0.63 bits, exact first-ring count 0.99 bits")
print("[%.0fs]" % (time.time() - T0))
