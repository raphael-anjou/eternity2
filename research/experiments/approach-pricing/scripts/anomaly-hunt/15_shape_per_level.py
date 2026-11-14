"""Shape, not means: per level, the share of population members with no child
and the variance-to-mean ratio of the number of children (Track 10 rows,
field levels_E_M_zero_mean_var), official set against the 20 painted boards.
python3 -I 15_shape_per_level.py
"""
import json, os, statistics as st
HERE = os.path.dirname(os.path.abspath(__file__))
def load(f): return [json.loads(l) for l in open(os.path.join(HERE, "data", f))]
real = load("five-np1e6.jsonl"); real5 = load("five-np1e5.jsonl"); paint = load("paint-np1e5.jsonl")
inst = {}
for r in paint: inst.setdefault(r["label"], []).append(r)
def get(rows, d, what):
    v = []
    for r in rows:
        L = r["levels_E_M_zero_mean_var"]
        if d < len(L):
            E, M, zero, mean, var = L[d]
            v.append(zero if what == "zero" else (var / mean if mean > 0 else 0.0))
    return st.mean(v) if v else None
for what in ("zero", "vmr"):
    zs = []; rows = []
    for d in range(16, 180):
        # like for like: official at N = 1e5 (same population as the null) and at N = 1e6
        o5 = get(real5, d, what); o6 = get(real, d, what)
        nv = [get(v, d, what) for v in inst.values()]
        nv = [x for x in nv if x is not None]
        m, s = st.mean(nv), st.stdev(nv)
        if s > 1e-9:
            z5 = (o5 - m) / s; z6 = (o6 - m) / s
            zs.append(z5); rows.append((d, o5, o6, m, s, z5, z6))
    print("\n%s: levels %d ; z of official (N=1e5) against 20 painted: mean %+.2f sd %.2f ; |z|>2: %d ; |z|>3: %d" % (
        what, len(zs), st.mean(zs), st.stdev(zs), sum(1 for z in zs if abs(z) > 2), sum(1 for z in zs if abs(z) > 3)))
    for r in sorted(rows, key=lambda r: -abs(r[5]))[:6]:
        print("   level %3d  official 1e5 %.4f  1e6 %.4f   null %.4f sd %.4f   z(1e5) %+.2f  z(1e6) %+.2f" % r)
    for d in (64, 96, 112, 128, 144, 158, 175):
        r = [x for x in rows if x[0] == d]
        if r: print("   level %3d  official 1e5 %.4f  1e6 %.4f   null %.4f sd %.4f   z(1e5) %+.2f  z(1e6) %+.2f" % r[0])
