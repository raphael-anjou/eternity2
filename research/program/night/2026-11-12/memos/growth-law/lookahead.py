"""Lookahead ("erosion") variant of the model, for propagating solvers.

Stdlib only. Run: python3 -I lookahead.py
Assumption (not derived): a solver whose propagation sees L cells ahead counts,
at depth d, about min_{0<=l<=L} H_j(d+l) nodes in each hanging tree, where the
cells are taken in a greedy fail-first static order (next cell = smallest
expected number of candidates b, ties by row-major index). First-solution cost
= N + (1/2) sum_j W_j^L, valid above the critical colour count only.
"""
import math, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from model import Profile, cell_class, edge_kind, neighbours, order_rowmajor, order_border_first


def greedy_order(n, c, cf=5):
    left = {"corner": 4, "edge": 4 * (n - 2), "interior": (n - 2) ** 2}
    placed, order = set(), []
    cells = order_rowmajor(n)
    while len(order) < n * n:
        best = None
        for cell in cells:
            if cell in placed:
                continue
            cls = cell_class(n, *cell)
            lb = math.log(left[cls] * (4 if cls == "interior" else 1))
            for nb in neighbours(n, *cell):
                if nb in placed:
                    lb -= math.log(cf if edge_kind(n, cell, nb) == "F" else c)
            if best is None or lb < best[0] - 1e-12:
                best = (lb, cell)
        order.append(best[1])
        placed.add(best[1])
        left[cell_class(n, *best[1])] -= 1
    return order


def first_cost(p, L):
    N = p.N
    tot = float(N)
    for j in range(1, N):
        if p.lnwrong[j] == -math.inf:
            continue
        for d in range(j + 1, N + 1):
            tot += 0.5 * math.exp(min(p.lnH(j, min(d + l, N)) for l in range(L + 1)))
    return tot


MEASURED = {  # KM medians computed by medians.py (bb-mrv, scb), 30 seeds where available
    (6, 5): (400, 951), (6, 7): (57, 106), (7, 6): (588305, 921773), (7, 8): (3218, 7088), (8, 10): (69036, 555342),
}

if __name__ == "__main__":
    print("n c | rowmajor L=0 | greedy order, L = 0 1 2 3 4 6 8 | border-first L=0, L=4 | measured bb-mrv, scb")
    for (n, c) in list(MEASURED) + [(9, 11), (10, 12)]:
        pr = Profile(n, c, "rowmajor")
        pg = Profile(n, c, greedy_order(n, c))
        pb = Profile(n, c, order_border_first(n))
        row = ["%.3g" % first_cost(pg, L) for L in (0, 1, 2, 3, 4, 6, 8)]
        print(n, c, "| %.3g |" % first_cost(pr, 0), " ".join(row), "| %.3g %.3g |" % (first_cost(pb, 0), first_cost(pb, 4)), MEASURED.get((n, c)))


# ---------------------------------------------------------------- frontier forward checking
def frontier_thinning(n, c, order, cf=5):
    """ln Phi(d), d = 0..N: probability that every empty cell with a placed
    neighbour, other than the next cell of the order, has at least one
    candidate, for a generic partial board on the first d cells. Candidates of
    a cell are taken Poisson with mean lambda_v = (unused pieces of its class)
    * rotations * prod(match probabilities); cells are taken independent."""
    left = {"corner": 4, "edge": 4 * (n - 2), "interior": (n - 2) ** 2}
    placed = set()
    out = []
    for d in range(n * n + 1):
        nxt = order[d] if d < n * n else None
        ln = 0.0
        seen = set()
        for cell in placed:
            for v in neighbours(n, *cell):
                if v in placed or v == nxt or v in seen:
                    continue
                seen.add(v)
                cls = cell_class(n, *v)
                lam = left[cls] * (4 if cls == "interior" else 1)
                for nb in neighbours(n, *v):
                    if nb in placed:
                        lam /= cf if edge_kind(n, v, nb) == "F" else c
                ln += math.log(max(1.0 - math.exp(-lam), 1e-300))
        out.append(ln)
        if nxt is not None:
            placed.add(nxt)
            left[cell_class(n, *nxt)] -= 1
    return out


def first_cost_fc(n, c, order):
    p = Profile(n, c, order)
    phi = frontier_thinning(n, c, order)
    N = p.N
    tot = float(N)
    for j in range(1, N):
        if p.lnwrong[j] == -math.inf:
            continue
        tot += 0.5 * math.exp(p.lnH(j, j + 1))
        for d in range(j + 2, N + 1):
            tot += 0.5 * math.exp(p.lnH(j, d) + phi[d - 1])
    return tot


if __name__ == "__main__":
    print()
    print("frontier forward checking: n c | rowmajor | greedy | border-first | diagonal | measured bb-mrv, scb")
    from model import order_diagonal
    for (n, c) in list(MEASURED) + [(9, 11), (10, 12)]:
        print(n, c, "| %.3g | %.3g | %.3g | %.3g |" % (
            first_cost_fc(n, c, order_rowmajor(n)), first_cost_fc(n, c, greedy_order(n, c)),
            first_cost_fc(n, c, order_border_first(n)), first_cost_fc(n, c, order_diagonal(n))), MEASURED.get((n, c)))
