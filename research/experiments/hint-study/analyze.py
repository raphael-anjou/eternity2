#!/usr/bin/env python3
"""Summarise the hint-geometry study: does WHERE you pin beat HOW MANY?

Reads results.jsonl (one row per variant x seed: score at a fixed producer beam)
and prints two tables:

  1. GEOMETRY at matched count (16 hints): scattered vs contiguous vs interior
     vs border. This is the page's core claim.
  2. COUNT sweep at fixed (scattered) geometry: 0..24 hints. This is the
     commenter's 18 -> 15 -> ... experiment.

Metric is final matched-edge score (max 480) at a fixed beam. Higher = the
solver got closer with that hint set. We report median and the [min, max] spread
across seeds, because a single seed is noise.
"""
import json
import statistics as st
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).parent
rows = [json.loads(l) for l in (HERE / "results.jsonl").read_text().splitlines() if l.strip()]

by_variant = defaultdict(list)
for r in rows:
    by_variant[r["variant"]].append(r)


def summ(name):
    rs = by_variant.get(name)
    if not rs:
        return None
    scores = [r["score"] for r in rs]
    secs = [r["secs"] for r in rs]
    return {
        "n": len(scores),
        "nhints": rs[0]["nhints"],
        "median": st.median(scores),
        "mean": st.mean(scores),
        "min": min(scores),
        "max": max(scores),
        "solved": sum(1 for s in scores if s >= 480),
        "med_secs": st.median(secs),
    }


def line(label, s):
    if s is None:
        print(f"  {label:<26} (no data)")
        return
    print(
        f"  {label:<26} hints={s['nhints']:>2}  "
        f"median={s['median']:>3.0f}  mean={s['mean']:>5.1f}  "
        f"[{s['min']:>3}-{s['max']:>3}]  solved={s['solved']}/{s['n']}  "
        f"~{s['med_secs']:.1f}s"
    )


beam = rows[0]["beam"] if rows else "?"
print(f"\n=== HINT-GEOMETRY STUDY (producer, beam={beam}, single-core) ===")
print("metric = final matched-edge score /480 at fixed beam; higher is better\n")

print("[1] GEOMETRY at matched count (16 hints) -- the page's claim:")
for v in ["scattered_lattice_16", "contiguous_16", "interior_only_16", "border_only_16"]:
    line(v, summ(v))

print("\n[2] COUNT sweep at fixed scattered geometry -- the commenter's test:")
for n in [0, 3, 6, 9, 12, 15, 18, 21, 24]:
    line(f"sweep_scattered_{n:02d}", summ(f"sweep_scattered_{n:02d}"))

print("\n[3] The page's own contradiction at matched 18 hints:")
print("    prose ('dotted deep, reaches the endgame') vs figure ('top rows only')")
for v in ["article_prose_18", "article_figure_18"]:
    line(v, summ(v))
pp, ff = summ("article_prose_18"), summ("article_figure_18")
if pp and ff:
    d = pp["median"] - ff["median"]
    which = "DEEP (prose) beats shallow (figure)" if d > 0 else \
            "SHALLOW (figure) beats deep (prose)" if d < 0 else "tie"
    print(f"    -> prose - figure median = {d:+.0f}  -> {which}")

# Verdict helpers
g = {v: summ(v) for v in ["scattered_lattice_16", "contiguous_16"]}
if all(g.values()):
    d = g["scattered_lattice_16"]["median"] - g["contiguous_16"]["median"]
    verdict = (
        "scattered BEATS contiguous" if d > 0
        else "contiguous BEATS scattered" if d < 0
        else "tie"
    )
    print(f"\n[verdict] scattered - contiguous median = {d:+.0f}  -> {verdict}")

sweep = [(n, summ(f"sweep_scattered_{n:02d}")) for n in [0, 6, 12, 18, 24]]
sweep = [(n, s["median"]) for n, s in sweep if s]
if len(sweep) >= 2:
    lo_n, lo = sweep[0]
    hi_n, hi = sweep[-1]
    print(f"[verdict] count {lo_n}->{hi_n} hints moved median score {lo:.0f}->{hi:.0f} ({hi-lo:+.0f})")
print()
