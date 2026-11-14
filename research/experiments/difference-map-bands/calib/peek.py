"""Summarise calibration rows: per file and n, solved/total, iterations, us/it, censored dynamics.

    python3 -I calib/peek.py calib/cal1
"""
import json
import sys
from pathlib import Path

for f in sorted(Path(sys.argv[1]).glob("*.jsonl")):
    rows = [json.loads(l) for l in f.open() if l.strip()]
    by = {}
    for r in rows:
        by.setdefault(r["n"], []).append(r)
    for n, rs in sorted(by.items()):
        its = sorted(r["iterations"] for r in rs if r["status"] == "solved")
        cen = [r for r in rs if r["status"] != "solved"]
        us = sorted(r["us_per_iter"] for r in rs)[len(rs) // 2]
        print(f"{f.name:16s} n={n} solved {len(its)}/{len(rs)} its {its} us/it {us:.0f} "
              f"cens term_mm {[r['terminal_mismatches'] for r in cen]} best {[r['best_mismatches'] for r in cen]} "
              f"period {[r.get('cycle_period') for r in cen]} distinct {[r.get('distinct_layouts_last2000') for r in cen]}")
