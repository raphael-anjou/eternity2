"""Analysis of every raw file in results/. Prints the tables of RESULTS.md and
writes results/summary.json. Stdlib only.

Run: python3 -I scripts/analyze.py
"""
import glob
import json
import math
import os
import statistics as st

EXP = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
R = os.path.join(EXP, "results")
T95 = {1: 12.706, 2: 4.303, 3: 3.182, 4: 2.776, 5: 2.571, 9: 2.262, 19: 2.093, 29: 2.045, 99: 1.984, 199: 1.972}
FRAMES_EXACT = 40519056449429419697492635191544381440
MODEL = {"five": 40.49, "centre": 47.14}
out = {}
# clue cells of the official CSV (x, y): depth in the top-left row scan of M1
CLUE_DEPTHS = {8 * 16 + 7, 13 * 16 + 2, 2 * 16 + 2, 13 * 16 + 13, 2 * 16 + 13}


def jl(path):
    return [json.loads(l) for l in open(path) if l.strip()]


def tq(df):
    return T95.get(df, 1.96)


def lg(x):
    return math.log10(x) if x > 0 else float("-inf")


# ---------------- gates ----------------
def check_rows(rows):
    """per-depth 3-se test and total error for check_config rows"""
    res = []
    for r in rows:
        if not r["exact_complete"]:
            res.append({"label": r["label"], "complete": False})
            continue
        ex, mean, sd, n = r["exact"], r["mean"], r["sd"], r["reps"]
        out3 = 0
        for k in range(1, len(ex)):
            se = sd[k] / math.sqrt(n)
            if se == 0:
                bad = abs(mean[k] - ex[k]) > 1e-9 * max(1, ex[k])
            else:
                bad = abs(mean[k] - ex[k]) > 3 * se
            out3 += bad
        T = sum(ex[1:])
        tots = r["totals"]
        m = st.mean(tots)
        s = st.stdev(tots)
        h = tq(n - 1) * s / math.sqrt(n)
        res.append({"label": r["label"], "complete": True, "depths": len(ex) - 1, "out3": out3, "T": T,
                    "mean_T": m, "err_log10": abs(lg(m / T)), "covered": (m - h) <= T <= (m + h)})
    return res


gates = {}
p = os.path.join(R, "gate", "gate-a.jsonl")
if os.path.exists(p):
    res = check_rows(jl(p))
    done = [x for x in res if x["complete"]]
    pairs = sum(x["depths"] for x in done)
    out3 = sum(x["out3"] for x in done)
    med = st.median(x["err_log10"] for x in done) if done else None
    g = {"instances": len(res), "complete": len(done), "pairs": pairs, "out3": out3,
         "share_out3": out3 / pairs if pairs else None, "median_abs_log10_total_err": med,
         "max_abs_log10_total_err": max(x["err_log10"] for x in done) if done else None,
         "T_range": [min(x["T"] for x in done), max(x["T"] for x in done)] if done else None}
    g["pass"] = bool(done) and len(done) == len(res) and g["share_out3"] <= 0.02 and med <= 0.05
    gates["a"] = g
    print("Gate (a), 30 hard-ladder 6x6, N=200, 200 reps:", json.dumps(g))

p = os.path.join(R, "gate", "gate-b.jsonl")
if os.path.exists(p):
    rows = jl(p)
    w60 = [10 ** r["log10_width"][60] if r["log10_width"][60] is not None else 0.0 for r in rows]
    m, s = st.mean(w60), st.stdev(w60)
    se = s / math.sqrt(len(w60))
    g = {"reps": len(w60), "mean": m, "se": se, "exact": float(FRAMES_EXACT), "ratio": m / FRAMES_EXACT,
         "z": (m - FRAMES_EXACT) / se, "log10_mean": lg(m), "log10_exact": lg(FRAMES_EXACT),
         "seconds_per_rep": st.mean(r["seconds"] for r in rows)}
    g["pass"] = abs(g["z"]) <= 3
    gates["b"] = g
    print("Gate (b), official frame in ring order, N=1e5:", json.dumps(g))

for np_ in (1000, 100):
    p = os.path.join(R, "gate", "gen-np%d.jsonl" % np_)
    if os.path.exists(p):
        res = check_rows(jl(p))
        done = [x for x in res if x["complete"]]
        errs = [x["err_log10"] for x in done]
        g = {"boards": len(res), "complete": len(done), "median_abs_log10_err": st.median(errs),
             "max_abs_log10_err": max(errs), "coverage": sum(x["covered"] for x in done) / len(done),
             "share_out3": sum(x["out3"] for x in done) / sum(x["depths"] for x in done),
             "T_range": [min(x["T"] for x in done), max(x["T"] for x in done)],
             "worst": max(done, key=lambda x: x["err_log10"])["label"]}
        g["pass_prereg_rule"] = g["median_abs_log10_err"] <= 0.05 and g["max_abs_log10_err"] <= 0.3 and g["coverage"] >= 0.8
        gates["gen_np%d" % np_] = g
        print("Secondary check, generated boards, N=%d:" % np_, json.dumps(g))
out["gates"] = gates

# ---------------- M1 ----------------
m1 = []
for f in sorted(f for f in glob.glob(os.path.join(R, "m1", "*.jsonl")) if not os.path.basename(f).startswith("m1b-")):
    for r in jl(f):
        first = min(r["unlock"])

        def shares(h):
            tot = sum(h)
            return {"nodes": tot,
                    "after_first_unlock": sum(h[first:]) / tot,
                    "d_ge_201": sum(h[201:]) / tot,
                    "last56_d_ge_200": sum(h[200:]) / tot,
                    "last40_d_ge_216": sum(h[216:]) / tot}
        row = {"label": r["label"], "seed": r["seed"], "first_unlock": first, "seconds": r["seconds"],
               "deepest": r["deepest"], "breaks_at_deepest": r["breaks_at_deepest"],
               "completions": r["completions"], "best_complete_score": r["best_complete_score"],
               "final": shares(r["hist"]),
               "entries_d201": r["hist"][201], "late_per_entry_201": (sum(r["hist"][201:]) / r["hist"][201]) if r["hist"][201] else None,
               "peak_depth": max(range(256), key=lambda d: r["hist"][d]), "checkpoints": {str(n): shares(h) for n, h in r["checkpoints"]}}
        m1.append(row)
out["m1"] = m1
if m1:
    print("\nM1 per run (share of nodes at depth >= first unlock, by budget)")
    print("%-15s %4s %8s %8s %8s %8s | %7s %7s %7s | %7s %5s %6s" % ("schedule", "seed", "1e7", "1e8", "1e9", "final", ">=201", "last56", "last40", "deepest", "brk", "s"))
    for r in m1:
        cp = r["checkpoints"]
        f = r["final"]
        print("%-15s %4d %8.1e %8.1e %8.1e %8.1e | %7.1e %7.1e %7.1e | %7d %5d %6.0f" % (
            r["label"], r["seed"], cp["10000000"]["after_first_unlock"], cp["100000000"]["after_first_unlock"],
            cp["1000000000"]["after_first_unlock"], f["after_first_unlock"], f["d_ge_201"], f["last56_d_ge_200"],
            f["last40_d_ge_216"], r["deepest"], r["breaks_at_deepest"], r["seconds"]))
    print("\nM1 summary per schedule: median [min, max] over seeds")
    summ = {}
    for lab in sorted({r["label"] for r in m1}):
        rs = [r for r in m1 if r["label"] == lab]
        d = {}
        for key, getter in (("1e9", lambda r: r["checkpoints"]["1000000000"]["after_first_unlock"]),
                            ("final", lambda r: r["final"]["after_first_unlock"]),
                            ("d>=201", lambda r: r["final"]["d_ge_201"]),
                            ("last56", lambda r: r["final"]["last56_d_ge_200"]),
                            ("last40", lambda r: r["final"]["last40_d_ge_216"]),
                            ("deepest", lambda r: r["deepest"]),
                            ("late_per_entry_201", lambda r: r["late_per_entry_201"] or 0.0),
                            ("peak_depth", lambda r: r["peak_depth"])):
            v = [getter(r) for r in rs]
            d[key] = [st.median(v), min(v), max(v)]
        if any("10000000000" in r["checkpoints"] for r in rs):
            v = [r["checkpoints"]["10000000000"]["after_first_unlock"] for r in rs]
            d["1e10_checkpoint"] = [st.median(v), min(v), max(v)]
        med9, med10 = d["1e9"][0], d["final"][0]
        d["budget_ratio_final_over_1e9"] = med10 / med9 if med9 > 0 else None
        d["n"] = len(rs)
        summ[lab] = d
        print(lab, json.dumps(d))
    out["m1_summary"] = summ


# ---------------- M1b: whole-tree late share by SMC, and its gate ----------------
p = os.path.join(R, "gate", "m1b-val.jsonl")
if os.path.exists(p) and os.path.getsize(p) > 0:
    rows = jl(p)
    pairs = out3 = 0
    errs = []
    for r in rows:
        ex, mean, sd, n, d0 = r["exact"], r["mean"], r["sd"], r["reps"], r["d0"]
        for k in range(d0 + 1, 257):
            if ex[k] == 0 and mean[k] == 0:
                continue
            pairs += 1
            se = sd[k] / math.sqrt(n)
            out3 += (abs(mean[k] - ex[k]) > 3 * se) if se > 0 else (abs(mean[k] - ex[k]) > 1e-9 * max(1, ex[k]))
        T = sum(ex[d0 + 1:])
        errs.append(abs(lg(sum(mean[d0 + 1:]) / T)))
    g = {"prefixes": len(rows), "pairs": pairs, "out3": out3, "share_out3": out3 / pairs if pairs else None,
         "median_abs_log10_total_err": st.median(errs), "max_abs_log10_total_err": max(errs),
         "subtree_sizes": [sum(r["exact"][r["d0"] + 1:]) for r in rows],
         "deepest_exact": [max(k for k, v in enumerate(r["exact"]) if v) for r in rows]}
    g["pass"] = len(rows) == 10 and g["share_out3"] <= 0.02 and g["median_abs_log10_total_err"] <= 0.05
    out["gates"]["m1b"] = g
    print("\nM1b gate:", json.dumps(g))

m1b = {}
for f in sorted(glob.glob(os.path.join(R, "m1", "m1b-*.jsonl"))):
    rows = jl(f)
    if not rows:
        continue
    lab = rows[0]["label"]
    first = min(rows[0]["unlock"])
    ent = [r["entries"] for r in rows]
    mean = [st.mean(e[k] for e in ent) for k in range(257)]
    # nodes are entries at non-clue depths; clue depths carry pass-through values
    clue = CLUE_DEPTHS
    nodes = [0.0 if k in clue or k == 256 else mean[k] for k in range(257)]
    tot = sum(nodes)
    per_rep_share = []
    for e in ent:
        nd = [0.0 if k in clue or k == 256 else e[k] for k in range(257)]
        per_rep_share.append(sum(nd[first:]) / sum(nd) if sum(nd) > 0 else None)
    pk = max(range(257), key=lambda k: nodes[k])
    d = {"reps": len(rows), "np": rows[0]["np"], "first_unlock": first, "clue_depths": sorted(clue),
         "log10_total_nodes": lg(tot), "share_after_first_unlock": sum(nodes[first:]) / tot,
         "share_d_ge_201": sum(nodes[201:]) / tot, "share_last56": sum(nodes[200:]) / tot, "share_last40": sum(nodes[216:]) / tot,
         "per_rep_share_after_first_unlock": per_rep_share, "peak_depth": pk, "log10_peak": lg(nodes[pk]),
         "log10_completions": lg(mean[256]), "extinction_depths": [max(k for k, v in enumerate(e) if v > 0) for e in ent],
         "seconds_per_rep": st.mean(r["seconds"] for r in rows),
         "log10_mean_entries": [round(lg(x), 3) if x > 0 else None for x in mean]}
    m1b[lab] = d
    print("\nM1b %s:" % lab, json.dumps({k: v for k, v in d.items() if k != "log10_mean_entries"}))
out["m1b"] = m1b

# ---------------- M2 ----------------
m2 = {}
for f in sorted([f for f in glob.glob(os.path.join(R, "m2", "*-np1e*.jsonl")) if "paint" not in f]):
    rows = jl(f)
    if not rows:
        continue
    lab = rows[0]["label"]
    tots = [r["total"] for r in rows]
    n = len(tots)
    m = st.mean(tots)
    s = st.stdev(tots) if n > 1 else float("nan")
    h = tq(n - 1) * s / math.sqrt(n) if n > 1 else float("nan")
    L = len(rows[0]["log10_width"])
    meanw = [st.mean(10 ** r["log10_width"][k] if r["log10_width"][k] is not None else 0.0 for r in rows) for k in range(L)]
    peak = max(range(L), key=lambda k: meanw[k])
    d = {"reps": n, "np": rows[0]["np"], "mean_total": m, "log10_mean_total": lg(m),
         "t95": [m - h, m + h], "log10_t95": [lg(m - h), lg(m + h)],
         "log10_each": [lg(t) for t in tots], "span_decades": lg(max(tots)) - lg(min(tots)),
         "mean_total_to_175": st.mean(r["total_to_175"] for r in rows),
         "peak_depth": peak, "log10_peak_width": lg(meanw[peak]),
         "extinction_depths": [r["extinction_depth"] for r in rows],
         "seconds_per_rep": st.mean(r["seconds"] for r in rows),
         "log10_mean_width": [round(lg(x), 3) if x > 0 else None for x in meanw]}
    d["log10_mean_total_to_175"] = lg(d["mean_total_to_175"])
    # diversity at the diagnostic depths, mean over reps
    div = {}
    for r in rows:
        for dep, anc, dist, ss in r["diversity_depth_anc_distinct_sumsq"]:
            div.setdefault("%d<-%d" % (dep, anc), []).append((dist, ss))
    d["diversity"] = {k: [st.mean(a for a, _ in v), st.mean(b for _, b in v)] for k, v in div.items()}
    zf = {}
    for k in (32, 64, 96, 128, 158, 175, 200, 230):
        vals = [r["levels_E_M_zero_mean_var"][k - 1] for r in rows if len(r["levels_E_M_zero_mean_var"]) >= k]
        if vals:
            zf[k] = [st.mean(v[2] for v in vals), st.mean(v[3] for v in vals), st.mean(v[4] for v in vals)]
    d["zero_frac_mean_var_ext_at"] = zf
    key = lab.split("-")[0]
    if key in MODEL:
        d["model_log10"] = MODEL[key]
        d["minus_model"] = d["log10_mean_total"] - MODEL[key]
    m2[lab] = d
    print("\nM2 %s:" % lab, json.dumps({k: v for k, v in d.items() if k != "log10_mean_width"}))
out["m2"] = m2

p = os.path.join(R, "m2", "paint-np1e5.jsonl")
if os.path.exists(p) and os.path.getsize(p) > 0:
    rows = jl(p)
    by = {}
    for r in rows:
        by.setdefault(r["label"], []).append(r["total"])
    vals = sorted(lg(st.mean(v)) for v in by.values())
    q = lambda a: vals[min(len(vals) - 1, max(0, int(round(a * (len(vals) - 1)))))]
    nd = {"instances": len(vals), "log10_totals": vals, "median": st.median(vals), "p2.5": q(0.025), "p97.5": q(0.975),
          "sd": st.stdev(vals) if len(vals) > 1 else None}
    out["m2_null"] = nd
    print("\nM2 null (painted from the official census):", json.dumps(nd))

for f in sorted(glob.glob(os.path.join(R, "m2", "frame-*-enum.jsonl"))):
    for r in jl(f):
        print("\nFrame stand-in %s: free=%d complete=%s total=%d (log10 %.2f) seconds=%.1f deepest nonzero=%d" % (
            r["label"], r["free"], r["complete"], r["total"], lg(r["total"]) if r["total"] else float("-inf"), r["seconds"],
            max(k for k, w in enumerate(r["widths"]) if w > 0)))
        out.setdefault("frames", {})[r["label"]] = {k: r[k] for k in ("free", "complete", "total", "seconds", "widths")}

json.dump(out, open(os.path.join(R, "summary.json"), "w"), indent=1)

# ---------------- markdown tables for RESULTS.md ----------------
if m1:
    print("\nMARKDOWN M1 per run")
    print("| schedule | seed | 1e7 | 1e8 | 1e9 | 1e10 checkpoint | final | d >= 201 | last 56 | last 40 | deepest | breaks | s |")
    print("|---|---|---|---|---|---|---|---|---|---|---|---|---|")
    g = lambda cp, k: ("%.1e" % cp[k]["after_first_unlock"]) if k in cp else ""
    for r in sorted(m1, key=lambda r: (r["label"], r["seed"])):
        cp, f = r["checkpoints"], r["final"]
        print("| %s | %d | %s | %s | %s | %s | %.1e | %.1e | %.1e | %.1e | %d | %d | %.0f |" % (
            r["label"], r["seed"], g(cp, "10000000"), g(cp, "100000000"), g(cp, "1000000000"), g(cp, "10000000000"),
            f["after_first_unlock"], f["d_ge_201"], f["last56_d_ge_200"], f["last40_d_ge_216"], r["deepest"], r["breaks_at_deepest"], r["seconds"]))
    print("\nMARKDOWN M1 summary")
    fm = lambda v, p="%.1e": (p + " [" + p + ", " + p + "]") % tuple(v)
    for lab, d in sorted(out["m1_summary"].items()):
        print("| %s | %d | %s | %s | %s | %s | %s | %s | %s |" % (lab, d["n"], fm(d["final"]), fm(d["d>=201"]), fm(d["last56"]), fm(d["last40"]),
              fm(d["deepest"], "%d"), fm(d["late_per_entry_201"], "%.0f"), fm(d["peak_depth"], "%d")))
