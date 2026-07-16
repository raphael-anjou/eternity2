#!/usr/bin/env python3
"""Render a repair-study grid run into a human-readable markdown report.

  results/<run>/results.jsonl + matrix.json  ->  results/<run>/report.md

Grouped by family, in the study's presentation order. For each variant: mean
final score and the mean lift over its starting board, plus the two axes that
carry the repair story — the mean iteration at which the best last improved (the
stall) against the total iterations, and the accept rate.
"""
import argparse
import json
import statistics
from pathlib import Path

FAMILY_ORDER = ["start", "destroy", "repair", "accept", "restart"]


def mean1(xs):
    xs = [x for x in xs if x is not None]
    return round(statistics.mean(xs), 1) if xs else None


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--run", required=True)
    args = ap.parse_args()
    run_dir = Path(args.run)

    rows = [json.loads(ln) for ln in (run_dir / "results.jsonl").read_text().splitlines() if ln.strip()]
    matrix = json.loads((run_dir / "matrix.json").read_text())
    meta = json.loads((run_dir / "run_meta.json").read_text())

    by_algo = {}
    for r in rows:
        by_algo.setdefault(r["algo"], []).append(r)

    lines = []
    lines.append("# Repair study — grid report\n")
    lines.append(
        f"{meta['n_runs']} runs · {len(meta['variant_ids'])} corner-pinned variants · "
        f"{meta['budget_s']} s each · seed {meta['seed']} · single core.\n"
    )
    lines.append(
        "Score = matched edges of 480 (canonical rescore). Lift = final − starting "
        "board. `last best` is the mean iteration the global best last improved; "
        "compare it to `iters` to see how early the run stalled. Iters/sec is never "
        "compared across families.\n"
    )

    for fam in FAMILY_ORDER:
        fam_specs = [s for s in matrix if s["family"] == fam]
        if not fam_specs:
            continue
        lines.append(f"\n## {fam}\n")
        lines.append("| variant | delta | mean score | mean lift | best | last best | iters | accept |")
        lines.append("|:--|:--|--:|--:|--:|--:|--:|--:|")
        for s in fam_specs:
            runs = by_algo.get(s["name"], [])
            scores = [r["score"] for r in runs if r.get("score") is not None]
            mean = mean1(scores)
            best = max(scores) if scores else "—"
            lift = mean1([r.get("lift") for r in runs])
            last_best = mean1([r.get("last_best_iter") for r in runs])
            iters = mean1([r.get("iterations") for r in runs])
            acc = mean1([r.get("accept_rate") for r in runs])
            iters_str = f"{iters/1e3:.0f}K" if iters and iters >= 1e3 else (str(iters) if iters else "—")
            lb_str = f"{last_best/1e3:.1f}K" if last_best and last_best >= 1e3 else (str(round(last_best)) if last_best else "—")
            lines.append(
                f"| {s['display']} | {s['delta']} | {mean} | {lift} | {best} | {lb_str} | {iters_str} | {acc} |"
            )

    report = "\n".join(lines) + "\n"
    (run_dir / "report.md").write_text(report)
    print(f"wrote {run_dir / 'report.md'}")


if __name__ == "__main__":
    main()
