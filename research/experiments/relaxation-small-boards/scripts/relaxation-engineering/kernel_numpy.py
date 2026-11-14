"""NumPy reference for kernel-bench (same closed-form inputs, same two kernels), written the way
bmcopy/bm.py computes them:

  pairs     hz = (V[ci] * V[cj]).sum(1) (in chunks of 2^18 pairs when K x r doubles would not fit),
            then the symmetric CSR matrix with the weights at both slots, times V
            (bm.py: Structure.conflict_matrix(vals) @ V, including its CSR copy and value gather)
  allpairs  blocks of 16 MB: G = V[rows] @ V.T, W = max(0, c - rho G), sum(W^2), g[rows] -= W @ V

Usage: kernel_numpy.py P K r reps     (one BLAS thread unless VECLIB_MAXIMUM_THREADS is set)
"""
import os, sys, json, time
os.environ.setdefault("VECLIB_MAXIMUM_THREADS", "1")
os.environ.setdefault("OMP_NUM_THREADS", "1")
os.environ.setdefault("OPENBLAS_NUM_THREADS", "1")
import numpy as np
import scipy.sparse as sp

P, K, r, reps = map(int, sys.argv[1:5])
threads = int(os.environ["VECLIB_MAXIMUM_THREADS"])


def best(fn):
    t = np.inf
    for _ in range(reps):
        t0 = time.perf_counter(); out = fn(); t = min(t, time.perf_counter() - t0)
    return t, out


for dt in (np.float64, np.float32):
    q = np.arange(P, dtype=np.int64)[:, None]; t = np.arange(r, dtype=np.int64)[None, :]
    V = ((((q * 31 + t * 17 + q * t * 7) % 101) / 101.0 - 0.5) / np.sqrt(r)).astype(dt)
    k = np.arange(K, dtype=np.uint64)
    ci = ((k * np.uint64(2654435761)) % np.uint64(P)).astype(np.int64)
    cj = ((ci.astype(np.uint64) + np.uint64(1) + (k * np.uint64(40503)) % np.uint64(P - 1)) % np.uint64(P)).astype(np.int64)
    lam = (((np.arange(K) % 13) - 6.0) * 0.01).astype(dt)
    rho, c = dt(10.0), dt(0.02)
    rows = np.concatenate([ci, cj]); cols = np.concatenate([cj, ci])
    M = sp.csr_matrix((np.arange(1, 2 * K + 1, dtype=np.float64), (rows, cols)), shape=(P, P))
    # duplicate pairs are summed by the constructor; rebuild the slot map without merging
    order = np.lexsort((cols, rows))
    indptr = np.concatenate([[0], np.cumsum(np.bincount(rows, minlength=P))])
    M = sp.csr_matrix((np.zeros(2 * K, dt), cols[order].astype(np.int32), indptr), shape=(P, P))
    slot = (order % K).astype(np.int32)
    ci = ci.astype(np.int32); cj = cj.astype(np.int32)
    del rows, cols, order, k, q, t          # keep the P = 20,000 case under 4 GB
    CH = 1 << 18

    def pairs():
        hz = np.empty(K, dt)
        for s in range(0, K, CH):
            hz[s:s + CH] = (V[ci[s:s + CH]] * V[cj[s:s + CH]]).sum(1)
        f = float(lam.astype(np.float64) @ hz + 0.5 * float(rho) * (hz.astype(np.float64) @ hz))
        A = M.copy(); A.data = (lam + rho * hz)[slot]
        return f, A @ V

    bs = max(16, int(16 * 2**20 // (8 * P)))
    blocks = [np.arange(s, min(P, s + bs)) for s in range(0, P, bs)]

    def allpairs():
        g = np.zeros_like(V); f = 0.0
        for rws in blocks:
            G = V[rws] @ V.T
            W = np.maximum(dt(0.0), c - rho * G)
            f += float((W * W).sum(dtype=np.float64))
            g[rws] -= W @ V
        return f, g

    # Restructured evaluation: the Gram block is computed anyway, so the pair values are read out
    # of it (K scalars instead of 2 K r doubles) and the pair weights are written into W before the
    # one product W V. Same work as pairs() + allpairs() on a board, where pairs are distinct; on
    # these synthetic lists some pairs repeat and the plain indexed write keeps one of them, so the
    # checksum is not comparable. Time only.
    starts = [int(b[0]) for b in blocks]; ends = [int(b[-1]) + 1 for b in blocks]
    loc = [(np.repeat(np.arange(e - s0), np.diff(indptr[s0:e + 1])), M.indices[indptr[s0]:indptr[e]], slot[indptr[s0]:indptr[e]])
           for s0, e in zip(starts, ends)]

    def dense_one_pass():
        g = np.zeros_like(V); f = 0.0
        for rws, (rl, cl, sl) in zip(blocks, loc):
            G = V[rws] @ V.T
            h = G[rl, cl]
            W = np.maximum(dt(0.0), c - rho * G)
            f += float((W * W).sum(dtype=np.float64)) + 0.5 * float(lam[sl].astype(np.float64) @ h + 0.5 * float(rho) * (h.astype(np.float64) @ h))
            W[rl, cl] -= lam[sl] + rho * h
            g[rws] -= W @ V
        return f, g

    for name, fn in (("pairs_bm_style", pairs), ("allpairs_bm_style", allpairs), ("both_dense_one_pass", dense_one_pass)):
        tt, (f, g) = best(fn)
        print(json.dumps(dict(lang="numpy", kernel=name, prec=np.dtype(dt).name.replace("float", "f"), threads=threads,
                              P=P, K=K, r=r, seconds=tt, f=f, g_sumsq=float((g.astype(np.float64) ** 2).sum()))), flush=True)
