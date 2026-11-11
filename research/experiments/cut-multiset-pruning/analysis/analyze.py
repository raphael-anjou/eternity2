#!/usr/bin/env python3
"""Cut-multiset pruning analysis (PREREG.md sections 3, 5, 6, 7). Stdlib only.

python3 analysis/analyze.py calibration   # V2 selection on seeds 1001..1010
python3 analysis/analyze.py test V2NAME   # labels, Holm, summary.json
"""
import json
import math
import os
import random
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
LADDER = os.path.join(ROOT, "..", "hard-ladder")
B = 10_000
BOOT_SEED = 20261111
INF = math.inf
V1 = "cmp-hall-frontier"
V2_GRID = ["cmp-hall-all", "cmp-hall-frontier-k4", "cmp-hall-all-k4"]
BASE = "scb-gacolor_ac3_random"
RUNGS = {"R6": ("primary", 6), "R7": ("primary", 7), "R8": ("8-c10", 8)}


def load(path):
    with open(path) as f:
        return [json.loads(l) for l in f if l.strip()]


def rows(role, solver, rung):
    ladder, n = RUNGS[rung]
    local = os.path.join(ROOT, "results", role, ladder, f"{solver}.jsonl")
    committed = os.path.join(LADDER, "results", role, ladder, f"{solver}.jsonl")
    path = local if os.path.exists(local) else committed
    return sorted((r for r in load(path) if r["n"] == n), key=lambda r: r["seed"]), os.path.relpath(path, ROOT)


def obs(r):
    return r["nodes"] if r["status"] == "solved" else INF


def kmq(vals, q):
    s = sorted(vals)
    return s[max(1, math.ceil(len(s) * q)) - 1]


def upper_median(v):
    s = sorted(v)
    return s[len(s) // 2]


def pct(v, p):
    s = sorted(v)
    return s[min(len(s) - 1, max(0, int(math.floor(p * (len(s) - 1) + 0.5))))]


def wilson(k, n, z=1.959964):
    p = k / n
    d = 1 + z * z / n
    c = (p + z * z / (2 * n)) / d
    h = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / d
    return [round(max(0.0, c - h), 4), round(min(1.0, c + h), 4)]


def fin(x):
    return None if x is None or math.isinf(x) else x


def stats(rs):
    vals = [obs(r) for r in rs]
    k = sum(r["status"] == "solved" for r in rs)
    walls = sorted(r["wall_ms"] for r in rs)
    nodes = sum(r["nodes"] for r in rs)
    return {
        "instances": len(rs),
        "solved": k,
        "errors": sum(r["status"] == "error" for r in rs),
        "solve_wilson95": wilson(k, len(rs)),
        "km_median_nodes": fin(kmq(vals, 0.5)),
        "km_q1_nodes": fin(kmq(vals, 0.25)),
        "km_q3_nodes": fin(kmq(vals, 0.75)),
        "median_wall_ms": walls[len(walls) // 2],
        "total_wall_s": round(sum(walls) / 1000, 2),
        "nodes_per_s": round(nodes / max(1e-9, sum(walls) / 1000)),
        "inconsistent_tries": sum(r["inconsistent"] for r in rs),
    }


def log_ratio(c, b):
    cs, bs = c["status"] == "solved", b["status"] == "solved"
    if cs and bs:
        return math.log(c["nodes"] / b["nodes"])
    if cs:
        return math.log(c["nodes"] / b["cap"])
    if bs:
        return INF
    return 0.0


def compare(cand, base):
    assert [r["instance"] for r in cand] == [r["instance"] for r in base]
    assert all(c["cap"] == b["cap"] for c, b in zip(cand, base))
    lr = [log_ratio(c, b) for c, b in zip(cand, base)]
    rng = random.Random(BOOT_SEED)
    med = upper_median(lr)
    boots = [upper_median([lr[rng.randrange(len(lr))] for _ in lr]) for _ in range(B)]
    wl = [math.log(max(1, c["wall_ms"]) / max(1, b["wall_ms"])) for c, b in zip(cand, base)]
    return {
        "pairs": len(lr),
        "median_ratio": fin(math.exp(med) if not math.isinf(med) else INF),
        "median_log_ratio": fin(med),
        "ci95_median_log_ratio": [fin(pct(boots, 0.025)), fin(pct(boots, 0.975))],
        "p_one_sided": sum(x >= 0 for x in boots) / B,
        "wins": sum(x < 0 for x in lr),
        "losses": sum(x > 0 for x in lr),
        "ties": sum(x == 0 for x in lr),
        "median_wall_ratio": round(math.exp(upper_median(wl)), 4),
    }


def cmd_calibration():
    out = {"rule": "most solved over R6+R7+R8, then sum of log10 KM median (cap if censored), then total wall", "candidates": {}}
    for s in [BASE, V1] + V2_GRID:
        per = {}
        for g in RUNGS:
            rs, path = rows("calibration", s, g)
            st = stats(rs)
            st["source"] = path
            per[g] = st
        solved = sum(per[g]["solved"] for g in RUNGS)
        logm = sum(math.log10(per[g]["km_median_nodes"] or 2_000_000) for g in RUNGS)
        wall = sum(per[g]["total_wall_s"] for g in RUNGS)
        out["candidates"][s] = {"solved_total": solved, "sum_log10_km_median": round(logm, 4), "total_wall_s": round(wall, 2), "rungs": per}
    out["V2"] = min(V2_GRID, key=lambda s: (-out["candidates"][s]["solved_total"], out["candidates"][s]["sum_log10_km_median"], out["candidates"][s]["total_wall_s"]))
    path = os.path.join(ROOT, "results", "calibration", "selection.json")
    with open(path, "w") as f:
        json.dump(out, f, indent=1)
    for s, c in out["candidates"].items():
        print(f"{s:24} solved {c['solved_total']:2}/30  sumlog10med {c['sum_log10_km_median']:7.3f}  wall {c['total_wall_s']:8.1f}s  "
              + "  ".join(f"{g}:{c['rungs'][g]['solved']}/{c['rungs'][g]['km_median_nodes']}" for g in RUNGS))
    print("V2 =", out["V2"])


def label(ratio, sig):
    if not sig:
        return "NO-GAIN"
    return "PASS" if ratio is not None and ratio <= 0.5 else "GAIN"


def cmd_test(v2):
    variants = [V1, v2]
    res = {"baseline": BASE, "variants": {"V1": V1, "V2": v2}, "bootstrap": {"resamples": B, "seed": BOOT_SEED}, "rungs": {}}
    for g in RUNGS:
        base, bpath = rows("test", BASE, g)
        dfs, dpath = rows("test", "hp-dfs", g)
        r = {"stats": {BASE: stats(base), "hp-dfs": stats(dfs)}, "sources": {BASE: bpath, "hp-dfs": dpath}, "vs_baseline": {}, "vs_hp_dfs": {}}
        for v in variants:
            cand, cpath = rows("test", v, g)
            r["stats"][v] = stats(cand)
            r["sources"][v] = cpath
            r["vs_baseline"][v] = compare(cand, base)
            r["vs_hp_dfs"][v] = compare(cand, dfs)
            r["vs_baseline"][v]["error_rows"] = r["stats"][v]["errors"]
        # Holm over the two looks, one-sided alpha 0.025.
        order = sorted(variants, key=lambda v: r["vs_baseline"][v]["p_one_sided"])
        prev = True
        for i, v in enumerate(order):
            c = r["vs_baseline"][v]
            thr = 0.025 / (2 - i)
            c["holm_threshold"] = thr
            c["holm_significant"] = prev and c["p_one_sided"] <= thr
            prev = c["holm_significant"]
            c["label"] = "INVALID" if c["error_rows"] else label(c["median_ratio"], c["holm_significant"])
            c["label_unadjusted"] = label(c["median_ratio"], c["ci95_median_log_ratio"][1] is not None and c["ci95_median_log_ratio"][1] < 0)
        res["rungs"][g] = r
    labels = {g: {v: res["rungs"][g]["vs_baseline"][v]["label"] for v in variants} for g in RUNGS}
    best = {g: ("PASS" if "PASS" in labels[g].values() else "GAIN" if "GAIN" in labels[g].values() else "NO-GAIN") for g in RUNGS}
    if all(b == "NO-GAIN" for b in best.values()):
        outcome = "killed"
    elif best["R8"] == "PASS":
        outcome = "survived"
    else:
        outcome = "inconclusive (promising, needs a variant)"
    res["labels"] = labels
    res["best_label_per_rung"] = best
    res["self_assessed_outcome"] = outcome
    with open(os.path.join(ROOT, "results", "summary.json"), "w") as f:
        json.dump(res, f, indent=1)
    for g in RUNGS:
        r = res["rungs"][g]
        print(f"== {g}")
        for s in [BASE, "hp-dfs"] + variants:
            st = r["stats"][s]
            line = (f"  {s:24} solved {st['solved']:2}/{st['instances']} KM med {st['km_median_nodes']} [q1 {st['km_q1_nodes']}, q3 {st['km_q3_nodes']}] "
                    f"wall med {st['median_wall_ms']} ms total {st['total_wall_s']} s  {st['nodes_per_s']} nodes/s")
            if s in variants:
                c = r["vs_baseline"][s]
                line += (f"\n      vs base: ratio {c['median_ratio']} CI {c['ci95_median_log_ratio']} p {c['p_one_sided']} "
                         f"W/L/T {c['wins']}/{c['losses']}/{c['ties']} wall ratio {c['median_wall_ratio']} -> {c['label']}")
            print(line)
    print("best per rung", best, "->", outcome)


if __name__ == "__main__":
    if sys.argv[1:2] == ["calibration"]:
        cmd_calibration()
    elif sys.argv[1:2] == ["test"]:
        cmd_test(sys.argv[2])
    else:
        sys.exit(__doc__)
