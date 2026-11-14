"""Referee's own first-moment model and exact enumerator. Stdlib only.

Model (fixed-census ensemble). The coloured half-edges of a piece set are
split into classes; inside a class the colours are a uniformly random
arrangement of the class's colour multiset over its slots. A partial board is
a choice of distinct pieces and rotations for some cells; it is valid when
every joint between two placed cells is monochromatic. E[number of valid
partial boards] = (ordered piece choices) x 4^(free interior cells) x
P(all those joints match). Because slots are exchangeable inside a class, P
depends only on how many joints of each kind are closed.

Two censuses:
  pooled      interior class = sides of interior pieces + inward sides of edge
              pieces (840 slots on the official set); frame class = clockwise
              lateral sides against counter-clockwise lateral sides.
              P = P_int(e) * P_frame(f).  (Owen's complex theory.)
  class-aware inward sides of edge pieces are their own class, so a seam joint
              (edge piece to interior piece) differs from an inner joint.
              P = P2(s, e) * P_frame(f).

Derivation of P_int(e), written out so it can be checked: choose a colour for
each of the e joints; k_c joints get colour c in e!/prod k_c! ways; the 2e
slots carry those colours with probability prod (n_c)_(2k_c) / (H)_(2e).
So P_int(e) = e!/(H)_(2e) * [y^e] prod_c sum_k C(n_c,2k) (2k)!/k! y^k.
P_frame(f) = f!/((M)_f)^2 * [x^f] prod_c sum_k C(m_c,k)^2 k! x^k.
P2(s,e)   = s! e! /((S)_s (I)_(s+2e)) * [x^s y^e] prod_c sum_{j,k}
            C(S_c,j) (I_c)_(j+2k)/k! x^j y^k.
"""
import math
import os
from math import comb, factorial, log10

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 6))
CSV = os.path.join(REPO, "engine", "data", "official_eternity2.csv")


def ilog10(n):
    """log10 of a positive int of any size."""
    b = n.bit_length()
    if b < 900:
        return log10(n)
    s = b - 800
    return log10(n >> s) + s * log10(2)


def falling(n, k):
    r = 1
    for i in range(k):
        r *= n - i
    return r


def load_official():
    lines = [l.strip() for l in open(CSV) if l.strip()]
    pieces, hints = [], []
    for i, l in enumerate(lines[1:]):
        c = l.split(",")
        pieces.append(tuple(0 if int(x, 2) == 65535 else int(x, 2) for x in c[:4]))
        if int(c[4]) or int(c[5]) or int(c[6]):
            hints.append((int(c[5]), int(c[4])))  # (row, col)
    return pieces, hints


def census(pieces):
    """Returns dict with per-colour counts: inner (sides of interior pieces),
    seam (inward sides of edge pieces), cw / ccw (lateral frame sides)."""
    inner, seam, cw, ccw = {}, {}, {}, {}
    kinds = [0, 0, 0]
    for p in pieces:
        g = sum(1 for v in p if v == 0)
        kinds[g] += 1
        for i in range(4):
            v = p[i]
            if v == 0:
                continue
            if g == 0:
                d = inner
            elif p[(i + 1) % 4] == 0:
                d = cw
            elif p[(i - 1) % 4] == 0:
                d = ccw
            else:
                d = seam
            d[v] = d.get(v, 0) + 1
    assert cw == ccw, "frame colours are not balanced between the two hands"
    return {"inner": inner, "seam": seam, "frame": cw, "kinds": kinds}


def p_int_table(counts):
    """log10 P_int(e), e = 0..H/2, exact integer arithmetic."""
    H = sum(counts)
    poly = [1]
    for n in counts:
        g = [comb(n, 2 * k) * factorial(2 * k) // factorial(k) for k in range(n // 2 + 1)]
        new = [0] * (len(poly) + len(g) - 1)
        for i, a in enumerate(poly):
            if a:
                for j, b in enumerate(g):
                    new[i + j] += a * b
        poly = new
    out = []
    for e in range(H // 2 + 1):
        num = poly[e] * factorial(e) if e < len(poly) else 0
        out.append(ilog10(num) - ilog10(falling(H, 2 * e)) if num else float("-inf"))
    return out


def p_frame_table(counts):
    M = sum(counts)
    poly = [1]
    for m in counts:
        g = [comb(m, k) ** 2 * factorial(k) for k in range(m + 1)]
        new = [0] * (len(poly) + len(g) - 1)
        for i, a in enumerate(poly):
            if a:
                for j, b in enumerate(g):
                    new[i + j] += a * b
        poly = new
    out = []
    for f in range(M + 1):
        num = poly[f] * factorial(f)
        out.append(ilog10(num) - 2 * ilog10(falling(M, f)) if num else float("-inf"))
    return out


def p2_table(seam, inner):
    """log10 P2(s, e) as a list of lists [s][e]; exact integers, the y
    dimension packed into one big int per x power (Kronecker substitution)."""
    cols = sorted(set(seam) | set(inner))
    S = sum(seam.values())
    I = sum(inner.values())
    emax = (I - S) // 2
    # bound on coefficient size: every coefficient is below (S+I)! * 2^S
    B = ilog10(factorial(S + I)) / log10(2) + S + 64
    B = int(B) + 1
    mask = (1 << B) - 1
    rows = [1]  # rows[j] = packed polynomial in y
    for c in cols:
        sc, ic = seam.get(c, 0), inner.get(c, 0)
        fac = []
        for j in range(sc + 1):
            packed = 0
            for k in range((ic - j) // 2 + 1):
                packed |= (comb(sc, j) * falling(ic, j + 2 * k) // factorial(k)) << (B * k)
            fac.append(packed)
        new = [0] * (len(rows) + len(fac) - 1)
        trunc = (1 << (B * (emax + 1))) - 1
        for i, a in enumerate(rows):
            if a:
                for j, b in enumerate(fac):
                    new[i + j] += (a * b) & trunc
        rows = [r & trunc for r in new]
    out = []
    for s in range(S + 1):
        r = rows[s] if s < len(rows) else 0
        line = []
        fs = factorial(s)
        ds = falling(S, s)
        for e in range(emax + 1):
            coef = (r >> (B * e)) & mask
            if coef == 0 or s + 2 * e > I:
                line.append(float("-inf"))
            else:
                line.append(ilog10(coef * fs * factorial(e)) - ilog10(ds * falling(I, s + 2 * e)))
        out.append(line)
    return out


def kind(n, r, c):
    return (r in (0, n - 1)) + (c in (0, n - 1))  # 0 interior, 1 edge, 2 corner


def joint_kind(n, a, b):
    ka, kb = kind(n, *a), kind(n, *b)
    if ka == 0 and kb == 0:
        return "inner"
    if ka == 0 or kb == 0:
        return "seam"
    return "frame"


class Model:
    def __init__(self, n, cen):
        self.n = n
        self.cen = cen
        pooled = dict(cen["inner"])
        for k, v in cen["seam"].items():
            pooled[k] = pooled.get(k, 0) + v
        self.lp_int = p_int_table(list(pooled.values()))
        self.lp_fr = p_frame_table(list(cen["frame"].values()))
        self._p2 = None
        self.pools = {0: (n - 2) ** 2, 1: 4 * (n - 2), 2: 4}

    def p2(self):
        if self._p2 is None:
            self._p2 = p2_table(self.cen["seam"], self.cen["inner"])
        return self._p2

    def profile(self, order, hints=(), mode="pooled", pools=None, count_hint_levels=False, defer=False):
        """log10 expected number of valid partial boards after each cell of
        `order`. Hint cells are on the board from the start (one piece, one
        rotation) and produce no level unless count_hint_levels is set.
        mode: pooled | class | spread-pooled | spread-class."""
        n = self.n
        hs = set(hints)
        left = dict(pools) if pools else dict(self.pools)
        if not pools:
            left[0] -= len(hs)
        placed = set() if defer else set(hs)  # defer: path 3's convention, see 03_ledger.py
        s = e = f = 0
        for a in hs:
            for b in hs:
                assert abs(a[0] - b[0]) + abs(a[1] - b[1]) != 1
        la = 0.0
        out = []
        if mode.startswith("spread"):
            if mode == "spread-pooled":
                qe = self.lp_int[-1] / (len(self.lp_int) - 1)
                qs = qe
            else:
                p2 = self.p2()
                S = len(p2) - 1
                E = len(p2[0]) - 1
                # path 2's split: seam factor and inner factor of the full board
                seam, inner = self.cen["seam"], self.cen["inner"]
                ls = sum(ilog10(falling(inner.get(c, 0), seam[c])) for c in seam) - ilog10(falling(sum(inner.values()), S))
                qs = ls / S
                qe = (p2[S][E] - ls) / E
            qf = self.lp_fr[-1] / (len(self.lp_fr) - 1)
        for cell in order:
            r, c = cell
            if cell in hs and not defer:
                if count_hint_levels:
                    out.append(out[-1] if out else 0.0)
                continue
            if cell in hs:  # deferred hint: its joints are closed only now, no piece choice
                for nb in ((r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)):
                    if nb in placed:
                        jk = joint_kind(n, cell, nb)
                        if jk == "inner":
                            e += 1
                        elif jk == "seam":
                            s += 1
                        else:
                            f += 1
                placed.add(cell)
                out.append(la + self.lp_int[e + s] + self.lp_fr[f])
                continue
            k = kind(n, r, c)
            if left[k] <= 0:
                la = float("-inf")
            else:
                la += log10(left[k]) + (log10(4) if k == 0 else 0.0)
            left[k] -= 1
            for nb in ((r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)):
                if nb in placed:
                    jk = joint_kind(n, cell, nb)
                    if jk == "inner":
                        e += 1
                    elif jk == "seam":
                        s += 1
                    else:
                        f += 1
            placed.add(cell)
            if mode == "pooled":
                lp = self.lp_int[e + s] + self.lp_fr[f]
            elif mode == "class":
                lp = self.p2()[s][e] + self.lp_fr[f]
            else:
                lp = qe * e + qs * s + qf * f
            out.append(la + lp)
        return out


def logsum(xs):
    m = max(xs)
    return m + log10(sum(10 ** (x - m) for x in xs))


def rows_from(n, top=True):
    rs = range(n) if top else range(n - 1, -1, -1)
    return [(r, c) for r in rs for c in range(n)]


def ring(n):
    """Frame cells in ring order starting at (0,0), so each has a placed neighbour."""
    o = [(0, c) for c in range(n)] + [(r, n - 1) for r in range(1, n)]
    o += [(n - 1, c) for c in range(n - 2, -1, -1)] + [(r, 0) for r in range(n - 2, 0, -1)]
    return o


# ---------------------------------------------------------------- enumeration

def rotations(p):
    return [p[4 - k:] + p[:4 - k] if k else p for k in range(4)]


class Enum:
    """Exact depth-first enumeration of valid partial boards along any cell
    order. A node is one (cell, piece, rotation) that has grey exactly on the
    rim and agrees with every already-placed neighbour."""

    def __init__(self, n, pieces):
        self.n = n
        self.pieces = [tuple(p) for p in pieces]
        self.allrot = []
        seen = set()
        for i, p in enumerate(self.pieces):
            for q in rotations(p):  # all four turns, as the model counts them (4^k)
                self.allrot.append((i, q))
        self.cache = {}

    def cands(self, key):
        v = self.cache.get(key)
        if v is None:
            v = []
            for i, q in self.allrot:
                ok = True
                for s in range(4):
                    w = key[s]
                    if w == -1:
                        if q[s] == 0:
                            ok = False
                            break
                    elif q[s] != w:
                        ok = False
                        break
                if ok:
                    v.append((i, q))
            self.cache[key] = v
        return v

    def run(self, order, preset=None, allowed=None, cap=None, watch=None, target=None):
        """Returns per-depth node counts (index d = d cells placed).
        preset: dict cell -> (piece index, oriented tuple), already on the board.
        watch: depth at which each node's subtree size is recorded (list returned).
        target: depth D; then also returns, per depth k, (sum c, sum c^2) of the
        number c of depth-D descendants over the depth-k nodes."""
        n = self.n
        N = len(order)
        board = {}
        used = [False] * len(self.pieces)
        if allowed is not None:
            for i in range(len(used)):
                used[i] = i not in allowed
        if preset:
            for cell, (i, q) in preset.items():
                board[cell] = q
                used[i] = True
        per = [0] * (N + 1)
        nodes = [0]
        sub = []
        s1 = [0] * (N + 1)
        s2 = [0] * (N + 1)
        OPP = (2, 3, 0, 1)
        NB = ((-1, 0), (0, 1), (1, 0), (0, -1))  # top right bottom left

        def go(d):
            """returns (subtree nodes below this node, descendants at target)"""
            if d == N:
                return 0, 0
            r, c = order[d]
            key = []
            for s in range(4):
                rr, cc = r + NB[s][0], c + NB[s][1]
                if rr < 0 or rr >= n or cc < 0 or cc >= n:
                    key.append(0)
                else:
                    q = board.get((rr, cc))
                    key.append(q[OPP[s]] if q is not None else -1)
            tot = 0
            desc = 0
            for i, q in self.cands(tuple(key)):
                if used[i]:
                    continue
                per[d + 1] += 1
                nodes[0] += 1
                if cap and nodes[0] > cap:
                    raise OverflowError
                used[i] = True
                board[(r, c)] = q
                t, ds = go(d + 1)
                used[i] = False
                del board[(r, c)]
                if target is not None:
                    if d + 1 == target:
                        ds = 1
                    if d + 1 <= target:
                        s1[d + 1] += ds
                        s2[d + 1] += ds * ds
                    desc += ds
                if watch is not None and d + 1 == watch:
                    sub.append(t)
                tot += 1 + t
            return tot, desc

        import sys
        sys.setrecursionlimit(10000)
        go(0)
        res = {"per": per}
        if watch is not None:
            res["sub"] = sub
        if target is not None:
            res["s1"], res["s2"] = s1, s2
        return res


def gen_planted(n, cf, ci, rng, balanced=False):
    """Planted framed board: cf colours on frame joints, ci on the others.
    Returns (pieces shuffled and rotated, solution dict cell -> (index, oriented))."""
    fr = [(r, c, "h") for r in (0, n - 1) for c in range(n - 1)] + [(r, c, "v") for c in (0, n - 1) for r in range(n - 1)]
    it = [(r, c, "h") for r in range(1, n - 1) for c in range(n - 1)] + [(r, c, "v") for c in range(1, n - 1) for r in range(n - 1)]
    col = {}
    for js, k, off in ((fr, cf, 0), (it, ci, cf)):
        if balanced:
            bag = [off + 1 + (i % k) for i in range(len(js))]
            rng.shuffle(bag)
        else:
            bag = [off + rng.randint(1, k) for _ in js]
        for j, v in zip(js, bag):
            col[j] = v
    cells = []
    for r in range(n):
        for c in range(n):
            t = col[(r - 1, c, "v")] if r else 0
            b = col[(r, c, "v")] if r < n - 1 else 0
            l = col[(r, c - 1, "h")] if c else 0
            ri = col[(r, c, "h")] if c < n - 1 else 0
            cells.append(((r, c), (t, ri, b, l)))
    idx = list(range(n * n))
    rng.shuffle(idx)
    pieces = [None] * (n * n)
    sol = {}
    for (cell, q), i in zip(cells, idx):
        k = rng.randrange(4)
        pieces[i] = q[k:] + q[:k]
        sol[cell] = (i, q)
    return pieces, sol


def shuffle_census(pieces, rng):
    """A fresh draw from the pooled ensemble with the same census: colours
    permuted inside the interior class and inside each frame hand."""
    slots = {"int": [], "cw": [], "ccw": []}
    for pi, p in enumerate(pieces):
        g = sum(1 for v in p if v == 0)
        for i in range(4):
            if p[i] == 0:
                continue
            if g == 0:
                k = "int"
            elif p[(i + 1) % 4] == 0:
                k = "cw"
            elif p[(i - 1) % 4] == 0:
                k = "ccw"
            else:
                k = "int"
            slots[k].append((pi, i))
    new = [list(p) for p in pieces]
    for k, sl in slots.items():
        vals = [pieces[pi][i] for pi, i in sl]
        rng.shuffle(vals)
        for (pi, i), v in zip(sl, vals):
            new[pi][i] = v
    return [tuple(p) for p in new]
