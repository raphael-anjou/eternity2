#!/usr/bin/env python3
"""Pre-registered analysis for A4 (see PREREG.md). Standard library only.

Reads basins.jsonl (one row per run), writes summary.json and prints the
README tables.
"""
import argparse
import json
import random
import statistics
from collections import defaultdict

PRIMARY = "k2_d0.25"
KS = [0, 1, 2, 4]
DELTAS = ["0.10", "0.25", "0.50"]
ALPHA = 0.05
QUALITY_GUARD = -2.0


def wilcoxon_exact(diffs, alternative="greater"):
    """Exact Wilcoxon signed-rank p-value; zeros dropped, mid-ranks for ties."""
    d = [x for x in diffs if x != 0]
    n = len(d)
    if n == 0:
        return {"n_nonzero": 0, "w_plus": 0.0, "p": 1.0}
    order = sorted(range(n), key=lambda i: abs(d[i]))
    ranks2 = [0] * n  # doubled ranks (integers)
    i = 0
    while i < n:
        j = i
        while j + 1 < n and abs(d[order[j + 1]]) == abs(d[order[i]]):
            j += 1
        r2 = (i + 1) + (j + 1)  # 2 * mean rank
        for t in range(i, j + 1):
            ranks2[order[t]] = r2
        i = j + 1
    w2 = sum(r for r, x in zip(ranks2, d) if x > 0)
    total = sum(ranks2)
    dist = defaultdict(int)
    dist[0] = 1
    for r in ranks2:
        nxt = defaultdict(int)
        for s, c in dist.items():
            nxt[s] += c
            nxt[s + r] += c
        dist = nxt
    denom = 2 ** n
    upper = sum(c for s, c in dist.items() if s >= w2) / denom
    lower = sum(c for s, c in dist.items() if s <= w2) / denom
    if alternative == "greater":
        p = upper
    elif alternative == "less":
        p = lower
    else:
        p = min(1.0, 2 * min(upper, lower))
    return {"n_nonzero": n, "w_plus": w2 / 2, "w_total": total / 2, "p": p}


def bootstrap_mean_ci(xs, reps=10_000, seed=12345):
    rng = random.Random(seed)
    n = len(xs)
    means = sorted(sum(rng.choice(xs) for _ in range(n)) / n for _ in range(reps))
    return [means[int(0.025 * reps)], means[int(0.975 * reps) - 1]]


def paired(per_puzzle, fn, alternative):
    diffs = [fn(v["me"]) - fn(v["pt"]) for v in per_puzzle.values()]
    out = {"n_puzzles": len(diffs), "mean_diff": statistics.mean(diffs),
           "median_diff": statistics.median(diffs),
           "mean_pt": statistics.mean(fn(v["pt"]) for v in per_puzzle.values()),
           "mean_me": statistics.mean(fn(v["me"]) for v in per_puzzle.values()),
           "n_me_higher": sum(1 for x in diffs if x > 0),
           "n_pt_higher": sum(1 for x in diffs if x < 0),
           "n_tied": sum(1 for x in diffs if x == 0)}
    out["bootstrap95_mean_diff"] = bootstrap_mean_ci(diffs)
    out["wilcoxon"] = wilcoxon_exact(diffs, alternative)
    out["wilcoxon"]["alternative"] = alternative
    return out


def mean_of(rows, f):
    return statistics.mean(f(r) for r in rows)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--basins", required=True)
    ap.add_argument("--out", required=True)
    args = ap.parse_args()
    rows = [json.loads(line) for line in open(args.basins) if line.strip()]
    by_size = defaultdict(lambda: defaultdict(lambda: {"pt": [], "me": []}))
    for r in rows:
        by_size[r["size"]][r["puzzle_seed"]][r["method"]].append(r)

    summary = {"primary_statistic": PRIMARY, "alpha_family": ALPHA, "sizes": {}}
    for size in sorted(by_size):
        pp = by_size[size]
        allr = [r for v in pp.values() for m in ("pt", "me") for r in v[m]]
        s = {"colors": allr[0]["colors"], "total_edges": allr[0]["total_edges"],
             "evals_budget": allr[0]["evals"], "n_runs": len(allr)}
        s["primary"] = paired(pp, lambda rs: mean_of(rs, lambda r: r["basins"][PRIMARY]), "greater")
        s["per_core_hour"] = paired(
            pp, lambda rs: mean_of(rs, lambda r: r["basins"][PRIMARY] / (r["cpu_s"] / 3600)), "greater")
        s["best_score"] = paired(pp, lambda rs: mean_of(rs, lambda r: r["best_score"]), "two-sided")
        s["near_best_boards_k2"] = paired(pp, lambda rs: mean_of(rs, lambda r: r["basins"]["k2_boards"]), "two-sided")
        s["sensitivity"] = {}
        for k in KS:
            for d in DELTAS:
                key = f"k{k}_d{d}"
                res = paired(pp, lambda rs, key=key: mean_of(rs, lambda r: r["basins"][key]), "greater")
                s["sensitivity"][key] = {x: res[x] for x in ("mean_pt", "mean_me", "mean_diff", "median_diff",
                                                             "n_me_higher", "n_pt_higher")}
                s["sensitivity"][key]["p_greater"] = res["wilcoxon"]["p"]
        per_method = {}
        for m in ("pt", "me"):
            mr = [r for v in pp.values() for r in v[m]]
            per_method[m] = {
                "runs": len(mr),
                "solved_runs": sum(1 for r in mr if r["best_score"] == r["total_edges"]),
                "puzzles_solved_by_any_seed": sum(1 for v in pp.values()
                                                  if any(r["best_score"] == r["total_edges"] for r in v[m])),
                "mean_best_score": statistics.mean(r["best_score"] for r in mr),
                "evals_per_run": sorted({r["evals"] for r in mr}),
                "mean_wall_s": statistics.mean(r["wall_s"] for r in mr),
                "mean_cpu_s": statistics.mean(r["cpu_s"] for r in mr),
                "total_cpu_s": sum(r["cpu_s"] for r in mr),
                "mean_candidates": statistics.mean(r["n_candidates"] for r in mr),
                "mean_archive_filled": (statistics.mean(r["archive_filled"] for r in mr)
                                        if m == "me" else None),
            }
        s["per_method"] = per_method
        s["cpu_ratio_me_over_pt"] = per_method["me"]["mean_cpu_s"] / per_method["pt"]["mean_cpu_s"]
        s["puzzles_best_known_is_planted"] = sum(
            1 for v in pp.values() if v["pt"][0]["best_known"] == v["pt"][0]["total_edges"])
        summary["sizes"][str(size)] = s

    # Holm over the two sizes on the primary one-sided p-values.
    ps = sorted((s["primary"]["wilcoxon"]["p"], size) for size, s in summary["sizes"].items())
    m = len(ps)
    still = True
    for i, (p, size) in enumerate(ps):
        thr = ALPHA / (m - i)
        rej = still and p <= thr
        if not rej:
            still = False
        summary["sizes"][size]["primary"]["holm_threshold"] = thr
        summary["sizes"][size]["primary"]["holm_reject"] = rej
    wins = [size for size, s in summary["sizes"].items() if s["primary"]["holm_reject"]]
    guarded = [size for size in wins
               if summary["sizes"][size]["best_score"]["median_diff"] < QUALITY_GUARD]
    summary["me_significant_on_sizes"] = wins
    summary["me_wins_on_both_sizes"] = len(wins) == len(summary["sizes"])
    summary["quality_guard_triggered_on"] = guarded
    if not wins:
        outcome = "killed"
    elif len(guarded) == len(wins):
        outcome = "inconclusive"
    else:
        outcome = "survived"
    summary["outcome"] = outcome
    with open(args.out, "w") as f:
        json.dump(summary, f, indent=2, sort_keys=True)
        f.write("\n")

    print(f"outcome: {outcome}; significant on {wins or 'none'}; both: {summary['me_wins_on_both_sizes']}")
    print("\n| size | colours | budget (evals) | PT basins | ME basins | mean diff [95% CI] | ME>PT / PT>ME | p (one-sided) | Holm |")
    print("|---|---|---|---|---|---|---|---|---|")
    for size, s in summary["sizes"].items():
        p = s["primary"]
        ci = p["bootstrap95_mean_diff"]
        print(f"| {size}x{size} | {s['colors']} | {s['evals_budget']:,} | {p['mean_pt']:.2f} | {p['mean_me']:.2f} | "
              f"{p['mean_diff']:+.2f} [{ci[0]:+.2f}, {ci[1]:+.2f}] | {p['n_me_higher']} / {p['n_pt_higher']} | "
              f"{p['wilcoxon']['p']:.3g} | {'reject' if p['holm_reject'] else 'keep'} |")
    print("\n| size | PT best | ME best | median diff | p (two-sided) | PT solved runs | ME solved runs | PT cpu s/run | ME cpu s/run | ME archive cells |")
    print("|---|---|---|---|---|---|---|---|---|---|")
    for size, s in summary["sizes"].items():
        b = s["best_score"]
        pm = s["per_method"]
        print(f"| {size}x{size} | {b['mean_pt']:.2f} | {b['mean_me']:.2f} | {b['median_diff']:+.2f} | "
              f"{b['wilcoxon']['p']:.3g} | {pm['pt']['solved_runs']}/{pm['pt']['runs']} | "
              f"{pm['me']['solved_runs']}/{pm['me']['runs']} | {pm['pt']['mean_cpu_s']:.1f} | "
              f"{pm['me']['mean_cpu_s']:.1f} | {pm['me']['mean_archive_filled']:.1f} |")
    for size, s in summary["sizes"].items():
        print(f"\nSensitivity {size}x{size} (mean PT / mean ME, one-sided p):\n")
        print("| k | " + " | ".join(f"delta {d}" for d in DELTAS) + " |")
        print("|---|" + "---|" * len(DELTAS))
        for k in KS:
            cells = []
            for d in DELTAS:
                x = s["sensitivity"][f"k{k}_d{d}"]
                cells.append(f"{x['mean_pt']:.2f} / {x['mean_me']:.2f} (p={x['p_greater']:.3g})")
            print(f"| {k} | " + " | ".join(cells) + " |")


if __name__ == "__main__":
    main()
