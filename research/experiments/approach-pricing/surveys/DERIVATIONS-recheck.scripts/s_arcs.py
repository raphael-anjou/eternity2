"""Item 5: how many arrangements does a freed border arc really have? Real edge pieces of the official set.
An edge piece is an arc (colour on its counter-clockwise side -> colour on its clockwise side). A valid run of pieces is a
trail. Free m cells of a valid run, keep the m pieces: count the orderings that still chain between the same two end colours
(inward colours ignored: nothing inside is assumed). Compare with the survey's m!/5^(m+1).
Run: python3 -I s_arcs.py <engine/data/official_eternity2.csv>"""
import sys, random, functools
from math import lgamma, log, log2, factorial
from collections import Counter
rng = random.Random(5)
rows = [l.strip() for l in open(sys.argv[1]) if l.strip()]
arcs = []
for l in rows[1:]:
    p = [0 if int(x, 2) == 65535 else int(x, 2) for x in l.split(",")[:4]]
    z = [i for i, v in enumerate(p) if v == 0]
    if len(z) == 1: arcs.append((p[(z[0] + 3) % 4], p[(z[0] + 1) % 4]))
assert len(arcs) == 56
tc = Counter(arcs)
print("56 edge pieces, %d distinct (ccw, cw) colour pairs of 25; pairs of pieces with the same frame colours: %d (model expectation 1540/25 = 61.6)" % (len(tc), sum(v * (v - 1) // 2 for v in tc.values())))

def chain(L):
    while True:
        left = list(range(56)); rng.shuffle(left); seq = [left.pop()]
        while len(seq) < L:
            cand = [i for i in left if arcs[i][0] == arcs[seq[-1]][1]]
            if not cand: break
            x = rng.choice(cand); left.remove(x); seq.append(x)
        if len(seq) == L: return seq

def count(segments, pieces):
    """segments: list of (start colour, end colour, length). pieces: indices. Number of labelled fillings."""
    types = sorted(set(arcs[i] for i in pieces)); mult = Counter(arcs[i] for i in pieces)
    start = tuple(mult[t] for t in types)
    @functools.lru_cache(maxsize=None)
    def rec(cnt, s, pos, col):
        if s == len(segments): return 1
        a, b, ln = segments[s]
        if pos == 0: col = a
        tot = 0
        for j, t in enumerate(types):
            if cnt[j] and t[0] == col:
                if pos == ln - 1:
                    if t[1] != b: continue
                    tot += rec(cnt[:j] + (cnt[j] - 1,) + cnt[j + 1:], s + 1, 0, 0)
                else:
                    tot += rec(cnt[:j] + (cnt[j] - 1,) + cnt[j + 1:], s, pos + 1, t[1])
        return tot
    r = rec(start, 0, 0, 0)
    for v in mult.values(): r *= factorial(v)
    return r
lf2 = lambda k: lgamma(k + 1) / log(2)
print("\none contiguous freed arc of m cells (200 random valid runs each)")
print("  m | survey log2(m!/5^(m+1)) | measured log2(count): median  mean  min  max | share with a single arrangement")
for m in (2, 3, 4, 6, 8, 10, 12, 13, 16, 20, 24, 30):
    v = []
    for _ in range(200 if m <= 20 else 40):
        seq = chain(m)
        v.append(log2(count([(arcs[seq[0]][0], arcs[seq[-1]][1], m)], seq)))
    v.sort()
    print(" %2d | %6.1f | %5.1f %5.1f %5.1f %5.1f | %.2f" % (m, lf2(m) - (m + 1) * log2(5), v[len(v) // 2], sum(v) / len(v), v[0], v[-1], sum(1 for x in v if x == 0) / len(v)))
print("\nscattered: 30 freed cells in 10 arcs of 3, one kept piece between arcs (survey: 2^14.8)")
v = []
for _ in range(30):
    seq = chain(40)
    segs = []; free = []
    for a in range(10):
        blk = seq[4 * a:4 * a + 3]; free += blk
        segs.append((arcs[blk[0]][0], arcs[blk[-1]][1], 3))
    v.append(log2(count(segs, free)))
v.sort()
print("  measured log2(count): median %.1f mean %.1f min %.1f max %.1f" % (v[len(v) // 2], sum(v) / len(v), v[0], v[-1]))
print("scattered: 15 freed cells in 5 arcs of 3 (survey form 15!/5^20 = 2^%.1f)" % (lf2(15) - 20 * log2(5)))
v = []
for _ in range(60):
    seq = chain(20); segs = []; free = []
    for a in range(5):
        blk = seq[4 * a:4 * a + 3]; free += blk; segs.append((arcs[blk[0]][0], arcs[blk[-1]][1], 3))
    v.append(log2(count(segs, free)))
v.sort()
print("  measured log2(count): median %.1f mean %.1f min %.1f max %.1f" % (v[len(v) // 2], sum(v) / len(v), v[0], v[-1]))
