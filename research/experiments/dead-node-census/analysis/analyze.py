#!/usr/bin/env python3
"""Dead-node refutation census analysis (PREREG.md "Analysis").

Reads results/run/*.jsonl (census rows), results/gate/*.jsonl (soundness gate)
and the committed bb-mrv rows (node identity gate); writes
results/summary.json and results/tables.md. Stdlib only.
"""
import glob
import json
import os
import random
import statistics
import sys
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
EXP = os.path.dirname(ROOT)
RUNGS = {("primary", 6): "R6", ("primary", 7): "R7", ("plus2", 7): "N7"}
ORDER = ["R6", "R7", "N7"]
B_SEED = 20261111
B_REPS = 10000

COMMITTED = [
    "mrv-baseline/results/check/bb-mrv-a16-primary.jsonl",
    "mrv-baseline/results/raw/bb-mrv/q1-r6r7.jsonl",
    "mrv-baseline/results/raw/bb-mrv/q2-plus2.jsonl",
]


def load(pattern):
    rows = []
    for f in sorted(glob.glob(os.path.join(ROOT, pattern))):
        with open(f) as fh:
            for l in fh:
                if not l.endswith("\n"):
                    continue  # a line still being written by a running stage
                rows.append(json.loads(l))
    return rows


def committed():
    ref = {}
    for f in COMMITTED:
        with open(os.path.join(EXP, f)) as fh:
            for l in fh:
                r = json.loads(l)
                ref[(r["ladder"], r["n"], r["seed"])] = r
    return ref


def identity(inst_rows):
    ref = committed()
    bad, checked = [], 0
    for r in inst_rows:
        key = (r["ladder"], r["n"], r["seed"])
        c = ref.get(key)
        if c is None:
            bad.append({"instance": key, "why": "no committed row"})
            continue
        checked += 1
        for f in ("status", "nodes", "propagation", "backtracks"):
            if r[f] != c[f]:
                bad.append({"instance": key, "field": f, "census": r[f], "committed": c[f]})
        if r["hash_p2"] != r["hash"] or (r.get("hash_p3") not in (None, r["hash"])):
            bad.append({"instance": key, "why": "replay hash differs"})
    return {"checked": checked, "mismatches": bad}


# ------------------------------------------------------------------ indicators

def ind_defs():
    """name -> (field needed, function(row) -> 0/1)."""
    d = {}
    for v in ("b_pool", "b_dom"):
        for k in (2, 3, 4):
            d[f"{v}_r<={k}"] = (v, lambda r, v=v, k=k: int(r[v]["r"] is not None and r[v]["r"] <= k))
    for k in (1, 2, 3):
        d[f"a{k}"] = (f"a{k}", lambda r, k=k: int(r[f"a{k}"]["dead"]))
        d[f"a{k}_rows"] = (f"a{k}", lambda r, k=k: int(r[f"a{k}"]["dead_rows"]))
    d["e"] = ("e", lambda r: int(r["e"]["dead"]))
    for arm in ("cut", "rand", "poac", "poac_mrv"):
        d[f"d_{arm}"] = ("d", lambda r, arm=arm: int(r["d"][arm]["dead"]))
    d["pocket"] = ("pocket", lambda r: int(r["pocket"]["dead"]))
    return d


def per_instance_share(nodes, f, weighted):
    num = den = 0.0
    for r in nodes:
        wt = r["w"] * (r["size"] if weighted else 1)
        num += wt * f(r)
        den += wt
    return num / den if den else None


def boot_mean(vals, seed=B_SEED, reps=B_REPS):
    rng = random.Random(seed)
    n = len(vals)
    if n == 0:
        return None, None, None
    m = sum(vals) / n
    bs = sorted(sum(vals[rng.randrange(n)] for _ in range(n)) / n for _ in range(reps))
    return m, bs[int(0.025 * reps)], bs[int(0.975 * reps) - 1]


def wmedian(pairs):
    """Weighted median of (value, weight); value None = inf (ordered last)."""
    key = lambda p: float("inf") if p[0] is None else p[0]
    pairs = sorted(pairs, key=key)
    tot = sum(w for _, w in pairs)
    acc = 0.0
    for v, w in pairs:
        acc += w
        if acc >= tot / 2:
            return v if v is not None else "inf"
    return None


def main():
    run = load("results/run/*.jsonl")
    gate = load("results/gate/gate-*.jsonl")
    fuzz = load("results/gate/fuzz*.jsonl")
    inst_rows = [r for r in run if r["type"] == "instance"]
    node_rows = [r for r in run if r["type"] == "node"]
    out = {"identity": identity(inst_rows)}

    # Merge node rows of the same (instance, id) across oracle stages.
    merged = {}
    for r in node_rows:
        k = (r["ladder"], r["n"], r["seed"], r["id"])
        merged.setdefault(k, {}).update(r)
    by_inst = defaultdict(list)
    for (lad, n, seed, _), r in merged.items():
        by_inst[(lad, n, seed)].append(r)
    insts = {}
    for r in inst_rows:
        insts[(r["ladder"], r["n"], r["seed"])] = r

    # Soundness gate per oracle.
    defs = ind_defs()
    gate_v = defaultdict(lambda: [0, 0])
    for g in gate:
        o = g["oracles"]
        for name, (field, f) in defs.items():
            if field in o:
                gate_v[name][0] += 1
                o2 = dict(o)
                gate_v[name][1] += f(o2)
    out["gate"] = {
        "live_states": len(gate),
        "instances": len({(g["ladder"], g["n"], g["seed"]) for g in gate}),
        "per_oracle": {k: {"checks": v[0], "violations": v[1]} for k, v in gate_v.items()},
        "fuzz": [f for f in fuzz if "check" in f],
        "fuzz_violation_rows": [f for f in fuzz if "violation" in f],
    }

    # Per rung shares.
    rungs = {}
    for rung in ORDER:
        keys = sorted(k for k in by_inst if RUNGS.get(k[:2]) == rung)
        ikeys = sorted(k for k in insts if RUNGS.get(k[:2]) == rung)
        res = {"instances_run": len(ikeys), "instances_with_dead": len(keys)}
        res["dead_total"] = sum(insts[k]["dead_total"] for k in ikeys)
        res["sampled"] = sum(len(by_inst[k]) for k in keys)
        res["status"] = dict(
            (s, sum(1 for k in ikeys if insts[k]["status"] == s)) for s in ("solved", "censored", "exhausted")
        )
        shares = {}
        for name, (field, f) in defs.items():
            ks = [k for k in keys if all(field in r for r in by_inst[k])]
            if not ks:
                continue
            row = {"instances": len(ks)}
            for wtd in (True, False):
                vals = [per_instance_share(by_inst[k], f, wtd) for k in ks]
                m, lo, hi = boot_mean(vals)
                row["weighted" if wtd else "unweighted"] = {"mean": m, "lo": lo, "hi": hi}
            shares[name] = row
        res["shares"] = shares
        # Work per check.
        work = {}
        for field, wkey in [("b_pool", "work"), ("b_dom", "work"), ("a1", "work"), ("a2", "work"), ("a3", "work"),
                            ("e", "work"), ("pocket", "work")]:
            rs = [r for k in keys for r in by_inst[k] if field in r]
            if not rs:
                continue
            nsn = [insts[(r["ladder"], r["n"], r["seed"])]["ns_per_node"] for r in rs]
            work[field] = {
                "median_work": statistics.median(r[field][wkey] for r in rs),
                "median_ns": statistics.median(r[field]["ns"] for r in rs),
                "median_ns_over_subtree_ns": statistics.median(
                    r[field]["ns"] / (r["size"] * x) for r, x in zip(rs, nsn)),
            }
        rs = [r for k in keys for r in by_inst[k] if "d" in r]
        if rs:
            for arm in ("cut", "rand", "poac", "poac_mrv"):
                work[f"d_{arm}"] = {
                    "median_calls": statistics.median(r["d"][arm]["calls"] for r in rs),
                    "median_calls_over_subtree": statistics.median(r["d"][arm]["calls"] / r["size"] for r in rs),
                    "key_found_share": sum(r["d"][arm]["key"] for r in rs) / len(rs),
                }
            work["d_ns_all_arms"] = statistics.median(r["d"]["ns"] for r in rs)
        rs = [r for k in keys for r in by_inst[k] if "b_pool" in r]
        if rs:
            work["b_pool_budget_outs"] = sum(r["b_pool"]["budget_outs"] for r in rs)
            work["b_dom_budget_outs"] = sum(r["b_dom"]["budget_outs"] for r in rs)
        rs = [r for k in keys for r in by_inst[k] if "e" in r]
        if rs:
            work["e_overflows"] = sum(r["e"]["overflow"] for r in rs)
        rs = [r for k in keys for r in by_inst[k] if "pocket" in r]
        if rs:
            work["pocket_overflows"] = sum(r["pocket"]["overflow"] for r in rs)
        res["work"] = work
        # B-pool recurrence and r distribution.
        ks = [k for k in keys if all("b_pool" in r for r in by_inst[k])]
        if ks:
            rec = {}
            for fld in ("raw_rec", "can_rec", "raw_sound", "can_sound"):
                for wtd in (True, False):
                    vals = []
                    for k in ks:
                        ref = [r for r in by_inst[k] if r["b_pool"]["r"] is not None]
                        if ref:
                            vals.append(per_instance_share(ref, lambda r: int(r["b_pool"].get(fld, 0) > 0), wtd))
                    m, lo, hi = boot_mean(vals)
                    rec[f"{fld}_{'weighted' if wtd else 'unweighted'}"] = {"mean": m, "lo": lo, "hi": hi,
                                                                             "instances": len(vals)}
            mult = [r["b_pool"].get("raw_rec", 0) for k in ks for r in by_inst[k] if r["b_pool"]["r"] is not None]
            rec["raw_rec_multiplicity_median"] = statistics.median(mult) if mult else None
            res["b_pool_recurrence"] = rec
            for v in ("b_pool", "b_dom"):
                pairs = []
                dist = defaultdict(float)
                for k in ks:
                    tot = sum(r["w"] * r["size"] for r in by_inst[k])
                    for r in by_inst[k]:
                        wt = r["w"] * r["size"] / tot
                        pairs.append((r[v]["r"], wt))
                        dist[str(r[v]["r"] if r[v]["r"] is not None else "inf")] += wt / len(ks)
                res[f"{v}_r_weighted_median"] = wmedian(pairs)
                res[f"{v}_r_distribution_weighted"] = dict(sorted(dist.items()))
        # ST local-core rate (pocket on sampled nodes with size >= 200).
        ks = [k for k in keys if all("pocket" in r for r in by_inst[k])]
        if ks:
            rates = []
            for k in ks:
                big = [r for r in by_inst[k] if r["size"] >= 200]
                if big:
                    rates.append(per_instance_share(big, lambda r: int(r["pocket"]["dead"]), False))
            res["pocket_local_core_rate"] = {"median": statistics.median(rates) if rates else None,
                                             "instances": len(rates)}
            ref = [r for k in ks for r in by_inst[k] if r["pocket"]["dead"]]
            res["pocket_recurrence_sampled_share"] = (
                sum(r["pocket"].get("rec_sampled", 0) > 0 for r in ref) / len(ref) if ref else None)
        # By depth band (weighted shares pooled by band, for description).
        bands = {}
        for name in ("b_pool_r<=3", "a2", "e", "d_cut", "pocket"):
            field, f = defs[name]
            acc = defaultdict(lambda: [0.0, 0.0])
            for k in keys:
                cells = k[1] * k[1]
                tot = sum(r["w"] * r["size"] for r in by_inst[k])
                for r in by_inst[k]:
                    if field not in r:
                        continue
                    b = "early" if r["depth"] < cells / 3 else ("mid" if r["depth"] < 2 * cells / 3 else "late")
                    wt = r["w"] * r["size"] / tot
                    acc[b][0] += wt * f(r)
                    acc[b][1] += wt
            if acc:
                bands[name] = {b: (v[0] / v[1] if v[1] else None) for b, v in sorted(acc.items())}
        res["by_depth_band_weighted"] = bands
        rungs[rung] = res
    out["rungs"] = rungs
    out["verdicts"] = verdicts(rungs)
    with open(os.path.join(ROOT, "results", "summary.json"), "w") as fh:
        json.dump(out, fh, indent=1, sort_keys=True)
    with open(os.path.join(ROOT, "results", "tables.md"), "w") as fh:
        fh.write(tables(out))
    print(tables(out))


def sh(rungs, rung, name, kind="weighted"):
    try:
        return rungs[rung]["shares"][name][kind]
    except KeyError:
        return None


def verdicts(rungs):
    v = {}
    # GC / T2.
    s = sh(rungs, "R7", "b_pool_r<=3")
    rec = rungs.get("R7", {}).get("b_pool_recurrence", {}).get("raw_rec_weighted")
    mr7 = rungs.get("R7", {}).get("b_pool_r_weighted_median")
    mn7 = rungs.get("N7", {}).get("b_pool_r_weighted_median")
    if s is None:
        v["T2_GC"] = "not reached"
    else:
        rank = lambda x: float("inf") if x in ("inf", None) else x
        k0a = s["mean"] < 0.30
        k0b = rec is not None and rec["mean"] is not None and rec["mean"] < 0.10
        k0c = mn7 is not None and rank(mr7) <= rank(mn7)
        v["T2_GC"] = {"K0a": {"stat": s["mean"], "threshold": "<0.30", "fires": k0a},
                      "K0b": {"stat": rec["mean"] if rec else None, "threshold": "<0.10", "fires": k0b},
                      "K0c": {"R7_median_r": mr7, "N7_median_r": mn7, "fires": k0c},
                      "killed": k0a or k0b or k0c}
    # TW / T5.
    a2r6, a2r7 = sh(rungs, "R6", "a2"), sh(rungs, "R7", "a2")
    a1r6, a1r7 = sh(rungs, "R6", "a1"), sh(rungs, "R7", "a1")
    if a2r6 is None or a2r7 is None:
        v["T5_TW"] = "not reached"
    else:
        census = a2r6["mean"] < 0.05 and a2r7["mean"] < 0.05
        mech = all(a2["lo"] <= a1["mean"] <= a2["hi"] for a1, a2 in ((a1r6, a2r6), (a1r7, a2r7)))
        v["T5_TW"] = {"census_kill": {"R6": a2r6["mean"], "R7": a2r7["mean"], "threshold": "<0.05 at both",
                                      "fires": census},
                      "mechanism_kill": {"R6_a1": a1r6["mean"], "R6_a2_ci": [a2r6["lo"], a2r6["hi"]],
                                         "R7_a1": a1r7["mean"], "R7_a2_ci": [a2r7["lo"], a2r7["hi"]], "fires": mech},
                      "killed": census or mech}
    # CO / T3.
    c6, c7 = sh(rungs, "R6", "d_cut"), sh(rungs, "R7", "d_cut")
    r6, r7 = sh(rungs, "R6", "d_rand"), sh(rungs, "R7", "d_rand")
    p7 = sh(rungs, "R7", "d_poac")
    if c6 is None or c7 is None:
        v["T3_CO"] = "not reached"
    else:
        k2 = abs(r6["mean"] - c6["mean"]) <= 0.10 and abs(r7["mean"] - c7["mean"]) <= 0.10
        k3 = p7["mean"] >= c7["mean"]
        v["T3_CO"] = {"K2": {"R6_cut": c6["mean"], "R6_rand": r6["mean"], "R7_cut": c7["mean"],
                             "R7_rand": r7["mean"], "fires": k2},
                      "K3": {"R7_cut": c7["mean"], "R7_poac": p7["mean"], "fires": k3},
                      "K1": "not decided by the census",
                      "killed": k2 or k3}
    # ST.
    lc = rungs.get("R7", {}).get("pocket_local_core_rate")
    if lc is None:
        v["ST"] = "not reached"
    else:
        v["ST"] = {"local_core_rate_R7_median": lc["median"], "threshold": "<0.10",
                   "fires": lc["median"] is not None and lc["median"] < 0.10,
                   "projected_saving": "not decided by the census"}
        v["ST"]["killed"] = v["ST"]["fires"]
    # OC.
    e7 = sh(rungs, "R7", "e")
    v["OC"] = "no kill decided by the census" + ("" if e7 is None else f"; E floor at R7 = {e7['mean']:.4f}")
    return v


def fmt(x, nd=3):
    if x is None:
        return "-"
    if isinstance(x, float):
        return f"{x:.{nd}f}"
    return str(x)


def tables(out):
    L = []
    idt = out["identity"]
    L.append(f"Node identity gate: {idt['checked']} instances checked, {len(idt['mismatches'])} mismatches.\n")
    g = out["gate"]
    L.append(f"Soundness gate: {g['live_states']} live states on {g['instances']} instances.\n")
    L.append("| oracle | live checks | violations |\n|---|---|---|")
    for k, v in sorted(g["per_oracle"].items()):
        L.append(f"| {k} | {v['checks']} | {v['violations']} |")
    L.append("\nFuzz against brute force:\n\n| check | cases | violations |\n|---|---|---|")
    for f in g["fuzz"]:
        L.append(f"| {f['check']} | {f['cases']} | {f['violations']} |")
    for rung in ORDER:
        r = out["rungs"].get(rung)
        if not r:
            continue
        L.append(f"\n## {rung}\n")
        L.append(f"Instances {r['instances_run']} ({r['status']}), with dead nodes {r['instances_with_dead']}, "
                 f"dead nodes {r['dead_total']}, sampled {r['sampled']}.\n")
        L.append("| oracle | instances | weighted share [95% CI] | unweighted share [95% CI] |\n|---|---|---|---|")
        for name, s in r["shares"].items():
            w, u = s["weighted"], s["unweighted"]
            L.append(f"| {name} | {s['instances']} | {fmt(w['mean'])} [{fmt(w['lo'])}, {fmt(w['hi'])}] | "
                     f"{fmt(u['mean'])} [{fmt(u['lo'])}, {fmt(u['hi'])}] |")
        L.append("\n| work | values |\n|---|---|")
        for k, v in r["work"].items():
            if isinstance(v, dict):
                L.append(f"| {k} | " + ", ".join(f"{a} {fmt(b)}" for a, b in v.items()) + " |")
            else:
                L.append(f"| {k} | {fmt(v)} |")
        if "b_pool_recurrence" in r:
            L.append("\nB-pool key recurrence among refuted sampled nodes (weighted unless stated):\n")
            L.append("| statistic | mean [95% CI] | instances |\n|---|---|---|")
            for k, v in r["b_pool_recurrence"].items():
                if isinstance(v, dict):
                    L.append(f"| {k} | {fmt(v['mean'])} [{fmt(v['lo'])}, {fmt(v['hi'])}] | {v['instances']} |")
                else:
                    L.append(f"| {k} | {fmt(v)} | |")
            for v in ("b_pool", "b_dom"):
                L.append(f"\n{v}: weighted median r = {r.get(v + '_r_weighted_median')}; weighted distribution "
                         + ", ".join(f"r={a}: {fmt(b)}" for a, b in r.get(v + "_r_distribution_weighted", {}).items()))
        if "pocket_local_core_rate" in r:
            L.append(f"\nPocket local-core rate (s >= 200): median {fmt(r['pocket_local_core_rate']['median'])} over "
                     f"{r['pocket_local_core_rate']['instances']} instances; recurrence among sampled refuted pockets "
                     f"{fmt(r.get('pocket_recurrence_sampled_share'))}.")
        L.append("\nWeighted share by depth band (pooled):\n")
        for k, v in r["by_depth_band_weighted"].items():
            L.append(f"- {k}: " + ", ".join(f"{a} {fmt(b)}" for a, b in v.items()))
    L.append("\n## Verdicts\n")
    L.append("```\n" + json.dumps(out["verdicts"], indent=1) + "\n```")
    return "\n".join(L) + "\n"


if __name__ == "__main__":
    sys.exit(main())
