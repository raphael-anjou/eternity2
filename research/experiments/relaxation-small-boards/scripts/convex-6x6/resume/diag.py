"""TOY BOARDS ONLY. Cross-machine check of one Douglas-Rachford step from a checkpoint: prints hashes
of the board structures and the norms of each piece, so two machines can be compared line by line.
Usage: python -I diag.py n cf ci seed ckpt.npy"""
import os, sys, hashlib
for _k in ("OMP_NUM_THREADS", "OPENBLAS_NUM_THREADS", "MKL_NUM_THREADS", "VECLIB_MAXIMUM_THREADS"):
    os.environ.setdefault(_k, "1")
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import common  # noqa
import numpy as np
from board import Board
import bm
n, cf, ci, seed = map(int, sys.argv[1:5]); v = np.load(sys.argv[5])
h = lambda a: hashlib.sha256(np.ascontiguousarray(a).tobytes()).hexdigest()[:12]
B = Board(n, cf, ci, seed); S = bm.Structure(B)
P, N, T = B.P, B.N, B.T; D = P + 1
sol = np.zeros(P, bool)
for s in B.sol_idx: sol[s] = True
C = np.vstack([S.Ccell.toarray(), S.Ctile.toarray()])
print("sols", h(np.array(sorted(map(tuple, B.sols)))), "sol_mask", h(sol), "ci", h(np.asarray(S.ci)), "cj", h(np.asarray(S.cj)), "C", h(C))
print("cellOf", h(np.asarray(B.cellOf)), "tileOf", h(np.asarray(B.tileOf)), "X0", h(np.asarray(B.X0)))
Wn = np.hstack([np.ones((N + T, 1)), -C])
u, sv, vt = np.linalg.svd(Wn, full_matrices=True)
r = int((sv > 1e-9 * sv[0]).sum()); Wb = vt[r:].T
print("rank_w", r, "sv near cut", sv[r - 2:r + 2] if r + 2 <= len(sv) else sv[r - 2:])
print("residual of face basis ||Wn Wb||", float(np.abs(Wn @ Wb).max()))
M = Wb.T @ v @ Wb; w = np.linalg.eigvalsh(M)
print("eig min max", float(w[0]), float(w[-1]), "npos", int((w > 0).sum()))
print("||v - Wb Wb^T v Wb Wb^T||", float(np.linalg.norm(v - Wb @ M @ Wb.T)))
G = Wb.T @ Wb
print("||Wb^T Wb - I|| max", float(np.abs(G - np.eye(G.shape[0])).max()), "||vt vt^T - I|| max", float(np.abs(vt @ vt.T - np.eye(D)).max()))
Q, _ = np.linalg.qr(Wn.T, mode="complete")                      # independent basis of the same null space
Wq = Q[:, r:]
print("||Wq^T Wq - I||", float(np.abs(Wq.T @ Wq - np.eye(Wq.shape[1])).max()), "||Wn Wq||", float(np.abs(Wn @ Wq).max()))
print("projector difference ||Wb Wb^T - Wq Wq^T|| max", float(np.abs(Wb @ Wb.T - Wq @ Wq.T).max()))
wq = np.linalg.eigvalsh(Wq.T @ v @ Wq); print("eig via QR basis min max", float(wq[0]), float(wq[-1]), "npos", int((wq > 0).sum()))
print("trace M", float(np.trace(M)), "||M||_F", float(np.linalg.norm(M)), "sym M", float(np.abs(M - M.T).max()))
w2, Q2 = np.linalg.eigh(M)
print("eigh residual ||M Q - Q w|| max", float(np.abs(M @ Q2 - Q2 * w2).max()), "||Q^T Q - I||", float(np.abs(Q2.T @ Q2 - np.eye(len(w2))).max()))
print("sum eig", float(w2.sum()), "sum eig^2", float((w2 ** 2).sum()), "||M||_F^2", float((M ** 2).sum()))
Mr = np.random.default_rng(1).standard_normal((1223, 1223)); Mr = Mr + Mr.T
wr, Qr = np.linalg.eigh(Mr); print("random eigh residual", float(np.abs(Mr @ Qr - Qr * wr).max()))
