"""Gap 3 review, item 4: read r3_shape outputs. python3 -I r3_shape.py <out files...>

Per depth d: sample mean of the subtree size against the level profile (sum of deeper widths
over the width at d), median, share of empty subtrees, share of the work held by the largest
1% of subtrees, and the size bias: the subtree size at which half of the level's work is
reached, over the mean (where a prefix drawn in proportion to work sits in the median).
"""
import math
import sys
from collections import defaultdict

for fn in sys.argv[1:]:
    lv, us, capped, end = {}, defaultdict(list), defaultdict(int), "no END line (run cut)"
    for l in open(fn):
        t = l.split()
        if t[0] == "LV":
            lv[int(t[1])] = float(t[2])
        elif t[0] == "US":
            us[int(t[1])].append(int(t[2]))
            capped[int(t[1])] += int(t[3])
        elif t[0] == "END":
            end = "nodes counted %s" % t[1]
        elif t[0] == "STOP":
            end = "NODE LIMIT REACHED at depth %s" % t[1]
    tot = math.log10(sum(10 ** v for v in lv.values()))
    print("%s: tree 10^%.3f = 2^%.1f, widest level %d, last level %d, %s" % (fn.rsplit("/", 1)[-1], tot, tot * math.log2(10), max(lv, key=lv.get), max(lv), end))
    for d in sorted(us):
        S = sorted(us[d])
        n = len(S)
        mean = sum(S) / n
        prof = sum(10 ** (lv[k] - lv[d]) for k in lv if k > d)
        top = sum(S[n - n // 100:]) / sum(S)
        acc, half = 0, None
        for x in S:
            acc += x
            if acc >= sum(S) / 2:
                half = x
                break
        m2 = sum(x * x for x in S) / n / mean ** 2
        med = S[n // 2]
        typ = math.log2(mean / (2 * sum(min(x, med) for x in S) / n))
        print("  d=%3d n=%5d capped %d | mean %.4g (profile %.4g) median %d empty %.3f top 1%% holds %.3f | median-node gain %.1f bits | work-weighted median size / mean = %.1f, E[S^2]/E[S]^2 = %.0f, largest subtree holds %.3f"
              % (d, n, capped[d], mean, prof, med, S.count(0) / n, top, typ, half / mean, m2, S[-1] / sum(S)))
