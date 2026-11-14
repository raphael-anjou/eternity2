"""Claim B (and the variance term of claim F). Independent recomputation of the
small-board calibration on the 30 committed hard-ladder 6x6 c=5 test instances,
with the referee's enumerator and model, plus the control the memos did not
run: fresh draws from the model's own ensemble (same census, colours
reshuffled, no planted solution), where the model must be unbiased.
Run: python3 -I 02_calibration.py"""
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
LAD = os.path.join(REPO, "research", "experiments", "hard-ladder")
inst = [json.loads(l) for l in open(os.path.join(LAD, "instances", "primary-test.jsonl"))]
inst = [x for x in inst if x["n"] == 6][:30]
n = 6
order = rows_from(n, True)
rng = random.Random(20261112)
rows_p, rows_u = [], []
smc = []
for x in inst:
    ps = [tuple(p) for p in x["puzzle"]["pieces"]]
    cen = census(ps)
    M = Model(n, cen)
    mod = M.profile(order)
    D = mod.index(max(mod)) + 1
    en = Enum(n, ps).run(order, target=D)
    per = en["per"][1:]
    rows_p.append((sum(per), 10 ** logsum(mod), per, mod, per[-1]))
    # SMC variance term: CV^2 over depth-k nodes of the number of depth-D descendants
    cv2 = []
    for k in range(1, D):
        nk, s1, s2 = en["per"][k], en["s1"][k], en["s2"][k]
        cv2.append(nk * s2 / (s1 * s1) - 1)
    smc.append((D, cv2, per[D - 1]))
    us = shuffle_census(ps, rng)
    assert census(us)["frame"] == cen["frame"]
    pu = Enum(n, us).run(order)["per"][1:]
    rows_u.append((sum(pu), 10 ** logsum(mod), pu, mod, pu[-1]))
print("30 committed 6x6 c=5 instances, row scan r=0 first, model total 10^%.3f each (same census)   [%.0fs]" % (log10(rows_p[0][1]), time.time() - t0))
for name, rows in (("planted (committed instances)", rows_p), ("unplanted (same census, reshuffled)", rows_u)):
    lr = [log10(r[0] / r[1]) for r in rows]
    ar = [r[0] / r[1] for r in rows]
    print("%s:" % name)
    print("   tree / model: geometric mean %.3f, arithmetic mean %.3f (s.e. %.3f), sd of log10 %.3f, min %.2f, max %.2f; solutions in total %d"
          % (10 ** statistics.mean(lr), statistics.mean(ar), statistics.stdev(ar) / len(ar) ** 0.5, statistics.pstdev(lr), min(ar), max(ar), sum(r[4] for r in rows)))
    line = []
    for d in (4, 8, 12, 16, 20, 22, 24, 26, 28, 30, 32, 34, 36):
        m = statistics.mean(r[2][d - 1] for r in rows)
        line.append("%d:%.2f" % (d, m / 10 ** rows[0][3][d - 1]))
    print("   mean exact width / model width by depth:", " ".join(line))
print("\nSMC variance term on the same 30 planted instances (target depth D = model peak = %d):" % smc[0][0])
S = [sum(c) for _, c, _ in smc]
print("   sum over k<D of CV^2_k(descendants at D): median %.1f, min %.1f, max %.1f" % (statistics.median(S), min(S), max(S)))
D = smc[0][0]
print("   median CV^2_k by depth k:", " ".join("%d:%.2f" % (k, statistics.median(c[k - 1] for _, c, _ in smc)) for k in range(1, D)))
json.dump({"planted": [(r[0], r[1]) for r in rows_p], "unplanted": [(r[0], r[1]) for r in rows_u], "smc_sum_cv2": S},
          open(os.path.join(HERE, "02_calibration.json"), "w"))
print("[%.0fs]" % (time.time() - t0))
