"""TOY BOARDS ONLY (5x5). A hierarchy of windows glued by consistency, with each-tile-once carried
LINEARLY (a transportation polytope on x), no semidefinite cone:

  x[p] per placement; cell sums and tile sums equal 1;
  y[e, p, q] per inner edge e and compatible pair (matching sides, different tiles), marginals x;
  mode "2x2": z[W, f] per 2x2 window W and filling f (4 distinct tiles, 4 matched inner sides),
              with the pair marginal of z on each of the window's 4 edges equal to y.
Objective: maximise the mass on placements no solution uses (the theory memo's test of exactness).
Usage: winlp.py n cf ci seed mode(edge|2x2) [time_limit]"""
import os, sys, json, time
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, os.path.join(HERE, "bmcopy"))
import common  # noqa: F401,E402
import numpy as np  # noqa: E402
import scipy.sparse as sp  # noqa: E402
from scipy.optimize import linprog  # noqa: E402
from board import Board  # noqa: E402

n, cf, ci, seed = map(int, sys.argv[1:5])
assert n <= 5
mode = sys.argv[5]
tl = float(sys.argv[6]) if len(sys.argv) > 6 else 540.0
B = Board(n, cf, ci, seed)
P, N, T = B.P, B.N, B.T
sol = np.zeros(P, bool)
for s in B.sol_idx:
    sol[s] = True
cells = [[j for j in range(P) if B.cellOf[j] == x] for x in range(N)]
COLS, TILE = B.COLS, B.TILE
nv = P
rows, cols, vals, rhs = [], [], [], []
nr = 0


def add_row(idx, coef, b):
    global nr
    rows.extend([nr] * len(idx)); cols.extend(idx); vals.extend(coef); rhs.append(b); nr += 1


for x in range(N):
    add_row(cells[x], [1.0] * len(cells[x]), 1.0)
for t in range(T):
    g = [j for j in range(P) if TILE[j] == t]
    add_row(g, [1.0] * len(g), 1.0)
ypair = {}
for x in range(N):
    for d in (1, 2):
        y = int(B.nb[x, d])
        if y < 0:
            continue
        e = (x, y)
        byp, byq = {}, {}
        for p in cells[x]:
            for q in cells[y]:
                if TILE[p] != TILE[q] and COLS[p][d] == COLS[q][(d + 2) % 4]:
                    ypair[(p, q)] = nv
                    byp.setdefault(p, []).append(nv); byq.setdefault(q, []).append(nv); nv += 1
        for p in cells[x]:
            v = byp.get(p, [])
            add_row(v + [p], [1.0] * len(v) + [-1.0], 0.0)
        for q in cells[y]:
            v = byq.get(q, [])
            add_row(v + [q], [1.0] * len(v) + [-1.0], 0.0)
n_y = nv - P
n_z = 0
if mode == "2x2":
    for r in range(n - 1):
        for c in range(n - 1):
            a, b, cc, dd = r * n + c, r * n + c + 1, (r + 1) * n + c, (r + 1) * n + c + 1
            acc = {}
            for pa in cells[a]:
                for pb in cells[b]:
                    if (pa, pb) not in ypair:
                        continue
                    for pc in cells[cc]:
                        if (pa, pc) not in ypair or TILE[pc] == TILE[pb]:
                            continue
                        for pd in cells[dd]:
                            if (pb, pd) in ypair and (pc, pd) in ypair and TILE[pd] != TILE[pa]:
                                zi = nv; nv += 1; n_z += 1
                                for key in ((pa, pb), (pa, pc), (pb, pd), (pc, pd)):
                                    acc.setdefault(key, []).append(zi)
            for (u, v_) in ((a, b), (a, cc), (b, dd), (cc, dd)):
                for p in cells[u]:
                    for q in cells[v_]:
                        if (p, q) in ypair:
                            zs = acc.get((p, q), [])
                            add_row(zs + [ypair[(p, q)]], [1.0] * len(zs) + [-1.0], 0.0)
A = sp.csr_matrix((vals, (rows, cols)), shape=(nr, nv))
cvec = np.zeros(nv); cvec[:P][~sol] = -1.0
t0 = time.time()
res = linprog(cvec, A_eq=A, b_eq=np.array(rhs), bounds=(0, None), method="highs",
              options=dict(time_limit=tl, presolve=True))
out = dict(board=[n, cf, ci, seed], mode=mode, P=P, n_y=n_y, n_z=n_z, rows=nr, status=int(res.status),
           message=str(res.message)[:80], seconds=round(time.time() - t0, 1), n_sols=len(B.sols))
if res.x is not None:
    x = res.x[:P]
    out.update(max_mass_off_per_cell=float(x[~sol].sum() / N),
               off_placements_above_1e6=int((x[~sol] > 1e-6).sum()), off_placements=int((~sol).sum()))
print(json.dumps(out))
