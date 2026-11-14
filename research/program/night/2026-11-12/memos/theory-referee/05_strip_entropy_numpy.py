"""Width-n strip entropy h(n) of the reusable-tile grammar (interior pieces, free top and bottom), by power iteration
on the column transfer operator. Run: OMP_NUM_THREADS=1 uv run --with numpy python 05_strip_entropy_numpy.py"""
import math, os, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
from e2lib import load, kind
_, P, _ = load()
C = 17
W = np.zeros((C, C, C, C))  # W[left, up, right, down] = number of oriented interior tiles
for p in P:
    if kind(p) != "interior": continue
    for r in range(4):
        q = tuple(p[(i - r) % 4] for i in range(4))  # top,right,bottom,left
        W[q[3] - 6, q[0] - 6, q[1] - 6, q[2] - 6] += 1
W_top = W.sum(axis=1)                       # free top: [left, right, down]
M = W.transpose(0, 1, 2, 3).reshape(C * C, C * C)  # (left, up) -> (right, down)
def column(v, n):
    # v has shape (C,)*n indexed by the colours on the left edges of the n cells
    if n == 1:
        return W.sum(axis=(1, 3)).T @ v
    x = np.tensordot(v, W_top, axes=([0], [0]))            # [a2..an, b1, d1]
    for i in range(1, n):
        # bring (a_{i+1}, d_i) to the end: current axes [a_{i+1}..a_n, b1..b_i, d_i]
        x = np.moveaxis(x, 0, -2)                          # [a_{i+2}.., b1..b_i, a_{i+1}, d_i]
        sh = x.shape[:-2]
        x = x.reshape(-1, C * C) @ M                       # -> (b_{i+1}, d_{i+1})
        x = x.reshape(*sh, C, C)
    return x.sum(axis=-1)                                  # free bottom
out = []
for n in range(1, 6):
    t = time.time(); v = np.ones((C,) * n); lam = 0
    for it in range(60 if n < 5 else 14):
        w = column(v, n); new = np.linalg.norm(w.ravel()) / np.linalg.norm(v.ravel())
        v = w / np.linalg.norm(w.ravel())
        if abs(new - lam) < 1e-10 * new: lam = new; break
        lam = new
    h = math.log10(lam) / n
    mf = math.log10(784) - (2 - 1 / n) * math.log10(17)
    out.append((n, lam, h))
    print(f"n={n}: lambda_n={lam:.6g}  h(n)=log10(lambda_n)/n={h:.4f}  mean-field log10(784)-(2-1/n)log10(17)={mf:.4f}  iterations={it + 1}  {time.time() - t:.1f}s", flush=True)
print("mean-field limit log10(784/289) =", round(math.log10(784 / 289), 4))
for (n1, l1, _), (n2, l2, _) in zip(out, out[1:]):
    print(f"log10(lambda_{n2}/lambda_{n1}) = {math.log10(l2 / l1):.4f}   (increment per added row; tends to h_inf)")
# Richardson: h(n) = h_inf + c/n  =>  h_inf = n2*h(n2) - n1*h(n1) for consecutive n
print("fit h(n) = h_inf + c/n on the last two widths: h_inf =", round(out[-1][0] * out[-1][2] - out[-2][0] * out[-2][2], 4))
