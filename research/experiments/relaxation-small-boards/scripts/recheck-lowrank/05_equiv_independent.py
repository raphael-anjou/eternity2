"""TOY. re-check: recompute NumPy's objective and gradient for the 30 stored equivalence states with
the ORIGINAL bm.py (scripts/lowrank-relaxation/), from the stored W and multipliers only, and compare
with the stored references the Rust test reads. Shows the references are bm.py's and what the
states cover (rank, penalty, share of free pairs with a multiplier).
Usage: uv run --with numpy --with scipy python -I 05_equiv_independent.py"""
import os, sys
H = os.path.dirname(os.path.abspath(__file__))
O = os.path.join(H, "..", "lowrank-relaxation"); sys.path.insert(0, O)
import common  # noqa
from common import gen
import numpy as np, scipy.sparse as sp
from board import Board
from bm import Structure, SphereAL
L = os.path.join(H, "..", "lowrank-rust")
sys.path.insert(1, os.path.join(L, "py"))
worst = 0
def broken_tiles():
    import importlib.util
    spec = importlib.util.spec_from_file_location("exp", os.path.join(L, "py", "export.py")); m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
    return m.broken_tiles(5, 3, 4, 0)
for name in ["b5_s0", "b5_s1", "b5_s2", "b5_s3", "b5_s4", "b5_broken"]:
    B = Board(5, 3, 4, 0, tiles=broken_tiles()) if name.endswith("broken") else Board(5, 3, 4, int(name[-1]))
    S = Structure(B); P = B.P; ed = os.path.join(L, "boards", name, "equiv")
    for k in range(5):
        r, rho, non, f_ref = open(os.path.join(ed, f"{k}.txt")).read().split(); r = int(r); rho = float(rho); f_ref = float(f_ref)
        rd = lambda s: np.fromfile(os.path.join(ed, f"{k}_{s}.bin"), "<f8")
        al = SphereAL(S, r, rho=rho, vec=True)
        al.lc = rd("lc").reshape(B.N, r); al.lt = rd("lt").reshape(B.T, r); al.lz = rd("lz")
        mu = rd("mu").reshape(-1, 3)
        M = sp.coo_matrix((mu[:, 2], (mu[:, 0].astype(int), mu[:, 1].astype(int))), shape=(P, P)).tocsr()
        al.M2 = (M + M.T).tocsr()
        W = rd("W")
        f, g = al.fg(W)
        gref = rd("gW")
        e = max(abs(f - f_ref) / abs(f_ref), np.abs(g - gref).max() / np.abs(gref).max()); worst = max(worst, e)
        print(f"{name} state {k}: r {r} rho {rho:.3g} multipliers on {len(mu)}/{S.F} free pairs ({len(mu)/S.F:.2f}) f {f:.6g} rel err vs stored {e:.1e}")
print("worst relative error, original bm.py against the stored references:", f"{worst:.2e}")
