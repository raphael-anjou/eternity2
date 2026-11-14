"""learning-vs-propagation: parse raw outputs (../results/raw/machine-a/, ../results/raw/machine-b/) and print the per-size tables. Stdlib only.
TOY boards. usage: python3 -I analyze.py"""
import glob, json, math, os, random, re, statistics
HERE = os.path.dirname(os.path.abspath(__file__))
R = {}  # (arm, n, seed) -> dict(work=..., sols=..., cens=0/1, first=..., extra)


def clasp(txt, err):
    g = lambda k: re.search(r"^c %s\s*:\s*(\d+)(\+?)" % k, txt, re.M)
    m = g("Models")
    if not m:
        return None
    lem = g("Lemmas")
    t = re.search(r"^c CPU Time\s*:\s*([\d.]+)", txt, re.M)
    cens = int(m.group(2) == "+" or "INTERRUPTED" in err or "INTERRUPTED" in txt)
    return dict(sols=int(m.group(1)), conflicts=int(g("Conflicts").group(1)), choices=int(g("Choices").group(1)),
                lemmas=int(lem.group(1)) if lem else None, sec=float(t.group(1)) if t else None, cens=cens)


for d in ("../results/raw/machine-a", "../results/raw/machine-b"):
    for f in glob.glob(os.path.join(HERE, d, "*.out")):
        base = os.path.basename(f)[:-4]
        arm, n, seed = base.rsplit("_", 2)
        n, seed = int(n), int(seed)
        txt = open(f).read()
        err = open(f[:-4] + ".err").read() if os.path.exists(f[:-4] + ".err") else ""
        r = None
        if arm == "T":
            v = txt.split()
            r = dict(nodes=int(v[0]), dead=int(v[1]), sols=int(v[2]), first=int(v[3]), cens=0) if len(v) == 4 else dict(cens=1)
        elif arm in ("AC", "WD"):
            if txt.strip():
                j = json.loads(txt.strip().splitlines()[-1])
                r = dict(tried=j["tried"], sols=j["sols"], sec=j["sec"], cens=int(j["censored"] != 0))
            else:
                r = dict(cens=1)
        elif arm == "K":
            if txt.strip():
                j = json.loads(txt.strip().splitlines()[-1])
                r = dict(conflicts=j["conflicts"], decisions=j["decisions"], sols=j["sols"], first=(j["first"] or {}).get("conflicts"), sec=j["sec"], cens=j["censored"])
            else:
                r = dict(cens=1)
        else:
            r = clasp(txt, err) or dict(cens=1)
        R[(arm, n, seed)] = r

random.seed(12345)


def med(v):
    return statistics.median(v) if v else float("nan")


def boot(v, reps=10000):
    if len(v) < 2:
        return (float("nan"), float("nan"))
    ms = sorted(med([random.choice(v) for _ in v]) for _ in range(reps))
    return ms[int(0.025 * reps)], ms[int(0.975 * reps) - 1]


def boot_diff(a, b, reps=10000):
    if len(a) < 2 or len(b) < 2:
        return (float("nan"), float("nan"))
    ms = sorted(med([random.choice(b) for _ in b]) - med([random.choice(a) for _ in a]) for _ in range(reps))
    return ms[int(0.025 * reps)], ms[int(0.975 * reps) - 1]


SIZES = sorted({k[1] for k in R})
WORK = {"AC": "tried", "WD": "tried", "K": "conflicts", "U0": "conflicts", "U1": "conflicts", "L0": "conflicts", "L1": "conflicts", "C": "conflicts"}
NAMES = {"AC": "row scan + arc consistency (dyn.c AC ROW), branches tried", "WD": "weighted degree + arc consistency (dyn.c AC WDEG), branches tried",
         "U0": "clasp, learning off, static order, unit propagation only: conflicts", "U1": "clasp, learning on, same static order: conflicts",
         "L0": "clasp, learning off, failed-literal lookahead: conflicts", "L1": "clasp, learning on, same lookahead heuristic: conflicts",
         "C": "clasp default (learning): conflicts", "K": "CaDiCaL 1.5.3: conflicts"}


def ratio(arm, n, s, key=None, first=False):
    t, a = R.get(("T", n, s)), R.get((arm + ("f" if first and arm not in ("K",) else ""), n, s))
    if not t or t.get("cens") or not a or "sols" not in a and not a.get("cens"):
        return None
    if first:
        if arm == "K":
            w = a.get("first")
        else:
            w = a.get("conflicts") if a.get("sols", 0) >= 1 else None
        if not w:
            return None
        return (math.log2(t["first"] / max(w, 1)), 0)
    key = key or WORK[arm]
    if key not in a:
        return None
    return (math.log2(t["nodes"] / max(a[key], 1)), a["cens"])


out = {}
print("# TOY boards only. log2(row-scan tree nodes / work), exhaustive enumeration unless marked first")
print("# sols check: boards where some arm's (uncensored) solution count differs from the tree's")
for (arm, n, s), r in sorted(R.items()):
    t = R.get(("T", n, s))
    if arm.endswith("f") or arm == "T" or r.get("cens") or not t or t.get("cens"):
        continue
    if r.get("sols") != t["sols"]:
        print("  MISMATCH", arm, n, s, r.get("sols"), t["sols"])
for n in SIZES:
    t = [R[k] for k in R if k[0] == "T" and k[1] == n]
    nodes = [math.log2(x["nodes"]) for x in t if not x.get("cens")]
    if not nodes:
        continue
    print("\n## n=%d  tree: %d boards, log2 nodes median %.2f (range %.2f to %.2f), solutions median %s" % (n, len(nodes), med(nodes), min(nodes), max(nodes), med([x["sols"] for x in t if not x.get("cens")])))
    for arm in ("AC", "WD", "U0", "U1", "L0", "L1", "C", "K"):
        v = [ratio(arm, n, s) for s in range(1, 11)]
        v = [x for x in v if x]
        if not v:
            continue
        vals = [x[0] for x in v]
        nc = sum(x[1] for x in v)
        lo, hi = boot(vals)
        out[(arm, n)] = vals
        out[(arm, n, "cens")] = nc
        print("  %-3s %-75s boards %2d censored %d  median %+.2f  range %+.2f..%+.2f  boot95 %+.2f..%+.2f" % (arm, NAMES[arm], len(vals), nc, med(vals), min(vals), max(vals), lo, hi))
    for arm in ("U0",):
        v = [ratio(arm, n, s, key="choices") for s in range(1, 11)]
        v = [x[0] for x in v if x]
        if v:
            print("  U0  decisions instead of conflicts                                       boards %2d median %+.2f" % (len(v), med(v)))
    for arm in ("U0", "U1", "L0", "L1", "C", "K"):
        v = [ratio(arm, n, s, first=True) for s in range(1, 11)]
        v = [x[0] for x in v if x]
        if v:
            out[(arm, n, "first")] = v
            print("  %-3s first solution: boards %2d median %+.2f range %+.2f..%+.2f" % (arm, len(v), med(v), min(v), max(v)))


def paired(a, b, n, first=False):
    """per-board difference ratio(a) - ratio(b) on boards where both exist; censored flag if either censored"""
    res = []
    for s in range(1, 11):
        x, y = ratio(a, n, s, first=first), ratio(b, n, s, first=first)
        if x and y:
            res.append((x[0] - y[0], x[1] or y[1]))
    return res


print("\n# Bits due to learning: paired per board, ratio(learning arm) - ratio(control)")
PAIRS = [("U1", "U0", "same solver and static order, learning on vs off (cleanest)"), ("L1", "L0", "same solver and lookahead heuristic, learning on vs off"),
         ("K", "U0", "CaDiCaL minus clasp, learning off"), ("C", "U0", "clasp default minus clasp learning off"),
         ("K", "AC", "pre-stated test: CaDiCaL lead over row scan + arc consistency"), ("U1", "AC", "pre-stated test: clasp static-order learning lead over row scan + AC"),
         ("C", "AC", "pre-stated test: clasp default lead over row scan + AC"), ("U0", "AC", "clasp no-learning over row scan + AC (propagation strength)")]
SUM = {}
for a, b, label in PAIRS:
    print("\n%s - %s: %s" % (a, b, label))
    for first in (False, True):
        for n in SIZES:
            d = paired(a, b, n, first)
            if not d:
                continue
            v = [x[0] for x in d]
            lo, hi = boot(v)
            SUM[(a, b, n, first)] = v
            print("  n=%d %s boards %2d censored %d  median %+.2f  range %+.2f..%+.2f  boot95 %+.2f..%+.2f" % (n, "first" if first else "exhaust", len(v), sum(x[1] for x in d), med(v), min(v), max(v), lo, hi))
        for n1, n2 in zip(SIZES, SIZES[1:]):
            if (a, b, n1, first) in SUM and (a, b, n2, first) in SUM:
                A, B = SUM[(a, b, n1, first)], SUM[(a, b, n2, first)]
                lo, hi = boot_diff(A, B)
                print("  trend %s n=%d->%d: change of median %+.2f  boot95 %+.2f..%+.2f" % ("first" if first else "exhaust", n1, n2, med(B) - med(A), lo, hi))
