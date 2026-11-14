"""Track 1 pilot analysis (PREREG.md). Stdlib only.

analyze.py               -> results/summary.json, results/tables.md
analyze.py --build-check -> A22 binary rebuilt here reproduces A22 committed rows
"""
import glob
import json
import math
import os
import random
import sys

HERE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
A22 = os.path.join(HERE, "..", "mrv-baseline")
CAP = 100_000_000
BB, SCB = "bb-mrv", "scb-gacolor_ac3_random"
SEEDS = list(range(101, 109))
# A22 8x8 c=10 committed median_ln_ratio (../mrv-baseline/results/summary.json)
REF_LN = -1.9160979637943325
T = REF_LN / math.log(10)
BOOT_N, BOOT_SEED = 10_000, 20261111


def rows(path):
    with open(path) as f:
        return [json.loads(line) for line in f if line.strip()]


def build_check():
    bad = 0
    for s in (BB, SCB):
        mine = {r["seed"]: r for r in rows(os.path.join(HERE, f"results/check/{s}-plus2-n7-s1-10.jsonl"))}
        ref = {
            r["seed"]: r
            for r in rows(os.path.join(A22, f"results/raw/{s}/q2-plus2.jsonl"))
            if r["n"] == 7 and 1 <= r["seed"] <= 10
        }
        for seed in range(1, 11):
            a, b = mine[seed], ref[seed]
            same = (a["nodes"], a["status"], a["nodes_counted"]) == (b["nodes"], b["status"], b["nodes_counted"])
            bad += not same
            print(f"{s} n7 s{seed}: here {a['status']} {a['nodes']} | A22 {b['status']} {b['nodes']} {'ok' if same else 'MISMATCH'}")
    print("build check:", "PASS" if bad == 0 else f"FAIL ({bad} mismatches)")
    sys.exit(1 if bad else 0)


def kind(r):
    if r is None:
        return "missing"
    if r["status"] == "solved":
        return "solved"
    if r["status"] == "censored":
        return "node-censored" if r["nodes_counted"] >= r["cap"] else "time-censored"
    return "error"


def score(b, s):
    """(r_i, class) per PREREG.md table; log10."""
    kb, ks = kind(b), kind(s)
    if "error" in (kb, ks) or "missing" in (kb, ks):
        return None, "invalid"
    if "time-censored" in (kb, ks):
        return 0.0, "uninformative (time-censored)"
    if kb == "solved" and ks == "solved":
        return math.log10(b["nodes"] / s["nodes"]), "both solved"
    if kb == "solved":
        return math.log10(b["nodes"] / CAP), "scb node-censored (upper bound)"
    if ks == "solved":
        return math.inf, "bb-mrv node-censored (+inf)"
    return 0.0, "both node-censored"


def upper_median(v):
    v = sorted(v)
    return v[len(v) // 2]


def main():
    allr = []
    for p in sorted(glob.glob(os.path.join(HERE, "results/raw/wave*.jsonl")) + glob.glob(os.path.join(HERE, "results/remote/*/raw/wave*.jsonl"))):
        allr += rows(p)
    by = {}
    for r in allr:
        if r.get("seed") in SEEDS:
            by[(r["solver"], r["seed"])] = r
    pairs = []
    for seed in SEEDS:
        b, s = by.get((BB, seed)), by.get((SCB, seed))
        ri, cls = score(b, s)
        pairs.append({"seed": seed, "bb": b, "scb": s, "r": ri, "class": cls})
    valid = [p for p in pairs if p["r"] is not None]
    rs = [p["r"] for p in valid]
    u = sum(1 for p in valid if p["class"].startswith(("both node", "uninformative")))
    out = {"pairs_valid": len(valid), "pairs_invalid_or_missing": [p["seed"] for p in pairs if p["r"] is None]}
    if rs:
        m = upper_median(rs)
        rng = random.Random(BOOT_SEED)
        boots = sorted(upper_median([rng.choice(rs) for _ in rs]) for _ in range(BOOT_N))
        lo, hi = boots[int(0.025 * BOOT_N)], boots[int(0.975 * BOOT_N) - 1]
        k1 = m >= T
        k2 = u > 4
        out.update(
            M_upper_median_log10=m,
            M_ratio=10**m if m != math.inf else None,
            reference_T_log10=T,
            reference_ratio=10**T,
            boot95_log10=[lo, hi],
            U_uninformative=u,
            K1_fires=k1,
            K2_fires=k2,
            within_rounding=(-0.833 <= m < T),
            verdict="kill" if (k1 or k2) else "survives",
            complete=len(valid) == len(SEEDS),
        )
    # throughput and wall per run
    runs = []
    for r in allr:
        if r.get("status") in ("solved", "censored"):
            runs.append({
                "run": f"{r['solver']}-s{r['seed']}", "machine": r.get("machine"), "cpu": r.get("cpu"),
                "binary_sha256": r.get("binary_sha256"), "wave": r.get("wave"), "class": kind(r),
                "nodes_counted": r["nodes_counted"], "wall_s": r["wall_ms"] / 1000,
                "nodes_per_s": r["nodes_counted"] / (r["wall_ms"] / 1000),
            })
    out["runs"] = runs
    out["pairs"] = [
        {
            "seed": p["seed"], "class": p["class"], "r_log10": p["r"],
            "bb_nodes": p["bb"] and p["bb"]["nodes"], "bb_status": kind(p["bb"]),
            "scb_nodes": p["scb"] and p["scb"]["nodes"], "scb_status": kind(p["scb"]),
            "wall_ratio": (p["bb"]["wall_ms"] / p["scb"]["wall_ms"]) if p["bb"] and p["scb"] else None,
            "same_machine": bool(p["bb"] and p["scb"] and p["bb"].get("machine") == p["scb"].get("machine")),
        }
        for p in pairs
    ]
    with open(os.path.join(HERE, "results/summary.json"), "w") as f:
        json.dump(out, f, indent=1, default=str)
    lines = ["| seed | bb-mrv nodes | bb-mrv status | scb nodes | scb status | r (log10) | class |", "|---|---|---|---|---|---|---|"]
    for p in out["pairs"]:
        r = p["r_log10"]
        rt = "n/a" if r is None else ("+inf" if r == math.inf else f"{r:.3f}")
        lines.append(f"| {p['seed']} | {p['bb_nodes']} | {p['bb_status']} | {p['scb_nodes']} | {p['scb_status']} | {rt} | {p['class']} |")
    lines += ["", "| run | machine | wave | class | nodes_counted | wall s | nodes/s |", "|---|---|---|---|---|---|---|"]
    for x in sorted(runs, key=lambda x: x["run"]):
        lines.append(f"| {x['run']} | {x['machine']} | {x['wave']} | {x['class']} | {x['nodes_counted']} | {x['wall_s']:.0f} | {x['nodes_per_s']:.0f} |")
    with open(os.path.join(HERE, "results/tables.md"), "w") as f:
        f.write("\n".join(lines) + "\n")
    print("\n".join(lines))
    print(json.dumps({k: v for k, v in out.items() if k not in ("runs", "pairs")}, indent=1, default=str))


if __name__ == "__main__":
    build_check() if "--build-check" in sys.argv else main()
