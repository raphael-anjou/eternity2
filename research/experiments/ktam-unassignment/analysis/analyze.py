#!/usr/bin/env python3
"""A15 analysis (stdlib only). See ../PREREG.md sections 5 to 8.

  analyze.py select    calibration rows -> results/calibration/selection.json, CALIBRATION table
  analyze.py summary   test rows -> results/summary.json, results/summary.md
"""
import json
import math
import random
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
RES = ROOT / "results"
LADDER = ROOT.parent / "hard-ladder" / "results" / "test" / "primary"
E = 1_000_000
NODE_CAP = 1_000_000
B = 10_000
BOOT_SEED = 20261111
GRIDS = {
    "B3": [f"b3-r{r}-k{k}" for r in ("1", "2", "inf") for k in (1, 2, 3)],
    "V1": [f"v1-beta{b}-r{r}-k{k}" for b in ("0.5", "1", "2", "4") for r in ("1", "2", "inf") for k in (1, 2, 3)],
    "V2": [f"v2-t{t}-n{n}" for t in (0, 16, 128) for n in ("0", "0.1", "0.3")],
}
ARMS = ["B1", "B2", "B3", "V1", "V2"]
RUNG_N = {"R6": 6, "R7": 7, "R8": 8}


def load(path):
    with open(path) as f:
        return [json.loads(line) for line in f if line.strip()]


def ev(r):
    """Events to solve within E, else inf."""
    return r["events"] if r["within_e"] else math.inf


def nd(r):
    """Ladder nodes to solve within the 1e6 node cap, else inf."""
    return r["nodes"] if r["status"] == "solved" and r["nodes"] <= NODE_CAP else math.inf


def km_q(vals, q):
    s = sorted(vals)
    return s[max(1, math.ceil(len(s) * q)) - 1]


def upper_median(vals):
    s = sorted(vals)
    return s[len(s) // 2]


def wilson(k, n, z=1.959964):
    p = k / n
    d = 1 + z * z / n
    c = (p + z * z / (2 * n)) / d
    h = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / d
    return [max(0.0, c - h), min(1.0, c + h)]


def fin(v):
    return None if math.isinf(v) else v


def log_ratio(c, b, cap):
    if not math.isinf(c) and not math.isinf(b):
        return math.log(c / b)
    if not math.isinf(c):
        return math.log(c / cap)
    if not math.isinf(b):
        return math.inf
    return 0.0


def g1(cand, base, cap):
    """cand, base: dict instance -> effort (inf = censored)."""
    keys = sorted(base)
    assert sorted(cand) == keys, "unpaired instances"
    lr = [log_ratio(cand[k], base[k], cap) for k in keys]
    med = upper_median(lr)
    rng = random.Random(BOOT_SEED)
    boots = sorted(upper_median([lr[rng.randrange(len(lr))] for _ in lr]) for _ in range(B))
    lo, hi = boots[int(0.025 * B)], boots[int(0.975 * B) - 1]
    p = sum(1 for m in boots if m >= 0) / B
    return {
        "n": len(lr),
        "median_log_ratio": fin(med),
        "median_ratio": None if math.isinf(med) else math.exp(med),
        "ci95_median_log_ratio": [fin(lo), fin(hi)],
        "p_one_sided": p,
        "unadjusted_significant": p <= 0.025,
    }


def label(res, sig):
    if sig and res["median_ratio"] is not None and res["median_ratio"] <= 0.5:
        return "PASS"
    return "GAIN" if sig else "NO-GAIN"


def holm(pair):
    """pair: {'V1': g1, 'V2': g1} -> adds holm_significant and label."""
    order = sorted(pair, key=lambda v: (pair[v]["p_one_sided"], v))
    first, second = order
    s1 = pair[first]["p_one_sided"] <= 0.0125
    s2 = s1 and pair[second]["p_one_sided"] <= 0.025
    for v, s in ((first, s1), (second, s2)):
        pair[v]["holm_significant"] = s
        pair[v]["label"] = label(pair[v], s)
        pair[v]["label_unadjusted"] = label(pair[v], pair[v]["unadjusted_significant"])
    return pair


def cmd_select():
    sel, table = {}, {}
    for arm, names in GRIDS.items():
        rows = []
        for name in names:
            rs = load(RES / "calibration" / arm / f"{name}.jsonl")
            assert len(rs) == 20 and not any(r["status"] == "error" for r in rs), name
            solved = sum(r["within_e"] for r in rs)
            effort = sum(math.log(min(r["events"], E)) for r in rs)
            rows.append({"config": name, "solved": solved, "sum_ln_events": round(effort, 4),
                         "solved_R6": sum(r["within_e"] for r in rs if r["n"] == 6),
                         "solved_R7": sum(r["within_e"] for r in rs if r["n"] == 7)})
        best = min(range(len(rows)), key=lambda i: (-rows[i]["solved"], rows[i]["sum_ln_events"], i))
        sel[arm] = rows[best]["config"]
        table[arm] = rows
    out = {"rule": "max solved within E on 20 calibration instances (R6+R7), then min sum ln(min(events,E)), then grid order",
           "selected": sel, "grids": table}
    (RES / "calibration" / "selection.json").write_text(json.dumps(out, indent=2) + "\n")
    for arm in GRIDS:
        print(f"\n### {arm} (selected: `{sel[arm]}`)\n")
        print("| config | solved /20 | R6 /10 | R7 /10 | sum ln(min(events, E)) |")
        print("|---|---|---|---|---|")
        for r in table[arm]:
            mark = " **selected**" if r["config"] == sel[arm] else ""
            print(f"| `{r['config']}`{mark} | {r['solved']} | {r['solved_R6']} | {r['solved_R7']} | {r['sum_ln_events']:.2f} |")


def check_baseline(rung, arm, rows):
    """B1/B2 at R6/R7 must equal the committed ladder rows node for node."""
    name = {"B1": "scb-gacolor_ac3_random", "B2": "hp-dfs"}[arm]
    ref = {r["instance"]: r for r in load(LADDER / f"{name}.jsonl") if r["n"] == RUNG_N[rung]}
    bad = [r["instance"] for r in rows if (ref[r["instance"]]["status"], ref[r["instance"]]["nodes"]) != (r["status"], r["nodes"])]
    return {"checked_against": str((LADDER / f"{name}.jsonl").relative_to(ROOT.parent.parent.parent)), "mismatches": bad}


def arm_stats(rows):
    n = len(rows)
    k = sum(r["within_e"] for r in rows)
    evs = [ev(r) for r in rows]
    nds = [nd(r) for r in rows]
    walls = sorted(r["wall_ms"] for r in rows)
    s = {
        "n": n,
        "errors": sum(r["status"] == "error" for r in rows),
        "solved_within_E": k,
        "wilson95": wilson(k, n),
        "km_events": {q: fin(km_q(evs, p)) for q, p in (("q25", 0.25), ("median", 0.5), ("q75", 0.75))},
        "solved_within_node_cap": sum(not math.isinf(v) for v in nds),
        "km_nodes": {q: fin(km_q(nds, p)) for q, p in (("q25", 0.25), ("median", 0.5), ("q75", 0.75))},
        "wall_ms_median": walls[n // 2],
        "wall_ms_total": sum(walls),
        "propagation_total": sum(r["propagation"] for r in rows),
        "equals_planted": sum(r["equals_planted"] for r in rows if r["status"] == "solved"),
    }
    if rows[0]["removals"] is not None:
        rem = sum(r["removals"] for r in rows)
        placed = sum(r["nodes"] for r in rows)
        rp = sum(r["removals_planted"] for r in rows)
        pp = sum(r["placements_planted"] for r in rows)
        s.update({
            "dead_ends_total": sum(r["dead_ends"] for r in rows),
            "removals_total": rem,
            "removals_median": sorted(r["removals"] for r in rows)[n // 2],
            "removed_planted_fraction": rp / rem if rem else None,
            "placed_planted_fraction": pp / placed if placed else None,
        })
    return s


def better(stats, a, b):
    """Prereg: more solved within E, ties by lower KM median events, then the first."""
    ka, kb = stats[a]["solved_within_E"], stats[b]["solved_within_E"]
    if ka != kb:
        return a if ka > kb else b
    ma = stats[a]["km_events"]["median"]
    mb = stats[b]["km_events"]["median"]
    ma = math.inf if ma is None else ma
    mb = math.inf if mb is None else mb
    return b if mb < ma else a


def cmd_summary():
    rungs = {}
    for rung in ("R6", "R7", "R8"):
        d = RES / "test" / rung
        if not d.exists():
            continue
        rows = {a: load(d / f"{a}.jsonl") for a in ARMS}
        for a in ARMS:
            assert len(rows[a]) == 30, (rung, a)
        stats = {a: arm_stats(rows[a]) for a in ARMS}
        efforts = {a: {r["instance"]: ev(r) for r in rows[a]} for a in ARMS}
        nodes = {a: {r["instance"]: nd(r) for r in rows[a]} for a in ARMS}
        ref_b13 = better(stats, "B1", "B3")
        comps = {}
        for ref in ("B1", "B1orB3", "B3", "B2"):
            base = ref_b13 if ref == "B1orB3" else ref
            comps[ref] = {"reference_arm": base,
                          "events": holm({v: g1(efforts[v], efforts[base], E) for v in ("V1", "V2")}),
                          "nodes_secondary": holm({v: g1(nodes[v], nodes[base], NODE_CAP) for v in ("V1", "V2")})}
        best = better(stats, "V1", "V2")
        g_b3 = g1(efforts[best], efforts["B3"], E)
        rungs[rung] = {
            "arms": stats,
            "comparisons": comps,
            "best_variant": best,
            "best_vs_B3_events": g_b3,
            "baseline_checks": {a: check_baseline(rung, a, rows[a]) for a in ("B1", "B2")} if rung != "R8" else None,
        }
        kb = max(stats[a]["solved_within_E"] for a in ("B1", "B2", "B3"))
        rungs[rung]["kill_terms"] = {
            "both_no_gain_vs_B1": all(comps["B1"]["events"][v]["label"] == "NO-GAIN" for v in ("V1", "V2")),
            "best_solved": stats[best]["solved_within_E"],
            "max_baseline_solved": kb,
            "best_le_max_plus_3": stats[best]["solved_within_E"] <= kb + 3,
            "b3_within_2": stats["B3"]["solved_within_E"] >= stats[best]["solved_within_E"] - 2,
            "best_not_sig_vs_B3": not g_b3["unadjusted_significant"],
        }
        rungs[rung]["r8_trigger"] = any(comps["B1"]["events"][v]["label"] in ("GAIN", "PASS") for v in ("V1", "V2"))
    run = list(rungs)
    kt = [rungs[r]["kill_terms"] for r in run]
    method_killed = all(t["both_no_gain_vs_B1"] and t["best_le_max_plus_3"] for t in kt)
    mech_killed = all(t["b3_within_2"] and t["best_not_sig_vs_B3"] for t in kt)
    labels = [rungs[r]["comparisons"]["B1"]["events"][v]["label"] for r in run for v in ("V1", "V2")]
    if method_killed or mech_killed:
        outcome = "killed"
    elif "PASS" in labels:
        outcome = "survived"
    elif "GAIN" in labels:
        outcome = "inconclusive"
    else:
        outcome = "inconclusive"
    summary = {
        "event_budget": E,
        "node_cap": NODE_CAP,
        "rungs_run": run,
        "r8_triggered": any(rungs[r]["r8_trigger"] for r in ("R6", "R7") if r in rungs),
        "rungs": rungs,
        "method_killed": method_killed,
        "mechanism_killed": mech_killed,
        "self_assessed_outcome": outcome,
    }
    (RES / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    md = render(summary)
    (RES / "summary.md").write_text(md)
    print(md)


def f(v, nd=0):
    if v is None:
        return "censored"
    return f"{v:,.{nd}f}"


def render(s):
    out = []
    for rung, R in s["rungs"].items():
        out.append(f"### {rung}\n")
        out.append("| arm | solved within E /30 | Wilson 95% | KM q25 / median / q75 events | solved within 1e6 nodes | KM median nodes | removals (total) | removed = planted | placed = planted | wall median ms | wall total s |")
        out.append("|---|---|---|---|---|---|---|---|---|---|---|")
        for a, st in R["arms"].items():
            km = st["km_events"]
            rem = f"{st['removals_total']:,}" if "removals_total" in st else "n/a"
            rpf = f"{st['removed_planted_fraction']:.3f}" if st.get("removed_planted_fraction") is not None else "n/a"
            ppf = f"{st['placed_planted_fraction']:.3f}" if st.get("placed_planted_fraction") is not None else "n/a"
            out.append(f"| {a} | {st['solved_within_E']} | {st['wilson95'][0]:.2f} to {st['wilson95'][1]:.2f} | {f(km['q25'])} / {f(km['median'])} / {f(km['q75'])} | {st['solved_within_node_cap']} | {f(st['km_nodes']['median'])} | {rem} | {rpf} | {ppf} | {st['wall_ms_median']:.0f} | {st['wall_ms_total'] / 1000:.1f} |")
        out.append("")
        out.append("| comparison (events, G1) | variant | median ratio | 95% CI of median ln ratio | p one-sided | Holm label | unadjusted label |")
        out.append("|---|---|---|---|---|---|---|")
        for ref, C in R["comparisons"].items():
            for v, g in C["events"].items():
                ci = g["ci95_median_log_ratio"]
                ci_s = " to ".join("+inf" if x is None else f"{x:.3f}" for x in ci)
                mr = "+inf" if g["median_ratio"] is None else f"{g['median_ratio']:.3f}"
                out.append(f"| vs {ref} ({C['reference_arm']}) | {v} | {mr} | {ci_s} | {g['p_one_sided']:.4f} | {g['label']} | {g['label_unadjusted']} |")
        g = R["best_vs_B3_events"]
        mr = "+inf" if g["median_ratio"] is None else f"{g['median_ratio']:.3f}"
        out.append(f"\nBest variant: {R['best_variant']}; vs B3 median ratio {mr}, p = {g['p_one_sided']:.4f}.")
        out.append(f"Kill terms: `{json.dumps(R['kill_terms'])}`")
        if R["baseline_checks"]:
            out.append(f"Baseline reproduction: `{json.dumps({a: len(c['mismatches']) for a, c in R['baseline_checks'].items()})}` mismatching rows.")
        out.append("")
    out.append(f"R8 triggered: {s['r8_triggered']}. Method killed: {s['method_killed']}. Mechanism killed: {s['mechanism_killed']}. Self-assessed outcome: **{s['self_assessed_outcome']}**.\n")
    return "\n".join(out)


if __name__ == "__main__":
    {"select": cmd_select, "summary": cmd_summary}[sys.argv[1]]()
