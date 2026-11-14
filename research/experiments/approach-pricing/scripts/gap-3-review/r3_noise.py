"""Gap 3 review: noise of the memo's sibling sizes (SMC, population 10^4).

  python3 -I r3_noise.py <memo sib file> <rerun of the same case, other seed or population> ...

Pairs of files for the same board and direction. Siblings are matched by line order (the
program lists them in a fixed order). Prints, per depth band, the spread of the log2 ratio of
the two estimates, to be read against the spread of log2 size among the siblings of one node
(what the placement law of r3_tail.py section E is fitted on).
"""
import math
import sys
from collections import defaultdict


def load(fn):
    out = []
    for l in open(fn):
        t = l.split()
        if t and t[0] == "SB":
            out.append((int(t[1]), 1.0 + float(t[2])))
    return out


def band(j):
    return 0 if j < 32 else 1 if j < 96 else 2 if j < 160 else 3


NAMES = ["depth 0-31", "depth 32-95", "depth 96-159", "depth 160-250"]
a = sys.argv[1:]
for i in range(0, len(a), 2):
    x, y = load(a[i]), load(a[i + 1])
    assert len(x) == len(y) and all(p[0] == q[0] for p, q in zip(x, y)), "sibling lists differ"
    print("%s against %s: %d siblings, tree 2^%.2f and 2^%.2f" % (a[i].rsplit("/", 1)[-1], a[i + 1].rsplit("/", 1)[-1], len(x), math.log2(sum(p[1] for p in x)), math.log2(sum(p[1] for p in y))))
    d = defaultdict(list)
    within = defaultdict(list)
    bydepth = defaultdict(list)
    for p, q in zip(x, y):
        d[band(p[0])].append(math.log2(p[1] / q[1]))
        bydepth[p[0]].append(math.log2(p[1]))
    for j, v in bydepth.items():
        if len(v) > 1:
            m = sum(v) / len(v)
            within[band(j)] += [u - m for u in v]
    for b in range(4):
        v = d[b]
        if not v:
            continue
        rms = math.sqrt(sum(u * u for u in v) / len(v))
        w = within[b]
        sdw = math.sqrt(sum(u * u for u in w) / max(1, len(w)))
        print("   %-13s %4d siblings: rms log2 ratio %.3f bits (per estimate about %.3f), largest %.2f; spread of log2 size among siblings of one node: %.2f bits"
              % (NAMES[b], len(v), rms, rms / math.sqrt(2), max(abs(u) for u in v), sdw))
