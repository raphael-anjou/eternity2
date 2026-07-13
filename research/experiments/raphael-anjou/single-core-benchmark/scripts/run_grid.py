#!/usr/bin/env python3
"""Single-core apples-to-apples benchmark grid runner (public port).

Lifted from the v2 research vault and made repo-relative. The diversity axis is
the puzzle: each algorithm runs ONCE per corner-pinned variant (10 puzzles => 10
runs per algorithm), single-core, fixed wall budget, at most PARALLEL at a time.
Every run emits a bucas .url; the score is the canonical matched-edge count from
the RESULT line. Raw results stream to results.jsonl (timestamped run dir).

Two engine families share one puzzle-in / url-out interface:

  * native      -> ../engine/target/release/run_algo  (JSON variant in, url out)
                   RUNNABLE HERE. This is the ported engine family (naive + CSP
                   presets, ranks 5-15 in the report).
  * standalone  -> the four strong engines (producer, blackwood, verhaard, alns)
                   live in the private v2 vault and are NOT in this repo. They
                   are skipped unless BENCH_STANDALONE_SH points at a wrapper
                   script that defines run_<algo> bash functions. Their published
                   results are committed under ../results; this runner reproduces
                   only the native family.

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
import time
from pathlib import Path

# Repo-relative: this script lives at <experiment>/scripts/run_grid.py, so the
# engine and default variants/results dirs are its siblings. No absolute paths.
HERE = Path(__file__).resolve().parent
EXPERIMENT = HERE.parent
RUN_ALGO = EXPERIMENT / "engine" / "target" / "release" / "run_algo"

# native engine-family algos (driven by run_algo, JSON variant input). This is
# the exact registry the ported run_algo exposes (see `run_algo --list`).
NATIVE_ALGOS = [
    "naive_rowmajor",
    "border_first_lcv",
    "border_first_full",
    "rare_color_first",
    "gacolor_ac3",
    "gacolor_ac3_ns1",
    "gacolor_ac3_random",
    "border_first_random",
    "joe_depth150",
    "joe_depth150_bp",
    "verhaard_preferred",
]
# standalone strong engines: private vault binaries, opt-in only (see module doc).
STANDALONE_ALGOS = ["producer", "blackwood", "verhaard", "alns"]
STANDALONE_SH = os.environ.get("BENCH_STANDALONE_SH", "")

RESULT_RE = re.compile(r"score=(\d+)")
SCORE_RE = re.compile(r"canonical_score=(\d+)")
NODES_RE = re.compile(r"nodes=(\d+)")
NPS_RE = re.compile(r"nps=(\d+)")
NPS_UNIT_RE = re.compile(r"nps_unit=(\S+)")


def run_native(algo, variant_json, seed, budget_s, url_path):
    cmd = [
        str(RUN_ALGO),
        "--puzzle", str(variant_json),
        "--algo", algo,
        "--seed", str(seed),
        "--budget-ms", str(budget_s * 1000),
        "--emit", str(url_path),
    ]
    # Single core: run_algo uses only single-core presets, but pin anyway.
    env = dict(os.environ, RAYON_NUM_THREADS="1")
    t0 = time.time()
    p = subprocess.run(cmd, capture_output=True, text=True, env=env,
                       timeout=budget_s + 30)
    wall = time.time() - t0
    m = RESULT_RE.search(p.stdout)
    score = int(m.group(1)) if m else None
    nodes = int(NODES_RE.search(p.stdout).group(1)) if NODES_RE.search(p.stdout) else None
    nps = int(NPS_RE.search(p.stdout).group(1)) if NPS_RE.search(p.stdout) else None
    um = NPS_UNIT_RE.search(p.stdout)
    nps_unit = um.group(1) if um else "search-nodes/s"
    return score, wall, p.stdout.strip(), p.stderr.strip()[-500:], nodes, nps, nps_unit


def run_standalone(algo, variant_csv, seed, budget_s, url_path):
    # Delegates to a bash function from the private vault wrapper (opt-in via
    # BENCH_STANDALONE_SH): run_<algo> <csv> <seed> <budget_s> <out.url>.
    if not STANDALONE_SH:
        raise RuntimeError(
            f"{algo} is a private-vault engine; set BENCH_STANDALONE_SH to a "
            f"wrapper that defines run_{algo}, or omit it from --algos")
    cmd = ["bash", "-c",
           f'source "{STANDALONE_SH}"; run_{algo} "{variant_csv}" {seed} {budget_s} "{url_path}"']
    env = dict(os.environ, RAYON_NUM_THREADS="1")
    t0 = time.time()
    p = subprocess.run(cmd, capture_output=True, text=True, env=env,
                       timeout=budget_s + 60)
    wall = time.time() - t0
    m = SCORE_RE.search(p.stdout)
    score = int(m.group(1)) if m else None
    nodes = int(NODES_RE.search(p.stdout).group(1)) if NODES_RE.search(p.stdout) else None
    nps = int(NPS_RE.search(p.stdout).group(1)) if NPS_RE.search(p.stdout) else None
    um = NPS_UNIT_RE.search(p.stdout)
    default_unit = {
        "producer": "beam-nodes/s",
        "blackwood": "search-nodes/s",
        "verhaard": "search-nodes/s",
        "alns": "iters/s",
    }.get(algo, "nodes/s")
    nps_unit = um.group(1) if um else default_unit
    return score, wall, p.stdout.strip()[-500:], p.stderr.strip()[-500:], nodes, nps, nps_unit


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--variants", default=str(EXPERIMENT / "variants"),
                    help="dir of variant_NN.json (+ .csv for standalone)")
    ap.add_argument("--out", default=str(EXPERIMENT / "results" / "rerun"))
    ap.add_argument("--budget-s", type=int, default=60)
    ap.add_argument("--parallel", type=int, default=6)
    ap.add_argument("--seed", type=int, default=1,
                    help="fixed seed for every run (one run per variant)")
    ap.add_argument("--algos", default="")
    ap.add_argument("--native-only", action="store_true", default=True,
                    help="only the runnable native family (default; standalone "
                         "engines need the private vault)")
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()

    if not RUN_ALGO.exists():
        raise SystemExit(
            f"run_algo not built. From {EXPERIMENT/'engine'} run:\n"
            f"  cargo build --release --bin run_algo")

    variants_dir = Path(args.variants)
    variant_ids = sorted(int(p.stem.split("_")[1])
                         for p in variants_dir.glob("variant_*.json"))
    if not variant_ids:
        raise SystemExit(f"no variant_*.json in {variants_dir}")
    run_dir = Path(args.out)
    (run_dir / "urls").mkdir(parents=True, exist_ok=True)

    if args.algos:
        algos = args.algos.split(",")
    else:
        algos = list(NATIVE_ALGOS)
        if not args.native_only and STANDALONE_SH:
            algos += STANDALONE_ALGOS

    # One run per (algo, variant): 10 puzzles -> 10 runs per algorithm.
    jobs = [(algo, vid) for algo in algos for vid in variant_ids]

    meta = {
        "budget_s": args.budget_s,
        "seed": args.seed,
        "parallel": args.parallel,
        "algos": algos,
        "variant_ids": variant_ids,
        "variants_dir": str(variants_dir),
        "n_jobs": len(jobs),
    }
    (run_dir / "run_meta.json").write_text(json.dumps(meta, indent=2))
    print(f"grid: {len(algos)} algos x {len(variant_ids)} variants "
          f"= {len(jobs)} jobs @ {args.budget_s}s, "
          f"{args.parallel}-wide (one run per puzzle)", flush=True)
    if args.dry_run:
        print("dry-run; not executing.")
        return

    results_path = run_dir / "results.jsonl"
    done = 0
    t_start = time.time()

    def one(job):
        algo, vid = job
        seed = args.seed
        tag = f"{algo}__v{vid:02d}"
        url_path = run_dir / "urls" / f"{tag}.url"
        try:
            if algo in STANDALONE_ALGOS:
                vfile = variants_dir / f"variant_{vid:02d}.csv"
                score, wall, out, err, nodes, nps, unit = run_standalone(
                    algo, vfile, seed, args.budget_s, url_path)
            else:
                vfile = variants_dir / f"variant_{vid:02d}.json"
                score, wall, out, err, nodes, nps, unit = run_native(
                    algo, vfile, seed, args.budget_s, url_path)
            return dict(algo=algo, variant=vid, seed=seed,
                        score=score, wall_s=round(wall, 1),
                        nodes=nodes, nps=nps, nps_unit=unit,
                        url=str(url_path), ok=score is not None, err=err[:200])
        except subprocess.TimeoutExpired:
            return dict(algo=algo, variant=vid, seed=seed,
                        score=None, wall_s=None, nodes=None, nps=None,
                        nps_unit=None, url=str(url_path),
                        ok=False, err="TIMEOUT")
        except Exception as e:  # noqa
            return dict(algo=algo, variant=vid, seed=seed,
                        score=None, wall_s=None, nodes=None, nps=None,
                        nps_unit=None, url=str(url_path),
                        ok=False, err=f"EXC {e}")

    with open(results_path, "w") as rf, \
         cf.ThreadPoolExecutor(max_workers=args.parallel) as ex:
        for res in ex.map(one, jobs):
            rf.write(json.dumps(res) + "\n")
            rf.flush()
            done += 1
            elapsed = time.time() - t_start
            rate = done / elapsed if elapsed else 0
            eta = (len(jobs) - done) / rate if rate else 0
            npsinfo = (f"{res['nps']}{('/'+res['nps_unit']) if res.get('nps_unit') else ''}"
                       if res.get('nps') else "nps=?")
            print(f"[{done}/{len(jobs)}] {res['algo']} v{res['variant']:02d} "
                  f"score={res['score']} nps={npsinfo} "
                  f"wall={res['wall_s']}s  ETA~{eta/60:.1f}min", flush=True)

    print(f"\nDONE {done} jobs in {(time.time()-t_start)/60:.1f} min -> "
          f"{results_path}", flush=True)


if __name__ == "__main__":
    main()
