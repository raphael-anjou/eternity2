"""Model (no free parameter) against measured row-major DFS (hp-dfs).

Stdlib only. Sources: the ladder result rows (via medians.py) and the 8x8
colour sweep research/topics/hardness-peak/results/hardness-peak-8x8.json
(placements, budget 1.5e7 placements).
"""
import json, math, os, random, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from model import Profile, critical_c, owen
import medians

def fmt(x):
    return "inf" if x == math.inf else "%.3g" % x

def main():
    rng = random.Random(7)
    g = medians.load()
    rows = {}
    for (src, n, c, solver, cap), d in g.items():
        if solver != "hp-dfs" or "ext" in src:
            continue
        vals = [x for x, _ in d.values()]
        rows.setdefault((n, c), []).append((src, len(vals), sum(v < math.inf for v in vals), medians.km(vals), cap))
    hp = json.load(open(os.path.join(medians.ROOT, "research/topics/hardness-peak/results/hardness-peak-8x8.json")))
    by = {}
    for r in hp["rows"]:
        by.setdefault(r["interiorColors"], []).append(r["placements"] if r["solved"] else math.inf)
    for c, v in by.items():
        rows.setdefault((8, c), []).append(("hardness-peak-8x8", len(v), sum(x < math.inf for x in v), medians.km(v), 1.5e7))
    print("n\tc\tOwen\tc_crit\tx=c^2/4P\tlnS\tpeak_d/N\tmodel_mean\tmodel_q25\tmodel_median\tmodel_q75\tmeasured (source: solved/N, KM median)")
    for (n, c) in sorted(rows):
        p = Profile(n, c, "rowmajor")
        s = p.first_solution_sample(rng, 2001)
        d, _ = p.peak()
        meas = "; ".join("%s: %d/%d, %s" % (src, sv, N, fmt(m)) for src, N, sv, m, cap in sorted(rows[(n, c)]))
        print("%d\t%d\t%.2f\t%.2f\t%.2f\t%.1f\t%.2f\t%s\t%s\t%s\t%s\t%s" % (
            n, c, owen(n), critical_c(n), c * c / (4.0 * (n - 2) ** 2), p.lnA[-1], d / p.N,
            fmt(p.first_solution_mean()), fmt(s[len(s) // 4]), fmt(s[len(s) // 2]), fmt(s[3 * len(s) // 4]), meas))

if __name__ == "__main__":
    main()
