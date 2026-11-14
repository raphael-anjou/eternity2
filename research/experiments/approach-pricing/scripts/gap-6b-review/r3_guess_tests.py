"""Review of gap 6b, script 3: lemma L3. Coarse commitments that DO unlock a test
the unconditioned search does not have, priced on the official pieces.

Accounting for a guess of I bits: price = I - t + R, with t the bits the unlocked
test removes from the guesses and R the tree left under one surviving guess.
The guess is profitable only if t > I - (B - R), B the unconditioned price.

 (a) orientation of every tile; test = 32 directional census equalities
 (b) checkerboard class of every tile (black or white cell); test = for every
     colour, as many half-edges on black cells as on white cells (every edge joins
     the two classes). 16 + 4 independent equalities, no orientation needed.
 (c) pool of a half board; test = colour-count parity against the free boundary
 (d) the same tests applied to the tiles LEFT at a row-scan node: when do they bite?
Local-limit (Gaussian) probabilities, checked against exact one- and two-coordinate
dynamic programs.
Usage: python3 -I r3_guess_tests.py <csv> [seed]
"""
import sys, os, random
from collections import Counter, defaultdict
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rlib import *
from math import comb, sqrt

tiles, extra = load(sys.argv[1])
rng = random.Random(int(sys.argv[2]) if len(sys.argv) > 2 else 3)
RIM = 65535
inner = [t for t in tiles if RIM not in t]
edge = [t for t in tiles if sum(c == RIM for c in t) == 1]
corner = [t for t in tiles if sum(c == RIM for c in t) == 2]
ICOL = sorted({c for t in inner for c in t}); FCOL = sorted({c for t in edge + corner for c in t if c != RIM and c not in ICOL})
ci = {c: i for i, c in enumerate(ICOL)}; cf = {c: i for i, c in enumerate(FCOL)}
PIr, PFr, PIu, PFu = tables()
L2PI = log2(2 * pi)

def profile(nint_choice, nedge_choice, ncor_choice, PI=PIr, PF=PFr):
    """Row scan with five clues; the three callbacks give log2(number of choices) at a cell."""
    filled = set(); eff = ein = 0; lg = 0.0; out = []
    used = Counter()
    for r in range(16):
        for c in range(16):
            k = kind(r, c); cls = (r + c) % 2
            if (r, c) in CLUES: pass
            elif k == 2: lg += ncor_choice(used[(2, cls)], cls)
            elif k == 1: lg += nedge_choice(used[(1, cls)], cls)
            else: lg += nint_choice(used[(0, cls)], used[(0, 1 - cls)], cls)
            if (r, c) not in CLUES: used[(k, cls)] += 1
            for dr, dc in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                q = (r + dr, c + dc)
                if q in filled:
                    if kind(*q) > 0 and k > 0: eff += 1
                    else: ein += 1
            filled.add((r, c)); out.append(lg + PI[ein] + PF[eff])
    return out

base = profile(lambda u, o, cls: log2(4 * (191 - u - o)), lambda u, cls: log2(56 - 2 * u) if False else 0, None) if False else None
used_tot = lambda u, o: u + o
plain = profile(lambda u, o, cls: log2(4 * (191 - u - o)), None, None) if False else None

def prof_plain():
    cnt = {"e": 0, "c": 0}
    def ne(u, cls):
        v = log2(56 - cnt["e"]); cnt["e"] += 1; return v
    def nc(u, cls):
        v = log2(4 - cnt["c"]); cnt["c"] += 1; return v
    return profile(lambda u, o, cls: log2(4 * (191 - u - o)), ne, nc)
def prof_orient():
    cnt = {"e": 0, "c": 0}
    def ne(u, cls):
        v = log2(56 - cnt["e"]); cnt["e"] += 1; return v
    def nc(u, cls):
        v = log2(4 - cnt["c"]); cnt["c"] += 1; return v
    return profile(lambda u, o, cls: log2(191 - u - o), ne, nc)
def prof_axis():
    cnt = {"e": 0, "c": 0}
    def ne(u, cls):
        v = log2(56 - cnt["e"]); cnt["e"] += 1; return v
    def nc(u, cls):
        v = log2(4 - cnt["c"]); cnt["c"] += 1; return v
    return profile(lambda u, o, cls: log2(2 * (191 - u - o)), ne, nc)
def prof_checker():
    # clue cells: (2,2) and (13,13) are class 0, the other three class 1: 96 and 95 free interior tiles
    avail = {0: 96, 1: 95}
    return profile(lambda u, o, cls: log2(4 * (avail[cls] - u)), lambda u, cls: log2(28 - u), lambda u, cls: log2(2 - u))

B = prof_plain(); Btot = lsum(B)
print(f"baseline, real-count ledger, five clues: row scan total 2^{Btot:.2f}, peak level 2^{max(B):.2f} (measured total 134.4; this ledger is {Btot-134.4:+.1f} bits off)")

# ---------- (a) orientation ----------
def orient_cov(ts):
    n = len(ICOL); M = [[0.0] * (2 * n) for _ in range(2 * n)]
    for t in ts:
        for k in range(4):
            v = defaultdict(int)
            v[ci[t[k]]] += 1; v[ci[t[(k + 2) % 4]]] -= 1
            v[n + ci[t[(k + 1) % 4]]] += 1; v[n + ci[t[(k + 3) % 4]]] -= 1
            items = [(a, x) for a, x in v.items() if x]
            for a, x in items:
                for b, y in items: M[a][b] += x * y / 4
    keep = [i for i in range(2 * n) if i != n - 1 and i != 2 * n - 1]
    return [[M[a][b] for b in keep] for a in keep]
def orient_bits(ts):
    S = orient_cov(ts); d = len(S)
    # lattice: (N-S)_c + (E-W)_c has a fixed parity for each kept colour: index 2^(d/2)
    return d / 2 * L2PI + 0.5 * logdet(S) - d / 2
t_or = orient_bits(inner)
# exact checks of the local-limit formula on two coordinates
def exact_orient_2d(ts, c):
    dist = {(0, 0): 1.0}
    for t in ts:
        steps = []
        for k in range(4):
            a = (t[k] == c) - (t[(k + 2) % 4] == c); b = (t[(k + 1) % 4] == c) - (t[(k + 3) % 4] == c)
            steps.append((a, b))
        nd = defaultdict(float)
        for (x, y), p in dist.items():
            for a, b in steps: nd[(x + a, y + b)] += p / 4
        dist = nd
    return dist[(0, 0)], sum(p for (x, y), p in dist.items() if x == 0)
c0 = ICOL[0]
p2, p1 = exact_orient_2d(inner, c0)
S = orient_cov(inner); n = len(ICOL)
i1, i2 = 0, n - 1    # (N-S)_c0 and (E-W)_c0 in the reduced matrix
v1 = S[i1][i1]; det2 = S[i1][i1] * S[i2][i2] - S[i1][i2] ** 2
print("\n== (a) orientation of every interior tile first")
print(f"check of the local-limit formula on colour {c0}: P(N-S = 0) exact {p1:.5f}, Gaussian {1/sqrt(2*pi*v1):.5f}; P(N-S = 0 and E-W = 0) exact {p2:.6f}, Gaussian with the parity factor 2 {2/(2*pi*sqrt(det2)):.6f}")
print(f"variance of N_c - S_c per colour on the real tiles: min {min(S[i][i] for i in range(16)):.2f}, max {max(S[i][i] for i in range(16)):.2f} (memo's uniform figure 21.70)")
print(f"test worth: memo 32 x 3.55 = 113.6 bits; real tiles, joint local limit with the parity lattice: {t_or:.1f} bits (without the parity factor it would read {t_or+16:.1f})")
Ro = lsum(prof_orient())
I_or = 2 * 191
print(f"committed {I_or} bits (191 free tiles); survivors 2^{I_or - t_or:.1f}; tree under one surviving guess 2^{Ro:.1f}; price 2^{I_or - t_or + Ro:.1f} against 2^{Btot:.1f}: {I_or - t_or + Ro - Btot:+.1f} bits")
Ra = lsum(prof_axis())
print(f"axis only (191 bits, no test): tree 2^{Ra:.1f}; price 2^{191+Ra:.1f}: {191+Ra-Btot:+.1f} bits")

# ---------- (b) checkerboard class ----------
def checker_cov(pops):
    """pops: list of (vectors, n_drawn). Covariance of the black-cell colour census, joint over the 22 colours."""
    D = len(ICOL) + len(FCOL); M = [[0.0] * D for _ in range(D)]
    for vecs, nd in pops:
        N = len(vecs)
        if N < 2: continue
        mean = [sum(v[a] for v in vecs) / N for a in range(D)]
        f = nd * (N - nd) / (N * (N - 1))
        for v in vecs:
            d = [(a, v[a] - mean[a]) for a in range(D) if v[a] - mean[a]]
            for a, x in d:
                for b, y in d: M[a][b] += f * x * y
    keep = [i for i in range(D) if i != len(ICOL) - 1 and i != D - 1]
    return [[M[a][b] for b in keep] for a in keep]
def vec(t):
    v = [0] * (len(ICOL) + len(FCOL))
    for c in t:
        if c == RIM: continue
        v[ci[c] if c in ci else len(ICOL) + cf[c]] += 1
    return v
VI = [vec(t) for t in inner]; VE = [vec(t) for t in edge]; VC = [vec(t) for t in corner]
Sc = checker_cov([(VI, 98), (VE, 28), (VC, 2)])
t_cb = len(Sc) / 2 * L2PI + 0.5 * logdet(Sc)
# exact one-coordinate check: P(black census of colour c0 = half), interior tiles and edge tiles
def exact_half(c):
    tot = 0; dist = None
    def conv(vecs, nd, a):
        d = {(0, 0): 1}
        for v in vecs:
            nd2 = defaultdict(int)
            for (k, s), w in d.items():
                nd2[(k, s)] += w
                if k < nd: nd2[(k + 1, s + v[a])] += w
            d = nd2
        out = defaultdict(int)
        for (k, s), w in d.items():
            if k == nd: out[s] += w
        return out
    a = ci[c]
    d1 = conv(VI, 98, a); d2 = conv(VE, 28, a)
    total = sum(v[a] for v in VI + VE)
    num = sum(w1 * d2.get(total // 2 - s1, 0) for s1, w1 in d1.items())
    return num / (comb(196, 98) * comb(56, 28))
pe = exact_half(c0)
print("\n== (b) checkerboard class of every tile first (98/98 interior, 28/28 edge, 2/2 corner)")
print(f"check on colour {c0}: P(black census = half) exact {pe:.5f}, Gaussian {1/sqrt(2*pi*Sc[0][0]):.5f}")
I_cb = log2(comb(191, 96)) + log2(comb(56, 28)) + log2(comb(4, 2))
Rc = lsum(prof_checker())
print(f"committed {I_cb:.1f} bits; test = {len(Sc)} independent equalities worth {t_cb:.1f} bits on the real tiles; survivors 2^{I_cb - t_cb:.1f}")
print(f"tree under one surviving guess 2^{Rc:.1f} (peak level 2^{max(prof_checker()):.1f}); price 2^{I_cb - t_cb + Rc:.1f} against 2^{Btot:.1f}: {I_cb - t_cb + Rc - Btot:+.1f} bits")
print(f"   saved by the guess {Btot - Rc:.1f} of {I_cb:.1f} committed ({(Btot-Rc)/I_cb:.2f} bit per bit); the test would have to be worth {I_cb-(Btot-Rc):.1f} bits to break even and is worth {t_cb:.1f}")

# ---------- (c) half-board pool, parity test ----------
print("\n== (c) pool of the top half (98 interior tiles in rows 1..7) first; test = colour counts against the boundary")
print("frame unknown: 42 free boundary half-edges, the number of odd-count colours is always even and at most 17: the test never fails (0 bits)")
fails = 0; T = 20000
inward = [t[[j for j in range(4) if t[(j + 2) % 4] == RIM][0]] for t in edge]
for _ in range(T):
    pool = rng.sample(inner, 98); fr = rng.sample(inward, 28)
    cnt = Counter(c for t in pool for c in t); f = Counter(fr)
    odd = sum(1 for c in ICOL if (cnt[c] - f[c]) % 2); neg = any(cnt[c] < f[c] for c in ICOL)
    if odd > 14 or neg: fails += 1
print(f"frame known (28 boundary sides fixed, 14 seam sides free): fails {fails} of {T} random pools: {-log2(1 - fails / T) if fails < T else float('inf'):.3f} bits for {log2(comb(196,98)):.1f} bits committed")
e_blk = 2 * 14 * 7 - 14 - 7
fill = lfall(196, 98) + 196 + PIr[e_blk]
print(f"with an ORACLE for 'this pool tiles a free 14x7 block': strict fillings 2^{fill:.1f} (real counts), expected tilings per pool 2^{fill - log2(comb(196,98)):.1f}, so about 2^{fill:.1f} pools pass and the exact test is worth {log2(comb(196,98)) - fill:.1f} bits")

# ---------- (d) the same tests on the tiles left at a row-scan node ----------
print("\n== (d) census tests applied to the r interior tiles left at a row-scan node: first-moment number of ways to satisfy them")
print("r    log2(orientations) - directional census bits     log2(black/white splits) - checkerboard census bits")
for r in (16, 24, 32, 40, 56, 72, 100):
    a = []; b = []
    for _ in range(5):
        sub = rng.sample(range(196), r)
        a.append(2 * r - orient_bits([inner[i] for i in sub]))
        S2 = checker_cov([([VI[i] for i in sub], r // 2)]); S2 = [row[:16] for row in S2[:16]]
        b.append(log2(comb(r, r // 2)) - (8 * L2PI + 0.5 * logdet(S2)))
    fmt = lambda xs: "degenerate" if any(x == float("inf") for x in xs) else f"{sum(xs)/5:8.1f}"
    print(f"{r:3d}   {fmt(a):>10s}                                       {fmt(b):>10s}")
print("degenerate = the census covariance is singular on so few tiles (a colour pair is missing), the local-limit formula does not apply and the test may bite")
print("positive = the test is expected to pass on a typical remaining pool (it cannot refute); row scan's widest level has r = 68 to 72 interior tiles left")
