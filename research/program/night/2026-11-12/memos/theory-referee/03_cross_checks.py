"""Cross-checks of quoted numbers against committed result files, plus small recomputations."""
import json, math, os, random, statistics, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from collections import Counter, defaultdict
from itertools import combinations
from e2lib import load, kind, ROOT
n, P, H = load()
T = os.path.join(ROOT, "research", "topics")
rot = lambda p, r: tuple(p[(i - r) % 4] for i in range(4))
INT = [p for p in P if kind(p) == "interior"]; EDG = [p for p in P if kind(p) == "edge"]
lg10f = lambda k: math.lgamma(k + 1) / math.log(10)

print("== (a) first moment for an N x N framed board, 5 frame colours, c interior colours, no hints ==")
def log10_df(k2):  # log10 (2k-1)!! with k = k2/2 possibly non-integer: (2k-1)!! = 2^k Gamma(k+1/2)/sqrt(pi)
    k = k2 / 2
    return (k * math.log(2) + math.lgamma(k + 0.5) - 0.5 * math.log(math.pi)) / math.log(10)
def full_first_moment(N, c):
    m = (N - 2) ** 2; e = 4 * (N - 2); ring = 4 * (N - 1); J = 2 * (N - 2) * (N - 3) + e
    arr = math.log10(24) + lg10f(e) + lg10f(m) + m * math.log10(4)
    ringp = 5 * lg10f(ring / 5) - lg10f(ring)          # balanced ring census
    intp = c * log10_df(2 * J / c) - log10_df(2 * J)    # uniform interior census, random pairing
    return arr + ringp + intp
def crude(N, c):  # the formula on how-hard-is-this-instance.mdx:285
    m = (N - 2) ** 2; j = 2 * (N - 2) * (N - 3)
    return lg10f(m) + m * math.log10(4) - j * math.log10(c)
print("  check at N=16,c=17 (official census gives 7.0617):", round(full_first_moment(16, 17), 3))
for c in (17, 21):
    print(f"  c={c}: " + "  ".join(f"N={N}: full {full_first_moment(N, c):7.1f} / page-crude {crude(N, c):6.1f}" for N in range(10, 17)))
print("  interior colours giving log10 E = 0 at N=16 (bisection):", end=" ")
lo, hi = 10.0, 30.0
for _ in range(60):
    mid = (lo + hi) / 2
    if full_first_moment(16, mid) > 0: lo = mid
    else: hi = mid
print(round(lo, 2), "; with starter pinned:", end=" ")
lo, hi = 10.0, 30.0
for _ in range(60):
    mid = (lo + hi) / 2
    if full_first_moment(16, mid) - math.log10(784) > 0: lo = mid
    else: hi = mid
print(round(lo, 2))
print("  Owen criterion 6x6:", math.exp((math.lgamma(17) + 16 * math.log(4)) / 32))

print("== (b) hardness-peak sweeps ==")
for f in ("hardness-peak-8x8.json", "hardness-peak-16x16.json"):
    d = json.load(open(os.path.join(T, "hardness-peak", "results", f)))
    by = defaultdict(list)
    for r in d["rows"]: by[r["interiorColors"]].append(r)
    print(" ", f, "budget", d["nodeBudget"])
    print("   solved/seeds by interior colours:", {c: f"{sum(r['solved'] for r in v)}/{len(v)}" for c, v in sorted(by.items())})
    print("   median bestPlaced:", {c: statistics.median(r["bestPlaced"] for r in v) for c, v in sorted(by.items())})

print("== (c) forbidden patterns and partners ==")
colset = [set(p) for p in INT]
feas2 = sum(1 for i in range(196) for j in range(196) if i != j and colset[i] & colset[j])
print("  ordered pairs feasible:", feas2, "forbidden:", 196 * 195 - feas2)
partners = [sum(1 for j in range(196) if j != i and colset[i] & colset[j]) for i in range(196)]
print("  partners min/median/max/mean:", min(partners), statistics.median(partners), max(partners), round(statistics.mean(partners), 1))
has = defaultdict(int)
for j, p in enumerate(INT):
    for c in set(p): has[c] |= 1 << j
featL = 0
for i, a in enumerate(INT):
    ai = 1 << i
    opts = {(rot(a, r)[1], rot(a, r)[2]) for r in range(4)}   # (right, bottom) colour pairs of the corner cell
    for j in range(196):
        if j == i: continue
        mask = 0
        for (cr, cb) in opts:
            if has[cr] >> j & 1: mask |= has[cb]
        mask &= ~(ai | (1 << j))
        featL += bin(mask).count("1")
print("  L-tromino feasible:", featL, "forbidden:", 196 * 195 * 194 - featL)

print("== (e) stored-orientation statistics (not rotation-invariant) ==")
near_stored = sum(1 for p, q in combinations(P, 2) if sum(a == b for a, b in zip(p, q)) == 3)
print("  pairs agreeing on 3 same-position sides in the CSV orientation:", near_stored)
def cap(pieces):
    d = [Counter() for _ in range(4)]
    for p in pieces:
        for i, c in enumerate(p):
            if c: d[i][c] += 1
    return sum(min(d[1][c], d[3][c]) + min(d[0][c], d[2][c]) for c in range(1, 23))
print("  fixed-orientation cap in CSV orientation:", cap(P))
rng = random.Random(1)
caps = []
for _ in range(2000):
    caps.append(cap([rot(p, rng.randrange(4)) for p in P]))
print("  same cap, every piece given a uniform random rotation (2000 draws): mean", round(statistics.mean(caps), 1), "sd", round(statistics.pstdev(caps), 1), "min", min(caps), "max", max(caps))
print("  CSV orientation of interior pieces: fraction with smallest colour on top:", sum(p[0] == min(p) for p in INT) / 196,
      "; grey side index histogram for edge pieces:", Counter(p.index(0) for p in EDG))

print("== (f) null model for rotation duplicates: paint a solved board with the official joint-colour counts ==")
def painted(rng):
    ringc = [c for c in range(1, 6) for _ in range(12)]; rng.shuffle(ringc)
    inner = []
    cnt = {c: (24 if c <= 10 else 25) for c in range(6, 23)}
    for c, k in cnt.items(): inner += [c] * k
    rng.shuffle(inner)
    it = iter(inner)
    Hh = [[0] * 15 for _ in range(16)]; V = [[0] * 16 for _ in range(15)]
    ri = iter(ringc)
    for y in range(16):
        for x in range(15):
            Hh[y][x] = next(ri) if y in (0, 15) else next(it)
    for y in range(15):
        for x in range(16):
            V[y][x] = next(ri) if x in (0, 15) else next(it)
    out = []
    for y in range(16):
        for x in range(16):
            out.append((V[y - 1][x] if y else 0, Hh[y][x] if x < 15 else 0, V[y][x] if y < 15 else 0, Hh[y][x - 1] if x else 0))
    return out
rng = random.Random(7)
canon = lambda p: min(rot(p, r) for r in range(4))
tr = 4000; zero_dup = 0; dup_sum = 0; dup_int = 0; dup_border = 0; zero_sym = 0; zero_dup_int = 0
for _ in range(tr):
    B = painted(rng)
    cc = Counter(canon(p) for p in B)
    d = sum(v * (v - 1) // 2 for v in cc.values()); dup_sum += d; zero_dup += d == 0
    di = sum(v * (v - 1) // 2 for k, v in cc.items() if 0 not in k); dup_int += di; dup_border += d - di; zero_dup_int += di == 0
    zero_sym += not any(rot(p, 2) == p for p in B)
print(f"  {tr} painted boards: mean duplicate pairs {dup_sum / tr:.3f} (interior {dup_int / tr:.3f}, border {dup_border / tr:.3f});"
      f" P(no duplicate at all) {zero_dup / tr:.4f}; P(no interior duplicate) {zero_dup_int / tr:.3f}; P(no half-turn-symmetric piece) {zero_sym / tr:.3f}")
print("  analytic: interior C(196,2)*4/17^4 =", 19110 * 4 / 17 ** 4, "; edge C(56,2)/(5*5*17) =", 1540 / 425, "; corner C(4,2)/25 =", 6 / 25)

print("== (g) frame variance control: one-way ANOVA recomputed ==")
d = json.load(open(os.path.join(T, "frame-is-not-the-basin", "results", "frame_variance_control.json")))
S = d["scores_by_frame"]; k = len(S); m = len(S[0]); gm = sum(map(sum, S)) / (k * m)
ssb = m * sum((sum(r) / m - gm) ** 2 for r in S); ssw = sum((x - sum(r) / m) ** 2 for r in S for x in r)
F = (ssb / (k - 1)) / (ssw / (k * (m - 1)))
def betainc(a, b, x, steps=200000):  # regularised incomplete beta by midpoint rule in log space
    h = x / steps; s = 0.0
    for i in range(steps):
        t = (i + 0.5) * h; s += math.exp((a - 1) * math.log(t) + (b - 1) * math.log(1 - t))
    return s * h / math.exp(math.lgamma(a) + math.lgamma(b) - math.lgamma(a + b))
d1, d2 = k - 1, k * (m - 1)
pval = betainc(d2 / 2, d1 / 2, d2 / (d2 + d1 * F))
print(f"  F({d1},{d2}) = {F:.4f}, p = {pval:.3f}, ICC = {(ssb / d1 - ssw / d2) / (ssb / d1 + (m - 1) * ssw / d2):.4f}, range {min(map(min, S))}-{max(map(max, S))}")
# largest ICC not rejected at 5%: find rho with P(F_obs or smaller | rho) = 0.05, F ~ (1 + m rho/(1-rho)) F(d1,d2)
lo, hi = 0.0, 0.9
for _ in range(40):
    mid = (lo + hi) / 2; scale = 1 + m * mid / (1 - mid); Fs = F / scale
    plow = 1 - betainc(d2 / 2, d1 / 2, d2 / (d2 + d1 * Fs), 20000)
    if plow > 0.05: lo = mid
    else: hi = mid
print(f"  one-sided 95% upper bound on the border share of variance (ICC): {lo:.3f}")

print("== (h) sigma-cycles population ==")
d = json.load(open(os.path.join(T, "sigma-cycles", "results", "sigma-cycles.json")))
prs = d["pairs"]
big = [c for p in prs for c in p["largeCycleCurves"]]
print("  pairs", len(prs), "large cycles", len(big), "prefixes", sum(c["prefixesTested"] for c in big), ">= start:", sum(c["prefixesGE"] for c in big),
      "closest", max(c["maxDelta"] for c in big), "deepest", min(c["minDelta"] for c in big))
lc = [p["largestCycle"] for p in prs]
print("  largest cycle per pair: min", min(lc), "median", statistics.median(lc), "max", max(lc), "; single-cycle pairs:", sum(p["cycleCount"] == 1 for p in prs))
print("  moved cells: min", min(p["movedCells"] for p in prs), "median", statistics.median(p["movedCells"] for p in prs))
far = [p for p in prs if p["movedCells"] >= 240]
print("  pairs moving >= 240 cells:", len(far), "; their largest cycle median:", statistics.median(p["largestCycle"] for p in far), " mean largest/moved:", round(statistics.mean(p["largestCycle"] / p["movedCells"] for p in far), 3))
rng = random.Random(3)
def largest_cycle(sizes):
    best = 0
    for s in sizes:
        perm = list(range(s)); rng.shuffle(perm); seen = [False] * s
        for i in range(s):
            if not seen[i]:
                L = 0; j = i
                while not seen[j]: seen[j] = True; j = perm[j]; L += 1
                best = max(best, L)
    return best
sim = [largest_cycle((196, 56, 4)) for _ in range(4000)]
print("  null: uniform random class-preserving permutation (196 interior, 56 edge, 4 corner): largest cycle median", statistics.median(sim),
      "mean", round(statistics.mean(sim), 1), "5%-95%", sorted(sim)[200], sorted(sim)[3800], "; Golomb-Dickman 0.6243*196 =", round(0.6243 * 196, 1))

print("== (k) defect-2 neighbours of a solution by one ADJACENT swap that is not a near-twin swap ==")
# P left of Q sharing a matched joint. After the swap Q' sits left, P' right. Count bag-level configurations with
# exactly two broken joints among the 6 outer joints + the shared one, where neither new piece agrees with the hole on 3+1 sides.
ori = [[rot(p, r) for r in range(4)] for p in INT]
exotic_pairs = set(); exotic_cfg = 0; twin_cfg = 0
byE = defaultdict(list)
for i in range(196):
    for r in range(4): byE[ori[i][r][1]].append((i, r))
for j in range(196):
    for rq in range(4):
        q = ori[j][rq]                       # right piece, its left colour is q[3]
        for (i, rp) in byE[q[3]]:
            if i == j: continue
            p = ori[i][rp]                   # left piece with p[1] == q[3]
            for q2 in ori[j]:                # Q' goes left: outer sides N(0), S(2), W(3); its E (1) is the new shared side
                bl = (q2[0] != p[0]) + (q2[2] != p[2]) + (q2[3] != p[3])
                if bl > 2: continue
                for p2 in ori[i]:            # P' goes right: outer sides N(0), E(1), S(2); its W (3) is shared
                    br = (p2[0] != q[0]) + (p2[1] != q[1]) + (p2[2] != q[2])
                    if bl + br + (q2[1] != p2[3]) == 2:
                        if q2[1] == p[1] and p2[3] == q[3]: twin_cfg += 1
                        else: exotic_cfg += 1; exotic_pairs.add((min(i, j), max(i, j)))
print("  horizontal adjacent configurations giving exactly 2 broken joints: with the shared colour kept", twin_cfg,
      "; with a new shared colour", exotic_cfg, "; distinct unordered piece pairs of the second kind:", len(exotic_pairs))
