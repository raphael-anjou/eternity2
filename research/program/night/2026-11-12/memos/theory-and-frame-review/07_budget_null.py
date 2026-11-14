"""Review check 7 (stdlib): irreducible-hard-region's budget test against the
ordinary-tree null. Refereed first-moment model (rv_lib), official census, no
clues, row scan. A depth-first search with budget B reaches about the depth D
where width(D) = peak width / B. How far does D move from 1.5 to 4 million nodes?

Run: python3 -I 07_budget_null.py
"""
import os, sys
from math import log10
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "price-and-ledger-review"))
import rv_lib
ps, hints = rv_lib.load_official()
M = rv_lib.Model(16, rv_lib.census(ps))
pr = M.profile(rv_lib.rows_from(16, True))
pk = max(range(256), key=lambda i: pr[i])
print("no clues, row scan: sum 10^%.2f, peak 10^%.2f at depth %d ; width at depth 50 / 100 / 150 / 200: 10^%.1f / 10^%.1f / 10^%.1f / 10^%.1f"
      % (rv_lib.logsum(pr), pr[pk], pk + 1, pr[49], pr[99], pr[149], pr[199]))
for B in (1.5e6, 4e6, 1.5e9, 1.5e12):
    target = pr[pk] - log10(B)
    d = next(i for i in range(pk, 256) if pr[i] < target)
    # linear interpolation
    frac = (pr[d - 1] - target) / (pr[d - 1] - pr[d])
    print("  budget %.1e nodes: reaches depth %.1f = row %.2f of 16 (frontier fraction %.3f)" % (B, d + frac, (d + frac) / 16, (d + frac) / 256))
