#!/usr/bin/env python3
"""Aggregate results/ladder.jsonl into results/summary.json.

Deterministic byte-stable output: sorted keys, no timestamps, no wall times.
Per (solver, N): best/min/median/max gap over seeds, best raw score, the
ceiling, full-solve count, and one representative board URL (the best seed's).

Usage: python3 summarize.py ../results/ladder.jsonl ../results/summary.json
"""

import json
import statistics
import sys


def main() -> None:
    src, dst = sys.argv[1], sys.argv[2]
    rows = [json.loads(line) for line in open(src) if line.strip()]

    cells = {}
    for r in rows:
        cells.setdefault((r["solver"], r["n"]), []).append(r)

    out = []
    for (solver, n), rs in sorted(cells.items()):
        rs.sort(key=lambda r: (-r["score"], r["seed"]))
        gaps = sorted(r["gap"] for r in rs)
        out.append(
            {
                "solver": solver,
                "n": n,
                "ceiling": rs[0]["ceiling"],
                "seeds": len(rs),
                "best_score": rs[0]["score"],
                "gap_best": max(gaps),
                "gap_min": min(gaps),
                "gap_median": round(statistics.median(gaps), 3),
                "gap_max": max(gaps),
                "full_solves": sum(1 for r in rs if r["full_solve"]),
                "best_url": rs[0]["url"],
            }
        )

    with open(dst, "w") as f:
        json.dump(out, f, indent=1, sort_keys=True)
        f.write("\n")


if __name__ == "__main__":
    main()
