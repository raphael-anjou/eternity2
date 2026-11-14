"""Estimator check. Run: python3 -I analyze_valid.py valid8.jsonl
Exact tree against the population estimate with a deliberately small population (M = 100, 30 repetitions)."""
import sys, json
from math import log2
from collections import defaultdict
ex = {}; sm = defaultdict(list)
for l in open(sys.argv[1]):
    r = json.loads(l); k = (r["board"], r["prop"], r["rule"])
    if r["mode"] == "exact": ex[k] = log2(r["tried"]) if not r["censored"] else None
    else: sm[k].append(r["log2_tried"])
print("board, rule: exact log2 tried | estimate: log2 of mean, median, [min, max] over reps | log2(mean) - exact | share of reps below exact")
for k in sorted(sm):
    x = sorted(sm[k]); m = log2(sum(2.0 ** v for v in x) / len(x)); e = ex.get(k)
    print("%-16s %-5s %s | %6.2f %6.2f [%6.2f, %6.2f] | %s | %s" % (k[0], k[1] + "-" + k[2], "%6.2f" % e if e else "  n/a", m, x[len(x) // 2], x[0], x[-1], "%+.2f" % (m - e) if e else "n/a", "%.2f" % (sum(v < e for v in x) / len(x)) if e else "n/a"))
