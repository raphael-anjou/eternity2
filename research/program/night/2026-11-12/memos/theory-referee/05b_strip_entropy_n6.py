"""Width-6 strip eigenvalue in float32 (rigorous Fekete upper bound h_inf <= h(6)).
Run: OMP_NUM_THREADS=1 uv run --with numpy python 05b_strip_entropy_n6.py"""
import math, os, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
from e2lib import load, kind
_, P, _ = load()
C = 17; n = 6
W = np.zeros((C, C, C, C), dtype=np.float32)
for p in P:
    if kind(p) != "interior": continue
    for r in range(4):
        q = tuple(p[(i - r) % 4] for i in range(4))
        W[q[3] - 6, q[0] - 6, q[1] - 6, q[2] - 6] += 1
W_top = W.sum(axis=1); M = W.reshape(C * C, C * C)
def column(v):
    x = np.tensordot(v, W_top, axes=([0], [0]))
    for i in range(1, n):
        x = np.moveaxis(x, 0, -2); sh = x.shape[:-2]
        x = (np.ascontiguousarray(x).reshape(-1, C * C) @ M).reshape(*sh, C, C)
    return x.sum(axis=-1)
v = np.ones((C,) * n, dtype=np.float32); v /= np.sqrt(float(v.size)); lam = 0; t = time.time()
for it in range(9):
    w = column(v); nw = float(np.sqrt(np.sum(w.astype(np.float64) ** 2))); v = (w / nw).astype(np.float32)
    print(f"iteration {it + 1}: lambda_6 ~ {nw:.6g}  h(6) ~ {math.log10(nw) / 6:.5f}  ({time.time() - t:.0f}s)", flush=True)
print("for comparison: 0.4352 + 1.2293/6 =", round(0.4352 + 1.2293 / 6, 4), "; lambda_5 = 2541.3 -> log10(lambda_6/lambda_5) =", round(math.log10(nw / 2541.3), 4))
