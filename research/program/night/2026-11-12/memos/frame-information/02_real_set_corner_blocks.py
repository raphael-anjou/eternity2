"""Frame information, part 2: exact counts on the official pieces, corner by corner.

For each board corner: enumerate every valid chain of 5 border pieces (the
corner piece and two edge pieces on each side), then ask whether the 2x2
interior block in that corner, whose far cell is a clue, can be filled with
distinct interior pieces. The answer depends only on the 4 inward colours of
the chain. Also: the plain corner-cell test (no clue used), and a sampled
extension to the 3x3 block (7 border pieces).

Uses the referee's enumerator (read only): ../price-and-ledger-review/rv_lib.py
Run: python3 -I 02_real_set_corner_blocks.py
"""
import os
import random
import sys
import time
from math import log2

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "price-and-ledger-review"))
from rv_lib import *

t0 = time.time()
pieces, hints = load_official()
N = 16
en = Enum(N, pieces)
OPP = (2, 3, 0, 1)
NB = ((-1, 0), (0, 1), (1, 0), (0, -1))

# clue cells with their oriented pieces, read from the CSV (clockwise rotation)
lines = [l.strip() for l in open(CSV) if l.strip()][1:]
clue = {}
for i, l in enumerate(lines):
    c = l.split(",")
    if int(c[4]) or int(c[5]) or int(c[6]):
        clue[(int(c[5]), int(c[4]))] = (i, rotations(pieces[i])[int(c[6])])
print("clues (row, col) -> (piece index, top right bottom left):", {k: v for k, v in sorted(clue.items())})
n_int = sum(1 for p in pieces if 0 not in p)
n_edge = sum(1 for p in pieces if sum(1 for v in p if v == 0) == 1)
print("pieces: %d interior, %d edge, %d corner" % (n_int, n_edge, len(pieces) - n_int - n_edge))


def chains(cells):
    """every valid placement of distinct border pieces on `cells` (each cell
    after the first touches an earlier one); yields the board dict."""
    board = {}
    used = set()

    def go(d):
        if d == len(cells):
            yield dict(board)
            return
        r, c = cells[d]
        key = []
        for s in range(4):
            rr, cc = r + NB[s][0], c + NB[s][1]
            if rr < 0 or rr >= N or cc < 0 or cc >= N:
                key.append(0)
            else:
                q = board.get((rr, cc))
                key.append(q[1][OPP[s]] if q is not None else -1)
        for i, q in en.cands(tuple(key)):
            if i in used:
                continue
            used.add(i)
            board[(r, c)] = (i, q)
            yield from go(d + 1)
            used.discard(i)
            del board[(r, c)]

    yield from go(0)


def fills(order, preset):
    """number of valid fills of `order` (distinct interior pieces) given preset cells."""
    return en.run(order, preset=preset)["per"][-1]


def nodes(order, preset):
    return sum(en.run(order, preset=preset)["per"][1:])


rng = random.Random(20261112)
tot_bits = 0.0
tot_bits_plain = 0.0
print("\nper corner: 5-piece border chains, clue block test (3 free cells), plain corner-cell test, 3x3 extension")
for (R, C) in ((0, 0), (0, 15), (15, 0), (15, 15)):
    dr = 1 if R == 0 else -1
    dc = 1 if C == 0 else -1
    fr5 = [(R, C), (R, C + dc), (R, C + 2 * dc), (R + dr, C), (R + 2 * dr, C)]
    cl = (R + 2 * dr, C + 2 * dc)
    assert cl in clue
    a, b, d = (R + dr, C + 2 * dc), (R + 2 * dr, C + dc), (R + dr, C + dc)
    order = [a, b, d]
    cache = {}
    cache1 = {}
    nch = feas = plain_ok = 0
    nodes_sum = 0
    feasible = []
    for bd in chains(fr5):
        nch += 1
        # inward colours seen by the block: two from the row side, two from the column side
        vside = 2 if R == 0 else 0   # side of a row-border piece that faces the interior
        hside = 1 if C == 0 else 3
        key = (bd[fr5[1]][1][vside], bd[fr5[2]][1][vside], bd[fr5[3]][1][hside], bd[fr5[4]][1][hside])
        v = cache.get(key)
        if v is None:
            pre = dict(bd)
            pre[cl] = clue[cl]
            res = en.run(order, preset=pre)["per"]
            v = (res[-1], sum(res[1:]))
            cache[key] = v
        k1 = (key[0], key[2])
        v1 = cache1.get(k1)
        if v1 is None:
            # plain test: the corner cell alone, clue ignored (clue pieces stay unavailable)
            used_c = {x[0] for x in clue.values()}
            cnt = 0
            for i, q in en.allrot:
                if i in used_c or 0 in q:
                    continue
                # d touches fr5[1] vertically and fr5[3] horizontally
                if q[OPP[vside]] == key[0] and q[OPP[hside]] == key[2]:
                    cnt += 1
            v1 = cnt
            cache1[k1] = v1
        nodes_sum += v[1]
        if v1 > 0:
            plain_ok += 1
        if v[0] > 0:
            feas += 1
            feasible.append(bd)
    nt = sum(1 for v in cache.values() if v[0] > 0)
    f = feas / nch
    tot_bits += -log2(f)
    tot_bits_plain += -log2(plain_ok / nch)
    print("  corner (%2d,%2d): chains %d | clue block fillable: %d (fraction %.4f, %.2f bits) | distinct inward 4-tuples seen %d, fillable %d | mean nodes of the test %.2f | plain corner cell alive: fraction %.4f (%.2f bits)"
          % (R, C, nch, feas, f, -log2(f), len(cache), nt, nodes_sum / nch, plain_ok / nch, -log2(plain_ok / nch)))
    # 3x3 extension, sampled: 7 border pieces, 8 free interior cells
    e1, e2 = (R, C + 3 * dc), (R + 3 * dr, C)
    o9 = order + [(R + dr, C + 3 * dc), (R + 2 * dr, C + 3 * dc), (R + 3 * dr, C + dc), (R + 3 * dr, C + 2 * dc), (R + 3 * dr, C + 3 * dc)]
    S = 1500
    tried = alive = 0
    cnts = []
    while tried < S:
        bd = rng.choice(feasible)
        usedb = {x[0] for x in bd.values()}
        # all valid one-piece extensions on each side
        ext = []
        for cell, prev, side in ((e1, fr5[2], 1 if C == 0 else 3), (e2, fr5[4], 2 if R == 0 else 0)):
            want = bd[prev][1][side]
            lst = []
            for i, q in en.allrot:
                if i in usedb or sum(1 for v in q if v == 0) != 1:
                    continue
                rimside = (0 if R == 0 else 2) if cell[0] == R else (3 if C == 0 else 1)
                if q[rimside] == 0 and q[OPP[side]] == want:
                    lst.append((i, q))
            ext.append(lst)
        pairs = [(x, y) for x in ext[0] for y in ext[1] if x[0] != y[0]]
        if not pairs:
            continue
        x, y = rng.choice(pairs)
        pre = dict(bd)
        pre[e1], pre[e2], pre[cl] = x, y, clue[cl]
        n9 = fills(o9, pre)
        tried += 1
        cnts.append(n9)
        if n9 > 0:
            alive += 1
    cnts.sort()
    print("      3x3 block given a fillable 2x2 block (%d sampled 7-chains): fillable %.3f, extra bits %.2f; fills mean %.1f median %d"
          % (S, alive / S, -log2(alive / S), sum(cnts) / S, cnts[S // 2]))
print("\nsum over four corners: clue block test %.2f bits (model: 17.2) ; plain corner-cell test %.2f bits" % (tot_bits, tot_bits_plain))
print("[%.1fs]" % (time.time() - t0))
