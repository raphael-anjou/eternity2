"""gap 4 toy ladder: paired gains in bits, per board size. Run: python3 -I analyze.py ladder.jsonl"""
import sys, json
from math import log2
from collections import defaultdict
rows = [json.loads(l) for l in open(sys.argv[1]) if l.strip()]
by = defaultdict(dict)
for r in rows: by[(r["n"], r["cf"], r["ci"], r["seed"])][r["variant"]] = r
groups = defaultdict(list)
for k, v in by.items(): groups[k[:3]].append(v)
for g in sorted(groups):
    bs = groups[g]; print("\n== %dx%d, %d frame + %d interior colours, %d toy boards ==" % (g[0], g[0], g[1], g[2], len(bs)))
    names = []
    for b in bs:
        for nme in b:
            if nme not in names: names.append(nme)
    print("%-10s %5s %9s | bits gained over ROW-PLAIN: mean [min, max] | over ROW-AC: mean [min, max] | censored" % ("variant", "n", "log2 mean"))
    for nme in names:
        xs = [b[nme] for b in bs if nme in b]
        l = [log2(x["tried"]) for x in xs]
        def gain(ref):
            d = [log2(b[ref]["tried"]) - log2(b[nme]["tried"]) for b in bs if nme in b and ref in b]
            return "%6.2f [%6.2f, %6.2f]" % (sum(d) / len(d), min(d), max(d)) if d else "   n/a"
        print("%-10s %5d %9.2f | %s | %s | %d" % (nme, len(xs), sum(l) / len(l), gain("ROW-PLAIN"), gain("ROW-AC"), sum(x["censored"] for x in xs)))
    sols = {tuple(sorted({x["sols"] for x in b.values() if not x["censored"]})) for b in bs}
    print("solution counts agree across uncensored variants on every board:", all(len(s) == 1 for s in sols))
