#!/usr/bin/env python3
"""Run the A4 grid (calibration or main) with the qd_basins binary.

Standard library only. Every run is single-threaded; `--parallel` runs that
many at once. Paths are relative to research/experiments/.
"""
import argparse
import json
import os
import subprocess
import sys
import time
from concurrent.futures import ThreadPoolExecutor

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
BIN = os.path.join(ROOT, "engine", "target", "release", "qd_basins")

BURST = 20_000
METHODS = ["pt", "me"]
SIZES = [8, 10]
COLOUR_CANDIDATES = {8: [8, 10, 12, 14], 10: [10, 12, 14, 16]}
CAL_PUZZLE_SEEDS = [9001, 9002, 9003, 9004]
BUDGET_LADDER = [40_000_000 * 2**i for i in range(8)]  # 40M .. 5.12G, multiples of 8*BURST
TARGET_WALL_S = 75.0
MAIN_PUZZLE_SEEDS = list(range(1, 31))
MAIN_RUN_SEEDS = [1, 2]


def run_one(size, colors, pseed, rseed, method, evals, out):
    if os.path.exists(out):
        return out
    cmd = [BIN, "run", "--size", str(size), "--colors", str(colors),
           "--puzzle-seed", str(pseed), "--run-seed", str(rseed),
           "--method", method, "--evals", str(evals), "--burst", str(BURST),
           "--out", out + ".tmp"]
    subprocess.run(cmd, check=True)
    os.replace(out + ".tmp", out)
    return out


def summary(path):
    with open(path) as f:
        r = json.load(f)
    r.pop("candidates")
    r.pop("best_cells")
    return r


def run_many(jobs, parallel):
    with ThreadPoolExecutor(max_workers=parallel) as ex:
        futs = [ex.submit(run_one, *j) for j in jobs]
        done = 0
        for fu in futs:
            fu.result()
            done += 1
            if done % 10 == 0 or done == len(futs):
                print(f"  {done}/{len(futs)} runs", file=sys.stderr, flush=True)
    return [j[-1] for j in jobs]


def calibrate(args):
    os.makedirs(args.scratch, exist_ok=True)
    cal = {"burst": BURST, "sizes": {}}
    rows = []
    for size in SIZES:
        # Step 1: throughput of PT at this size (first candidate colour count).
        probe_evals = BUDGET_LADDER[0]
        out = os.path.join(args.scratch, f"probe_{size}.json")
        if os.path.exists(out):
            os.remove(out)
        run_one(size, COLOUR_CANDIDATES[size][0], CAL_PUZZLE_SEEDS[0], 1, "pt", probe_evals, out)
        s = summary(out)
        rate = s["evals"] / s["wall_s"]
        budget = max([b for b in BUDGET_LADDER if b / rate <= TARGET_WALL_S] or [BUDGET_LADDER[0]])
        # Step 2: smallest colour count whose pooled solve rate is <= 25%.
        chosen = None
        per_colour = {}
        for c in COLOUR_CANDIDATES[size]:
            jobs = [(size, c, p, 1, m, budget, os.path.join(args.scratch, f"cal_{size}_{c}_{p}_{m}_{budget}.json"))
                    for p in CAL_PUZZLE_SEEDS for m in METHODS]
            outs = run_many(jobs, args.parallel)
            ss = [summary(o) for o in outs]
            rows.extend(ss)
            solved = sum(1 for x in ss if x["best_score"] == x["total_edges"])
            per_colour[c] = {"runs": len(ss), "solved": solved,
                             "solved_pt": sum(1 for x in ss if x["method"] == "pt" and x["best_score"] == x["total_edges"]),
                             "solved_me": sum(1 for x in ss if x["method"] == "me" and x["best_score"] == x["total_edges"]),
                             "mean_best": sum(x["best_score"] for x in ss) / len(ss),
                             "total_edges": ss[0]["total_edges"]}
            print(f"size {size} colours {c}: solved {solved}/{len(ss)}", file=sys.stderr, flush=True)
            if solved <= 0.25 * len(ss):
                chosen = c
                break
        if chosen is None:
            chosen = COLOUR_CANDIDATES[size][-1]
        cal["sizes"][str(size)] = {"probe_evals_per_s": rate, "evals_budget": budget,
                                   "colors": chosen, "colour_trials": per_colour}
    with open(os.path.join(args.out, "calibration.json"), "w") as f:
        json.dump(cal, f, indent=2, sort_keys=True)
        f.write("\n")
    with open(os.path.join(args.out, "calibration_runs.jsonl"), "w") as f:
        for r in sorted(rows, key=lambda r: (r["size"], r["colors"], r["puzzle_seed"], r["method"])):
            r = {k: v for k, v in r.items() if k not in ("wall_s", "cpu_s")} | {"wall_s": round(r["wall_s"], 2)}
            f.write(json.dumps(r, sort_keys=True) + "\n")


def main_grid(args):
    with open(args.calibration) as f:
        cal = json.load(f)
    os.makedirs(args.runs, exist_ok=True)
    jobs = []
    for size in SIZES:
        sc = cal["sizes"][str(size)]
        for p in MAIN_PUZZLE_SEEDS:
            for r in MAIN_RUN_SEEDS:
                for m in METHODS:
                    jobs.append((size, sc["colors"], p, r, m, sc["evals_budget"],
                                 os.path.join(args.runs, f"run_{size}_{p:03d}_{r}_{m}.json")))
    t0 = time.time()
    run_many(jobs, args.parallel)
    print(f"main grid wall clock {time.time() - t0:.0f} s", file=sys.stderr)


def main():
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)
    c = sub.add_parser("calibrate")
    c.add_argument("--out", required=True)
    c.add_argument("--scratch", required=True)
    c.add_argument("--parallel", type=int, default=7)
    m = sub.add_parser("main")
    m.add_argument("--calibration", required=True)
    m.add_argument("--runs", required=True)
    m.add_argument("--parallel", type=int, default=7)
    args = ap.parse_args()
    calibrate(args) if args.cmd == "calibrate" else main_grid(args)


if __name__ == "__main__":
    main()
