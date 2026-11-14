"""TOY (5x5 only). Numerical check of the collapse identities of the memo, at a random point on the
spheres (nothing feasible about it) and at mixtures of true solutions.
Usage: identities.py [seed]   -> prints JSON
"""
import os, sys, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
from structure import Board, Structure, edges, edge_groups, linear_system
from bm import V_of_U, init_U

seed = int(sys.argv[1]) if len(sys.argv) > 1 else 0
B = Board(5, 3, 4, seed); S = Structure(B)
P, N, T = B.P, B.N, B.T
rng = np.random.default_rng(7)
r = 8
V = V_of_U(init_U(B, r, rng)); V[:, 1:] *= rng.choice([-1, 1], (P, 1))
G = V @ V.T
x = V[:, 0]
out = dict(board=[5, 3, 4, seed], P=P, K=int(S.K))
c, t = B.cellOf, B.tileOf

# I1: sum over same-cell pairs (p != q) of G = sum_c ||S_c||^2 - sum_p ||v_p||^2 ; I2 same for tiles
same_c = (c[:, None] == c[None, :]); same_t = (t[:, None] == t[None, :])
off = ~np.eye(P, dtype=bool)
Sc = np.zeros((N, r)); np.add.at(Sc, c, V)
Ut = np.zeros((T, r)); np.add.at(Ut, t, V)
n2 = (V * V).sum(1)
out["I1_cell_sum"] = [float(G[same_c & off].sum()), float((Sc * Sc).sum() - n2.sum())]
out["I2_tile_sum"] = [float(G[same_t & off].sum()), float((Ut * Ut).sum() - n2.sum())]
# I3: per edge, sum over mismatched pairs of G = <S_a, S_b> - sum_k <A_k, B_k>
# I4: per edge, sum over mismatched pairs of G^2 = tr(M_a M_b) - sum_k tr(M_ak M_bk), M = sum v v^T
groups = edge_groups(B)
e3 = e4 = 0.0
n_mis = 0
for (a, b, d) in edges(B):
    pa = np.where(c == a)[0]; pb = np.where(c == b)[0]
    mis = B.cols[pa, d][:, None] != B.cols[pb, (d + 2) % 4][None, :]
    Gab = G[np.ix_(pa, pb)]
    n_mis += int(mis.sum())
    direct = Gab[mis].sum(); direct2 = (Gab[mis] ** 2).sum()
    gs = [(ga, gb) for ((aa, bb, dd, k), ga, gb) in groups if (aa, bb, dd) == (a, b, d)]
    coll = Sc[a] @ Sc[b] - sum(V[ga].sum(0) @ V[gb].sum(0) for ga, gb in gs)
    Ma = V[pa].T @ V[pa]; Mb = V[pb].T @ V[pb]
    coll2 = np.sum(Ma * Mb) - sum(np.sum((V[ga].T @ V[ga]) * (V[gb].T @ V[gb])) for ga, gb in gs)
    e3 = max(e3, abs(direct - coll)); e4 = max(e4, abs(direct2 - coll2))
out["I3_mismatch_sum_max_abs_err_over_edges"] = float(e3)
out["I4_mismatch_sum_of_squares_max_abs_err_over_edges"] = float(e4)
# I5: same-cell sum of squares = sum_c ||V_c^T V_c||_F^2 - sum_p ||v_p||^4
d5 = (G[same_c & off] ** 2).sum()
c5 = sum(np.sum((V[c == k].T @ V[c == k]) ** 2) for k in range(N)) - (n2 ** 2).sum()
out["I5_cell_sum_of_squares"] = [float(d5), float(c5)]
# I6: all-pairs sum of squares = ||V^T V||_F^2
out["I6_all_pairs_sum_of_squares"] = [float((G ** 2).sum()), float(np.sum((V.T @ V) ** 2))]
# counts: the conflict list against cell / tile / mismatch blocks and their overlaps
adj_mis = np.zeros((P, P), bool)
for (a, b, d) in edges(B):
    pa = np.where(c == a)[0]; pb = np.where(c == b)[0]
    mis = B.cols[pa, d][:, None] != B.cols[pb, (d + 2) % 4][None, :]
    adj_mis[np.ix_(pa, pb)] = mis; adj_mis[np.ix_(pb, pa)] = mis.T
union = (same_c | same_t | adj_mis) & off
adj_all = np.zeros((P, P), bool)
for (a, b, d) in edges(B):
    pa = np.where(c == a)[0]; pb = np.where(c == b)[0]
    adj_all[np.ix_(pa, pb)] = True; adj_all[np.ix_(pb, pa)] = True
out["counts_unordered"] = dict(same_cell=int((same_c & off).sum() // 2), same_tile=int((same_t & off).sum() // 2),
                               adjacent_mismatch=int(adj_mis.sum() // 2), union=int(union.sum() // 2),
                               union_equals_K=bool(union.sum() // 2 == S.K),
                               dense_block_pairs_cell_tile_adjacent=int(((same_c | same_t | adj_all) & off).sum() // 2))
L, b = linear_system(B)
out["linear_rows"] = dict(m=int(L.shape[0]), cells=N, tiles=T, edge_colour=int(L.shape[0] - N - T),
                          rank=int(np.linalg.matrix_rank(np.hstack([L.toarray(), -b[:, None]]))),
                          nnz=int(L.nnz), max_nnz_per_column=int(np.diff(L.tocsc().indptr).max()))
# F1: at mixtures of solutions (factor row s = sqrt(w_s) indicator, rotated so v_0 = e_1), L V = b e_1^T
ns = len(B.sol_idx); w = rng.random(ns); w /= w.sum()
Vs = np.zeros((P, ns))
for s, js in enumerate(B.sol_idx):
    Vs[js, s] = np.sqrt(w[s])
v0 = np.sqrt(w)
Q, _ = np.linalg.qr(np.column_stack([v0, rng.standard_normal((ns, ns - 1))]))
if Q[:, 0] @ v0 < 0:
    Q[:, 0] *= -1
Vm = Vs @ Q
E1 = np.zeros((1, ns)); E1[0, 0] = 1
out["F1_mixture_linear_residual_max"] = float(np.abs(L @ Vm - b[:, None] * E1).max())
out["F1_mixture_sphere_residual_max"] = float(np.abs((Vm * Vm).sum(1) - Vm[:, 0]).max())
# F2: the nonnegative factor itself (v_0 = sqrt(w), not rotated) satisfies L V = b v_0^T
out["F2_nonneg_factor_linear_residual_max"] = float(np.abs(L @ Vs - b[:, None] * v0[None, :]).max())
print(json.dumps(out, indent=1))
