#!/usr/bin/env python3
"""Cut-keyed replay analysis (see ../PREREG.md). Stdlib only.

Reads results/run/primary-n{6,7}.jsonl, results/check/check.jsonl, the
committed bb-mrv rows and the census instance rows (node identity gate);
writes results/summary.json and prints the tables the README quotes.
Run from research/experiments/cut-keyed-replay/.
"""
import json
import random
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent.parent
EXP = HERE.parent
COMMITTED = [
    EXP / "mrv-baseline/results/check/bb-mrv-a16-primary.jsonl",
    EXP / "mrv-baseline/results/raw/bb-mrv/q1-r6r7.jsonl",
]
CENSUS = {6: EXP / "dead-node-census/results/run/D-primary-n6.jsonl",
          7: EXP / "dead-node-census/results/run/D-primary-n7.jsonl"}
RANDS = [f"rand{s}" for s in range(1, 6)]
ARMS = ["cut", *RANDS, "poac", "poacu"]
B, BSEED = 10_000, 20261111


def rows(p):
    return [json.loads(l) for l in open(p) if l.strip()]


def upper_median(xs):
    s = sorted(xs)
    return s[len(s) // 2]  # 16th of 30


def boot(per, stat=upper_median):
    """Percentile 95% CI of stat over instances; per is a list of values or tuples."""
    rng = random.Random(BSEED)
    n = len(per)
    vals = []
    for _ in range(B):
        smp = [per[rng.randrange(n)] for _ in range(n)]
        vals.append(stat(smp))
    vals.sort()
    return [vals[int(0.025 * B)], vals[int(0.975 * B) - 1]]


def main():
    committed = {}
    for p in COMMITTED:
        for r in rows(p):
            if r.get("ladder") == "primary" and r["n"] in (6, 7) and r.get("solver") == "bb-mrv":
                committed[(r["n"], r["seed"])] = r
    census = {}
    for n, p in CENSUS.items():
        for r in rows(p):
            if r.get("type") == "instance":
                census[(n, r["seed"])] = r["hash"]

    out = {"identity": [], "soundness": [], "rungs": {}}
    insts = {}
    for n in (6, 7):
        p = HERE / f"results/run/primary-n{n}.jsonl"
        if not p.exists():
            continue
        for r in rows(p):
            insts[(n, r["seed"])] = r
    # Identity gate.
    for k, r in sorted(insts.items()):
        c = committed.get(k)
        if c is None:
            out["identity"].append({"instance": k, "why": "no committed row"})
            continue
        for f in ("status", "nodes", "propagation", "backtracks"):
            if r[f] != c[f]:
                out["identity"].append({"instance": k, "field": f, "replay": r[f], "committed": c[f]})
        if census.get(k) != r["hash"]:
            out["identity"].append({"instance": k, "field": "hash", "replay": r["hash"], "census": census.get(k)})
        if r["prop_mismatch"]:
            out["identity"].append({"instance": k, "field": "prop_mismatch", "value": r["prop_mismatch"]})
    # Soundness gates 1 and 2 (3 is the check file).
    for k, r in sorted(insts.items()):
        for a in ARMS:
            t = r["arms"][a]
            if r["status"] == "solved" and t["found_alive"] is not True:
                out["soundness"].append({"instance": k, "arm": a, "gate": 1})
            if t["planted_alive"] is not True:
                out["soundness"].append({"instance": k, "arm": a, "gate": 2})
    chk = HERE / "results/check/check.jsonl"
    if chk.exists():
        cr = rows(chk)
        out["exhaustive"] = {
            a: {"boards": sum(1 for x in cr if x["arm"] == a),
                "kill_points_checked": sum(x["checked"] for x in cr if x["arm"] == a),
                "violations": sum(x["violations"] for x in cr if x["arm"] == a),
                "solution_lost": sum(1 for x in cr if x["arm"] == a and x["found_alive"] is False)}
            for a in ARMS}
    out["mono_total"] = {a: sum(r["arms"][a]["mono"] for r in insts.values()) for a in ARMS}

    for n in (6, 7):
        R = [insts[(n, s)] for s in range(1, 31) if (n, s) in insts]
        if not R:
            continue
        rung = {"instances": len(R), "censored": sum(r["status"] == "censored" for r in R)}

        def node_ratio(r, a):
            return r["arms"][a]["surv"] / r["nodes"]

        def work_ratio(r, a):
            return r["arms"][a]["work"] / r["base_work"]

        per = {}
        for a in ARMS:
            per[a] = [node_ratio(r, a) for r in R]
            per[a + "_work"] = [work_ratio(r, a) for r in R]
            per[a + "_calls"] = [r["arms"][a]["calls"] / r["base_calls"] for r in R]
            per[a + "_wall"] = [r["arms"][a]["ns"] / r["base_ns"] for r in R]
        per["C1"] = [sum(node_ratio(r, a) for a in RANDS) / len(RANDS) for r in R]
        per["C1_work"] = [sum(work_ratio(r, a) for a in RANDS) / len(RANDS) for r in R]
        stats = {}
        for key, xs in per.items():
            stats[key] = {"median": upper_median(xs), "ci": boot(xs),
                          "min": min(xs), "max": max(xs)}
        # Paired difference C1 - cut and difference of medians.
        pairs = list(zip(per["C1"], per["cut"]))
        diff = [a - b for a, b in pairs]
        stats["C1_minus_cut_paired"] = {"median": upper_median(diff), "ci": boot(diff)}
        dm = upper_median(per["C1"]) - upper_median(per["cut"])
        stats["C1_minus_cut_medians"] = {
            "value": dm,
            "ci": boot(pairs, lambda s: upper_median([a for a, _ in s]) - upper_median([b for _, b in s]))}
        qp = list(zip(per["poac"], per["cut"]))
        stats["poac_minus_cut_medians"] = {
            "value": upper_median(per["poac"]) - upper_median(per["cut"]),
            "ci": boot(qp, lambda s: upper_median([a for a, _ in s]) - upper_median([b for _, b in s]))}
        stats["C1_seed_medians"] = {a: upper_median(per[a]) for a in RANDS}
        # Partition diagnostics.
        diag = {}
        for a in ARMS:
            T = [r["arms"][a] for r in R]
            diag[a] = {
                "live_per_surv_pooled": sum(t["live_sum"] for t in T) / max(1, sum(t["surv"] for t in T)),
                "made_per_rekey_pooled": sum(t["made"] for t in T) / max(1, sum(t["rekeys"] for t in T)),
                "rekeys": sum(t["rekeys"] for t in T),
                "mismatch": sum(t["mismatch"] for t in T),
                "calls": sum(t["calls"] for t in T),
                "kill_dead": sum(t["kill_dead"] for t in T),
                "kill_prune": sum(t["kill_prune"] for t in T),
            }
        diag["poac_runs"] = sum(r["arms"]["poac"]["poac_runs"] for r in R)
        diag["poacu_runs"] = sum(r["arms"]["poacu"]["poac_runs"] for r in R)
        diag["base_calls"] = sum(r["base_calls"] for r in R)
        # By censoring status.
        by = {}
        for st in ("solved", "censored", "exhausted"):
            S = [r for r in R if r["status"] == st]
            if S:
                by[st] = {"count": len(S),
                          "cut": upper_median([node_ratio(r, "cut") for r in S]),
                          "C1": upper_median([sum(node_ratio(r, a) for a in RANDS) / 5 for r in S]),
                          "poac": upper_median([node_ratio(r, "poac") for r in S])}
        rung["stats"] = stats
        rung["diag"] = diag
        rung["by_status"] = by
        rung["per_instance"] = [
            {"seed": r["seed"], "status": r["status"], "nodes": r["nodes"],
             **{a: round(node_ratio(r, a), 4) for a in ARMS},
             "C1": round(sum(node_ratio(r, a) for a in RANDS) / 5, 4),
             "cut_work": round(work_ratio(r, "cut"), 4)} for r in R]
        out["rungs"][f"R{n}"] = rung

    # Kill criteria.
    if "R6" in out["rungs"] and "R7" in out["rungs"]:
        s6, s7 = out["rungs"]["R6"]["stats"], out["rungs"]["R7"]["stats"]
        k1 = s7["cut"]["median"] > 0.5
        k1i = k1 or s7["cut_work"]["median"] > 1
        d6 = s6["C1"]["median"] - s6["cut"]["median"]
        d7 = s7["C1"]["median"] - s7["cut"]["median"]
        k2 = d6 <= 0.10 and d7 <= 0.10
        k2lit = abs(d6) <= 0.10 and abs(d7) <= 0.10
        k3 = s7["poac"]["median"] <= s7["cut"]["median"]
        out["kill"] = {
            "K1": {"fires": k1, "R7_cut_node_median": s7["cut"]["median"], "threshold": 0.5},
            "K1_IDEAS": {"fires": k1i, "R7_cut_work_median": s7["cut_work"]["median"]},
            "K2": {"fires": k2, "R6_C1_minus_cut": d6, "R7_C1_minus_cut": d7, "threshold": 0.10},
            "K2_literal_two_sided": {"fires": k2lit},
            "K3": {"fires": k3, "R7_poac_median": s7["poac"]["median"], "R7_cut_median": s7["cut"]["median"],
                   "R6_poac_median": s6["poac"]["median"], "R6_cut_median": s6["cut"]["median"]},
            "killed": k1 or k2 or k3,
        }
    (HERE / "results/summary.json").write_text(json.dumps(out, indent=1, default=list) + "\n")
    # Print the headline.
    print("identity mismatches:", len(out["identity"]), " soundness violations:", len(out["soundness"]))
    if "exhaustive" in out:
        print("exhaustive:", {a: (v["kill_points_checked"], v["violations"]) for a, v in out["exhaustive"].items()})
    print("mono:", out["mono_total"])
    for rn, rung in out["rungs"].items():
        st = rung["stats"]
        print(f"\n{rn}: {rung['instances']} instances, {rung['censored']} censored")
        for a in [*ARMS, "C1"]:
            w = st.get(a + "_work")
            ws = f" work {w['median']:.3f} [{w['ci'][0]:.3f}, {w['ci'][1]:.3f}]" if w else ""
            print(f"  {a:6s} node {st[a]['median']:.3f} [{st[a]['ci'][0]:.3f}, {st[a]['ci'][1]:.3f}]{ws}")
        print("  C1-cut medians", st["C1_minus_cut_medians"], " paired", st["C1_minus_cut_paired"])
        print("  poac-cut medians", st["poac_minus_cut_medians"])
        print("  diag", json.dumps(rung["diag"]))
        print("  by status", rung["by_status"])
    if "kill" in out:
        print("\nkill:", json.dumps(out["kill"], indent=1))
    return 0


if __name__ == "__main__":
    sys.exit(main())
