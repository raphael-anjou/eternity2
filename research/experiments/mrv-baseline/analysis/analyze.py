#!/usr/bin/env python3
"""A22 analysis (PREREG.md). Stdlib only.

python3 analysis/analyze.py                 # write results/summary.json, pairs.csv, sets.csv, tables.md
python3 analysis/analyze.py --need-extended # print Q2 sizes with < 10/30 solved by both at 1e6
"""
import csv
import json
import math
import random
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent.parent
LADDER = HERE.parent / "hard-ladder"
sys.path.insert(0, str(LADDER / "analysis"))
from analyze import km_quantile, upper_median, wilson, pct  # noqa: E402

RAW = HERE / "results" / "raw"
OUT = HERE / "results"
INF = math.inf
B = 10_000
SEED = 20261111
BB, SCB, HP = "bb-mrv", "scb-gacolor_ac3_random", "hp-dfs"
ABL = ["mrv-nohs", "mrv-bf", "mrv-rand", "scb-ad"]
PARENT = {"mrv-nohs": BB, "mrv-bf": BB, "mrv-rand": BB, "scb-ad": SCB, "scb-fork-off": SCB}
Q2_SIZES = [6, 7, 8, 9, 10]


def load(arm, name):
    p = RAW / arm / f"{name}.jsonl"
    if not p.exists():
        return []
    return [json.loads(l) for l in p.read_text().splitlines() if l.strip()]


def sets_for(arm):
    """Named instance sets -> rows, for one arm."""
    s = {}
    r67 = load(arm, "q1-r6r7")
    s["Q1 R6 6x6 c=5"] = [r for r in r67 if r["n"] == 6]
    s["Q1 R7 7x7 c=6"] = [r for r in r67 if r["n"] == 7]
    s["Q1 R8 8x8 c=10 s21-50"] = load(arm, "q1-r8a") + load(arm, "q1-r8b")
    q2 = load(arm, "q2-plus2")
    for n in (6, 7, 9, 10):
        s[f"Q2 n={n} c={ {6: 7, 7: 8, 9: 11, 10: 12}[n] }"] = [r for r in q2 if r["n"] == n]
    s["Q2 n=8 c=10"] = s["Q1 R8 8x8 c=10 s21-50"]
    ext = load(arm, "ext-r8a") + load(arm, "ext-r8b")
    if ext:
        s["EXT 1e7 n=8 c=10"] = ext
    for n in (6, 7, 9, 10):
        e = load(arm, f"ext-n{n}")
        if e:
            s[f"EXT 1e7 n={n}"] = e
    return {k: v for k, v in s.items() if v}


def q2_key(n):
    return "Q2 n=8 c=10" if n == 8 else f"Q2 n={n} c={ {6: 7, 7: 8, 9: 11, 10: 12}[n] }"


def solved(r):
    return r["status"] == "solved"


def time_censored(r):
    return r["status"] == "censored" and r["nodes_counted"] < r["cap"]


def lr(c, b, log=math.log):
    """PROTOCOL.md G1 pair score; a time-censored baseline vs solved candidate scores 0 (PREREG)."""
    cs, bs = solved(c), solved(b)
    if cs and bs:
        return log(c["nodes"] / b["nodes"])
    if cs and not bs:
        return 0.0 if time_censored(b) else log(c["nodes"] / b["cap"])
    if not cs and bs:
        return INF
    return 0.0


def boot_median(vals, rng):
    n = len(vals)
    meds = [upper_median([vals[rng.randrange(n)] for _ in range(n)]) for _ in range(B)]
    return pct(meds, 0.025), pct(meds, 0.975)


def pair(cand, base):
    bm = {r["instance"]: r for r in base}
    ps = [(c, bm[c["instance"]]) for c in cand if c["instance"] in bm]
    if not ps:
        return None
    rng = random.Random(SEED)
    l = [lr(c, b) for c, b in ps]
    med = upper_median(l)
    lo, hi = boot_median(l, rng)
    w = [c["wall_ms"] / b["wall_ms"] for c, b in ps]
    rng = random.Random(SEED)
    wlo, whi = boot_median(w, rng)
    ratio = math.exp(med) if med != INF else INF
    return {
        "pairs": len(ps),
        "median_ln_ratio": med,
        "node_ratio": ratio,
        "ci_ln": [lo, hi],
        "g1_pass": ratio <= 0.5 and hi < 0,
        "both_censored": sum(1 for c, b in ps if not solved(c) and not solved(b)),
        "cand_only_solved": sum(1 for c, b in ps if solved(c) and not solved(b)),
        "base_only_solved": sum(1 for c, b in ps if not solved(c) and solved(b)),
        "both_solved": sum(1 for c, b in ps if solved(c) and solved(b)),
        "time_censored": sum(time_censored(c) + time_censored(b) for c, b in ps),
        "wall_ratio_median": upper_median(w),
        "wall_ratio_ci": [wlo, whi],
    }


def describe(rows, arm):
    n = len(rows)
    k = sum(solved(r) for r in rows)
    nodes = [r["nodes"] if solved(r) else INF for r in rows]
    wall = [r["wall_ms"] for r in rows]
    cnt = sum(r["nodes_counted"] for r in rows)
    return {
        "N": n,
        "solved": k,
        "wilson95": list(wilson(k, n)),
        "km_q25": km_quantile(nodes, 0.25),
        "km_median": km_quantile(nodes, 0.5),
        "km_q75": km_quantile(nodes, 0.75),
        "median_wall_ms": upper_median(wall),
        "total_wall_s": sum(wall) / 1000,
        "nodes_per_s": cnt / (sum(wall) / 1000) if sum(wall) else None,
        # bb arms: Model::propagate calls; scb arms: one place_and_propagate per
        # node by construction, native counter = pruned domain rows.
        "propagate_calls_per_node": (1.0 if arm.startswith("scb") else sum(r["propagation"] for r in rows) / cnt) if cnt else None,
        "scb_pruned_rows_per_node": (sum(r["propagation"] for r in rows) / cnt if arm.startswith("scb") else None) if cnt else None,
        "time_censored": sum(time_censored(r) for r in rows),
        "errors": sum(r["status"] == "error" for r in rows),
    }


def slope(xs, ys):
    if any(y == INF for y in ys):
        return INF
    mx, my = sum(xs) / len(xs), sum(ys) / len(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)


def q2_slope(bb, scb, ext=False, sizes=Q2_SIZES):
    per = {}
    for n in sizes:
        k = q2_key(n)
        ek = f"EXT 1e7 n={n}" + (" c=10" if n == 8 else "")
        if ext and ek in bb and ek in scb:
            k2 = ek
        else:
            k2 = k
        bm = {r["instance"]: r for r in scb.get(k2, [])}
        per[n] = (k2, [lr(c, bm[c["instance"]], math.log10) for c in bb.get(k2, []) if c["instance"] in bm])
    if any(not v[1] for v in per.values()):
        return None
    xs = sizes
    meds = [upper_median(per[n][1]) for n in xs]
    s = slope(xs, meds)
    rng = random.Random(SEED)
    reps = []
    for _ in range(B):
        m = []
        for n in xs:
            v = per[n][1]
            m.append(upper_median([v[rng.randrange(len(v))] for _ in range(len(v))]))
        reps.append(slope(xs, m))
    lo, hi = pct(reps, 0.025), pct(reps, 0.975)
    return {
        "sets": {n: per[n][0] for n in xs},
        "median_log10_ratio": dict(zip(xs, meds)),
        "slope": s,
        "ci": [lo, hi],
        "inf_replicates": sum(r == INF for r in reps),
        "claim": "the gain grows with n" if hi < 0 else "no evidence of a growing gain",
    }


def need_extended():
    bb, scb = sets_for(BB), sets_for(SCB)
    out = []
    for n in Q2_SIZES:
        k = q2_key(n)
        if k not in bb or k not in scb:
            raise SystemExit(f"missing 1e6 rows for {k}")
        sm = {r["instance"]: r for r in scb[k]}
        both = sum(1 for r in bb[k] if solved(r) and r["instance"] in sm and solved(sm[r["instance"]]))
        if both < 10:
            out.append(str(n))
    print(" ".join(out))


def checks():
    """Wrapper scb / hp-dfs rows vs committed hard-ladder rows (nodes, status); fork-off vs scb."""
    res = {}
    for arm, f in ((SCB, "scb-gacolor_ac3_random"), (HP, "hp-dfs")):
        ref = {}
        for p in (LADDER / "results" / "test" / "primary").glob(f"{f}.jsonl"):
            for l in p.read_text().splitlines():
                if l.strip():
                    r = json.loads(l)
                    if r.get("solver") == f and r.get("ladder") == "primary":
                        ref[r["instance"]] = r
        mine = load(arm, "q1-r6r7")
        cmp = [(r, ref[r["instance"]]) for r in mine if r["instance"] in ref]
        res[f"{arm} vs committed hard-ladder"] = {
            "compared": len(cmp),
            "identical_nodes_status": sum(a["nodes"] == b["nodes"] and a["status"] == b["status"] for a, b in cmp),
        }
    off, base = load("scb-fork-off", "q1-r6r7"), {r["instance"]: r for r in load(SCB, "q1-r6r7")}
    cmp = [(r, base[r["instance"]]) for r in off if r["instance"] in base]
    res["scb-fork-off vs scb"] = {
        "compared": len(cmp),
        "identical_nodes_status": sum(a["nodes"] == b["nodes"] and a["status"] == b["status"] for a, b in cmp),
    }
    a16 = {}
    for lad in ("primary", "8-c10"):
        p = HERE.parent / "backdoor-branching" / "results" / "test" / lad / "bb-mrv.jsonl"
        if p.exists():
            for l in p.read_text().splitlines():
                if l.strip():
                    r = json.loads(l)
                    a16[r["instance"]] = r
    mine = []
    for f in ("bb-mrv-a16-primary", "bb-mrv-a16-8-c10"):
        p = HERE / "results" / "check" / f"{f}.jsonl"
        if p.exists():
            mine += [json.loads(l) for l in p.read_text().splitlines() if l.strip()]
    cmp = [(r, a16[r["instance"]]) for r in mine if r["instance"] in a16]
    res["bb-mrv vs committed A16 bb-mrv (seeds 1..20, implementation check only)"] = {
        "compared": len(cmp),
        "identical_nodes_status": sum(a["nodes"] == b["nodes"] and a["status"] == b["status"] for a, b in cmp),
    }
    return res


def fmt(x, nd=0):
    if x is None:
        return "-"
    if x == INF:
        return "inf"
    return f"{x:,.{nd}f}"


def main():
    arms = [BB, SCB, HP] + ABL
    data = {a: sets_for(a) for a in arms}
    summary = {"describe": {}, "pairs": {}, "checks": checks()}
    lines = ["## Per set and arm", "", "| set | arm | solved/N | KM q25 | KM median | KM q75 | median wall ms | nodes/s | propagate calls/node | scb pruned rows/node | time-cens |", "|---|---|---|---|---|---|---|---|---|---|---|"]
    rows_csv = []
    for a in arms:
        for k, rows in data[a].items():
            d = describe(rows, a)
            summary["describe"].setdefault(k, {})[a] = d
            rows_csv.append({"set": k, "arm": a, **{x: d[x] for x in d if x != "wilson95"}})
    for k in summary["describe"]:
        for a, d in summary["describe"][k].items():
            lines.append(f"| {k} | {a} | {d['solved']}/{d['N']} | {fmt(d['km_q25'])} | {fmt(d['km_median'])} | {fmt(d['km_q75'])} | {fmt(d['median_wall_ms'])} | {fmt(d['nodes_per_s'])} | {fmt(d['propagate_calls_per_node'], 2)} | {fmt(d['scb_pruned_rows_per_node'], 1)} | {d['time_censored']} |")
    comps = [(BB, SCB), (HP, SCB), (BB, HP)] + [(x, PARENT[x]) for x in ABL] + [(x, SCB) for x in ABL[:3]]
    lines += ["", "## Paired comparisons (candidate vs baseline)", "", "| set | candidate | baseline | pairs | node ratio | 95% CI | G1 | both-cens | cand-only | base-only | wall ratio | wall 95% CI |", "|---|---|---|---|---|---|---|---|---|---|---|---|"]
    pcsv = []
    for c, b in comps:
        for k in data[c]:
            if k not in data[b]:
                continue
            p = pair(data[c][k], data[b][k])
            if not p:
                continue
            summary["pairs"].setdefault(k, {})[f"{c} vs {b}"] = p
            ex = "" if (c, b) in ((BB, SCB), (HP, SCB), (BB, HP)) else " (exploratory)"
            ci = [math.exp(x) if x != INF else INF for x in p["ci_ln"]]
            lines.append(f"| {k} | {c}{ex} | {b} | {p['pairs']} | {fmt(p['node_ratio'], 3)} | [{fmt(ci[0], 3)}, {fmt(ci[1], 3)}] | {'PASS' if p['g1_pass'] else 'fail'} | {p['both_censored']} | {p['cand_only_solved']} | {p['base_only_solved']} | {fmt(p['wall_ratio_median'], 3)} | [{fmt(p['wall_ratio_ci'][0], 3)}, {fmt(p['wall_ratio_ci'][1], 3)}] |")
            pcsv.append({"set": k, "candidate": c, "baseline": b, **{x: p[x] for x in p}})
    s1 = q2_slope(data[BB], data[SCB])
    s2 = q2_slope(data[BB], data[SCB], ext=True)
    summary["q2_slope_binding_1e6"] = s1
    summary["q2_slope_secondary_ext"] = s2
    # Exploratory (not preregistered): sizes where at least 10/30 pairs are both solved.
    ok = []
    for n in Q2_SIZES:
        k = q2_key(n)
        if k in data[BB] and k in data[SCB]:
            sm = {r["instance"]: r for r in data[SCB][k]}
            if sum(1 for r in data[BB][k] if solved(r) and r["instance"] in sm and solved(sm[r["instance"]])) >= 10:
                ok.append(n)
    s3 = q2_slope(data[BB], data[SCB], sizes=ok) if len(ok) >= 3 else None
    summary["q2_slope_exploratory_informative_sizes"] = s3
    r8 = summary["pairs"].get("Q1 R8 8x8 c=10 s21-50", {}).get(f"{BB} vs {SCB}")
    if r8:
        kill = (not r8["g1_pass"]) or r8["wall_ratio_median"] > 1
        summary["kill"] = {
            "g1_pass_r8": r8["g1_pass"],
            "wall_ratio_median_r8": r8["wall_ratio_median"],
            "adopted": not kill,
        }
    lines += ["", "## Q2 slope (median paired log10 node ratio bb-mrv/scb vs n)", ""]
    for name, s in (("binding (cap 1e6)", s1), ("secondary (1e7 where run)", s2), ("exploratory, sizes with >= 10 both-solved pairs", s3)):
        if s:
            meds = ", ".join(f"n={n}: {fmt(v, 3)}" for n, v in s["median_log10_ratio"].items())
            lines.append(f"- {name}: {meds}; slope {fmt(s['slope'], 4)} per n, 95% CI [{fmt(s['ci'][0], 4)}, {fmt(s['ci'][1], 4)}], inf replicates {s['inf_replicates']}; claim: {s['claim']}")
    lines += ["", "## Checks", ""] + [f"- {k}: {v['identical_nodes_status']}/{v['compared']} identical" for k, v in summary["checks"].items()]
    if "kill" in summary:
        lines += ["", f"Kill rule at R8: G1 {'PASS' if summary['kill']['g1_pass_r8'] else 'fail'}, median wall ratio {summary['kill']['wall_ratio_median_r8']:.3f}; adopted: {summary['kill']['adopted']}"]

    def clean(o):
        if isinstance(o, float) and math.isinf(o):
            return "inf"
        if isinstance(o, dict):
            return {str(k): clean(v) for k, v in o.items()}
        if isinstance(o, list):
            return [clean(v) for v in o]
        return o

    (OUT / "summary.json").write_text(json.dumps(clean(summary), indent=1) + "\n")
    for name, rows in (("sets.csv", rows_csv), ("pairs.csv", pcsv)):
        with open(OUT / name, "w", newline="") as f:
            w = csv.DictWriter(f, fieldnames=list(rows[0].keys()))
            w.writeheader()
            for r in rows:
                w.writerow({k: json.dumps(clean(v)) if isinstance(v, (list, dict)) else clean(v) for k, v in r.items()})
    (OUT / "tables.md").write_text("\n".join(lines) + "\n")
    print("\n".join(lines))


if __name__ == "__main__":
    if "--need-extended" in sys.argv:
        need_extended()
    else:
        main()
