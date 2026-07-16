#!/usr/bin/env python3
"""Repair study grid runner.

Runs every repair variant once on each of the ten corner-pinned variants, single
core, fixed wall budget, at most PARALLEL at a time. Every run emits a bucas .url
and a convergence-curve sidecar; the score is the canonical matched-edge count
from the RESULT line. Raw results stream to results.jsonl.

The variant list comes from the engine registry itself (`run_repair
--matrix-json`), so there is no hand-kept algo list to drift.

Usage:
  python3 run_grid.py --variants ../variants --out ../results/rerun \\
      --budget-s 60 --parallel 6 [--algos a,b,c] [--seed N] [--dry-run]
"""
import argparse
import concurrent.futures as cf
import json
import os
import re
import subprocess
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
EXPERIMENT = HERE.parent
BIN_DIR = EXPERIMENT / "engine" / "target" / "release"
RUN_REPAIR = BIN_DIR / "run_repair"

INT_FIELDS = (
    "score", "start_score", "lift", "iterations", "last_best_iter",
    "accepts", "improvements", "best_improvements", "restarts",
)
FLOAT_FIELDS = ("accept_rate", "mean_destroy", "elapsed_s", "ips")
INT_RE = {k: re.compile(rf"\b{k}=(-?\d+)") for k in INT_FIELDS}
FLOAT_RE = {k: re.compile(rf"\b{k}=([\d.]+)") for k in FLOAT_FIELDS}
UNIT_RE = re.compile(r"ips_unit=(\S+)")


def load_matrix():
    out = subprocess.check_output([str(RUN_REPAIR), "--matrix-json"], text=True)
    return json.loads(out)


def run_one(spec, variant_json, seed, budget_s, url_path, curve_path):
    """Run a single (variant, instance). Returns a result dict."""
    name = spec["name"]
    cmd = [str(RUN_REPAIR), "--puzzle", str(variant_json), "--algo", name,
           "--seed", str(seed), "--budget-s", str(budget_s),
           "--emit", str(url_path), "--emit-curve", str(curve_path)]
    # Single core: the engine is single-threaded, but pin anyway.
    env = dict(os.environ, RAYON_NUM_THREADS="1")
    t0 = time.time()
    try:
        p = subprocess.run(cmd, capture_output=True, text=True, env=env,
                           timeout=budget_s + 60)
        stdout, stderr, rc = p.stdout, p.stderr, p.returncode
    except subprocess.TimeoutExpired:
        stdout, stderr, rc = "", "timeout", -1
    wall = time.time() - t0

    def ival(k):
        m = INT_RE[k].search(stdout)
        return int(m.group(1)) if m else None

    def fval(k):
        m = FLOAT_RE[k].search(stdout)
        return float(m.group(1)) if m else None

    um = UNIT_RE.search(stdout)
    ok = rc == 0 and ival("score") is not None
    row = {"algo": name, "ok": ok, "wall_s": round(wall, 2),
           "url_file": str(url_path.name), "curve_file": str(curve_path.name),
           "ips_unit": um.group(1) if um else "repair-iters/s",
           "err": "" if ok else stderr.strip()[-400:]}
    for k in INT_FIELDS:
        row[k] = ival(k)
    for k in FLOAT_FIELDS:
        row[k] = fval(k)
    return row


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--variants", default=str(EXPERIMENT / "variants"))
    ap.add_argument("--out", default=str(EXPERIMENT / "results" / "rerun"))
    ap.add_argument("--budget-s", type=int, default=60)
    ap.add_argument("--seed", type=int, default=1)
    ap.add_argument("--parallel", type=int, default=6)
    ap.add_argument("--algos", default="", help="comma-separated subset; default all")
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()

    if not RUN_REPAIR.exists():
        raise SystemExit(f"missing {RUN_REPAIR}; build with `cargo build --release` first")

    matrix = load_matrix()
    specs = matrix
    if args.algos:
        wanted = set(args.algos.split(","))
        specs = [s for s in specs if s["name"] in wanted]

    variants_dir = Path(args.variants)
    variant_files = sorted(variants_dir.glob("variant_*.json"))
    if not variant_files:
        raise SystemExit(f"no variant_*.json under {variants_dir}")

    out_dir = Path(args.out)
    out_dir.mkdir(parents=True, exist_ok=True)
    url_dir = out_dir / "urls"
    curve_dir = out_dir / "curves"
    url_dir.mkdir(exist_ok=True)
    curve_dir.mkdir(exist_ok=True)

    jobs = []
    for spec in specs:
        for vf in variant_files:
            vid = int(re.search(r"variant_(\d+)", vf.name).group(1))
            url_path = url_dir / f"{spec['name']}_v{vid:02d}.url"
            curve_path = curve_dir / f"{spec['name']}_v{vid:02d}.csv"
            jobs.append((spec, vf, vid, url_path, curve_path))

    print(f"{len(specs)} variants x {len(variant_files)} instances "
          f"= {len(jobs)} runs, {args.budget_s}s each, parallel={args.parallel}")
    if args.dry_run:
        for spec, vf, vid, _, _ in jobs:
            print(f"  {spec['name']:<20} variant {vid}")
        return

    results = []
    results_path = out_dir / "results.jsonl"
    with open(results_path, "w") as fh, \
            cf.ThreadPoolExecutor(max_workers=args.parallel) as ex:
        futs = {
            ex.submit(run_one, spec, vf, args.seed, args.budget_s, url_path, curve_path): (spec, vid)
            for spec, vf, vid, url_path, curve_path in jobs
        }
        done = 0
        for fut in cf.as_completed(futs):
            spec, vid = futs[fut]
            row = fut.result()
            row["variant"] = vid
            results.append(row)
            fh.write(json.dumps(row) + "\n")
            fh.flush()
            done += 1
            status = "ok" if row["ok"] else "FAIL"
            print(f"[{done}/{len(jobs)}] {row['algo']:<20} v{vid:02d} "
                  f"score={row['score']} lift={row['lift']} "
                  f"lastbest={row['last_best_iter']} {status}")

    (out_dir / "matrix.json").write_text(json.dumps(matrix, indent=1) + "\n")
    (out_dir / "run_meta.json").write_text(json.dumps({
        "budget_s": args.budget_s,
        "seed": args.seed,
        "parallel": args.parallel,
        "variant_ids": sorted({int(re.search(r"variant_(\d+)", vf.name).group(1)) for vf in variant_files}),
        "n_runs": len(jobs),
    }, indent=1) + "\n")

    n_fail = sum(1 for r in results if not r["ok"])
    print(f"\ndone: {len(results)} runs, {n_fail} failures -> {results_path}")
    if n_fail:
        sys.exit(1)


if __name__ == "__main__":
    main()
