"""Sizes of the lift (theory memo, Appendix A formulas, no pin, uniform colours) and a first-moment
model of the row-scan tree for the same generated TOY boards, for the scaling table.

Row-scan model (derived): cells in row-major order; the expected number of valid partial fillings
of the first d cells is the product, over those cells, of the tiles of the cell's class still
unused (corner: 1 orientation, the first corner pinned; edge: 1 orientation; inner: 4 orientations)
times, for each edge closed, 1/cf if the edge runs along the frame ring and 1/ci otherwise. Nodes =
sum over d. The model ignores that the board is planted (at least one solution exists), which adds
at most a few nodes per level, and colour-count fluctuations.

Usage: python3 -I counts_and_tree.py
"""
import json, math


def c2(a):
    return a * (a - 1) // 2


def sizes(n, cf, ci):
    E, I = 4 * (n - 2), (n - 2) ** 2
    P = 16 + E * E + 4 * I * I
    same_cell = 4 * c2(4) + E * c2(E) + I * c2(4 * I)
    same_tile = 4 * c2(4) + E * c2(E) + I * (c2(4 * I) - 6 * I)
    adj = (2 * (n - 2) * (n - 3) * (16 * I * I - 16 * I) * (1 - 1 / ci) + 4 * (n - 2) * (4 * E * I) * (1 - 1 / ci)
           + 4 * (n - 3) * (E * E - E) * (1 - 1 / cf) + 32 * E * (1 - 1 / cf))
    Z = same_cell + same_tile + adj
    F = c2(P) - Z
    return dict(P=P, zeros=round(Z), free=round(F), zeros_over_P2=round(Z / P / P, 4), free_over_P2=round(F / P / P, 4),
                equalities_2n2=2 * n * n, face_order=P + 1 - (2 * n * n - 1))


def tree_log2(n, cf, ci):
    E, I = 4 * (n - 2), (n - 2) ** 2
    used = dict(c=0, e=0, i=0)
    lg = 0.0          # log2 of the expected count at the current depth
    tot = [0.0]       # log2 counts per depth
    for x in range(n * n):
        r, c = divmod(x, n)
        rim_r, rim_c = r in (0, n - 1), c in (0, n - 1)
        if rim_r and rim_c:
            ch = 1 if used["c"] == 0 else 4 - used["c"]; used["c"] += 1
        elif rim_r or rim_c:
            ch = E - used["e"]; used["e"] += 1
        else:
            ch = 4 * (I - used["i"]); used["i"] += 1
        lg += math.log2(ch)
        if c > 0:
            lg -= math.log2(cf if rim_r else ci)
        if r > 0:
            lg -= math.log2(cf if rim_c else ci)
        tot.append(lg)
    m = max(tot)
    nodes = m + math.log2(sum(2 ** (t - m) for t in tot))
    return dict(log2_nodes=round(nodes, 1), log2_peak=round(m, 1), peak_depth=tot.index(m), log2_expected_solutions=round(tot[-1], 1))


measured = {(5, 3, 4): "2^15.0 to 2^15.8 (lift memo, 3 boards)", (6, 4, 5): "2^20.9 to 2^21.8 (lift memo, 3 boards)",
            (7, 3, 7): "2^31.3 (gap-1 review, 1 board)", (8, 4, 11): "2^23 to 2^28 (run log 12:08, 20 boards)",
            (16, 5, 17): "2^134.4 with the five clues and the official colour counts (the plan document)"}
rows = []
for (n, cf, ci) in [(4, 2, 4), (5, 3, 4), (6, 4, 5), (7, 3, 7), (7, 4, 7), (8, 4, 11), (8, 5, 8), (10, 5, 9), (10, 5, 10), (12, 5, 12), (12, 5, 13), (16, 5, 17)]:
    d = dict(n=n, cf=cf, ci=ci); d.update(sizes(n, cf, ci)); d.update(tree_log2(n, cf, ci))
    d["measured_tree"] = measured.get((n, cf, ci))
    rows.append(d)
    print(json.dumps(d))
json.dump(rows, open(__file__.replace("counts_and_tree.py", "raw/counts_and_tree.json"), "w"), indent=1)
