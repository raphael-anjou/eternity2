"""Summarise results/calib/calib.jsonl: per configuration, log10 of the mean
total, the 95% t interval, the sd of the per-repetition log10 totals, and the
95% half-width (log10) projected for 5 repetitions. Stdlib only."""
import json
import math
import os
import statistics as st
from collections import defaultdict

EXP = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
T95 = {1: 12.706, 2: 4.303, 3: 3.182, 4: 2.776, 5: 2.571, 6: 2.447, 7: 2.365, 8: 2.306, 9: 2.262, 19: 2.093}


def interval(tots):
    n = len(tots)
    m = st.mean(tots)
    se = st.stdev(tots) / math.sqrt(n)
    h = T95.get(n - 1, 1.96) * se
    lo = math.log10(m - h) if m - h > 0 else float("-inf")
    return math.log10(m), lo, math.log10(m + h), se / m


def main():
    rows = [json.loads(l) for l in open(os.path.join(EXP, "results", "calib", "calib.jsonl")) if l.strip()]
    g = defaultdict(list)
    for r in rows:
        if "total" in r and "np" in r:
            g[(r["label"], r["np"])].append(r)
    for (lab, np_), rs in sorted(g.items()):
        tots = [r["total"] for r in rs]
        if len(tots) < 2:
            continue
        m, lo, hi, rel = interval(tots)
        sdl = st.stdev(math.log10(t) for t in tots)
        proj5 = math.log10(1 + T95[4] * rel * math.sqrt(len(tots) / 5))
        secs = st.mean(r["seconds"] for r in rs)
        print(f"{lab:28s} N={np_:<8d} reps={len(tots):2d} log10T={m:7.3f} [{lo:7.3f},{hi:7.3f}] "
              f"sd_log10_rep={sdl:.3f} proj_halfwidth_5reps={proj5:.3f} s/rep={secs:.1f}")
    for r in rows:
        if "complete" in r:
            print(r["label"], "complete", r["complete"], "total", r["total"], "solutions", r["solutions"],
                  "first", r["nodes_at_first_solution"], "s", r["seconds"])


if __name__ == "__main__":
    main()
