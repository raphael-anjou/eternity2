"""Review item 4, toy check (generated boards, not Eternity II): at which rate do two halves agree on the seam?
 all pairs (independent halves), pairs with complementary used sets, pairs whose seams have the same colour-parity vector.
Also asserts the parity identity on every strict half filling: parity of each colour on the seam = parity on the tiles used.
Imports gen() and region() from the memo's toy script, read only. Run: python3 -I r3_toy_seam.py"""
import sys, os, random, time
from math import log2
from collections import defaultdict
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "gap-2"))
from g2_toy_sketch import gen, region
def run(n, cb, ci, k, seed):
    rng = random.Random(seed); pcs = gen(n, cb, ci, rng); nt = n * n; full = (1 << nt) - 1
    strict = lambda m, i: None if m >> i & 1 else m | 1 << i
    _, _, top = region(n, k, pcs, False, 0, strict); _, _, bot = region(n, n - k, pcs, True, 0, strict)
    if top is None or bot is None: print("skipped (cap)"); return
    ncol = cb + ci + 1
    tpar = [sum(1 << c for c in set(p) if c and p.count(c) % 2) for p in pcs]
    def spar(f): return sum(1 << c for c in set(f) if c and f.count(c) % 2)
    def mpar(m):
        v = 0
        for i in range(nt):
            if m >> i & 1: v ^= tpar[i]
        return v
    bad = sum(1 for (f, m) in list(top) + list(bot) if spar(f) != mpar(m))
    NA = sum(top.values()); NB = sum(bot.values())
    sa, sb, pa, pb, ma, mb = (defaultdict(int) for _ in range(6))
    for (f, m), c in top.items(): sa[f] += c; pa[spar(f)] += c; ma[m] += c
    for (f, m), c in bot.items(): sb[f] += c; pb[spar(f)] += c; mb[m] += c
    seam_all = sum(c * sb.get(f, 0) for f, c in sa.items())
    par_all = sum(c * pb.get(p, 0) for p, c in pa.items())
    compl = sum(c * mb.get(full ^ m, 0) for m, c in ma.items())
    Z = sum(c * bot.get((f, full ^ m), 0) for (f, m), c in top.items())
    # uniform width of the seam
    Fu = (n - 2) * log2(ci) + 2 * log2(cb)
    g = lambda x, y: log2(y / x) if x else float("nan")
    print("%dx%d %d+%d colours seed %2d cut %d | NA %7d NB %7d | parity identity violated on %d fillings | uniform seam %.1f bits | all pairs %.2f | same seam parity %.2f | complementary sets %.2f (%d pairs, %d solutions) | parity vector is worth %.2f bits"
          % (n, n, cb, ci, seed, k, NA, NB, bad, Fu, g(seam_all, NA * NB), g(seam_all, par_all), g(Z, compl), compl, Z, g(par_all, NA * NB)))
    return g(seam_all, NA * NB), g(seam_all, par_all), g(Z, compl)
t0 = time.time(); acc = []
for (n, cb, ci, k, seeds) in ((4, 2, 3, 2, range(1, 9)), (5, 2, 4, 2, range(1, 13)), (5, 2, 4, 3, range(1, 13)), (5, 3, 4, 3, (3,))):
    for s in seeds:
        r = run(n, cb, ci, k, s)
        if r and all(x == x for x in r): acc.append(r)
        if time.time() - t0 > 600: break
print("mean over %d boards with at least one solution: all pairs %.2f bits, same-parity pairs %.2f, complementary pairs %.2f" % (len(acc), *[sum(a[i] for a in acc) / len(acc) for i in range(3)]))
