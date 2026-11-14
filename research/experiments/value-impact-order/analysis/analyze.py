#!/usr/bin/env python3
"""A21 analysis (../PREREG.md). Stdlib only.

python3 analysis/analyze.py   # reads results/test/*.jsonl, results/check/*.jsonl
                              # writes results/summary.json and results/tables.md
"""
import json
import math
import random
import statistics
import sys
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(HERE.parent / "hard-ladder" / "analysis"))
from analyze import km_quantile, upper_median, wilson, pct  # noqa: E402

INF = math.inf
B = 10_000
SEED = 20261111
CAP = 1_000_000
RUNGS = {"r6": "R6 6x6 c=5", "r7": "R7 7x7 c=6", "r8": "R8 8x8 c=10 s21-50"}
BB, CAND, SCB = "bb-mrv", "vi-impact", "scb-gacolor_ac3_random"
COMPOSITES = {"vi-probe-rand": [f"vi-probe-rand-s{i}" for i in (1, 2, 3)],
              "mrv-rand": [f"mrv-rand-s{i}" for i in (1, 2, 3)]}


def read(p):
    return [json.loads(l) for l in p.read_text().splitlines() if l.strip()] if p.exists() else []


def obs(r):
    return r["nodes"] if r["status"] == "solved" else INF


def time_cens(r):
    return r["status"] == "censored" and r["nodes_counted"] < r["cap"]


def collapse(rows):
    """arm -> instance -> one record (wall = median over repetitions)."""
    g = defaultdict(lambda: defaultdict(list))
    for r in rows:
        g[r["solver"]][r["instance"]].append(r)
    out, rep_mismatch = {}, []
    for arm, per in g.items():
        out[arm] = {}
        for inst, rs in per.items():
            keys = {(x["status"], x["nodes"], x["backtracks"], x.get("seq_hash")) for x in rs}
            if len(keys) > 1:
                rep_mismatch.append((arm, inst))
            r = dict(rs[0])
            r["wall_ms"] = statistics.median(x["wall_ms"] for x in rs)
            r["reps"] = len(rs)
            r["time_censored"] = any(time_cens(x) for x in rs)
            out[arm][inst] = r
    return out, rep_mismatch


def composite(arms, members):
    if not all(m in arms for m in members):
        return None
    insts = set.intersection(*(set(arms[m]) for m in members))
    out = {}
    for i in insts:
        rs = [arms[m][i] for m in members]
        v = sorted(obs(r) for r in rs)[1]
        out[i] = {
            "instance": i,
            "status": "solved" if v != INF else "censored",
            "nodes": v if v != INF else CAP,
            "nodes_counted": sorted(r["nodes_counted"] for r in rs)[1],
            "cap": CAP,
            "wall_ms": statistics.median(r["wall_ms"] for r in rs),
            "propagation": statistics.median(r["propagation"] for r in rs),
            "time_censored": any(r["time_censored"] for r in rs),
        }
    return out


def lr(c, b):
    cs, bs = c["status"] == "solved", b["status"] == "solved"
    if cs and bs:
        return math.log(c["nodes"] / b["nodes"])
    if cs:
        return 0.0 if b["time_censored"] else math.log(c["nodes"] / CAP)
    if bs:
        return INF
    return 0.0


def boot(vals):
    rng = random.Random(SEED)
    n = len(vals)
    meds = [upper_median([vals[rng.randrange(n)] for _ in range(n)]) for _ in range(B)]
    return [pct(meds, 0.025), pct(meds, 0.975)]


def pair(cand, base):
    ks = sorted(set(cand) & set(base))
    if not ks:
        return None
    l = [lr(cand[k], base[k]) for k in ks]
    med = upper_median(l)
    ci = boot(l)
    w = [cand[k]["wall_ms"] / base[k]["wall_ms"] for k in ks]
    p = [cand[k]["propagation"] / base[k]["propagation"] for k in ks]
    s = lambda r: r["status"] == "solved"  # noqa: E731
    return {
        "pairs": len(ks),
        "median_ln_ratio": med,
        "node_ratio": math.exp(med) if med != INF else INF,
        "ci_ln": ci,
        "ci_ratio": [math.exp(x) if x != INF else INF for x in ci],
        "g1_pass": med != INF and math.exp(med) <= 0.5 and ci[1] < 0,
        "below_one": med < 0 and ci[1] < 0,
        "both_solved": sum(s(cand[k]) and s(base[k]) for k in ks),
        "cand_only": sum(s(cand[k]) and not s(base[k]) for k in ks),
        "base_only": sum(not s(cand[k]) and s(base[k]) for k in ks),
        "both_censored": sum(not s(cand[k]) and not s(base[k]) for k in ks),
        "wall_ratio": upper_median(w),
        "wall_ci": boot(w),
        "prop_ratio": upper_median(p),
        "prop_ci": boot(p),
    }


def describe(recs):
    rs = list(recs.values())
    n = len(rs)
    k = sum(r["status"] == "solved" for r in rs)
    nodes = [obs(r) for r in rs]
    cnt = sum(r["nodes_counted"] for r in rs)
    wall = [r["wall_ms"] for r in rs]
    probes = [r.get("probes") for r in rs if r.get("probes") is not None]
    return {
        "N": n, "solved": k, "wilson95": list(wilson(k, n)),
        "km_q25": km_quantile(nodes, 0.25), "km_median": km_quantile(nodes, 0.5),
        "km_q75": km_quantile(nodes, 0.75),
        "median_wall_ms": upper_median(wall), "total_wall_s": sum(wall) / 1000,
        "propagate_per_node": sum(r["propagation"] for r in rs) / cnt if cnt else None,
        "probes_per_node": sum(probes) / cnt if probes and cnt else None,
        "time_censored": sum(r["time_censored"] for r in rs),
        "errors": sum(r["status"] == "error" for r in rs),
    }


def f(x, d=3):
    if x is None:
        return "-"
    if x == INF:
        return "inf"
    return f"{x:,.{d}f}"


def main():
    summary = {"rungs": {}, "decision": {}}
    md = ["# A21 results (generated by analysis/analyze.py; do not edit)", ""]
    per_rung_dec = {}
    for rung, label in RUNGS.items():
        rows = read(HERE / "results" / "test" / f"{rung}.jsonl")
        if not rows:
            continue
        arms, mism = collapse(rows)
        check_rows, _ = collapse(read(HERE / "results" / "check" / f"{rung}.jsonl"))
        for name, mem in COMPOSITES.items():
            c = composite(arms, mem)
            if c:
                arms[name] = c
        desc = {a: describe(v) for a, v in arms.items()}
        guard = sorted(f"{a}:{i}" for a, v in arms.items() if a not in COMPOSITES for i, r in v.items() if r["time_censored"])
        errors = sorted(f"{a}:{i}" for a, v in arms.items() for i, r in v.items() if r["status"] == "error")
        # implementation check: vi-index vs bb-mrv
        impl = None
        if "vi-index" in check_rows and BB in arms:
            ci, bb = check_rows["vi-index"], arms[BB]
            bad = [i for i in bb if i not in ci or
                   (ci[i]["nodes"], ci[i]["backtracks"], ci[i]["status"], ci[i]["equals_planted"]) !=
                   (bb[i]["nodes"], bb[i]["backtracks"], bb[i]["status"], bb[i]["equals_planted"])]
            impl = {"instances": len(bb), "mismatches": bad, "pass": not bad}
        pairs = {}
        lines = [(CAND, BB), (CAND, "vi-probe-rand"), (CAND, "mrv-rand"), (CAND, SCB),
                 ("vi-probe-rand", BB), ("mrv-rand", BB), (BB, SCB)]
        lines += [(m, BB) for mem in COMPOSITES.values() for m in mem]
        for c, b in lines:
            if c in arms and b in arms:
                pairs[f"{c} vs {b}"] = pair(arms[c], arms[b])
        main_ = pairs.get(f"{CAND} vs {BB}")
        mech = pairs.get(f"{CAND} vs vi-probe-rand")
        survive = bool(main_ and main_["g1_pass"] and main_["wall_ratio"] <= 1)
        per_rung_dec[rung] = {
            "g1_vs_bb": main_ and main_["g1_pass"],
            "wall_ratio_vs_bb": main_ and main_["wall_ratio"],
            "survive_here": survive,
            "mechanism_attributed": bool(mech and mech["median_ln_ratio"] < 0 and mech["ci_ln"][1] < 0),
        }
        summary["rungs"][rung] = {"label": label, "arms": desc, "pairs": pairs, "impl_check": impl,
                                  "guard_stopped": guard, "errors": errors, "rep_mismatch": mism,
                                  "reps": {a: max(r["reps"] for r in v.values()) for a, v in arms.items() if a not in COMPOSITES}}
        md += [f"## {label}", "",
               f"Implementation check (vi-index == bb-mrv on nodes, backtracks, status, equals_planted): "
               f"{'PASS' if impl and impl['pass'] else ('not run' if impl is None else 'FAIL ' + str(impl['mismatches']))}"
               f" over {impl['instances'] if impl else 0} instances. Guard-stopped runs: {guard or 'none'}. "
               f"Errors: {errors or 'none'}. Repetition node mismatches: {mism or 'none'}.", "",
               "| arm | solved/N | Wilson 95% | KM q25 | KM median | KM q75 | median wall ms | total wall s | propagate/node | probes/node |",
               "|---|---|---|---|---|---|---|---|---|---|"]
        for a in [BB, CAND, "vi-probe-rand", *COMPOSITES["vi-probe-rand"], "mrv-rand", *COMPOSITES["mrv-rand"], SCB]:
            if a not in desc:
                continue
            d = desc[a]
            md.append(f"| {a} | {d['solved']}/{d['N']} | [{d['wilson95'][0]:.2f}, {d['wilson95'][1]:.2f}] | "
                      f"{f(d['km_q25'], 0)} | {f(d['km_median'], 0)} | {f(d['km_q75'], 0)} | {f(d['median_wall_ms'], 1)} | "
                      f"{f(d['total_wall_s'], 1)} | {f(d['propagate_per_node'], 2)} | {f(d['probes_per_node'], 2)} |")
        md += ["", "| candidate vs baseline | pairs | node ratio | 95% CI | G1 | both solved | cand only | base only | both cens | wall ratio | wall 95% CI | propagate ratio | prop 95% CI |",
               "|---|---|---|---|---|---|---|---|---|---|---|---|---|"]
        for k, p in pairs.items():
            md.append(f"| {k} | {p['pairs']} | {f(p['node_ratio'])} | [{f(p['ci_ratio'][0])}, {f(p['ci_ratio'][1])}] | "
                      f"{'PASS' if p['g1_pass'] else 'fail'} | {p['both_solved']} | {p['cand_only']} | {p['base_only']} | "
                      f"{p['both_censored']} | {f(p['wall_ratio'])} | [{f(p['wall_ci'][0])}, {f(p['wall_ci'][1])}] | "
                      f"{f(p['prop_ratio'])} | [{f(p['prop_ci'][0])}, {f(p['prop_ci'][1])}] |")
        md.append("")
    if per_rung_dec:
        survive = any(d["survive_here"] for d in per_rung_dec.values())
        complete = set(per_rung_dec) == set(RUNGS)
        attributed = survive and all(d["mechanism_attributed"] for d in per_rung_dec.values() if d["survive_here"])
        drop = None
        for r in ("r6", "r8"):
            if r in per_rung_dec and not per_rung_dec[r]["g1_vs_bb"]:
                drop = r
                break
        verdict = ("SURVIVE" + ("" if attributed else ", gain not attributable to the impact ranking")) if survive else "KILL"
        summary["decision"] = {"per_rung": per_rung_dec, "all_rungs_present": complete, "verdict": verdict,
                               "protocol_dropped_at": drop}
        md += ["## Decision (PREREG.md)", "", "| rung | G1 vs bb-mrv | wall ratio vs bb-mrv | survives here | ranking attributed (vs vi-probe-rand) |",
               "|---|---|---|---|---|"]
        for r, d in per_rung_dec.items():
            md.append(f"| {RUNGS[r]} | {'PASS' if d['g1_vs_bb'] else 'fail'} | {f(d['wall_ratio_vs_bb'])} | {d['survive_here']} | {d['mechanism_attributed']} |")
        md += ["", f"Verdict (A21 kill rule){'' if complete else ' (incomplete: not every rung run)'}: **{verdict}**.",
               f"PROTOCOL.md dropping rule (R6 then R8): {'dropped at ' + drop if drop else 'not dropped'}.", ""]
    (HERE / "results" / "summary.json").write_text(json.dumps(summary, indent=1, default=lambda x: None) + "\n")
    (HERE / "results" / "tables.md").write_text("\n".join(md) + "\n")
    print("\n".join(md))


if __name__ == "__main__":
    main()
