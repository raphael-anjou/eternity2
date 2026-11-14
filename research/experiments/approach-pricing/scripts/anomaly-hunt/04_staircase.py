"""Gap 4 re-check's raw rows (gap-4-review, read only): the three-row
staircase Y3 under forward checking on the official pieces. Is the +2.85 bits
an estimator stopping early (the D3 independent assessor's F1 mechanism)? Look at the number
of levels each run reached, its peak level, and the spread.
python3 -I 04_staircase.py
"""
import json, math, os, statistics as st, collections
G = "scripts/gap-4-review"
def L(f): return [json.loads(l) for l in open(os.path.join(G, f)) if l.strip()]
def lmean(v):  # log2 of the mean of 2^v
    m = max(v); return m + math.log2(sum(2 ** (x - m) for x in v) / len(v))
for f in ("real16_fc.jsonl", "real16.jsonl", "real16_ac.jsonl", "toy16.jsonl"):
    rows = L(f)
    g = collections.defaultdict(list)
    for r in rows: g[(r["board"], r["prop"], r["rule"], r["M"])].append(r)
    print("\n", f)
    for k, v in g.items():
        t = [r["log2_tried"] for r in v]
        print("  %-22s %-5s %-5s M=%-6d n=%2d  log2 mean %.2f  median %.2f  min %.2f max %.2f sd %.2f | levels %s | peak_level %s | log2_peak %s" % (
            k[0], k[1], k[2], k[3], len(v), lmean(t), st.median(t), min(t), max(t), st.stdev(t) if len(t) > 1 else 0,
            sorted(r["levels"] for r in v), sorted(set(r["peak_level"] for r in v)), [round(r["log2_peak"], 1) for r in v][:6]))
