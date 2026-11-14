"""TOY BOARDS ONLY (5x5). re-check check: is the leaky point of seed 3 next to an (almost exactly)
feasible point of the lift, or is its leak an artefact of a loose tolerance?

Independent of the memos' solver code: the constraint sets are rebuilt as in verify_point.py
(own placements, own conflict pairs, own face basis), and the iteration is a plain two-set
Douglas-Rachford FEASIBILITY iteration with NO objective and no penalty parameter:
    Z = proj_face(v),  Y = proj_A(2 Z - v),  v = v + (Y - Z)
  face: W R W^T with R PSD, W the complement of the vectors e_0 - indicator(cell or tile)
  A:    Z[0,0] = 1, Z[0,p] = Z[p,0] = Z[p,p] = x_p, cell and tile sums of x equal to 1,
        Y = 0 on conflict pairs, Y >= 0 on free pairs.
Start: the projection on the face of a checkpoint of the largest-leak run. With no objective the
iteration only looks for the nearest-ish feasible point; if the lift were exact on this board,
every feasible point has zero mass off the solutions, so the mass would have to go to zero as the
violation does.

Usage: polish_feasible.py seed checkpoint.npy seconds out_x.npy
"""
import os, sys, json, time
import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
MEMOS = os.path.dirname(HERE)
sys.path.insert(0, os.path.join(MEMOS, "far-ideas-geometry"))
from common import gen  # noqa: E402

seed, ck, seconds, outx = int(sys.argv[1]), sys.argv[2], float(sys.argv[3]), sys.argv[4]
ns = {"__file__": os.path.join(HERE, "check_mass.py")}
exec(compile(open(ns["__file__"]).read().split("POINTS = {")[0], "check_mass_head", "exec"), ns)
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
for d, dx in ((1, 1), (2, n)):
    for x in range(N):
        r, c = divmod(x, n)
        if (d == 1 and c == n - 1) or (d == 2 and r == n - 1):
            continue
        a = np.where(cell == x)[0]; b = np.where(cell == x + dx)[0]
        ia, ib = np.nonzero(sides[a, d][:, None] != sides[b, (d + 2) % 4][None, :])
        conf[a[ia], b[ib]] = True; conf[b[ib], a[ia]] = True
np.fill_diagonal(conf, False)
free = ~conf; np.fill_diagonal(free, False)
D = P + 1
confD = np.zeros((D, D), bool); confD[1:, 1:] = conf
freeD = np.zeros((D, D), bool); freeD[1:, 1:] = free
C = np.zeros((N + T, P)); C[cell, np.arange(P)] = 1.0; C[N + tile, np.arange(P)] = 1.0
CCt_pinv = np.linalg.pinv(C @ C.T)
Wn = np.hstack([np.ones((N + T, 1)), -C])
_, sv, vt = np.linalg.svd(Wn, full_matrices=True)
Wb = np.ascontiguousarray(vt[int((sv > 1e-9 * sv[0]).sum()):].T)
idx = np.arange(1, D)


def proj_face(M):
    w, Q = np.linalg.eigh(Wb.T @ M @ Wb)
    k = int(np.searchsorted(w, 0.0))
    Qp = Wb @ Q[:, k:]
    return (Qp * w[k:]) @ Qp.T


def proj_A(M):
    Y = M.copy()
    Y[freeD] = np.maximum(M[freeD], 0.0)
    Y[confD] = 0.0
    x0 = (M[0, 1:] + M[1:, 0] + M[idx, idx]) / 3.0
    x = x0 - C.T @ (CCt_pinv @ (C @ x0 - 1.0))
    Y[0, 0] = 1.0; Y[0, 1:] = x; Y[1:, 0] = x; Y[idx, idx] = x
    return Y


def report(Z, it, t):
    x = Z[0, 1:]; Yb = Z[1:, 1:]
    viol = dict(Z00=float(abs(Z[0, 0] - 1)), diag=float(np.abs(np.diag(Yb) - x).max()),
                sums=float(np.abs(C @ x - 1).max()), zero=float(np.abs(Yb[conf]).max()),
                nonneg=float(max(0.0, -Yb[free].min())))
    off = np.clip(x, 0, None)[~sol]
    print(json.dumps(dict(it=it, t=round(t, 1), viol_max=max(viol.values()), mass_off_total=float(off.sum()),
                          mass_off_per_cell=float(off.sum() / N), n_off_above_1e3=int((off > 1e-3).sum()),
                          largest_off=float(off.max()))), flush=True)
    return max(viol.values())


v = proj_face(np.load(ck))
t0 = time.time(); it = 0
Z = v
report(Z, 0, 0.0)
while True:
    Z = proj_face(v)
    Y = proj_A(2.0 * Z - v)
    v = v + (Y - Z)
    it += 1
    el = time.time() - t0
    if it % 250 == 0 or el > seconds:
        vm = report(Z, it, el)
        if vm < 1e-12 or el > seconds:
            break
ev = np.linalg.eigvalsh(Z)
x = Z[0, 1:]
big = sorted(((round(float(x[j]), 5), list(map(int, pl[j][:3]))) for j in np.where(~sol)[0] if x[j] > 1e-3), reverse=True)
print(json.dumps(dict(final=True, seed=seed, iters=it, min_eig=float(ev[0]), rank_1e6=int((ev > 1e-6 * ev[-1]).sum()),
                      largest_off=big, loadavg=os.getloadavg())), flush=True)
np.save(outx, x)
