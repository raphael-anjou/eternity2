"""Kaplan-Meier medians of node counts per (source file set, n, c, solver).

Stdlib only. Run: python3 -I medians.py
KM median as in hard-ladder/README.md: the ceil(N/2)-th order statistic with
censored runs set to +infinity. Rows from the committed result files only.
Duplicate (solver, instance) rows across files are kept apart by source.
"""
import glob, json, math, os, sys
from collections import defaultdict

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "../../../../../.."))
EXP = os.path.join(ROOT, "research", "experiments")
SOURCES = {
    "hard-ladder/test": "hard-ladder/results/test/*/*.jsonl",
    "mrv-baseline/raw": "mrv-baseline/results/raw/*/*.jsonl",
    "cut-multiset/test": "cut-multiset-pruning/results/test/*/*.jsonl",
    "backdoor/test": "backdoor-branching/results/test/*/*.jsonl",
}

def km(vals, q=0.5):
    v = sorted(vals)
    k = max(1, math.ceil(len(v) * q))
    return v[k - 1]

def load():
    groups = defaultdict(dict)
    for src, pat in SOURCES.items():
        for f in sorted(glob.glob(os.path.join(EXP, pat))):
            tag = os.path.basename(f)[:-6]
            for line in open(f):
                r = json.loads(line)
                if r.get("error"):
                    continue
                cap = r["cap"]
                if src == "mrv-baseline/raw" and tag.startswith("ext"):
                    key = (src + "/ext", r["n"], r["c"], r["solver"], cap)
                else:
                    key = (src, r["n"], r["c"], r["solver"], cap)
                x = r["nodes"] if r["status"] == "solved" else math.inf
                groups[key][(r["seed"], r.get("run_seed", 0))] = (x, r["nodes"])
    return groups

def main():
    g = load()
    print("source\tn\tc\tsolver\tcap\tN\tsolved\tq25\tmedian\tq75\tmin_censored_nodes")
    for key in sorted(g):
        rows = list(g[key].values())
        vals = [x for x, _ in rows]
        cens = [n for x, n in rows if x == math.inf]
        f = lambda x: "inf" if x == math.inf else str(int(x))
        print("\t".join(map(str, key)) + "\t%d\t%d\t%s\t%s\t%s\t%s" % (
            len(vals), sum(1 for x in vals if x < math.inf),
            f(km(vals, .25)), f(km(vals, .5)), f(km(vals, .75)),
            min(cens) if cens else "-"))

if __name__ == "__main__":
    main()
