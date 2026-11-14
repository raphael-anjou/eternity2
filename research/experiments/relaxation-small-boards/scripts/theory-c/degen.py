"""TOY (5x5). Structure counts, no solve: how far the facially reduced program is from having an
interior point when the lift is exact (feasible set = convex hull of the solutions' rank-one
matrices), and the natural scales of the two constraint classes.
Usage: degen.py seed [seed ...]
"""
import os, sys, json
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import common  # noqa: F401,E402
import numpy as np  # noqa: E402
from board import Board  # noqa: E402
import bm  # noqa: E402

for seed in map(int, sys.argv[1:]):
    B = Board(5, 3, 4, seed); S = bm.Structure(B)
    P, N, T = B.P, B.N, B.T
    C = np.vstack([S.Ccell.toarray(), S.Ctile.toarray()])
    D = P + 1 - np.linalg.matrix_rank(np.hstack([np.ones((N + T, 1)), -C]))
    k = len(B.sol_idx)
    Zs = np.zeros((k, P + 1)); Zs[:, 0] = 1
    for i, s in enumerate(B.sol_idx):
        Zs[i, 1 + s] = 1
    span = int(np.linalg.matrix_rank(Zs))                      # largest rank of a point of the hull
    X = Zs[:, 1:]
    Ymix = (X.T @ X) > 0                                        # pairs used together by a solution
    iu = np.triu_indices(P, 1)
    conf = np.zeros((P, P), bool); conf[S.ci, S.cj] = True
    free_pos = int((Ymix[iu] & ~conf[iu]).sum())
    lifted = np.array([np.outer(z, z)[np.triu_indices(P + 1)] for z in Zs])
    hull_dim = int(np.linalg.matrix_rank(lifted[1:] - lifted[0])) if k > 1 else 0
    used = int((X.sum(0) > 0).sum())
    print(json.dumps(dict(seed=seed, P=P, K=int(S.K), F=int(S.F), solutions=k, face_order=int(D),
                          max_rank_if_exact=span, slater_deficit=int(D - span), hull_dim=hull_dim,
                          face_matrix_dim=int(D * (D + 1) // 2),
                          min_face_matrix_dim=span * (span + 1) // 2,
                          placements_in_a_solution=used,
                          free_pairs_positive_at_full_mixture=free_pos,
                          free_pairs_active_at_full_mixture=int(S.F) - free_pos,
                          entry_scale=(N / P) ** 2, penalty_ratio_sum_to_pair=(P / N) ** 4)))
