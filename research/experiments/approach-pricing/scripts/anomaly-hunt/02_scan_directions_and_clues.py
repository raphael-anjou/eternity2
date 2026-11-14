"""D3 Part A raw rows (wt-d3, read only): measured tree minus first-moment
model for the eight line scans, official set against the three null boards.
Then the leave-one-out worth of each clue, official against five painted boards.
python3 -I 02_scan_directions_and_clues.py
"""
import json, math, os, sys, statistics as st, collections
D3 = "research/experiments/orders-and-pins"
sys.path.insert(0, D3 + "/scripts")
from model import Model
import orders as O
M = Model(16, O.load_official())
named = O.named(16)
def L(f): return [json.loads(l) for l in open(os.path.join(D3, "results", f))]
off = L("a/a-official.jsonl"); nul = L("a/a-null.jsonl")
scans = ["rows-b-lr", "rows-b-rl", "snake-rows-b", "cols-l-bt", "cols-l-tb", "snake-cols-l", "cols-r-bt", "cols-r-tb", "rows-t-lr", "rows-t-rl"]
def est(rows):
    t = [r["total"] for r in rows]
    lm = math.log10(sum(t) / len(t))
    ls = [math.log10(x) for x in t]
    return lm, (st.stdev(ls) / math.sqrt(len(ls)) if len(ls) > 1 else float("nan")), len(t)
print("order          model   official(1e6) se     off-model | null boards (N=1e5): est-model each")
allnull = []; alloff = []
for s in scans:
    m = M.total(named[s], O.CLUES16)
    o, se, n = est([r for r in off if r["label"] == "a-" + s])
    line = "%-13s %7.3f  %7.3f  %.3f   %+6.3f  |" % (s, m, o, se, o - m)
    alloff.append((s, o - m))
    for b in ("200115", "200139", "200181"):
        rr = [r for r in nul if r["label"] == "n-%s-%s" % (b, s)]
        if rr:
            e, se2, _ = est(rr)
            line += "  %+6.3f(%.3f)" % (e - m, se2)
            allnull.append((b, s, e - m))
    print(line)
# group into the four independent sides (rows-b, cols-l, cols-r, rows-t)
side = lambda s: s.replace("snake-", "").rsplit("-", 1)[0] if not s.startswith("snake") else {"snake-rows-b": "rows-b", "snake-cols-l": "cols-l"}[s]
print("\nper side (mean over the scans that start from that side):")
bo = collections.defaultdict(list)
for s, d in alloff: bo[side(s)].append(d)
bn = collections.defaultdict(list)
for b, s, d in allnull: bn[(b, side(s))].append(d)
offs = {k: st.mean(v) for k, v in bo.items()}
print(" official:", {k: round(v, 3) for k, v in offs.items()}, "range %.3f" % (max(offs.values()) - min(offs.values())))
for b in ("200115", "200139", "200181"):
    d = {k[1]: st.mean(v) for k, v in bn.items() if k[0] == b}
    print(" null", b, {k: round(v, 3) for k, v in d.items()}, "range %.3f" % (max(d.values()) - min(d.values())))
nv = [st.mean(v) for v in bn.values()]
print(" null side values: n=%d mean %+.3f sd %.3f" % (len(nv), st.mean(nv), st.stdev(nv)))
for k, v in offs.items():
    print("   official %-7s %+.3f  z %+.2f" % (k, v, (v - st.mean(nv)) / st.stdev(nv)))

print("\nClue leave-one-out (results/b or results): files present:")
import glob
for f in glob.glob(D3 + "/results/**/bl.jsonl", recursive=True) + glob.glob(D3 + "/results/**/b0.jsonl", recursive=True):
    rows = [json.loads(l) for l in open(f)]
    print(f.replace(D3, ""), len(rows), {k: (v if not isinstance(v, (list, dict)) else type(v).__name__) for k, v in rows[0].items()})
    print(collections.Counter(r.get("label") for r in rows).most_common(60))
