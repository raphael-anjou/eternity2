"""Second analysis script 3. Structure the memo may not have looked for, computed on the official tiles.
  1. side counts per colour, evenness, split by tile class
  2. the seam: the 56 inward sides of the edge tiles (the joints between rim and interior)
  3. the rim as a directed multigraph on the 5 frame colours (tiles = arcs)
  4. colour-pair graphs on the 17 interior colours: tiles as 4-cycles (adjacent sides) and as
     two diagonals (opposite sides), against 200 painted boards with the official counts
  5. parity per colour jointly with the checkerboard: the GF(2) system "which tiles sit on
     black cells"
  6. the five clues: cell colours, and what they add to the flux and checkerboard identities
Run: python3 -I r3_structure.py
"""
import sys, os, random
from collections import Counter
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rcommon import load, rots, tiles_from_joints, N

P, clues = load()
cnt = Counter(x for t in P for x in t)
print("== 1. sides per colour ==")
print("grey:", cnt[0], "| frame colours 1..5:", [cnt[c] for c in range(1, 6)],
      "| interior 6..22:", [cnt[c] for c in range(6, 23)])
print("every count even:", all(v % 2 == 0 for v in cnt.values()),
      "| frame colours on interior tiles:", sum(1 for t in P if 0 not in t for x in t if 1 <= x <= 5),
      "| interior colours on corner tiles:", sum(1 for t in P if t.count(0) == 2 for x in t if x > 5))
edges = [t for t in P if t.count(0) == 1]
corners = [t for t in P if t.count(0) == 2]
inner = [t for t in P if 0 not in t]

def up(t):
    g = t.count(0)
    for rt in rots(t):
        if rt[0] == 0 and (g == 1 or rt[1] == 0):
            return rt

print("== 2. seam ==")
seam = Counter(up(t)[2] for t in edges)
print("inward colours of the 56 edge tiles:", dict(sorted(seam.items())))
print("joints: rim-to-rim 60, seam (rim to interior) 56, interior-to-interior 364; total 480")
pool = [c for c in range(6, 23) for _ in range(cnt[c] // 2)]
rng = random.Random(3)
def chi(h):
    return sum((h.get(c, 0) - 56 * (cnt[c] / 2) / len(pool)) ** 2 / (56 * (cnt[c] / 2) / len(pool)) for c in range(6, 23))
obs = chi(seam)
sims = [chi(Counter(rng.sample(pool, 56))) for _ in range(5000)]
print(f"chi-square of the seam histogram against a uniform draw of 56 of the 420 interior joints: "
      f"{obs:.1f}; share of 5000 draws at least as large: {sum(s >= obs for s in sims) / 5000:.3f}; "
      f"interior colours absent from the seam: {[c for c in range(6, 23) if c not in seam]}")
print("== 3. rim digraph (arc = border tile, from the colour it is entered by to the colour it "
      "is left by, clockwise) ==")
arcs = Counter()
for t in edges:
    rt = up(t); arcs[(rt[3], rt[1])] += 1
for t in corners:
    rt = up(t); arcs[(rt[3], rt[2])] += 1
ind = Counter(); outd = Counter()
for (a, b), n in arcs.items():
    outd[a] += n; ind[b] += n
print("out-degree:", [outd[c] for c in range(1, 6)], "in-degree:", [ind[c] for c in range(1, 6)],
      "| balanced (needed for a closed rim):", all(ind[c] == outd[c] for c in range(1, 6)))
print("arc counts a->b, rows a = 1..5:", [[arcs[(a, b)] for b in range(1, 6)] for a in range(1, 6)])
print("== 4. colour-pair graphs, interior tiles ==")
def pair_stats(tiles):
    adj = Counter(); opp = Counter(); pat = Counter()
    for t in tiles:
        for i in range(4):
            adj[tuple(sorted((t[i], t[(i + 1) % 4])))] += 1
        opp[tuple(sorted((t[0], t[2])))] += 1; opp[tuple(sorted((t[1], t[3])))] += 1
        pat[tuple(sorted(Counter(t).values(), reverse=True))] += 1
    loops_adj = sum(n for (a, b), n in adj.items() if a == b)
    loops_opp = sum(n for (a, b), n in opp.items() if a == b)
    return dict(adj_pairs_absent=153 - len(adj), opp_pairs_absent=153 - len(opp),
                adj_loops=loops_adj, opp_loops=loops_opp, adj_max=max(adj.values()),
                opp_max=max(opp.values()), four_distinct=pat[(1, 1, 1, 1)], one_pair=pat[(2, 1, 1)],
                two_pairs=pat[(2, 2)], triple=pat[(3, 1)], four_equal=pat[(4,)])
off = pair_stats(inner)
ring_pool = [c for c in range(1, 6) for _ in range(cnt[c] // 2)]
null = []
for b in range(200):
    rp = ring_pool[:]; ip = pool[:]
    rng.shuffle(rp); rng.shuffle(ip)
    H = [[0] * (N - 1) for _ in range(N)]; V = [[0] * N for _ in range(N - 1)]
    for r in range(N):
        for c in range(N - 1):
            H[r][c] = rp.pop() if r in (0, N - 1) else ip.pop()
    for r in range(N - 1):
        for c in range(N):
            V[r][c] = rp.pop() if c in (0, N - 1) else ip.pop()
    T = tiles_from_joints(N, H, V)
    null.append(pair_stats([t for t in T if 0 not in t]))
print("statistic: official | painted boards min, mean, max | share of boards at or beyond official")
for k in off:
    xs = [d[k] for d in null]
    m = sum(xs) / len(xs)
    tail = min(sum(x >= off[k] for x in xs), sum(x <= off[k] for x in xs)) / len(xs)
    print(f"  {k}: {off[k]} | {min(xs)}, {m:.1f}, {max(xs)} | {tail:.3f}")
print("== 5. parity per colour with the checkerboard ==")
# unknown b_i = 1 if tile i sits on a black cell. Each joint has one black and one white side,
# so for each colour c: sum_i b_i * (sides of colour c on tile i) = cnt[c] / 2. Mod 2:
rows = []
for c in range(1, 23):
    v = 0
    for i, t in enumerate(P):
        if t.count(c) % 2:
            v |= 1 << i
    rows.append((v, (cnt[c] // 2) % 2))
piv = {}; consistent = True
for v, b in rows:
    while v:
        h = v.bit_length() - 1
        if h in piv:
            v ^= piv[h][0]; b ^= piv[h][1]
        else:
            piv[h] = (v, b); break
    else:
        if b: consistent = False
print(f"22 parity equations on 256 unknowns: rank {len(piv)}, consistent: {consistent}. "
      f"At most {len(piv)} bits of the 2^256 black/white assignments, and each equation is the "
      f"mod 2 reading of a checkerboard count the memo's test 1 already contains.")
print("== 6. clues ==")
for i, r, c, rt in clues:
    print(f"  tile {i + 1} at row {r} col {c} as {rt}: cell colour {'black' if (r + c) % 2 == 0 else 'white'}")
dv = Counter(); dh = Counter()
for i, r, c, rt in clues:
    dv[rt[2]] += 1; dv[rt[0]] -= 1; dh[rt[1]] += 1; dh[rt[3]] -= 1
print("  flux the other 251 tiles must cancel, vertical (south minus north) per colour:",
      {c: v for c, v in sorted(dv.items()) if v}, "| horizontal (east minus west):",
      {c: v for c, v in sorted(dh.items()) if v})
print("  clue tiles on black cells:", sum(1 for i, r, c, rt in clues if (r + c) % 2 == 0), "of 5")
