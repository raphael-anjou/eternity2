"""Review item 2, toy check of the lookahead lemma (generated boards, not Eternity II).
For each sketch DP of the bottom region and each row boundary inside it (bottom layer j, depth D = n*n - j of the top-down scan):
  theta = (states of the sketch DP at layer j) / (states of the strict DP at layer j)
  g     = share of the strict top-down nodes at depth D whose ancestor at the cut passes the sketch table
Lemma: g >= 1 - theta, up to sampling noise. Reported: every (sketch, layer) pair, and the count of violations by more than 0.05.
Uses gen() from the memo's toy script, read only. Run: python3 -I r6_toy_lookahead.py"""
import sys, os, random, time
from collections import defaultdict
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "gap-2"))
from g2_toy_sketch import gen
CAP = 3_000_000
def table(n, pcs, flip):
    by = defaultdict(list)
    for i, p in enumerate(pcs):
        for k in range(4):
            t = p[k:] + p[:k]
            if flip: t = (t[2], t[1], t[0], t[3])
            by[(t[0], t[3])].append((i, t[1], t[2]))
    return by
def bottom(n, rows, pcs, init, step):
    """sketch DP from the bottom rim; returns states per row boundary and the final cell set"""
    by = table(n, pcs, True); cur = {((0,) * n, 0, init)}; per = []
    for r in range(rows):
        for c in range(n):
            nxt = set()
            for (cols, w, val) in cur:
                for (i, e, s) in by[(cols[c] if r else 0, w if c else 0)]:
                    if s == 0 or (e == 0) != (c == n - 1): continue
                    v2 = step(val, i)
                    if v2 is None: continue
                    nxt.add((cols[:c] + (s,) + cols[c + 1:], e, v2))
            cur = nxt
            if len(cur) > CAP: return None, None
        per.append(len(cur))
    return per, {(cols, val) for (cols, w, val) in cur}
def topdown(n, k, pcs):
    """strict top-down scan; returns the nodes at the cut (seam, mask) and, per later row boundary, the multiset of cut ancestors"""
    by = table(n, pcs, False); cur = [((0,) * n, 0, 0, -1)]; cutnodes = []; anc = {}
    for r in range(n - 1):
        for c in range(n):
            nxt = []
            for (cols, w, m, a) in cur:
                for (i, e, s) in by[(cols[c] if r else 0, w if c else 0)]:
                    if s == 0 or (e == 0) != (c == n - 1) or m >> i & 1: continue
                    nxt.append((cols[:c] + (s,) + cols[c + 1:], e, m | 1 << i, a))
            cur = nxt
            if len(cur) > CAP: return cutnodes, anc
        if r == k - 1:
            cutnodes = [(cols, m) for (cols, w, m, a) in cur]; cur = [(cols, w, m, idx) for idx, (cols, w, m, a) in enumerate(cur)]
        elif r >= k:
            anc[(r + 1) * n] = [a for (_, _, _, a) in cur]
    return cutnodes, anc
viol = tot = 0; rows_out = []
t0 = time.time()
for (n, cb, ci, k, seed) in [(5, 3, 4, 2, s) for s in (1, 2, 3, 7, 8, 9)] + [(5, 2, 4, 2, s) for s in (1, 2)] + [(6, 3, 6, 3, 11)]:
    if time.time() - t0 > 600: break
    rng = random.Random(seed); pcs = gen(n, cb, ci, rng); nt = n * n; full = (1 << nt) - 1
    strict = lambda m, i: None if m >> i & 1 else m | 1 << i
    sper, _ = bottom(n, n - k, pcs, 0, strict)
    cutnodes, anc = topdown(n, k, pcs)
    if sper is None or not anc: print("%dx%d seed %d skipped (cap)" % (n, n, seed)); continue
    r2 = random.Random(1000 + seed); sketches = []
    for t in sorted({nt // 4, nt // 2, 3 * nt // 4, nt - 4, nt - 2}):
        T = r2.sample(range(nt), t); tm = sum(1 << i for i in T)
        sketches.append(("tracked %d" % t, 0, (lambda v, i, tm=tm: (None if v >> i & 1 else v | 1 << i) if tm >> i & 1 else v), (lambda m, tm=tm: m & tm)))
    for kc in (2, 4, nt // 3):
        perm = list(range(nt)); r2.shuffle(perm); cls = [0] * nt
        for j, i in enumerate(perm): cls[i] = j % kc
        cap = [cls.count(j) for j in range(kc)]
        def sk(m, cls=cls, kc=kc):
            v = [0] * kc
            for i in range(nt):
                if m >> i & 1: v[cls[i]] += 1
            return tuple(v)
        def step(v, i, cls=cls, cap=cap):
            j = cls[i]
            return None if v[j] >= cap[j] else v[:j] + (v[j] + 1,) + v[j + 1:]
        sketches.append(("classes %d" % kc, (0,) * kc, step, sk))
    for rws in (4, 8, 12, 16):
        vec = [r2.getrandbits(rws) for _ in range(nt)]
        def sk(m, vec=vec):
            v = 0
            for i in range(nt):
                if m >> i & 1: v ^= vec[i]
            return v
        sketches.append(("linear GF(2) %d bits" % rws, 0, (lambda v, i, vec=vec: v ^ vec[i]), sk))
    for name, init, step, sk in sketches:
        per, cells_ = bottom(n, n - k, pcs, init, step)
        if per is None: print("  %dx%d seed %d %-20s over the cap" % (n, n, seed, name)); continue
        passes = [(f, sk(full ^ m)) in cells_ for (f, m) in cutnodes]
        prate = sum(passes) / len(passes)
        for rb in range(1, n - k):          # bottom layer j = rb rows ; depth D = nt - rb*n
            j = rb * n; D = nt - j
            if D not in anc or not anc[D]: continue
            theta = per[rb - 1] / sper[rb - 1]; g = sum(passes[a] for a in anc[D]) / len(anc[D])
            tot += 1; bad = g < 1 - theta - 0.05; viol += bad
            rows_out.append((theta, g))
            print("  %dx%d %d+%d seed %2d %-20s pass rate %.3f | bottom layer %2d: theta %.3f, 1-theta %.3f, share g of depth-%d nodes kept %.3f %s" % (n, n, cb, ci, seed, name, prate, j, theta, max(0, 1 - theta), D, g, "VIOLATION" if bad else ""))
print("pairs tested %d, violations of g >= 1 - theta - 0.05: %d  [%.0fs]" % (tot, viol, time.time() - t0))
lo = [(t, g) for t, g in rows_out if t < 0.5]
print("pairs with theta under 0.5 (the DP holds under half the strict states): %d ; smallest g among them %.3f" % (len(lo), min(g for t, g in lo) if lo else float("nan")))
