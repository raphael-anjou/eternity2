"""Claims A and F: independent first-moment model of row-major DFS against raw rows.

python3 -I a_dfs_model.py [run]
Without `run`: model (two variants) against every committed hp-dfs rung.
With `run`: also runs the referee's row-major DFS (enum.c, node for node equal to
hp-dfs on committed rows) on rungs where the committed rows are all censored:
7x7 c=6 (cap 2e8), 8x8 c=8 (cap 1.5e8), 9x9 c=11 (cap 2e8, A22 seeds 1..30 and the
pilot's seeds 101..108). Every run stops after a few seconds.
"""
import json, math, os, random, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rvlib as rv


def profile(n, c, cf=5):
    """Row-major: per cell (remaining*rotations, interior seams closed, frame seams closed)."""
    w = n - 2
    left = {0: w * w, 1: 4 * w, 2: 4}
    steps = []
    for y in range(n):
        for x in range(n):
            k = (x in (0, n - 1)) + (y in (0, n - 1))
            a = f = 0
            if x > 0:
                if y in (0, n - 1): f += 1
                else: a += 1
            if y > 0:
                if x in (0, n - 1): f += 1
                else: a += 1
            steps.append((left[k] * (4 if k == 0 else 1), a, f))
            left[k] -= 1
    return steps


def model(n, c, deplete, cf=5):
    """Returns (N, [b'_j], [w_j]): expected wrong consistent siblings of the planted piece at
    cell j, and the mean size of the subtree under one of them (itself included)."""
    st = profile(n, c, cf)
    N = n * n
    w = n - 2
    EI, EF = 2 * w * (w + 1), 4 * (n - 1)
    lnA = [0.0]
    for m, a, f in st:
        lnA.append(lnA[-1] + math.log(m) - a * math.log(c) - f * math.log(cf))
    qi = (EI / c - 1) / (EI - 1) if deplete else 1.0 / c
    qf = (EF / cf - 1) / (EF - 1) if deplete else 1.0 / cf
    bw, ws = [], []
    for j, (m, a, f) in enumerate(st):
        h = 4 if j == 0 else 1
        bw.append(max(m - h, 0) * max(qi, 1e-12) ** a * max(qf, 1e-12) ** f)
        ws.append(sum(math.exp(lnA[d] - lnA[j + 1]) for d in range(j + 1, N + 1)))
    return N, bw, ws, lnA


def poisson(rng, lam):
    if lam > 60:
        return max(0, int(round(rng.gauss(lam, math.sqrt(lam)))))
    k, p, L = 0, rng.random(), math.exp(-lam)
    while p > L:
        k += 1
        p *= rng.random()
    return k


def sample(n, c, deplete, rng, size=4001):
    N, bw, ws, _ = model(n, c, deplete)
    out = []
    for _ in range(size):
        t = float(N)
        for b, w_ in zip(bw, ws):
            if b <= 0:
                continue
            K = poisson(rng, b)
            if K:
                t += rng.randint(0, K) * w_
        out.append(t)
    out.sort()
    return out


def measured():
    """(n, c) -> list of (source, N, solved, KM median, cap) from raw rows."""
    rows = rv.load_rows(["hard-ladder/results/test/*/hp-dfs.jsonl", "mrv-baseline/results/raw/hp-dfs/*.jsonl",
                         "cut-multiset-pruning/results/test/*/hp-dfs.jsonl", "backdoor-branching/results/test/*/hp-dfs.jsonl"])
    g = {}
    for r in rows:
        if r["solver"] != "hp-dfs":
            continue
        src = r["_file"].split("/")[0]
        g.setdefault((r["n"], r["c"], src, r["cap"]), {})[r["seed"]] = r["nodes"] if r["status"] == "solved" else math.inf
    hp = json.load(open(os.path.join(rv.ROOT, "research/topics/hardness-peak/results/hardness-peak-8x8.json")))
    for r in hp["rows"]:
        g.setdefault((8, r["interiorColors"], "hardness-peak", 15000000), {})[r["seed"]] = r["placements"] if r["solved"] else math.inf
    out = {}
    for (n, c, src, cap), d in g.items():
        v = list(d.values())
        out.setdefault((n, c), []).append((src, len(v), sum(x < math.inf for x in v), rv.km_median(v), cap))
    return out


def frac_below(s, cap):
    import bisect
    return bisect.bisect_left(s, cap) / len(s)


def main():
    rng = random.Random(11)
    meas = measured()
    print("Every committed hp-dfs rung. Model median over 4001 value orders, no fitted parameter.")
    print("n c x | lnS naive, balanced | model median: no depletion, with depletion (ii) | measured: source solved/N KM median | ratio model/measured: no depletion, with depletion | model P(nodes < cap)")
    ratios = {False: [], True: []}
    for (n, c) in sorted(meas):
        x = c * c / (4.0 * (n - 2) ** 2)
        s0, s1 = sample(n, c, False, rng), sample(n, c, True, rng)
        m0, m1 = s0[len(s0) // 2], s1[len(s1) // 2]
        for src, N, sv, med, cap in sorted(meas[(n, c)]):
            if med < math.inf:
                r0, r1 = m0 / med, m1 / med
                tag = "%.2f, %.2f" % (r0, r1)
                if rv.lnS_naive(n, c) < 0:
                    ratios[False].append((r0, n, c, src)); ratios[True].append((r1, n, c, src))
            else:
                tag = "censored"
            print("%d %d %.2f | %.1f, %.1f | %.3g, %.3g | %s %d/%d %s | %s | %.2f" % (
                n, c, x, rv.lnS_naive(n, c), rv.lnS_balanced(n, c), m0, m1, src, sv, N,
                "inf" if med == math.inf else "%d" % med, tag, frac_below(s1, cap)))
    for dep in (False, True):
        r = sorted(ratios[dep])
        print("solved rungs with lnS(naive) < 0, depletion=%s: %d comparisons, ratio min %.2f max %.2f, geometric mean %.2f" % (
            dep, len(r), r[0][0], r[-1][0], math.exp(sum(math.log(v[0]) for v in r) / len(r))))
        bulge = [v for v in r if v[2] * v[2] / (4.0 * (v[1] - 2) ** 2) < 1]
        print("   of which x < 1 (an interior bulge exists): %d, min %.2f max %.2f, geometric mean %.2f" % (
            len(bulge), bulge[0][0], bulge[-1][0], math.exp(sum(math.log(v[0]) for v in bulge) / len(bulge))))

    if len(sys.argv) > 1 and sys.argv[1] == "run":
        print()
        print("Referee runs of row-major DFS on rungs whose committed rows are all censored")
        pilot = []
        pf = "<repo>/research/experiments/mrv-scaling-pilot/instances/plus2-test.jsonl"
        if os.path.exists(pf):
            for line in open(pf):
                r = json.loads(line)
                pilot.append((r["id"], r["n"], [tuple(p) for p in r["puzzle"]["pieces"]]))
        jobs = [("7x7 c=6 primary seeds 1..30", 7, 6, 2 * 10**8, [(a, b, p) for a, b, p, _ in rv.ladder_instances("primary", 7)]),
                ("8x8 c=8 primary seeds 1..30", 8, 8, 15 * 10**7, [(a, b, p) for a, b, p, _ in rv.ladder_instances("primary", 8)]),
                ("9x9 c=11 pilot seeds 101..108", 9, 11, 2 * 10**8, pilot),
                ("9x9 c=11 plus2 seeds 1..30", 9, 11, 2 * 10**8, [(a, b, p) for a, b, p, _ in rv.ladder_instances("plus2", 9)])]
        for label, n, c, cap, boards in jobs:
            t = time.time()
            res = rv.run_enum("first", cap, boards)
            dt = time.time() - t
            vals = [math.inf if r["capped"] else r["nodes"] for r in res]
            s1 = sample(n, c, True, rng)
            s0 = sample(n, c, False, rng)
            solved = sorted(v for v in vals if v < math.inf)
            med = rv.km_median(vals)
            print("%s: cap %.3g, solved %d/%d, KM median %s, model median %.3g (no depletion %.3g), model P(nodes < cap) %.2f (no depletion %.2f), %.0f s" % (
                label, cap, len(solved), len(vals), "above cap" if med == math.inf else "%d" % med,
                s1[len(s1) // 2], s0[len(s0) // 2], frac_below(s1, cap), frac_below(s0, cap), dt))
            print("   " + " ".join("%s:%s" % (r["tag"].split("-")[-1], "cens" if r["capped"] else r["nodes"]) for r in res))
            sys.stdout.flush()


if __name__ == "__main__":
    main()
