"""learning-real-prefixes: parse the raw outputs and print every table. Stdlib only.
usage: python3 -I analyze.py raw_dir [raw_dir ...]     (each raw_dir holds <prefix>.<arm>.out/.err and runs.jsonl)
From this folder: python3 -I analyze.py ../results/raw/machine-c ../results/raw/machine-b ../results/raw/machine-a
Writes per_prefix.out.tsv into ../results/analysis/. Rules: see ../notes/plan.md."""
import json, math, os, random, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ARMS = ["T", "AC", "K", "C", "U0"]
DEPTHS = [112, 104, 96, 88]
INF = float("inf")
R, EXT, DEFECT = {}, {}, []


def clasp(txt):
    g = lambda k: re.search(r"^c %s\s*:\s*(\d+)(\+?)" % k, txt, re.M)
    m, c = g("Models"), g("Conflicts")
    if not m or not c:
        return None
    t = re.search(r"^c Time\s*:\s*([\d.]+)s \(Solving: ([\d.]+)s", txt, re.M)
    cpu = re.search(r"^c CPU Time\s*:\s*([\d.]+)", txt, re.M)
    cut = int(m.group(2) == "+" or "INTERRUPTED" in txt or "TIME LIMIT" in txt)
    return dict(work=int(c.group(1)), sols=int(m.group(1)), cut=cut, sec=float(t.group(2)), total=float(t.group(1)), cpu=float(cpu.group(1)) if cpu else None)


for d in sys.argv[1:]:
    for line in open(os.path.join(d, "runs.jsonl")):
        j = json.loads(line)
        EXT[(j["prefix"], j["arm"])] = j
    for f in sorted(os.listdir(d)):
        if not f.endswith(".out"):
            continue
        pid, arm = f[:-4].split(".")
        txt = open(os.path.join(d, f)).read()
        r = None
        try:
            if arm in ("T", "AC", "K"):
                j = json.loads(txt.strip().splitlines()[-1])
                if arm == "T":
                    r = dict(work=j["nodes"], sols=j["sols"], cut=int(j["censored"] != 0), sec=j["wall"], cpu=j["cpu"])
                elif arm == "AC":
                    r = dict(work=j["tried"], failed=j["failed"], sols=j["sols"], cut=int(j["censored"] != 0), sec=j["wall"], cpu=j["cpu"])
                else:
                    r = dict(work=j["conflicts"], sols=j["sols"], cut=int(j["censored"] != 0 or j["final"] == 0), sec=j["wall"], cpu=j["cpu"], load=j["load_wall"], clauses=j["clauses"])
            else:
                r = clasp(txt)
        except (ValueError, IndexError, KeyError):
            r = None
        if r is None:
            DEFECT.append((pid, arm, d))
            continue
        r["dir"] = d
        R[(pid, arm)] = r

PIDS = sorted({p for p, _ in R} | {p for p, _ in EXT}, key=lambda p: (-int(p[1:].split("_")[0]), p))
depth = lambda p: int(p[1:].split("_")[0])
random.seed(20261114)


def med(v):
    v = sorted(v)
    n = len(v)
    if not n:
        return float("nan")
    return v[n // 2] if n % 2 else (v[n // 2 - 1] if v[n // 2 - 1] == v[n // 2] else (v[n // 2 - 1] + v[n // 2]) / 2)


def lead(pid, x, y, key="work", ykey=None):
    """(low, high) of log2(Y / X); a cut run's quantity is a lower bound."""
    a, b = R.get((pid, x)), R.get((pid, y))
    if not a or not b:
        return None
    if key == "work":
        va, vb = a["work"] + 1, b[ykey or "work"] + 1
    else:
        va, vb = max(a[key], 0.01), max(b[key], 0.01)
    v = math.log2(vb / va)
    return (-INF if a["cut"] else v, INF if b["cut"] else v)


def summary(pairs):
    """median interval and bootstrap 95% on each end"""
    lo, hi = [p[0] for p in pairs], [p[1] for p in pairs]
    n = len(pairs)
    bl, bh = [], []
    for _ in range(10000):
        idx = [random.randrange(n) for _ in range(n)]
        bl.append(med([lo[i] for i in idx]))
        bh.append(med([hi[i] for i in idx]))
    bl.sort()
    bh.sort()
    return med(lo), med(hi), bl[249], bh[9749], bl, bh


def fmt(x):
    return "%+.2f" % x if abs(x) != INF else ("+inf" if x > 0 else "-inf")


def iv(lo, hi):
    return fmt(lo) if lo == hi else "[%s, %s]" % (fmt(lo), fmt(hi))


print("runs parsed: %d; outputs with no counts (defects): %d %s" % (len(R), len(DEFECT), DEFECT))
missing = [(p, a) for p in PIDS for a in ARMS if (p, a) not in R]
print("missing runs:", len(missing), missing[:20])
print("\n== agreement on the number of completions (finished runs only)")
bad = 0
for p in PIDS:
    s = {R[(p, a)]["sols"] for a in ARMS if (p, a) in R and not R[(p, a)]["cut"]}
    if len(s) > 1:
        bad += 1
        print("  DISAGREE", p, {a: R[(p, a)]["sols"] for a in ARMS if (p, a) in R})
print("  prefixes with disagreement: %d of %d" % (bad, len(PIDS)))

print("\n== finished / cut per arm and depth; completions; hosts")
for d in DEPTHS:
    ps = [p for p in PIDS if depth(p) == d]
    if not ps:
        continue
    row = []
    for a in ARMS:
        fin = sum(1 for p in ps if (p, a) in R and not R[(p, a)]["cut"])
        cut = sum(1 for p in ps if (p, a) in R and R[(p, a)]["cut"])
        row.append("%s %d/%d" % (a, fin, cut))
    sols = [R[(p, "T")]["sols"] for p in ps if (p, "T") in R and not R[(p, "T")]["cut"]]
    hosts = {}
    for p in ps:
        h = EXT.get((p, "T"), {}).get("host", "?")
        hosts[h] = hosts.get(h, 0) + 1
    print("  depth %d (%d prefixes): finished/cut %s | prefixes with a completion: %d, completions in all: %d | %s" % (d, len(ps), "  ".join(row), sum(1 for s in sols if s), sum(sols), hosts))

print("\n== size of the work per arm and depth: log2(count + 1), quantiles min q25 median q75 max (cut runs at their count, a lower bound)")
for d in DEPTHS:
    ps = [p for p in PIDS if depth(p) == d]
    for a in ARMS + ["ACfailed"]:
        v = sorted(math.log2((R[(p, "AC")]["failed"] if a == "ACfailed" else R[(p, a)]["work"]) + 1) for p in ps if (p, a.replace("failed", "")) in R)
        if v:
            q = lambda f: v[min(len(v) - 1, int(f * (len(v) - 1) + 0.5))]
            print("  depth %d %-8s n %2d: %5.1f %5.1f %5.1f %5.1f %5.1f   log2 of the sum %5.1f" % (d, a, len(v), v[0], q(.25), q(.5), q(.75), v[-1], math.log2(sum(2 ** x for x in v))))

PAIRS = [("K", "AC", None, "K over AC (branches tried)  [stop line pair]"), ("K", "AC", "failed", "K over AC (failed branches)"), ("C", "AC", None, "C over AC (tried)"),
         ("C", "AC", "failed", "C over AC (failed)"), ("U0", "AC", None, "U0 over AC (tried)"), ("U0", "AC", "failed", "U0 over AC (failed)"), ("K", "U0", None, "K over U0"), ("C", "U0", None, "C over U0"),
         ("AC", "T", None, "AC over T"), ("K", "T", None, "K over T"), ("C", "T", None, "C over T"), ("U0", "T", None, "U0 over T")]
for key, title in (("work", "COUNTS: median over prefixes of log2(work Y / work X), X over Y; interval when a run was cut; (bootstrap 95%)"),
                   ("sec", "SECONDS, search phase, wall: median of log2(seconds Y / seconds X); positive = X faster"),
                   ("cpu", "PROCESSOR SECONDS as each program reports them (clasp: whole process; K: search phase)")):
    print("\n== " + title)
    for x, y, yk, name in PAIRS:
        if key != "work" and yk:
            continue
        S = {}
        cells = []
        for d in DEPTHS:
            pr = [lead(p, x, y, key, yk) for p in PIDS if depth(p) == d]
            pr = [q for q in pr if q]
            if len(pr) < 2:
                continue
            S[d] = summary(pr)
            nb = sum(1 for q in pr if q[0] != q[1])
            cells.append("d%d %s (%s to %s) n%d bounded %d" % (d, iv(S[d][0], S[d][1]), fmt(S[d][2]), fmt(S[d][3]), len(pr), nb))
        print("  %-42s %s" % (name, " | ".join(cells)))
        if 104 in S and 88 in S:
            a, b = S[104], S[88]
            dl = sorted(b[4][i] - a[5][i] for i in range(10000))
            dh = sorted(b[5][i] - a[4][i] for i in range(10000))
            f = lambda u: u if u == u else float("nan")
            print("  %-42s   rise 104 -> 88 (median at 88 minus median at 104): %s  (bootstrap 95%% %s to %s)" % ("", iv(f(b[0] - a[1]), f(b[1] - a[0])), fmt(dl[249]), fmt(dh[9749])))
        if 112 in S and 88 in S:
            a, b = S[112], S[88]
            f = lambda u: u if u == u else float("nan")
            print("  %-42s   rise 112 -> 88: %s" % ("", iv(f(b[0] - a[1]), f(b[1] - a[0]))))

print("\n== sums (the mean cost): log2(sum of Y / sum of X) per depth; '>=' or '<=' when a cut run is in a sum")
for key in ("work", "sec"):
    for x, y, yk, name in PAIRS:
        if yk:
            continue
        cells = []
        for d in DEPTHS:
            ps = [p for p in PIDS if depth(p) == d and (p, x) in R and (p, y) in R]
            if not ps:
                continue
            g = lambda a: sum((R[(p, a)]["work"] + 1) if key == "work" else max(R[(p, a)]["sec"], 0.01) for p in ps)
            cx, cy = any(R[(p, x)]["cut"] for p in ps), any(R[(p, y)]["cut"] for p in ps)
            cells.append("d%d %s%+.2f" % (d, "?? " if cx and cy else "<= " if cx else ">= " if cy else "", math.log2(g(y) / g(x))))
        print("  %-7s %-14s %s" % (key, x + " over " + y, " | ".join(cells)))

print("\n== seconds per arm and depth: median and sum of search wall seconds; sum of outer wall and of outer processor seconds (whole process)")
tot_w = tot_c = 0.0
for d in DEPTHS:
    ps = [p for p in PIDS if depth(p) == d]
    for a in ARMS + ["ENC"]:
        e = [EXT[(p, a)] for p in ps if (p, a) in EXT]
        v = [R[(p, a)]["sec"] for p in ps if (p, a) in R]
        ew, ec = sum(j["wall_ext"] for j in e), sum(j.get("cpu_user", 0) + j.get("cpu_sys", 0) for j in e)
        tot_w += ew
        tot_c += ec
        print("  depth %d %-3s search median %9.3f sum %10.1f | outer wall sum %10.1f  outer processor sum %10.1f  killed by backstop %d" % (d, a, med(v) if v else float("nan"), sum(v), ew, ec, sum(1 for j in e if j.get("killed"))))
print("  total outer wall %.2f core-hours, outer processor %.2f core-hours" % (tot_w / 3600, tot_c / 3600))

print("\n== per-prefix extremes of the stop line pair (K over AC tried), three lowest and three highest per depth")
for d in DEPTHS:
    pr = [(lead(p, "K", "AC"), p) for p in PIDS if depth(p) == d]
    pr = sorted((q for q in pr if q[0]), key=lambda q: (q[0][0] if q[0][0] != -INF else q[0][1]))
    for q, p in pr[:3] + pr[-3:]:
        print("  %s %s  T %d AC %d K %d C %d U0 %d  sec AC %.2f K %.2f  cut %s" % (p, iv(*q), R[(p, "T")]["work"] if (p, "T") in R else -1, R[(p, "AC")]["work"], R[(p, "K")]["work"],
              R[(p, "C")]["work"] if (p, "C") in R else -1, R[(p, "U0")]["work"] if (p, "U0") in R else -1, R[(p, "AC")]["sec"], R[(p, "K")]["sec"], [a for a in ARMS if (p, a) in R and R[(p, a)]["cut"]]))

with open(os.path.join(HERE, "..", "results", "analysis", "per_prefix.out.tsv"), "w") as f:
    f.write("prefix\tdepth\thost\tsols\t" + "\t".join("%s_work\t%s_cut\t%s_sec\t%s_cpu_outer\t%s_wall_outer" % (a, a, a, a, a) for a in ARMS) + "\tAC_failed\tK_load_sec\tcnf_clauses\torder\n")
    for p in PIDS:
        t = R.get((p, "T"))
        row = [p, str(depth(p)), EXT.get((p, "T"), {}).get("host", "?"), str(t["sols"]) if t else "?"]
        for a in ARMS:
            r, e = R.get((p, a)), EXT.get((p, a), {})
            row += [str(r["work"]), str(r["cut"]), "%.4f" % r["sec"]] if r else ["?", "?", "?"]
            row += ["%.3f" % (e.get("cpu_user", 0) + e.get("cpu_sys", 0)), "%.3f" % e.get("wall_ext", -1)]
        ac, k = R.get((p, "AC")), R.get((p, "K"))
        order = sorted((EXT[(p, a)]["pos"], a) for a in ARMS if (p, a) in EXT)
        row += [str(ac["failed"]) if ac else "?", "%.2f" % k["load"] if k else "?", str(k["clauses"]) if k else "?", ",".join(a for _, a in order)]
        f.write("\t".join(row) + "\n")

# ---- sections added after the run (descriptive; nothing above was changed) ----
SOLV = ["AC", "K", "C", "U0"]
print("\n== prefixes with zero or near-zero work, per depth (every one of AC, K, C, U0 at count 0; at count <= 1; at count <= 100; tree T <= 100 nodes)")
for d in DEPTHS:
    ps = [p for p in PIDS if depth(p) == d and all((p, a) in R for a in ARMS)]
    if not ps:
        continue
    z = lambda lim: sum(1 for p in ps if all(R[(p, a)]["work"] <= lim for a in SOLV))
    k0 = sum(1 for p in ps if R[(p, "K")]["work"] == 0 and R[(p, "AC")]["work"] == 0)
    print("  depth %d: all four at 0: %d; all four <= 1: %d; all four <= 100: %d; T <= 100: %d; AC and K both at 0 (lead exactly 0): %d; of %d" % (
        d, z(0), z(1), z(100), sum(1 for p in ps if R[(p, "T")]["work"] <= 100), k0, len(ps)))

print("\n== NOT PLANNED, descriptive: K over AC (tried) on the prefixes where AC or K did more than 100 units of work: n, median interval, min, max, share positive")
for d in DEPTHS:
    pr = [lead(p, "K", "AC") for p in PIDS if depth(p) == d and (p, "AC") in R and (p, "K") in R and max(R[(p, "AC")]["work"], R[(p, "K")]["work"]) > 100]
    if pr:
        hi = [q[1] for q in pr]
        print("  depth %d: n %d, median %s, upper values min %s max %s, strictly positive (upper value) %d" % (d, len(pr), iv(med([q[0] for q in pr]), med(hi)), fmt(min(hi)), fmt(max(hi)), sum(1 for h in hi if h > 0)))
    for x, y in (("C", "AC"), ("U0", "AC")):
        pr = [lead(p, x, y) for p in PIDS if depth(p) == d and (p, x) in R and (p, y) in R and max(R[(p, x)]["work"], R[(p, y)]["work"]) > 100]
        if pr:
            print("      %s over %s: n %d, median %s" % (x, y, len(pr), iv(med([q[0] for q in pr]), med([q[1] for q in pr]))))

print("\n== heavy prefixes: the eight largest trees per depth (counts; * = cut at 1200 s, count is a lower bound; search wall seconds; host)")
for d in DEPTHS:
    ps = sorted((p for p in PIDS if depth(p) == d and (p, "T") in R), key=lambda p: -R[(p, "T")]["work"])[:8]
    tot = sum(R[(p, "T")]["work"] for p in PIDS if depth(p) == d and (p, "T") in R)
    for p in ps:
        c = lambda a: "%d%s" % (R[(p, a)]["work"], "*" if R[(p, a)]["cut"] else "")
        print("  %s T %s (%.0f%% of the depth's tree nodes) AC %s failed %d K %s C %s U0 %s | sec T %.2f AC %.1f K %.1f C %.1f U0 %.1f | K over AC %s | %s" % (
            p, c("T"), 100.0 * R[(p, "T")]["work"] / tot, c("AC"), R[(p, "AC")]["failed"], c("K"), c("C"), c("U0"),
            R[(p, "T")]["sec"], R[(p, "AC")]["sec"], R[(p, "K")]["sec"], R[(p, "C")]["sec"], R[(p, "U0")]["sec"], iv(*lead(p, "K", "AC")), EXT[(p, "T")]["host"]))

print("\n== every cut run")
for (p, a), r in sorted(R.items()):
    if r["cut"]:
        print("  %s %s count >= %d after %.0f s search (T %d, AC %d)" % (p, a, r["work"], r["sec"], R[(p, "T")]["work"], R[(p, "AC")]["work"]))

print("\n== rates on prefixes with more than 10 s of search: median units of work per second, per arm (all depths)")
for a in ARMS:
    v = [r["work"] / r["sec"] for (p, x), r in R.items() if x == a and r["sec"] > 10]
    if v:
        print("  %-3s n %3d median %.0f per second" % (a, len(v), med(v)))

pd = os.path.join(HERE, "..", "results", "raw", "pilot-corrected")
if os.path.isdir(pd) and "pilot-corrected" not in " ".join(sys.argv[1:]):
    print("\n== pilot (machine-c) against the full run, same prefix, counts of the five arms")
    same = diff = 0
    for f in sorted(os.listdir(pd)):
        if not f.endswith(".out"):
            continue
        pid, arm = f[:-4].split(".")
        txt = open(os.path.join(pd, f)).read()
        if arm in ("T", "AC", "K"):
            j = json.loads(txt.strip().splitlines()[-1])
            w = j["nodes"] if arm == "T" else j["tried"] if arm == "AC" else j["conflicts"]
        else:
            w = clasp(txt)["work"]
        r = R.get((pid, arm))
        if r and not r["cut"]:
            if r["work"] == w:
                same += 1
            else:
                diff += 1
                print("  differs: %s %s pilot %d full %d (full run host %s)" % (pid, arm, w, r["work"], EXT[(pid, arm)]["host"]))
    hosts = sorted({EXT[(f[:-4].split(".")[0], "T")]["host"] for f in os.listdir(pd) if f.endswith(".T.out")})
    print("  identical %d, different %d; full-run hosts of the pilot prefixes: %s" % (same, diff, hosts))
