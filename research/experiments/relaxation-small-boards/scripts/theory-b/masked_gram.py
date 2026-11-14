"""TOY BOARDS ONLY. Experiment 3: how far from PSD is a masked nonnegative Gram matrix?

Intermediate form: Y = (N N^T) with the conflict entries set to 0 (N >= 0, P x r), so Y >= 0 and
the zeros hold by construction and the only constraint left besides the sums is PSD of
Z = [[1, x^T], [x, Y]], x = diag(Y). "A few PSD-only corrections" is plausible only if Z has few
negative eigenvalues along the way. This counts them at random nonnegative N around the Sinkhorn
centre, and at a mixture of true solutions (where the mask is inactive and the count must be 0).
Usage: masked_gram.py n cf ci seed
"""
import os, sys, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
from cp_common import load

n, cf, ci, seed = map(int, sys.argv[1:5])
B, S, C = load(n, cf, ci, seed)
P = B.P
free = 1.0 - C.toarray()


def count(Nf):
    Y = (Nf @ Nf.T) * free
    x = np.diag(Y).copy()
    Z = np.zeros((P + 1, P + 1)); Z[0, 0] = 1; Z[0, 1:] = x; Z[1:, 0] = x; Z[1:, 1:] = Y
    ev = np.linalg.eigvalsh(Z)
    return dict(neg=int((ev < -1e-9).sum()), lam_min=float(ev[0]), neg_mass=float(-ev[ev < 0].sum()), trace=float(ev.sum()))


rng = np.random.default_rng(0)
for r in (1, 4, 16, 64, 256):
    share = rng.dirichlet(np.ones(r), size=P) if r > 1 else np.ones((P, 1))
    out = count(np.sqrt(B.X0[:, None] * share))
    print(json.dumps(dict(board=[n, cf, ci, seed], P=P, kind="random around centre", r=r, **out)))
k = min(3, len(B.sol_idx))
Nf = np.zeros((P, k))
for j in range(k):
    Nf[B.sol_idx[j], j] = np.sqrt(1.0 / k)
print(json.dumps(dict(board=[n, cf, ci, seed], P=P, kind=f"mixture of {k} solutions", r=k, **count(Nf))))
