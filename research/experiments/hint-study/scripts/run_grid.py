#!/usr/bin/env python3
"""Hint-study grid runner.

For each board seed, generate the hint-geometry variants, then run every fill
path (a strict DFS backtracker) on every layout, plus the beam solver once per
layout, single core per run, and record one JSON line per (seed, layout, solver).

The scientific single-core constraint holds PER RUN — each solver process is
pinned to one thread. Independent (seed, layout, path) cells are run concurrently
across `--parallel` workers, since they do not interact.

Usage:
  run_grid.py --out results/rerun --budget-s 8 --seeds 15 --parallel 6 [--size 16]
"""
import argparse
import json
import os
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

HERE = Path(__file__).resolve().parent.parent          # research/experiments/hint-study
EXP = HERE.parent                                       # research/experiments
REPO = EXP.parent.parent                                # repo root

GEN = REPO / "engine/target/release/hint_variants"
DFS = EXP / "dfs-study/engine/target/release/run_dfs"
PROD = EXP / "single-core-benchmark/engine/target/release/producer_trie"

# The study's axes.
PATHS = [
    "rowmajor", "rowmajor-bottomup", "spiral-in", "spiral-out",
    "border-first", "verhaard-comb", "clue-rows-first", "connect-hints-first",
]
LAYOUTS = [
    "geom_scattered", "geom_contiguous", "geom_interior", "geom_border",
    "ladder_spread_04", "ladder_spread_09", "ladder_spread_16", "ladder_spread_25",
    "ladder_clustered_k2_20", "ladder_clustered_k3_45", "ladder_clustered_k4_80",
    "clue_shape_5", "shallow_18", "deep_18", "sweep_18", "sweep_09", "baseline_00",
]


def grep_field(line, key):
    for tok in line.split():
        if tok.startswith(key + "="):
            return tok.split("=", 1)[1]
    return None


def run_one(job, work, budget):
    kind, seed, layout, algo = job
    vdir = work / f"seed{seed}"
    jpath = vdir / f"{layout}.json"
    cpath = vdir / f"{layout}.csv"
    if not jpath.exists():
        return None
    if kind == "dfs":
        out = subprocess.run(
            [str(DFS), "--puzzle", str(jpath), "--algo", algo, "--seed", "1", "--budget-s", str(budget)],
            capture_output=True, text=True,
        )
        line = next((l for l in out.stdout.splitlines() if l.startswith("RESULT")), None)
        if not line:
            # Loud failure, not a silent drop. A panicked/mismatched binary must be
            # visible — a size-mismatched run_dfs once corrupted a whole grid by
            # failing silently here. Return an error marker the writer surfaces.
            return {"__error__": True, "kind": kind, "seed": seed, "layout": layout,
                    "algo": algo, "returncode": out.returncode,
                    "stderr": (out.stderr or "").strip()[:200]}
        return {
            "seed": seed, "layout": layout, "solver": f"dfs-{algo}",
            "score": int(grep_field(line, "score") or 0),
            "max_depth": int(grep_field(line, "max_depth") or 0),
            "depth_at_timeout": int(grep_field(line, "depth_at_timeout") or 0),
            "nodes": int(grep_field(line, "nodes") or 0),
            "backtracks": int(grep_field(line, "backtracks") or 0),
            "budget": budget,
        }
    else:
        env = dict(os.environ, RAYON_NUM_THREADS="1")
        out = subprocess.run(
            [str(PROD), str(cpath), "--order", "comb:14", "--beam", "20000",
             "--tol", "0", "--seed", "1", "--threads", "1"],
            capture_output=True, text=True, env=env,
        )
        row = next((l for l in out.stdout.splitlines() if l.startswith("comb")), None)
        score = int(row.split(",")[4]) if row else 0
        return {
            "seed": seed, "layout": layout, "solver": "beam-20k",
            "score": score, "max_depth": 0, "depth_at_timeout": 0,
            "nodes": 0, "backtracks": 0, "budget": budget,
        }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", required=True)
    ap.add_argument("--budget-s", type=float, default=8.0)
    ap.add_argument("--seeds", type=int, default=15)
    ap.add_argument("--parallel", type=int, default=6)
    ap.add_argument("--size", type=int, default=16)
    args = ap.parse_args()

    for tool in (GEN, DFS, PROD):
        if not tool.exists():
            sys.exit(f"missing build artifact: {tool}\nbuild the engines first (see the justfile recipe).")

    outdir = Path(args.out)
    outdir.mkdir(parents=True, exist_ok=True)
    work = outdir / "boards"

    # Pass 1: generate all boards (fast, serial).
    for seed in range(1, args.seeds + 1):
        subprocess.run([str(GEN), "--out", str(work / f"seed{seed}"),
                        "--size", str(args.size), "--seed", str(seed)],
                       capture_output=True, check=True)
    print(f"generated boards for {args.seeds} seeds", file=sys.stderr)

    # Pass 2: build the job list and run it `--parallel`-wide.
    jobs = []
    for seed in range(1, args.seeds + 1):
        for layout in LAYOUTS:
            if not (work / f"seed{seed}" / f"{layout}.json").exists():
                continue
            for algo in PATHS:
                jobs.append(("dfs", seed, layout, algo))
            jobs.append(("beam", seed, layout, None))
    print(f"{len(jobs)} jobs, {args.parallel}-wide, {args.budget_s}s each", file=sys.stderr)

    results_path = outdir / "results.jsonl"
    errors = []
    with open(results_path, "w") as f, ThreadPoolExecutor(max_workers=args.parallel) as ex:
        done = 0
        written = 0
        for res in ex.map(lambda j: run_one(j, work, args.budget_s), jobs):
            done += 1
            if res is None:
                continue
            if res.get("__error__"):
                errors.append(res)
                continue
            f.write(json.dumps(res) + "\n")
            f.flush()
            written += 1
            if done % 100 == 0:
                print(f"  {done}/{len(jobs)} ({written} ok, {len(errors)} failed)", file=sys.stderr)
    print(f"wrote {results_path}: {written} rows", file=sys.stderr)
    if errors:
        # Fail loudly: a silent drop once corrupted a grid (a size-8 run_dfs run
        # against 16x16 boards). Summarise the failures and exit non-zero so the
        # recipe does not proceed to make_site_json on a hole-y dataset.
        print(f"\nERROR: {len(errors)} solver runs FAILED (no RESULT line).", file=sys.stderr)
        sample = errors[0]
        print(f"  e.g. {sample['algo']} on {sample['layout']} seed {sample['seed']}: "
              f"rc={sample['returncode']} stderr={sample['stderr']!r}", file=sys.stderr)
        by_algo = {}
        for e in errors:
            by_algo[e["algo"]] = by_algo.get(e["algo"], 0) + 1
        print(f"  failures by algo: {by_algo}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
