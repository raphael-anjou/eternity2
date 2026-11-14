"""Summary of the population estimates on the official 16x16 pieces. Run: python3 -I analyze16.py real16.jsonl
Per (board orientation, propagation, rule): log2 of the mean estimate over repetitions (the estimate of a count is unbiased,
its log is not), the range of the per-repetition log2 values, and the gain in bits over ROW under the same propagation."""
import sys, json
from math import log2
from collections import defaultdict
g = defaultdict(list)
for f in sys.argv[1:]:
    for l in open(f):
        if l.strip():
            r = json.loads(l); g[(r["board"].split(":")[0], r["prop"], r["rule"])].append(r)
def agg(rs, key):
    xs = [r[key] for r in rs]; m = log2(sum(2.0 ** (x - 100) for x in xs) / len(xs)) + 100
    return m, min(xs), max(xs)
print("%-4s %-5s %-5s %3s %5s | %-26s | %-26s | %s" % ("brd", "prop", "rule", "rep", "M", "log2 tried: mean [min,max]", "log2 alive: mean [min,max]", "gain over ROW same prop (tried, alive) ; peak level ; cens"))
for b in ("e2r", "e2"):
    for p in ("PLAIN", "FC", "AC"):
        base = g.get((b, p, "ROW"))
        for (bb, pp, rule), rs in g.items():
            if bb != b or pp != p: continue
            t = agg(rs, "log2_tried"); a = agg(rs, "log2_alive")
            gt = ga = float("nan")
            if base: gt = agg(base, "log2_tried")[0] - t[0]; ga = agg(base, "log2_alive")[0] - a[0]
            print("%-4s %-5s %-5s %3d %5d | %7.2f [%7.2f,%7.2f] | %7.2f [%7.2f,%7.2f] | %+6.2f %+6.2f ; %s ; %d" % (b, p, rule, len(rs), rs[0]["M"], t[0], t[1], t[2], a[0], a[1], a[2], gt, ga, sorted(r["peak_level"] for r in rs), sum(r["censored"] != 0 for r in rs)))
    pl = g.get((b, "PLAIN", "ROW"))
    if pl:
        for p in ("FC", "AC"):
            if (b, p, "ROW") in g: print("   %s: ROW-%s over ROW-PLAIN: %+.2f bits (tried)  %+.2f bits (alive nodes against valid fillings)" % (b, p, agg(pl, "log2_tried")[0] - agg(g[(b, p, "ROW")], "log2_tried")[0], agg(pl, "log2_alive")[0] - agg(g[(b, p, "ROW")], "log2_alive")[0]))
