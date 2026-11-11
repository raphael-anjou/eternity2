#!/usr/bin/env python3
"""Summarise results/results.jsonl into results/summary.json (PREREG.md analysis plan).

Standard library only. Deterministic: bootstrap uses random.Random(20261111),
a fresh generator per regime, 2000 replicates; the same replicate (resampled
instances per size) feeds every statistic of that regime.
"""
import json
import math
import random
import statistics
import sys
from collections import defaultdict

REPS = 2000
SEED = 20261111
B_THRESHOLD = 0.9


def ols_slope(xs, ys):
    mx, my = sum(xs) / len(xs), sum(ys) / len(ys)
    sxx = sum((x - mx) ** 2 for x in xs)
    sxy = sum((x - mx) * (y - my) for x, y in zip(xs, ys))
    b = sxy / sxx
    return b, my - b * mx


def loglog_exponent(sizes, means):
    if any(m <= 0 for m in means):
        return None
    return ols_slope([math.log(n * n) for n in sizes], [math.log(m) for m in means])[0]


def pct(sorted_vals, q):
    return sorted_vals[int(round(q * (len(sorted_vals) - 1)))]


def ci(vals):
    vals = sorted(v for v in vals if v is not None)
    if not vals:
        return None
    return [pct(vals, 0.025), pct(vals, 0.975)]


def r6(x):
    return None if x is None else round(x, 6)


def main(path_in, path_out):
    rows = [json.loads(line) for line in open(path_in)]
    by = defaultdict(list)
    for r in rows:
        by[(r["regime"], r["n"])].append(r)
    for v in by.values():
        v.sort(key=lambda r: r["seed"])
    summary = {"instances": len(rows), "bootstrap": {"reps": REPS, "seed": SEED},
               "per_size": [], "fits": {}, "verdict": {}}
    regimes = sorted({r["regime"] for r in rows})
    for g in regimes:
        sizes = sorted(n for (rg, n) in by if rg == g)
        for n in sizes:
            rs = by[(g, n)]
            N = n * n
            ks = [r["k"] for r in rs]
            t50 = [r["t50"] for r in rs]
            cert = [r for r in rs if r["exact_certified"]]
            gaps = [r["upper_bound"] - r["exact_value"] for r in cert]
            summary["per_size"].append({
                "regime": g, "n": n, "cells": N, "instances": len(rs),
                "border_colors": rs[0]["border_colors"], "interior_colors": rs[0]["interior_colors"],
                "colors_returned": sorted({r["colors_returned"] for r in rs}),
                "root_complete": sum(r["root_complete"] for r in rs),
                "root_singletons_mean": r6(statistics.mean(r["root_singletons"] for r in rs)),
                "k_mean": r6(statistics.mean(ks)), "k_median": statistics.median(ks),
                "k_sd": r6(statistics.stdev(ks)), "k_min": min(ks), "k_max": max(ks),
                "k_over_N_mean": r6(statistics.mean(ks) / N),
                "k_source_exact": sum(r["k_source"] == "exact" for r in rs),
                "ub_mean": r6(statistics.mean(r["upper_bound"] for r in rs)),
                "forward_greedy_mean": r6(statistics.mean(r["forward_greedy"] for r in rs)),
                "best_restart_mean": r6(statistics.mean(r["best_restart"] for r in rs)),
                "calibration": {
                    "certified": len(cert),
                    "ub_equals_exact": sum(1 for x in gaps if x == 0),
                    "gap_counts": {str(x): gaps.count(x) for x in sorted(set(gaps))},
                    "gap_mean": r6(statistics.mean(gaps)) if gaps else None,
                },
                "lower_bound_when_capped": [r["exact_value"] for r in rs if not r["exact_certified"]],
                "t50_mean": r6(statistics.mean(t50)), "t50_median": statistics.median(t50),
                "t50_over_N_mean": r6(statistics.mean(t50) / N),
                "mean_t_mean": r6(statistics.mean(r["mean_t"] for r in rs)),
                "k_over_t50": r6(statistics.mean(ks) / statistics.mean(t50)),
                "p_rand_at_k_mean": r6(statistics.mean(r["p_rand_at_k"] for r in rs)),
                "p_rand_at_k_max": max(r["p_rand_at_k"] for r in rs),
                "all_completed_valid": all(r["completed_valid"] for r in rs),
                "all_completed_planted": all(r["completed_is_planted"] for r in rs),
            })
        kbar = [statistics.mean(r["k"] for r in by[(g, n)]) for n in sizes]
        tbar = [statistics.mean(r["t50"] for r in by[(g, n)]) for n in sizes]
        lo, hi = sizes[0], sizes[-1]

        def frac_change(means):
            return means[-1] / (hi * hi) - means[0] / (lo * lo)

        rng = random.Random(SEED)
        boot = {"b_k": [], "b_t50": [], "D_k": [], "D_t50": []}
        for _ in range(REPS):
            kb, tb = [], []
            for n in sizes:
                rs = by[(g, n)]
                sample = [rs[rng.randrange(len(rs))] for _ in rs]
                kb.append(statistics.mean(r["k"] for r in sample))
                tb.append(statistics.mean(r["t50"] for r in sample))
            boot["b_k"].append(loglog_exponent(sizes, kb))
            boot["b_t50"].append(loglog_exponent(sizes, tb))
            boot["D_k"].append(frac_change(kb))
            boot["D_t50"].append(frac_change(tb))
        fit = {
            "sizes": sizes,
            "kbar": [r6(x) for x in kbar], "t50bar": [r6(x) for x in tbar],
            "b_k": r6(loglog_exponent(sizes, kbar)), "b_k_ci95": [r6(x) for x in ci(boot["b_k"]) or []],
            "b_k_boot_undefined": sum(v is None for v in boot["b_k"]),
            "D_k": r6(frac_change(kbar)), "D_k_ci95": [r6(x) for x in ci(boot["D_k"])],
            "b_t50": r6(loglog_exponent(sizes, tbar)), "b_t50_ci95": [r6(x) for x in ci(boot["b_t50"]) or []],
            "D_t50": r6(frac_change(tbar)), "D_t50_ci95": [r6(x) for x in ci(boot["D_t50"])],
        }
        summary["fits"][g] = fit
        if fit["b_k"] is None:
            verdict = "inconclusive"
            reason = "kbar(n) = 0 at some size; power-law fit undefined"
        else:
            c1 = fit["b_k_ci95"][1] >= B_THRESHOLD
            c2 = not (fit["D_k_ci95"][1] < 0)
            verdict = "kill" if (c1 or c2) else "survive"
            reason = (f"b_k CI upper {fit['b_k_ci95'][1]:.3f} {'>=' if c1 else '<'} {B_THRESHOLD}; "
                      f"D_k CI {fit['D_k_ci95']} {'not entirely below 0' if c2 else 'entirely below 0'}")
        summary["verdict"][g] = {"outcome": verdict, "reason": reason,
                                 "decides_preregistered_outcome": g == "A"}

    # Post hoc (not pre-registered): how far the regime-A exponent could move if
    # the capped instances' true minimum sat at its certified lower bound.
    sens = {}
    for g in regimes:
        sizes = sorted(n for (rg, n) in by if rg == g)
        variants = {}
        for label, lb_sizes in (("capped_n7_at_lower_bound", {7}),
                                ("all_capped_at_lower_bound", set(sizes))):
            means = []
            for n in sizes:
                ks = [r["exact_value"] if (not r["exact_certified"] and n in lb_sizes) else r["k"]
                      for r in by[(g, n)]]
                means.append(statistics.mean(ks))
            variants[label] = {"kbar": [r6(x) for x in means], "b_k": r6(loglog_exponent(sizes, means))}
        sens[g] = variants
    summary["post_hoc_sensitivity"] = sens
    with open(path_out, "w") as f:
        json.dump(summary, f, indent=2, sort_keys=True)
        f.write("\n")
    print(json.dumps(summary["fits"], indent=1))
    print(json.dumps(summary["verdict"], indent=1))
    print(json.dumps(summary["post_hoc_sensitivity"], indent=1))


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "results/results.jsonl",
         sys.argv[2] if len(sys.argv) > 2 else "results/summary.json")
