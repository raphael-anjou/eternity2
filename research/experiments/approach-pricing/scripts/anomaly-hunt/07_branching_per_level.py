"""Per-level branching (difference of consecutive log10 widths) of the official
set against the 20 painted boards of Track 10. Level-to-level differences
remove the cumulative correlation of the widths, so this is about 180 nearly
separate comparisons. Also the model's branching.
python3 -I 07_branching_per_level.py
"""
import json, math, os, sys, statistics as st
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, "research/experiments/orders-and-pins/scripts")
from model import Model
import orders as O
def load(f): return [json.loads(l) for l in open(os.path.join(HERE, "data", f))]
def ml(rows, d):
    v = [10 ** r["log10_width"][d] for r in rows]
    return math.log10(sum(v) / len(v))
real = load("five-np1e6.jsonl"); real5 = load("five-np1e5.jsonl"); paint = load("paint-np1e5.jsonl")
inst = {}
for r in paint: inst.setdefault(r["label"], []).append(r)
prof = [0.0] + Model(16, O.load_official()).profile(O.rows(16), O.CLUES16)
out = []
for d in range(1, 180):
    rb = ml(real, d + 1) - ml(real, d)
    # noise of the official branching: spread over the 5 reps
    reps = [r["log10_width"][d + 1] - r["log10_width"][d] for r in real]
    nb = [ml(v, d + 1) - ml(v, d) for v in inst.values()]
    # estimator noise inside the null: half-difference of the two reps
    en = [abs((v[0]["log10_width"][d + 1] - v[0]["log10_width"][d]) - (v[1]["log10_width"][d + 1] - v[1]["log10_width"][d])) / 2 for v in inst.values()]
    sd = st.stdev(nb)
    z = (rb - st.mean(nb)) / sd if sd > 1e-9 else 0.0
    out.append((d, rb, st.stdev(reps), st.mean(nb), sd, st.mean(en), z, prof[d + 1] - prof[d]))
zs = [o[6] for o in out if o[4] > 1e-9]
print("levels compared:", len(zs))
print("mean z %.3f  sd of z %.3f" % (st.mean(zs), st.stdev(zs)))
for t in (2, 2.5, 3, 3.5):
    n = sum(1 for z in zs if abs(z) > t)
    # with 20 null boards the reference is Student t with 19 df, scaled by sqrt(1+1/20)
    print("  |z| > %.1f: %d" % (t, n))
print("ten largest |z|:")
for o in sorted(out, key=lambda o: -abs(o[6]))[:10]:
    print("  level %3d->%3d  official %+.4f (rep sd %.4f)  null mean %+.4f sd %.4f (null estimator noise %.4f)  z %+.2f   model %+.4f" % (o[0], o[0] + 1, o[1], o[2], o[3], o[4], o[5], o[6], o[7]))
print("\nlevels 94..100:")
for o in out:
    if 94 <= o[0] <= 100:
        print("  level %3d->%3d  official %+.4f  null mean %+.4f sd %.4f  z %+.2f  model %+.4f" % (o[0], o[0] + 1, o[1], o[3], o[4], o[6], o[7]))
# sign runs: is the official branching persistently below the null over a stretch
import itertools
runs = [(k, len(list(g))) for k, g in itertools.groupby([z > 0 for z in zs])]
print("\nlongest run of same-sign z:", max(r[1] for r in runs), "of", len(zs), "; number of runs", len(runs), "(about %d expected if independent)" % (len(zs) / 2 + 1))
print("share of levels with official branching below the null mean: %.3f" % (sum(1 for z in zs if z < 0) / len(zs)))
# real minus model per-level branching, by position inside the row of 16
print("\nofficial minus model branching summed by scan row (16 levels), in dex:")
for a in range(0, 176, 16):
    s = sum(o[1] - o[7] for o in out if a < o[0] + 1 <= a + 16)
    sn = sum(o[3] - o[7] for o in out if a < o[0] + 1 <= a + 16)
    print("  levels %3d..%3d  official-model %+.4f   null-model %+.4f" % (a + 1, a + 16, s, sn))
