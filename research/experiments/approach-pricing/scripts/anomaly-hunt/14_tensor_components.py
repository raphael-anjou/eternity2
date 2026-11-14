"""Gap 5, appendix A: connected components of the (top,right | bottom,left)
graph of the interior tiles: official 9, 40 painted boards min 1 mean 4.8
max 10. Tail from 2,000 painted boards (same painter, stdlib union-find).
python3 -I 14_tensor_components.py
"""
import os, sys, random
os.environ.setdefault("E2_CSV", "engine/data/official_eternity2.csv")
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "gap-5"))
import e2common as E
def comps(T):
    par = {}
    def find(x):
        par.setdefault(x, x)
        while par[x] != x:
            par[x] = par[par[x]]; x = par[x]
        return x
    for t in T:
        if 0 in t: continue
        for i in range(4):
            r = t[i:] + t[:i]
            par[find(("a", r[0], r[1]))] = find(("b", r[2], r[3]))
    return len({find(x) for x in list(par)})
loaded = E.load()
P = loaded[0] if isinstance(loaded, tuple) else loaded
obs = comps(P); print("official components:", obs)
ring_pool, inner_pool = E.official_pools(P)
rng = random.Random(14); B = 2000; v = []
for _ in range(B):
    T = [t for row in E.paint(rng, ring_pool, inner_pool) for t in row]
    v.append(comps(T))
import statistics as st, collections
print("painted: mean %.2f sd %.2f  P(>= %d) = %.4f" % (st.mean(v), st.stdev(v), obs, sum(1 for x in v if x >= obs) / B))
print(sorted(collections.Counter(v).items()))
