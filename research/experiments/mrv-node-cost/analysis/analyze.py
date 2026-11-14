"""Track 0 analysis (see ../PREREG.md). Stdlib only.

Reads results/raw/gate-*.jsonl and results/raw/time-*.jsonl, writes
results/summary.json and prints the tables used in README.md.
"""

import glob
import json
import os
import random
import statistics

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.join(HERE, "..", "results")
RAW = os.path.join(RES, "raw")
B = 10_000
SEED = 20261111


def rows(pattern):
    out = []
    for p in sorted(glob.glob(os.path.join(RAW, pattern))):
        with open(p) as f:
            out += [json.loads(l) for l in f if l.strip()]
    return out


def rung(r):
    return f"R{r['n']}"


def upper_median(xs):
    s = sorted(xs)
    return s[len(s) // 2]


def boot_ci(xs, rng):
    k = len(xs)
    meds = sorted(upper_median([xs[rng.randrange(k)] for _ in range(k)]) for _ in range(B))
    return meds[int(0.025 * B)], meds[int(0.975 * B) - 1]


def gate_summary(rs):
    by = {}
    for r in rs:
        g = by.setdefault(rung(r) if r["cap"] == 1_000_000 else f"supp-n{r['n']}", {
            "instances": 0, "ref_eq_orig": 0, "fast_eq_ref": 0, "mismatches": [],
            "solved": 0, "censored": 0, "nodes": 0, "ms_orig": 0.0, "ms_ref": 0.0, "ms_fast": 0.0,
        })
        g["instances"] += 1
        g["ref_eq_orig"] += r["ref_eq_orig"]
        g["fast_eq_ref"] += r["fast_eq_ref"]
        if not (r["ref_eq_orig"] and r["fast_eq_ref"]):
            g["mismatches"].append(r["instance"])
        g[r["orig"]["status"]] = g.get(r["orig"]["status"], 0) + 1
        g["nodes"] += r["orig"]["nodes"]
        for a in ("orig", "ref", "fast"):
            g[f"ms_{a}"] += r[a]["ms"]
    return by


def timing_summary(rs, rng):
    inst = {}
    for r in rs:
        d = inst.setdefault(r["instance"], {"n": r["n"], "runs": []})
        d["runs"].append(r)
    out = {}
    for name in sorted(inst, key=lambda k: (inst[k]["n"], k)):
        d = inst[name]
        runs = d["runs"]
        assert len(runs) == 10, name
        same = len({(x["status"], x["nodes"], x["backtracks"], x["propagation"]) for x in runs}) == 1
        t = {a: [x["wall_ms"] for x in runs if x["solver"] == a] for a in ("bb-mrv", "bb-mrv-fast")}
        assert all(len(v) == 5 for v in t.values()), name
        g = out.setdefault(f"R{d['n']}", {"pairs": [], "identical_runs": 0, "instances": 0})
        g["instances"] += 1
        g["identical_runs"] += same
        g["pairs"].append({
            "instance": name, "status": runs[0]["status"], "nodes": runs[0]["nodes"],
            "min_orig": min(t["bb-mrv"]), "min_fast": min(t["bb-mrv-fast"]),
            "med_orig": statistics.median(t["bb-mrv"]), "med_fast": statistics.median(t["bb-mrv-fast"]),
        })
    for g in out.values():
        p = g["pairs"]
        r_min = [x["min_fast"] / x["min_orig"] for x in p]
        r_med = [x["med_fast"] / x["med_orig"] for x in p]
        g["ratio_min_median"] = upper_median(r_min)
        g["ratio_min_ci95"] = boot_ci(r_min, rng)
        g["ratio_med_median"] = upper_median(r_med)
        g["ratio_med_ci95"] = boot_ci(r_med, rng)
        g["ratio_min_range"] = [min(r_min), max(r_min)]
        nodes = sum(x["nodes"] for x in p)
        g["total_nodes"] = nodes
        g["sum_min_orig_ms"] = sum(x["min_orig"] for x in p)
        g["sum_min_fast_ms"] = sum(x["min_fast"] for x in p)
        g["nodes_per_s_orig"] = nodes / (g["sum_min_orig_ms"] / 1000)
        g["nodes_per_s_fast"] = nodes / (g["sum_min_fast_ms"] / 1000)
        g["pass_ci_below_1"] = g["ratio_min_ci95"][1] < 1
    return out


def main():
    rng = random.Random(SEED)
    gate = gate_summary(rows("gate-*.jsonl"))
    timing = timing_summary(rows("time-*.jsonl"), rng)
    binding = {k: v for k, v in gate.items() if not k.startswith("supp")}
    gate_pass = sum(v["instances"] for v in binding.values()) == 140 and all(
        not v["mismatches"] for v in binding.values()
    )
    timing_ok = all(v["identical_runs"] == v["instances"] for v in timing.values())
    verdict = {
        "gate_instances": sum(v["instances"] for v in binding.values()),
        "gate_mismatches": sum(len(v["mismatches"]) for v in binding.values()),
        "gate_pass": gate_pass,
        "timing_runs_identical": timing_ok,
        "rungs_timed": sorted(timing),
        "all_rungs_ci_below_1": len(timing) == 3 and all(v["pass_ci_below_1"] for v in timing.values()),
    }
    verdict["adopt"] = verdict["gate_pass"] and verdict["timing_runs_identical"] and verdict["all_rungs_ci_below_1"]
    summary = {"verdict": verdict, "gate": gate, "timing": timing}
    with open(os.path.join(RES, "summary.json"), "w") as f:
        json.dump(summary, f, indent=1, sort_keys=True)
        f.write("\n")

    print("gate")
    for k, v in sorted(gate.items()):
        print(f"  {k:8} inst {v['instances']:3} ref==orig {v['ref_eq_orig']:3} fast==ref {v['fast_eq_ref']:3} "
              f"mismatch {len(v['mismatches'])} nodes {v['nodes']} "
              f"s orig {v['ms_orig']/1e3:.1f} ref {v['ms_ref']/1e3:.1f} fast {v['ms_fast']/1e3:.1f}")
    print("timing (ratio = fast / orig, per-instance min of 5; secondary = median of 5)")
    for k in sorted(timing, key=lambda s: int(s[1:])):
        v = timing[k]
        lo, hi = v["ratio_min_ci95"]
        mlo, mhi = v["ratio_med_ci95"]
        print(f"  {k}: n={v['instances']} identical {v['identical_runs']} median ratio {v['ratio_min_median']:.3f} "
              f"[{lo:.3f}, {hi:.3f}] (speed-up {1/v['ratio_min_median']:.2f}x); secondary {v['ratio_med_median']:.3f} "
              f"[{mlo:.3f}, {mhi:.3f}]; range {v['ratio_min_range'][0]:.3f}..{v['ratio_min_range'][1]:.3f}; "
              f"nodes/s orig {v['nodes_per_s_orig']:.0f} fast {v['nodes_per_s_fast']:.0f}; "
              f"sum min s orig {v['sum_min_orig_ms']/1e3:.1f} fast {v['sum_min_fast_ms']/1e3:.1f}")
    print("verdict", json.dumps(verdict))


if __name__ == "__main__":
    main()
