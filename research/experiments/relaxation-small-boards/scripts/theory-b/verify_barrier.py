"""TOY BOARDS ONLY. Check of the barrier derivation (memo, appendix A3).

On the polytope {s >= 0, cell sums 1, tile sums 1} with f(s) = s^T C s, the edges leaving an
integral board s0 include re-rotations of one tile and exchanges of two tiles (any rotations).
Derived: along an exchange edge d, the slope is 2 (C s0) . d = 2 (4 - 2 c + D), c = 1 if the two
cells are adjacent and mismatched before, D = change in mismatches on the edges to other cells.
So the descent sees an exchange only if it removes about 4 mismatches or more.
This script (a) checks the formula on every exchange edge of random boards, (b) counts, for
random boards and for boards first improved by rotations only, how many have NO descending
rotation or exchange edge (a slope < 0, or a slope 0 with negative curvature), and (c) how many of
those same boards a discrete search would still improve by one exchange.
Usage: verify_barrier.py n cf ci seed boards
"""
import os, sys, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
from cp_common import load

n, cf, ci, seed, nb = map(int, sys.argv[1:6])
B, S, C = load(n, cf, ci, seed)
Cd = C.toarray()
kinds = [[x for x in B.rim_cells[k] if x != 0] for k in range(3)]
pairs = [(x, y) for cells in kinds for i, x in enumerate(cells) for y in cells[i + 1:]]


def scan(js):
    """returns (formula mismatches, continuous descending edges, discrete improving exchanges/rotations)."""
    g = Cd[:, js].sum(1)                      # C s0
    bad = 0; cont = 0; disc = 0
    m0 = B.mismatches(list(js))
    for x in range(1, B.N):
        for q in B.OPT[(x, B.TILE[js[x]])]:
            if q == js[x]:
                continue
            slope = 2 * (g[q] - g[js[x]])
            j2 = list(js); j2[x] = q
            dm = B.mismatches(j2) - m0
            if slope < 0 or (slope == 0 and -2 * Cd[q, js[x]] < 0):
                cont += 1
            disc += dm < 0
    for x, y in pairs:
        ox, oy = js[x], js[y]
        c = Cd[ox, oy]
        for qx in B.OPT[(x, B.TILE[oy])]:
            for qy in B.OPT[(y, B.TILE[ox])]:
                slope = 2 * (g[qx] + g[qy] - g[ox] - g[oy])
                j2 = list(js); j2[x] = qx; j2[y] = qy
                dm = B.mismatches(j2) - m0
                adj = y in B.NB[x]
                new_xy = int(adj and Cd[qx, qy] > 0)      # mismatch on the x-y edge after
                old_xy = int(adj and c > 0)
                D = dm - new_xy + old_xy                    # change on edges to other cells
                if abs(slope - 2 * (4 - 2 * c + D)) > 1e-9:
                    bad += 1
                if slope < 0 or slope == 0:               # curvature along an exchange is negative
                    cont += 1
                disc += dm < 0
    return bad, cont, disc


def rot_climb(js):
    js = list(js); improved = True
    while improved:
        improved = False
        for x in range(1, B.N):
            for q in B.OPT[(x, B.TILE[js[x]])]:
                j2 = list(js); j2[x] = q
                if B.mismatches(j2) < B.mismatches(js):
                    js = j2; improved = True
    return js


for kind in ("random", "rotation-improved"):
    tot_bad = 0; no_cont = 0; has_disc = 0; mm = []
    for b in range(nb):
        js = [int(v) for v in B.ctrl_idx[b]]
        if kind != "random":
            js = rot_climb(js)
        bad, cont, disc = scan(js)
        tot_bad += bad; no_cont += cont == 0; has_disc += disc > 0; mm.append(B.mismatches(js))
    print(json.dumps(dict(board=[n, cf, ci, seed], kind=kind, boards=nb, formula_failures=tot_bad,
                          boards_with_no_descending_edge=int(no_cont),
                          boards_a_discrete_exchange_or_rotation_improves=int(has_disc),
                          median_mismatches=float(np.median(mm)))))
