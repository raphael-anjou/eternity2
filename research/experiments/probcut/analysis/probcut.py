#!/usr/bin/env python3
"""ProbCut analysis (stdlib only). See ../PREREG.md.

  probcut.py calibrate   stage 1 + Luby unit + params.json + CALIBRATION.md
  probcut.py report      stage 2: per-rung metrics, G1, Holm, kill, summary.json
"""
import json
import math
import os
import random
import statistics
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
LADDER = os.path.join(ROOT, "..", "hard-ladder")
sys.path.insert(0, os.path.join(LADDER, "analysis"))
from analyze import INF, km_quantile, log_ratio, obs, pct, upper_median, wilson  # noqa: E402

BOOT_SEED = 20261111
B = 10_000
Z = {0.2: 0.841621233572914, 0.05: 1.644853626951472, 0.01: 2.326347874040841}
RUNGS = [(6, 5, "primary"), (7, 6, "primary"), (8, 10, "8-c10")]
LUBY_UNITS = [100, 1000, 10000]


def load(path):
    with open(path) as f:
        return [json.loads(line) for line in f if line.strip()]


def res(*parts):
    return os.path.join(ROOT, "results", *parts)


def stage1_rows(role, tag="stage1"):
    rows = load(res(role, f"{tag}-primary-67.jsonl")) + load(res(role, f"{tag}-8-c10.jsonl"))
    return rows


def set_auc(vals, labels):
    pos = [v for v, l in zip(vals, labels) if l]
    neg = [v for v, l in zip(vals, labels) if not l]
    if not pos or not neg:
        return None
    s = sum((p > q) + 0.5 * (p == q) for p in pos for q in neg)
    return s / (len(pos) * len(neg))


def sibling_sets(rows, n, d):
    by = {}
    for r in rows:
        if r["n"] == n and r["d"] == d:
            by.setdefault(r["instance"], []).append(r)
    return [s for _, s in sorted(by.items()) if len(s) >= 2 and any(r["planted"] for r in s) and not all(r["planted"] for r in s)]


def mean_auc(sets, j):
    a = [set_auc([r["v"][j] for r in s], [r["planted"] for r in s]) for s in sets]
    return statistics.mean(a) if a else None, a


def r2_sigma(xs, ys):
    if len(xs) < 3 or len(set(xs)) < 2:
        return None, None
    mx, my = statistics.mean(xs), statistics.mean(ys)
    sxx = sum((x - mx) ** 2 for x in xs)
    slope = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx
    b = my - slope * mx
    sse = sum((y - slope * x - b) ** 2 for x, y in zip(xs, ys))
    sst = sum((y - my) ** 2 for y in ys)
    return (1 - sse / sst if sst > 0 else None), math.sqrt(sse / (len(xs) - 2))


def planted_v(rows, n, d, j):
    """seed -> v_j of the planted-prefix child (first planted child)."""
    out = {}
    for r in rows:
        if r["n"] == n and r["d"] == d and r["planted"] and r["seed"] not in out:
            out[r["seed"]] = r["v"][j]
    return out


def mu_sd(vals):
    return statistics.mean(vals), (statistics.stdev(vals) if len(vals) > 1 else 0.0)


def cmd_calibrate():
    rows = stage1_rows("calibration")
    out = {"rungs": {}}
    params = {"rungs": []}
    for n, c, ladder in RUNGS:
        depths = [n, 2 * n, 3 * n]
        rr = {"n": n, "c": c, "depths": {}}
        pooled_sets = []
        for d in depths:
            sets = sibling_sets(rows, n, d)
            pooled_sets += sets
            dep = {"sibling_sets": len(sets), "planted_parent_children": [
                sum(1 for r in rows if r["n"] == n and r["d"] == d and r["seed"] == s) for s in range(1001, 1011)]}
            dep["auc"] = [mean_auc(sets, j)[0] for j in range(5)]
            best = None
            if sets:
                best = max(range(1, 5), key=lambda j: (dep["auc"][j], -j))
                rng = random.Random(7)
                ctrl = []
                for _ in range(1000):
                    sh = []
                    for s in sets:
                        lab = [False] * len(s)
                        lab[rng.randrange(len(s))] = True
                        sh.append(set_auc([r["v"][best] for r in s], lab))
                    ctrl.append(statistics.mean(sh))
                dep["shuffled_control_auc_best_k"] = statistics.mean(ctrl)
                _, ab = mean_auc(sets, best)
                _, a0 = mean_auc(sets, 0)
                diffs = [x - y for x, y in zip(ab, a0)]
                rng = random.Random(BOOT_SEED)
                boots = [statistics.mean([diffs[rng.randrange(len(diffs))] for _ in diffs]) for _ in range(B)]
                dep["best_k"] = best
                dep["auc_gain_best_minus_v0"] = statistics.mean(diffs)
                dep["auc_gain_ci95"] = [pct(boots, 0.025), pct(boots, 0.975)]
            at_d = [r for r in rows if r["n"] == n and r["d"] == d]
            dep["children"] = len(at_d)
            dep["l8_capped"] = sum(r["l8_capped"] for r in at_d)
            dep["v_capped"] = sum(any(r["v_capped"]) for r in at_d)
            dep["regression"] = {}
            for j in range(5):
                r2, sig = r2_sigma([r["v"][j] for r in at_d], [r["l8"] for r in at_d])
                dep["regression"][f"v{j}"] = {"r2": r2, "sigma": sig}
            dep["regression_best_k"] = dep["regression"][f"v{best}"] if best else {"r2": None, "sigma": None}
            rr["depths"][d] = dep
        pooled = [mean_auc(pooled_sets, j)[0] for j in range(5)]
        k = max(range(1, 5), key=lambda j: (pooled[j], -j)) if pooled_sets else 1
        rr["pooled_auc"] = pooled
        rr["pooled_sibling_sets"] = len(pooled_sets)
        rr["selected_k"] = k
        # held-out false-cut rate of t(d, p) for the selected k
        fc = {}
        for p in (0.05, 0.2):
            hits, tot, per_d = 0, 0, {}
            for d in depths:
                pv = planted_v(rows, n, d, k)
                h = t = 0
                for fit, held in ((range(1001, 1006), range(1006, 1011)), (range(1006, 1011), range(1001, 1006))):
                    m, s = mu_sd([pv[x] for x in fit if x in pv])
                    thr = m - Z[p] * s
                    for x in held:
                        if x in pv:
                            t += 1
                            h += pv[x] < thr
                per_d[d] = h / t if t else None
                hits += h
                tot += t
            fc[str(p)] = {"pooled": hits / tot if tot else None, "held_out_values": tot, "per_depth": per_d}
        rr["held_out_false_cut"] = fc
        mus, sds = [], []
        for d in depths:
            pv = planted_v(rows, n, d, k)
            m, s = mu_sd(list(pv.values()))
            mus.append(m)
            sds.append(s)
        params["rungs"].append({"n": n, "k": k, "depths": depths, "mu": mus, "sigma": sds})
        out["rungs"][n] = rr

    r8 = out["rungs"][8]
    auc_fire = all(
        dep.get("best_k") is None or dep["auc_gain_best_minus_v0"] < 0.05 or dep["auc_gain_ci95"][0] <= 0
        for dep in r8["depths"].values())
    r2_fire = all((dep["regression_best_k"]["r2"] or 0) < 0.2 for dep in r8["depths"].values())
    fc_fire = any((r8["held_out_false_cut"][p]["pooled"] or 0) > 2 * float(p) for p in ("0.05", "0.2"))
    out["stage1_kill_R8"] = {"auc_gain": auc_fire, "regression_r2": r2_fire, "false_cut": fc_fire, "fires": auc_fire or r2_fire or fc_fire}

    luby = {}
    for u in LUBY_UNITS:
        vals = [obs(r) for f in (f"primary-u{u}.jsonl", f"8-c10-u{u}.jsonl") for r in load(res("calibration", "luby", f))]
        luby[u] = {"km_median": km_quantile(vals, 0.5), "solved": sum(not math.isinf(v) for v in vals), "n": len(vals)}
    unit = min(LUBY_UNITS, key=lambda u: (luby[u]["km_median"], u))
    out["luby"] = {str(u): v for u, v in luby.items()}
    out["luby_unit"] = unit
    params["luby_unit"] = unit
    with open(res("calibration", "params.json"), "w") as f:
        json.dump(params, f, indent=1)
    with open(res("calibration", "stage1.json"), "w") as f:
        json.dump(out, f, indent=1, default=lambda x: None)
    write_calibration_md(out, params)


def f3(x):
    return "n/a" if x is None else f"{x:.3f}"


def write_calibration_md(out, params):
    L = ["# Calibration (seeds 1001..1010 only)", "",
         "Generated by `analysis/probcut.py calibrate` before any test-seed run.", "",
         f"Luby unit: {out['luby_unit']} nodes (pooled KM median over the 30 calibration instances of R6, R7, R8; cap 2e6).", "",
         "| unit | solved | KM median |", "|---|---|---|"]
    for u, v in out["luby"].items():
        L.append(f"| {u} | {v['solved']}/{v['n']} | {v['km_median']:,.0f} |" if not math.isinf(v["km_median"]) else f"| {u} | {v['solved']}/{v['n']} | > cap |")
    L += ["", "## ProbCut parameters (fitted on all 10 calibration seeds)", "",
          "| rung | k (probe depth k+1) | d | mu | sigma | t(0.2) | t(0.05) | t(0.01) |", "|---|---|---|---|---|---|---|---|"]
    for rp in params["rungs"]:
        for d, m, s in zip(rp["depths"], rp["mu"], rp["sigma"]):
            L.append(f"| {rp['n']}x{rp['n']} | {rp['k']} | {d} | {m:.3f} | {s:.3f} | " + " | ".join(f"{m - Z[p] * s:.3f}" for p in (0.2, 0.05, 0.01)) + " |")
    L += ["", "## Stage 1", "",
          "| rung | d | sibling sets | AUC v0 | AUC v1 | AUC v2 | AUC v3 | AUC v4 | best k | gain (95% CI) | shuffled ctrl | R2 best k | sigma | L8 capped |",
          "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|"]
    for n, rr in out["rungs"].items():
        for d, dep in rr["depths"].items():
            g = "n/a" if dep.get("best_k") is None else f"{dep['auc_gain_best_minus_v0']:+.3f} [{dep['auc_gain_ci95'][0]:+.3f}, {dep['auc_gain_ci95'][1]:+.3f}]"
            L.append(f"| {n}x{n} c={rr['c']} | {d} | {dep['sibling_sets']} | " + " | ".join(f3(a) for a in dep["auc"]) +
                     f" | {dep.get('best_k', 'n/a')} | {g} | {f3(dep.get('shuffled_control_auc_best_k'))} | {f3(dep['regression_best_k']['r2'])} | {f3(dep['regression_best_k']['sigma'])} | {dep['l8_capped']}/{dep['children']} |")
    L += ["", "| rung | pooled sets | pooled AUC v0..v4 | selected k | held-out false cut p=0.05 | p=0.2 |", "|---|---|---|---|---|---|"]
    for n, rr in out["rungs"].items():
        fc = rr["held_out_false_cut"]
        L.append(f"| {n}x{n} | {rr['pooled_sibling_sets']} | " + ", ".join(f3(a) for a in rr["pooled_auc"]) +
                 f" | {rr['selected_k']} | {f3(fc['0.05']['pooled'])} | {f3(fc['0.2']['pooled'])} |")
    k = out["stage1_kill_R8"]
    L += ["", f"Stage-1 kill on R8: AUC-gain clause {k['auc_gain']}, R2 clause {k['regression_r2']}, false-cut clause {k['false_cut']}; fires: {k['fires']}.", ""]
    with open(os.path.join(ROOT, "CALIBRATION.md"), "w") as f:
        f.write("\n".join(L))

CANDS = ["probcut-v1", "probcut-v2"]
ALL = ["scb-gacolor_ac3_random", "hp-dfs", "fc-dfs", "fc-luby"] + CANDS
FIRST_P = {"probcut-v1": 0.2, "probcut-v2": 0.05}


def test_rows(solver, n):
    if n < 8 and solver in ("scb-gacolor_ac3_random", "hp-dfs"):
        path = os.path.join(LADDER, "results", "test", "primary", f"{solver}.jsonl")
    else:
        path = res("test", "primary" if n < 8 else "8-c10", f"{solver}.jsonl")
    return sorted((r for r in load(path) if r["n"] == n), key=lambda r: r["seed"])


def g1(cand, base):
    bmap = {r["instance"]: r for r in base}
    lr = [log_ratio(c, bmap[c["instance"]]) for c in cand]
    assert all(c["cap"] == bmap[c["instance"]]["cap"] for c in cand)
    med = upper_median(lr)
    rng = random.Random(BOOT_SEED)
    boots = [upper_median([lr[rng.randrange(len(lr))] for _ in lr]) for _ in range(B)]
    lo, hi = pct(boots, 0.025), pct(boots, 0.975)
    p1 = sum(b >= 0 for b in boots) / B
    return {"pairs": len(lr), "median_ratio": None if math.isinf(med) else math.exp(med),
            "median_log_ratio": None if math.isinf(med) else med,
            "ci95": [None if math.isinf(lo) else lo, None if math.isinf(hi) else hi],
            "p_one_sided": p1, "label": label(med, hi)}


def label(med, hi):
    if hi < 0:
        return "PASS" if math.exp(med) <= 0.5 else "GAIN"
    return "NO-GAIN"


def stats(rows, passes=None):
    vals = [obs(r) for r in rows]
    k = sum(r["status"] == "solved" for r in rows)
    w = sorted(r["wall_ms"] for r in rows)
    st = {"solved": k, "n": len(rows), "wilson95": wilson(k, len(rows)),
          "km_q25": km_quantile(vals, 0.25), "km_median": km_quantile(vals, 0.5), "km_q75": km_quantile(vals, 0.75),
          "median_wall_ms": w[len(w) // 2], "total_wall_s": sum(w) / 1000,
          "statuses": {s: sum(r["status"] == s for r in rows) for s in sorted({r["status"] for r in rows})}}
    if passes is not None:
        st["first_pass_solved"] = sum(p["info"]["solved_in_pass"] == 1 for p in passes)
        st["passes_used_hist"] = {}
        for p in passes:
            key = str(p["info"]["passes_run"])
            st["passes_used_hist"][key] = st["passes_used_hist"].get(key, 0) + 1
        st["cuts_fired_total"] = sum(p["info"]["cuts_fired"] for p in passes)
        st["probes_total"] = sum(p["info"]["probes"] for p in passes)
    return st


def false_cut(n, p):
    """Fraction of test instances whose planted path fails at least one pass-1 cut test."""
    params = json.load(open(res("calibration", "params.json")))
    rp = next(r for r in params["rungs"] if r["n"] == n)
    rows = load(res("test", "planted-primary-67.jsonl") if n < 8 else res("test", "planted-8-c10.jsonl"))
    cut = {}
    for d, m, s in zip(rp["depths"], rp["mu"], rp["sigma"]):
        for seed, v in planted_v(rows, n, d, rp["k"]).items():
            cut[seed] = cut.get(seed, False) or v < m - Z[p] * s
    return sum(cut.values()) / len(cut), len(cut)


def cmd_report():
    out = {"rungs": {}, "cap": 1_000_000}
    for n, c, ladder in RUNGS:
        rows = {s: test_rows(s, n) for s in ALL}
        for s in ALL:
            assert [r["seed"] for r in rows[s]] == list(range(1, 31)), (s, n)
        ru = {"n": n, "c": c, "ladder": ladder, "solvers": {}}
        for s in ALL:
            passes = None
            if s in CANDS:
                pf = res("test", "primary" if n < 8 else "8-c10", f"{s}.passes.jsonl")
                passes = [p for p in load(pf) if p["n"] == n]
            ru["solvers"][s] = stats(rows[s], passes)
            if s in CANDS:
                fr, m = false_cut(n, FIRST_P[s])
                ru["solvers"][s]["false_cut_rate_pass1"] = fr
                ru["solvers"][s]["false_cut_instances"] = m
        ru["G1"] = {s: {b: g1(rows[s], rows[b]) for b in ["scb-gacolor_ac3_random", "fc-dfs", "fc-luby", "hp-dfs"]}
                    for s in CANDS + ["fc-dfs", "fc-luby"]}
        # Holm over the two looks vs the verdict baseline
        ps = sorted(CANDS, key=lambda s: ru["G1"][s]["scb-gacolor_ac3_random"]["p_one_sided"])
        holm, reject_prev = {}, True
        for i, s in enumerate(ps):
            g = ru["G1"][s]["scb-gacolor_ac3_random"]
            ok = reject_prev and g["p_one_sided"] < 0.025 / (2 - i)
            reject_prev = ok
            holm[s] = (("PASS" if g["median_ratio"] is not None and g["median_ratio"] <= 0.5 else "GAIN") if ok else "NO-GAIN")
        ru["holm_labels"] = holm
        out["rungs"][n] = ru
    R = out["rungs"]

    def better_base(n):
        return min(R[n]["solvers"][b]["km_median"] for b in ("fc-dfs", "fc-luby", "scb-gacolor_ac3_random"))

    def ratio(n):
        m = R[n]["solvers"]["probcut-v1"]["km_median"]
        bb = better_base(n)
        return INF if math.isinf(m) else (0.0 if math.isinf(bb) else m / bb)

    st2_cost = not (ratio(7) <= 0.7 and ratio(8) <= 0.7)
    st2_first = R[8]["solvers"]["probcut-v1"]["first_pass_solved"] <= 15
    nogain_all = all(all(R[n]["holm_labels"][s] == "NO-GAIN" for s in CANDS) for n in R)
    r8_nogain = all(R[8]["holm_labels"][s] == "NO-GAIN" for s in CANDS)
    r8_pass = any(R[8]["holm_labels"][s] == "PASS" for s in CANDS)
    s1 = json.load(open(res("calibration", "stage1.json")))["stage1_kill_R8"]
    own = s1["fires"] or st2_cost or st2_first
    killed = own or nogain_all or r8_nogain
    out["kill"] = {"stage1": s1, "stage2_cost": {"fires": st2_cost, "v1_over_better_baseline_km_median": {7: ratio(7), 8: ratio(8)}},
                   "stage2_first_pass": {"fires": st2_first, "v1_first_pass_solved_R8": R[8]["solvers"]["probcut-v1"]["first_pass_solved"]},
                   "nogain_all_rungs": nogain_all, "r8_nogain_both": r8_nogain, "r8_pass": r8_pass,
                   "outcome": "killed" if killed else ("survived" if r8_pass else "promising, needs a variant")}
    g2 = {}
    from analyze import cmd_compare
    import contextlib
    import io
    for s in CANDS:
        dst = res("test", f"g2-{s}-vs-scb.json")
        with contextlib.redirect_stdout(io.StringIO()):
            cmd_compare(res("test", "primary", f"{s}.jsonl"), os.path.join(LADDER, "results", "test", "primary", "scb-gacolor_ac3_random.jsonl"), dst)
        g2[s] = json.load(open(dst))["G2"]
    out["G2_R6_R7"] = g2
    with open(res("summary.json"), "w") as f:
        json.dump(out, f, indent=1, default=lambda x: None)
    write_results_md(out)


def km(v):
    return "> cap" if math.isinf(v) else f"{v:,.0f}"


def write_results_md(out):
    L = ["| rung | solver | solved (Wilson 95%) | KM q25 / median / q75 nodes | median wall ms | total wall s | first-pass solved | passes used | false-cut rate (pass 1) |",
         "|---|---|---|---|---|---|---|---|---|"]
    for n, ru in out["rungs"].items():
        for s, st in ru["solvers"].items():
            w = st["wilson95"]
            fp = f"{st['first_pass_solved']}/30" if "first_pass_solved" in st else ""
            pu = ", ".join(f"{k}:{v}" for k, v in sorted(st["passes_used_hist"].items())) if "passes_used_hist" in st else ""
            fc = f"{st['false_cut_rate_pass1']:.2f}" if "false_cut_rate_pass1" in st else ""
            L.append(f"| {n}x{n} c={ru['c']} | {s} | {st['solved']}/30 ({w[0]:.2f}-{w[1]:.2f}) | {km(st['km_q25'])} / {km(st['km_median'])} / {km(st['km_q75'])} | {st['median_wall_ms']:.1f} | {st['total_wall_s']:.1f} | {fp} | {pu} | {fc} |")
    L += ["", "| rung | candidate | baseline | median ratio | 95% CI log-ratio | one-sided p | label (unadj.) | Holm label |", "|---|---|---|---|---|---|---|---|"]
    for n, ru in out["rungs"].items():
        for s, gs in ru["G1"].items():
            for b, g in gs.items():
                ci = ", ".join("inf" if x is None else f"{x:+.2f}" for x in g["ci95"])
                mr = "inf" if g["median_ratio"] is None else f"{g['median_ratio']:.3f}"
                hl = ru["holm_labels"].get(s, "") if b == "scb-gacolor_ac3_random" else ""
                L.append(f"| {n}x{n} | {s} | {b} | {mr} | [{ci}] | {g['p_one_sided']:.4f} | {g['label']} | {hl} |")
    k = out["kill"]
    L += ["", "Kill evaluation: " + json.dumps(k, default=lambda x: None), ""]
    with open(res("RESULTS.md"), "w") as f:
        f.write("\n".join(L))


if __name__ == "__main__":
    cmd = sys.argv[1] if len(sys.argv) > 1 else ""
    if cmd == "calibrate":
        cmd_calibrate()
    elif cmd == "report":
        cmd_report()
    else:
        sys.exit(__doc__)
