"""Review check 1: strip entropy h(n) for n = 1..6 and the distinct-piece
fraction rho on patches larger than 3x3, by a route different from the memo.

Memo route: power iteration on the column-to-column strip operator.
This route: a cell-by-cell broken-profile count, written from scratch.
  - A(n) for n = 2, 3, 4 in exact int64, to be compared with the committed
    counts (4550669, 195941569440, 62607390876341976).
  - lambda_n as the row-to-row growth of the count of n-wide patches.
  - rho on 2x2, 3x3 (check against the committed exact 0.892166, 0.647680),
    then 4x4, 4x6, 4x8, 4x12 by exact uniform sampling of colour-valid
    patches (backward completion counts, forward sampling).

Run: uv run --with numpy python 01_entropy_and_rho.py
"""
import os
import sys
import time
from math import comb, exp, log, log10, sqrt

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
CSV = os.path.abspath(os.path.join(HERE, *[".."] * 6, "engine", "data", "official_eternity2.csv"))
T0 = time.time()

lines = [l.strip() for l in open(CSV) if l.strip()][1:]
pieces = []
for l in lines:
    c = l.split(",")
    pieces.append(tuple(0 if int(x, 2) == 65535 else int(x, 2) for x in c[:4]))  # top right bottom left
interior = [(i, p) for i, p in enumerate(pieces) if 0 not in p]
cols = sorted({v for _, p in interior for v in p})
K = len(cols)
cid = {v: k for k, v in enumerate(cols)}
TILES = []  # (piece id, top, right, bottom, left) in 0..K-1
for i, p in interior:
    for k in range(4):
        q = p[4 - k:] + p[:4 - k] if k else p
        TILES.append((i, cid[q[0]], cid[q[1]], cid[q[2]], cid[q[3]]))
assert len(interior) == 196 and K == 17 and len(set(t[1:] for t in TILES)) == 784
print("interior pieces %d, colours %d, oriented tiles %d (all distinct)" % (len(interior), K, len(TILES)))


def row_step(v, n, dtype):
    """v: array shape (K,)*n, count by profile of bottom colours. Returns the
    array after one more row of n cells (free left and right ends)."""
    R = K ** (n - 1)
    if n == 1:
        out = np.zeros(K, dtype)
        for _, t, r, b, l in TILES:
            out[b] += v[t]
        return out
    cur = v.reshape(K, R)  # (t = p_0, rest)
    # cell 0: no left neighbour
    o2 = np.zeros((K, K, R), dtype)  # (r, b, rest)
    for _, t, r, b, l in TILES:
        o2[r, b] += cur[t]
    cur = np.ascontiguousarray(o2.reshape(K, K, K, R // K).transpose(0, 2, 3, 1)).reshape(K, K, R)  # (l, p_1, rest.. , b_0)
    del o2
    for i in range(1, n - 1):
        o2 = np.zeros((K, K, R), dtype)
        for _, t, r, b, l in TILES:
            o2[r, b] += cur[l, t]
        cur = np.ascontiguousarray(o2.reshape(K, K, K, R // K).transpose(0, 2, 3, 1)).reshape(K, K, R)
        del o2
    # last cell: no right neighbour; cur is (l, t = p_{n-1}, rest = b_0..b_{n-2})
    o = np.zeros((K, R), dtype)  # (b, rest)
    for _, t, r, b, l in TILES:
        o[b] += cur[l, t]
    return np.ascontiguousarray(o.T).reshape((K,) * n)  # (b_0..b_{n-2}, b_{n-1})


print("\n== A(n): exact int64 block counts against the committed file ==")
committed = {2: 4550669, 3: 195941569440, 4: 62607390876341976}
for n in (2, 3, 4):
    v = np.ones((K,) * n, np.int64)
    # first row has no top neighbour: a profile entry of 1 for every top colour counts each tile once
    for _ in range(n):
        v = row_step(v, n, np.int64)
        v_sum = int(v.sum())
        # after the first row, later rows read the profile; nothing else to do
    print("  A(%d) = %d   committed %d   %s" % (n, v_sum, committed[n], "match" if v_sum == committed[n] else "MISMATCH"))

print("\n== strip growth lambda_n (rows of n cells, many rows) ==")
lam = {}
for n in range(1, 7):
    dt = np.float32 if n == 6 else np.float64
    v = np.ones((K,) * n, dt)
    prev = None
    t1 = time.time()
    hist = []
    for it in range(40):
        v = row_step(v, n, dt)
        s = float(v.sum(dtype=np.float64))
        v /= s
        hist.append(s)
        if it >= 3 and abs(hist[-1] / hist[-2] - 1) < (2e-6 if n < 6 else 2e-5):
            break
    lam[n] = hist[-1]
    print("  n=%d: lambda=%.4f  h(n)=log10(lambda)/n=%.4f  rows=%d  last three growth factors %s  [%.0fs]"
          % (n, lam[n], log10(lam[n]) / n, len(hist), " ".join("%.3f" % x for x in hist[-3:]), time.time() - t1))
    sys.stdout.flush()
print("  increments log10(lambda_{n+1}/lambda_n): " + " ".join("%.4f" % log10(lam[n + 1] / lam[n]) for n in range(1, 6)))
print("  page values h(1..4): 1.6645 1.0497 0.8449 0.7425 ; n*h(n) differences from the page's own four numbers: %.4f %.4f %.4f"
      % (2 * 1.0497 - 1.6645, 3 * 0.8449 - 2 * 1.0497, 4 * 0.7425 - 3 * 0.8449))
print("  Fekete: h_inf = inf h(n) <= h(6) = %.4f ; the page says the limit is about 0.67" % (log10(lam[6]) / 6))
print("  mean field log10(784/289) = %.4f" % log10(784 / 289))


def sample_rho(W, H, S, rng):
    """uniform colour-valid W-wide, H-high patches (reusable tiles); returns
    (count A, number of samples with all pieces distinct, mean number of
    equal adjacent pairs, mean number of equal non-adjacent pairs)."""
    n = W
    R = K ** n
    tid = np.array([t[0] for t in TILES])
    tt = np.array([t[1] for t in TILES])
    tr = np.array([t[2] for t in TILES])
    tb = np.array([t[3] for t in TILES])
    tl = np.array([t[4] for t in TILES])
    ncell = W * H
    # state before cell k = (profile p_0..p_{n-1}, left colour). Completion counts C[k][l, profile-flat].
    # profile flat index: sum p_j * K^j
    pw = [K ** j for j in range(n)]
    C = [None] * (ncell + 1)
    C[ncell] = np.ones((K, R))
    for k in range(ncell - 1, -1, -1):
        row, col = divmod(k, W)
        nxt = C[k + 1].reshape((K,) + (K,) * n)  # (left, p_{n-1}, ..., p_0) because flat index weights p_0 least
        ax = 1 + (n - 1 - col)  # axis of p_col
        nx = np.moveaxis(nxt, ax, 1)  # (left', p_col', others)
        out = np.zeros_like(nx)  # (left, p_col, others)
        first_row, first_col, last_col = row == 0, col == 0, col == W - 1
        for j in range(len(TILES)):
            t, r, b, l = tt[j], tr[j], tb[j], tl[j]
            src = nx[r, b] if not last_col else nx[0, b]  # after the last cell of a row the left colour is unused: all entries equal
            if first_row and first_col:
                out[:, :] += src
            elif first_row:
                out[l, :] += src
            elif first_col:
                out[:, t] += src
            else:
                out[l, t] += src
        C[k] = np.ascontiguousarray(np.moveaxis(out, 1, ax)).reshape(K, R)
    A = C[0][0, 0]
    # forward sampling
    prof = np.zeros((S, n), np.int64)
    left = np.zeros(S, np.int64)
    ids = np.zeros((S, ncell), np.int64)
    for k in range(ncell):
        row, col = divmod(k, W)
        first_row, first_col, last_col = row == 0, col == 0, col == W - 1
        newflat_base = (prof * np.array(pw)).sum(1) - prof[:, col] * pw[col]
        w = np.zeros((S, len(TILES)))
        nl = np.zeros(len(TILES), np.int64) if last_col else tr
        flat = newflat_base[:, None] + tb[None, :] * pw[col]
        w = C[k + 1][nl[None, :], flat]
        if not first_row:
            w = w * (tt[None, :] == prof[:, col][:, None])
        if not first_col:
            w = w * (tl[None, :] == left[:, None])
        cs = np.cumsum(w, axis=1)
        u = rng.random(S) * cs[:, -1]
        pick = (cs < u[:, None]).sum(1)
        pick = np.minimum(pick, len(TILES) - 1)
        ids[:, k] = tid[pick]
        prof[:, col] = tb[pick]
        left = np.where(last_col, 0, tr[pick])
    srt = np.sort(ids, axis=1)
    distinct = int((np.diff(srt, axis=1) != 0).all(axis=1).sum())
    g = ids.reshape(S, H, W)
    adj = float(((g[:, :, 1:] == g[:, :, :-1]).sum((1, 2)) + (g[:, 1:, :] == g[:, :-1, :]).sum((1, 2))).mean())
    allpairs = float(sum((ids[:, a] == ids[:, b]).mean() for a in range(ncell) for b in range(a + 1, ncell)))
    return A, distinct, adj, allpairs - adj


print("\n== distinct-piece fraction rho by exact uniform sampling of colour-valid patches ==")
print("  memo law: -ln rho = 0.0209 x (adjacent pairs) + C(m,2)/196 ; page fit: rho = exp(-0.0445 x cells)")
rng = np.random.default_rng(20261113)
for W, H, S, exact in ((2, 2, 200000, 0.892166), (3, 3, 200000, 0.647680), (4, 4, 200000, None), (4, 6, 200000, None), (4, 8, 200000, None), (4, 12, 300000, None)):
    t1 = time.time()
    A, d, adj, far = sample_rho(W, H, S, rng)
    m = W * H
    pairs = 2 * m - W - H
    rho = d / S
    se = sqrt(max(rho * (1 - rho), 1e-12) / S)
    law = exp(-0.0209 * pairs - comb(m, 2) / 196)
    page = exp(-0.0445 * m)
    print("  %dx%-2d cells %2d: A=%.6e  rho=%.5f (s.e. %.5f, %d of %d)%s | memo law %.5f | page fit %.5f | E[equal adjacent pairs]=%.4f (per pair %.4f) E[equal non-adjacent pairs]=%.4f (per pair %.5f, 1/196=%.5f)  [%.0fs]"
          % (W, H, m, A, rho, se, d, S, ("  exact %.6f" % exact) if exact else "", law, page, adj, adj / pairs, far,
             far / (comb(m, 2) - pairs), 1 / 196, time.time() - t1))
    sys.stdout.flush()
print("[total %.0fs]" % (time.time() - T0))
