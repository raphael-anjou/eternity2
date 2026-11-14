"""First-moment model on the real colour census (M12 path 3 model, copied verbatim as m12_model.py).
Items 6 (cut floor, scan directions), 7 (SMC as search), 8 (fill orders, value of a pin).
Run: python3 -I s_model.py <engine/data/official_eternity2.csv>"""
import sys
from collections import Counter
from math import log10, log2
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from m12_model import interior_match_logp, frame_match_logp, profile, logsum10
N = 16; B = log2(10)
def load(fn):
    rows = [l.strip() for l in open(fn) if l.strip()]
    pieces, hints = [], []
    for i, l in enumerate(rows[1:]):
        c = l.split(",")
        pieces.append([0 if int(x, 2) == 65535 else int(x, 2) for x in c[:4]])
        x, y, rot = int(c[4]), int(c[5]), int(c[6])
        if x or y or rot: hints.append((y, x, i))
    return pieces, hints
pieces, hints = load(sys.argv[1])
fc, ic = Counter(), Counter()
for p in pieces:
    z = [i for i, v in enumerate(p) if v == 0]
    if len(z) == 0:
        for v in p: ic[v] += 1
    elif len(z) == 1:
        i = z[0]; ic[p[(i + 2) % 4]] += 1; fc[p[(i + 1) % 4]] += 1; fc[p[(i + 3) % 4]] += 1
    else:
        for v in p:
            if v: fc[v] += 1
lpi = interior_match_logp(list(ic.values()), 420); lpf = frame_match_logp([v // 2 for v in fc.values()], 60)
upi = [-e * log10(17) for e in range(421)]; upf = [-f * log10(5) for f in range(61)]
def ring(k): 
    a, b = k, N - 1 - k
    return [(a, c) for c in range(a, b + 1)] + [(r, b) for r in range(a + 1, b + 1)] + [(b, c) for c in range(b - 1, a - 1, -1)] + [(r, a) for r in range(b - 1, a, -1)]
ORD = {
 "rows top-down": [(r, c) for r in range(N) for c in range(N)],
 "rows bottom-up": [(r, c) for r in reversed(range(N)) for c in range(N)],
 "cols left-right": [(r, c) for c in range(N) for r in range(N)],
 "cols right-left": [(r, c) for c in reversed(range(N)) for r in range(N)],
 "spiral inward": [x for k in range(N // 2) for x in ring(k)],
 "frame then rows": ring(0) + [(r, c) for r in range(1, N - 1) for c in range(1, N - 1)],
 "anti-diagonal": sorted(((r, c) for r in range(N) for c in range(N)), key=lambda x: (x[0] + x[1], x[0])),
}
clue = {(h[0], h[1]) for h in hints}
def tot(order, fixed, li, lf, pools=None):
    pools = pools or {"corner": 4, "edge": 56, "interior": 196 - len(fixed)}
    logs = [x[0] for x in profile(N, order, pools, li, lf, fixed)]
    return logsum10(logs), logs
print("== item 8: static fill orders, log10 expected nodes ==")
print("%-16s | uniform colours, no clue | real census, no clue | real census, five clues" % "order")
for name, o in ORD.items():
    a = tot(o, set(), upi, upf)[0]; b = tot(o, set(), lpi, lpf)[0]; c = tot(o, clue, lpi, lpf)[0]
    print("%-16s | %6.2f | %6.2f | %6.2f" % (name, a, b, c))
base, blogs = tot(ORD["rows bottom-up"], clue, lpi, lpf)
print("\nbaseline (model, rows bottom-up, five clues): 10^%.2f = 2^%.1f ; peak 10^%.2f = 2^%.1f at depth %d" % (base, base * B, max(blogs), max(blogs) * B, blogs.index(max(blogs)) + 1))
print("\n== item 6: scan directions as 'presentations' ==")
four = [tot(ORD[k], clue, lpi, lpf)[0] for k in list(ORD)[:4]]
print("four directions: %s ; running all four costs 2^%.2f times the best one (4 equal trees would be 2^2)" % (" ".join("%.2f" % x for x in four), (logsum10(four) - min(four)) * B))
print("\n== item 6: best two-list band cut (five clues), log10 list sizes ==")
td = tot(ORD["rows top-down"], clue, lpi, lpf)[1]; bu = blogs
for r in range(5, 12):
    print("  cut after row %2d: tops %.2f bottoms %.2f  max %.2f" % (r, td[16 * r - 1], bu[16 * (16 - r) - 1], max(td[16 * r - 1], bu[16 * (16 - r) - 1])))
print("\n== item 7: SMC or beam of population P used as a search ==")
W = [10 ** x for x in blogs]; pk = max(W)
for lp in (6, 20, 30, 36, log10(pk)):
    P = 10 ** lp; M = 0.0; cost = 0.0
    for w in W:
        M = max(M, w)
        cost += w if M <= P else P * w / M
    succ = min(1.0, P / pk)
    print("  P=10^%5.2f  nodes per run 10^%.2f  success %.1e  expected nodes 2^%.1f  (tree 2^%.1f, 256*W_peak 2^%.1f)" % (lp, log10(cost), succ, log2(cost / succ), base * B, log2(256 * pk)))
print("\n== item 8: value of one extra correct pin, in bits (model, real census, five clues) ==")
gains = {}
for r in range(N):
    for c in range(N):
        if (r, c) in clue: continue
        kind = ("interior", "edge", "corner")[(r in (0, N - 1)) + (c in (0, N - 1))]
        pools = {"corner": 4, "edge": 56, "interior": 191}; pools[kind] -= 1
        fx = clue | {(r, c)}
        fixed_dir = tot(ORD["rows bottom-up"], fx, lpi, lpf, pools)[0]
        best_dir = min(tot(ORD[k], fx, lpi, lpf, pools)[0] for k in list(ORD)[:4])
        gains[(r, c)] = ((base - fixed_dir) * B, (min(four) - best_dir) * B, kind)
for j, lab in ((0, "scan fixed (rows bottom-up)"), (1, "best of four scan directions")):
    v = sorted(g[j] for g in gains.values())
    print("  %-30s best %.1f  median %.1f  mean %.1f  worst %.1f  | interior mean %.1f  edge mean %.1f  corner mean %.1f" % (lab, v[-1], v[len(v) // 2], sum(v) / len(v), v[0],
          *[sum(g[j] for g in gains.values() if g[2] == k) / sum(1 for g in gains.values() if g[2] == k) for k in ("interior", "edge", "corner")]))
print("  by row (scan fixed), mean bits:", " ".join("%.1f" % (sum(g[0] for (r, c), g in gains.items() if r == rr) / sum(1 for (r, c) in gains if r == rr)) for rr in range(N)))
# greedy: how many well-placed pins to remove 56.5 bits
fx = set(clue); pools = {"corner": 4, "edge": 56, "interior": 191}; cur = min(four); seq = []
while (min(four) - cur) * B < 66.4 and len(seq) < 40:
    best = None
    for r in range(N):
        for c in range(N):
            if (r, c) in fx: continue
            kind = ("interior", "edge", "corner")[(r in (0, N - 1)) + (c in (0, N - 1))]
            if pools[kind] == 0: continue
            p2 = dict(pools); p2[kind] -= 1
            v = min(tot(ORD[k], fx | {(r, c)}, lpi, lpf, p2)[0] for k in ("rows bottom-up", "rows top-down"))
            if best is None or v < best[0]: best = (v, (r, c), kind)
    cur = best[0]; fx.add(best[1]); pools[best[2]] -= 1; seq.append(((min(four) - cur) * B, best[1]))
print("  greedy best-placed pins (rows either way), cumulative bits:", " ".join("%.0f" % s[0] for s in seq))
print("  pins to pass 56.5 bits: %d ; 66.4 bits: %d" % (next(i + 1 for i, s in enumerate(seq) if s[0] >= 56.5), next((i + 1 for i, s in enumerate(seq) if s[0] >= 66.4), -1)))
