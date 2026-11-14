"""TOY. Probe B: a flow inside the Birkhoff polytope (graduated assignment, Gold and Rangarajan 1996).

Variables X[p] >= 0 over rim-consistent placements p = (cell, tile, rotation). Constraints of the
relaxation: each cell sums to 1, each tile (over cells and rotations) sums to 1, i.e. a point of
the (rotation-lifted) Birkhoff polytope. Energy: expected number of matched inner edges,
E(X) = sum over adjacent pairs of sum_{p,q} X[p] X[q] [facing colours equal], a quadratic form.
Graduated assignment: X <- Sinkhorn(X_prev^(1-a) * exp(beta * dE/dX)) with beta raised geometrically;
as beta grows X is pushed toward a vertex (a permutation). Rounded at the end by a max-weight
assignment (Hungarian) and scored by matched edges.

Measured per board (every solution enumerated first):
  success      = share of runs whose rounded board is a solution;
  best_matched = matched edges of the rounded board / inner edges;
  (measures below as implemented in stats(): expected agreeing cells and bits, z of the solutions'
   mean overlap against the spread of 200 control boards; rounded board's distance to the
   nearest solution against the nearest of 200 controls)
  info_sol     = sum_cells log2(X[c,s_c] / X0[c,s_c]) averaged (and maxed) over solutions s, at the
                 point of the schedule where it most exceeds the control; X0 = the beta=0 point
                 (Sinkhorn of the uniform point: the polytope's centre on this face). This is how
                 many bits the flow concentrates on the solutions;
  info_ctrl    = the same quantity averaged (and maxed) over 200 random rim-consistent boards
                 with the pin. info_sol is a signal only where it beats info_ctrl.
  Medians over runs.
Usage: probe_b_birkhoff_flow.py n cf ci seed runs [traj 0|1] [max_mismatch 2] [anneal_steps 15000] [tries 150]
"""
import sys, os, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
from scipy.optimize import linear_sum_assignment
from common import gen, enumerate_all, oriented, pin_of

n, cf, ci, seed, runs = map(int, sys.argv[1:6])
TRAJ = len(sys.argv) > 6 and sys.argv[6] not in ("0",)
MAXM = int(sys.argv[7]) if len(sys.argv) > 7 else 2
STEPS = int(sys.argv[8]) if len(sys.argv) > 8 else 15000
TRIES = int(sys.argv[9]) if len(sys.argv) > 9 else 150
NEARGEN = sys.argv[10] if len(sys.argv) > 10 else "anneal"   # anneal | dfs
tiles = gen(n, cf, ci, seed)
N = n * n
T = len(tiles)
base = enumerate_all(n, tiles)
sols = base["sols"]
pin = pin_of(n, tiles)

orient = oriented(n, tiles)
cellOf, tileOf, rotOf, cols = [], [], [], []
for x in range(N):
    for (i, k, p) in orient[x]:
        if x == 0 and (i, k) != (pin[0], pin[1]):
            continue
        if x != 0 and i == pin[0]:
            continue
        cellOf.append(x); tileOf.append(i); rotOf.append(k); cols.append(p)
cellOf = np.array(cellOf); tileOf = np.array(tileOf); rotOf = np.array(rotOf); cols = np.array(cols)
P = len(cellOf)
K = int(cols.max()) + 1
index = {(c, t, k): j for j, (c, t, k) in enumerate(zip(cellOf, tileOf, rotOf))}

# neighbour table: direction d in N,E,S,W; opposite side (d+2)%4
nb = np.full((N, 4), -1)
for x in range(N):
    r, c = divmod(x, n)
    if r > 0: nb[x, 0] = x - n
    if c < n - 1: nb[x, 1] = x + 1
    if r < n - 1: nb[x, 2] = x + n
    if c > 0: nb[x, 3] = x - 1
inner_edges = 2 * n * (n - 1)


def sinkhorn(X, it=30):
    for _ in range(it):
        cs = np.bincount(cellOf, X, N)
        X = X / cs[cellOf]
        ts = np.bincount(tileOf, X, T)
        X = X / ts[tileOf]
    cs = np.bincount(cellOf, X, N)
    return X / cs[cellOf]


def grad(X):
    # S[c,d,k] = mass of colour k on side d of cell c
    S = np.zeros((N, 4, K))
    for d in range(4):
        np.add.at(S[:, d, :], (cellOf, cols[:, d]), X)
    Q = np.zeros(P)
    for d in range(4):
        nbr = nb[cellOf, d]
        has = nbr >= 0
        Q[has] += S[nbr[has], (d + 2) % 4, cols[has, d]]
    return Q


def round_board(X):
    W = np.full((N, T), -1e9)
    best_k = {}
    for j in np.argsort(X):          # ascending, so the largest rotation weight wins
        W[cellOf[j], tileOf[j]] = X[j]
        best_k[(cellOf[j], tileOf[j])] = rotOf[j]
    r, c = linear_sum_assignment(-W)
    board = [None] * N
    for x, t in zip(r, c):
        board[x] = (t, best_k.get((x, t), 0))
    sides = [tuple(tiles[t][k:] + tiles[t][:k]) for t, k in board]
    m = 0
    for x in range(N):
        if nb[x, 1] >= 0 and sides[x][1] == sides[x + 1][3]: m += 1
        if nb[x, 2] >= 0 and sides[x][2] == sides[x + n][0]: m += 1
    return board, m


sol_idx = [np.array([index.get((x, a[0], a[1]), -1) for x, a in enumerate(s)]) for s in sols]
rng = np.random.default_rng(1)
# control boards: random rim-consistent assignment with the pin (corners/edges/interior shuffled)
ctrl_idx = []
kinds = {}
for x in range(N):
    kinds.setdefault(sum(rim_ for rim_ in (x // n == 0, x % n == n - 1, x // n == n - 1, x % n == 0)), []).append(x)
for _ in range(200):
    js = np.full(N, -1)
    for kind, cells in kinds.items():
        cand_tiles = sorted({tileOf[j] for j in range(P) if cellOf[j] in cells})
        perm = rng.permutation(cand_tiles)
        free_cells = [x for x in cells if x != 0]
        perm = [t for t in perm if t != pin[0]]
        for x, t in zip(free_cells, perm):
            opts = [j for j in range(P) if cellOf[j] == x and tileOf[j] == t]
            js[x] = opts[0] if opts else -1
    js[0] = index[(0, pin[0], pin[1])]
    ctrl_idx.append(js)

X0 = sinkhorn(np.ones(P))

# near-solution controls: annealed boards with at most 2 mismatched inner edges, 8+ cells from every
# solution (the null that matters: boards a local consistency test cannot tell from a solution)
srng = np.random.default_rng(7)
rim_cells = {}
for x in range(N):
    rim_cells.setdefault(sum((x // n == 0, x % n == n - 1, x // n == n - 1, x % n == 0)), []).append(x)
opt = {}                      # (cell, tile) -> list of (j, sides)
for j in range(P):
    opt.setdefault((cellOf[j], tileOf[j]), []).append(j)


import random as _r
_R = _r.Random(7)
COLS = cols.tolist(); NB = nb.tolist(); TILE = tileOf.tolist()
OPT = {k: list(v) for k, v in opt.items()}


def mism_cell(x, js):
    m = 0
    cx = COLS[js[x]]
    for d in range(4):
        y = NB[x][d]
        if y >= 0 and cx[d] != COLS[js[y]][(d + 2) % 4]:
            m += 1
    return m


def anneal(steps=STEPS):
    js = [int(v) for v in ctrl_idx[_R.randrange(len(ctrl_idx))]]
    kinds_l = [[x for x in rim_cells[k] if x != 0] for k in range(3)]
    for st in range(steps):
        temp = 1.5 * (1 - st / steps) + 0.05
        cells = kinds_l[_R.randrange(3)]
        if len(cells) < 2:
            continue
        x, y = _R.sample(cells, 2)
        if cells is kinds_l[0] and _R.random() < 0.3:
            y = x
        aff = {x, y} | {z for z in NB[x] if z >= 0} | {z for z in NB[y] if z >= 0}
        before = sum(mism_cell(z, js) for z in aff)
        old = (js[x], js[y])
        tx, ty = TILE[js[x]], TILE[js[y]]
        if x == y:
            js[x] = _R.choice(OPT[(x, tx)])
        else:
            js[x] = _R.choice(OPT[(x, ty)])
            js[y] = _R.choice(OPT[(y, tx)])
        delta = (sum(mism_cell(z, js) for z in aff) - before) / 2
        if not (delta <= 0 or _R.random() < np.exp(-delta / temp)):
            js[x], js[y] = old
    return np.array(js), int(round(sum(mism_cell(x, js) for x in range(N)) / 2))


def dfs_near(budget=3_000_000):
    """Randomised row-scan DFS that tolerates up to MAXM mismatched inner edges; returns complete
    boards (as placement index arrays) at 8+ cells from every solution, restarted with new value
    orders until 20 are found or the node budget is spent."""
    by_cell = [[j for j in range(P) if cellOf[j] == x] for x in range(N)]
    found = []
    nodes = [0]

    def rec(x, js, used, mm):
        if len(found) >= 20 or nodes[0] > budget:
            return
        nodes[0] += 1
        if x == N:
            arr = np.array(js)
            if min(int((s_ != arr).sum()) for s_ in sol_idx) >= 8:
                found.append(arr)
                raise StopIteration
            return
        r, c = divmod(x, n)
        cand = by_cell[x][:]
        _R.shuffle(cand)
        for j in cand:
            t = TILE[j]
            if used[t]:
                continue
            add = 0
            if r > 0 and COLS[j][0] != COLS[js[x - n]][2]:
                add += 1
            if c > 0 and COLS[j][3] != COLS[js[x - 1]][1]:
                add += 1
            if mm + add > MAXM:
                continue
            used[t] = True
            js.append(j)
            rec(x + 1, js, used, mm + add)
            js.pop()
            used[t] = False

    while len(found) < 20 and nodes[0] <= budget:
        used = [False] * T
        used[pin[0]] = True
        try:
            rec(1, [index[(0, pin[0], pin[1])]], used, 0)
        except StopIteration:
            pass
    return found, nodes[0]


near_idx = []
tries = 0
if NEARGEN == "dfs":
    near_idx, tries = dfs_near()
else:
    while len(near_idx) < 20 and tries < TRIES:
        tries += 1
        js, m = anneal()
        if m <= MAXM and min(int((s != js).sum()) for s in sol_idx) >= 8:
            near_idx.append(js.copy())


def stats(X, idxs):
    """mean and max over boards of: expected agreeing cells sum_c X[c,s_c]; bits sum_c log2(X/X0)."""
    ok = [s for s in idxs if not (s < 0).any()]
    ov = np.array([X[s].sum() for s in ok])
    bits = np.array([np.log2(np.maximum(X[s], 1e-12) / X0[s]).sum() for s in ok])
    return ov, bits


def entropy(X):
    h = -X * np.log2(np.maximum(X, 1e-300))
    return float(np.bincount(cellOf, h, N).mean())


def nearest(board, boards_idx):
    js = np.array([index.get((x, a[0], a[1]), -2) for x, a in enumerate(board)])
    return int(min((s != js).sum() for s in boards_idx))


out = dict(board=[n, cf, ci, seed], n_sols=len(sols), dfs_nodes=base["nodes"], P=P, runs=runs,
           near_controls=len(near_idx), near_tries=tries, near_max_mismatch=MAXM, near_gen=NEARGEN)
succ = 0; matched = []; best = []; traj = []; dsol = []; dctrl = []; iters = 0
for run in range(runs):
    X = sinkhorn(X0 * np.exp(0.05 * rng.standard_normal(P)))
    beta = 0.5
    gap = (-1e9, None)
    while beta < 200:
        for _ in range(4):
            Q = grad(X)
            Y = np.exp(beta * (Q - Q.max())) * np.maximum(X, 1e-300) ** 0.5
            X = sinkhorn(np.maximum(Y, 1e-200))
            iters += 1
        ovs, bs = stats(X, sol_idx); ovc, bc = stats(X, ctrl_idx)
        if near_idx:
            ovn, bn = stats(X, near_idx)
        else:
            ovn, bn = np.array([np.nan]), np.array([np.nan])
        z = (ovs.mean() - ovc.mean()) / max(1e-9, ovc.std())
        row = dict(beta=round(beta, 2), H=round(entropy(X), 3), ov_sol=round(float(ovs.mean()), 3),
                   ov_sol_max=round(float(ovs.max()), 3), ov_ctrl=round(float(ovc.mean()), 3),
                   ov_ctrl_sd=round(float(ovc.std()), 3), ov_ctrl_max=round(float(ovc.max()), 3),
                   bits_sol=round(float(bs.mean()), 2), bits_sol_max=round(float(bs.max()), 2),
                   bits_ctrl=round(float(bc.mean()), 2), bits_ctrl_max=round(float(bc.max()), 2), z=round(float(z), 2),
                   bits_near=round(float(bn.mean()), 2), bits_near_max=round(float(bn.max()), 2),
                   z_vs_near=round(float((bs.mean() - bn.mean()) / max(1e-9, bn.std())), 2))
        if run == 0:
            traj.append(row)
        if z > gap[0]:
            gap = (z, row)
        beta *= 1.15
    board, m = round_board(X)
    matched.append(m / inner_edges)
    succ += m == inner_edges
    best.append(gap[1])
    dsol.append(nearest(board, sol_idx)); dctrl.append(nearest(board, ctrl_idx))
out.update(success=succ / runs, matched_mean=float(np.mean(matched)), matched_max=float(np.max(matched)),
           best_z_median=float(np.median([b["z"] for b in best])),
           best_rows=best[:3],
           rounded_dist_to_nearest_solution_median=float(np.median(dsol)),
           rounded_dist_to_nearest_control_median=float(np.median(dctrl)),
           flow_iters_per_run=iters / runs, log2_dfs_nodes=float(np.log2(base["nodes"])))
print(json.dumps(out), flush=True)
if TRAJ:
    for r in traj:
        print(json.dumps(r))
