"""Where the repeated pieces of a randomly painted set fall (corner, edge, interior).

Stdlib only:  python3 -I duplicates_by_kind.py

Paints a solved 16x16 board with the official joint-colour counts (60 ring
joints, 12 per frame colour; 420 interior joints, 24 for each of five interior
colours and 25 for each of twelve), shuffled uniformly within each pool, cuts
it into 256 pieces and counts pairs of pieces equal up to rotation, by kind.
Same model as research/topics/adversarial-piece-set/compute/null_model.py,
written independently here to split the count by kind.
"""
import random
from collections import Counter

N = 16
TRIALS = 20000
rng = random.Random(20261113)


def canon(p):
    return min(tuple(p[(i + k) % 4] for i in range(4)) for k in range(4))


tot = Counter()
none_any = none_int = 0
for _ in range(TRIALS):
    ring = [c for c in range(1, 6) for _ in range(12)]
    inner = [c for c in range(6, 11) for _ in range(24)] + [c for c in range(11, 23) for _ in range(25)]
    rng.shuffle(ring)
    rng.shuffle(inner)
    # h[r][c]: joint between (r,c) and (r,c+1); v[r][c]: joint between (r,c) and (r+1,c)
    h = [[None] * (N - 1) for _ in range(N)]
    v = [[None] * N for _ in range(N - 1)]
    ri = ii = 0
    for r in range(N):
        for c in range(N - 1):
            if r in (0, N - 1):
                h[r][c] = ring[ri]; ri += 1
            else:
                h[r][c] = inner[ii]; ii += 1
    for r in range(N - 1):
        for c in range(N):
            if c in (0, N - 1):
                v[r][c] = ring[ri]; ri += 1
            else:
                v[r][c] = inner[ii]; ii += 1
    assert ri == 60 and ii == 420
    groups = {0: Counter(), 1: Counter(), 2: Counter()}
    for r in range(N):
        for c in range(N):
            t = 0 if r == 0 else v[r - 1][c]
            b = 0 if r == N - 1 else v[r][c]
            l = 0 if c == 0 else h[r][c - 1]
            rt = 0 if c == N - 1 else h[r][c]
            p = (t, rt, b, l)
            groups[p.count(0)][canon(p)] += 1
    pairs = {k: sum(m * (m - 1) // 2 for m in g.values()) for k, g in groups.items()}
    for k in pairs:
        tot[k] += pairs[k]
    if sum(pairs.values()) == 0:
        none_any += 1
    if pairs[0] == 0:
        none_int += 1
print("trials %d" % TRIALS)
print("mean rotation-duplicate pairs: interior %.2f, edge %.2f, corner %.2f, total %.2f"
      % (tot[0] / TRIALS, tot[1] / TRIALS, tot[2] / TRIALS, sum(tot.values()) / TRIALS))
print("share of painted sets with no duplicate among the 196 interior pieces: %.3f" % (none_int / TRIALS))
print("share of painted sets with no duplicate among all 256 pieces: %.4f" % (none_any / TRIALS))
