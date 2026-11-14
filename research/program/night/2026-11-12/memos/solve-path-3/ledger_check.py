"""Conditional-collapse checks on small planted boards (exact enumeration).

Run: python3 -I ledger_check.py N FRAME_COLOURS INTERIOR_COLOURS SEEDS [node_cap]
(a) frame given: the planted frame is pinned, the interior is enumerated row-major.
(b) half given: the top half may only use the pieces the planted solution puts there.
Both against the first-moment model, and against the model's free-pool counts.
"""
import sys
from collections import Counter
from math import log10

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from model import interior_match_logp, frame_match_logp, profile, cell_kind, logsum10
from small_check import gen, colour_stats, Solver


def main():
    n, cf, ci = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3])
    seeds = range(1, int(sys.argv[4]) + 1)
    cap = int(float(sys.argv[5])) if len(sys.argv) > 5 else 3_000_000
    h = n // 2
    td = [(r, c) for r in range(n) for c in range(n)]
    kinds = Counter(cell_kind(n, r, c) for r, c in td)
    frame_cells = [x for x in td if cell_kind(n, *x) != "interior"]
    inner = [x for x in td if cell_kind(n, *x) == "interior"]
    nf = len(frame_cells)
    print("board %dx%d, %d frame colours, %d interior colours, 4*n_int/q^2=%.2f"
          % (n, n, cf, ci, 4 * kinds["interior"] / ci ** 2))
    out = []
    for seed in seeds:
        pieces, where = gen(n, cf, ci, seed)
        icnt, fcnt = colour_stats(n, pieces, cf)
        lpi = interior_match_logp(icnt, sum(icnt) // 2)
        lpf = frame_match_logp(fcnt, sum(fcnt))
        pools = dict(kinds)
        free = [x[0] for x in profile(n, td, pools, lpi, lpf)]
        fr = [x[0] for x in profile(n, frame_cells + inner, pools, lpi, lpf)]
        given = [x - fr[nf - 1] for x in fr[nf:]]
        # (a) frame given
        cell_of = {cell: pid for pid, cell in where.items()}
        frame_ids = {cell_of[r * n + c] for r, c in frame_cells}
        allowed = set(range(n * n)) - frame_ids
        s = Solver(n, pieces, inner, allowed=allowed, preset_frame=(where, frame_ids))
        try:
            ca = s.run(len(inner), cap=cap)
            a_nodes, a_peak = sum(ca), max(ca)
        except OverflowError:
            a_nodes = a_peak = float("nan")
        # (b) half given
        top_ids = {pid for pid, cell in where.items() if cell < h * n}
        cb = Solver(n, pieces, td, allowed=top_ids).run(h * n)
        top_pools = dict(Counter(cell_kind(n, r, c) for r, c in td[:h * n]))
        mb = [x[0] for x in profile(n, td[:h * n], top_pools, lpi, lpf)]
        row = (seed, logsum10(free), max(free), a_nodes, a_peak, 10 ** logsum10(given), 10 ** max(given),
               sum(cb), max(cb), 10 ** logsum10(mb), 10 ** max(mb), free[h * n - 1], logsum10(free[:h * n]))
        out.append(row)
        print("seed %d: free DFS model log10 nodes %.2f (peak %.2f) | frame given: nodes %s peak %s "
              "(model %.3g, %.3g) | planted top pool: nodes %d peak %d (model %.3g, %.3g; "
              "free top half model log10 fills %.2f, log10 nodes %.2f)" % row)
    m = len(out)
    g = lambda i: sum(r[i] for r in out) / m
    print("means: frame given nodes %.4g (model %.4g + the planted path) ; planted top pool nodes %.4g (model %.4g) ; "
          "free DFS model 10^%.2f ; free top half model 10^%.2f"
          % (g(3), g(5), g(7), g(9), log10(sum(10 ** r[1] for r in out) / m),
             log10(sum(10 ** r[12] for r in out) / m)))


main()
