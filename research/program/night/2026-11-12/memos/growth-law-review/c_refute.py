"""Claim C, second half: can a check that ignores piece distinctness refute nodes before the peak?

python3 -I c_refute.py
Whole row-major trees of committed instances. For every node, three lookahead depths
L = 1, n, 2n cells: `strict` = no consistent descendant L cells deeper (what plain search finds
out by itself), `relaxed` = no filling of the next L cells even when a piece may be reused
inside the lookahead (a distinctness-free check). The peak is the depth with most nodes.
Also the Galton-Watson estimate: a node whose cells ahead have b expected candidates each is
refutable by a distinctness-free check with the extinction probability q = exp(b (q - 1)).
"""
import math, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rvlib as rv


def extinction(b):
    q = 0.0
    for _ in range(10000):
        q = math.exp(b * (q - 1.0))
    return q


for ladder, n, k in (("primary", 6, 30), ("plus2", 7, 30)):
    inst = [(a, b, p) for a, b, p, _ in rv.ladder_instances(ladder, n)][:k]
    res = rv.run_enum("census", 10**10, inst)
    N = n * n
    c = int(inst[0][0].split("-")[1][1:])
    Z = [sum(r["Z"][d] for r in res) for d in range(N + 1)]
    peak = max(range(1, N + 1), key=lambda d: Z[d])
    print("%dx%d c=%d, %d committed instances, %d nodes, peak of the node count at depth %d of %d" % (n, n, c, len(res), sum(Z), peak, N))
    for name, lo, hi in (("before the peak (depth < peak - n)", 1, peak - n), ("around the peak (peak - n .. peak)", peak - n, peak + 1), ("after the peak", peak + 1, N + 1)):
        tot = sum(Z[lo:hi])
        row = []
        for i, L in enumerate((1, n, 2 * n)):
            s = sum(sum(r["strict%d" % i][lo:hi]) for r in res)
            x = sum(sum(r["relaxed%d" % i][lo:hi]) for r in res)
            row.append("L=%d strict %.3f relaxed %.3f" % (L, s / tot, x / tot))
        print("   %s: %d nodes; share refuted: %s" % (name, tot, "; ".join(row)))
    print("   by depth (interior cells with two filled neighbours only): depth, nodes, pieces left m, b = 4m/c^2, share with no child, Galton-Watson q(b)")
    w = n - 2
    for d in range(n + 1, peak + 2):
        x_, y_ = d % n, d // n          # next cell to fill has index d
        if not (1 < x_ < n - 1 and 1 <= y_ < n - 1):
            continue
        m = w * w - ((y_ - 1) * w + (x_ - 1))
        b = 4.0 * m / (c * c)
        dead = sum(r["strict0"][d] for r in res)
        print("      %d %d %d %.2f %.3f %.3f" % (d, Z[d], m, b, dead / Z[d], extinction(b)))
