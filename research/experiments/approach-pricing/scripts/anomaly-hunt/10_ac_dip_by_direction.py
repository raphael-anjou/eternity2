"""D4 arc-consistency follow-up (wt-d4, read only): the share of sampled wrong
prefixes that arc consistency keeps, by depth and by scan direction. The
pooled table has a dip at depth 112 that was left unexplained.
python3 -I 10_ac_dip_by_direction.py
"""
import json, glob, collections, statistics as st
D = "research/experiments/prefix-statistics/results/acfollowup/*.jsonl"
rows = []
for f in sorted(glob.glob(D)): rows += [json.loads(l) for l in open(f)]
g = collections.defaultdict(list)
for r in rows: g[(r["depth"], r["dir"])].append(r["share"])
print("depth  dir0   dir1   dir2   dir3   pooled   (mean over 15 painted boards)   sd between boards per dir")
for d in sorted(set(k[0] for k in g)):
    m = [st.mean(g[(d, k)]) for k in range(4)]
    print("%5d  " % d + "  ".join("%.3f" % x for x in m) + "   %.3f     " % st.mean(m) + " ".join("%.3f" % st.stdev(g[(d, k)]) for k in range(4)))
