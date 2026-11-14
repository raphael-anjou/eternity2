"""Route audit, 2026-11-14. Two derived figures from Track 10's independently assessed level profile
(five-np1e6.jsonl, copied from branch night/2026-11-12-t10-real-puzzle-profile).

1. Ceiling of a perfect free cut at depth d (check of the quoted 29.8 / 17.1 / 10.7 / 6.8 / 2.4).
2. What a coarse stratified painter prior (solve-path-2, section 4 to 6) would be worth under
   row scan, replacing that memo's assumed usable share S1 = 0.5 to 0.75 by a computation.

Model for 2 (first moment, same spirit as the program's node model): the colours of the
inner joints already fixed by a row-scan prefix are draws from the 17 inner colours with
probabilities n_c / 420 (multinomial; the true law is without replacement). A prefix
survives the stratum budget when, in every stratum, no colour exceeds its ceiling and the
floors can still be reached with the joints left. Strata treated as independent.
Nothing here is a measurement on the real pieces. Run: python3 -I strata_gain.py
"""
import json, math, os
from functools import lru_cache

HERE = os.path.dirname(os.path.abspath(__file__))
reps = [json.loads(l) for l in open(os.path.join(HERE, "five-np1e6.jsonl"))]
L = max(len(r["log10_width"]) for r in reps)
W = []
for d in range(L):
    v = [10 ** r["log10_width"][d] for r in reps if d < len(r["log10_width"]) and r["log10_width"][d] is not None]
    W.append(sum(v) / len(reps))
TOT = sum(W)
print("total nodes log10 %.3f log2 %.2f, widest free depth %d" % (math.log10(TOT), math.log2(TOT), max(range(L), key=lambda d: W[d])))
for d in (52, 64, 80, 96, 112, 128, 144, 158):
    print("perfect free cut at free depth %3d: ceiling %.1f bits" % (d, math.log2(TOT / sum(W[: d + 1]))))

N = 16
CLUES = sorted([2 * 16 + 2, 2 * 16 + 13, 13 * 16 + 2, 13 * 16 + 13, 8 * 16 + 7])  # positions approximate; only shifts depth by <= 5


def cell_index(free_depth):
    i = 0
    placed = 0
    while placed < free_depth or i in CLUES:
        if i not in CLUES:
            placed += 1
        i += 1
    return i  # cells 0..i-1 filled (clues inside that range are filled too)


# inner joints: both cells not separated along the rim, i.e. the joint is not a rim-to-rim border joint
def joints():
    out = []
    for r in range(N):
        for c in range(N):
            if c + 1 < N:  # horizontal neighbour, vertical joint line
                rim = r in (0, N - 1)
                if not rim:
                    out.append(((r, c), (r, c + 1), "V"))
            if r + 1 < N:
                rim = c in (0, N - 1)
                if not rim:
                    out.append(((r, c), (r + 1, c), "H"))
    return out


J = joints()
assert len(J) == 420, len(J)
NC = [25] * 12 + [24] * 5  # 24a + 25b = 420 with a + b = 17
P = [n / 420 for n in NC]


def strata(kind):
    def lab(j):
        (r1, c1), (r2, c2), o = j
        mr, mc = (r1 + r2) / 2, (c1 + c2) / 2
        if kind == "quadrants":
            return (mr < 7.75, mc < 7.75)  # seam joints go to the top / left stratum
        if kind == "halves":
            return mr < 7.75
        if kind == "hv":
            return o
    return lab


def survive(kind, filled):
    """log2 probability that every stratum budget is still satisfiable."""
    lab = strata(kind)
    tot, det = {}, {}
    for j in J:
        s = lab(j)
        tot[s] = tot.get(s, 0) + 1
        a, b = j[0], j[1]
        if a[0] * N + a[1] < filled or b[0] * N + b[1] < filled:
            det[s] = det.get(s, 0) + 1
    k = len(tot)
    lg = 0.0
    for s in tot:
        Jd, R = det.get(s, 0), tot[s] - det.get(s, 0)
        if kind == "hv" and s == "V":
            continue  # with fixed colour totals the V budget is implied by the H budget
        share = tot[s] / 420  # quotas proportional to the stratum's size (strata are not exactly equal)
        fl = [math.floor(n * share) for n in NC]
        ce = [math.ceil(n * share) for n in NC]
        # dp over colours: state (joints used, deficit to floors) -> sum of prod p^x / x!
        dp = {(0, 0): 1.0}
        for c in range(17):
            nd = {}
            for (u, de), v in dp.items():
                for x in range(0, min(ce[c], Jd - u) + 1):
                    key = (u + x, de + max(0, fl[c] - x))
                    nd[key] = nd.get(key, 0.0) + v * (max(Jd, 1) * P[c]) ** x / math.factorial(x)  # Poisson scaling against underflow
            dp = nd
        p = sum(v for (u, de), v in dp.items() if u == Jd and de <= R)
        if p > 0:
            lg += (math.log(p) + math.lgamma(Jd + 1) - Jd * math.log(max(Jd, 1))) / math.log(2)
        else:
            lg += -1e9
    return lg


for kind in ("hv", "halves", "quadrants"):
    f = [survive(kind, cell_index(d)) for d in range(L)]
    thin = sum(W[d] * 2 ** f[d] for d in range(L))
    full = survive(kind, 256)
    row = ", ".join("%d: %.1f" % (d, -f[d]) for d in (48, 64, 80, 96, 112, 128, 144, 160))
    # full-board figure: one stratum is implied by the others, so scale by (k-1)/k except hv (already one stratum)
    kk = {"hv": 1.0, "halves": 0.5, "quadrants": 0.75}[kind]
    full *= kk
    print("%-9s bits carried by a full board %.1f; bits removed at free depth {%s}; row-scan gain %.1f bits (tree 2^%.1f)" % (
        kind, -full, row, math.log2(TOT / thin), math.log2(thin)))
