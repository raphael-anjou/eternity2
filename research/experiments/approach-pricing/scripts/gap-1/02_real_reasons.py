"""Gap 1, real pieces: how large is the reason for a dead row-scan node, and what would
a store of such reasons cut?

Samples nodes of the real five-clue row-scan tree by SMC (uniform child, systematic
resampling at every level, the estimator of Track 10), gated on the measured level
widths. At chosen free depths, for each sampled node v:

  hp  = height of the subtree under parent(v) (complete exploration, capped at HMAX)
  S   = its size, R = the blocked tiles of its refutation, b = bits of the reason
        (window colours read + blocked tiles), see reason_bits().

v is "saved" by a free perfect oracle of height H iff hp <= H (the parent's dead
subtree is cut at its root, the root is still paid). v is saved by a store of reasons
of at most B bits iff b(parent) <= B.

usage: python3 -I 02_real_reasons.py NP NS SEED [depths...]
"""
import random
import sys
import time
from math import log2, log10

sys.path.insert(0, __import__("os").path.dirname(__file__))
import e2real as E

NP = int(sys.argv[1])
NS = int(sys.argv[2])
SEED = int(sys.argv[3])
DEPTHS = [int(x) for x in sys.argv[4:]] or [96, 112, 128, 136, 144, 152, 158, 164, 170, 176]
HMAX = 32
CAP = 60000
HS = (1, 2, 4, 8, 12, 16, 24, 32)
BS = (20, 30, 40, 50, 61, 80)
cands, context, CELLS, NFREE = E.cands, E.context, E.CELLS, E.NFREE
INTERIOR = [1 if k == 0 else 0 for k in E.KIND]
MASK_INT = sum(1 << t for t, k in enumerate(E.KIND) if k == 0)


class Abort(Exception):
    pass


def explore(pos, used, front, lastright, budget):
    """Complete DFS of the subtree under the node 'about to fill pos'.
    Returns (height, size, R). height 0 = no child. Raises Abort past HMAX or CAP."""
    root = pos
    count = [0]

    def rec(pos, used, lastright):
        count[0] += 1
        if count[0] > budget:
            raise Abort("cap")
        if pos - root > HMAX:
            raise Abort("deep")
        if pos >= NFREE:
            raise Abort("solution")
        left, down = context(pos, front, lastright)
        h, R = 0, 0
        col = CELLS[pos][1]
        old = front[col]
        for t, up, right, bit in cands(pos, left, down):
            if used & bit:
                R |= bit
                continue
            front[col] = up
            try:
                ch, cR = rec(pos + 1, used | bit, right)
            finally:
                front[col] = old
            R |= cR & ~bit
            if ch + 1 > h:
                h = ch + 1
        return h, R

    h, R = rec(pos, used, lastright)
    return h, count[0], R


def reason_bits(pos, h, R, used):
    """Bits of the reason (window, R) for a dead subtree rooted at scan position pos
    with height h. Window: the left colour (if taken from the state) and the frontier
    colours at the h + 1 positions read, at most 16. log2(17) per inner colour,
    log2(5) for a frame colour (columns 0 and 15), 0 for a clue or rim constant.
    Tiles: -log2 P(R subset of used) for a uniform used set of the same size, interior
    and edge tiles separately."""
    cb = 0.0
    ls, _, _, _ = E.SPEC[pos]
    if ls[0] == "s":
        cb += log2(5) if CELLS[pos][0] in (0, 15) else log2(17)
    for q in range(pos, min(pos + min(h + 1, 16), NFREE)):
        if E.SPEC[q][1][0] == "s":
            cb += log2(5) if CELLS[q][1] in (0, 15) else log2(17)
    ri = bin(R & MASK_INT).count("1")
    re = bin(R & ~MASK_INT).count("1")
    ui = bin(used & MASK_INT).count("1")
    ue = bin(used & ~MASK_INT).count("1")
    tb = 0.0
    for j in range(ri):
        tb += log2((E.INTERIOR_POOL - j) / (ui - j))
    for j in range(re):
        tb += log2((E.EDGE_POOL + 4 - j) / (ue - j))
    return cb, tb, ri + re


def main():
    rng = random.Random(SEED)
    meas = E.measured_widths()
    # particle: (used, front tuple, lastright, parent state or None)
    parts = [(0, (0,) * 16, 0, None)] * NP
    logZ = 0.0
    t0 = time.time()
    print("# NP %d NS %d seed %d HMAX %d CAP %d" % (NP, NS, SEED, HMAX, CAP))
    print("# gate: depth, log10 width here, measured (Track 10, N = 1e6), difference")
    targets = set(DEPTHS)
    rows = []
    for pos in range(max(DEPTHS)):
        new, ws = [], []
        col = CELLS[pos][1]
        for st in parts:
            used, front, lastright, _ = st
            left, down = context(pos, front, lastright)
            cs = [x for x in cands(pos, left, down) if not used & x[3]]
            if cs:
                t, up, right, bit = cs[rng.randrange(len(cs))]
                new.append((used | bit, front[:col] + (up,) + front[col + 1:], right, (used, front, lastright)))
                ws.append(len(cs))
        tot = sum(ws)
        if not tot:
            print("# extinct at", pos)
            break
        logZ += log10(tot / len(parts))
        stepw = tot / NP
        u = rng.random() * stepw
        res, acc = [], 0.0
        for p, w in zip(new, ws):
            acc += w
            while u < acc and len(res) < NP:
                res.append(p)
                u += stepw
        while len(res) < NP:
            res.append(new[-1])
        parts = res
        depth = pos + 1
        if depth in targets:
            print("# gate %3d  %.3f  %.3f  %+.3f   (%.0f s)" % (depth, logZ, meas[depth], logZ - meas[depth], time.time() - t0), flush=True)
            rows.append(analyse(depth, parts, rng))
    print()
    print("depth n distinct | mean children of v, zero-child share | share saved by a free oracle of height H: " + " ".join("H%d" % h for h in HS))
    for r in rows:
        print(r["line1"])
    print()
    print("depth | share saved by a store of reasons of at most B bits: " + " ".join("B%d" % b for b in BS) + " | dead parents within HMAX: n, median S, median h, median |R|, median b, bits per subtree node (least squares through origin of tile bits on S)")
    for r in rows:
        print(r["line2"])
    print()
    print("# rows for 01_ceilings.py: depth, then remaining share 1 - s for each H, then for each B")
    for r in rows:
        print("ROW %d %s %s" % (r["depth"], " ".join("%.4f" % (1 - x) for x in r["sH"]), " ".join("%.4f" % (1 - x) for x in r["sB"])))


def analyse(depth, parts, rng):
    seen = set()
    sample = []
    idx = list(range(len(parts)))
    rng.shuffle(idx)
    for i in idx:
        st = parts[i]
        key = (st[0], st[1], st[2])
        if key in seen:
            continue
        seen.add(key)
        sample.append(st)
        if len(sample) >= NS:
            break
    pos = depth  # v is "about to fill" scan position depth; its parent fills depth - 1
    nch, zero = 0, 0
    hps, recs = [], []
    aborted = {"cap": 0, "deep": 0, "solution": 0}
    for used, front, lastright, par in sample:
        left, down = context(pos, front, lastright) if pos < NFREE else (0, 0)
        k = sum(1 for x in cands(pos, left, down) if not used & x[3])
        nch += k
        zero += k == 0
        pu, pf, pl = par
        try:
            h, S, R = explore(pos - 1, pu, list(pf), pl, CAP)
            cb, tb, r = reason_bits(pos - 1, h, R, pu)
            hps.append(h)
            recs.append((S, h, r, cb, tb))
        except Abort as a:
            aborted[str(a)] += 1
            hps.append(10 ** 9)
            recs.append(None)
    n = len(sample)
    sH = [sum(1 for h in hps if h <= H) / n for H in HS]
    sB = [sum(1 for x in recs if x and x[3] + x[4] <= B) / n for B in BS]
    dead = [x for x in recs if x]
    med = lambda v: sorted(v)[len(v) // 2] if v else float("nan")
    sxx = sum(x[0] * x[0] for x in dead) or 1
    slope = sum(x[0] * x[4] for x in dead) / sxx
    line1 = "%3d %5d %5d | %.3f %.3f | %s | aborts %s" % (depth, n, len(seen), nch / n, zero / n, " ".join("%.3f" % x for x in sH), aborted)
    line2 = "%3d | %s | %d, %s, %s, %s, %.1f, %.2f" % (depth, " ".join("%.3f" % x for x in sB), len(dead), med([x[0] for x in dead]), med([x[1] for x in dead]), med([x[2] for x in dead]), med([x[3] + x[4] for x in dead]), slope)
    # detail: reason bits by subtree size class
    cls = {}
    for S, h, r, cb, tb in dead:
        c = 1 if S <= 1 else 2 if S <= 3 else 3 if S <= 10 else 4 if S <= 30 else 5 if S <= 100 else 6
        cls.setdefault(c, []).append((S, h, r, cb, tb))
    names = {1: "S=1", 2: "S 2-3", 3: "S 4-10", 4: "S 11-30", 5: "S 31-100", 6: "S >100"}
    for c in sorted(cls):
        v = cls[c]
        print("#   depth %d %-9s n %4d  mean S %.1f  mean h %.1f  mean |R| %.1f  mean colour bits %.1f  mean tile bits %.1f" % (
            depth, names[c], len(v), sum(x[0] for x in v) / len(v), sum(x[1] for x in v) / len(v), sum(x[2] for x in v) / len(v),
            sum(x[3] for x in v) / len(v), sum(x[4] for x in v) / len(v)), flush=True)
    return {"depth": depth, "sH": sH, "sB": sB, "line1": line1, "line2": line2}


main()
