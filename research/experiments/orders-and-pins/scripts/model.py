"""First-moment (bulge) model of a fixed-order backtracker's tree, stdlib only.

Port of `research/program/night/2026-11-12/memos/solve-path-5/bulge.py`
(commit d3d727b4, Owen's complex theory as implemented by McGavin, refereed in
`price-and-ledger.REVIEW.md`), with two changes, both needed for pins:

1. Pre-placed cells may be corners, borders or middles (bulge.py assumed every
   hint is a middle). Pre-placed cells of a class leave the falling factorial
   of that class and, for middles, the 4^k rotation factor.
2. A joint between two pre-placed cells is not counted as a random joint: it
   is satisfied by construction (pins come from a solution). Every other
   joint closed by the order, including a joint between a placed cell and a
   pre-placed one, is a random joint, exactly as in bulge.py for clues.

Everything else (pooled interior census, frame joints as cw against ccw
lateral sides) is bulge.py unchanged. Arithmetic: the coefficient tables are
exact integers, turned into log10 once; widths are summed in floating point
from their log10, which is exact to about 1e-12 relative.

width(state) = expected number of valid partial fills of the occupied cells.
The model's tree size for an order is the sum of widths over the prefixes of
the order (pre-placed cells skipped), i.e. the expected number of nodes.
"""
import math

LOG10_2 = math.log10(2)


def ilog10(n):
    if n <= 0:
        return float("-inf")
    b = n.bit_length()
    if b < 900:
        return math.log10(n)
    s = b - 800
    return math.log10(n >> s) + s * LOG10_2


def perm(n, k):
    r = 1
    for i in range(n - k + 1, n + 1):
        r *= i
    return r


def histograms(pieces):
    """(bt, mt): joints per frame colour and per interior colour, as in bulge.py."""
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


class Model:
    def __init__(self, n, pieces):
        self.n = n
        bt, mt = histograms(pieces)
        self.nbt, self.nmt = sum(bt), sum(mt)
        self.NC, self.NB, self.NM = 4, 4 * (n - 2), (n - 2) ** 2
        vm, vb = _table(mt, True), _table(bt, False)
        # log10 of the joint factors: pm(m) = vm[m] / (2 nmt)_(2m), pb(b) = vb[b] / ((nbt)_b)^2
        self.lpm = [ilog10(vm[m]) - ilog10(perm(2 * self.nmt, 2 * m)) for m in range(len(vm))]
        self.lpb = [ilog10(vb[b]) - 2 * ilog10(perm(self.nbt, b)) for b in range(len(vb))]
        lf = [0.0]
        for i in range(1, n * n + 1):
            lf.append(lf[-1] + math.log10(i))
        self.lf = lf

    def lperm(self, a, k):
        return self.lf[a] - self.lf[a - k]

    def kind(self, r, c):
        return (r in (0, self.n - 1)) + (c in (0, self.n - 1))  # 0 middle, 1 border, 2 corner

    def is_frame_joint(self, r, c, dr):
        # horizontal neighbour (dr == 0): frame joint iff the row is a rim row;
        # vertical neighbour: frame joint iff the column is a rim column.
        return r in (0, self.n - 1) if dr == 0 else c in (0, self.n - 1)

    def profile(self, order, fixed=()):
        """log10 widths after each free cell of `order`. `fixed`: (r, c) cells
        on the board from the start (not nodes). Cells of `order` that are
        fixed are skipped."""
        n = self.n
        occ = [[False] * n for _ in range(n)]
        h = [0, 0, 0]  # pre-placed middles, borders, corners
        for (r, c) in fixed:
            occ[r][c] = True
            h[self.kind(r, c)] += 1
        cnt = [h[0], h[1], h[2]]  # placed middles, borders, corners
        b = m = 0  # random frame / interior joints closed so far
        out = []
        for (r, c) in order:
            if occ[r][c]:
                continue
            for dr, dc in ((0, 1), (0, -1), (1, 0), (-1, 0)):
                rr, cc = r + dr, c + dc
                if 0 <= rr < n and 0 <= cc < n and occ[rr][cc]:
                    if self.is_frame_joint(r, c, dr):
                        b += 1
                    else:
                        m += 1
            occ[r][c] = True
            cnt[self.kind(r, c)] += 1
            lw = (self.lperm(self.NC - h[2], cnt[2] - h[2])
                  + self.lperm(self.NB - h[1], cnt[1] - h[1])
                  + self.lperm(self.NM - h[0], cnt[0] - h[0])
                  + (cnt[0] - h[0]) * math.log10(4)
                  + self.lpm[m] + self.lpb[b])
            out.append(lw)
        return out

    def start(self, fixed=()):
        """Incremental state for `extend`: (occupied set, pre-placed counts,
        placed counts, frame joints, interior joints)."""
        occ = set(fixed)
        h = [0, 0, 0]
        for (r, c) in fixed:
            h[self.kind(r, c)] += 1
        return (frozenset(occ), tuple(h), tuple(h), 0, 0)

    def extend(self, st, cell):
        """(new state, log10 width) after placing `cell`; same arithmetic as
        `profile`."""
        occ, h, cnt, b, m = st
        r, c = cell
        n = self.n
        for dr, dc in ((0, 1), (0, -1), (1, 0), (-1, 0)):
            rr, cc = r + dr, c + dc
            if 0 <= rr < n and 0 <= cc < n and (rr, cc) in occ:
                if self.is_frame_joint(r, c, dr):
                    b += 1
                else:
                    m += 1
        cnt = list(cnt)
        cnt[self.kind(r, c)] += 1
        lw = (self.lperm(self.NC - h[2], cnt[2] - h[2])
              + self.lperm(self.NB - h[1], cnt[1] - h[1])
              + self.lperm(self.NM - h[0], cnt[0] - h[0])
              + (cnt[0] - h[0]) * math.log10(4)
              + self.lpm[m] + self.lpb[b])
        return (occ | {cell}, h, tuple(cnt), b, m), lw

    def total(self, order, fixed=()):
        return logsum(self.profile(order, fixed))


def logsum(lws):
    mx = max(lws)
    return mx + math.log10(sum(10 ** (x - mx) for x in lws))
