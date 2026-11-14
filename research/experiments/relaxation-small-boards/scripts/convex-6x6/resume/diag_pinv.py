"""TOY BOARDS ONLY. Singular values of C C^T at the cut and the size of numpy's pinv of it, per seed.
Usage: python -I diag_pinv.py"""
import os, sys
for _k in ("OMP_NUM_THREADS", "OPENBLAS_NUM_THREADS", "MKL_NUM_THREADS", "VECLIB_MAXIMUM_THREADS"):
    os.environ.setdefault(_k, "1")
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import common  # noqa
import numpy as np
from board import Board
import bm
for n, cf, ci, seed in [(6, 4, 5, 0), (6, 4, 5, 1), (6, 4, 5, 2), (5, 3, 4, 1)]:
    B = Board(n, cf, ci, seed); S = bm.Structure(B)
    C = np.vstack([S.Ccell.toarray(), S.Ctile.toarray()]); G = C @ C.T
    s = np.linalg.svd(G, compute_uv=False); Pi = np.linalg.pinv(G)
    print((n, cf, ci, seed), "sv tail", s[-6:], "ratio to cutoff", s[-6:] / (1e-15 * s[0]), "max|pinv|", float(np.abs(Pi).max()))
