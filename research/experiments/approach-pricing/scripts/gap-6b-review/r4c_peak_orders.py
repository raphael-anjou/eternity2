"""Review of gap 6b, script 4c.
(i)   Focused search at the peak sizes: for every k from 146 to 166, is there a set
      of k cells with fewer strict partial boards than the row-scan prefix?
      (five clues, real-count ledger; longer annealing than r4b).
(ii)  First-moment price of the "black cells first" order (the memo's E/C, which it
      could only run on 5x5 toys): a black tile is placed, then the white cell above
      it, whose four sides are now known, is filled. As a static cell order.
(iii) Row E, two colour classes: exact number of distinct coarse arrangements of the
      official interior tiles, against the memo's cap of 196 x log2(1/q^4).
Usage: python3 -I r4c_peak_orders.py <csv> [seed]
"""
import sys, os, random
from collections import Counter
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rlib import *
import r4b_region_min as R
tiles, extra = load(sys.argv[1])
rng = random.Random(int(sys.argv[2]) if len(sys.argv) > 2 else 11)
PIr, PFr, PIu, PFu = tables()

print("== (i) focused search, real-count ledger, five clues")
prof = [R.State(range(d), PIr, PFr).val() for d in range(1, 257)]
worst = None
for k in range(146, 167, 2):
    best = (prof[k - 1], "row-scan prefix")
    for trial in range(6):
        start = list(range(k)) if trial < 3 else rng.sample(range(256), k)
        v, S = R.anneal(start, PIr, PFr, rng, 300000, (0.6, 2.0, 5.0)[trial % 3])
        if v < best[0] - 1e-9: best = (v, f"annealed, open edges {R.State(S, PIr, PFr).open()}")
    print(f"  k={k}: prefix {prof[k-1]:7.2f}; best found {best[0]:7.2f} ({best[1]}); gain {prof[k-1]-best[0]:+.2f}", flush=True)
    if worst is None or best[0] > worst[0]: worst = (best[0], k)
print(f"  bottleneck over these sizes: 2^{worst[0]:.2f} at k={worst[1]}; row-scan peak level 2^{max(prof):.2f}")

print("\n== (ii) static orders priced as sequences of regions (five clues)")
def price(order, PI, PF):
    st = R.State([], PI, PF); lv = []
    for c in order:
        st.add(R.idx[c]); lv.append(st.val())
    return lsum(lv), max(lv), lv.index(max(lv)) + 1
rowscan = [(r, c) for r in range(16) for c in range(16)]
bf = []
for r in range(16):
    for c in range(16):
        if (r + c) % 2 == 0:
            bf.append((r, c))
            if r > 0: bf.append((r - 1, c))
    # after the black cells of row r, every white cell of row r-1 is complete
bf += [(15, c) for c in range(16) if (15 + c) % 2 == 1]
assert sorted(bf) == sorted(rowscan)
allblack = [x for x in rowscan if sum(x) % 2 == 0] + [x for x in rowscan if sum(x) % 2 == 1]
for nm, o in (("row scan", rowscan), ("black cell, then the white cell above it", bf), ("all black cells, then all white cells", allblack)):
    for ln, PI, PF in (("real", PIr, PFr), ("uniform", PIu, PFu)):
        t, p, d = price(o, PI, PF)
        print(f"  {nm:42s} {ln:8s} total 2^{t:7.2f}, widest level 2^{p:7.2f} at depth {d}")

print("\n== (iii) two colour classes on the official interior tiles")
RIM = 65535
inner = [t for t in tiles if RIM not in t]
cols = sorted({c for t in inner for c in t})
def coarse_entropy(classA):
    pat = Counter()
    for t in inner:
        b = tuple(1 if c in classA else 0 for c in t)
        pat[min(b[k:] + b[:k] for k in range(4))] += 1
    e = lf(196)
    for p, m in pat.items():
        e += m * log2(len({p[k:] + p[:k] for k in range(4)})) - lf(m)
    return e, dict(pat)
cnt = Counter(c for t in tiles for c in t if c in cols)
outs = []
for trial in range(20):
    A = set(cols[:9]) if trial == 0 else set(rng.sample(cols, 9))
    e, pat = coarse_entropy(A)
    nA = sum(cnt[c] for c in A)
    m = match_table([nA, 840 - nA], 420)[420]
    outs.append((e, m, e + m))
print(f"  distinct coarse arrangements of the 196 interior tiles: 2^{min(o[0] for o in outs):.1f} to 2^{max(o[0] for o in outs):.1f} over 20 splits into 9 and 8 colours (memo's cap: 2^{196*log2(1/0.5017**4):.1f}; all 420 two-class edge colourings: 2^420)")
print(f"  times P(all 420 edges match), real counts: interior coarse solutions 2^{min(o[2] for o in outs):.1f} to 2^{max(o[2] for o in outs):.1f} (memo: 476.1 - 115.2 = 360.9); with the exact frame count 124.93: 2^{min(o[2] for o in outs)+124.93:.1f} to 2^{max(o[2] for o in outs)+124.93:.1f} (memo 476.1)")
