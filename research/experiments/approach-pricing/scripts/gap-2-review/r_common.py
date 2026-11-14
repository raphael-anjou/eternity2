"""Shared loader for the gap-2 review scripts. First-moment model of the derivations second analysis (m12_model.py, read only).
Run any script as: python3 -I <script>.py <official csv>"""
import sys, os
from collections import Counter
from math import log2, lgamma, log
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "..", "surveys", "DERIVATIONS-recheck.scripts"))
from m12_model import interior_match_logp, frame_match_logp, profile
N = 16; B = log2(10)
def lf(n): return lgamma(n + 1) / log(2)
def lch(n, k): return lf(n) - lf(k) - lf(n - k) if 0 <= k <= n else float("-inf")
def lsum(xs):
    m = max(xs); return m + log2(sum(2 ** (x - m) for x in xs))
def load(fn):
    rows = [l.strip() for l in open(fn) if l.strip()]
    pieces, hints = [], []
    for i, l in enumerate(rows[1:]):
        c = l.split(",")
        pieces.append([0 if int(x, 2) == 65535 else int(x, 2) for x in c[:4]])
        x, y, rot = int(c[4]), int(c[5]), int(c[6])
        if x or y or rot: hints.append((y, x, i))
    fc, ic = Counter(), Counter()
    for p in pieces:
        z = [i for i, v in enumerate(p) if v == 0]
        if len(z) == 0:
            for v in p: ic[v] += 1
        elif len(z) == 1:
            i = z[0]; ic[p[(i + 2) % 4]] += 1; fc[p[(i + 1) % 4]] += 1; fc[p[(i + 3) % 4]] += 1
        else:
            for v in p:
                if v: fc[v] += 1
    return pieces, hints, fc, ic
pieces, hints, fc, ic = load(sys.argv[1])
lpi10 = interior_match_logp(list(ic.values()), 420); lpf10 = frame_match_logp([v // 2 for v in fc.values()], 60)
LPI = [x * B for x in lpi10]; LPF = [x * B for x in lpf10]
clue = {(h[0], h[1]) for h in hints}
POOLS = {"corner": 4, "edge": 56, "interior": 196 - len(clue)}
def levels(order, pools=None):
    """log2 expected strict fillings after each cell of `order` (full pools unless given)."""
    return [x[0] * B for x in profile(N, order, dict(pools or POOLS), lpi10, lpf10, clue)]
ROWS_BU = [(r, c) for r in reversed(range(N)) for c in range(N)]
