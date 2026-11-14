"""Claims D and F: node ratios of plain row-major DFS to bb-mrv and scb, from raw rows.

python3 -I d_ratios.py [run]
Paired by instance. A censored run enters at its cap (so a ratio with a censored DFS is a lower
bound, with a censored propagating solver an upper bound); pairs censored on both sides are dropped.
With `run`: validates enum.c against every committed solved hp-dfs row, and adds referee DFS
runs at 7x7 c=6 and at 9x9 c=11 (pilot seeds), each a few seconds.
"""
import json, math, os, random, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rvlib as rv

rows = rv.load_rows(["mrv-baseline/results/raw/*/*.jsonl", "hard-ladder/results/test/*/*.jsonl", "backdoor-branching/results/test/*/*.jsonl"])
data = {}   # (n, c, solver) -> {seed: (nodes, solved)}; the largest cap wins
for r in rows:
    key = (r["n"], r["c"], r["solver"])
    d = data.setdefault(key, {})
    old = d.get(r["seed"])
    val = (r["nodes"], r["status"] == "solved", r["cap"])
    if old is None or (not old[1] and val[2] > old[2]) or (val[1] and not old[1]):
        d[r["seed"]] = val

rng = random.Random(3)


def paired(n, c, a, b):
    A, B = data.get((n, c, a), {}), data.get((n, c, b), {})
    seeds = sorted(set(A) & set(B))
    logs, lower, upper, both = [], 0, 0, 0
    for s in seeds:
        (na, sa, _), (nb, sb, _) = A[s], B[s]
        if not sa and not sb:
            both += 1
            continue
        lower += (not sa)
        upper += (not sb)
        logs.append(math.log(na / nb))
    if not logs:
        return "%d seeds, all censored on both sides" % len(seeds)
    logs.sort()
    med = math.exp(logs[len(logs) // 2] if len(logs) % 2 else 0.5 * (logs[len(logs) // 2 - 1] + logs[len(logs) // 2]))
    bs = []
    for _ in range(4000):
        smp = sorted(rng.choice(logs) for _ in logs)
        bs.append(smp[len(smp) // 2])
    bs.sort()
    ka = rv.km_median([x[0] if x[1] else math.inf for x in A.values()])
    kb = rv.km_median([x[0] if x[1] else math.inf for x in B.values()])
    return "%d pairs (%d dropped, %d lower bounds, %d upper bounds): median paired ratio %.2f, bootstrap 95%% [%.2f, %.2f]; KM medians %s / %s = %s" % (
        len(logs), both, lower, upper, med, math.exp(bs[100]), math.exp(bs[3899]), ka, kb,
        "%.2f" % (ka / kb) if ka < math.inf and kb < math.inf else "n/a")


if len(sys.argv) > 1 and sys.argv[1] == "run":
    ok = bad = 0
    for ladder, sizes in (("primary", (6,)), ("secondary", (6, 7)), ("plus2", (6, 7)), ("8-c10", (8,))):
        for n in sizes:
            inst = rv.ladder_instances(ladder, n)
            c = inst[0][3]["c"]
            ref = data.get((n, c, "hp-dfs"), {})
            todo = [(a, b, p) for a, b, p, r in inst if r["seed"] in ref and ref[r["seed"]][1]]
            seeds = [r["seed"] for a, b, p, r in inst if r["seed"] in ref and ref[r["seed"]][1]]
            for s, r in zip(seeds, rv.run_enum("first", 2 * 10**6, todo)):
                if r["nodes"] == ref[s][0]:
                    ok += 1
                else:
                    bad += 1
    print("enum.c against committed solved hp-dfs rows: %d identical node counts, %d different" % (ok, bad))
    for label, n, c, cap, boards in (
            ("7x7 c=6", 7, 6, 2 * 10**8, [(r["seed"], b, p) for a, b, p, r in rv.ladder_instances("primary", 7)]),):
        res = rv.run_enum("first", cap, [(str(s), b, p) for s, b, p in boards])
        data[(n, c, "hp-dfs")] = {int(r["tag"]): (r["nodes"], not r["capped"], cap) for r in res}
    pf = "<repo>/research/experiments/mrv-scaling-pilot/instances/plus2-test.jsonl"
    pilot = [json.loads(l) for l in open(pf)]
    res = rv.run_enum("first", 2 * 10**8, [(str(r["seed"]), 9, [tuple(p) for p in r["puzzle"]["pieces"]]) for r in pilot])
    data[(9, 11, "hp-dfs")] = {int(r["tag"]): (r["nodes"], not r["capped"], 2 * 10**8) for r in res}
    import glob
    for sol in ("bb-mrv", "scb-gacolor_ac3_random"):
        data[(9, 11, sol)] = {}
    base = "<repo>/research/experiments/mrv-scaling-pilot/results"
    for f in glob.glob(base + "/raw/wave*.jsonl") + glob.glob(base + "/remote/*/raw/wave*.jsonl"):
        for line in open(f):
            r = json.loads(line)
            data[(9, 11, r["solver"])][r["seed"]] = (r["nodes"], r["status"] == "solved", r["cap"])
    print("9x9 c=11 pilot seeds, DFS (cap 2e8) | bb-mrv (cap 1e8) | scb (cap 1e8):")
    for s in range(101, 109):
        f = lambda v: "no row" if v is None else ("%d" % v[0] if v[1] else "above %.0e" % v[2])
        print("   s%d: %s | %s | %s" % (s, f(data[(9, 11, "hp-dfs")].get(s)), f(data[(9, 11, "bb-mrv")].get(s)), f(data[(9, 11, "scb-gacolor_ac3_random")].get(s))))

print()
for title, rungs in (("+2 colour track", ((6, 7), (7, 8), (8, 10), (9, 11))), ("primary rungs", ((6, 5), (7, 6)))):
    print(title)
    for n, c in rungs:
        x = c * c / (4.0 * (n - 2) ** 2)
        print(" n=%d c=%d x=%.2f" % (n, c, x))
        for a, b in (("hp-dfs", "bb-mrv"), ("hp-dfs", "scb-gacolor_ac3_random"), ("scb-gacolor_ac3_random", "bb-mrv"), ("mrv-bf", "bb-mrv"), ("scb-gacolor_ac3_random", "scb-ad")):
            if (n, c, a) in data and (n, c, b) in data and set(data[(n, c, a)]) & set(data[(n, c, b)]):
                print("   %s / %s: %s" % (a, b, paired(n, c, a, b)))
