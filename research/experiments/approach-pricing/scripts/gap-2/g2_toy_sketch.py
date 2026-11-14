"""Gap 2, script 2 (TOY boards, generated; not Eternity II): does any lossy summary of the used set give a table of the
bottom half that is both cheaper to build than the strict list and able to reject wrong top halves?
Board n x n, framed generator of the derivations second analysis (cb frame colours, ci interior colours). Cut after row k.
Top list: strict fillings of rows 0..k-1 (row scan, top down). Bottom: rows k..n-1 scanned from the bottom row up.
For each sketch s of the used set:
  strict-hash table = {(seam, s(used set))} over strict bottom fillings (what a hashed or Bloom-filtered list stores);
  sketch-DP table   = cells reached by a DP whose state is (frontier, s) only (tiles may repeat where s cannot see it).
A top filling (seam f, used A) passes if the cell (f, s(complement of A)) is present. True solution pairs always pass.
Reported: work (sum over depths of states), occupied cells, pass rate over wrong top fillings.
Run: python3 -I g2_toy_sketch.py [n cb ci k seed]"""
import random, sys, functools, hashlib, time
from collections import defaultdict
from math import log2, exp
print = functools.partial(print, flush=True)

def gen(n, cb, ci, rng):
    while True:
        H = [[0] * (n + 1) for _ in range(n)]; V = [[0] * n for _ in range(n + 1)]
        for r in range(n):
            for c in range(1, n):
                H[r][c] = rng.randrange(1, cb + 1) if r in (0, n - 1) else rng.randrange(cb + 1, cb + ci + 1)
        for r in range(1, n):
            for c in range(n):
                V[r][c] = rng.randrange(1, cb + 1) if c in (0, n - 1) else rng.randrange(cb + 1, cb + ci + 1)
        pcs = [(V[r][c], H[r][c + 1], V[r + 1][c], H[r][c]) for r in range(n) for c in range(n)]
        canon = set(); ok = True
        for p in pcs:
            rots = [p[i:] + p[:i] for i in range(4)]
            if len(set(rots)) < 4 or min(rots) in canon: ok = False; break
            canon.add(min(rots))
        if ok:
            rng.shuffle(pcs); return pcs

CAP = 4_000_000
def region(n, rows, pcs, flip, init, step):
    """Layered DP over `rows` rows from the rim inward. State (south colours per column, west colour, sketch value).
    flip: scan from the bottom rim (tiles read with N and S swapped). step(val, tile) -> new val or None.
    Returns (work = sum of state counts per depth, widest layer, final dict (seam, val) -> count)."""
    by = defaultdict(list)
    for i, p in enumerate(pcs):
        for k in range(4):
            t = p[k:] + p[:k]
            if flip: t = (t[2], t[1], t[0], t[3])
            by[(t[0], t[3])].append((i, t[1], t[2]))
    cur = {((0,) * n, 0, init): 1}; work = 0; widest = 0
    for r in range(rows):
        for c in range(n):
            nxt = defaultdict(int)
            for (cols, w, val), cnt in cur.items():
                for (i, e, s) in by[(cols[c] if r else 0, w if c else 0)]:
                    if s == 0 or (e == 0) != (c == n - 1): continue
                    v2 = step(val, i)
                    if v2 is None: continue
                    nxt[(cols[:c] + (s,) + cols[c + 1:], e, v2)] += cnt
            cur = nxt; work += len(cur); widest = max(widest, len(cur))
            if len(cur) > CAP: return work, widest, None
    fin = defaultdict(int)
    for (cols, w, val), cnt in cur.items(): fin[(cols, val)] += cnt
    return work, widest, fin

def main(n, cb, ci, k, seed):
    rng = random.Random(seed); pcs = gen(n, cb, ci, rng); nt = n * n; full = (1 << nt) - 1
    strict = lambda m, i: None if m >> i & 1 else m | 1 << i
    t0 = time.time()
    wa, pa, top = region(n, k, pcs, False, 0, strict)
    wb, pb, bot = region(n, n - k, pcs, True, 0, strict)
    if top is None or bot is None: print('TOY %dx%d seed %d: a strict list exceeds the cap of %d states per layer; skipped' % (n, n, seed, CAP)); return
    nsol = sum(cnt * bot.get((f, full ^ m), 0) for (f, m), cnt in top.items())
    seams = len({f for f, m in bot})
    print("TOY %dx%d, %d frame + %d interior colours, seed %d, cut after row %d: top list %d (work %d), bottom list %d distinct keys (work %d, widest layer %d), seams seen %d, solutions %d  [%.0fs]"
          % (n, n, cb, ci, seed, k, len(top), wa, len(bot), wb, pb, seams, nsol, time.time() - t0))
    wrong = [(f, m) for (f, m) in top if (f, full ^ m) not in bot]
    sizeb = nt - k * n
    def report(name, bits, sk, step, init):
        """sk(mask) = sketch of a strict used set; the DP reaches sk(B) for a strict bottom filling with set B."""
        cells_h = {(f, sk(m)) for (f, m) in bot}
        ph = sum((f, sk(full ^ m)) in cells_h for (f, m) in wrong) / max(1, len(wrong))
        if step is None:
            print("  %-34s %5.1f | hashed strict list: cells %8d (%.2f of list) pass %.4f | sketch DP: n/a" % (name, bits, len(cells_h), len(cells_h) / len(bot), ph)); return
        t0 = time.time(); wd, pd, fin = region(n, n - k, pcs, True, init, step)
        if fin is None:
            print("  %-34s %5.1f | hashed strict list: cells %8d (%.2f of list) pass %.4f | sketch DP: over the cap, work above %d (%.1f of strict) before the seam  [%.0fs]" % (name, bits, len(cells_h), len(cells_h) / len(bot), ph, wd, wd / wb, time.time() - t0))
            return wd / wb, 1.0 if False else None
        cells_d = set(fin)
        missing = len(cells_h - cells_d)
        pdp = sum((f, sk(full ^ m)) in cells_d for (f, m) in wrong) / max(1, len(wrong))
        print("  %-34s %5.1f | hashed strict list: cells %8d (%.2f of list) pass %.4f | sketch DP: work %9d (%.3f of strict) cells %9d (%.2f of list) pass %.4f%s  [%.0fs]"
              % (name, bits, len(cells_h), len(cells_h) / len(bot), ph, wd, wd / wb, len(cells_d), len(cells_d) / len(bot), pdp, "  UNSOUND: %d strict cells missing" % missing if missing else "", time.time() - t0))
        return wd / wb, pdp
    print("  sketch                             bits  |")
    res = []
    r2 = random.Random(1000 + seed)
    for b in (4, 8, 12, 16, 20, 24, 32):
        salt = r2.getrandbits(64)
        h = lambda m, b=b, salt=salt: int.from_bytes(hashlib.blake2b((m ^ salt << 64).to_bytes(24, "little"), digest_size=8).digest(), "little") & ((1 << b) - 1)
        report("random hash", b, h, None, None)
    for t in sorted({0, nt // 4, nt // 2, 3 * nt // 4, nt - 4, nt - 2, nt}):
        T = r2.sample(range(nt), t); tm = sum(1 << i for i in T)
        step = lambda v, i, tm=tm: (None if v >> i & 1 else v | 1 << i) if tm >> i & 1 else v
        res.append(("tracked", t, report("tracked subset of %d tiles" % t, t, lambda m, tm=tm: m & tm, step, 0)))
    for kc in sorted({2, 3, 4, 6, nt // 3, nt // 2}):
        perm = list(range(nt)); r2.shuffle(perm); cls = [0] * nt
        for j, i in enumerate(perm): cls[i] = j % kc
        cap = [sum(1 for i in range(nt) if cls[i] == j) for j in range(kc)]
        def sk(m, cls=cls, kc=kc):
            v = [0] * kc
            for i in range(nt):
                if m >> i & 1: v[cls[i]] += 1
            return tuple(v)
        def step(v, i, cls=cls, cap=cap):
            j = cls[i]
            if v[j] >= cap[j]: return None
            return v[:j] + (v[j] + 1,) + v[j + 1:]
        res.append(("classes", kc, report("%d classes, count per class" % kc, sum(log2(x + 1) for x in cap), sk, step, (0,) * kc)))
    for (q, rws) in ((2, 4), (2, 8), (2, 12), (2, 16), (2, 20), (257, 1), (257, 2), (65537, 2)):
        M = [[r2.randrange(q) for _ in range(nt)] for _ in range(rws)]
        vec = [tuple(M[j][i] for j in range(rws)) for i in range(nt)]
        def sk(m, vec=vec, q=q, rws=rws):
            v = [0] * rws
            for i in range(nt):
                if m >> i & 1:
                    for j in range(rws): v[j] = (v[j] + vec[i][j]) % q
            return tuple(v)
        step = lambda v, i, vec=vec, q=q: tuple((a + b) % q for a, b in zip(v, vec[i]))
        res.append(("linear", rws * log2(q), report("linear mod %d, %d rows" % (q, rws), rws * log2(q), sk, step, (0,) * rws)))
    res = [(fam, p, x) for fam, p, x in res if x and x[1] is not None]
    good = [(fam, p, x) for fam, p, x in res if x and x[0] < 0.5 and x[1] < 0.5]
    print("  sketch DPs with work under half the strict list's work AND pass rate under 0.5: %d %s" % (len(good), good))
    cheap = [x[1] for fam, p, x in res if x and x[0] < 0.5]; tight = [x[0] for fam, p, x in res if x and x[1] < 0.5]
    print("  among sketch DPs cheaper than half the strict work, lowest pass rate: %s ; among those with pass rate under 0.5, cheapest work ratio: %s"
          % ("%.4f" % min(cheap) if cheap else "none", "%.3f" % min(tight) if tight else "none"))

if __name__ == "__main__":
    a = [int(x) for x in sys.argv[1:]]
    main(*(a if a else (4, 3, 4, 2, 1)))
