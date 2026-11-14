#!/usr/bin/env python3
"""Analysis for difference-map-bands (PREREG.md). Stdlib only.

    python3 -I analysis/analyze.py [--raw results/raw] [--out results/summary.json]

Reads the raw rows, re-verifies every reported solution against the instance
file with an independent checker, and computes, per ladder, rung and arm:
solved fractions (runs and instances, Wilson 95%), the per-instance median over
run seeds, Kaplan-Meier rung medians, the slope fits with paired bootstrap
intervals, the kill rule as preregistered, and the wall-clock table.
"""
import json
import math
import random
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent.parent
INST = HERE.parent / "hard-ladder" / "instances"
BOOT = 10_000
BOOT_SEED = 20261111
KS = [0, 1, 2, 3]
LADDERS = {"primary": ("primary-test.jsonl", [6, 7, 8]), "8-c10": ("8-c10-test.jsonl", [8])}
BASE_CAP = 1_000_000


def rotated(e, r):
    return [e[(4 - r) & 3], e[(5 - r) & 3], e[(6 - r) & 3], e[(7 - r) & 3]]


def verify(pieces, n, board):
    if board is None or len(board) != n * n:
        return "no board"
    used = set()
    e = []
    for v in board:
        p, r = v // 4, v % 4
        if p in used or p >= n * n:
            return f"piece {p} reused"
        used.add(p)
        e.append(rotated(pieces[p], r))
    for y in range(n):
        for x in range(n):
            t, r, b, l = e[y * n + x]
            if (y == 0) != (t == 0) or (y == n - 1) != (b == 0) or (x == 0) != (l == 0) or (x == n - 1) != (r == 0):
                return f"rim ({x},{y})"
            if x + 1 < n and r != e[y * n + x + 1][3]:
                return f"seam right ({x},{y})"
            if y + 1 < n and b != e[(y + 1) * n + x][0]:
                return f"seam below ({x},{y})"
    return None


def wilson(k, n, z=1.96):
    if n == 0:
        return (float("nan"), float("nan"))
    p = k / n
    d = 1 + z * z / n
    c = (p + z * z / (2 * n)) / d
    h = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / d
    return (max(0.0, c - h), min(1.0, c + h))


def km_median(vals):
    """ceil(N/2)-th order statistic, censored = +inf (the ladder's KM median)."""
    v = sorted(vals)
    return v[(len(v) + 1) // 2 - 1]


def quart(vals, q):
    v = sorted(vals)
    return v[max(0, math.ceil(q * len(v)) - 1)]


def ols_slope(xs, ys):
    if any(math.isinf(y) for y in ys):
        return math.inf
    mx = sum(xs) / len(xs)
    my = sum(ys) / len(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)


def load_jsonl(p):
    if not p.exists():
        return []
    return [json.loads(l) for l in p.open() if l.strip()]


def main():
    args = sys.argv[1:]
    raw = Path(args[args.index("--raw") + 1]) if "--raw" in args else HERE / "results" / "raw"
    outp = Path(args[args.index("--out") + 1]) if "--out" in args else HERE / "results" / "summary.json"
    summary = {"checks": {}, "ladders": {}}
    verify_fail = 0
    verified = 0
    for lad, (fname, sizes) in LADDERS.items():
        insts = {(r["n"], r["seed"]): r for r in load_jsonl(INST / fname)}
        L = {"rungs": {}}
        # per-instance values: arm -> n -> {seed: (value or inf)}
        per = {}
        wall = {}
        rows_by_arm = {}
        for k in KS:
            rows = load_jsonl(raw / f"dm-k{k}-{lad}.jsonl")
            rows_by_arm[f"dm-k{k}"] = rows
            for r in rows:
                if r["status"] == "solved":
                    inst = insts[(r["n"], r["seed"])]
                    err = verify(inst["puzzle"]["pieces"], r["n"], r["board"])
                    verified += 1
                    if err or r.get("verify_error"):
                        verify_fail += 1
                        r["status"] = "invalid"
        for b in ("bb-mrv", "scb-gacolor_ac3_random"):
            rows_by_arm[b] = load_jsonl(raw / f"{b}-{lad}.jsonl")
        for arm, rows in rows_by_arm.items():
            if not rows:
                continue
            for n in sizes:
                rs = [r for r in rows if r["n"] == n]
                if not rs:
                    continue
                by = {}
                for r in rs:
                    by.setdefault(r["seed"], []).append(r)
                inst_val = {}
                inst_wall = {}
                for sd, lst in by.items():
                    if arm.startswith("dm-"):
                        its = [r["iterations"] if r["status"] == "solved" else math.inf for r in lst]
                        ws = [r["wall_ms"] if r["status"] == "solved" else math.inf for r in lst]
                    else:
                        its = [r["nodes"] if r["status"] == "solved" else math.inf for r in lst]
                        ws = [r["wall_ms"] if r["status"] == "solved" else math.inf for r in lst]
                    inst_val[sd] = km_median(its)
                    inst_wall[sd] = km_median(ws)
                per.setdefault(arm, {})[n] = inst_val
                wall.setdefault(arm, {})[n] = inst_wall
                runs_solved = sum(r["status"] == "solved" for r in rs)
                inst_solved = sum(not math.isinf(v) for v in inst_val.values())
                any_solved = sum(any(r["status"] == "solved" for r in lst) for lst in by.values())
                vals = list(inst_val.values())
                cell = {
                    "instances": len(by), "runs": len(rs),
                    "runs_solved": runs_solved, "runs_wilson95": wilson(runs_solved, len(rs)),
                    "instances_median_solved": inst_solved, "instances_wilson95": wilson(inst_solved, len(by)),
                    "instances_any_run_solved": any_solved,
                    "km_median": km_median(vals), "q1": quart(vals, 0.25), "q3": quart(vals, 0.75),
                    "km_median_wall_ms": km_median(list(inst_wall.values())),
                    "total_wall_s": sum(r["wall_ms"] for r in rs) / 1e3,
                    "total_wall_s_per_run_seed": sum(r["wall_ms"] for r in rs) / 1e3 / max(1, len(rs) // max(1, len(by))),
                }
                if arm.startswith("dm-"):
                    cell["median_us_per_iter"] = sorted(r["us_per_iter"] for r in rs)[len(rs) // 2]
                    cens = [r for r in rs if r["status"] != "solved"]
                    cell["censored_runs"] = len(cens)
                    cell["censored_cycling"] = sum(r.get("cycle_period") is not None for r in cens)
                    cell["censored_terminal_mismatches_median"] = (
                        sorted(r["terminal_mismatches"] for r in cens)[len(cens) // 2] if cens else None)
                    cell["censored_best_mismatches_median"] = (
                        sorted(r["best_mismatches"] for r in cens)[len(cens) // 2] if cens else None)
                    cell["censored_distinct_layouts_last2000_median"] = (
                        sorted(r["distinct_layouts_last2000"] or 0 for r in cens)[len(cens) // 2] if cens else None)
                    cell["equals_planted_runs"] = sum(bool(r.get("equals_planted")) for r in rs)
                L["rungs"].setdefault(str(n), {})[arm] = cell
        # slopes and kill rule (primary only is binding; computed for both where >= 2 rungs)
        if len(sizes) >= 2 and all(f"dm-k{k}" in per for k in KS) and "bb-mrv" in per:
            cells = [n * n for n in sizes]
            seeds = {n: sorted(per["bb-mrv"][n]) for n in sizes}

            def slopes(idx):
                out = {}
                for k in KS:
                    ys = []
                    for n in sizes:
                        v = [per[f"dm-k{k}"][n].get(seeds[n][i], math.inf) for i in idx[n]]
                        m = km_median(v)
                        ys.append(math.log10(m) if not math.isinf(m) else math.inf)
                    out[k] = ols_slope(cells, ys)
                ys = []
                for n in sizes:
                    v = [per["bb-mrv"][n][seeds[n][i]] for i in idx[n]]
                    m = km_median(v)
                    ys.append(math.log10(BASE_CAP if math.isinf(m) else m))
                out["base"] = ols_slope(cells, ys)
                return out

            full = {n: list(range(len(seeds[n]))) for n in sizes}
            pt = slopes(full)
            rng = random.Random(BOOT_SEED)
            boots = []
            for _ in range(BOOT):
                idx = {n: [rng.randrange(len(seeds[n])) for _ in seeds[n]] for n in sizes}
                boots.append(slopes(idx))

            def ci(vals):
                v = sorted(vals)
                return (v[int(0.025 * len(v))], v[int(0.975 * len(v)) - 1])

            def diff(a, b):
                if math.isinf(a) and math.isinf(b):
                    return 0.0
                return a - b

            sl = {}
            for k in KS:
                sl[f"s_{k}"] = {"point": pt[k], "ci95": ci([b[k] for b in boots])}
            sl["s_base_bb_mrv"] = {"point": pt["base"], "ci95": ci([b["base"] for b in boots])}
            d30 = [diff(b[3], b[0]) for b in boots]
            sl["s3_minus_s0"] = {"point": diff(pt[3], pt[0]), "ci95": ci(d30)}
            if not math.isinf(pt[0]) and pt[0] > 0 and not math.isinf(pt[3]):
                rel = (pt[0] - pt[3]) / pt[0]
            elif math.isinf(pt[0]) and not math.isinf(pt[3]):
                rel = math.inf
            else:
                rel = None
            sl["relative_fall_s0_to_s3"] = rel
            kill_a = pt[3] > 0.5 * pt["base"]
            fall_ok = rel is not None and rel >= 0.25
            ci_ok = sl["s3_minus_s0"]["ci95"][1] < 0
            kill_b = not (fall_ok and ci_ok)
            sl["kill"] = {"a_s3_gt_half_sbase": kill_a, "b_fall_lt_25pct_or_ci_not_below_0": kill_b,
                          "fall_ge_25pct": fall_ok, "ci_upper_below_0": ci_ok, "dead": kill_a or kill_b,
                          "binding": lad == "primary"}
            L["slopes"] = sl
        summary["ladders"][lad] = L
    summary["checks"] = {"solutions_reverified": verified, "verification_failures": verify_fail}
    outp.write_text(json.dumps(summary, indent=1, default=lambda o: None))
    print(json.dumps(summary["checks"]))
    for lad, L in summary["ladders"].items():
        print(f"== {lad}")
        for n, arms in L["rungs"].items():
            for arm, c in arms.items():
                print(f"  n={n} {arm:24s} inst {c['instances_median_solved']:2d}/{c['instances']} "
                      f"runs {c['runs_solved']}/{c['runs']} KM med {c['km_median']} "
                      f"wall KM med ms {c['km_median_wall_ms']} total s {c['total_wall_s']:.1f}"
                      + (f" us/it {c['median_us_per_iter']:.0f}" if 'median_us_per_iter' in c else ""))
        if "slopes" in L:
            print("  slopes", json.dumps(L["slopes"], default=str))


if __name__ == "__main__":
    main()
