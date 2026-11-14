"""TOY counts plus one derived estimate. For each toy board: P, K (conflict pairs), F (free pairs)
measured from Structure, and the per-evaluation cost formulas of bm.py evaluated at r = 16.
For the 16x16 (derived, not measured): P from the placement model (rim-consistent placements, pin
at cell 0: 10 corner + 56 x 56 edge + 196 x 196 x 4 inner), K with uniform colours (5 frame and
17 inner colours, a facing pair matches with probability 1/5 or 1/17), F = P(P-1)/2 - K, and the
active nonnegativity multipliers A at the 5x5 measured share of F (0.77).
Usage: scale.py"""
import os, sys, json
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import common  # noqa: F401
from board import Board
from bm import Structure

R = 16
SHARE = 0.77   # active nonnegativity multipliers as a share of F: 5x5 median at r = 32 (66,578 of 86,800)


def cost(P, K, A, N, T, r=R, m=10):
    flops = 4 * P * P * r + 3 * P * P + 4 * K * r + 2 * (N + T) * r
    blocks = 3 * 16 * 2**20                # Gram, multiplier and W blocks of 16 MB each
    mem = 8 * ((2 * m + 4) * P * r + 4 * K + 2 * A) + blocks
    mem_store_conflicts_free = 8 * ((2 * m + 4) * P * r + 2 * A) + blocks
    return flops, mem, mem_store_conflicts_free


out = []
for (n, cf, ci, s) in [(5, 3, 4, 0), (6, 4, 5, 0), (7, 4, 7, 0)]:
    B = Board(n, cf, ci, s); S = Structure(B)
    A = int(SHARE * S.F)
    fl, mem, _ = cost(B.P, S.K, A, B.N, B.T)
    out.append(dict(board=[n, cf, ci, s], P=B.P, K=int(S.K), F=int(S.F), flops_per_eval_r16=fl,
                    mem_bytes_r16=mem))
# 16x16 estimate
n = 16
P = 10 + 56 * 56 + 196 * 196 * 4
cells_in, cells_edge = 196, 56
same_cell = 4 * 0 + cells_edge * (56 * 55 // 2) + cells_in * (784 * 783 // 2) + 3 * (3 * 2 // 2)
same_tile = 56 * (56 * 55 // 2) + 196 * (784 * 783 // 2) + 3 * 3
both = 196 * 196 * 6                       # same cell and same tile: rotation pairs
# adjacencies: inner-inner, edge-inner, edge-edge along the rim, corner-edge
inner_inner = 2 * 14 * 13                  # 364
edge_inner = 56                            # each edge cell has one inner neighbour
rim = 60                                   # rim ring adjacencies (corner-edge and edge-edge)
mis = inner_inner * 784 * 784 * (16 / 17) + edge_inner * 56 * 784 * (16 / 17) + rim * 56 * 56 * (4 / 5)
K = int(same_cell + same_tile - both + mis)
F = P * (P - 1) // 2 - K
fl, mem, mem_nc = cost(P, K, int(SHARE * F), 256, 256)
out.append(dict(board="16x16 derived (uniform colours)", P=P, K=K, F=F, flops_per_eval_r16=fl,
                mem_bytes_r16=mem, mem_bytes_r16_conflicts_from_structure=mem_nc,
                gram_bytes_dense_PxP=8 * P * P))
print(json.dumps(out, indent=1))
