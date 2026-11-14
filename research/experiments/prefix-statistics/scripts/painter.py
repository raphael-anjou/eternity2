#!/usr/bin/env python3
"""A4 change 8, painter control reading. Stdlib only.

    python3 -I scripts/painter.py results/painter/painter.jsonl

Per board and depth, exact delta = (value - exact node-uniform mean) / exact
sd for the planted solution's prefix and, averaged, for the prefixes of the
other solutions (rotations of the planted board excluded). Pass (no painter
signature): at each depth and statistic, the mean over boards of the planted
delta lies inside the 95% bootstrap interval of the mean over boards of the
non-planted delta.
"""
import json
import random
import sys

NAMES = ["s4_balance_chi2", "s6_first_moment", "s9_colour_entropy"]
rows = [json.loads(l) for l in open(sys.argv[1]) if l.strip()]
rows = [r for r in rows if "skipped" not in r and r["others"] >= 1]
print(f"boards with at least one non-planted solution: {len(rows)}; solutions per board median "
      f"{sorted(r['solutions'] for r in rows)[len(rows) // 2] if rows else 'n/a'}")
ok_all = True
depths = [d["depth"] for d in rows[0]["depths"]] if rows else []
rng = random.Random(20261113)
for di, k in enumerate(depths):
    for s, nm in enumerate(NAMES):
        pl, ot = [], []
        for r in rows:
            d = r["depths"][di]
            sd = d["sd"][s]
            if not sd or sd <= 0 or d["others_mean"][s] is None:
                continue
            pl.append((d["planted"][s] - d["mean"][s]) / sd)
            ot.append((d["others_mean"][s] - d["mean"][s]) / sd)
        if len(ot) < 3:
            continue
        n = len(ot)
        bs = sorted(sum(ot[rng.randrange(n)] for _ in range(n)) / n for _ in range(10000))
        lo, hi = bs[250], bs[9749]
        mp, mo = sum(pl) / n, sum(ot) / n
        ok = lo <= mp <= hi
        ok_all &= ok
        diff = [a - b for a, b in zip(pl, ot)]
        bd = sorted(sum(diff[rng.randrange(n)] for _ in range(n)) / n for _ in range(10000))
        print(f"  depth {k:2d} {nm:18s} planted {mp:+.3f}  others {mo:+.3f} [{lo:+.3f}, {hi:+.3f}]  "
              f"{'pass' if ok else 'FAIL'}   paired diff {sum(diff) / n:+.3f} [{bd[250]:+.3f}, {bd[9749]:+.3f}]  boards {n}")
print("painter control: " + ("PASS (no painter signature)" if ok_all else "FAIL (planted prefixes differ from other solutions' prefixes)"))
