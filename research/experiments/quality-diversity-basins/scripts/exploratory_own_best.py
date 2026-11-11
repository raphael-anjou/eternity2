#!/usr/bin/env python3
"""Exploratory, NOT pre-registered: distinct basins counted against each run's
own best score instead of the pooled best_known, to separate "fewer basins"
from "lower best score". Same leader clustering and C4 distance as the engine.
"""
import glob
import json
import os
import statistics
import sys
from collections import defaultdict

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from analyse import wilcoxon_exact  # noqa: E402


def rot_cw(cells, n):
    # cell = piece * 4 + rotation; rotating the board 90 degrees clockwise moves
    # (r, c) to (c, n-1-r) and adds one quarter turn to every piece.
    out = [0] * (n * n)
    for r in range(n):
        for c in range(n):
            v = cells[r * n + c]
            out[c * n + (n - 1 - r)] = (v // 4) * 4 + (v % 4 + 1) % 4
    return out


def rots(cells, n):
    a = [list(cells)]
    for _ in range(3):
        a.append(rot_cw(a[-1], n))
    return a


def dist(a, lead):
    m = len(a)
    return min(sum(1 for x, y in zip(a, b) if x != y) for b in lead) / m


def basins(cands, n, delta):
    order = sorted(cands, key=lambda t: (-t[0], t[1]))
    leaders = []
    for _, c in order:
        if not any(dist(c, l) < delta for l in leaders):
            leaders.append(rots(c, n))
    return len(leaders)


def main():
    runs_dir, k, delta = sys.argv[1], int(sys.argv[2]), float(sys.argv[3])
    per = defaultdict(lambda: {"pt": [], "me": []})
    for f in sorted(glob.glob(os.path.join(runs_dir, "*.json"))):
        r = json.load(open(f))
        n = r["size"]
        sel = [(c["s"], tuple(c["c"])) for c in r["candidates"] if c["s"] + k >= r["best_score"]]
        per[(n, r["puzzle_seed"])][r["method"]].append(basins(sel, n, delta))
    out = {}
    for n in sorted({key[0] for key in per}):
        d = [statistics.mean(v["me"]) - statistics.mean(v["pt"]) for key, v in per.items() if key[0] == n]
        pt = statistics.mean(statistics.mean(v["pt"]) for key, v in per.items() if key[0] == n)
        me = statistics.mean(statistics.mean(v["me"]) for key, v in per.items() if key[0] == n)
        w = wilcoxon_exact(d, "greater")
        out[str(n)] = {"k": k, "delta": delta, "mean_pt": pt, "mean_me": me,
                       "n_me_higher": sum(x > 0 for x in d), "n_pt_higher": sum(x < 0 for x in d),
                       "p_greater": w["p"]}
    print(json.dumps(out, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
