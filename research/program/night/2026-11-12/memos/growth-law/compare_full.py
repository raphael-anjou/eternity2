"""Model against exhaustive row-major trees on committed instances.

Run after dfs_check.py has written data/full_*.jsonl. Stdlib only.
"""
import json, random, statistics as st, sys, os
sys.path.insert(0, os.path.dirname(__file__))
from model import Profile

SETS = (("data/full_n6_c7.jsonl", 6, 7), ("data/full_n6_c6.jsonl", 6, 6),
        ("data/full_n6_c5.jsonl", 6, 5), ("data/full_n7_c8.jsonl", 7, 8))
here = os.path.dirname(os.path.abspath(__file__))
rng = random.Random(1)
print("n c inst | full tree: measured mean, model, ratio | solutions: measured mean, model | "
      "first: measured mean, model mean, measured median, model median | max |ln ratio| of Z_d up to the peak, past it")
for f, n, c in SETS:
    rows = [json.loads(l) for l in open(os.path.join(here, f))]
    ok = [r for r in rows if r["full"]]
    p = Profile(n, c, "rowmajor")
    N = n * n
    fulls = [r["full"] for r in ok]
    firsts = [r["first"] for r in rows]
    mz = [st.mean(r["Z"][d] for r in ok) for d in range(N + 1)]
    dpk = max(range(1, N + 1), key=lambda d: p.Z_planted(d))
    import math
    up = max(abs(math.log(mz[d] / p.Z_planted(d))) for d in range(1, dpk + 1))
    down = max(abs(math.log(mz[d] / p.Z_planted(d))) for d in range(dpk + 1, N + 1))
    smp = p.first_solution_sample(rng, 4001)
    print(n, c, len(ok), "| %.0f %.0f %.2f | %.2f %.2f | %.0f %.0f %.0f %.0f | %.2f %.2f" % (
        st.mean(fulls), p.full_tree(), st.mean(fulls) / p.full_tree(),
        st.mean(r["solutions"] for r in ok), p.solutions_planted(),
        st.mean(firsts), p.first_solution_mean(), st.median(firsts), smp[len(smp) // 2], up, down))
