#!/usr/bin/env python3
"""Stage 2 (PREREG.md): G1 per rung with Holm over the two variants.

Reads results/test/<ladder>/<solver>.jsonl (bb-* solvers and the R8
baselines) and the committed hard-ladder rows for R6/R7, restricted to the
test seeds that the candidates ran. Writes results/summary.json and
results/pairs.csv. Standard library only.
"""
import csv
import json
import math
import random
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LADDER = ROOT.parent / "hard-ladder"
sys.path.insert(0, str(LADDER / "analysis"))
from analyze import km_quantile, log_ratio, upper_median, wilson, pct  # noqa: E402

B = 10_000
BOOT_SEED = 20261111
INF = math.inf
RUNGS = {"R6": ("primary", 6), "R7": ("primary", 7), "R8": ("8-c10", 8)}
VARIANTS = ["bb-learned", "bb-single"]
DYNAMIC = ["bb-mrv", "bb-impact", "scb-gacolor_ac3_random"]
BASELINES = DYNAMIC + ["hp-dfs"]
RANK = {"PASS": 2, "GAIN": 1, "NO-GAIN": 0}


def load(p):
    return [json.loads(l) for l in Path(p).read_text().splitlines() if l.strip()]


def rows(solver, ladder, n):
    p = ROOT / "results" / "test" / ladder / f"{solver}.jsonl"
    if not p.exists():
        p = LADDER / "results" / "test" / ladder / f"{solver}.jsonl"
    if not p.exists():
        return None
    return {r["instance"]: r for r in load(p) if r["n"] == n}


def time_censored(r):
    return r["status"] == "censored" and r["nodes_counted"] < r["cap"]


def lr(c, b):
    if c["status"] == "solved" and time_censored(b):
        return 0.0
    return log_ratio(c, b)


def compare(cand, base, ids):
    v = [lr(cand[i], base[i]) for i in ids]
    rng = random.Random(BOOT_SEED)
    boots = [upper_median([v[rng.randrange(len(v))] for _ in v]) for _ in range(B)]
    med = upper_median(v)
    p = min(1.0, 2 * sum(x >= 0 for x in boots) / B)
    fin = lambda x: None if math.isinf(x) else x  # noqa: E731
    return {"median_log_ratio": fin(med), "median_ratio": None if math.isinf(med) else math.exp(med),
            "ci95": [fin(pct(boots, 0.025)), fin(pct(boots, 0.975))], "p_boot": p, "log_ratios": v}


def holm(ps, alpha=0.05):
    order = sorted(range(len(ps)), key=lambda k: ps[k])
    rej = [False] * len(ps)
    for rank, k in enumerate(order):
        if ps[k] <= alpha / (len(ps) - rank):
            rej[k] = True
        else:
            break
    return rej


def stats(rs):
    v = [r["nodes"] if r["status"] == "solved" else INF for r in rs]
    k = sum(r["status"] == "solved" for r in rs)
    fin = lambda x: None if math.isinf(x) else x  # noqa: E731
    walls = sorted(r["wall_ms"] for r in rs)
    return {"runs": len(rs), "solved": k, "wilson95": list(wilson(k, len(rs))),
            "km_q1": fin(km_quantile(v, 0.25)), "km_median": fin(km_quantile(v, 0.5)), "km_q3": fin(km_quantile(v, 0.75)),
            "median_wall_ms": walls[len(walls) // 2], "total_wall_s": sum(walls) / 1000,
            "total_nodes_counted": sum(r["nodes_counted"] for r in rs),
            "propagation_calls": sum(r["propagation"] or 0 for r in rs),
            "time_censored": sum(time_censored(r) for r in rs),
            "time_censored_ids": [r["instance"] for r in rs if time_censored(r)],
            "errors": sum(r["status"] == "error" for r in rs)}


def main():
    out = {"rungs": {}}
    pair_rows = []
    for rung, (ladder, n) in RUNGS.items():
        data = {s: rows(s, ladder, n) for s in VARIANTS + BASELINES}
        absent = [s for s, d in data.items() if d is None]
        if absent:
            out["rungs"][rung] = {"ladder": ladder, "n": n, "incomplete": True, "missing_solvers": absent}
            print(f"== {rung} INCOMPLETE, missing {absent}")
            continue
        ids = sorted(data["bb-learned"], key=lambda i: data["bb-learned"][i]["seed"])
        for s, d in data.items():
            missing = [i for i in ids if i not in d]
            if missing:
                sys.exit(f"{rung} {s}: missing {missing}")
            if any(d[i]["cap"] != 1_000_000 for i in ids):
                sys.exit(f"{rung} {s}: cap mismatch")
        R = {"ladder": ladder, "n": n, "seeds": [data["bb-learned"][i]["seed"] for i in ids],
             "solvers": {s: stats([data[s][i] for i in ids]) for s in data}, "g1": {}}
        for b in BASELINES:
            cmp = {v: compare(data[v], data[b], ids) for v in VARIANTS}
            rej = holm([cmp[v]["p_boot"] for v in VARIANTS])
            for v, rj in zip(VARIANTS, rej):
                c = cmp[v]
                ratio = c["median_ratio"]
                c["holm_reject"] = rj
                c["label"] = ("PASS" if ratio is not None and ratio <= 0.5 else "GAIN") if rj else "NO-GAIN"
                c["unadjusted_g1_pass"] = ratio is not None and ratio <= 0.5 and c["ci95"][1] is not None and c["ci95"][1] < 0
                for i, x in zip(ids, c["log_ratios"]):
                    pair_rows.append([rung, v, b, i, data[v][i]["status"], data[v][i]["nodes_counted"],
                                      data[b][i]["status"], data[b][i]["nodes_counted"], x])
                del c["log_ratios"]
            R["g1"][b] = cmp
        R["label_vs_strongest"] = {
            v: min((R["g1"][b][v]["label"] for b in DYNAMIC), key=RANK.get) for v in VARIANTS}
        R["label_vs_hp_dfs"] = {v: R["g1"]["hp-dfs"][v]["label"] for v in VARIANTS}
        out["rungs"][rung] = R
    s1 = json.loads((ROOT / "results" / "stage1" / "summary.json").read_text())
    k1 = s1["K1_fires"]
    done = [r for r in RUNGS if not out["rungs"][r].get("incomplete")]
    k2 = None if "R8" not in done else not any(
        out["rungs"]["R8"]["label_vs_strongest"][v] == "PASS" for v in VARIANTS)
    k3 = all(out["rungs"][r]["label_vs_strongest"][v] == "NO-GAIN" for r in done for v in VARIANTS)
    out["kill"] = {"K1": k1, "K2": k2, "K3": k3, "killed": bool(k1 or k2 or k3), "rungs_complete": done,
                   "stage1_delta": s1["delta"], "stage1_delta_ci": s1["delta_ci"]}
    (ROOT / "results" / "summary.json").write_text(json.dumps(out, indent=1) + "\n")
    with open(ROOT / "results" / "pairs.csv", "w", newline="") as f:
        w = csv.writer(f)
        w.writerow(["rung", "variant", "baseline", "instance", "cand_status", "cand_nodes", "base_status", "base_nodes", "log_ratio"])
        w.writerows(pair_rows)
    for rung, R in out["rungs"].items():
        if R.get("incomplete"):
            continue
        print(f"== {rung} seeds {R['seeds'][0]}..{R['seeds'][-1]} ({len(R['seeds'])})")
        for s, st in R["solvers"].items():
            print(f"  {s:24s} solved {st['solved']:2d}/{st['runs']}  KM med {st['km_median']}  "
                  f"q1 {st['km_q1']} q3 {st['km_q3']}  wall med {st['median_wall_ms']:.0f} ms tot {st['total_wall_s']:.0f} s  "
                  f"tcens {st['time_censored']}")
        for b, cmp in R["g1"].items():
            for v, c in cmp.items():
                print(f"  {v:10s} vs {b:24s} ratio {c['median_ratio']}  CI {c['ci95']}  p {c['p_boot']:.4f}  {c['label']}")
        print("  vs strongest:", R["label_vs_strongest"])
    print("kill:", out["kill"])


if __name__ == "__main__":
    main()
