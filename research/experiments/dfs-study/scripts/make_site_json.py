#!/usr/bin/env python3
"""Aggregate a grid run into the site JSON the DFS-study page reads.

  results/<run>/results.jsonl + matrix.json  ->  web/src/data/dfs-study.json

Per variant we report mean/best/worst score over the ten instances, plus the
study's raised stats (median nps, max depth reached, breaks) and the matrix
descriptor (family, parent, delta, path/value/propagate/breaks, allows_breaks).
The page renders the leaderboard and the "what stacks on what" matrix straight
from this, so it can never drift from what was measured.
"""
import argparse
import json
import statistics
from pathlib import Path

HERE = Path(__file__).resolve().parent
EXPERIMENT = HERE.parent
# repo root: dfs-study -> experiments -> research -> <root>
REPO_ROOT = EXPERIMENT.parent.parent.parent
DEFAULT_OUT = REPO_ROOT / "web" / "src" / "data" / "dfs-study.json"


def median(xs):
    xs = [x for x in xs if x is not None]
    return round(statistics.median(xs)) if xs else None


def agg(rows):
    scores = [r["score"] for r in rows if r.get("score") is not None]
    depths = [r["max_depth"] for r in rows if r.get("max_depth") is not None]
    return {
        "n": len(rows),
        "mean": round(statistics.mean(scores), 1) if scores else None,
        "best": max(scores) if scores else None,
        "worst": min(scores) if scores else None,
        # Every instance's raw score, in run order, so the site can draw the
        # per-variant spread (each instance a dot) beneath the mean bar. The mean
        # above is exactly the mean of this list, so the two never disagree.
        "scores": scores,
        "median_nps": median([r.get("nps") for r in rows]),
        "nps_unit": rows[0].get("nps_unit", "search-nodes/s") if rows else "search-nodes/s",
        "max_depth": max(depths) if depths else None,
        "median_breaks": median([r.get("breaks") for r in rows]),
        # The best-scoring run's canonical board .json (which itself carries an
        # eternity2.dev viewer URL), so the page can link to it. Guard the
        # falsy-zero trap: a real score of 0 must not sort as -1 (missing).
        "best_board_file": max(
            rows, key=lambda r: r["score"] if r.get("score") is not None else -1
        ).get("board_file"),
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
            "kind": spec["kind"],
            "parent": spec["parent"],
            "delta": spec["delta"],
            "path": spec["path"],
            "value": spec["value"],
            "propagate": spec["propagate"],
            "breaks": spec["breaks"],
            "allows_breaks": spec["allows_breaks"],
            "note": spec.get("note", ""),
        }
        if spec["name"] in by_algo:
            entry.update(agg(by_algo[spec["name"]]))
        else:
            # External community rows: cited, not measured on these variants.
            entry.update({"n": 0, "mean": None, "best": None})
        variants.append(entry)

    # Community reference engines, measured on their NATIVE instances (they can
    # not take the corner-pinned variants). Loaded from the committed
    # community.json so the numbers are structured, not scraped from prose.
    community_path = EXPERIMENT / "results" / "community.json"
    community = []
    if community_path.exists():
        community = json.loads(community_path.read_text()).get("engines", [])

    out = {
        "budget_s": meta["budget_s"],
        "seed": meta["seed"],
        "n_instances": len(meta["variant_ids"]),
        "max_score": 480,
        "community_5clue_record": 464,
        "variants": variants,
        "community": community,
    }
    Path(args.out).write_text(json.dumps(out, indent=1) + "\n")
    print(f"wrote {len(variants)} variants + {len(community)} community -> {args.out}")


if __name__ == "__main__":
    main()
