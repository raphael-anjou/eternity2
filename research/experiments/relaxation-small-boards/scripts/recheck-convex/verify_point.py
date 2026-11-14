"""TOY BOARDS ONLY (5x5). re-check check of one point of the splitting, from its checkpoint v.

Rebuilds the lift's constraints WITHOUT the memos' bm.Structure or board.Board: placements, pin,
solutions (reverse-scan search) and the conflict pairs come from check_mass.build and the three
rules (same cell, same tile, side-adjacent cells with different facing colours). Then
Z = projection of v on the face, and every constraint of the lift is measured on Z:
  Z PSD (it is a projection output; smallest eigenvalue printed), Z[0,0] = 1, Z[0,p] = Z[p,p],
  cell and tile sums, zeros on conflict pairs, nonnegativity on free pairs.
Prints the mass off solutions of x = Z[0,1:], and saves x.

Usage: verify_point.py seed checkpoint.npy out_x.npy
"""
import os, sys, json
import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
MEMOS = os.path.dirname(HERE)
sys.path.insert(0, os.path.join(MEMOS, "far-ideas-geometry"))
from common import gen  # noqa: E402

seed, ck, outx = int(sys.argv[1]), sys.argv[2], sys.argv[3]
src = open(os.path.join(HERE, "check_mass.py")).read().split("POINTS = {")[0]
ns = {"__file__": os.path.join(HERE, "check_mass.py")}
exec(compile(src, "check_mass_head", "exec"), ns)          # build() and enumerate_reverse() only
n, cf, ci = 5, 3, 4
tiles = gen(n, cf, ci, seed)
pl, pin = ns["build"](n, tiles)
P, N, T = len(pl), n * n, len(tiles)
sols = ns["enumerate_reverse"](n, tiles, pl)
sol = np.zeros(P, bool)
for s in sols:
    sol[list(s)] = True
cell = np.array([p[0] for p in pl]); tile = np.array([p[1] for p in pl]); sides = np.array([p[3] for p in pl])
conf = (cell[:, None] == cell[None, :]) | (tile[:, None] == tile[None, :])
for d, dx in ((1, 1), (2, n)):                              # east and south neighbours
    for x in range(N):
        r, c = divmod(x, n)
        if (d == 1 and c == n - 1) or (d == 2 and r == n - 1):
            continue
        a = np.where(cell == x)[0]; b = np.where(cell == x + dx)[0]
        mis = sides[a, d][:, None] != sides[b, (d + 2) % 4][None, :]
        ia, ib = np.nonzero(mis)
        conf[a[ia], b[ib]] = True; conf[b[ib], a[ia]] = True
np.fill_diagonal(conf, False)
free = ~conf; np.fill_diagonal(free, False)

C = np.zeros((N + T, P))
C[cell, np.arange(P)] = 1.0; C[N + tile, np.arange(P)] = 1.0
Wn = np.hstack([np.ones((N + T, 1)), -C])
_, sv, vt = np.linalg.svd(Wn, full_matrices=True)
rk = int((sv > 1e-9 * sv[0]).sum())
Wb = vt[rk:].T
v = np.load(ck)
w, Q = np.linalg.eigh(Wb.T @ v @ Wb)
k = int(np.searchsorted(w, 0.0))
Qp = Wb @ Q[:, k:]
Z = (Qp * w[k:]) @ Qp.T
x = Z[0, 1:]; Y = Z[1:, 1:]
ev = np.linalg.eigvalsh(Z)
off = np.where(~sol)[0]
big = sorted(((float(x[j]), list(map(int, pl[j][:3]))) for j in off if x[j] > 1e-3), reverse=True)
out = dict(
    seed=seed, P=P, solutions=len(sols), conflict_pairs=int(conf.sum() // 2), free_pairs=int(free.sum() // 2),
    face_order=int(Wb.shape[1]),
    min_eig=float(ev[0]), rank_1e3=int((ev > 1e-3 * ev[-1]).sum()), rank_1e6=int((ev > 1e-6 * ev[-1]).sum()),
    viol_Z00=float(abs(Z[0, 0] - 1)), viol_diag=float(np.abs(np.diag(Y) - x).max()),
    viol_sums=float(np.abs(C @ x - 1).max()), viol_zero=float(np.abs(Y[conf]).max()),
    viol_nonneg=float(max(0.0, -Y[free].min())), viol_x_nonneg=float(max(0.0, -x.min())),
    mass_off_total=float(np.clip(x, 0, None)[~sol].sum()), mass_off_per_cell=float(np.clip(x, 0, None)[~sol].sum() / N),
    n_off_above_1e3=len(big), n_off_above_1e5=int((x[~sol] > 1e-5).sum()), largest_off=[(round(a, 4), b) for a, b in big[:16]],
    cells_carrying_off_mass=sorted({b[0] for a, b in big}), tiles_carrying_off_mass=sorted({b[1] for a, b in big}))
print(json.dumps(out))
np.save(outx, x)
