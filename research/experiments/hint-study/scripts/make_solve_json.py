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
    ap.add_argument("--size", type=int, default=8,
                    help="board edge length; a contiguous row pins this many cells")
    args = ap.parse_args()
    out_size = args.size
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

    # --- Contiguous-ROW crossover ------------------------------------------
    # The scattered REFERENCE the hint-geometry page discusses: an ~18-hint
    # scattered lattice that solves easily. On this fully-solvable board its
    # closest analog is the densest fast-solving spread lattice — ladder_spread_16
    # (a 4-per-line, 16-hint lattice). We report its solve-rate and median nodes,
    # then walk the contiguous-row family (R full rows = R·8 hints) and find the
    # fewest rows whose solve-rate AND median-nodes both reach the reference. That
    # row count is the measured crossover.
    N = out_size
    ref_layout = "ladder_spread_16"
    ref = stat(ref_layout)
    row_families = sorted(
        int(k.split("_")[-1]) for k in by if k.startswith("row_block_")
    )
    rows_axis = []
    for r in row_families:
        st_r = stat(f"row_block_{r:02}")
        if st_r:
            rows_axis.append({"rows": r, "hints": r * N, **st_r})

    def solve_rate(s):
        return (s["solved"] / s["of"]) if s and s["of"] else 0.0

    crossover = None
    if ref:
        ref_rate = solve_rate(ref)
        ref_nodes = ref["median_nodes"]
        for row in rows_axis:
            rate_ok = solve_rate(row) >= ref_rate
            # median nodes: matched when the row family solves in no more nodes
            # than the reference (lower = easier). Only meaningful once it solves.
            nodes_ok = (
                row["median_nodes"] is not None
                and ref_nodes is not None
                and row["median_nodes"] <= ref_nodes
            )
            if rate_ok and nodes_ok:
                crossover = row["rows"]
                break

    crossover_block = {
        "reference_layout": ref_layout,
        "reference_desc": "16-hint scattered lattice (4 per line)",
        "reference": ref,
        "reference_solve_rate": round(solve_rate(ref), 3) if ref else None,
        "rows": rows_axis,
        "crossover_rows": crossover,
        "crossover_hints": (crossover * N) if crossover else None,
    }

    out = {
        "size": out_size,
        "baseline": baseline,
        "ladder": ladder,
        "row_crossover": crossover_block,
        "n_seeds": max((s["of"] for s in [baseline] if s), default=0),
    }
    json.dump(out, open(args.out, "w"), indent=2)
    print(f"wrote {args.out}: baseline solved {baseline['solved']}/{baseline['of'] if baseline else 0}, "
          f"{len(ladder)} counts, {len(rows_axis)} row families, "
          f"crossover = {crossover} contiguous rows")


if __name__ == "__main__":
    main()
