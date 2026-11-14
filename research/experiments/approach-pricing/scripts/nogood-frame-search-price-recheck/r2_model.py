"""Second analysis, claims 3 and 5. Own pooled first-moment model (not rv_lib).
python3 -I r2_model.py"""
from math import comb, factorial, log10, log2
from collections import Counter
CSV = "engine/data/official_eternity2.csv"
L10 = log2(10)
T_MEAS = 40.457 * L10
EXACT_FRAMES = 40519056449429419697492635191544381440
lines = [l.strip() for l in open(CSV) if l.strip()][1:]
P = []; hints = set()
for l in lines:
    c = l.split(",")
    P.append(tuple(0 if int(x, 2) == 65535 else int(x, 2) for x in c[:4]))
    if int(c[4]) or int(c[5]) or int(c[6]):
        hints.add((int(c[5]), int(c[4])))
# census: pooled interior class (all sides of interior pieces + inward sides of edge pieces); frame hands
pool = Counter(); cw = Counter(); ccw = Counter()
for p in P:
    g = p.count(0)
    for i in range(4):
        v = p[i]
        if v == 0: continue
        if g == 0: pool[v] += 1
        elif p[(i + 1) % 4] == 0: cw[v] += 1
        elif p[(i - 1) % 4] == 0: ccw[v] += 1
        else: pool[v] += 1
assert cw == ccw and sum(pool.values()) == 196 * 4 + 56 and sum(cw.values()) == 60

def lg(n):
    b = n.bit_length()
    if b < 900: return log10(n)
    return log10(n >> (b - 800)) + (b - 800) * log10(2)
def fall(n, k):
    r = 1
    for i in range(k): r *= n - i
    return r
def polymul(a, b):
    out = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        if x:
            for j, y in enumerate(b):
                out[i + j] += x * y
    return out
# P_int(e): e joints, each a pair of slots from the pooled class
H = sum(pool.values())
poly = [1]
for n in pool.values():
    poly = polymul(poly, [fall(n, 2 * k) // factorial(k) for k in range(n // 2 + 1)])
LPI = [lg(poly[e] * factorial(e)) - lg(fall(H, 2 * e)) for e in range(H // 2 + 1) if e < len(poly) and poly[e]]
M = sum(cw.values())
poly = [1]
for m in cw.values():
    poly = polymul(poly, [fall(m, k) ** 2 // factorial(k) for k in range(m + 1)])
LPF = [lg(poly[f] * factorial(f)) - 2 * lg(fall(M, f)) for f in range(M + 1)]

def kind(c):
    return (c[0] in (0, 15)) + (c[1] in (0, 15))
def profile(order, preset=()):
    """log10 E[# valid partial boards] after each free cell of order. preset cells are on the board, one piece one turn."""
    placed = set(preset)
    left = {0: 196 - sum(1 for c in preset if kind(c) == 0), 1: 56, 2: 4}
    la = 0.0; e = f = 0; out = []
    for cell in order:
        if cell in placed: continue
        k = kind(cell)
        la += log10(left[k]) + (log10(4) if k == 0 else 0.0)
        left[k] -= 1
        for d in ((-1, 0), (1, 0), (0, -1), (0, 1)):
            nb = (cell[0] + d[0], cell[1] + d[1])
            if nb in placed:
                if kind(cell) > 0 and kind(nb) > 0: f += 1
                else: e += 1
        placed.add(cell)
        out.append(la + LPI[e] + LPF[f])
    return out
def lsum(xs):
    m = max(xs)
    return m + log10(sum(10 ** (x - m) for x in xs))

for start, name in ((15, "rows 15 to 0 (Track 10)"), (0, "rows 0 to 15 (the other side)")):
    rows = range(15, -1, -1) if start == 15 else range(16)
    order = [(r, c) for r in rows for c in range(16)]
    W = profile(order, hints)
    free = [c for c in order if c not in hints]
    tot = lsum(W)
    print("row scan, %s, five clues: total 10^%.3f = 2^%.2f, widest free depth %d" % (name, tot, tot * L10, W.index(max(W)) + 1))
order = [(r, c) for r in range(15, -1, -1) for c in range(16)]
W = profile(order, hints); free = [c for c in order if c not in hints]; tot = lsum(W)
border = [c for c in free if kind(c) > 0]
NF = profile(border)
print("frames, model 2^%.2f, exact 2^%.2f" % (NF[-1] * L10, log2(EXACT_FRAMES)))
PB = log2(2669 / 57131 * 2508 / 57131)
print("\n k  arc  free depth  beta_k  log10 mu_k  share after row k  | budget log2 C (=134.40-beta_k+8.93)  core-s at 1e8 | walk: log2 sum_{j<=arc} N_j - beta_k | plain stand-alone refuter, log2 mean nodes per block-passing arc (rows <= k only)")
nb = 0; acc_mu = []
cum = 0.0
quant = {}
for i, c in enumerate(free):
    if kind(c) > 0: nb += 1
    mu = W[i] - NF[nb - 1]
    acc_mu.append(mu + (-PB / L10 if i >= 46 else 0.0))   # after the two bottom blocks close (free depth 47), nodes belong to block-passing arcs
    cum += 10 ** (W[i] - tot)
    for q in (0.01, 0.1, 0.5, 0.9, 0.99):
        if q not in quant and cum >= q: quant[q] = (i + 1, nb, NF[nb - 1] * L10)
    if i + 1 == len(free) or free[i + 1][0] != c[0]:
        k = 16 - c[0]
        beta = NF[nb - 1] * L10
        rest = lsum(W[i + 1:]) - tot if i + 1 < len(W) else float("-inf")
        be = T_MEAS - (beta + (PB if k >= 3 else 0))
        walk = lsum(NF[:nb]) * L10 - beta
        plain = lsum(acc_mu) * L10
        print("%2d  %3d  %4d  %7.2f  %7.2f  %9.3g  | %6.2f  %10.3g | %5.2f | %6.2f  (%.3g core-s)"
              % (k, nb, i + 1, beta, mu, 10 ** rest if rest > -400 else 0, be, 2 ** be / 1e8, walk, plain, 2 ** plain / 1e8))
print("quantiles of row-scan nodes (free depth, arc cells, beta):", quant)
nb = 0
for i, c in enumerate(free):
    if kind(c) > 0: nb += 1
    if i > 150 and W[i] - NF[nb - 1] < 0:
        print("mu < 1 first at free depth %d, arc cells %d, beta %.2f" % (i + 1, nb, NF[nb - 1] * L10)); break

# ---- claim 5: interior tree under a full frame
ring = [(0, c) for c in range(16)] + [(r, 15) for r in range(1, 16)] + [(15, c) for c in range(14, -1, -1)] + [(r, 0) for r in range(14, 0, -1)]
def spiral():
    o = []
    for L in range(1, 8):
        a, b = L, 15 - L
        o += [(a, c) for c in range(a, b + 1)] + [(r, b) for r in range(a + 1, b + 1)] + [(b, c) for c in range(b - 1, a - 1, -1)] + [(r, a) for r in range(b - 1, a, -1)]
    return o
blocks = [(1, 2), (2, 1), (1, 1), (1, 13), (2, 14), (1, 14), (13, 1), (14, 2), (14, 1), (13, 14), (14, 13), (14, 14)]
sp = spiral()
assert len(sp) == 196 and len(set(sp)) == 196
orders = {
    "four blocks, then inward spiral": blocks + [c for c in sp if c not in blocks],
    "inward spiral": sp,
    "rows 14 to 1": [(r, c) for r in range(14, 0, -1) for c in range(1, 15)],
    "rows 1 to 14": [(r, c) for r in range(1, 15) for c in range(1, 15)],
}
SH4 = 17.45 / L10; SH2 = 8.93 / L10
print("\ninterior tree under a given full frame, five clues, model mean (log10 nodes)")
print("order | mean over all valid frames | mean over frames of H (the true frame is in H) | peak depth")
for name, o in orders.items():
    Wf = profile(ring + o, hints)[60:]
    fo = [c for c in o if c not in hints]
    n60 = profile(ring)[-1]
    per = [w - n60 for w in Wf]
    # depth at which each clue block is closed: all three block cells placed
    bl = [blocks[0:3], blocks[3:6], blocks[6:9], blocks[9:12]]
    closed = sorted(max(fo.index(c) for c in b) for b in bl)
    # nodes at a depth after j blocks are closed belong to frames passing those j blocks: mean over H = per / (share of those j blocks)
    perH = []
    for d, x in enumerate(per):
        j = sum(1 for cd in closed if d >= cd)
        perH.append(x + j * (17.45 / 4) / L10)
    print("%-32s | %6.2f | %6.2f | %d (blocks closed at free depths %s)" % (name, lsum(per), lsum(perH), per.index(max(per)) + 1, [x + 1 for x in closed]))
