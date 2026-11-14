"""Review check 4 (stdlib): how much does "the bits arrive only at the end"
depend on the refuter?

Part A, 6x6 planted boards (the memo's three seeds, 4 frame colours, 6
interior colours). A uniform sample of valid frames; for each frame, is it
refuted with ZERO search nodes by
  corner   the four corner cells of the interior have a candidate
  AC       arc consistency on edge colours, pieces reusable
  AC+AD    AC plus all-different: naked and hidden singles, and a perfect
           matching between cells and pieces must exist, to a fixpoint
  SAC      AC+AD plus one level of probing (assume each cell/tile, propagate,
           delete on wipe-out): what failed-literal probing in a clause
           learner does before any branching
  reuse    exact: no fill even with pieces reusable (complete colour-only test)
  exact    exact: no fill with distinct pieces (the truth)
and how many depth-first nodes the plain spiral search needs to refute it.

Part B, official pieces: does propagation deeper than the first ring see more
than the first ring does? Random border words (shuffled inward letters, no
clues, pieces reusable), AC over the whole 14x14 interior against the four
corner cells alone.

Part C, model: does a row scan gain from closing the two far clue blocks first?

Run: python3 -I 04_refuter_dependence.py
"""
import os
import random
import sys
import time
from math import log2

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "price-and-ledger-review"))
import rv_lib

sys.setrecursionlimit(10000)
T0 = time.time()
OPP = (2, 3, 0, 1)
NB = ((-1, 0), (0, 1), (1, 0), (0, -1))


def frames_sample(n, en, k, rng):
    cells = rv_lib.ring(n)
    L = len(cells)
    board, used = {}, set()
    out, seen = [], [0]

    def go(d):
        r, c = cells[d]
        key = []
        for s in range(4):
            rr, cc = r + NB[s][0], c + NB[s][1]
            if rr < 0 or rr >= n or cc < 0 or cc >= n:
                key.append(0)
            else:
                q = board.get((rr, cc))
                key.append(q[OPP[s]] if q is not None else -1)
        for i, q in en.cands(tuple(key)):
            if i in used:
                continue
            board[(r, c)] = q
            if d + 1 == L:
                seen[0] += 1
                if len(out) < k:
                    out.append(dict(board))
                else:
                    j = rng.randrange(seen[0])
                    if j < k:
                        out[j] = dict(board)
            else:
                used.add(i)
                go(d + 1)
                used.discard(i)
            del board[(r, c)]

    go(0)
    return out, seen[0]


def spiral(n):
    o, lo, hi = [], 1, n - 2
    while lo <= hi:
        rc = [(lo, c) for c in range(lo, hi + 1)] + [(r, hi) for r in range(lo + 1, hi + 1)]
        rc += [(hi, c) for c in range(hi - 1, lo - 1, -1)] + [(r, lo) for r in range(hi - 1, lo, -1)]
        for x in rc:
            if x not in o:
                o.append(x)
        lo += 1
        hi -= 1
    return o


def propagate(dom, cells, nbrs, alldiff):
    """dom: cell -> set of (piece, oriented). In place. False on wipe-out."""
    while True:
        changed = False
        # arc consistency on colours
        again = True
        while again:
            again = False
            for x in cells:
                for s, y in nbrs[x]:
                    ok = {q[OPP[s]] for _, q in dom[y]}
                    new = {t for t in dom[x] if t[1][s] in ok}
                    if len(new) != len(dom[x]):
                        dom[x] = new
                        again = changed = True
                        if not new:
                            return False
        if not alldiff:
            return True
        ids = {x: {i for i, _ in dom[x]} for x in cells}
        # naked singles
        for x in cells:
            if len(ids[x]) == 1:
                i = next(iter(ids[x]))
                for y in cells:
                    if y != x and i in ids[y]:
                        dom[y] = {t for t in dom[y] if t[0] != i}
                        changed = True
                        if not dom[y]:
                            return False
        ids = {x: {i for i, _ in dom[x]} for x in cells}
        # hidden singles (every piece must be placed: as many pieces as cells)
        where = {}
        for x in cells:
            for i in ids[x]:
                where.setdefault(i, []).append(x)
        if len(where) < len(cells):
            return False
        for i, xs in where.items():
            if len(xs) == 1 and len(ids[xs[0]]) > 1:
                dom[xs[0]] = {t for t in dom[xs[0]] if t[0] == i}
                changed = True
        if not changed:
            # perfect matching cells -> pieces
            ids = {x: {i for i, _ in dom[x]} for x in cells}
            match = {}

            def aug(x, seen):
                for i in ids[x]:
                    if i in seen:
                        continue
                    seen.add(i)
                    if i not in match or aug(match[i], seen):
                        match[i] = x
                        return True
                return False

            for x in cells:
                if not aug(x, set()):
                    return False
            return True


def sac(dom, cells, nbrs):
    if not propagate(dom, cells, nbrs, True):
        return False
    changed = True
    while changed:
        changed = False
        for x in cells:
            for t in list(dom[x]):
                if t not in dom[x]:
                    continue
                d2 = {y: set(v) for y, v in dom.items()}
                d2[x] = {t}
                if not propagate(d2, cells, nbrs, True):
                    dom[x].discard(t)
                    changed = True
                    if not dom[x]:
                        return False
                    if not propagate(dom, cells, nbrs, True):
                        return False
    return True


def dfs(dom0, order, nbrs_prev, distinct, cap=200000):
    """returns (exists a fill, nodes visited) for a plain depth-first search
    along `order` using only the frame-filtered domains (no propagation)."""
    board, used = {}, set()
    nodes = [0]
    m = len(order)

    def go(d):
        x = order[d]
        for i, q in dom0[x]:
            if distinct and i in used:
                continue
            if any(board[y][OPP[s]] != q[s] for s, y in nbrs_prev[x]):
                continue
            nodes[0] += 1
            if nodes[0] > cap:
                raise OverflowError
            if d + 1 == m:
                return True
            board[x] = q
            used.add(i)
            r = go(d + 1)
            used.discard(i)
            del board[x]
            if r:
                return True
        return False

    return go(0), nodes[0]


print("== A. 6x6 planted boards, a uniform sample of valid frames ==")
K = 1200
for seed in (1, 2, 3):
    n, cf, ci = 6, 4, 6
    rng = random.Random(1000 * n + 100 * cf + 10 * ci + seed)  # the memo's boards
    ps, sol = rv_lib.gen_planted(n, cf, ci, rng)
    en = rv_lib.Enum(n, ps)
    fr, total = frames_sample(n, en, K, random.Random(40 + seed))
    cells = [(r, c) for r in range(1, n - 1) for c in range(1, n - 1)]
    order = spiral(n)
    pos = {x: k for k, x in enumerate(order)}
    nbrs = {x: [(s, (x[0] + NB[s][0], x[1] + NB[s][1])) for s in range(4) if (x[0] + NB[s][0], x[1] + NB[s][1]) in pos] for x in cells}
    nbrs_prev = {x: [(s, y) for s, y in nbrs[x] if pos[y] < pos[x]] for x in cells}
    interior_ids = [i for i, p in enumerate(ps) if 0 not in p]
    cnt = {k: 0 for k in ("corner", "AC", "AC+AD", "SAC", "reuse", "exact")}
    node_hist = []
    soundness = 0
    for bd in fr:
        dom0 = {}
        for x in cells:
            key = []
            for s in range(4):
                y = (x[0] + NB[s][0], x[1] + NB[s][1])
                key.append(bd[y][OPP[s]] if y in bd else -1)
            dom0[x] = [(i, q) for i, q in en.cands(tuple(key))]
        if any(not dom0[x] for x in ((1, 1), (1, n - 2), (n - 2, 1), (n - 2, n - 2))):
            cnt["corner"] += 1
        d = {x: set(v) for x, v in dom0.items()}
        r_ac = not propagate(d, cells, nbrs, False)
        cnt["AC"] += r_ac
        d = {x: set(v) for x, v in dom0.items()}
        r_ad = not propagate(d, cells, nbrs, True)
        cnt["AC+AD"] += r_ad
        d = {x: set(v) for x, v in dom0.items()}
        r_sac = not sac(d, cells, nbrs)
        cnt["SAC"] += r_sac
        ex_reuse, _ = dfs(dom0, order, nbrs_prev, False)
        ex, nodes = dfs(dom0, order, nbrs_prev, True)
        cnt["reuse"] += not ex_reuse
        cnt["exact"] += not ex
        if ex and (r_ac or r_ad or r_sac):
            soundness += 1
        if not ex:
            node_hist.append(nodes)
    S = len(fr)
    node_hist.sort()
    bits = lambda k: -log2(max(1 - cnt[k] / S, 0.5 / S))
    print("  seed %d: valid frames %d, sampled %d ; unsound refutations %d" % (seed, total, S, soundness))
    print("    refuted with zero search nodes: " + " | ".join("%s %.3f (%.2f bits)" % (k, cnt[k] / S, bits(k)) for k in cnt))
    print("    plain depth-first refutation of the frames that have no fill: median %d nodes, mean %.1f, 90%% %d ; share refuted within 0 nodes %.3f, within 4 nodes %.3f"
          % (node_hist[len(node_hist) // 2], sum(node_hist) / len(node_hist), node_hist[int(0.9 * len(node_hist))],
             sum(1 for x in node_hist if x == 0) / S, sum(1 for x in node_hist if x <= 4) / S))
    sys.stdout.flush()
print("  [%.0fs]" % (time.time() - T0))

# ------------------------------------------------------------------ B
print("\n== B. official pieces: arc consistency over the whole interior against the corner cells, random border words ==")
pieces, hints = rv_lib.load_official()
N = 16
int_or = [(i, rv_lib.rotations(p)[k]) for i, p in enumerate(pieces) if 0 not in p for k in range(4)]
letters = []
for p in pieces:
    if sum(1 for v in p if v == 0) == 1:
        g = p.index(0)
        letters.append(p[(g + 2) % 4])
assert len(letters) == 56
cellsB = [(r, c) for r in range(1, 15) for c in range(1, 15)]
setB = set(cellsB)
nbB = {x: [(s, (x[0] + NB[s][0], x[1] + NB[s][1])) for s in range(4) if (x[0] + NB[s][0], x[1] + NB[s][1]) in setB] for x in cellsB}
seam_slots = [((1, c), 0) for c in range(1, 15)] + [((r, 14), 1) for r in range(1, 15)] + [((14, c), 2) for c in range(1, 15)] + [((r, 1), 3) for r in range(1, 15)]
by_side = [{} for _ in range(4)]
for t in int_or:
    for s in range(4):
        by_side[s].setdefault(t[1][s], set()).add(t)
ALL = set(int_or)
rng = random.Random(9)
W = 300
dead_corner = dead_ac = 0
removed_far = 0
for _ in range(W):
    w = letters[:]
    rng.shuffle(w)
    dom = {}
    for (x, s), v in zip(seam_slots, w):
        dom[x] = (dom[x] if x in dom else ALL) & by_side[s].get(v, set())
    if any(not dom[x] for x in ((1, 1), (1, 14), (14, 1), (14, 14))):
        dead_corner += 1
    # worklist arc consistency; cells not in dom have the full set
    full_cols = [set(by_side[s]) for s in range(4)]
    queue = list(dom)
    inq = set(queue)
    alive = True
    touched = set(dom)
    while queue and alive:
        x = queue.pop()
        inq.discard(x)
        dx = dom.get(x, ALL)
        for s, y in nbB[x]:
            # restrict y by the colours x offers on side s
            offer = {q[s] for _, q in dx}
            if len(offer) == len(full_cols[s]):
                continue
            dy = dom.get(y, ALL)
            new = {t for t in dy if t[1][OPP[s]] in offer}
            if len(new) != len(dy):
                dom[y] = new
                touched.add(y)
                if not new:
                    alive = False
                    break
                if y not in inq:
                    queue.append(y)
                    inq.add(y)
    if not alive:
        dead_ac += 1
    removed_far += sum(1 for x in touched if 2 <= x[0] <= 13 and 2 <= x[1] <= 13)
print("  %d words: dead at a corner cell %.3f (%.2f bits) ; dead under arc consistency over all 196 cells %.3f (%.2f bits)"
      % (W, dead_corner / W, -log2(1 - dead_corner / W), dead_ac / W, -log2(1 - dead_ac / W)))
print("  memo, exact first-ring count with reusable pieces: 0.4955 dead (0.99 bits); corner cells 0.352 dead (0.63 bits)")
print("  cells beyond the first ring whose domain lost a tile, mean per word: %.1f of 144" % (removed_far / W))
print("  [%.0fs]" % (time.time() - T0))

# ------------------------------------------------------------------ C
print("\n== C. model: row scan with the two far clue blocks closed first ==")
M = rv_lib.Model(N, rv_lib.census(pieces))
row = rv_lib.rows_from(N, False)
base = rv_lib.logsum(M.profile(row, hints))
far = [(0, 0), (0, 1), (0, 2), (1, 0), (2, 0), (1, 2), (2, 1), (1, 1), (0, 15), (0, 14), (0, 13), (1, 15), (2, 15), (1, 13), (2, 14), (1, 14)]
o2 = far + [x for x in row if x not in set(far)]
pr = M.profile(o2, hints)
print("  plain row scan from row 15: 10^%.2f ; far blocks first, then the same scan: 10^%.2f ; log10 of the mean number of (two far chains + blocks) alive: %.2f"
      % (base, rv_lib.logsum(pr), pr[len(far) - 1]))
o3 = [x for x in row if x not in set(far)]
# far blocks inserted just before the peak instead
k = 150
o3 = o3[:k] + far + o3[k:]
print("  far blocks inserted after scan cell %d: 10^%.2f" % (k, rv_lib.logsum(M.profile(o3, hints))))
print("[%.0fs]" % (time.time() - T0))
