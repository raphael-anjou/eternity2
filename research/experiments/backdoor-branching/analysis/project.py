#!/usr/bin/env python3
"""PREREG.md sample-size rule: projected test wall clock from calibration.

projection = sum over (bb solver, rung) of 3 x calibration total wall / 8.
Prints the projection and the seed range it implies; writes
results/calibration/projection.json.
"""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CAL = ROOT / "results" / "calibration"
rows = {}
total = 0.0
for f in sorted(CAL.glob("*.jsonl")):
    for line in f.read_text().splitlines():
        r = json.loads(line)
        key = f"{r['solver']} n{r['n']}"
        rows.setdefault(key, {"runs": 0, "solved": 0, "wall_s": 0.0})
        rows[key]["runs"] += 1
        rows[key]["solved"] += r["status"] == "solved"
        rows[key]["wall_s"] += r["wall_ms"] / 1000
for v in rows.values():
    v["projected_test_s"] = 3 * v["wall_s"] / 8
    total += v["projected_test_s"]
seeds = "1..30" if total <= 50 * 60 else "1..20"
out = {"cells": rows, "projected_test_min": total / 60, "threshold_min": 50, "test_seeds": seeds}
(CAL / "projection.json").write_text(json.dumps(out, indent=1) + "\n")
for k, v in sorted(rows.items()):
    print(f"{k:20s} solved {v['solved']}/{v['runs']}  wall {v['wall_s']:.0f} s  projected {v['projected_test_s'] / 60:.1f} min")
print(f"projected test wall {total / 60:.1f} min -> seeds {seeds}")
