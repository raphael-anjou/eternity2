"""Second analysis script 1 (gap 1 review). Real pieces, five clues, row scan of Track 10.

Written without importing the memo's e2real.py. It does three things.

  1. Estimates the level widths by population SMC and compares them with the
     committed Track 10 profile (commit dc71ad56).
  2. Recomputes the memo's "share of nodes whose parent's dead subtree has a
     reason of at most B bits" with the memo's own definition of a reason
     (window colours read by the depth-first refutation, union R of blocked tiles).
  3. Tests the assumption M1 rests on: for the same dead parents, shrinks the
     reason greedily. A blocked tile is dropped when the subtree is still dead
     with that tile available again; a window colour is dropped when the subtree
     is still dead with any colour allowed there. Each accepted step must lower
     the bit count. The result is a sound reason, at most as large as the memo's.

Shrinking is tried only for subtrees of at most MINCAP / 2 nodes, each test capped at
MINCAP nodes; larger subtrees keep the memo's reason.

usage: python3 -I r1_real.py NP NS SEED MINCAP d0 d1 step
"""
import json
import random
import subprocess
import sys
import time
from math import log2, log10

REPO = "."
CSV = REPO + "/engine/data/official_eternity2.csv"
REF = "dc71ad56:research/experiments/real-puzzle-profile/results/m2/five-np1e6.jsonl"
N = 16
NP, NS, SEED, MINCAP = (int(x) for x in sys.argv[1:5])
D0, D1, DSTEP = (int(x) for x in sys.argv[5:8])
HMAX, CAP = 32, 60000
BS = (20, 40, 61, 80)
L17, L5 = log2(17), log2(5)


def rotate(e, r):
    t, ri, b, l = e
    return ((t, ri, b, l), (l, t, ri, b), (b, l, t, ri), (ri, b, l, t))[r & 3]


pieces, fixed = [], {}
with open(CSV) as f:
    rows = [l.strip() for l in f if l.strip()]
assert int(rows[0]) == N
for i, l in enumerate(rows[1:]):
    c = l.split(",")
    e = tuple(0 if int(x, 2) == 65535 else int(x, 2) for x in c[:4])
    pieces.append(e)
    x, y, r = int(c[4]), int(c[5]), int(c[6])
    if (x, y, r) != (0, 0, 0):
        fixed[(y, x)] = (i, r)
assert len(pieces) == 256 and len(fixed) == 5
FIXED_EDGES = {rc: rotate(pieces[p], r) for rc, (p, r) in fixed.items()}
FIXED_IDS = {p for p, _ in fixed.values()}
greys = [sum(1 for v in e if v == 0) for e in pieces]
INT_MASK = sum(1 << i for i in range(256) if greys[i] == 0)
INT_POOL = sum(1 for i in range(256) if greys[i] == 0 and i not in FIXED_IDS)
BORDER_POOL = sum(1 for i in range(256) if greys[i] > 0)

ORDER = [(r, c) for r in range(N - 1, -1, -1) for c in range(N) if (r, c) not in fixed]
NFREE = len(ORDER)

# oriented placements by left colour, then by (left, bottom)
BY_LB, BY_L = {}, {}
for t, e in enumerate(pieces):
    if t in FIXED_IDS:
        continue
    seen = set()
    for r in range(4):
        top, right, bottom, left = rotate(e, r)
        if (top, right, bottom, left) in seen:
            continue
        seen.add((top, right, bottom, left))
        BY_LB.setdefault((left, bottom), []).append((t, top, right, 1 << t))
        BY_L.setdefault(left, []).append((t, top, right, 1 << t, bottom))

# per scan position: where left and bottom come from, what top and right must be
LEFT_SRC, BOT_SRC, NEED_TOP, NEED_RIGHT = [], [], [], []
for r, c in ORDER:
    LEFT_SRC.append(0 if c == 0 else FIXED_EDGES[(r, c - 1)][1] if (r, c - 1) in fixed else -1)
    BOT_SRC.append(0 if r == N - 1 else FIXED_EDGES[(r + 1, c)][0] if (r + 1, c) in fixed else -1)
    NEED_TOP.append(0 if r == 0 else FIXED_EDGES[(r - 1, c)][2] if (r - 1, c) in fixed else -1)
    NEED_RIGHT.append(0 if c == N - 1 else FIXED_EDGES[(r, c + 1)][3] if (r, c + 1) in fixed else -1)
COL = [c for _, c in ORDER]
ROW = [r for r, _ in ORDER]
_c1, _c2 = {}, {}


def fits(pos, top, right):
    nt, nr = NEED_TOP[pos], NEED_RIGHT[pos]
    if (top == 0) if nt == -1 else (top != nt):
        return False
    if (right == 0) if nr == -1 else (right != nr):
        return False
    return True


def options(pos, left, bottom):
    key = (NEED_TOP[pos], NEED_RIGHT[pos], left, bottom)
    v = _c1.get(key)
    if v is None:
        v = _c1[key] = tuple(x for x in BY_LB.get((left, bottom), ()) if fits(pos, x[1], x[2]))
    return v


def options_any_bottom(pos, left):
    key = (NEED_TOP[pos], NEED_RIGHT[pos], left)
    v = _c2.get(key)
    if v is None:
        v = _c2[key] = tuple(x[:4] for x in BY_L.get(left, ()) if x[4] != 0 and fits(pos, x[1], x[2]))
    return v


def ctx(pos, front, lastright):
    ls, bs = LEFT_SRC[pos], BOT_SRC[pos]
    return (lastright if ls == -1 else ls), (front[COL[pos]] if bs == -1 else bs)


class Stop(Exception):
    pass


def explore(root, used, front, lastright, wild, cap):
    """Whole subtree under the node about to fill scan position root.
    Returns (height, nodes, blocked tiles). Raises Stop past the caps."""
    cnt = [0]

    def rec(pos, used, lastright):
        cnt[0] += 1
        if cnt[0] > cap or pos - root > HMAX or pos >= NFREE:
            raise Stop
        left, bottom = ctx(pos, front, lastright)
        opts = options_any_bottom(pos, left) if pos in wild else options(pos, left, bottom)
        h, R = 0, 0
        col = COL[pos]
        old = front[col]
        for t, top, right, bit in opts:
            if used & bit:
                R |= bit
                continue
            front[col] = top
            try:
                ch, cR = rec(pos + 1, used | bit, right)
            finally:
                front[col] = old
            R |= cR & ~bit
            if ch + 1 > h:
                h = ch + 1
        return h, R

    h, R = rec(root, used, lastright)
    return h, cnt[0], R


def bits(root, h, R, n_int_used, n_bor_used, wild):
    cb = 0.0
    if LEFT_SRC[root] == -1:
        cb += L5 if ROW[root] in (0, N - 1) else L17
    for q in range(root, min(root + min(h + 1, 16), NFREE)):
        if BOT_SRC[q] == -1 and q not in wild:
            cb += L5 if COL[q] in (0, N - 1) else L17
    ri = bin(R & INT_MASK).count("1")
    rb = bin(R & ~INT_MASK).count("1")
    tb = 0.0
    for j in range(ri):
        tb += log2((INT_POOL - j) / (n_int_used - j))
    for j in range(rb):
        tb += log2((BORDER_POOL - j) / (n_bor_used - j))
    return cb + tb, cb, tb, ri + rb


def shrink(root, used, front, lastright, h, S, R):
    """Greedy sound shrinking of the reason. Returns the smallest bit count found."""
    ui = bin(used & INT_MASK).count("1")
    ub = bin(used & ~INT_MASK).count("1")
    wild = set()
    best = bits(root, h, R, ui, ub, wild)[0]
    cur_used = used
    cap = MINCAP
    for rnd in range(2):
        changed = False
        # tiles
        for t in [i for i in range(256) if R >> i & 1]:
            if not R >> t & 1:
                continue
            try:
                h2, S2, R2 = explore(root, cur_used & ~(1 << t), front, lastright, wild, cap)
            except Stop:
                continue
            b2 = bits(root, h2, R2, ui, ub, wild)[0]
            if b2 < best:
                best, cur_used, h, R, changed = b2, cur_used & ~(1 << t), h2, R2, True
        # window colours, far end first
        for q in range(min(root + min(h + 1, 16), NFREE) - 1, root - 1, -1):
            if BOT_SRC[q] != -1 or q in wild:
                continue
            wild.add(q)
            try:
                h2, S2, R2 = explore(root, cur_used, front, lastright, wild, cap)
                b2 = bits(root, h2, R2, ui, ub, wild)[0]
            except Stop:
                b2 = 1e9
            if b2 < best:
                best, h, R, changed = b2, h2, R2, True
            else:
                wild.discard(q)
        if not changed:
            break
    return best


def reference_widths():
    txt = subprocess.run(["git", "-C", REPO, "show", REF], capture_output=True, text=True, check=True).stdout
    reps = [json.loads(l)["log10_width"] for l in txt.splitlines() if l.strip()]
    out = []
    for d in range(min(len(r) for r in reps)):
        vals = [r[d] for r in reps if r[d] is not None and r[d] > -1e300]
        if not vals:
            break
        out.append(log10(sum(10 ** (v - 30) for v in vals) / len(reps)) + 30 if max(vals) > 30 else log10(sum(10 ** v for v in vals) / len(reps)))
    return out


def main():
    rng = random.Random(SEED)
    ref = reference_widths()
    tot = sum(10 ** (x - 30) for x in ref)
    cum, acc = [], 0.0
    for x in ref:
        acc += 10 ** (x - 30)
        cum.append(acc)
    print("# reference profile: %d levels, total 2^%.2f, widest level %d" % (len(ref), (log10(tot) + 30) * log2(10), max(range(len(ref)), key=lambda d: ref[d])))
    print("# free cut at depth d saves: " + ", ".join("%d: %.1f" % (d, log2(tot / cum[d])) for d in (80, 96, 112, 113, 128, 144, 158)))
    targets = set(range(D0, D1 + 1, DSTEP))
    parts = [(0, (0,) * N, 0, None)] * NP
    logZ, t0 = 0.0, time.time()
    res = {}
    print("depth | log10 width mine, Track 10, diff | n | share with reason <= B, memo definition: " + " ".join("B%d" % b for b in BS) + " | after shrinking: " + " ".join("B%d" % b for b in BS) + " | median bits memo, shrunk | H<=16 share | aborts | s")
    for pos in range(max(targets)):
        new, ws = [], []
        col = COL[pos]
        for used, front, lastright, _ in parts:
            left, bottom = ctx(pos, front, lastright)
            cs = [x for x in options(pos, left, bottom) if not used & x[3]]
            if cs:
                t, top, right, bit = cs[rng.randrange(len(cs))]
                new.append((used | bit, front[:col] + (top,) + front[col + 1:], right, (used, front, lastright)))
                ws.append(len(cs))
        s = sum(ws)
        if not s:
            print("# extinct at", pos)
            break
        logZ += log10(s / len(parts))
        step = s / NP
        u = rng.random() * step
        out, acc = [], 0.0
        for p, w in zip(new, ws):
            acc += w
            while u < acc and len(out) < NP:
                out.append(p)
                u += step
        while len(out) < NP:
            out.append(new[-1])
        parts = out
        d = pos + 1
        if d not in targets:
            continue
        seen, sample = set(), []
        idx = list(range(NP))
        rng.shuffle(idx)
        for i in idx:
            k = parts[i][:3]
            if k in seen:
                continue
            seen.add(k)
            sample.append(parts[i])
            if len(sample) >= NS:
                break
        b_memo, b_min, hs, ab = [], [], [], 0
        for used, front, lastright, par in sample:
            pu, pf, pl = par
            fl = list(pf)
            try:
                h, S, R = explore(d - 1, pu, fl, pl, (), CAP)
            except Stop:
                ab += 1
                b_memo.append(1e9)
                b_min.append(1e9)
                hs.append(1e9)
                continue
            ui = bin(pu & INT_MASK).count("1")
            ub = bin(pu & ~INT_MASK).count("1")
            b = bits(d - 1, h, R, ui, ub, ())[0]
            b_memo.append(b)
            hs.append(h)
            b_min.append(shrink(d - 1, pu, fl, pl, h, S, R) if 0 < S <= MINCAP // 2 else b)
        n = len(sample)
        sm = [sum(1 for b in b_memo if b <= B) / n for B in BS]
        sn = [sum(1 for b in b_min if b <= B) / n for B in BS]
        med = lambda v: sorted(v)[len(v) // 2]
        res[d] = (sm, sn, sum(1 for h in hs if h <= 16) / n)
        print("ROW %d %s %s %.4f" % (d, " ".join("%.4f" % x for x in sm), " ".join("%.4f" % x for x in sn), res[d][2]))
        print("%3d | %.3f %.3f %+.3f | %d | %s | %s | %.0f %.0f | %.3f | %d | %.0f" % (
            d, logZ, ref[d], logZ - ref[d], n, " ".join("%.3f" % x for x in sm), " ".join("%.3f" % x for x in sn),
            med([b for b in b_memo if b < 1e8] or [0]), med([b for b in b_min if b < 1e8] or [0]), res[d][2], ab, time.time() - t0), flush=True)
    ds = sorted(res)
    if DSTEP == 1:
        print()
        print("bits of nodes removed over the whole tree (Track 10 widths as weights; depths below %d untouched, above %d removed)" % (ds[0], ds[-1]))
        for j, B in enumerate(BS):
            for name, k in (("memo definition", 0), ("after shrinking", 1)):
                rem = sum(10 ** (ref[d] - 30) for d in range(ds[0])) + sum(10 ** (ref[d] - 30) * (1 - res[d][k][j]) for d in ds)
                print("   reasons <= %2d bits, %s: %.2f" % (B, name, log2(tot / rem)))
        rem = sum(10 ** (ref[d] - 30) for d in range(ds[0])) + sum(10 ** (ref[d] - 30) * (1 - res[d][2]) for d in ds)
        print("   free oracle, height <= 16: %.2f" % log2(tot / rem))


main()
