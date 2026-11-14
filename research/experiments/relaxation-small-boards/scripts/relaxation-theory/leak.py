"""TOY BOARDS ONLY. Exactness test of the degree-2 lift on small rectangular boards.

Usage: uv run --with scs --with numpy --with scipy python -I -B leak.py R C framed cf ci seed [tlim] [mode]
  framed=1: rim colour 0, ring edges from cf colours, other edges from ci colours (the lift's toy rule).
  framed=0: no rim; every side (outer ones too) drawn from ci colours; any tile in any cell in 4 rotations.
  mode=sat (default): the planted board; mode=swapK: K exchanges of side colours between tiles (the
  colour census is kept), which usually leaves no solution. With ONLY_UNSAT=1 boards that still have
  a solution are skipped.
The pin is the planted tile of cell 0 in its planted rotation.
Test: maximise the total mass on placements that are in no solution, over the lift (SDP) and over
its LP shadow (same linear constraints, no PSD). 0 means the relaxation is supported on solution
placements. For a board with no solution the solver status is the result (infeasible = refuted).
The relaxation itself is lift.build of geometry-sdp-lift (imported read only, -B).
"""
import os, sys, json, time, random
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "geometry-sdp-lift"))
import numpy as np
import scs
from lift import build

R, C, framed, cf, ci, seed = map(int, sys.argv[1:7])
tlim = float(sys.argv[7]) if len(sys.argv) > 7 else 500.0
mode = sys.argv[8] if len(sys.argv) > 8 else "sat"
rng = random.Random(seed)
N = R * C


def paint():
    hor = [[0] * (C + 1) for _ in range(R)]      # hor[r][c]: side between (r,c-1) and (r,c)
    ver = [[0] * C for _ in range(R + 1)]        # ver[r][c]: side between (r-1,c) and (r,c)
    for r in range(R):
        for c in range(C + 1):
            outer = c in (0, C)
            if outer:
                hor[r][c] = 0 if framed else cf + rng.randint(1, ci)
            else:
                ring = framed and r in (0, R - 1)
                hor[r][c] = rng.randint(1, cf) if ring else cf + rng.randint(1, ci)
    for r in range(R + 1):
        for c in range(C):
            outer = r in (0, R)
            if outer:
                ver[r][c] = 0 if framed else cf + rng.randint(1, ci)
            else:
                ring = framed and c in (0, C - 1)
                ver[r][c] = rng.randint(1, cf) if ring else cf + rng.randint(1, ci)
    return [(ver[r][c], hor[r][c + 1], ver[r + 1][c], hor[r][c]) for r in range(R) for c in range(C)]


for _ in range(20000):
    tiles = paint()
    canon = set(); ok = True
    for t in tiles:
        rots = [t[k:] + t[:k] for k in range(4)]
        if len(set(rots)) < 4:
            ok = False
        m = min(rots)
        if m in canon:
            ok = False
        canon.add(m)
    if ok:
        break
else:
    raise SystemExit("no board")
planted = list(tiles)
if mode.startswith("swap"):      # exchange the colours of two inner-class sides of two tiles, k times:
    k_sw = int(mode[4:])         # colour counts stay balanced, so no counting argument refutes the set
    tl = [list(t) for t in tiles]
    done = 0
    while done < k_sw:
        a, b = rng.randrange(N), rng.randrange(N)
        da = [d for d in range(4) if tl[a][d] > cf]; db = [d for d in range(4) if tl[b][d] > cf]
        if a == b or not da or not db:
            continue
        i, j = rng.choice(da), rng.choice(db)
        if tl[a][i] == tl[b][j]:
            continue
        tl[a][i], tl[b][j] = tl[b][j], tl[a][i]
        done += 1
    tiles = [tuple(t) for t in tl]


def rimflags(x):
    r, c = divmod(x, C)
    return (r == 0, c == C - 1, r == R - 1, c == 0)


cellOf, tileOf, rotOf, cols = [], [], [], []
for x in range(N):
    for i, t in enumerate(tiles):
        for k in range(4):
            p = t[k:] + t[:k]
            if framed and not all((s == 0) == q for s, q in zip(p, rimflags(x))):
                continue
            if (x == 0) != (i == 0):
                continue
            if x == 0 and k != 0:
                continue
            cellOf.append(x); tileOf.append(i); rotOf.append(k); cols.append(p)
nbl = []
for x in range(N):
    r, c = divmod(x, C)
    nbl.append([x - C if r > 0 else -1, x + 1 if c < C - 1 else -1, x + C if r < R - 1 else -1, x - 1 if c > 0 else -1])


class B:
    pass


B.P = len(cellOf); B.N = N; B.T = N
B.cellOf = np.array(cellOf); B.tileOf = np.array(tileOf); B.cols = np.array(cols); B.nb = np.array(nbl)
P = B.P
# all solutions, row scan
bycell = [[j for j in range(P) if cellOf[j] == x] for x in range(N)]
sols = []; nodes = [0]
used = [False] * N; js = []


def rec(x):
    nodes[0] += 1
    if x == N:
        sols.append(list(js)); return
    r, c = divmod(x, C)
    for j in bycell[x]:
        if used[tileOf[j]]:
            continue
        if r > 0 and cols[j][0] != cols[js[x - C]][2]:
            continue
        if c > 0 and cols[j][3] != cols[js[x - 1]][1]:
            continue
        used[tileOf[j]] = True; js.append(j)
        rec(x + 1)
        js.pop(); used[tileOf[j]] = False


rec(0)
insol = np.zeros(P, bool)
for s in sols:
    insol[s] = True
out = dict(board=[R, C, framed, cf, ci, seed], mode=mode, P=P, n_sols=len(sols), dfs_nodes=nodes[0],
           in_some_solution=int(insol.sum()), placements_per_cell=round(P / N, 1))
if os.environ.get("ONLY_UNSAT") == "1" and sols:
    out["skipped"] = "has a solution"
    print(json.dumps(out)); raise SystemExit
w = (~insol).astype(float) if sols else np.zeros(P)
for name, psd in (("lp", False), ("sdp", True)):
    data, cone, lay = build(B, psd=psd, obj="lin", w=w)
    t0 = time.time()
    sol = scs.SCS(data, cone, eps_abs=1e-5, eps_rel=1e-5, max_iters=200000, time_limit_secs=tlim, verbose=False).solve()
    x = sol["x"][lay["ix"]]
    r = dict(status=sol["info"]["status"], iters=int(sol["info"]["iter"]), seconds=round(time.time() - t0, 1),
             free_pairs=int(lay["F"]), zero_pair_share=round(1 - lay["F"] / (P * (P - 1) / 2), 4))
    if sols and "solved" in sol["info"]["status"]:
        r["max_mass_off_solutions"] = round(float(x[~insol].sum()), 5)
        r["per_cell"] = round(float(x[~insol].sum()) / N, 5)
        r["placements_above_1e-3_off_solutions"] = int((x[~insol] > 1e-3).sum())
    out[name] = r
print(json.dumps(out))
