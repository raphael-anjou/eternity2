"""Distinctness fraction: a local-plus-birthday law against the committed exact counts and a strip Monte Carlo;
a constructive positivity proof for the reusable-tile entropy via toroidal 3x3 blocks."""
import json, math, os, random, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from collections import Counter, defaultdict
from e2lib import load, kind, ROOT
_, P, _ = load()
rot = lambda p, r: tuple(p[(i - r) % 4] for i in range(4))
INT = [p for p in P if kind(p) == "interior"]
OR = [(i, rot(p, r)) for i, p in enumerate(INT) for r in range(4)]     # (piece id, (top,right,bottom,left))

print("== committed block counts ==")
d = json.load(open(os.path.join(ROOT, "research", "topics", "entropy-area-law", "results", "entropy.json")))
rho = {b["n"]: b["distinct"] / b["reusable"] for b in d["blockCounts"] if b["distinct"]}
for nn in (2, 3):
    m = nn * nn; adj = 2 * nn * (nn - 1); L = -math.log(rho[nn])
    print(f"  n={nn}: rho={rho[nn]:.6f}  -ln rho={L:.5f}  alpha_n=-ln rho/n^2={L / m:.4f}  birthday term C(m,2)/196={m * (m - 1) / 392:.5f}"
          f"  residual per adjacent pair=({L:.5f}-{m * (m - 1) / 392:.5f})/{adj}={(L - m * (m - 1) / 392) / adj:.5f}")
a2 = (-math.log(rho[2]) - 12 / 392) / 4; a3 = (-math.log(rho[3]) - 72 / 392) / 12
# two-parameter solve: 4a + 6k/196 = L2 ; 12a + 36k/196 = L3
L2, L3 = -math.log(rho[2]), -math.log(rho[3])
k = (L3 - 3 * L2) / (18 / 196); a = (L2 - 6 * k / 196) / 4
print(f"  two-parameter fit -ln rho = a*(adjacent pairs) + k*C(m,2)/196: a={a:.5f}, k={k:.4f} (k=1 is the birthday prediction)")
a = (a2 + a3) / 2
print(f"  one-parameter law with k=1: a from n=2 is {a2:.5f}, from n=3 is {a3:.5f}")
print("  predicted adjacent-repeat excess: P(neighbour is the same piece) ~ (1 + 3/17)/46.1 =", round((1 + 3 / 17) / 46.1, 4), " minus baseline 1/196 =", round((1 + 3 / 17) / 46.1 - 1 / 196, 4))
for nn in range(2, 14):
    m = nn * nn; L = a * 2 * nn * (nn - 1) + m * (m - 1) / 392
    print(f"    n={nn:2d} cells={m:3d}: local+birthday rho={math.exp(-L):.3e}   page's exp(-0.0445 n^2)={math.exp(-0.0445 * m):.3e}")
lo, hi = 1.0, 20.0
for _ in range(60):
    mid = (lo + hi) / 2; m = mid * mid
    if a * 2 * mid * (mid - 1) + m * (m - 1) / 392 < math.log(1000): lo = mid
    else: hi = mid
print(f"  rho < 1e-3 at n = {lo:.2f}, that is {lo * lo:.0f} cells (the page extrapolates 155 cells)")

print("== 1 x L strips: exact sampling of uniform colour-valid chains, fraction with all pieces distinct ==")
byW = defaultdict(list)
for pid, q in OR: byW[q[3]].append((pid, q[1]))
rng = random.Random(5)
Lmax = 40
msg = [{c: 1 for c in range(6, 23)}]
for _ in range(Lmax): msg.append({c: sum(msg[-1][e] for _, e in byW[c]) for c in range(6, 23)})
def sample(L):
    # first tile: any oriented tile, weight = completions of length L-1 from its east colour
    tiles = [(pid, q[1]) for pid, q in OR]
    pid, e = rng.choices(tiles, weights=[msg[L - 1][x[1]] for x in tiles])[0]
    seen = {pid}; ok = True; adjrep = 0
    for k2 in range(L - 2, -1, -1):
        cand = byW[e]; prev = pid
        pid, e = rng.choices(cand, weights=[msg[k2][x[1]] for x in cand])[0]
        adjrep += pid == prev
        if pid in seen: ok = False
        seen.add(pid)
    return ok, adjrep
S = 20000
for L in (2, 5, 10, 15, 20, 30, 40):
    r = [sample(L) for _ in range(S)]
    f = sum(o for o, _ in r) / S; ar = sum(x for _, x in r) / (S * (L - 1))
    b1 = (-math.log(f) - L * (L - 1) / 392) / (L - 1)
    print(f"  L={L:2d}: distinct fraction {f:.4f} (-ln {-math.log(f):.4f}); birthday alone {L * (L - 1) / 392:.4f}; residual per adjacent pair {b1:.4f}; P(adjacent repeat) {ar:.4f}")

print("== toroidal 3x3 blocks of oriented interior tiles (rows cyclic, columns cyclic) ==")
byWl = defaultdict(list)
for t, (pid, q) in enumerate(OR): byWl[q[3]].append(t)
rows = []
for t1, (p1, q1) in enumerate(OR):
    for t2 in byWl[q1[1]]:
        q2 = OR[t2][1]
        for t3 in byWl[q2[1]]:
            q3 = OR[t3][1]
            if q3[1] == q1[3]: rows.append((t1, t2, t3))
print("  cyclic rows of length 3:", len(rows), "(mean-field 784^3/17^3 =", round(784 ** 3 / 17 ** 3), ")")
top = defaultdict(list)
for r in rows: top[tuple(OR[t][1][0] for t in r)].append(r)
bot = lambda r: tuple(OR[t][1][2] for t in r)
blocks = defaultdict(list)
for r1 in rows:
    for r2 in top.get(bot(r1), ()):
        for r3 in top.get(bot(r2), ()):
            if bot(r3) == tuple(OR[t][1][0] for t in r1):
                sig = (OR[r1[0]][1][3], OR[r2[0]][1][3], OR[r3[0]][1][3]) + tuple(OR[t][1][0] for t in r1)
                blocks[sig].append((r1, r2, r3))
nb = sum(len(v) for v in blocks.values())
coll = {s: v for s, v in blocks.items() if len(v) >= 2}
print("  toroidal 3x3 blocks:", nb, "(mean-field A(3)/17^6 =", round(195941569440 / 17 ** 6), "); boundary signatures shared by >= 2 blocks:", len(coll),
      "; largest class:", max((len(v) for v in blocks.values()), default=0))
if coll:
    s, v = max(coll.items(), key=lambda kv: len(kv[1]))
    print("  example signature (west colours of the 3 rows, north colours of the 3 columns):", s)
    for b in v[:3]: print("    block:", [[OR[t] for t in r] for r in b])
    print(f"  => any arrangement of these {len(v)} blocks on a 3x3 lattice is a valid plane tiling, so h_inf >= log10({len(v)})/9 = {math.log10(len(v)) / 9:.4f} > 0 (rigorous)")
else:
    print("  no collision at 3x3: positivity not proved by this construction")
