"""gap 4: first-moment model with a minimum over branching choices, real colour census, five clues.
Reuses the independently re-checked model tables (surveys/DERIVATIONS-recheck.scripts/m12_model.py, read only).
Clues are on the board from the start (a forward-checking solver sees them), scan = rows bottom-up.
Shapes: f full rows then up to 4 partial rows of non-increasing lengths (staircase). Row scan = one partial row.
N(S)  = expected number of valid fillings of S.
A(S)  = N(S) * prod over empty cells c touching S of (1 - exp(-mu_c)), mu_c = N(S+c)/N(S): forward-checking survivors.
Run: python3 -I model16.py <official_eternity2.csv>"""
import sys, os
from collections import Counter, defaultdict
from math import log10, log2, exp, log
here = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(here, "..", "..", "surveys", "DERIVATIONS-recheck.scripts"))
from m12_model import interior_match_logp, frame_match_logp, cell_kind, edge_kind, logsum10
N = 16; B = log2(10)
rows = [l.strip() for l in open(sys.argv[1]) if l.strip()]
pieces, clue = [], set()
for l in rows[1:]:
    c = l.split(",")
    pieces.append([0 if int(x, 2) == 65535 else int(x, 2) for x in c[:4]])
    x, y, rot = int(c[4]), int(c[5]), int(c[6])
    if x or y or rot: clue.add((y, x))
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
print("interior half-edge counts per colour:", sorted(ic.values()))
print("frame half-edge counts per colour:", sorted(fc.values()))
lpi = interior_match_logp(list(ic.values()), 420); lpf = frame_match_logp([v // 2 for v in fc.values()], 60)
POOL = {"corner": 4, "edge": 56, "interior": 196 - len(clue)}
KIND = {(r, c): cell_kind(N, r, c) for r in range(N) for c in range(N)}
NB = {(r, c): [(rr, cc) for rr, cc in ((r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)) if 0 <= rr < N and 0 <= cc < N] for r in range(N) for c in range(N)}
EK = {(a, b): edge_kind(N, a[0], a[1], b[0], b[1]) for a in NB for b in NB[a]}

def stats(S):
    """S: set of filled non-clue cells. returns (logN, used dict, ei, ef, present set)."""
    P = S | clue; used = Counter(KIND[c] for c in S); ei = ef = 0
    for a in P:
        for b in NB[a]:
            if b in P and a < b:
                if EK[(a, b)] == "frame": ef += 1
                else: ei += 1
    la = 0.0
    for k, u in used.items():
        if u > POOL[k]: return None
        la += sum(log10(POOL[k] - i) for i in range(u)) + (u * log10(4) if k == "interior" else 0)
    return la + lpi[ei] + lpf[ef], used, ei, ef, P

def mu(st, c):
    _, used, ei, ef, P = st; k = KIND[c]; left = POOL[k] - used[k]
    if left <= 0: return 0.0
    de = df = 0
    for b in NB[c]:
        if b in P:
            if EK[(c, b)] == "frame": df += 1
            else: de += 1
    return 10 ** (log10(left) + (log10(4) if k == "interior" else 0) + lpi[ei + de] - lpi[ei] + lpf[ef + df] - lpf[ef])

def survivors(st):
    P = st[4]; fr = {b for a in P for b in NB[a] if b not in P}; s = 0.0; corners = 0
    for c in fr:
        m = mu(st, c); s += log10(max(1e-300, 1 - exp(-m)))
        if sum(1 for b in NB[c] if b in P) >= 2 or (KIND[c] != "interior"): corners += 1
    return st[0] + s

def shape(f, ls):
    S = set()
    for i in range(f):
        for c in range(N): S.add((N - 1 - i, c))
    for j, l in enumerate(ls):
        for c in range(l): S.add((N - 1 - f - j, c))
    return S - clue

bestA = defaultdict(lambda: (1e9, None)); bestN = defaultdict(lambda: (1e9, None)); rowN = {}; rowA = {}
MAXM = int(sys.argv[2]) if len(sys.argv) > 2 else 4
def rec(f, ls):
    S = shape(f, ls); st = stats(S)
    if st is not None:
        k = len(S); a = survivors(st)
        if a < bestA[k][0]: bestA[k] = (a, (f, tuple(ls)))
        if st[0] < bestN[k][0]: bestN[k] = (st[0], (f, tuple(ls)))
        if len(ls) <= 1: rowN[k] = min(rowN.get(k, 1e9), st[0]); rowA[k] = min(rowA.get(k, 1e9), a)
    if len(ls) < MAXM and f + len(ls) < N:
        top = ls[-1] if ls else N - 1
        for l in range(1, top + 1): rec(f, ls + [l])
for f in range(N + 1): rec(f, [])
ks = sorted(rowN)
tN = logsum10([rowN[k] for k in ks]); tA = logsum10([rowA[k] for k in ks])
mA = logsum10([bestA[k][0] for k in ks]); mN = logsum10([bestN[k][0] for k in ks])
pk = max(ks, key=lambda k: rowN[k])
print("row scan, all valid fillings      : 10^%.2f = 2^%.1f   peak 10^%.2f at %d free cells filled" % (tN, tN * B, rowN[pk], pk))
print("row scan, forward-checking alive  : 10^%.2f = 2^%.1f   (gain %.2f bits)" % (tA, tA * B, (tN - tA) * B))
print("min over staircase shapes of N(S) : 10^%.2f = 2^%.1f   (gain %.2f bits)" % (mN, mN * B, (tN - mN) * B))
print("min over staircase shapes of A(S) : 10^%.2f = 2^%.1f   (gain over row scan %.2f bits, over row scan with FC %.2f bits), up to %d open rows" % (mA, mA * B, (tN - mA) * B, (tA - mA) * B, MAXM))
print("\nwhich shapes win near the peak (k, row-scan log10 N, log10 A row, best log10 A, best shape (full rows, partial lengths)):")
for k in range(pk - 24, pk + 25, 4):
    print("  k=%3d  N_row %.2f  A_row %.2f  A_min %.2f  %s" % (k, rowN[k], rowA[k], bestA[k][0], bestA[k][1]))
# corner trade at the peak: one more open row
f = pk // 16
print("\ncorner trade near the peak (uniform reading): one extra open row costs one interior joint.")
st = stats(shape(f, [8])); c = (N - 1 - f, 8); m2 = mu(st, c)
print("  mu of the next row-scan cell at f=%d, length 8: %.3f ; survival 1-exp(-mu) = %.3f ; sqrt(17) = %.2f ; product %.2f" % (f, m2, 1 - exp(-m2), 17 ** .5, 17 ** .5 * (1 - exp(-m2))))
# tile-centred branching: expected placements of one free interior tile vs the next cell's expected domain
print("\ntile-centred: expected number of places for one free interior tile (lambda) against the row-scan cell's expected domain (mu):")
for fr in (2, 6, 9, 10, 12, 14, 15):
    S = shape(fr, [8]); st = stats(S); P = st[4]; left = POOL["interior"] - st[1]["interior"]
    lam = sum(mu(st, cc) for cc in KIND if cc not in P and KIND[cc] == "interior") / max(1, left)
    lam_e = sum(mu(st, cc) for cc in KIND if cc not in P and KIND[cc] == "edge") / max(1, POOL["edge"] - st[1]["edge"])
    print("  %2d full rows + 8: free interior tiles %3d  lambda interior tile %.2f  lambda edge tile %.2f  mu next cell %.3f" % (fr, left, lam, lam_e, mu(st, (N - 1 - fr, 8))))
# annealed min-domain order: always fill the empty cell with the smallest expected domain (the first-moment image of MRV)
def greedy(rule):
    S = set(); st = stats(S); logs = []
    while len(S) < 256 - len(clue):
        P = st[4]; best = None
        for c in KIND:
            if c in P: continue
            m = mu(st, c)
            key = m if rule == "mrv" else (m, )
            if best is None or m < best[0] or (m == best[0] and c > best[1]): best = (m, c)
        S.add(best[1]); st = stats(S); logs.append(st[0])
    return logsum10(logs), max(logs), logs.index(max(logs)) + 1
t, p, d = greedy("mrv")
print("\nannealed min-domain order (smallest expected domain anywhere, clues on board): total 10^%.2f = 2^%.1f, peak 10^%.2f at depth %d ; against row scan %.2f bits" % (t, t * B, p, d, (tN - t) * B))
