"""TOY BOARDS ONLY (5x5). Window consistency: a placement survives when, in every listed window
containing its cell, some filling of the window (distinct tiles, matching sides inside the window,
rim and pin as in the model, the rest of the board left free) uses it. Run to a fixpoint. This is
the support of an EXACT relaxation of each window taken alone, so it bounds from above what any
lift of a window with the rest left free can remove.
Usage: wincons.py n cf ci seed "hxw,hxw,..." [node_cap]"""
import os, sys, json, time
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, os.path.join(HERE, "bmcopy"))
import common  # noqa: F401,E402
import numpy as np  # noqa: E402
from board import Board  # noqa: E402

n, cf, ci, seed = map(int, sys.argv[1:5])
assert n <= 5
shapes = [tuple(map(int, s.split("x"))) for s in sys.argv[5].split(",")]
cap = int(sys.argv[6]) if len(sys.argv) > 6 else 3_000_000
B = Board(n, cf, ci, seed)
P, N = B.P, B.N
sol = np.zeros(P, bool)
for s in B.sol_idx:
    sol[s] = True
ac = B.arc_consistency()
COLS, TILE = B.COLS, B.TILE


def window_support(cells_w, alive):
    """cells_w in row order; returns (supported mask over placements of these cells, nodes, complete)."""
    inw = {x: i for i, x in enumerate(cells_w)}
    dom = [[j for j in range(P) if B.cellOf[j] == x and alive[j]] for x in cells_w]
    sup = np.zeros(P, bool)
    used = set(); js = [0] * len(cells_w); nodes = [0]

    class Cap(Exception):
        pass

    def rec(i):
        if i == len(cells_w):
            return True
        nodes[0] += 1
        if nodes[0] > cap:
            raise Cap
        x = cells_w[i]
        up, left = x - n, x - 1
        cu = COLS[js[inw[up]]][2] if (x // n > 0 and up in inw) else None
        cl = COLS[js[inw[left]]][1] if (x % n > 0 and left in inw) else None
        any_ok = False
        for j in dom[i]:
            if TILE[j] in used:
                continue
            c = COLS[j]
            if cu is not None and c[0] != cu:
                continue
            if cl is not None and c[3] != cl:
                continue
            used.add(TILE[j]); js[i] = j
            if rec(i + 1):
                sup[j] = True; any_ok = True
            used.discard(TILE[j])
        return any_ok
    try:
        rec(0)
    except Cap:
        return None, nodes[0], False
    return sup, nodes[0], True


out = dict(board=[n, cf, ci, seed], P=P, n_sols=len(B.sols), sol_placements=int(sol.sum()),
           ac_kept=int(ac.sum()), rows=[])
for (h, w) in shapes:
    wins = [[(r0 + dr) * n + (c0 + dc) for dr in range(h) for dc in range(w)]
            for r0 in range(n - h + 1) for c0 in range(n - w + 1)]
    alive = np.ones(P, bool)
    t0 = time.time(); passes = 0; capped = 0; tot_nodes = 0
    changed = True
    while changed:
        changed = False; passes += 1; capped = 0
        for cw in wins:
            sup, nd, ok = window_support(cw, alive)
            tot_nodes += nd
            if not ok:
                capped += 1
                continue
            m = np.isin(B.cellOf, cw)
            kill = m & alive & ~sup
            if kill.any():
                alive[kill] = False; changed = True
    assert alive[sol].all(), "window consistency removed a solution placement"
    out["rows"].append(dict(window=f"{h}x{w}", windows=len(wins), kept=int(alive.sum()),
                            off_solution_kept=int((alive & ~sol).sum()),
                            removed_that_ac_keeps=int((ac & ~alive).sum()), passes=passes,
                            windows_capped_last_pass=capped, nodes=tot_nodes, seconds=round(time.time() - t0, 1)))
    print(json.dumps(out["rows"][-1]), flush=True)
print(json.dumps(out))
