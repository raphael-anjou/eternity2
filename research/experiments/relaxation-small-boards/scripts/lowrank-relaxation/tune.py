"""TOY. Solver tuning runs on 5x5 boards (penalty schedule), kept for the record.
Usage: tune.py seed r rho0 rho_up rho_max inner_iters time_limit start"""
import os, sys, json
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import common  # noqa: F401
import numpy as np
from board import Board
from bm import Structure, solve, init_U
seed, r = int(sys.argv[1]), int(sys.argv[2])
rho0, rho_up, rho_max = map(float, sys.argv[3:6]); inner = int(sys.argv[6]); tl = float(sys.argv[7]); st = int(sys.argv[8]); vec = len(sys.argv) > 9 and sys.argv[9] == "vec"
B = Board(5, 3, 4, seed); S = Structure(B)
p = os.path.join(HERE, "raw", f"x_sdp_5_3_4_{seed}.npy")
xs = np.load(p) if os.path.exists(p) else None
sol = np.zeros(B.P, bool)
for s in B.sol_idx: sol[s] = True
V, info = solve(S, r, init_U(B, r, np.random.default_rng(st)), time_limit=tl, inner_iters=inner, rho0=rho0, rho_up=rho_up, rho_max=rho_max, vec=vec)
x = V[:, 0]
print(json.dumps(dict(args=sys.argv[1:], status=info["status"], sec=info["seconds"], evals=info["evals"], viol=info["viol"]["max"],
                      obj=float(x @ x), linf=None if xs is None else float(np.abs(x - xs).max()),
                      massoff=float(np.clip(x, 0, None)[~sol].sum() / B.N), hist=info["hist"][::10] + info["hist"][-1:])))
