#!/usr/bin/env python3
"""Render a grid run into a human-readable markdown report.

  results/<run>/results.jsonl + matrix.json  ->  results/<run>/report.md

The report groups variants by family, in the study's presentation order, and
shows mean/best/worst score, max depth reached, and median node throughput —
the statistics the study raises. Community reference rows are listed but not
scored (they run their own instance, not these variants).
"""
import argparse
import json
import statistics
from pathlib import Path

FAMILY_ORDER = ["baseline", "path", "heuristic", "break", "community"]


def median(xs):
    xs = [x for x in xs if x is not None]
    return round(statistics.median(xs)) if xs else None


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
    lines.append("# DFS study — grid report\n")
    lines.append(
        f"{meta['n_runs']} runs · {len(meta['variant_ids'])} corner-pinned variants · "
        f"{meta['budget_s']} s each · seed {meta['seed']} · single core.\n"
    )
    lines.append("Score = matched edges of 480; break variants score 480 − #breaks. "
                 "Throughput is search-nodes/s and is never compared across families.\n")

    spec_by_name = {s["name"]: s for s in matrix}
    for fam in FAMILY_ORDER:
        fam_specs = [s for s in matrix if s["family"] == fam]
        if not fam_specs:
            continue
        lines.append(f"\n## {fam}\n")
        lines.append("| variant | delta | breaks | mean | best | worst | max depth | median nps |")
        lines.append("|:--|:--|:--|--:|--:|--:|--:|--:|")
        for s in fam_specs:
            runs = by_algo.get(s["name"], [])
            if runs:
                scores = [r["score"] for r in runs if r.get("score") is not None]
                depths = [r["max_depth"] for r in runs if r.get("max_depth") is not None]
                mean = round(statistics.mean(scores), 1) if scores else "—"
                best = max(scores) if scores else "—"
                worst = min(scores) if scores else "—"
                depth = max(depths) if depths else "—"
                nps = median([r.get("nps") for r in runs])
                nps_str = f"{nps/1e6:.1f}M" if nps and nps >= 1e6 else (f"{round(nps/1e3)}K" if nps else "—")
            else:
                mean = best = worst = depth = nps_str = "cited"
            breaks = s["breaks"] if s["allows_breaks"] else "strict"
            lines.append(
                f"| {s['display']} | {s['delta']} | {breaks} | {mean} | {best} | {worst} | {depth} | {nps_str} |"
            )

    report = "\n".join(lines) + "\n"
    (run_dir / "report.md").write_text(report)
    print(f"wrote {run_dir / 'report.md'}")


if __name__ == "__main__":
    main()
