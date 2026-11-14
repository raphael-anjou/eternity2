"""Review check 6: spot checks on pages the memo passed, and two controls on
my own scripts.

  - flux-invariants: rank of the 22 x 256 complex flux matrix.
  - ring-purity: in and out degree 12, arborescence count, BEST count.
  - control: the sampler of check 1 on 3x3 with 1,000,000 samples (exact
    value 0.647680), because the first run sat 2.6 standard errors low.
  - control: exact chance that an interior corner cell has no candidate under
    a shuffled border word (all 784 oriented tiles).

Run: uv run --with numpy python 06_spot_checks.py
"""
import os
import sys
import time
from fractions import Fraction
from math import factorial, log10, sqrt

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
CSV = os.path.abspath(os.path.join(HERE, *[".."] * 6, "engine", "data", "official_eternity2.csv"))
T0 = time.time()
pieces = []
for l in [l.strip() for l in open(CSV) if l.strip()][1:]:
    c = l.split(",")
    pieces.append(tuple(0 if int(x, 2) == 65535 else int(x, 2) for x in c[:4]))

print("== flux-invariants ==")
cols = sorted({v for p in pieces for v in p if v})
F = np.zeros((len(cols), len(pieces)), complex)
for j, p in enumerate(pieces):  # top right bottom left
    for s, w in ((1, 1), (3, -1), (2, 1j), (0, -1j)):
        if p[s]:
            F[cols.index(p[s]), j] += w
print("  colours %d ; rank of the complex flux matrix (one column per piece in stored orientation): %d (page: 22)" % (len(cols), np.linalg.matrix_rank(F)))

print("== ring-purity ==")
border = [p for p in pieces if 0 in p]
fc = sorted({v for p in border for k in range(4) if p[k] and (p[(k + 1) % 4] == 0 or p[(k - 1) % 4] == 0) for v in [p[k]]})
A = [[0] * len(fc) for _ in fc]
for p in border:
    # walking clockwise round the ring with grey outside: a piece is entered through the side
    # that follows grey counter-clockwise and left through the side that follows grey clockwise
    ins = [p[k] for k in range(4) if p[k] and p[(k + 1) % 4] == 0]
    outs = [p[k] for k in range(4) if p[k] and p[(k - 1) % 4] == 0]
    assert len(ins) == 1 and len(outs) == 1
    A[fc.index(ins[0])][fc.index(outs[0])] += 1
outd = [sum(r) for r in A]
ind = [sum(A[i][j] for i in range(len(fc))) for j in range(len(fc))]
Lm = [[Fraction((outd[i] if i == j else 0) - A[i][j]) for j in range(1, len(fc))] for i in range(1, len(fc))]
det = Fraction(1)
n = len(Lm)
for i in range(n):
    piv = next(r for r in range(i, n) if Lm[r][i] != 0)
    if piv != i:
        Lm[i], Lm[piv] = Lm[piv], Lm[i]
        det = -det
    det *= Lm[i][i]
    for r in range(i + 1, n):
        f = Lm[r][i] / Lm[i][i]
        for c2 in range(i, n):
            Lm[r][c2] -= f * Lm[i][c2]
best = det * factorial(11) ** 5
print("  frame colours %d ; out degrees %s ; in degrees %s ; self-loops %d ; arborescences %s ; BEST circuits %.4g = 10^%.3f (page: 3,432 and 3.478e41)"
      % (len(fc), outd, ind, sum(A[i][i] for i in range(len(fc))), det, float(best), log10(float(best))))

print("== control: corner cell with no candidate under a shuffled word ==")
letters = [p[(p.index(0) + 2) % 4] for p in pieces if p.count(0) == 1]
pairs = set()
for p in pieces:
    if 0 not in p:
        for k in range(4):
            pairs.add((p[k], p[(k + 1) % 4]))
dead = tot = 0
for i, a in enumerate(letters):
    for j, b in enumerate(letters):
        if i != j:
            tot += 1
            dead += (a, b) not in pairs
pd = dead / tot
print("  one corner: %.4f ; any of four (independent approximation): %.4f = %.2f bits ; my samples gave 0.387 (300 words) and 0.313 (1,500 words)" % (pd, 1 - (1 - pd) ** 4, -np.log2((1 - pd) ** 4)))

print("== control: sampler of check 1 on 3x3, 1,000,000 samples ==")
src = open(os.path.join(HERE, "01_entropy_and_rho.py")).read()
head = src[:src.index('def row_step')]
fn = src[src.index('def sample_rho'):src.index('print("\\n== distinct-piece')]
ns = {"__file__": os.path.join(HERE, "01_entropy_and_rho.py")}
import io, contextlib
with contextlib.redirect_stdout(io.StringIO()):
    exec(head, ns)
exec(fn, ns)
rng = np.random.default_rng(77)
d = 0
S = 250000
for _ in range(4):
    A_, dd, _, _ = ns["sample_rho"](3, 3, S, rng)
    d += dd
rho = d / (4 * S)
print("  rho(3) sampled %.5f (s.e. %.5f) ; exact 0.647680 ; deviation %.1f s.e." % (rho, sqrt(rho * (1 - rho) / (4 * S)), (rho - 0.647680) / sqrt(rho * (1 - rho) / (4 * S))))
d = 0
for _ in range(2):
    A_, dd, _, _ = ns["sample_rho"](4, 4, S, rng)
    d += dd
rho = d / (2 * S)
print("  rho(4) sampled %.5f (s.e. %.5f), second sample ; memo law 0.3283 ; page fit 0.4907" % (rho, sqrt(rho * (1 - rho) / (2 * S))))
print("[%.0fs]" % (time.time() - T0))
