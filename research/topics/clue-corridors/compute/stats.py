#!/usr/bin/env python3
"""Aggregate the per-size ladder JSONs into the committed results file.

    python3 stats.py <ladder_8.json> ... <ladder_16.json> > ../results/ladder_ab.json

Pairs each corridor arm against control by seed and reports, per (arm, size):
the per-seed scores, the mean paired delta, and the paired statistics. The
Wilcoxon signed-rank z and paired t come from the starter kit's
scripts/compare.py (imported, not re-implemented). No wall-clock fields are
carried; the tier is seeded-statistical.

Pure standard library.
"""
import json
import sys
from pathlib import Path

KIT_SCRIPTS = Path(__file__).resolve().parents[3] / "starter-kit" / "scripts"
sys.path.insert(0, str(KIT_SCRIPTS))
from compare import mean, paired_t, sd, wilcoxon_z  # noqa: E402


def main() -> None:
    rungs = []
    for path in sorted(sys.argv[1:], key=lambda p: json.load(open(p))["size"]):
        d = json.load(open(path))
        control = {r["seed"]: r["score"] for r in d["arms"]["control"]["per_seed"]}
        rung = {
            "size": d["size"],
            "colors": d["colors"],
            "clue_cells": d["clue_cells"],
            "max_score": d["max_score"],
            "budget_s_per_run": d["budget_s_per_run"],
            "paired_seeds": d["paired_seeds"],
            "control_mean": round(mean(list(control.values())), 2),
            "arms": {},
        }
        for arm in ("control", "path1", "ribbon2"):
            per_seed = {r["seed"]: r["score"] for r in d["arms"][arm]["per_seed"]}
            entry = {
                "scores_by_seed": {str(s): per_seed[s] for s in sorted(per_seed)},
                "mean": round(mean(list(per_seed.values())), 2),
                "board_url_seed1": d["arms"][arm]["board_url_seed1"],
            }
            if arm != "control":
                deltas = [per_seed[s] - control[s] for s in sorted(control)]
                entry["paired_delta_mean"] = round(mean(deltas), 2)
                entry["paired_delta_sd"] = round(sd(deltas), 2)
                entry["wins_vs_control"] = sum(1 for x in deltas if x > 0)
                entry["losses_vs_control"] = sum(1 for x in deltas if x < 0)
                t = paired_t(deltas)
                z = wilcoxon_z(deltas)
                entry["paired_t"] = round(t, 2) if t is not None else None
                entry["wilcoxon_z"] = round(z, 2) if z is not None else None
            rung["arms"][arm] = entry
        p1 = rung["arms"]["path1"]["paired_delta_mean"]
        r2 = rung["arms"]["ribbon2"]["paired_delta_mean"]
        rung["ribbon2_over_path1"] = round(r2 / p1, 2) if p1 else None
        rungs.append(rung)
    print(json.dumps({"experiment": "corridor-vs-control ladder", "rungs": rungs},
                     indent=1, sort_keys=True))


if __name__ == "__main__":
    main()
