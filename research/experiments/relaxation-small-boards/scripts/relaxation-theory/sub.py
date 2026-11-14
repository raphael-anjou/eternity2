"""TOY BOARDS ONLY. The lift on a sub-problem of a generated board: some cells pinned, the rest free.

Usage: uv run --with scs --with numpy --with scipy python -I -B sub.py n cf ci seed spec tlim [parts] [pmax]
  spec  true:frame:s        pin the rim cells as in solution s
        true:rows<k>:s      pin the first k rows as in solution s
        true:frame+rows<k>:s
        dead:<d>:<j>        pin the first d cells (row scan) as in the j-th sampled DEAD prefix: a prefix
                            that matches everywhere and uses each tile once but has no solution below
                            (sampled with random.Random(seed*1000+d), without replacement)
  parts comma list of lp, sdp, theta (theta = the lift without the entrywise Y >= 0; default lp,sdp)
Why this bears on the full board: pinning placements of a solution only adds equalities x_p = 1 to the
lift, so a point of the pinned lift with mass outside the solutions is a point of the full lift with
mass outside the solutions. A leak here proves the full board's lift is not exact. For a dead prefix
there is no solution to average over: "infeasible" is a refutation, "solved" is a failure to refute.
"""
import os, sys, json, time, random
here = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(here, "..", "geometry-sdp-lift"))
sys.path.insert(0, os.path.join(here, "..", "far-ideas-geometry"))
import numpy as np
import scs
from common import gen, enumerate_all, oriented
from lift import build

n, cf, ci, seed = map(int, sys.argv[1:5])
spec = sys.argv[5]; tlim = float(sys.argv[6])
parts = (sys.argv[7] if len(sys.argv) > 7 else "lp,sdp").split(",")
pmax = int(sys.argv[8]) if len(sys.argv) > 8 else 450
tiles = gen(n, cf, ci, seed)
N = n * n
orient = oriented(n, tiles)
kind, a, b = spec.split(":")
out = dict(tag=spec, board=[n, cf, ci, seed])
rot = lambda t, k: t[k:] + t[:k]
if kind == "true":
    base = enumerate_all(n, tiles)
    sols = base["sols"]
    s = sols[int(b)]
    S = set()
    for token in a.split("+"):
        if token == "frame":
            S |= {x for x in range(N) if x // n in (0, n - 1) or x % n in (0, n - 1)}
        elif token.startswith("rows"):
            S |= set(range(n * int(token[4:])))
    pinned = {x: s[x] for x in S}
    S.add(0); pinned[0] = s[0]
    sub_sols = [t for t in sols if all(t[x] == pinned[x] for x in pinned)]
else:
    d = int(a)
    base = enumerate_all(n, tiles, keep_depth=d)
    dead = [p for p, below in base["live"] if below == 0]
    out.update(live_at_depth=len(base["live"]), dead_at_depth=len(dead))
    R = random.Random(seed * 1000 + d)
    R.shuffle(dead)
    pref = dead[int(b)]
    pinned = {x: pref[x] for x in range(d)}
    sub_sols = []
free = [x for x in range(N) if x not in pinned]
used = {pinned[x][0] for x in pinned}
side = {x: rot(tiles[pinned[x][0]], pinned[x][1]) for x in pinned}
nbr = lambda x: [x - n if x // n > 0 else -1, x + 1 if x % n < n - 1 else -1, x + n if x // n < n - 1 else -1, x - 1 if x % n > 0 else -1]
ftiles = [i for i in range(len(tiles)) if i not in used]
cidx = {x: j for j, x in enumerate(free)}; tidx = {i: j for j, i in enumerate(ftiles)}
cellOf, tileOf, cols, key = [], [], [], {}
for x in free:
    for (i, k, p) in orient[x]:
        if i in used:
            continue
        if any(y in pinned and p[dd] != side[y][(dd + 2) % 4] for dd, y in enumerate(nbr(x)) if y >= 0):
            continue
        key[(x, i, k)] = len(cellOf)
        cellOf.append(cidx[x]); tileOf.append(tidx[i]); cols.append(p)


class B:
    pass


B.P = P = len(cellOf); B.N = len(free); B.T = len(ftiles)
B.cellOf = np.array(cellOf); B.tileOf = np.array(tileOf); B.cols = np.array(cols).reshape(P, 4)
B.nb = np.array([[cidx.get(y, -1) if y >= 0 else -1 for y in nbr(x)] for x in free]).reshape(len(free), 4)
insol = np.zeros(P, bool)
for t in sub_sols:
    for x in free:
        insol[key[(x, t[x][0], t[x][1])]] = True
out.update(free_cells=len(free), P=P, n_sols=len(sub_sols), in_some_solution=int(insol.sum()),
           cells_with_no_placement=int(len(free) - len(set(cellOf))))
# refutation size by plain row-scan DFS on the free cells (also the node count when solutions exist)
bycell = [[j for j in range(P) if cellOf[j] == c] for c in range(len(free))]
nodes = [0]; usedt = [False] * len(ftiles); js = []
found = [0]


def rec(c):
    nodes[0] += 1
    if c == len(free):
        found[0] += 1; return
    x = free[c]
    for j in bycell[c]:
        if usedt[tileOf[j]]:
            continue
        ok = True
        for dd in (0, 3):
            y = nbr(x)[dd]
            if y >= 0 and y in cidx and cols[j][dd] != cols[js[cidx[y]]][(dd + 2) % 4]:
                ok = False
        if ok:
            usedt[tileOf[j]] = True; js.append(j)
            rec(c + 1)
            js.pop(); usedt[tileOf[j]] = False


rec(0)
assert found[0] == len(sub_sols), (found[0], len(sub_sols))
out["dfs_nodes"] = nodes[0]
if P > pmax or out["cells_with_no_placement"]:
    out["skipped"] = "too large" if P > pmax else "empty cell (refuted by forward checking)"
    print(json.dumps(out)); raise SystemExit
w = (~insol).astype(float) if sub_sols else np.zeros(P)
for name in parts:
    data, cone, lay = build(B, psd=name != "lp", obj="lin", w=w)
    if name == "theta":                      # drop the rows y >= 0 (keep x >= 0)
        z, l, F = cone["z"], cone["l"], lay["F"]
        keep = np.r_[np.arange(z + P), np.arange(z + l, data["A"].shape[0])]
        data = dict(A=data["A"].tocsr()[keep].tocsc(), b=data["b"][keep], c=data["c"])
        cone = dict(z=z, l=P, s=cone["s"])
    t0 = time.time()
    sol = scs.SCS(data, cone, eps_abs=1e-5, eps_rel=1e-5, max_iters=200000, time_limit_secs=tlim, verbose=False).solve()
    x = sol["x"][lay["ix"]]
    r = dict(status=sol["info"]["status"], iters=int(sol["info"]["iter"]), seconds=round(time.time() - t0, 1))
    if sub_sols and "solved" in sol["info"]["status"]:
        r["max_mass_off_solutions"] = round(float(x[~insol].sum()), 5)
        r["per_cell"] = round(float(x[~insol].sum()) / len(free), 5)
        r["placements_above_1e-3_off_solutions"] = int((x[~insol] > 1e-3).sum())
    out[name] = r
print(json.dumps(out))
