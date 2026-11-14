"""Does the number of edge pieces with the same frame colour on both sides
(B09 of how-the-puzzle-was-made; official 14, painted 10.4 +/- 2.9) predict the
row-scan tree? Eight painted boards of D3 (wt-d3, read only) plus the official
set. Tree totals: Part B boards at N = 1e6 (b0), null boards at N = 1e5 x 5
(a-null, rows-b-lr), official at N = 1e6.
python3 -I 17_tree_vs_same_colour_edges.py
"""
import json, math, os, statistics as st
D3 = "research/experiments/orders-and-pins"
def L(p): return [json.loads(l) for l in open(os.path.join(D3, "results", p))]
def lm(rows): return math.log10(sum(r["total"] for r in rows) / len(rows))
def same(path):
    Ls = [l.split() for l in open(path) if l.strip() and not l.startswith("#")]
    n = int(Ls[0][0]); P = [tuple(int(x) for x in l[:4]) for l in Ls[1:1 + n * n]]
    e = [p for p in P if sum(1 for x in p if x == 0) == 1]
    return sum(1 for p in e if p[(p.index(0) + 1) % 4] == p[(p.index(0) + 3) % 4])
b0 = L("b/b0.jsonl"); nul = L("a/a-null.jsonl")
pts = []
for s in ("300051", "300061", "300097", "300117", "300131"):
    pts.append((s, same(os.path.join(D3, "boards/pins/board-%s.txt" % s)), lm([r for r in b0 if r["label"] == "b0-%s-smc" % s])))
for s in ("200115", "200139", "200181"):
    pts.append((s, same(os.path.join(D3, "boards/null/board-%s.txt" % s)), lm([r for r in nul if r["label"] == "n-%s-rows-b-lr" % s])))
for p in pts: print("board %s  same-colour edge pieces %2d  log10 tree %.3f" % p)
x = [p[1] for p in pts]; y = [p[2] for p in pts]
mx, my = st.mean(x), st.mean(y)
b = sum((a - mx) * (c - my) for a, c in zip(x, y)) / sum((a - mx) ** 2 for a in x)
r = b * st.stdev(x) / st.stdev(y)
print("8 painted boards: slope %.4f dex per piece, r %.3f" % (b, r))
print("prediction for the official set (14 pieces): %.3f ; measured 40.457 ; model 40.495" % (my + b * (14 - mx)))
print("frame-only null (09_frame_null.out): slope -0.0342 dex per piece, r -0.82")
