"""Bulge model (Owen's complex theory, ported from McGavin's reference C at
research/topics/complex-theory/reference/complex_theory.c), exact integer
arithmetic, stdlib only. Run with python3 -I.

width(state) = P * pm2 * pb2: expected number of valid partial fills of the
occupied cells, given the colour histogram of the piece set.
"""
import math, os
from fractions import Fraction
from functools import lru_cache

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), *[".."] * 6))
CSV = os.path.join(ROOT, "engine", "data", "official_eternity2.csv")


def perm(n, k):
    r = 1
    for i in range(n - k + 1, n + 1):
        r *= i
    return r


def load_official():
    lines = [l.strip() for l in open(CSV) if l.strip()]
    n = int(lines[0])
    pieces, hints = [], []
    for i, l in enumerate(lines[1:]):
        c = l.split(",")
        pieces.append(tuple(0 if int(x, 2) == 65535 else int(x, 2) for x in c[:4]))
        if len(c) >= 7 and (int(c[4]) or int(c[5]) or int(c[6])):
            hints.append((int(c[5]), int(c[4]), i, int(c[6])))  # row, col, piece, rot
    return n, pieces, hints


def histograms(pieces):
    """(bt, mt): joins per border colour and per middle colour (half the edge
    counts). A border edge is a lateral side of a border or corner piece."""
    lat, mid = {}, {}
    for p in pieces:
        g = [i for i in range(4) if p[i] == 0]
        if not g:
            for c in p:
                mid[c] = mid.get(c, 0) + 1
        elif len(g) == 1:
            k = g[0]
            mid[p[(k + 2) % 4]] = mid.get(p[(k + 2) % 4], 0) + 1
            for d in (1, 3):
                lat[p[(k + d) % 4]] = lat.get(p[(k + d) % 4], 0) + 1
        else:
            for c in p:
                if c:
                    lat[c] = lat.get(c, 0) + 1
    assert all(v % 2 == 0 for v in lat.values()) and all(v % 2 == 0 for v in mid.values())
    return sorted(v // 2 for v in lat.values()), sorted(v // 2 for v in mid.values())


class Model:
    def __init__(self, rows, cols, bt, mt, n_mid_hints=0):
        self.R, self.C = rows, cols
        self.bt, self.mt = list(bt), list(mt)
        self.nbt, self.nmt = sum(bt), sum(mt)
        self.NC, self.NB, self.NM = 4, 2 * (rows - 2) + 2 * (cols - 2), (rows - 2) * (cols - 2)
        self.h = n_mid_hints
        self._vm = self._table(self.mt, middle=True)
        self._vb = self._table(self.bt, middle=False)
        self._cache = {}

    @staticmethod
    def _table(n, middle):
        tot = sum(n)
        cur = [1] + [0] * tot
        for ni in n:
            nxt = [0] * (tot + 1)
            for m in range(tot + 1):
                s = 0
                for j in range(0, min(ni, m) + 1):
                    if cur[m - j]:
                        w = perm(2 * ni, 2 * j) if middle else perm(ni, j) ** 2
                        s += cur[m - j] * w * math.comb(m, j)
                nxt[m] = s
            cur = nxt
        return cur

    def width(self, nc, nb, nm, b, m):
        """Exact Fraction. nm includes the pre-placed middle hints."""
        key = (nc, nb, nm, b, m)
        v = self._cache.get(key)
        if v is None:
            p = perm(self.NC, nc) * perm(self.NB, nb) * perm(self.NM - self.h, nm - self.h) * 4 ** (nm - self.h)
            v = Fraction(p * self._vm[m] * self._vb[b], perm(2 * self.nmt, 2 * m) * perm(self.nbt, b) ** 2)
            self._cache[key] = v
        return v

    def kind(self, r, c):
        e = (r in (0, self.R - 1)) + (c in (0, self.C - 1))
        return e  # 0 middle, 1 border, 2 corner

    def joins_added(self, occ, r, c):
        b = m = 0
        for dr, dc in ((0, 1), (0, -1), (1, 0), (-1, 0)):
            rr, cc = r + dr, c + dc
            if 0 <= rr < self.R and 0 <= cc < self.C and (rr, cc) in occ:
                if dr == 0:
                    border = r in (0, self.R - 1)
                else:
                    border = c in (0, self.C - 1)
                if border:
                    b += 1
                else:
                    m += 1
        return b, m

    def profile(self, order, hints=()):
        """Widths after each placed cell of `order` (hint cells are pre-placed
        and skipped). Returns list of Fractions, one per non-hint cell."""
        occ, st = set(), [0, 0, 0, 0, 0]
        for (r, c) in hints:
            self._add(occ, st, r, c)
        out = []
        for (r, c) in order:
            if (r, c) in occ:
                continue
            self._add(occ, st, r, c)
            out.append(self.width(*st))
        return out

    def _add(self, occ, st, r, c):
        b, m = self.joins_added(occ, r, c)
        occ.add((r, c))
        k = self.kind(r, c)
        if k == 2:
            st[0] += 1
        elif k == 1:
            st[1] += 1
        else:
            st[2] += 1
        st[3] += b
        st[4] += m

    def greedy(self, hints=(), start=None, lookahead_tiebreak=True):
        """Order built by always adding the cell that minimises the next width."""
        occ, st = set(), [0, 0, 0, 0, 0]
        for (r, c) in hints:
            self._add(occ, st, r, c)
        order = []
        cells = [(r, c) for r in range(self.R) for c in range(self.C) if (r, c) not in occ]
        if start is not None:
            self._add(occ, st, *start)
            order.append(start)
            cells.remove(start)
        while cells:
            best, bw = None, None
            for (r, c) in cells:
                b, m = self.joins_added(occ, r, c)
                k = self.kind(r, c)
                s = (st[0] + (k == 2), st[1] + (k == 1), st[2] + (k == 0), st[3] + b, st[4] + m)
                w = self.width(*s)
                if bw is None or w < bw:
                    best, bw = (r, c), w
            self._add(occ, st, *best)
            order.append(best)
            cells.remove(best)
        return order


def L(x):
    """log10 of a positive Fraction or int."""
    if isinstance(x, Fraction):
        return math.log10(x.numerator) - math.log10(x.denominator)
    return math.log10(x)


def rowscan(R, C, from_bottom=True, right_to_left=False):
    rows = range(R - 1, -1, -1) if from_bottom else range(R)
    cols = range(C - 1, -1, -1) if right_to_left else range(C)
    return [(r, c) for r in rows for c in cols]


def colscan(R, C, from_left=True):
    cols = range(C) if from_left else range(C - 1, -1, -1)
    return [(r, c) for c in cols for r in range(R - 1, -1, -1)]
