import os, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np, scipy.sparse as sp
from board import Board
from lift import build, solve
B = Board(4, 2, 4, 3)
sol_mask = np.isin(np.arange(B.P), B.sol_idx[0])
for eps in (1e-4, 1e-5):
    data, cone, lay = build(B, psd=True)
    s, secs = solve(data, cone, eps=eps, time_limit=200)
    x = s["x"][lay["ix"]]
    print("entropy", eps, s["info"]["status"], s["info"]["iter"], round(secs,1), "err", np.abs(x - sol_mask).max(), flush=True)
    # l2 objective: min 0.5 ||x||^2, no t used (t free but unbounded -> fix c=0 on t)
    data2 = dict(data); c = np.zeros_like(data["c"]); data2["c"] = c
    n = len(c); Pm = sp.csc_matrix((np.ones(B.P), (lay["ix"], lay["ix"])), shape=(n, n)); data2["P"] = Pm
    s, secs = solve(data2, cone, eps=eps, time_limit=200)
    x = s["x"][lay["ix"]]
    print("l2", eps, s["info"]["status"], s["info"]["iter"], round(secs,1), "err", np.abs(x - sol_mask).max(), flush=True)
