"""Count valid 2x2 blocks of distinct INNER pieces of the official set (all rotations),
against the first-moment expectation from the real inner colour counts.
A block is (A, B / C, D) with A.right=B.left, A.bottom=C.top, B.bottom=D.top, C.right=D.left.
Ordered: each block is counted in all 4 orientations of the block (the board fixes one).
Also reports the super-colour alphabet of the rims.
Run: python3 -I 01_blocks_2x2.py   (stdlib, a few seconds)"""
import os, sys, math
from collections import defaultdict, Counter
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "gap-5"))
from e2common import load, rots, klass  # noqa: E402

pieces, hints = load()
inner = [i for i, p in enumerate(pieces) if klass(p) == "inner"]
O = [(i, r) for i in inner for r in rots(pieces[i])]  # (tile, (t, r, b, l))
by_left = defaultdict(list); by_top = defaultdict(list); by_lt = defaultdict(list)
for i, (t, rr, b, l) in O:
    by_left[l].append((i, (t, rr, b, l)))
    by_top[t].append((i, (t, rr, b, l)))
    by_lt[(l, t)].append(i)

count = 0
rim = Counter()  # super-colour of the block's top rim (A.top, B.top)
for a, A in O:
    for b, B in by_left[A[1]]:
        if b == a:
            continue
        for c, C in by_top[A[2]]:
            if c == a or c == b:
                continue
            Ds = by_lt[(C[1], B[2])]
            n = len(Ds) - sum(1 for d in Ds if d in (a, b, c))
            count += n
            if n:
                rim[(A[0], B[0])] += n

# first-moment expectation with real colour counts: P(match) per joint = sum_c n_c^2 / N^2
sides = Counter(s for i in inner for s in pieces[i])
N = sum(sides.values())
q = sum(v * v for v in sides.values()) / N ** 2
exp = 196 * 195 * 194 * 193 * 4 ** 4 * q ** 4
print(f"inner pieces 196, oriented 784, inner colours {len(sides)}, sides {N}")
print(f"per-joint match probability q = {q:.6f} (1/17 = {1/17:.6f})")
print(f"valid ordered 2x2 blocks (4 orientations each): {count:,}  log10 {math.log10(count):.4f}")
print(f"  up to rotation of the block: {count/4:,.0f}")
print(f"first-moment expectation: {exp:,.0f}  ratio observed/expected {count/exp:.4f}")
print(f"distinct top-rim super-colours used: {len(rim)} of {17*17}")
# what a 7x7 super-tile puzzle on the interior would look like
print("interior 14x14 = 7x7 blocks; each block has 4 rims of 2 colours (289 super-colours)")
print(f"mean blocks per (left rim, top rim) constraint: {count / 289**2:.2f}")
print(f"mean blocks per top rim alone: {count / 289:.1f}")
