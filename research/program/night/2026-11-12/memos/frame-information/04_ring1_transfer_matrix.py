"""Frame information, part 4: official pieces, the first interior ring by transfer matrix.

The first interior ring is the 52 cells next to the frame. For a border word
(the 56 inward colours of the edge pieces, in ring order) the number Z1 of
fills of that ring, pieces reusable, clue joints enforced, is an exact trace
of a product of 52 small matrices. Reuse makes Z1 an upper bound on the number
of fills with distinct pieces, so Z1 = 0 is a sound refutation of the word.

Words are uniformly random arrangements of the official multiset of inward
colours. That is NOT the uniform law over valid frames (lateral colours are
ignored); it is the law a windowed test sees if inward and lateral colours are
unrelated.

Also: the expected number of boards with k broken joints, against the number
of boards within k breaks of a solution (arithmetic for the marginals question).

Run: python3 -I 04_ring1_transfer_matrix.py
"""
import os
import random
import statistics
import sys
import time
from math import comb, log10, log2

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "price-and-ledger-review"))
from rv_lib import *

t0 = time.time()
pieces, hints = load_official()
N = 16
NB = ((-1, 0), (0, 1), (1, 0), (0, -1))
OPP = (2, 3, 0, 1)
lines = [l.strip() for l in open(CSV) if l.strip()][1:]
clue = {}
for i, l in enumerate(lines):
    c = l.split(",")
    if int(c[4]) or int(c[5]) or int(c[6]):
        clue[(int(c[5]), int(c[4]))] = (i, rotations(pieces[i])[int(c[6])])
clue_ids = {v[0] for v in clue.values()}
ORI = [q for i, p in enumerate(pieces) if 0 not in p and i not in clue_ids for q in rotations(p)]
seam = []
for p in pieces:
    if sum(1 for v in p if v == 0) == 1:
        k = p.index(0)
        seam.append(p[(k + 2) % 4])
cols = sorted({v for q in ORI for v in q})
print("oriented interior pieces without the clues: %d ; inward colours of the 56 edge pieces: %d distinct" % (len(ORI), len(set(seam))))

ring1 = [(1, c) for c in range(1, 15)] + [(r, 14) for r in range(2, 15)] + [(14, c) for c in range(13, 0, -1)] + [(r, 1) for r in range(13, 1, -1)]
L = len(ring1)
frame_edge = [(0, c) for c in range(1, 15)] + [(r, 15) for r in range(1, 15)] + [(15, c) for c in range(14, 0, -1)] + [(r, 0) for r in range(14, 0, -1)]
spec = []  # per ring cell: for each side 'F' + frame cell, 'C' colour, 'E' entry, 'X' exit, or None
for k, (r, c) in enumerate(ring1):
    prev, nxt = ring1[k - 1], ring1[(k + 1) % L]
    row = []
    for s in range(4):
        x = (r + NB[s][0], c + NB[s][1])
        if x == prev:
            row.append(("E",))
        elif x == nxt:
            row.append(("X",))
        elif x in clue:
            row.append(("C", clue[x][1][OPP[s]]))
        elif x[0] in (0, 15) or x[1] in (0, 15):
            row.append(("F", x))
        else:
            row.append(None)
    spec.append(row)

mcache = {}


def matrix(k, word):
    row = spec[k]
    key = tuple((s, a[0], word[a[1]] if a[0] == "F" else (a[1] if a[0] == "C" else 0)) for s, a in enumerate(row) if a is not None)
    m = mcache.get(key)
    if m is None:
        m = {}
        es = next(s for s, a in enumerate(row) if a and a[0] == "E")
        xs = next(s for s, a in enumerate(row) if a and a[0] == "X")
        fixed = [(s, v) for (s, t, v) in key if t in ("F", "C")]
        for q in ORI:
            if all(q[s] == v for s, v in fixed):
                d = m.setdefault(q[es], {})
                d[q[xs]] = d.get(q[xs], 0) + 1
        mcache[key] = m
    return m


def chain(ks, word, start=None):
    """sum over colour paths through cells ks; start = fixed entry colour or None (free ends)."""
    v = {start: 1} if start is not None else None
    for k in ks:
        m = matrix(k, word)
        if v is None:
            nv = {}
            for a, d in m.items():
                for b, n in d.items():
                    nv[b] = nv.get(b, 0) + n
        else:
            nv = {}
            for a, x in v.items():
                d = m.get(a)
                if d:
                    for b, n in d.items():
                        nv[b] = nv.get(b, 0) + x * n
        v = nv
        if not v:
            return {}
    return v


def Z1(word):
    z = 0
    for a in cols:
        z += chain(range(L), word, start=a).get(a, 0)
    return z


corner_k = [0, 13, 26, 39]
rng = random.Random(4)
S = 400
alive_plain = alive_blocks = alive_ring = 0
logs = []
for it in range(S):
    w = seam[:]
    rng.shuffle(w)
    word = dict(zip(frame_edge, w))
    # plain corner cells: a piece with the two outward colours on adjacent sides (clue joints ignored)
    ok_plain = True
    ok_blocks = True
    for k in corner_k:
        r, c = ring1[k]
        fs = [(s, word[a[1]]) for s, a in enumerate(spec[k]) if a and a[0] == "F"]
        if not any(all(q[s] == v for s, v in fs) for q in ORI):
            ok_plain = False
        if not chain([(k - 1) % L, k, (k + 1) % L], word):
            ok_blocks = False
    z = Z1(word)
    alive_plain += ok_plain
    alive_blocks += ok_blocks
    alive_ring += z > 0
    if z > 0:
        logs.append(log10(z))
print("%d random border words:" % S)
print("  all four plain corner cells alive: %.3f (%.2f bits)" % (alive_plain / S, -log2(alive_plain / S)))
print("  all four clue blocks fillable (reuse allowed): %d of %d = %.4f (%.1f bits; expected from the per-corner exact fractions of script 02: 17.2 bits, about %.1f words)"
      % (alive_blocks, S, alive_blocks / S, -log2(alive_blocks / S) if alive_blocks else float("inf"), S * 2 ** -17.17))
print("  whole first ring fillable (reuse allowed, clue joints on): %d of %d" % (alive_ring, S))

# the clue joints dominate; to see what the ring adds beyond the corners, drop the clue joints
spec_nc = [[None if (a and a[0] == "C") else a for a in row] for row in spec]
spec, spec_keep = spec_nc, spec
mcache.clear()
S2 = 4000
alive_plain2 = alive_ring2 = 0
win = {1: 0, 2: 0, 4: 0, 6: 0}   # corner windows: the corner cell and h ring cells on each side
logs2 = []
for it in range(S2):
    w = seam[:]
    rng.shuffle(w)
    word = dict(zip(frame_edge, w))
    ok = True
    for k in corner_k:
        fs = [(s, word[a[1]]) for s, a in enumerate(spec[k]) if a and a[0] == "F"]
        if not any(all(q[s] == v for s, v in fs) for q in ORI):
            ok = False
    for h in win:
        if all(chain([(k + j) % L for j in range(-h, h + 1)], word) for k in corner_k):
            win[h] += 1
    z = Z1(word)
    alive_plain2 += ok
    alive_ring2 += z > 0
    if z > 0:
        logs2.append(log10(z))
print("clue joints dropped, %d fresh words (pieces reusable throughout):" % S2)
print("  four corner cells alive: %.4f (%.2f bits)" % (alive_plain2 / S2, -log2(alive_plain2 / S2)))
for h in win:
    print("  four corner windows of %2d ring cells fillable: %.4f (%.2f bits)" % (2 * h + 1, win[h] / S2, -log2(win[h] / S2)))
print("  whole first ring (52 cells) fillable: %.4f (%.2f bits)" % (alive_ring2 / S2, -log2(alive_ring2 / S2)))
print("  log10 Z1 over fillable words: mean %.2f, sd %.2f, min %.2f, max %.2f" % (statistics.mean(logs2), statistics.pstdev(logs2), min(logs2), max(logs2)))

print("\nmarginals over high-scoring boards: unrelated boards against relatives of the solution (five clues, arithmetic)")
S5 = 4.1e-8      # expected solutions with five clues (model, script 01 of the review)
q = 17.3         # effective colours per joint: early-joint match probability 1/17.34 (review, section 2)
for k in (1, 2, 4, 6, 8, 10):
    unrelated = log10(S5) + log10(comb(480, k)) + k * log10(q - 1)
    # relatives: boards within k breaks of the solution, generous count: rotate j = k/4 interior pieces (3 turns each, 4 breaks each)
    j = k // 4
    rel = sum(comb(191, i) * 3 ** i for i in range(0, j + 1))
    print("  %2d broken joints: unrelated boards about 10^%.1f ; relatives by rotating up to %d free interior pieces: %d ; share of relatives about 10^%.1f"
          % (k, unrelated, j, rel, log10(rel) - unrelated if unrelated > log10(rel) else 0.0))
print("[%.1fs]" % (time.time() - t0))
