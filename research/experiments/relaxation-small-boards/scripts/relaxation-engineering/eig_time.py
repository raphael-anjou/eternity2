"""Time one symmetric eigendecomposition of order P + 1 (the dense projection of the splitting
method) with the LAPACK drivers NumPy and SciPy expose, one BLAS thread. Best of reps, wall and CPU.
Usage: eig_time.py P reps"""
import os, sys, json, time
os.environ.setdefault("VECLIB_MAXIMUM_THREADS", "1"); os.environ.setdefault("OMP_NUM_THREADS", "1")
import numpy as np, scipy.linalg as sla
P, reps = int(sys.argv[1]), int(sys.argv[2])
rng = np.random.default_rng(0)
A = rng.standard_normal((P + 1, 40)); M = A @ A.T / 40 - 0.3 * np.eye(P + 1)
N_ = rng.standard_normal((P + 1, P + 1)) * 0.01; M += N_ + N_.T
out = dict(P=P, loadavg=os.getloadavg())
for name, fn in (("numpy_eigh_syevd", lambda: np.linalg.eigh(M)),
                 ("scipy_evd", lambda: sla.eigh(M, driver="evd", check_finite=False)),
                 ("scipy_evr_all", lambda: sla.eigh(M, driver="evr", check_finite=False)),
                 ("scipy_evr_positive", lambda: sla.eigh(M, driver="evr", subset_by_value=[0, np.inf], check_finite=False)),
                 ("eigvalsh_only", lambda: np.linalg.eigvalsh(M)),
                 ("gemm_PxP", lambda: M @ M)):
    bw = bc = 1e9
    for _ in range(reps):
        w0, c0 = time.perf_counter(), time.process_time(); fn()
        bw = min(bw, time.perf_counter() - w0); bc = min(bc, time.process_time() - c0)
    out[name] = dict(wall_s=round(bw, 5), cpu_s=round(bc, 5))
print(json.dumps(out))
