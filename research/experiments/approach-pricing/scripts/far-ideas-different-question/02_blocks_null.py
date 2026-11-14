"""Null for script 01: the 2x2 block count of the inner pieces of boards painted with the
exchangeable painter (official colour pools, duplicate and rotation-symmetric pieces
rejected, as the official set has none). Is 4,059,952 unusual?
Run: python3 -I 02_blocks_null.py [n_boards]   (stdlib, about 1 s per board)"""
import os, sys, random, math
from collections import defaultdict
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "gap-5"))
from e2common import load, rots, klass, canon, paint, official_pools, N  # noqa: E402


def blocks(inner_pieces):
    O = [(i, r) for i, p in enumerate(inner_pieces) for r in rots(p)]
    bl = defaultdict(list); bt = defaultdict(list); blt = defaultdict(list)
    for i, (t, rr, b, l) in O:
        bl[l].append((i, (t, rr, b, l))); bt[t].append((i, (t, rr, b, l))); blt[(l, t)].append(i)
    n = 0
    for a, A in O:
        for b, B in bl[A[1]]:
            if b == a:
                continue
            for c, C in bt[A[2]]:
                if c == a or c == b:
                    continue
                Ds = blt[(C[1], B[2])]
                n += len(Ds) - sum(1 for d in Ds if d in (a, b, c))
    return n


pieces, _ = load()
off = blocks([p for p in pieces if klass(p) == "inner"])
ring, innerpool = official_pools(pieces)
rng = random.Random(20261113)
nb = int(sys.argv[1]) if len(sys.argv) > 1 else 60
vals = []
while len(vals) < nb:
    bd = paint(rng, ring, innerpool)
    ps = [bd[r][c] for r in range(N) for c in range(N)]
    cs = [canon(p) for p in ps]
    if len(set(cs)) < 256 or any(len(set(rots(p))) < 4 for p in ps if klass(p) == "inner"):
        continue
    vals.append(blocks([p for p in ps if klass(p) == "inner"]))
m = sum(vals) / len(vals)
sd = math.sqrt(sum((v - m) ** 2 for v in vals) / (len(vals) - 1))
below = sum(v <= off for v in vals)
print(f"official {off:,}; null n={len(vals)} mean {m:,.0f} sd {sd:,.0f}; z {(off-m)/sd:.2f}; "
      f"null boards at or below official {below}/{len(vals)}; min {min(vals):,} max {max(vals):,}")
