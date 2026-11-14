"""Level profile of the official set (Track 10, commit dc71ad56) against
(a) the first-moment model (wt-d3 scripts/model.py, bulge port) and
(b) 20 painted boards with the official colour census (paint-np1e5).
Per level: width difference, and per-level branching difference (first
difference of log10 width, which removes the level-to-level correlation).
python3 -I 01_profile_vs_model_vs_null.py
"""
import json, math, os, sys, statistics as st
HERE = os.path.dirname(os.path.abspath(__file__))
D3 = "research/experiments/orders-and-pins/scripts"
sys.path.insert(0, D3)
from model import Model
import orders as O

def load(f):
    return [json.loads(l) for l in open(os.path.join(HERE, "data", f))]

def meanlog(rows, d):
    # mean of estimates in linear space, then log10 (Track 10's convention)
    v = [10 ** r["log10_width"][d] for r in rows if d < len(r["log10_width"]) and d <= r["extinction_depth"] - 1]
    return math.log10(sum(v) / len(v)) if v else None

real6 = load("five-np1e6.jsonl"); real5 = load("five-np1e5.jsonl")
paint = load("paint-np1e5.jsonl")
inst = {}
for r in paint:
    inst.setdefault(r["label"], []).append(r)
print("painted instances", len(inst), "reps each", sorted(set(len(v) for v in inst.values())))
M = Model(16, O.load_official())
prof = M.profile(O.rows(16), O.CLUES16)   # index i = width after free cell i+1
print("model total log10", round(M.total(O.rows(16), O.CLUES16), 4), "levels", len(prof))
print("measured total log10 (1e6)", round(math.log10(sum(r["total"] for r in real6) / 5), 4))
tot_null = sorted(math.log10(sum(r["total"] for r in v) / len(v)) for v in inst.values())
print("null totals: mean %.4f sd %.4f se %.4f  min %.3f max %.3f" % (st.mean(tot_null), st.stdev(tot_null), st.stdev(tot_null) / math.sqrt(len(tot_null)), tot_null[0], tot_null[-1]))
print("null mean minus model: %.4f dex = %.2f se" % (st.mean(tot_null) - M.total(O.rows(16), O.CLUES16), (st.mean(tot_null) - M.total(O.rows(16), O.CLUES16)) / (st.stdev(tot_null) / math.sqrt(20))))
rt = math.log10(sum(r["total"] for r in real6) / 5)
print("official z against null totals: %.2f ; rank %d of 20 below" % ((rt - st.mean(tot_null)) / st.stdev(tot_null), sum(1 for x in tot_null if x < rt)))

print("\ndepth  model   real6   real-model  seReal  nullMean nullSd  zReal(null)  null-model")
rows_out = []
for d in range(1, 186):
    m = prof[d - 1]
    reps = [r["log10_width"][d] for r in real6]
    r6 = meanlog(real6, d)
    se = st.stdev(reps) / math.sqrt(len(reps))
    nv = [meanlog(v, d) for v in inst.values()]
    nv = [x for x in nv if x is not None]
    nm, ns = st.mean(nv), st.stdev(nv)
    z = (r6 - nm) / ns if ns > 0 else 0.0
    rows_out.append((d, m, r6, r6 - m, se, nm, ns, z, nm - m))
    if d % 8 == 0 or d in (158,):
        print("%4d  %7.3f %7.3f  %+7.3f   %.3f  %7.3f  %.3f   %+5.2f      %+6.3f" % rows_out[-1])
zs = [r[7] for r in rows_out]
print("\nmax |z| of official width against the 20 painted boards over depths 1..185: %.2f at depth %d" % (max(abs(z) for z in zs), rows_out[max(range(len(zs)), key=lambda i: abs(zs[i]))][0]))
print("depths with |z| > 2:", [r[0] for r in rows_out if abs(r[7]) > 2])
dm = [r[3] for r in rows_out]
print("real minus model: max %.3f at %d, min %.3f at %d" % (max(dm), rows_out[dm.index(max(dm))][0], min(dm), rows_out[dm.index(min(dm))][0]))
# per-row-of-16 increments: real minus model, and against the null
print("\nper 16-cell block: increment of (real - model), increment of (null mean - model), z of real increment against null increments")
def W(rows, d): return meanlog(rows, d)
for a in range(0, 176, 16):
    b = a + 16
    a0 = max(a, 1)
    ri = (W(real6, b) - prof[b - 1]) - (W(real6, a0) - prof[a0 - 1])
    ni = [(W(v, b) - W(v, a0)) for v in inst.values()]
    mi = prof[b - 1] - prof[a0 - 1]
    rr = W(real6, b) - W(real6, a0)
    print("  %3d..%3d  real-model %+6.3f   null-model %+6.3f (sd %.3f)   z %+5.2f" % (a0, b, ri, st.mean(ni) - mi, st.stdev(ni), (rr - st.mean(ni)) / st.stdev(ni)))
json.dump(rows_out, open(os.path.join(HERE, "01_profile.json"), "w"))
