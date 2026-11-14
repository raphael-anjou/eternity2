"""Claims B and F. (1) Where the instance-to-instance spread of the width comes
from, on fresh draws from the model's ensemble. (2) The population estimator of
path 5 section 7, simulated on a committed 6x6 instance whose exact widths are
known: bias, variance, and the variance formula.
Run: python3 -I 04_smc_and_variance.py"""
import json
import os
import random
import statistics
import sys
import time
from math import log10

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rv_lib import *

t0 = time.time()
rng = random.Random(4)

print("1. spread between instances of the same census (unplanted draws), by depth")
for (n, cf, ci, k) in ((4, 3, 3, 60), (5, 4, 4, 40), (6, 5, 5, 24)):
    base, _ = gen_planted(n, cf, ci, random.Random(n), balanced=True)
    Mm = Model(n, census(base))
    order = rows_from(n, True)
    mod = Mm.profile(order)
    D = mod.index(max(mod)) + 1
    rat = {d: [] for d in (n, 2 * n, D)}
    tot, fr = [], []
    for _ in range(k):
        us = shuffle_census(base, rng)
        per = Enum(n, us).run(order)["per"][1:]
        tot.append(sum(per) / 10 ** logsum(mod))
        for d in rat:
            rat[d].append(per[d - 1] / 10 ** mod[d - 1])
    def sdlog(v):
        z = [log10(x) for x in v if x > 0]
        return statistics.pstdev(z), len(v) - len(z)
    print("   %dx%d (%d/%d), %d draws, model tree 10^%.2f: tree/model mean %.2f (s.e. %.2f), sd log10 %.2f | width/model sd log10 (zeros dropped): row 1 %.2f (%d zero), row 2 %.2f (%d), peak depth %d %.2f (%d)   [%.0fs]"
          % (n, n, cf, ci, k, logsum(mod), statistics.mean(tot), statistics.stdev(tot) / k ** 0.5, sdlog(tot)[0],
             *sdlog(rat[n]), *sdlog(rat[2 * n]), D, *sdlog(rat[D]), time.time() - t0))

print("\n2. population estimator on committed instance n6-c5-s1 (row scan, no clue)")
LAD = os.path.join(REPO, "research", "experiments", "hard-ladder")
x = json.loads(open(os.path.join(LAD, "instances", "primary-test.jsonl")).readline())
n = 6
ps = [tuple(p) for p in x["puzzle"]["pieces"]]
order = rows_from(n, True)
Dt = 20
en = Enum(n, ps)
ex = en.run(order, target=Dt)
W = ex["per"]
cv2 = [W[k] * ex["s2"][k] / ex["s1"][k] ** 2 - 1 for k in range(1, Dt)]
print("   exact width at depth %d: %d ; sum of CV^2_k for k<%d: %.1f ; largest single term %.1f" % (Dt, W[Dt], Dt, sum(cv2), max(cv2)))


def smc(Np, rng):
    pop = [((), 0)]  # (tuple of oriented pieces in scan order, used bitmask)
    est = 1.0
    out = []
    for d in range(Dt):
        r, c = divmod(d, n)
        ext = []
        for st, used in pop:
            key = (st[d - n][2] if r else 0, 0 if c == n - 1 else -1, 0 if r == n - 1 else -1, st[d - 1][1] if c else 0)
            for i, q in en.cands(key):
                if not used >> i & 1:
                    ext.append((st + (q,), used | 1 << i))
        if not ext:
            out.extend([0.0] * (Dt - d))
            return out
        est *= len(ext) / len(pop)
        out.append(est)
        pop = ext if len(ext) <= Np else rng.sample(ext, Np)
    return out


for Np, R in ((50, 400), (200, 200)):
    runs = [smc(Np, rng) for _ in range(R)]
    fin = [r[Dt - 1] for r in runs]
    mean = statistics.mean(fin)
    rv = statistics.variance(fin) / W[Dt] ** 2
    logs = [log10(v) for v in fin if v > 0]
    print("   N=%d, %d runs: mean estimate / exact = %.3f (s.e. %.3f) | relative variance %.3f, formula sum CV^2 / N = %.3f | median estimate / exact = %.2f | mean of log10 estimates minus log10 exact = %+.3f | extinct runs %d   [%.0fs]"
          % (Np, R, mean / W[Dt], (rv / R) ** 0.5, rv, sum(cv2) / Np, statistics.median(fin) / W[Dt], statistics.mean(logs) - log10(W[Dt]), R - len(logs), time.time() - t0))
