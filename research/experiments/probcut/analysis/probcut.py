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
from analyze import km_quantile, log_ratio, obs, pct, upper_median, wilson  # noqa: E402

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


def cmd_report():
    raise SystemExit("report: not yet")


if __name__ == "__main__":
    cmd = sys.argv[1] if len(sys.argv) > 1 else ""
    if cmd == "calibrate":
        cmd_calibrate()
    elif cmd == "report":
        cmd_report()
    else:
        sys.exit(__doc__)
