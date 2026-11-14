"""Leave-one-out worth of each clue (D3 stage bl), official against five
painted boards, with the baseline noise made explicit.
python3 -I 03_clue_worth.py
"""
import json, math, os, statistics as st
D3 = "research/experiments/orders-and-pins/results"
def L(f): return [json.loads(l) for l in open(os.path.join(D3, f))]
def est(rows):
    t = [r["total"] for r in rows]; ls = [math.log10(x) for x in t]
    return math.log10(sum(t) / len(t)), st.stdev(ls) / math.sqrt(len(ls))
bl = L("b/bl.jsonl"); b0 = L("b/b0.jsonl"); off = L("a/a-official.jsonl")
base = {"official_1e6": est([r for r in off if r["label"] == "a-rows-b-lr"]),
        "official_1e5": est([r for r in off if r["label"] == "a-rows-b-lr-np1e5"])}
print("official baselines", {k: (round(v[0], 4), round(v[1], 4)) for k, v in base.items()})
boards = ["300051", "300061", "300097", "300117", "300131"]
for b in boards:
    base[b] = est([r for r in b0 if r["label"] == "b0-%s-smc" % b])
    print("board", b, "b0 smc 1e6: %.4f se %.4f" % base[b])
pos = {135: "(8,7) centre", 34: "(2,2)", 45: "(2,13)", 210: "(13,2)", 221: "(13,13)"}
print("\nclue        official worth vs 1e5 base | vs 1e6 base (se) | boards (vs their 1e6 b0): values, mean, sd | z of official(1e6 base)")
for p, name in pos.items():
    o, ose = est([r for r in bl if r["label"] == "bl-official-drop%d" % p])
    w5 = o - base["official_1e5"][0]; w6 = o - base["official_1e6"][0]
    se6 = math.hypot(ose, base["official_1e6"][1])
    bw = []
    for b in boards:
        e, _ = est([r for r in bl if r["label"] == "bl-%s-drop%d" % (b, p)])
        bw.append(e - base[b][0])
    z = (w6 - st.mean(bw)) / math.hypot(st.stdev(bw), se6)
    print("%-13s %.3f | %.3f (%.3f) | %s mean %.3f sd %.3f | z %+.2f" % (name, w5, w6, se6, " ".join("%.3f" % x for x in bw), st.mean(bw), st.stdev(bw), z))
