#!/usr/bin/env python3
"""Aggregate a repair grid run into the site JSON the repair-study page reads.

  results/<run>/results.jsonl + matrix.json  ->  web/src/data/repair-study.json

Per variant: mean/best/worst final score, mean lift over the starting board, the
stall axis (mean last-best-improved iteration vs mean total iterations), accept
rate, mean destroy size, median iters/sec, and the matrix descriptor (family,
parent, delta, and the five strategy tags). Plus one representative convergence
curve per variant (the run whose score is the median), so the site can draw the
"improvement arrives early, then the loop stalls" figure from measured data.
"""
import argparse
import json
import statistics
from pathlib import Path

HERE = Path(__file__).resolve().parent
EXPERIMENT = HERE.parent
# repo root: repair-study -> experiments -> research -> <root>
REPO_ROOT = EXPERIMENT.parent.parent.parent
DEFAULT_OUT = REPO_ROOT / "web" / "src" / "data" / "repair-study.json"


def mean1(xs, nd=1):
    xs = [x for x in xs if x is not None]
    return round(statistics.mean(xs), nd) if xs else None


def median(xs):
    xs = [x for x in xs if x is not None]
    return round(statistics.median(xs)) if xs else None


def read_curve(run_dir, curve_file):
    p = run_dir / "curves" / curve_file
    if not p.exists():
        return None
    txt = p.read_text().strip()
    if not txt:
        return None
    return [int(x) for x in txt.split(",") if x != ""]


def representative_curve(run_dir, runs):
    """The curve of the run whose final score is the median — a typical run, not
    the luckiest. Downsample to at most ~60 points so the JSON stays small."""
    scored = [r for r in runs if r.get("score") is not None and r.get("curve_file")]
    if not scored:
        return None
    scored.sort(key=lambda r: r["score"])
    mid = scored[len(scored) // 2]
    curve = read_curve(run_dir, mid["curve_file"])
    if not curve:
        return None
    if len(curve) > 60:
        step = len(curve) / 60.0
        curve = [curve[min(int(i * step), len(curve) - 1)] for i in range(60)]
    return curve


def agg(run_dir, rows):
    scores = [r["score"] for r in rows if r.get("score") is not None]
    return {
        "n": len(rows),
        "mean": mean1(scores),
        "best": max(scores) if scores else None,
        "worst": min(scores) if scores else None,
        "mean_lift": mean1([r.get("lift") for r in rows]),
        "start_score": median([r.get("start_score") for r in rows]),
        "mean_last_best_iter": median([r.get("last_best_iter") for r in rows]),
        "mean_iterations": median([r.get("iterations") for r in rows]),
        "accept_rate": mean1([r.get("accept_rate") for r in rows], 3),
        "mean_destroy": mean1([r.get("mean_destroy") for r in rows]),
        "median_ips": median([r.get("ips") for r in rows]),
        "ips_unit": rows[0].get("ips_unit", "repair-iters/s") if rows else "repair-iters/s",
        "restarts": median([r.get("restarts") for r in rows]),
        "curve": representative_curve(run_dir, rows),
        "best_url_file": max(
            rows, key=lambda r: r["score"] if r.get("score") is not None else -1
        ).get("url_file"),
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--run", required=True, help="results/<run> dir with results.jsonl")
    ap.add_argument("--out", default=str(DEFAULT_OUT))
    args = ap.parse_args()

    run_dir = Path(args.run)
    rows = [json.loads(ln) for ln in (run_dir / "results.jsonl").read_text().splitlines() if ln.strip()]
    matrix = json.loads((run_dir / "matrix.json").read_text())
    meta = json.loads((run_dir / "run_meta.json").read_text())

    by_algo = {}
    for r in rows:
        by_algo.setdefault(r["algo"], []).append(r)

    variants = []
    for spec in matrix:
        entry = {
            "name": spec["name"],
            "display": spec["display"],
            "family": spec["family"],
            "parent": spec["parent"],
            "delta": spec["delta"],
            "start": spec["start"],
            "destroy": spec["destroy"],
            "repair": spec["repair"],
            "accept": spec["accept"],
            "restart": spec["restart"],
        }
        if spec["name"] in by_algo:
            entry.update(agg(run_dir, by_algo[spec["name"]]))
        variants.append(entry)

    out = {
        "budget_s": meta["budget_s"],
        "seed": meta["seed"],
        "n_instances": len(meta["variant_ids"]),
        "max_score": 480,
        "curve_stride": 200,
        "variants": variants,
    }
    Path(args.out).write_text(json.dumps(out, indent=1) + "\n")
    print(f"wrote {len(variants)} variants -> {args.out}")


if __name__ == "__main__":
    main()
