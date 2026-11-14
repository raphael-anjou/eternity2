"""TOY BOARDS ONLY (generated; never Eternity II progress).
Structured pieces for theory study A. Imports the low-rank study's board model read-only.

Linear form of the equalities, on the factor V (P x r, v_0 = e_1):
  cells   S_c = sum_{p in c} v_p = v_0                        N rows
  tiles   U_t = sum_{p of t} v_p = v_0                        T rows
  edges   A_{e,k} - B_{e,k} = 0 for each inner edge e = (a, b) and colour k, with
          A_{e,k} = sum of v_p over p in a showing k on e, B_{e,k} the same in b    G rows
Stacked as L V = b e_1^T with L sparse (entries 0, +1, -1; at most 6 nonzeros per column).
"""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
LR = os.path.join(HERE, "..", "lowrank-relaxation")
sys.path.insert(0, LR)
import common  # noqa: F401,E402
import numpy as np  # noqa: E402
import scipy.sparse as sp  # noqa: E402
from board import Board  # noqa: E402,F401
from bm import Structure  # noqa: E402,F401


def edges(B):
    """List of (a, b, d): cell a, its neighbour b in direction d (east 1, south 2)."""
    return [(x, int(B.nb[x, d]), d) for x in range(B.N) for d in (1, 2) if B.nb[x, d] >= 0]


def edge_groups(B):
    """For each edge and colour: (index arrays of placements in a showing k, in b showing k)."""
    out = []
    for (a, b, d) in edges(B):
        pa = np.where(B.cellOf == a)[0]; pb = np.where(B.cellOf == b)[0]
        ka = B.cols[pa, d]; kb = B.cols[pb, (d + 2) % 4]
        for k in sorted(set(ka.tolist()) | set(kb.tolist())):
            out.append(((a, b, d, k), pa[ka == k], pb[kb == k]))
    return out


def linear_system(B):
    """L (m x P, CSR) and b (m,), m = N + T + G."""
    P, N, T = B.P, B.N, B.T
    rows = [B.cellOf, N + B.tileOf]; cols = [np.arange(P), np.arange(P)]; vals = [np.ones(P), np.ones(P)]
    m = N + T
    for (_, pa, pb) in edge_groups(B):
        rows += [np.full(len(pa), m), np.full(len(pb), m)]; cols += [pa, pb]
        vals += [np.ones(len(pa)), -np.ones(len(pb))]
        m += 1
    L = sp.csr_matrix((np.concatenate(vals), (np.concatenate(rows), np.concatenate(cols))), shape=(m, P))
    b = np.zeros(m); b[:N + T] = 1.0
    return L, b
