"""TOY. Finite-difference check of the EqAL gradient, and that it equals ObjAL when s = 1."""
import sys, runpy
import numpy as np
sys.argv = ["bm_obj.py", "1", "eq", "8", "0", "0", "/dev/null"]
ns = runpy.run_path("bm_obj.py")
B, S, bm, EqAL, ObjAL = ns["B"], ns["S"], ns["bm"], ns["EqAL"], ns["ObjAL"]
rng = np.random.default_rng(0)
r = 8
U = bm.init_U(B, r, rng)
al = EqAL(S, r, rho=37.0, vec=True); al.w = -1.0; al.c = rng.uniform(0, 1, B.P)
al.set_scale(bm.V_of_U(U)[:, 0])
al.lz = rng.standard_normal(S.K) * 0.1; al.lc = rng.standard_normal((S.N, r)); al.lt = rng.standard_normal((S.T, r))
al.update_multipliers(bm.V_of_U(U))          # fills M2 and rescales
w = (U * (1 + 0.1 * rng.standard_normal((B.P, 1)))).ravel()
f, g = al.fg(w)
d = rng.standard_normal(w.shape); d /= np.linalg.norm(d)
for h in (1e-5, 1e-6):
    fd = (al.fg(w + h * d)[0] - al.fg(w - h * d)[0]) / (2 * h)
    print("eq  h", h, "fd", fd, "an", g @ d, "rel", abs(fd - g @ d) / abs(fd))
a1 = EqAL(S, r, rho=37.0, vec=True); a1.s = np.ones(B.P); a2 = ObjAL(S, r, rho=37.0, vec=True)
for a in (a1, a2):
    a.w = 1.0; a.lz = al.lz.copy()
f1, g1 = a1.fg(w); f2, g2 = a2.fg(w)
print("s=1 against bm.py: f", abs(f1 - f2) / abs(f2), "g", np.linalg.norm(g1 - g2) / np.linalg.norm(g2))
