"""Gap 3 review, items 1 to 3: what the memo's own sibling data say.

  uv run --with numpy python r3_tail.py <memo's out/sib/sib-*.out>

Input: the memo's files, read only. One line "SB j size" per sibling of the planted path
(size = estimated nodes strictly below the sibling). TOY boards (painted 16x16).

Time to the solution under a random value order: T = D + sum over preceding siblings of
(1 + size). For a cutoff t, restart cost l(t) = E[min(T, t)] / q(t), q(t) = P(T <= t), and
gain(t) = log2(E[T] / l(t)).

A. The memo's two closed forms (achieved, upper bound) recomputed, then the EXACT gain by
   conditional sampling: q(t) = q0 * P(X <= t), q0 = prod_j 1/(1 + n_j(t)) the probability
   that no sibling larger than t precedes the planted child, X the time given that event
   (per depth, with positions iid uniform: given that the planted child is the first of
   itself and the n large siblings its position is the minimum of n + 1 uniforms, and each
   small sibling precedes it independently given that position).
B. Tail of the best gain against 2^-g (the memo's comparison) and 2^-(g+1) (the memo's bound).
C. Fixed cutoffs tree/2^g: exact gain, and the MEAN over cases of 2^gain. If the solution is
   located in proportion to work, E[q(t)] = t/tree and that mean is about 1/2 at every cutoff.
D. Blind portfolio over the four directions of one board, each with its own best cutoff.
E. Which law places the planted child among its siblings: P(child) proportional to
   (subtree size)^alpha. alpha = 0 is "exchangeable siblings", alpha = 1 is "in proportion to
   work". Maximum likelihood per depth band, and the probability integral transform under
   alpha = 1 (mean 0.5 if work-proportional, above 0.5 if the solution prefers larger subtrees
   more than in proportion).
"""
import math
import sys
from collections import defaultdict

import numpy as np

D = 251
NS = 4000
rng = np.random.default_rng(1113)


def load(fn):
    sib = defaultdict(list)
    for l in open(fn):
        t = l.split()
        if t and t[0] == "SB":
            sib[int(t[1])].append(1.0 + float(t[2]))
    return sib


def cond_times(sib, s):
    """NS samples of T given that no sibling larger than s precedes the planted child."""
    x = np.full(NS, float(D))
    for v in sib.values():
        small = [a for a in v if a <= s]
        if not small:
            continue
        n = sum(1 for a in v if a > s)
        up = 1.0 - rng.random(NS) ** (1.0 / (n + 1))   # planted position given it is first of n + 1
        for a in small:
            x += a * (rng.random(NS) < up)
    return np.sort(x)


def log2q0(sib, s):
    return -sum(math.log2(1 + sum(1 for a in v if a > s)) for v in sib.values())


def exact_fixed(sib, ET, t):
    """exact gain of the fixed cutoff t, and log2 q(t)"""
    lq0 = log2q0(sib, t)
    x = cond_times(sib, t)
    k = int(np.searchsorted(x, t, side="right"))
    if k == 0:
        return float("-inf"), float("-inf")
    q = 2.0 ** lq0 * k / NS
    emin = t * (1 - q) + 2.0 ** lq0 * float(x[:k].sum()) / NS
    return math.log2(ET / (emin / q)), math.log2(q)


def selfcheck(fn):
    """the conditional sampler against plain sampling of random value orders, where q is large"""
    sib = load(fn)
    tree = D + sum(sum(v) for v in sib.values())
    ET = D + (tree - D) / 2
    n = 200000
    x = np.full(n, float(D))
    for v in sib.values():
        up = rng.random(n)
        for a in v:
            x += a * (rng.random(n) < up)
    out = []
    for g in (1, 3, 5, 7):
        t = tree / 2.0 ** g
        ok = x <= t
        q = float(ok.mean())
        plain = math.log2(ET / ((t * (1 - q) + float(x[ok].sum()) / n) / q)) if q > 0 else float("-inf")
        out.append("t = tree/2^%d: plain %.2f (q %.4f), conditional %.2f" % (g, plain, q, exact_fixed(sib, ET, t)[0]))
    print("   self-check %s, mean T/tree %.3f | %s" % (fn.rsplit("/", 1)[-1], float(x.mean()) / tree, " | ".join(out)))


def analyse(fn):
    sib = load(fn)
    tree = D + sum(sum(v) for v in sib.values())
    ET = D + (tree - D) / 2
    sizes = sorted(set(a for v in sib.values() for a in v))
    allsz = sorted(a for v in sib.values() for a in v)
    # memo's closed forms
    rows = []
    pre = 0.0
    i = 0
    for s in sizes:
        while i < len(allsz) and allsz[i] <= s:
            pre += allsz[i]
            i += 1
        lq = log2q0(sib, s)
        if lq >= 0:
            continue
        q = 2.0 ** lq
        ub = math.log2(ET / max(s, D)) + lq - math.log2(1 - q)
        lb = math.log2(ET / (D + pre)) + lq
        rows.append((s, lq, lb, ub))
    best_lb = max(r[2] for r in rows)
    best_ub = max(r[3] for r in rows)
    # exact best gain: only classes whose upper bound can beat the best achieved value
    best = (best_lb, None, None)
    for idx, (s, lq, lb, ub) in enumerate(rows):
        if ub < best[0]:
            continue
        nxt = rows[idx + 1][0] if idx + 1 < len(rows) else float("inf")
        x = cond_times(sib, s)
        cs = np.cumsum(x)
        q0 = 2.0 ** lq
        for k in range(20, NS + 1, 20):            # cutoff at the k-th conditional sample
            t = x[k - 1]
            if t >= nxt:
                break
            q = q0 * k / NS
            emin = t * (1 - q) + q0 * cs[k - 1] / NS
            g = math.log2(ET / (emin / q))
            if g > best[0]:
                best = (g, t, q)
    fixed = {}
    for g in (10, 20, 40, 60, 85):
        fixed[g] = exact_fixed(sib, ET, tree / 2.0 ** g)
    # best exact gain with the cutoff at most tree/2^85: scan classes below that size
    lim = tree / 2.0 ** 85
    b85 = float("-inf")
    for idx, (s, lq, lb, ub) in enumerate(rows):
        if s > lim or ub < b85:
            continue
        nxt = min(rows[idx + 1][0] if idx + 1 < len(rows) else float("inf"), lim)
        x = cond_times(sib, s)
        cs = np.cumsum(x)
        q0 = 2.0 ** lq
        for k in range(20, NS + 1, 20):
            t = x[k - 1]
            if t > nxt:
                break
            q = q0 * k / NS
            g = math.log2(ET / ((t * (1 - q) + q0 * cs[k - 1] / NS) / q))
            b85 = max(b85, g)
    b85 = max(b85, fixed[85][0])
    # planted child's own subtree at each depth and the placement law
    below = 0.0
    place = []                                      # (depth, planted size, sibling sizes)
    for j in range(D - 1, -1, -1):
        c = 1.0 + (D - 1 - j) + below
        if sib.get(j):
            place.append((j, c, sib[j]))
        below += sum(sib.get(j, []))
    return dict(tree=tree, ET=ET, lb=best_lb, ub=best_ub, exact=best[0], fixed=fixed, b85=b85, place=place)


def band(j):
    return 0 if j < 32 else 1 if j < 96 else 2 if j < 160 else 3


BANDS = ["depth 0-31", "depth 32-95", "depth 96-159", "depth 160-250"]


def main():
    res = {}
    for fn in sys.argv[1:4]:
        selfcheck(fn)
    for fn in sys.argv[1:]:
        name = fn.rsplit("/", 1)[-1].replace(".out", "").replace("sib-", "")
        res[name] = analyse(fn)
        r = res[name]
        print("%-5s tree 2^%.1f | best gain: achieved bound %.2f, EXACT %.2f, upper bound %.2f | cutoff <= tree/2^85: exact best %.2f | fixed cutoff tree/2^g exact gain: %s"
              % (name, math.log2(r["tree"]), r["lb"], r["exact"], r["ub"], r["b85"],
                 " ".join("g=%d %.1f" % (g, r["fixed"][g][0]) for g in sorted(r["fixed"]))))
    n = len(res)
    med = lambda xs: sorted(xs)[len(xs) // 2]
    print("\nA. %d cases: best gain median, achieved bound %.2f, exact %.2f, upper bound %.2f; max exact %.2f"
          % (n, med([r["lb"] for r in res.values()]), med([r["exact"] for r in res.values()]),
             med([r["ub"] for r in res.values()]), max(r["exact"] for r in res.values())))
    x85 = sorted(r["b85"] for r in res.values())
    print("   cutoff at most tree/2^85: exact best gain median %.2f [%.2f, %.2f]" % (med(x85), x85[0], x85[-1]))
    print("\nB. tail of the best gain: cases at or above g bits (expected count at 2^-g | at the memo's bound 2^-(g+1))")
    for g in (1, 2, 3, 4, 5, 6):
        print("   g=%d: achieved bound %2d, exact %2d, upper bound %2d   (%.2f | %.2f)"
              % (g, sum(r["lb"] >= g for r in res.values()), sum(r["exact"] >= g for r in res.values()),
                 sum(r["ub"] >= g for r in res.values()), n * 2.0 ** -g, n * 2.0 ** -(g + 1)))
    boards = sorted(set(k.split("-")[0] for k in res))
    print("   boards with at least one direction at or above g bits (exact): "
          + ", ".join("g=%d: %d of %d" % (g, sum(any(res[k]["exact"] >= g for k in res if k.split("-")[0] == b) for b in boards), len(boards)) for g in (1, 2, 3, 4)))
    print("\nC. fixed cutoff tree/2^g, exact: median gain, mean over cases of 2^gain (about 0.5 if the solution is located in proportion to work), largest single share of that mean")
    for g in (10, 20, 40, 60, 85):
        gs = [r["fixed"][g][0] for r in res.values()]
        w = [2.0 ** x if x > -1e300 else 0.0 for x in gs]
        print("   g=%2d: median gain %6.2f, mean 2^gain %.3f, top case holds %.2f of the sum; median log2 q %.1f against log2(t/tree) = -%d"
              % (g, med(gs), sum(w) / n, max(w) / sum(w), med([r["fixed"][g][1] for r in res.values()]), g))
    print("\nD. blind portfolio over the four directions of a board, each direction at its own best cutoff (exact)")
    nets = []
    for b in boards:
        ks = [k for k in res if k.split("-")[0] == b]
        rate = sum(2.0 ** res[k]["exact"] / res[k]["ET"] for k in ks) / len(ks)
        plain = sum(1.0 / res[k]["ET"] for k in ks) / len(ks)       # same portfolio without cutoffs... per-node rate of plain runs
        net = math.log2(rate / plain)
        nets.append(net)
        print("   board %s: member gains %s -> portfolio net %.2f bits (best member %.2f, minus log2 4 = %.2f)"
              % (b, " ".join("%.2f" % res[k]["exact"] for k in ks), net, max(res[k]["exact"] for k in ks), max(res[k]["exact"] for k in ks) - 2))
    print("   net over boards: median %.2f, max %.2f; single direction median %.2f" % (med(nets), max(nets), med([r["exact"] for r in res.values()])))
    print("\nE. law of the planted child among its siblings, P proportional to size^alpha")
    data = defaultdict(list)
    for r in res.values():
        for (j, c, v) in r["place"]:
            data[band(j)].append((c, v))
            data[4].append((c, v))
    for bi in range(5):
        dd = data[bi]
        if not dd:
            continue

        def ll(alpha):
            s = 0.0
            for c, v in dd:
                lc = alpha * math.log(c)
                mx = max(lc, max(alpha * math.log(a) for a in v))
                s += lc - mx - math.log(math.exp(lc - mx) + sum(math.exp(alpha * math.log(a) - mx) for a in v))
            return s

        grid = [i / 20 for i in range(0, 61)]
        lls = [ll(a) for a in grid]
        im = max(range(len(grid)), key=lambda i: lls[i])
        lo = min(a for a, l in zip(grid, lls) if l >= lls[im] - 1.92)
        hi = max(a for a, l in zip(grid, lls) if l >= lls[im] - 1.92)
        pit = []
        for c, v in dd:
            tot = c + sum(v)
            pit.append((sum(a for a in v if a < c) + rng.random() * c) / tot)
        rk = [(sum(1 for a in v if a < c) + rng.random()) / (1 + len(v)) for c, v in dd]
        print("   %-13s %5d choices: alpha %.2f (interval %.2f to %.2f, choices treated as independent); log2 likelihood per choice at alpha 0, 1, best: %.3f %.3f %.3f; mean PIT under alpha=1 %.3f; mean rank among siblings %.3f"
              % (BANDS[bi] if bi < 4 else "all depths", len(dd), grid[im], lo, hi, lls[0] / len(dd) / math.log(2), lls[20] / len(dd) / math.log(2), lls[im] / len(dd) / math.log(2), sum(pit) / len(pit), sum(rk) / len(rk)))


main()
