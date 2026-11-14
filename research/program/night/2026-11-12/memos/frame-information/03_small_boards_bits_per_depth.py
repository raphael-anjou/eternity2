"""Frame information, part 3: small planted boards, exact.

For every valid frame of a 6x6 board (4 frame colours, 6 interior colours) the
interior is searched in inward spiral order and W_d, the
number of valid partial interiors after d interior cells, is recorded.
From W_d over frames:
  mu_d        mean of W_d (the first-moment quantity of the model)
  sure bits   -log2(fraction of frames with W_d > 0): what a sound depth-d
              refuter learns (the true frame always has W_d > 0)
  soft bits   -log2(fraction of frames with W_d >= W_d of the planted frame):
              what ranking frames by W_d learns
  post bits   log2(W_d(planted) / mu_d): what weighting frames by W_d learns
  size bias   log2(E[W_d^2] / E[W_d]^2): the value post bits should have on
              average if the planted frame is drawn in proportion to W_d
  cost        mean over frames of W_1 + ... + W_d (nodes spent per frame)
Also three cheap relaxations and the position marginals under W_d weights.

Uses the referee's library (read only): ../price-and-ledger-review/rv_lib.py
Run: python3 -I -u 03_small_boards_bits_per_depth.py
"""
import json
import os
import random
import sys
import time
from math import log2, log10

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "price-and-ledger-review"))
from rv_lib import *

sys.setrecursionlimit(10000)
T0 = time.time()
OPP = (2, 3, 0, 1)
NB = ((-1, 0), (0, 1), (1, 0), (0, -1))


def spiral(n):
    o, lo, hi = [], 1, n - 2
    while lo <= hi:
        rc = [(lo, c) for c in range(lo, hi + 1)] + [(r, hi) for r in range(lo + 1, hi + 1)]
        rc += [(hi, c) for c in range(hi - 1, lo - 1, -1)] + [(r, lo) for r in range(hi - 1, lo, -1)]
        o += rc
        lo += 1
        hi -= 1
    seen, out = set(), []
    for x in o:
        if x not in seen:
            seen.add(x)
            out.append(x)
    return out


def all_frames(n, en, on_leaf):
    """depth-first over the ring; calls on_leaf(board) for every valid frame."""
    cells = ring(n)
    L = len(cells)
    board = {}
    used = set()
    cnt = [0]

    def go(d):
        r, c = cells[d]
        key = []
        for s in range(4):
            rr, cc = r + NB[s][0], c + NB[s][1]
            if rr < 0 or rr >= n or cc < 0 or cc >= n:
                key.append(0)
            else:
                q = board.get((rr, cc))
                key.append(q[1][OPP[s]] if q is not None else -1)
        for i, q in en.cands(tuple(key)):
            if i in used:
                continue
            cnt[0] += 1
            board[(r, c)] = (i, q)
            if d + 1 == L:
                on_leaf(board)
            else:
                used.add(i)
                go(d + 1)
                used.discard(i)
            del board[(r, c)]

    go(0)
    return cnt[0]


class Interior:
    def __init__(self, n, en, order):
        self.n, self.en, self.order = n, en, order
        pos = {c: k for k, c in enumerate(order)}
        self.nbr = []
        for k, (r, c) in enumerate(order):
            row = []
            for s in range(4):
                x = (r + NB[s][0], c + NB[s][1])
                if x in pos and pos[x] > k:
                    row.append(None)
                else:
                    row.append((x, OPP[s]))
            self.nbr.append(row)
        self.m = len(order)

    def profile(self, frame, cap, distinct=True):
        """W_1..W_m under the frame (dict cell -> oriented tuple). Returns
        (W, censored). With distinct=False pieces may be reused."""
        m, order, nbr, cands = self.m, self.order, self.nbr, self.en.cands
        board = dict(frame)
        W = [0] * m
        used = set()
        tot = [0]

        def go(d):
            key = tuple(-1 if x is None else board[x[0]][x[1]] for x in nbr[d])
            cell = order[d]
            for i, q in cands(key):
                if distinct and i in used:
                    continue
                W[d] += 1
                tot[0] += 1
                if tot[0] > cap:
                    raise OverflowError
                if d + 1 < m:
                    if distinct:
                        used.add(i)
                    board[cell] = q
                    go(d + 1)
                    if distinct:
                        used.discard(i)
            board.pop(cell, None)

        try:
            go(0)
        except OverflowError:
            return W, True
        return W, False

    def exists_reuse(self, frame, cap):
        """is there a full interior fill when a piece may be used many times? (None if cap hit)"""
        m, order, nbr, cands = self.m, self.order, self.nbr, self.en.cands
        board = dict(frame)
        tot = [0]

        def go(d):
            key = tuple(-1 if x is None else board[x[0]][x[1]] for x in nbr[d])
            cell = order[d]
            for i, q in cands(key):
                tot[0] += 1
                if tot[0] > cap:
                    raise OverflowError
                if d + 1 == m:
                    return True
                board[cell] = q
                if go(d + 1):
                    return True
            board.pop(cell, None)
            return False

        try:
            return go(0)
        except OverflowError:
            return None


def study(n, cf, ci, seed, sample=None, cap=10 ** 9, marg=True):
    rng = random.Random(1000 * n + 100 * cf + 10 * ci + seed)  # the referee's seeds
    ps, sol = gen_planted(n, cf, ci, rng)
    en = Enum(n, ps)
    frn = ring(n)
    order = spiral(n)
    m = len(order)
    it = Interior(n, en, order)
    mdl = Model(n, census(ps))
    pr = mdl.profile(frn + order)
    mu_model = [x - pr[len(frn) - 1] for x in pr[len(frn):]]
    planted = {c: sol[c][1] for c in frn}
    planted_ids = tuple(sol[c][0] for c in frn)

    frames = []
    srng = random.Random(777 + seed)
    seen = [0]

    def leaf(board):
        seen[0] += 1
        rec = (tuple(board[c][0] for c in frn), tuple(board[c][1] for c in frn))
        if sample is None or len(frames) < sample:
            frames.append(rec)
        else:
            j = srng.randrange(seen[0])
            if j < sample:
                frames[j] = rec

    fnodes = all_frames(n, en, leaf)
    F_all = seen[0]
    t_fr = time.time() - T0
    # the planted frame is measured separately and is not part of the sample statistics unless drawn
    Wp, cens_p = it.profile(planted, 10 ** 8)
    Ws = []
    ncens = 0
    reuse_dead = reuse_unknown = 0
    corner_dead = 0
    icorners = [k for k, (r, c) in enumerate(order) if r in (1, n - 2) and c in (1, n - 2)]
    mg = {}
    for ids, qs in frames:
        fr = dict(zip(frn, qs))
        W, cens = it.profile(fr, cap)
        ncens += cens
        Ws.append(W)
        ex = it.exists_reuse(fr, 20000)
        if ex is None:
            reuse_unknown += 1
        elif not ex:
            reuse_dead += 1
        # plain corner test: each interior corner cell has a candidate given its two frame neighbours
        dead = False
        for k in icorners:
            r, c = order[k]
            key = []
            for s in range(4):
                x = (r + NB[s][0], c + NB[s][1])
                key.append(fr[x][OPP[s]] if x in fr else -1)
            if not en.cands(tuple(key)):
                dead = True
        corner_dead += dead
        if marg:
            for d in MARG_DEPTHS(m):
                w = 1 if d == 0 else W[d - 1]
                if w:
                    a = mg.setdefault(d, {})
                    for p, i in enumerate(ids):
                        a[(p, i)] = a.get((p, i), 0) + w
    F = len(frames)
    nsol_frames = sum(1 for W in Ws if W[-1] > 0)
    out = {"n": n, "cf": cf, "ci": ci, "seed": seed, "frames_all": F_all, "frames_used": F, "frame_tree_nodes": fnodes,
           "censored": ncens, "solution_frames_in_sample": nsol_frames, "planted_W": Wp, "rows": []}
    print("\n%dx%d (%d/%d) seed %d: valid frames %d (frame tree %d nodes), frames measured %d%s, censored at %d nodes: %d, frames with a full solution: %d, solutions under the planted frame: %d"
          % (n, n, cf, ci, seed, F_all, fnodes, F, "" if sample is None else " (uniform sample)", cap, ncens, nsol_frames, Wp[-1]))
    print("  bits available in total: log2(frames / solution frames) = %.1f" % (log2(F_all / max(1, nsol_frames * (F_all / F))) if nsol_frames else float("nan")))
    print("  relaxations at the root: plain corner cells dead %.3f (%.2f bits) | no fill even with pieces reused: %.3f (%.2f bits; undecided at 20000 nodes: %d) | no fill with distinct pieces: %.4f"
          % (corner_dead / F, -log2(1 - corner_dead / F), reuse_dead / F, -log2(1 - reuse_dead / F), reuse_unknown, 1 - nsol_frames / F))
    tot_cost = sum(sum(W) for W in Ws) / F
    print("  mean interior tree per frame %.1f nodes ; planted frame %d nodes" % (tot_cost, sum(Wp)))
    print("    d | mu_d (model) | -log2 mu_d | sure bits | soft bits | post bits | size bias | cost per frame up to d (share of total)" + (" | marginal bits" if marg else ""))
    cum = 0.0
    for d in range(1, m + 1):
        col = [W[d - 1] for W in Ws]
        mu = sum(col) / F
        cum += mu
        alive = sum(1 for x in col if x > 0)
        ge = sum(1 for x in col if x >= Wp[d - 1])
        sure = -log2(alive / F) if alive else float("inf")
        soft = -log2(ge / F) if ge else log2(F)
        post = log2(Wp[d - 1] / mu) if mu > 0 else float("inf")
        m2 = sum(x * x for x in col) / F
        sb = log2(m2 / (mu * mu)) if mu > 0 else float("inf")
        mb = None
        if marg and d in mg and 0 in mg:
            z0 = sum(v for (p, i), v in mg[0].items() if p == 0)
            zd = sum(v for (p, i), v in mg[d].items() if p == 0)
            mb = 0.0
            for p, i in enumerate(planted_ids):
                a0 = mg[0].get((p, i), 0) / z0
                ad = mg[d].get((p, i), 0) / zd
                mb += log2(ad / a0) if a0 > 0 and ad > 0 else 0.0
        out["rows"].append({"d": d, "mu": mu, "mu_model": 10 ** mu_model[d - 1], "sure": sure, "soft": soft, "post": post, "sizebias": sb, "cum": cum, "marg": mb})
        print("   %2d | %9.2f (%9.2f) | %6.2f | %6.2f | %6.2f | %6.2f | %6.2f | %10.1f (%.2f)%s"
              % (d, mu, 10 ** mu_model[d - 1], max(0.0, -log2(mu)) if mu > 0 else 99, sure, soft, post, sb, cum, cum / tot_cost,
                 "" if mb is None else " | %6.2f" % mb))
    print("  [%.0fs]" % (time.time() - T0))
    return out


def MARG_DEPTHS(m):
    return [0] + list(range(2, m + 1, 2))


if __name__ == "__main__":
    res = []
    # Larger boards were tried and dropped for cost in Python: 8x8 (5/8) has a frame
    # tree above 1e8 nodes; 7x7 (5/7) and 7x7 (5/8) did not finish one board in 3
    # core-minutes each. Seeds 1 to 3 are the referee's boards.
    for seed in (1, 2, 3, 4, 5):
        res.append(study(6, 4, 6, seed))
    json.dump(res, open(os.path.join(HERE, "03_small_boards_bits_per_depth.json"), "w"))
    print("\nSUMMARY: bits a sound depth-limited refuter has when a given share of the per-frame cost is spent")
    for o in res:
        tot = o["rows"][-1]["cum"]
        line = []
        for share in (0.1, 0.25, 0.5, 0.75, 0.9):
            b = 0.0
            for r in o["rows"]:
                if r["cum"] <= share * tot:
                    b = r["sure"]
            line.append("%.0f%%: %.2f" % (100 * share, b))
        print("  %dx%d (%d/%d) s%d  %s" % (o["n"], o["n"], o["cf"], o["ci"], o["seed"], "  ".join(line)))
    print("[%.0fs]" % (time.time() - T0))
