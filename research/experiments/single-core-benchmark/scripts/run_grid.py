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
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import manifest  # noqa: E402  (local module, sibling of this script)

# Repo-relative: this script lives at <experiment>/scripts/run_grid.py, so the
# engine and default variants/results dirs are its siblings. No absolute paths.
HERE = Path(__file__).resolve().parent
EXPERIMENT = HERE.parent
BIN_DIR = EXPERIMENT / "engine" / "target" / "release"
RUN_ALGO = BIN_DIR / "run_algo"

# The grid is defined by ../solvers.toml (the single source of truth for which
# solvers exist, their I/O format, family, author and calibration). run_grid.py
# reads it — there is no hand-kept algo list here to drift out of sync. See
# manifest.py / solvers.toml.
SPEC = {s["name"]: s for s in manifest.load()}
NATIVE_ALGOS = manifest.by_kind(list(SPEC.values()), "native")
STANDALONE_ALGOS = manifest.by_kind(list(SPEC.values()), "standalone")

# The standalone wrapper (CSV variant in, url out). BENCH_STANDALONE_SH overrides.
STANDALONE_SH = os.environ.get("BENCH_STANDALONE_SH", str(HERE / "run_standalone.sh"))


def check_native_registry():
    """Cross-check the manifest's native names against `run_algo --list` (the
    engine registry). Errors on ANY mismatch so the manifest and the registry
    can never silently diverge — the anti-drift guarantee. Skipped only if the
    binary isn't built yet (caller already handles that case)."""
    if not RUN_ALGO.exists():
        return
    out = subprocess.check_output([str(RUN_ALGO), "--list"], text=True)
    registry = {ln.strip() for ln in out.splitlines() if ln.strip()}
    manifest_native = set(NATIVE_ALGOS)
    missing = manifest_native - registry           # in manifest, not runnable
    unlisted = registry - manifest_native           # runnable, not in manifest
    if missing or unlisted:
        raise SystemExit(
            "solvers.toml native entries disagree with `run_algo --list`:\n"
            f"  in manifest but not in registry: {sorted(missing)}\n"
            f"  in registry but not in manifest: {sorted(unlisted)}\n"
            "Fix solvers.toml or the bench-grid registry so they match.")

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
    # Delegates to a bash function in run_standalone.sh (opt-in via
    # BENCH_STANDALONE_SH): run_<algo> <csv> <seed> <budget_s> <out.url>.
    if not STANDALONE_SH:
        raise RuntimeError(
            f"{algo} needs the standalone wrapper; set BENCH_STANDALONE_SH to a "
            f"file that defines run_{algo}, or omit it from --algos")
    cmd = ["bash", "-c",
           f'source "{STANDALONE_SH}"; run_{algo} "{variant_csv}" {seed} {budget_s} "{url_path}"']
    env = dict(os.environ, RAYON_NUM_THREADS="1")
    # Pass this solver's calibration knobs (solvers.toml [solver.calibration]) as
    # BENCH_<ALGO>_<KNOB> env vars; run_standalone.sh reads them with the
    # committed-run values as defaults, so an absent knob changes nothing.
    for knob, val in SPEC.get(algo, {}).get("calibration", {}).items():
        env[f"BENCH_{algo.upper()}_{knob.upper()}"] = str(val)
    t0 = time.time()
    p = subprocess.run(cmd, capture_output=True, text=True, env=env,
                       timeout=budget_s + 60)
    wall = time.time() - t0
    m = SCORE_RE.search(p.stdout)
    score = int(m.group(1)) if m else None
    nodes = int(NODES_RE.search(p.stdout).group(1)) if NODES_RE.search(p.stdout) else None
    nps = int(NPS_RE.search(p.stdout).group(1)) if NPS_RE.search(p.stdout) else None
    um = NPS_UNIT_RE.search(p.stdout)
    # Native unit comes from the manifest when the wrapper didn't print one.
    default_unit = SPEC.get(algo, {}).get("nps_unit", "nodes/s")
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
    ap.add_argument("--native-only", action="store_true",
                    help="run only the native engine family (skip the four "
                         "standalone strong engines)")
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()

    if not RUN_ALGO.exists():
        raise SystemExit(
            f"run_algo not built. From {EXPERIMENT/'engine'} run:\n"
            f"  cargo build --release --bin run_algo")

    # Guarantee the manifest's native list matches the engine registry before we
    # run anything (so neither can silently drop or add a solver).
    check_native_registry()

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
            # kind + input format both come from the manifest, so how a solver is
            # driven and which variant file it gets are never name-guessed here.
            spec = SPEC.get(algo, {})
            fmt = spec.get("input", "csv" if spec.get("kind") == "standalone" else "json")
            vfile = variants_dir / f"variant_{vid:02d}.{fmt}"
            if spec.get("kind") == "standalone":
                score, wall, out, err, nodes, nps, unit = run_standalone(
                    algo, vfile, seed, args.budget_s, url_path)
            else:
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
