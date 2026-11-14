"""TOY BOARDS ONLY (5x5). Reads the saved x of each run in raw/ and counts, per point, the placements
below 1e-4 (uncertified "kills"), how many of them arc consistency keeps, and whether any solution
placement is among them (which would make the filter unsafe even as a heuristic)."""
import os, sys, glob, json
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, os.path.join(HERE, "bmcopy"))
import common  # noqa: F401,E402
import numpy as np  # noqa: E402
from board import Board  # noqa: E402
cache = {}
for f in sorted(glob.glob(os.path.join(HERE, "raw", "x*.npy"))):
    name = os.path.basename(f)
    parts = name.split("_")
    n, cf, ci, seed = map(int, parts[1:5])
    if (n, cf, ci, seed) not in cache:
        B = Board(n, cf, ci, seed)
        sol = np.zeros(B.P, bool)
        for s in B.sol_idx:
            sol[s] = True
        cache[(n, cf, ci, seed)] = (B, sol, B.arc_consistency())
    B, sol, ac = cache[(n, cf, ci, seed)]
    x = np.load(f)
    low = x < 1e-4
    print(json.dumps(dict(run=name, P=B.P, ac_kept=int(ac.sum()), sol_placements=int(sol.sum()), below_1e4=int(low.sum()),
                          below_1e4_ac_keeps=int((low & ac).sum()), solution_placements_below_1e4=int((low & sol).sum()),
                          mass_off_per_cell=float(x[~sol].clip(0).sum() / B.N))))
