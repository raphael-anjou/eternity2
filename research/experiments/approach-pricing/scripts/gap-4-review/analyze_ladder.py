"""Toy ladder summary (GENERATED boards). Run: python3 -I analyze_ladder.py exact.jsonl smc.jsonl
Paired gain in bits of each variant over ROW under the same propagation (positive = smaller tree), per board, then
mean, standard deviation over boards, range, and the number of boards where the variant wins. Measure: branches tried.
For the population estimates a board's value is the log2 of the mean estimate over its repetitions."""
import sys, json
from math import log2, sqrt
from collections import defaultdict
ex = defaultdict(dict); sm = defaultdict(lambda: defaultdict(list))
for f in sys.argv[1:]:
    for l in open(f):
        if not l.strip(): continue
        r = json.loads(l); k = (r["n"], r["board"]); v = r["prop"] + "-" + r["rule"]
        if r["mode"] == "exact": ex[k][v] = r
        else: sm[k][v].append(r)
def stats(d):
    m = sum(d) / len(d); sd = sqrt(sum((x - m) ** 2 for x in d) / (len(d) - 1)) if len(d) > 1 else float("nan")
    return "%+6.2f sd %4.2f [%+6.2f,%+6.2f] wins %2d/%-2d" % (m, sd, min(d), max(d), sum(x > 0 for x in d), len(d))
def table(title, boards, val):
    groups = defaultdict(list)
    for (n, b) in boards: groups[(n, ":".join(b.split(":")[2:4]))].append((n, b))
    for gk in sorted(groups):
        bs = groups[gk]; names = []
        for k in bs:
            for v in boards[k]:
                if v not in names: names.append(v)
        print("\n== %s: %dx%d, frame:interior colours %s, %d boards ==" % (title, gk[0], gk[0], gk[1], len(bs)))
        for v in names:
            p = v.split("-")[0]; ref = p + "-ROW"
            xs = [val(boards[k][v]) for k in bs if v in boards[k] and val(boards[k][v]) is not None]
            d = [val(boards[k][ref]) - val(boards[k][v]) for k in bs if v in boards[k] and ref in boards[k] and val(boards[k][v]) is not None and val(boards[k][ref]) is not None]
            line = "%-11s n=%2d mean log2 tried %6.2f" % (v, len(xs), sum(xs) / len(xs) if xs else float("nan"))
            if d and v != ref: line += " | over %s: %s" % (ref, stats(d))
            if v in ("FC-ROW", "AC-ROW"):
                d2 = [val(boards[k]["PLAIN-ROW"]) - val(boards[k][v]) for k in bs if v in boards[k] and "PLAIN-ROW" in boards[k]]
                if d2: line += " | over PLAIN-ROW: %s" % stats(d2)
            print(line)
        yield gk, bs
def exval(r): return None if r["censored"] else log2(r["tried"])
def smval(rs): return log2(sum(2.0 ** r["log2_tried"] for r in rs) / len(rs))
for gk, bs in table("EXACT", ex, exval):
    cens = [(b, v) for (n, b) in bs for v, r in ex[(n, b)].items() if r["censored"]]
    sols = sorted({r["sols"] for (n, b) in bs for r in ex[(n, b)].values() if not r["censored"]})
    agree = all(len({r["sols"] for r in ex[k].values() if not r["censored"]}) == 1 for k in bs)
    print("   solution counts seen %s ; agree across variants on every board: %s ; censored runs: %s" % (sols, agree, cens))
    # estimator check where both exist
    chk = [smval(sm[k][v]) - exval(ex[k][v]) for k in bs if k in sm for v in ex[k] if v in sm[k] and exval(ex[k][v]) is not None]
    if chk: print("   population estimate minus exact, log2 tried, %d (board, variant) pairs: %s" % (len(chk), stats(chk)))
for gk, bs in table("POPULATION ESTIMATE", sm, smval): pass
