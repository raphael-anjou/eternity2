"""TOY BOARDS ONLY (generated; never Eternity II progress). Shared loader for theory study B.
Reuses the low-rank study's Board and conflict Structure, read only."""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
LR = os.path.join(HERE, "..", "lowrank-relaxation")
sys.path.insert(0, LR)
import common  # noqa: F401,E402
from common import gen, enumerate_all  # noqa: E402
import numpy as np  # noqa: E402
import scipy.sparse as sp  # noqa: E402
from board import Board  # noqa: E402
from bm import Structure, greedy_round  # noqa: E402


def load(n, cf, ci, seed, broken=False):
    """broken=True: the low-rank study's negative control (run_unit.py, mode "broken"): one side
    of the first inner tile recoloured so that no solution exists (enumeration confirms 0)."""
    if not broken:
        B = Board(n, cf, ci, seed)
    else:
        tiles = gen(n, cf, ci, seed)
        done = False
        for i, t in enumerate(tiles):
            if 0 not in t:
                nt = list(t); nt[0] = cf + 1 if t[0] != cf + 1 else cf + 2; nt = tuple(nt)
                canon = {min(u[k:] + u[:k] for k in range(4)) for j, u in enumerate(tiles) if j != i}
                if min(nt[k:] + nt[:k] for k in range(4)) in canon:
                    continue
                tt = tiles[:i] + [nt] + tiles[i + 1:]
                if len(enumerate_all(n, tt)["sols"]) == 0:
                    tiles = tt; done = True
                    break
        assert done
        B = Board(n, cf, ci, seed, tiles=tiles)
    S = Structure(B)
    C = S.conflict_matrix(np.ones(S.K)).tocsr()          # symmetric 0/1, 2K nonzeros
    return B, S, C


def is_solution(B, x):
    js = greedy_round(B, x)
    return B.mismatches(js.tolist()) == 0, js
