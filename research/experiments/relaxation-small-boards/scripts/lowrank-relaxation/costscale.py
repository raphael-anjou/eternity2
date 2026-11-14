"""TOY BOARDS ONLY. Cost of one augmented-Lagrangian evaluation (value + gradient) against the
number of placements P: median wall time of 20 evaluations at a random start after one multiplier
update (so the nonnegativity multipliers are populated as in a real run), the number of active
nonnegativity multipliers A, and the process peak memory. One board per process.
Usage: costscale.py n cf ci seed r [nocache]
The 8x8 board (frame 4, inner 9) is built with its solution list left empty (enumeration is not
needed to time an evaluation, and the Python enumerator would take hours at 8x8).
"""
import os, sys, json, time, resource, platform
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import common
import numpy as np
import board as board_mod

n, cf, ci, seed, r = map(int, sys.argv[1:6])
if n >= 8:
    board_mod.enumerate_all = lambda n_, tiles: dict(sols=[], nodes=1)
from board import Board
from bm import Structure, SphereAL, init_U, V_of_U

t0 = time.time()
B = Board(n, cf, ci, seed); S = Structure(B)
t_setup = time.time() - t0
al = SphereAL(S, r, rho=10.0, vec=True)
if len(sys.argv) > 6:
    al.cache_ok = False
U = init_U(B, r, np.random.default_rng(0))
al.update_multipliers(V_of_U(U))
w = U.ravel()
ts = []
for _ in range(20):
    a = time.time(); al.fg(w); ts.append(time.time() - a)
print(json.dumps(dict(board=[n, cf, ci, seed], r=r, P=B.P, K=int(S.K), F=int(S.F), A=int(al.M2.nnz // 2),
                      cache=bool(al.cache_ok), ms_per_eval_median=round(1000 * float(np.median(ts)), 2),
                      setup_seconds=round(t_setup, 1),
                      peak_rss_mb=round(resource.getrusage(resource.RUSAGE_SELF).ru_maxrss / (2**20 if sys.platform == "darwin" else 2**10), 1), host=platform.node())))
