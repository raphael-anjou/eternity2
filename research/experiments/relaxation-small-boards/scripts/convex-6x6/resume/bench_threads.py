"""TOY BOARDS ONLY. Times numpy.linalg.eigh on a random symmetric matrix of the 6x6 face order (1,223)
with the BLAS thread count set by the environment. Prints median seconds over REPS calls.
Usage: OPENBLAS_NUM_THREADS=k OMP_NUM_THREADS=k python -I bench_threads.py [order] [reps]"""
import os, sys, time
import numpy as np
n = int(sys.argv[1]) if len(sys.argv) > 1 else 1223
reps = int(sys.argv[2]) if len(sys.argv) > 2 else 15
rng = np.random.default_rng(0); A = rng.standard_normal((n, n)); A = A + A.T
np.linalg.eigh(A)
ts = []
for _ in range(reps):
    t = time.time(); np.linalg.eigh(A); ts.append(time.time() - t)
ts.sort()
print(f"threads {os.environ.get('OPENBLAS_NUM_THREADS')} order {n} median_s {ts[len(ts)//2]:.4f} min_s {ts[0]:.4f} load {os.getloadavg()[0]:.1f}")
