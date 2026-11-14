"""Review check 3 (stdlib).

(a) Exact corner counts on the official pieces, written from scratch (no
    rv_lib): 5-piece border chains per corner, how many leave the clue block
    fillable with distinct interior pieces, bits. Two conventions for the
    pool: every interior piece except this corner's clue (the memo's), and
    every interior piece except all five clues (what a solver may use).
    Also the same fraction weighted by an estimate of how many frames extend
    each chain is NOT computed; see the review text.
(b) Row-scan model (rv_lib, refereed tonight): where the row scan closes the
    four clue blocks, and what each pair of corner clues is worth to the
    10^40.49 price.
(c) design-recipe: chance that a painted set has no rotation-duplicate pair,
    interior pieces only against all 256.

Run: python3 -I 03_corner_chains_and_row_scan.py
"""
import os
import random
import sys
import time
from math import log2, log10

HERE = os.path.dirname(os.path.abspath(__file__))
CSV = os.path.abspath(os.path.join(HERE, *[".."] * 6, "engine", "data", "official_eternity2.csv"))
T0 = time.time()


def rot(p, k):
    return p[4 - k:] + p[:4 - k] if k else p


pieces, clue = [], {}
for i, l in enumerate([l.strip() for l in open(CSV) if l.strip()][1:]):
    c = l.split(",")
    p = tuple(0 if int(x, 2) == 65535 else int(x, 2) for x in c[:4])
    pieces.append(p)
    if int(c[4]) or int(c[5]) or int(c[6]):
        clue[(int(c[5]), int(c[4]))] = (i, rot(p, int(c[6])))  # (row, col) -> (id, top right bottom left)
N = 16
corners = [i for i, p in enumerate(pieces) if p.count(0) == 2]
edges = [i for i, p in enumerate(pieces) if p.count(0) == 1]
inter = [i for i, p in enumerate(pieces) if p.count(0) == 0]
clue_ids = {v[0] for v in clue.values()}
print("clues:", {k: v for k, v in sorted(clue.items())})

# Work in a local frame for each board corner: rotate the board so the corner
# sits at top-left. A board turn by k quarter turns clockwise maps an oriented
# piece q to rot(q, k). So for corner number g (0 top-left, 1 top-right,
# 2 bottom-right, 3 bottom-left) we turn the board counter-clockwise g times:
# pieces are free anyway, only the clue must be turned: q -> rot(q, -g).
CORNER_CLUE = {0: (2, 2), 1: (2, 13), 2: (13, 13), 3: (13, 2)}

# oriented border pieces in the local top-left frame
corner_or = []  # grey on top and left
for i in corners:
    for k in range(4):
        q = rot(pieces[i], k)
        if q[0] == 0 and q[3] == 0:
            corner_or.append((i, q))
top_or = []  # grey on top only
left_or = []  # grey on left only
for i in edges:
    for k in range(4):
        q = rot(pieces[i], k)
        if q[0] == 0:
            top_or.append((i, q))
        if q[3] == 0:
            left_or.append((i, q))
assert len(corner_or) == 4 and len(top_or) == 56 and len(left_or) == 56
int_or = [(i, rot(pieces[i], k)) for i in inter for k in range(4)]

print("\n== (a) corner chains and clue blocks, own enumeration ==")
tot = {"one": 0.0, "all": 0.0}
for g in range(4):
    cid, cq = clue[CORNER_CLUE[g]]
    cq = rot(cq, (4 - g) % 4)  # clue in the local frame, sitting at local (2,2)
    res = {}
    for pool_name in ("one", "all"):
        banned = {cid} if pool_name == "one" else set(clue_ids)
        cand = [(i, q) for i, q in int_or if i not in banned]
        # cell (1,2): top = a2, bottom = clue top. cell (2,1): left = b2, right = clue left.
        # cell (1,1): top = a1, left = b1, right = (1,2).left, bottom = (2,1).top
        by12 = {}
        for i, q in cand:
            if q[2] == cq[0]:
                by12.setdefault(q[0], []).append((i, q[3]))  # key top colour -> (id, left colour)
        by21 = {}
        for i, q in cand:
            if q[1] == cq[3]:
                by21.setdefault(q[3], []).append((i, q[0]))  # key left colour -> (id, top colour)
        by11 = {}
        for i, q in cand:
            by11.setdefault(q, []).append(i)
        cache = {}

        def fills(a1, a2, b1, b2):
            k = (a1, a2, b1, b2)
            v = cache.get(k)
            if v is None:
                v = 0
                for i, lc in by12.get(a2, ()):
                    for j, tc in by21.get(b2, ()):
                        if j == i:
                            continue
                        for m in by11.get((a1, lc, tc, b1), ()):
                            if m != i and m != j:
                                v += 1
                cache[k] = v
            return v

        nch = ok = 0
        for ci, cqq in corner_or:
            for i1, q1 in top_or:
                if q1[3] != cqq[1]:
                    continue
                for i2, q2 in top_or:
                    if i2 == i1 or q2[3] != q1[1]:
                        continue
                    for j1, r1 in left_or:
                        if j1 in (i1, i2) or r1[0] != cqq[2]:
                            continue
                        for j2, r2 in left_or:
                            if j2 in (i1, i2, j1) or r2[0] != r1[2]:
                                continue
                            nch += 1
                            if fills(q1[2], q2[2], r1[1], r2[1]) > 0:
                                ok += 1
        res[pool_name] = (nch, ok, len(cache), sum(1 for v in cache.values() if v > 0))
        tot[pool_name] += -log2(ok / nch)
    a, b = res["one"], res["all"]
    print("  corner %d (clue at %s): chains %d | fillable, pool without this clue only: %d (%.4f, %.2f bits; inward 4-tuples seen %d, fillable %d) | pool without all five clues: %d (%.4f, %.2f bits)"
          % (g, CORNER_CLUE[g], a[0], a[1], a[1] / a[0], -log2(a[1] / a[0]), a[2], a[3], b[1], b[1] / b[0], -log2(b[1] / b[0])))
print("  sum of bits: %.2f (memo's pool) ; %.2f (pool without all five clues)   memo: 57,131 chains; 2,716 / 3,499 / 2,875 / 2,637; 17.17 bits" % (tot["one"], tot["all"]))
print("  note: corner order here is top-left, top-right, bottom-right, bottom-left; the memo lists (0,0), (0,15), (15,0), (15,15)")
print("  [%.0fs]" % (time.time() - T0))
sys.stdout.flush()

# ---------------------------------------------------------------- (b)
print("\n== (b) the row-scan model and the corner clues ==")
sys.path.insert(0, os.path.join(HERE, "..", "price-and-ledger-review"))
import rv_lib

ps, hints = rv_lib.load_official()
M = rv_lib.Model(N, rv_lib.census(ps))
order = rv_lib.rows_from(N, False)  # row 15 first, the order priced at 10^40.49
names = {"all five": hints,
         "centre + the two near the start (rows 13)": [h for h in hints if h[0] >= 8],
         "centre + the two near the end (rows 2)": [h for h in hints if h[0] <= 8],
         "centre only": [h for h in hints if 5 <= h[0] <= 10],
         "none": []}
base = None
for nm, hs in names.items():
    pr = M.profile(order, hs, count_hint_levels=True)
    s = rv_lib.logsum(pr)
    pk = max(range(len(pr)), key=lambda i: pr[i])
    if base is None:
        base = s
        prof5 = pr
    print("  clues %-42s: sum 10^%.2f (%+.2f digits against all five), peak 10^%.2f at cell %d" % (nm, s, s - base, pr[pk], pk + 1))
# where in the scan are the blocks closed, and what share of the nodes lies beyond
pos = {c: k for k, c in enumerate(order)}
for name, cells in (("bottom blocks (rows 13 to 14)", [(13, 1), (13, 2), (14, 1), (14, 2), (13, 13), (13, 14), (14, 13), (14, 14)]),
                    ("top blocks (rows 1 to 2)", [(1, 1), (1, 2), (2, 1), (2, 2), (1, 13), (1, 14), (2, 13), (2, 14)])):
    last = max(pos[c] for c in cells)
    before = rv_lib.logsum(prof5[:last + 1])
    after = rv_lib.logsum(prof5[last + 1:]) if last + 1 < len(prof5) else float("-inf")
    print("  %s closed at scan cell %d of 256: nodes spent before 10^%.2f, nodes still to come 10^%.2f (share to come %.3g)"
          % (name, last + 1, before, after, 10 ** (after - base)))
print("  one corner block is worth %.2f bits = %.2f digits; four are worth %.2f digits" % (17.17 / 4, 17.17 / 4 / log2(10), 17.17 / log2(10)))

# ---------------------------------------------------------------- (c)
print("\n== (c) design-recipe: no rotation-duplicate pair in a painted set ==")
rng = random.Random(3)


def canon(p):
    return min(rot(p, k) for k in range(4))


def trial():
    col = {}
    n = 16
    for r in range(n):
        for c in range(n):
            if c < n - 1:
                col[(r, c, "h")] = rng.randint(1, 5) if r in (0, n - 1) else 5 + rng.randint(1, 17)
            if r < n - 1:
                col[(r, c, "v")] = rng.randint(1, 5) if c in (0, n - 1) else 5 + rng.randint(1, 17)
    seen_i, seen_b = set(), set()
    dup_i = dup_b = sym = 0
    for r in range(n):
        for c in range(n):
            p = (col[(r - 1, c, "v")] if r else 0, col[(r, c, "h")] if c < n - 1 else 0, col[(r, c, "v")] if r < n - 1 else 0, col[(r, c - 1, "h")] if c else 0)
            k = canon(p)
            if 0 in p:
                dup_b += k in seen_b
                seen_b.add(k)
            else:
                dup_i += k in seen_i
                seen_i.add(k)
                sym += p == rot(p, 2)
    return dup_i, dup_b, sym


T = 4000
r = [trial() for _ in range(T)]
print("  %d painted 16x16 sets (uniform colours, 5 frame + 17 interior):" % T)
print("    repeated interior pieces: mean %.2f ; repeated border pieces: mean %.2f ; half-turn-symmetric interior pieces: mean %.2f"
      % (sum(x[0] for x in r) / T, sum(x[1] for x in r) / T, sum(x[2] for x in r) / T))
print("    no symmetric and no duplicate INTERIOR piece (the page's 'one time in five'): %.3f" % (sum(1 for x in r if x[0] == 0 and x[2] == 0) / T))
print("    no duplicate piece among all 256: %.4f ; and no symmetric piece either: %.4f" % (sum(1 for x in r if x[0] == 0 and x[1] == 0) / T, sum(1 for x in r if x[0] == 0 and x[1] == 0 and x[2] == 0) / T))
print("[%.0fs]" % (time.time() - T0))
