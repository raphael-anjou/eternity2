"""Three samples of the same painted ensemble (Track 10's census painter, five
clues, row scan from row 15): are they consistent with each other?
python3 -I 13_null_ensembles.py
"""
import json, math, os, statistics as st
HERE = os.path.dirname(os.path.abspath(__file__))
D3 = "research/experiments/orders-and-pins/results"
def L(p): return [json.loads(l) for l in open(p)]
def lm(rows): return math.log10(sum(r["total"] for r in rows) / len(rows))
t10 = {}
for r in L(os.path.join(HERE, "data", "paint-np1e5.jsonl")): t10.setdefault(r["label"], []).append(r)
a = sorted(lm(v) for v in t10.values())
half = [abs(math.log10(v[0]["total"]) - math.log10(v[1]["total"])) for v in t10.values()]
# sd of one rep from paired differences: sd(diff) = sqrt(2) sd(rep)
sd_rep = math.sqrt(sum(h * h for h in half) / len(half) / 2)
print("T10 painted, 20 boards, N=1e5 x 2:", [round(x, 3) for x in a])
print("  mean %.4f sd %.4f ; sd of one repetition from the pairs %.4f ; so estimator sd of a 2-rep mean %.4f ; instance sd about %.4f" % (
    st.mean(a), st.stdev(a), sd_rep, sd_rep / math.sqrt(2), math.sqrt(max(st.variance(a) - sd_rep ** 2 / 2, 0))))
nul = L(os.path.join(D3, "a/a-null.jsonl"))
b = [lm([r for r in nul if r["label"] == "n-%s-rows-b-lr" % s]) for s in ("200115", "200139", "200181")]
print("D3 null, 3 boards, N=1e5 x 5:", [round(x, 3) for x in b], "mean %.4f sd %.4f" % (st.mean(b), st.stdev(b)))
b0 = L(os.path.join(D3, "b/b0.jsonl"))
c = [lm([r for r in b0 if r["label"] == "b0-%s-smc" % s]) for s in ("300051", "300061", "300097", "300117", "300131")]
print("D3 Part B, 5 boards, N=1e6 x 5:", [round(x, 3) for x in c], "mean %.4f sd %.4f" % (st.mean(c), st.stdev(c)))
allv = a + b + c
print("pooled 28 boards: mean %.4f sd %.4f ; model 40.4946 ; official 40.4567 -> z %+.2f, boards below %d of 28" % (
    st.mean(allv), st.stdev(allv), (40.4567 - st.mean(allv)) / st.stdev(allv), sum(1 for x in allv if x < 40.4567)))
# is the Part B five a plausible draw from the T10 twenty? permutation test on mean and on sd
import random
rng = random.Random(5); T = 200000; pool = a + c; k = 0; ks = 0
for _ in range(T):
    s = rng.sample(pool, 5)
    k += st.mean(s) <= st.mean(c); ks += st.stdev(s) <= st.stdev(c)
print("permutation (25 boards, pick 5): P(mean <= Part B mean) %.4f ; P(sd <= Part B sd) %.4f" % (k / T, ks / T))
