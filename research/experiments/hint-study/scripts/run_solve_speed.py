#!/usr/bin/env python3
"""Small-board solve-speed experiment for the hint study.

On a board small enough to fully solve (default 8×8), measure how many search
NODES a chronological backtracker needs to reach a complete solution, for
matched-count SCATTERED (spread lattice) vs CONTIGUOUS (top block) hints. This is
the metric the community hint-geometry claim is really about (time/nodes to a full
solve), which the 16×16-at-a-budget score cannot see. Nodes-to-solution is
machine-independent.

For each (seed, layout, path) we run hint_scale to completion (generous budget)
and record nodes + whether it solved. Reports the median nodes-to-solve per layout.

Usage:
  run_solve_speed.py --out results/solve8 --size 8 --seeds 30 --budget-s 30
"""
import argparse
import json
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

HERE = Path(__file__).resolve().parent.parent
EXP = HERE.parent
REPO = EXP.parent.parent
GEN = REPO / "engine/target/release/hint_variants"
SCALE = REPO / "engine/target/release/hint_scale"

# Scattered lattice vs matched contiguous block, at increasing matched counts, plus
# the no-hint baseline. This traces the solve-speed THRESHOLD (few hints hurt, many
# help) and the scattered-vs-contiguous question at each count. The paths are the
# site-engine names hint_scale accepts; row-major is the headline (compact) solver.
LAYOUTS = [
    "baseline_00",
    "ladder_spread_04", "ladder_contig_04",
    "ladder_spread_09", "ladder_contig_09",
    "ladder_spread_16", "ladder_contig_16",
    "ladder_spread_25", "ladder_contig_25",
    "ladder_spread_36", "ladder_contig_36",
]
PATHS = ["row-major"]


def field(line, key):
    for tok in line.split():
        if tok.startswith(key + "="):
            return tok.split("=", 1)[1]
    return None


def run_one(job, work, budget):
    seed, layout, path = job
    jpath = work / f"seed{seed}" / f"{layout}.json"
    if not jpath.exists():
        return None
    out = subprocess.run(
        [str(SCALE), "--puzzle", str(jpath), "--path", path, "--seed", "1", "--budget-s", str(budget)],
        capture_output=True, text=True,
    )
    line = next((l for l in out.stdout.splitlines() if l.startswith("RESULT")), None)
    if not line:
        return {"__error__": True, "seed": seed, "layout": layout, "path": path,
                "stderr": (out.stderr or "").strip()[:160]}
    solved = field(line, "solved") == "1"
    return {
        "seed": seed, "layout": layout, "path": path,
        "solved": solved,
        "nodes": int(field(line, "nodes") or 0),
        "score": int(field(line, "score") or 0),
        "elapsed_s": float(field(line, "elapsed_s") or 0.0),
        "budget": budget,
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", required=True)
    ap.add_argument("--size", type=int, default=8)
    ap.add_argument("--seeds", type=int, default=30)
    ap.add_argument("--budget-s", type=float, default=30.0)
    ap.add_argument("--parallel", type=int, default=6)
    args = ap.parse_args()

    for tool in (GEN, SCALE):
        if not tool.exists():
            sys.exit(f"missing build artifact: {tool}")

    outdir = Path(args.out)
    outdir.mkdir(parents=True, exist_ok=True)
    work = outdir / "boards"
    for seed in range(1, args.seeds + 1):
        subprocess.run([str(GEN), "--out", str(work / f"seed{seed}"),
                        "--size", str(args.size), "--seed", str(seed)],
                       capture_output=True, check=True)
    print(f"generated {args.seeds} boards at {args.size}×{args.size}", file=sys.stderr)

    jobs = [(s, lay, p) for s in range(1, args.seeds + 1) for lay in LAYOUTS for p in PATHS]
    results_path = outdir / "results.jsonl"
    errors = []
    with open(results_path, "w") as f, ThreadPoolExecutor(max_workers=args.parallel) as ex:
        for res in ex.map(lambda j: run_one(j, work, args.budget_s), jobs):
            if res is None:
                continue
            if res.get("__error__"):
                errors.append(res)
                continue
            f.write(json.dumps(res) + "\n")
            f.flush()
    print(f"wrote {results_path}", file=sys.stderr)
    if errors:
        print(f"ERROR: {len(errors)} runs failed; e.g. {errors[0]}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
