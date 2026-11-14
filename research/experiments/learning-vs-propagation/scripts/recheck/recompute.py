"""Independent recomputation for learning-vs-propagation. TOY boards only. Stdlib only.
Parses the raw outputs independently of analyze.py and prints paired leads with bounds.
usage: python3 -I recompute.py"""
import json, math, os, re, statistics as st
H = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "results", "raw")
R = {}
def num(txt, key):
    m = re.search(r"^c %s\s*:\s*(\d+)(\+?)" % key, txt, re.M)
    return (int(m.group(1)), m.group(2) == "+") if m else None
for d in ("machine-a", "machine-b"):
    for fn in sorted(os.listdir(os.path.join(H, d))):
        if not fn.endswith(".out"):
            continue
        arm, n, s = fn[:-4].rsplit("_", 2)
        k = (arm, int(n), int(s))
        assert k not in R, k
        txt = open(os.path.join(H, d, fn)).read()
        if not txt.strip():
            R[k] = None
            continue
        if arm == "T":
            v = [int(x) for x in txt.split()]
            R[k] = dict(work=v[0], dead=v[1], sols=v[2], cut=False)
        elif arm in ("AC", "WD"):
            j = json.loads(txt.strip().splitlines()[-1])
            R[k] = dict(work=j["tried"], dead=j["tried"] - j["alive"], sols=j["sols"], cut=j["censored"] != 0, sec=j["sec"])
        elif arm == "K":
            j = json.loads(txt.strip().splitlines()[-1])
            R[k] = dict(work=j["conflicts"], sols=j["sols"], cut=j["censored"] != 0, sec=j["sec"])
        else:
            mo, co = num(txt, "Models"), num(txt, "Conflicts")
            cut = mo[1] or "TIME LIMIT" in txt or "INTERRUPTED" in txt
            va = re.search(r"^c Variables\s*:\s*(\d+)", txt, re.M)
            tm = re.search(r"^c Time\s*:\s*([\d.]+)", txt, re.M)
            R[k] = dict(work=co[0], sols=mo[0], cut=cut, vars=int(va.group(1)), sec=float(tm.group(1)), lem=num(txt, "Lemmas")[0])
ARMS = ["AC", "WD", "U0", "U1", "L0", "L1", "C", "K"]
print("== solution counts of finished runs against the tree; variables across clasp arms")
bad = 0
for (arm, n, s), r in R.items():
    if r and not r["cut"] and not arm.endswith("f") and r["sols"] != R[("T", n, s)]["sols"]:
        bad += 1; print("  MISMATCH", arm, n, s)
    if r and r["cut"] and not arm.endswith("f") and r["sols"] > R[("T", n, s)]["sols"]:
        bad += 1; print("  CUT RUN OVERCOUNTS", arm, n, s)
for n in (5, 6, 7):
    for s in range(1, 11):
        vs = {R[(a, n, s)]["vars"] for a in ("U0", "U1", "L0", "L1", "C") if R.get((a, n, s))}
        if len(vs) != 1:
            bad += 1; print("  VARS DIFFER", n, s, vs)
print("  problems:", bad)
print("  learning-off arms with lemmas:", [(k) for k, r in R.items() if r and k[0] in ("U0", "L0", "U0f", "L0f") and r["lem"]])
print("== finished / cut / no counts at 7x7")
for a in ["T"] + ARMS:
    rs = [R.get((a, 7, s)) for s in range(1, 11)]
    print("  %-2s fin %2d cut %2d nocount %d" % (a, sum(1 for r in rs if r and not r["cut"]), sum(1 for r in rs if r and r["cut"]), sum(1 for r in rs if r is None)))
def lead(a, b, n, s, key="work"):
    """log2(work b / work a): bits by which a beats b. Returns (value, kind): exact, ub (a cut), lb (b cut), none."""
    x, y = R.get((a, n, s)), R.get((b, n, s))
    if not x or not y:
        return (None, "none")
    v = math.log2(y[key] / x["work"]) if key == "work" or b in ("AC", "WD") else None
    kind = "exact" if not x["cut"] and not y["cut"] else "ub" if x["cut"] and not y["cut"] else "lb" if y["cut"] and not x["cut"] else "none"
    return (v, kind)
def medbounds(vals):
    """vals: list of (v, kind). Bounds on the median over all 10 boards."""
    lo = [v if k in ("exact", "lb") else -math.inf for v, k in vals]
    hi = [v if k in ("exact", "ub") else math.inf for v, k in vals]
    return st.median(lo), st.median(hi)
for a, b in (("K", "AC"), ("C", "AC"), ("U1", "AC"), ("U1", "U0"), ("L1", "L0"), ("K", "U0"), ("C", "U0"), ("WD", "AC"), ("U0", "AC")):
    print("== lead of %s over %s, log2(work %s / work %s)" % (a, b, b, a))
    m6 = None
    for n in (5, 6, 7):
        vals = [lead(a, b, n, s) for s in range(1, 11)]
        ex = [v for v, k in vals if k == "exact"]
        lo, hi = medbounds(vals)
        line = "  n=%d exact %2d ub %d lb %d none %d | median of exact %s | all-10 median in [%+.2f, %+.2f]" % (
            n, len(ex), sum(k == "ub" for _, k in vals), sum(k == "lb" for _, k in vals), sum(k == "none" for _, k in vals),
            "%+.2f" % st.median(ex) if ex else "none", lo, hi)
        if n == 6:
            m6 = st.median(ex)
        if n == 7:
            line += " | rise from 6x6 in [%+.2f, %+.2f]" % (lo - m6, hi - m6)
            print(line)
            print("     per board:", " ".join("%d:%s%s" % (s + 1, "%+.2f" % v if v is not None else "?", {"exact": "", "ub": "<", "lb": ">", "none": "?"}[k]) for s, (v, k) in enumerate(vals)))
            for s in range(1, 11):
                e6 = lead(a, b, 6, s)[0]
            # same boards finished at 7: compare with the 6x6 values of the largest 6x6 trees is not possible (other boards)
        else:
            print(line)
print("== like for like: dead ends against conflicts (AC failed branches = tried - alive; tree dead ends)")
for n in (5, 6, 7):
    k = [math.log2(R[("AC", n, s)]["dead"] / R[("K", n, s)]["work"]) for s in range(1, 11) if R.get(("AC", n, s)) and R.get(("K", n, s)) and not R[("K", n, s)]["cut"]]
    t = [math.log2(R[("AC", n, s)]["work"] / R[("AC", n, s)]["dead"]) for s in range(1, 11) if R.get(("AC", n, s))]
    u = [math.log2(R[("T", n, s)]["work"] / R[("T", n, s)]["dead"]) for s in range(1, 11)]
    print("  n=%d K over AC in failed branches, exact pairs %d: median %+.2f | AC tried/failed %.2f bits | tree nodes/dead %.2f bits" % (n, len(k), st.median(k), st.median(t), st.median(u)))
print("== seconds, log2(sec control / sec learner), exact pairs; and 7x7 board 5")
for a, b in (("K", "AC"), ("U1", "U0"), ("C", "AC")):
    for n in (6, 7):
        v = [math.log2(R[(b, n, s)]["sec"] / R[(a, n, s)]["sec"]) for s in range(1, 11) if lead(a, b, n, s)[1] == "exact"]
        print("  %s vs %s n=%d pairs %d median %s" % (a, b, n, len(v), "%+.2f" % st.median(v) if v else "none"))
print("  board 7_5:", {a: (R.get((a, 7, 5)) or {}).get("work") for a in ["T"] + ARMS}, {a: (R.get((a, 7, 5)) or {}).get("cut") for a in ARMS}, "sols", R[("T", 7, 5)]["sols"])
print("  7x7 tree log2 nodes and sols:", [(s, round(math.log2(R[("T", 7, s)]["work"]), 2), R[("T", 7, s)]["sols"]) for s in range(1, 11)])
print("  6x6 tree log2 nodes and K-AC:", [(s, round(math.log2(R[("T", 6, s)]["work"]), 1), R[("T", 6, s)]["sols"], round(lead("K", "AC", 6, s)[0], 2)) for s in range(1, 11)])
print("  conflicts per second, U1 runs at 6x6 and 7x7:", [round(st.median(R[("U1", n, s)]["work"] / max(R[("U1", n, s)]["sec"], 1e-9) for s in range(1, 11))) for n in (6, 7)])
