"""TOY. Many short starts against one long one: from the 10 measured starts per board, the time to
the FIRST verified solution when k starts run in parallel on k cores (all subsets of size k of the
10 starts, exact average; failures count as their cap). Also the median and p90 of the single-start
times. Seconds are the runs' own CPU seconds, so parallel runs are assumed not to slow each other.
Usage: python3 -I py/portfolio.py results/<step>.jsonl [k ...]"""
import json, sys
from itertools import combinations
from statistics import median

rows = [json.loads(l) for l in open(sys.argv[1])]
ks = [int(a) for a in sys.argv[2:]] or [1, 3, 8]
by = {}
for r in rows:
    if "broken" not in r["board"]:
        by.setdefault(r["board"], []).append(r["t_solution"] if r["solved"] else r["seconds"])
print("| board | " + " | ".join(f"mean first solution, {k} parallel starts (s)" for k in ks) + " |")
print("|---|" + "---|" * len(ks))
for b in sorted(by):
    t = by[b]
    cells = []
    for k in ks:
        mins = [min(c) for c in combinations(t, k)]
        cells.append(f"{sum(mins) / len(mins):.1f}")
    print(f"| {b} | " + " | ".join(cells) + " |")
