"""Exploratory equal-CPU check at R7 (PREREG_AMENDMENTS.md E1). Non-binding.

  python3 analysis/explore_cpu.py caps      # node caps giving method arms B1's CPU time
  python3 analysis/explore_cpu.py summary   # results/exploratory/cpu/summary.{json,md}
"""
import json
import math
import statistics as st
import sys
from pathlib import Path

D = Path(__file__).resolve().parent.parent / "results/exploratory/cpu"
R = D / "R7"
ARMS = ["B1", "B2", "B3", "V1", "V2"]
METHOD = ["B3", "V1", "V2"]


def load(tag, arm):
    return [json.loads(l) for l in open(R / f"{tag}-{arm}.jsonl")]


def budget_ms():
    """C: median per-thread CPU time of a censored B1 run (1e6 nodes)."""
    return st.median(r["cpu_ms"] for r in load("timing", "B1") if r["status"] == "censored")


def nps(rows):
    return sum(r["nodes"] for r in rows) / sum(r["cpu_ms"] for r in rows) * 1000


def caps():
    c = budget_ms()
    return {a: math.ceil(c / 1000 * nps(load("timing", a)) / 1e5) * 100_000 for a in METHOD}


def km_median_cpu(rows, c):
    t = sorted(r["cpu_ms"] if r["status"] == "solved" and r["cpu_ms"] <= c else math.inf for r in rows)
    m = t[(len(t) - 1) // 2 + (len(t) % 2 == 0)]  # upper median, as in analyze.py
    return None if math.isinf(m) else round(m, 1)


def profile(rows):
    return {
        "n": len(rows),
        "solved": sum(r["status"] == "solved" for r in rows),
        "errors": sum(r["status"] == "error" for r in rows),
        "nodes_per_cpu_s": round(nps(rows)),
        "native_propagation_per_node": round(sum(r["propagation"] for r in rows) / sum(r["nodes"] for r in rows), 1),
        "median_cpu_ms": round(st.median(r["cpu_ms"] for r in rows), 1),
        "cpu_over_wall": round(sum(r["cpu_ms"] for r in rows) / sum(r["wall_ms"] for r in rows), 3),
    }


def summary():
    c = budget_ms()
    out = {"note": "exploratory, non-binding (PREREG_AMENDMENTS.md E1); CPU times are per-thread and machine-dependent",
           "rung": "R7", "cpu_budget_ms": round(c, 1), "timing_1e6_nodes_8_threads": {}, "timing_1e6_nodes_1_thread": {},
           "equal_cpu": {}}
    for a in ARMS:
        out["timing_1e6_nodes_8_threads"][a] = profile(load("timing", a))
    for a in ["B1", "V2"]:
        out["timing_1e6_nodes_1_thread"][a] = profile(load("timing1t", a))
    for a in ["B1"] + METHOD:
        rows = load("timing" if a == "B1" else "equalcpu", a)
        out["equal_cpu"][a] = {
            "node_cap": rows[0]["node_cap"],
            "solved_within_cpu_budget": sum(r["status"] == "solved" and r["cpu_ms"] <= c for r in rows),
            "solved_within_node_cap": sum(r["status"] == "solved" for r in rows),
            "km_median_cpu_ms": km_median_cpu(rows, c),
        }
    (D / "summary.json").write_text(json.dumps(out, indent=2) + "\n")
    t = out["timing_1e6_nodes_8_threads"]
    md = ["| arm | nodes per CPU s | native propagation per node | median CPU ms per 1e6-node run | CPU/wall |", "|---|---|---|---|---|"]
    md += [f"| {a} | {t[a]['nodes_per_cpu_s']:,} | {t[a]['native_propagation_per_node']} | {t[a]['median_cpu_ms']:,.0f} | {t[a]['cpu_over_wall']} |" for a in ARMS]
    md += ["", f"Equal CPU budget C = {c/1000:.1f} s per instance (median CPU time of a censored B1 run).", "",
           "| arm | node cap | solved within C /30 | KM median CPU s |", "|---|---|---|---|"]
    for a, e in out["equal_cpu"].items():
        k = "not reached" if e["km_median_cpu_ms"] is None else f"{e['km_median_cpu_ms']/1000:.1f}"
        md.append(f"| {a} | {e['node_cap']:,} | {e['solved_within_cpu_budget']} | {k} |")
    (D / "summary.md").write_text("\n".join(md) + "\n")
    print("\n".join(md))


if __name__ == "__main__":
    if sys.argv[1:] == ["caps"]:
        for a, n in caps().items():
            print(a, n)
    else:
        summary()
