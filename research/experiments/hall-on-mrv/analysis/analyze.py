#!/usr/bin/env python3
"""hall-on-mrv analysis (PREREG.md section 7). Stdlib only.

Reads results/test/{r6,r7,r8}.jsonl (one row per arm, instance, repetition)
and writes results/summary.json and results/tables.md.
"""
import json
import math
import random
import sys
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent.parent
RES = HERE / "results"
B = 10_000
SEED = 20261111
RUNGS = [("r6", "R6 6x6 c=5"), ("r7", "R7 7x7 c=6"), ("r8", "R8 8x8 c=10")]
HALL = ["hall-v1", "hall-v2"]
COST = {"hall-v1": "cost-v1", "hall-v2": "cost-v2"}
RAND = ["mrv-rand-1", "mrv-rand-2", "mrv-rand-3"]


def load(rung):
    p = RES / "test" / f"{rung}.jsonl"
    if not p.exists():
        return None
    rows = [json.loads(l) for l in p.read_text().splitlines() if l.strip()]
    by = defaultdict(lambda: defaultdict(list))
    for r in rows:
        by[r["arm"]][r["instance"]].append(r)
    agg = {}
    issues = []
    for arm, insts in by.items():
        agg[arm] = {}
        for inst, rs in insts.items():
            st = {(r["status"], r["nodes"]) for r in rs}
            if len(st) != 1:
                issues.append(f"{rung} {arm} {inst}: reps disagree {st}")
            walls = sorted(r["wall_ms"] for r in rs)
            r0 = dict(rs[0])
            r0["wall_med"] = walls[(len(walls) - 1) // 2] if len(walls) % 2 else 0.5 * (walls[len(walls) // 2 - 1] + walls[len(walls) // 2])
            r0["reps"] = len(rs)
            r0["guard_any"] = any(r.get("guard_stop") for r in rs)
            agg[arm][inst] = r0
    return agg, issues, rows


def upper_median(xs):
    s = sorted(xs)
    return s[len(s) // 2]


def boot(vals, rng_seed=SEED):
    """Percentile bootstrap of the upper median; returns (stat, lo, hi, p_ge0)."""
    n = len(vals)
    rng = random.Random(rng_seed)
    meds = []
    for _ in range(B):
        meds.append(upper_median([vals[rng.randrange(n)] for _ in range(n)]))
    meds.sort()
    lo = meds[int(0.025 * B)]
    hi = meds[int(0.975 * B) - 1]
    p = sum(1 for m in meds if m >= 0) / B
    return upper_median(vals), lo, hi, p


def g1_r(c, b, cap):
    cs, bs = c["status"] == "solved", b["status"] == "solved"
    if cs and bs:
        return math.log(c["nodes"] / b["nodes"])
    if cs and not bs:
        return math.log(c["nodes"] / cap)
    if not cs and bs:
        return math.inf
    return 0.0


def ex(x):
    return math.exp(x) if x != math.inf else math.inf


def km(rows):
    xs = sorted(r["nodes"] if r["status"] == "solved" else math.inf for r in rows)
    # 30 instances: q25 = 8th, median = 15th, q75 = 23rd order statistic
    n = len(xs)
    return xs[math.ceil(0.25 * n) - 1], xs[math.ceil(0.5 * n) - 1], xs[math.ceil(0.75 * n) - 1]


def wilson(k, n, z=1.959964):
    p = k / n
    d = 1 + z * z / n
    c = (p + z * z / (2 * n)) / d
    h = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / d
    return max(0.0, c - h), min(1.0, c + h)


def compare(agg, cand, base):
    insts = sorted(agg[base])
    cap = agg[base][insts[0]]["cap"]
    r = [g1_r(agg[cand][i], agg[base][i], cap) for i in insts]
    w = [math.log(agg[cand][i]["wall_med"] / agg[base][i]["wall_med"]) for i in insts]
    med, lo, hi, p = boot(r)
    wmed, wlo, whi, _ = boot(w)
    cs = [agg[cand][i]["status"] == "solved" for i in insts]
    bs = [agg[base][i]["status"] == "solved" for i in insts]
    return {
        "pairs": len(insts),
        "g1_ratio": ex(med), "g1_log_ci": [lo, hi], "p_ge0": p,
        "g1_pass": ex(med) <= 0.5 and hi < 0,
        "wall_ratio": math.exp(wmed), "wall_ci": [math.exp(wlo), math.exp(whi)],
        "wins": sum(1 for x in r if x < 0), "losses": sum(1 for x in r if x > 0), "ties": sum(1 for x in r if x == 0),
        "both_cens": sum(1 for a, b in zip(cs, bs) if not a and not b),
        "cand_only": sum(1 for a, b in zip(cs, bs) if a and not b),
        "base_only": sum(1 for a, b in zip(cs, bs) if b and not a),
    }


def holm_labels(cmp):
    """A17 labels with Holm over the two Hall variants at one rung."""
    ps = sorted(HALL, key=lambda v: cmp[v]["p_ge0"])
    sig = {}
    first = cmp[ps[0]]["p_ge0"] <= 0.025 / 2
    sig[ps[0]] = first
    sig[ps[1]] = first and cmp[ps[1]]["p_ge0"] <= 0.025
    out = {}
    for v in HALL:
        below = sig[v] and cmp[v]["g1_log_ci"][1] < 0
        if below and cmp[v]["g1_ratio"] <= 0.5:
            out[v] = "PASS"
        elif below:
            out[v] = "GAIN"
        else:
            out[v] = "NO-GAIN"
    return out


def checks(agg):
    c = {}
    base = agg["bb-mrv"]
    for arm in ["hall-off", "cost-v1", "cost-v2"]:
        if arm not in agg:
            continue
        same = 0
        diffs = []
        for i, b in base.items():
            a = agg[arm][i]
            key = ("status", "nodes", "backtracks", "propagation", "board_hash")
            if all(a[k] == b[k] for k in key):
                same += 1
            else:
                diffs.append(i)
        c[f"{arm}_equals_bb-mrv"] = f"{same}/{len(base)}"
        if diffs:
            c[f"{arm}_diffs"] = diffs
    for arm in ["cost-v1", "cost-v2"]:
        if arm in agg:
            same = sum(1 for i in base if agg[arm][i]["seq_hash"] == agg["hall-off"][i]["seq_hash"])
            c[f"{arm}_seq_hash_equals_hall-off"] = f"{same}/{len(base)}"
    for v in HALL:
        if v not in agg:
            continue
        le = sum(1 for i, b in base.items() if agg[v][i]["nodes"] <= b["nodes"])
        both = [i for i, b in base.items() if b["status"] == "solved" and agg[v][i]["status"] == "solved"]
        sb = sum(1 for i in both if agg[v][i]["board_hash"] == base[i]["board_hash"])
        c[f"{v}_nodes_le_bb-mrv"] = f"{le}/{len(base)}"
        c[f"{v}_same_board_when_both_solved"] = f"{sb}/{len(both)}"
        c[f"{v}_instances_with_a_prune"] = sum(1 for i in base if agg[v][i]["hall_fails"] > 0)
        c[f"{v}_total_prunes"] = sum(agg[v][i]["hall_fails"] for i in base)
    for v in ["cost-v1", "cost-v2"]:
        if v in agg:
            c[f"{v}_total_would_prune"] = sum(agg[v][i]["hall_fails"] for i in base)
            c[f"{v}_total_checks"] = sum(agg[v][i]["hall_checks"] for i in base)
    return c


def fmt(x):
    if x == math.inf:
        return "cens."
    return f"{x:,.0f}"


def main():
    summary = {"rungs": {}}
    md = ["# hall-on-mrv tables (generated by analysis/analyze.py)", ""]
    per_arm = ["| rung | arm | solved/30 [Wilson 95%] | KM q25 | KM median | KM q75 | median wall ms | total wall s | guard stops | total nodes |", "|" + "---|" * 10]
    pair = ["| rung | candidate | baseline | G1 ratio | 95% CI (ratio) | p(med>=0) | W/L/T | both cens. | cand only | base only | wall ratio | wall 95% CI |", "|" + "---|" * 12]
    verdict = {}
    for rung, label in RUNGS:
        got = load(rung)
        if got is None:
            continue
        agg, issues, rows = got
        rs = {"label": label, "issues": issues, "arms": {}, "pairs": {}}
        for arm in agg:
            vals = list(agg[arm].values())
            k = sum(1 for r in vals if r["status"] == "solved")
            q1, m, q3 = km(vals)
            walls = sorted(r["wall_med"] for r in vals)
            rs["arms"][arm] = {
                "solved": k, "n": len(vals), "wilson": wilson(k, len(vals)), "km": [q1, m, q3],
                "median_wall_ms": upper_median(walls), "total_wall_s": sum(walls) / 1000,
                "guard_stops": sum(1 for r in vals if r["guard_any"]),
                "total_nodes": sum(r["nodes"] for r in vals),
                "errors": sum(1 for r in vals if r["status"] == "error"),
                "inconsistent": sum(r["inconsistent"] for r in vals),
            }
        order = [a for a in ["bb-mrv", "hall-off", "hall-v1", "hall-v2", "cost-v1", "cost-v2", *RAND, "scb"] if a in agg]
        for arm in order:
            a = rs["arms"][arm]
            per_arm.append(f"| {label} | {arm} | {a['solved']}/{a['n']} [{a['wilson'][0]:.2f}, {a['wilson'][1]:.2f}] | "
                           f"{fmt(a['km'][0])} | {fmt(a['km'][1])} | {fmt(a['km'][2])} | {a['median_wall_ms']:.1f} | {a['total_wall_s']:.1f} | {a['guard_stops']} | {a['total_nodes']:,} |")
        comps = [(c, "bb-mrv") for c in ["hall-off", *HALL, "cost-v1", "cost-v2", *RAND, "scb"] if c in agg]
        comps += [(v, COST[v]) for v in HALL if v in agg and COST[v] in agg]
        if "scb" in agg:
            comps += [(c, "scb") for c in ["bb-mrv", *HALL]]
        for c, b in comps:
            k = f"{c}/{b}"
            rs["pairs"][k] = compare(agg, c, b)
            p = rs["pairs"][k]
            lo, hi = p["g1_log_ci"]
            pair.append(f"| {label} | {c} | {b} | {p['g1_ratio']:.3f} | [{ex(lo):.3f}, {ex(hi):.3f}] | {p['p_ge0']:.4f} | "
                        f"{p['wins']}/{p['losses']}/{p['ties']} | {p['both_cens']} | {p['cand_only']} | {p['base_only']} | "
                        f"{p['wall_ratio']:.3f} | [{p['wall_ci'][0]:.3f}, {p['wall_ci'][1]:.3f}] |")
        cmp = {v: rs["pairs"][f"{v}/bb-mrv"] for v in HALL if f"{v}/bb-mrv" in rs["pairs"]}
        if len(cmp) == 2:
            rs["labels"] = holm_labels(cmp)
        rand = [rs["pairs"][f"{r}/bb-mrv"]["g1_ratio"] for r in RAND if f"{r}/bb-mrv" in rs["pairs"]]
        if rand:
            rs["rand_spread"] = [min(rand), max(rand)]
            rs["hall_below_rand_spread"] = {v: cmp[v]["g1_ratio"] < min(rand) for v in cmp}
        rs["checks"] = checks(agg)
        summary["rungs"][rung] = rs
    # kill rule (PREREG.md section 6)
    survive = {}
    for v in HALL:
        ok = False
        detail = []
        for rung, _ in RUNGS:
            rs = summary["rungs"].get(rung)
            if not rs or "labels" not in rs:
                continue
            lab = rs["labels"][v]
            wr = rs["pairs"][f"{v}/bb-mrv"]["wall_ratio"]
            detail.append(f"{rung}: {lab}, wall {wr:.3f}")
            if lab in ("GAIN", "PASS") and wr <= 1:
                ok = True
        survive[v] = {"survives": ok, "detail": detail}
    summary["kill"] = {"variants": survive, "method_killed": not any(s["survives"] for s in survive.values())}
    md += ["## Per rung and arm", "", "Wall clock per instance is the median over repetitions (5 at R6, 1 at R7 and R8).", ""] + per_arm
    md += ["", "## Paired comparisons", "", "G1 ratio = exp(upper median of paired log node ratios), PROTOCOL.md censoring rules; wall ratio = exp(upper median of paired log wall ratios). Bootstrap 10,000, seed 20261111.", ""] + pair
    md += ["", "## Labels, order-effect spread, checks", ""]
    for rung, label in RUNGS:
        rs = summary["rungs"].get(rung)
        if not rs:
            continue
        md.append(f"- {label}: labels {rs.get('labels')}; mrv-rand G1 spread {rs.get('rand_spread')}; Hall below spread {rs.get('hall_below_rand_spread')}")
        md.append(f"  - checks: {json.dumps(rs['checks'])}")
        if rs["issues"]:
            md.append(f"  - issues: {rs['issues']}")
    md += ["", f"Kill rule: {json.dumps(summary['kill'])}"]
    RES.mkdir(exist_ok=True)
    (RES / "summary.json").write_text(json.dumps(summary, indent=1, default=str) + "\n")
    (RES / "tables.md").write_text("\n".join(md) + "\n")
    print("\n".join(md))


if __name__ == "__main__":
    sys.exit(main())
