"""TOY BOARDS ONLY. The low-rank study's solver (bm.py, unmodified, imported) with its early
stop: round every 2 outer steps (greedy), stop at the first rounding with 0 mismatches.
Same start as run_unit.py (rng 1000 r + start, init_U). One start per process.
Usage: baseline_bm.py n cf ci seed r start seconds out.jsonl [broken]
"""
import os, sys, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
from cp_common import load, is_solution
from bm import solve, init_U

n, cf, ci, seed, r, st = map(int, sys.argv[1:7])
tlim = float(sys.argv[7]); out = sys.argv[8]
broken = len(sys.argv) > 9 and sys.argv[9] == "broken"
B, S, C = load(n, cf, ci, seed, broken)
U0 = init_U(B, r, np.random.default_rng(1000 * r + st))
V, info = solve(S, r, U0, time_limit=tlim, stop_check=lambda x: is_solution(B, x)[0])
ok, js = is_solution(B, V[:, 0])
rec = dict(method="bm", board=[n, cf, ci, seed], broken=broken, r=r, start=st, solved=bool(ok),
           status=info["status"], evals=info["evals"], outer=info["outer"], seconds=info["seconds"],
           viol=info["viol"]["max"], mismatches=int(B.mismatches(js.tolist())))
with open(out, "a") as f:
    f.write(json.dumps(rec) + "\n")
print(json.dumps(rec))
