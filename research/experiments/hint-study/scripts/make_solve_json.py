#!/usr/bin/env python3
"""Turn the 8×8 solve-speed results into a small JSON block for the site.

Emits web/src/data/hint-study-solve.json with, per matched hint count, the
median nodes-to-solution and the solved rate for the spread lattice vs the
contiguous block (and the no-hint baseline). This is the fully-solvable
small-board view the 16×16-at-a-budget study cannot give.
"""
import argparse
import json
import statistics as st
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent.parent
REPO = HERE.parent.parent.parent
OUT = REPO / "web/src/data/hint-study-solve.json"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--run", required=True)
    ap.add_argument("--out", default=str(OUT))
    args = ap.parse_args()
    rp = Path(args.run)
    rp = rp / "results.jsonl" if rp.is_dir() else rp
    rows = [json.loads(l) for l in open(rp) if l.strip()]

    # group by layout (row-major only, the headline solver)
    by = defaultdict(list)
    for r in rows:
        if r["path"] == "row-major":
            by[r["layout"]].append(r)

    def stat(layout):
        rs = by.get(layout, [])
        if not rs:
            return None
        solved = [r for r in rs if r["solved"]]
        nodes = sorted(r["nodes"] for r in solved)
        return {
            "of": len(rs),
            "solved": len(solved),
            "median_nodes": int(st.median(nodes)) if nodes else None,
            "min_nodes": nodes[0] if nodes else None,
            "max_nodes": nodes[-1] if nodes else None,
        }

    baseline = stat("baseline_00")
    # count sweep: spread vs contiguous at each matched count
    counts = sorted({int(k.split("_")[-1]) for k in by if k.startswith("ladder_spread_")})
    ladder = []
    for c in counts:
        s = stat(f"ladder_spread_{c:02}")
        k = stat(f"ladder_contig_{c:02}")
        if s or k:
            ladder.append({"count": c, "spread": s, "contiguous": k})

    out = {
        "size": 8,
        "baseline": baseline,
        "ladder": ladder,
        "n_seeds": max((s["of"] for s in [baseline] if s), default=0),
    }
    json.dump(out, open(args.out, "w"), indent=2)
    print(f"wrote {args.out}: baseline solved {baseline['solved']}/{baseline['of'] if baseline else 0}, "
          f"{len(ladder)} counts")


if __name__ == "__main__":
    main()
