"""Structural measurements on the five public record boards (committed bucas strings).

1. score, where the mismatches sit, colour-parity structure of the mismatch set
2. pairwise Hamming distances between the boards
3. exact checkerboard-class assignment (one colour class of the checkerboard refilled
   optimally, the other fixed): gain and number of cells moved
4. first-moment null model for the committed SAT halo verdicts
5. parity ledger lower bound on mismatches across every straight cut
Stdlib only: python3 -I 03_record_boards.py
"""
import json, math, os, sys
from collections import Counter
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib4 import *

boards = load_boards()
ids = list(boards)
g0 = boards[ids[0]]
pieces = Counter(canon(g0[r][c]) for r in range(N) for c in range(N))
assert len(pieces) == 256 and max(pieces.values()) == 1, "pieces must be 256 distinct"
for i in ids:
    assert Counter(canon(boards[i][r][c]) for r in range(N) for c in range(N)) == pieces, i
# colour classes
frame_cols, inner_cols = set(), set()
for t in pieces:
    k = t.count(0)
    for j, col in enumerate(t):
        if col == 0:
            continue
        if k and (t[(j - 1) % 4] == 0 or t[(j + 1) % 4] == 0):
            frame_cols.add(col)
        else:
            inner_cols.add(col)
print("frame colours", len(frame_cols), "interior colours", len(inner_cols), "overlap", len(frame_cols & inner_cols))
half = Counter(col for t in pieces for col in t if col)
p_i = sum((half[c] / sum(half[c] for c in inner_cols)) ** 2 for c in inner_cols)
p_f = sum((half[c] / sum(half[c] for c in frame_cols)) ** 2 for c in frame_cols)
print("collision prob interior %.4f frame %.4f" % (p_i, p_f))

print("\n== 1. mismatches ==")
for i in ids:
    g = boards[i]
    d = defects(g)
    rows = Counter()
    deg = Counter()
    kinds = Counter()
    for a, sa, b, sb in d:
        ca, cb = g[a[0]][a[1]][sa], g[b[0]][b[1]][sb]
        deg[ca] += 1
        deg[cb] += 1
        rows[(a[0], b[0])] += 1
        kinds["frame" if ca in frame_cols else "interior"] += 1
    cells = {a for a, _, b, _ in d} | {b for _, _, b, _ in d}
    odd = [c for c in deg if deg[c] % 2]
    rr = sorted({x for k in rows for x in k})
    print("%-26s score %d mismatches %d rows %s defect cells %d kinds %s odd-degree colours %d" % (
        i, 480 - len(d), len(d), "%d..%d" % (rr[0], rr[-1]), len(cells), dict(kinds), len(odd)))

print("\n== 2. pairwise Hamming distance (cells whose placed piece or rotation differs), best of 4 board turns ==")
def turn(g):
    # rotate board 90 degrees clockwise
    return [[tuple(g[N - 1 - c][r][(s - 1) % 4] for s in range(4)) for c in range(N)] for r in range(N)]
for x in range(len(ids)):
    for y in range(x + 1, len(ids)):
        best = None
        gy = boards[ids[y]]
        for k in range(4):
            dd = sum(boards[ids[x]][r][c] != gy[r][c] for r in range(N) for c in range(N))
            best = dd if best is None else min(best, dd)
            gy = turn(gy)
        print("%-26s %-26s %d" % (ids[x], ids[y], best))


def hungarian(cost):
    n = len(cost)
    INF = float("inf")
    u = [0] * (n + 1); v = [0] * (n + 1); p = [0] * (n + 1); way = [0] * (n + 1)
    for i in range(1, n + 1):
        p[0] = i; j0 = 0
        minv = [INF] * (n + 1); used = [False] * (n + 1)
        while True:
            used[j0] = True
            i0 = p[j0]; delta = INF; j1 = 0
            row = cost[i0 - 1]
            for j in range(1, n + 1):
                if not used[j]:
                    cur = row[j - 1] - u[i0] - v[j]
                    if cur < minv[j]:
                        minv[j] = cur; way[j] = j0
                    if minv[j] < delta:
                        delta = minv[j]; j1 = j
            for j in range(n + 1):
                if used[j]:
                    u[p[j]] += delta; v[j] -= delta
                else:
                    minv[j] -= delta
            j0 = j1
            if p[j0] == 0:
                break
        while True:
            j1 = way[j0]; p[j0] = p[j1]; j0 = j1
            if j0 == 0:
                break
    ans = [0] * n
    for j in range(1, n + 1):
        ans[p[j] - 1] = j - 1
    return ans


def best_fit(g, r, c, t):
    """Max matched internal edges of piece t (any legal rotation) at (r,c) against current neighbours; -1 if rim-illegal."""
    nb = [g[r - 1][c][2] if r > 0 else 0, g[r][c + 1][3] if c + 1 < N else 0,
          g[r + 1][c][0] if r + 1 < N else 0, g[r][c - 1][1] if c > 0 else 0]
    rim = [r == 0, c == N - 1, r == N - 1, c == 0]
    best = -1
    for k in range(4):
        q = t[k:] + t[:k]
        ok = all((q[s] == 0) == rim[s] for s in range(4))
        if not ok:
            continue
        m = sum(1 for s in range(4) if not rim[s] and q[s] == nb[s])
        best = max(best, m)
    return best


print("\n== 3. exact checkerboard-class assignment ==")
print("class = cells with (row+col)%2 == q refilled optimally from their own 128 pieces, other class fixed")
for i in ids:
    g = boards[i]
    base = 480 - len(defects(g))
    for q in (0, 1):
        cells = [(r, c) for r in range(N) for c in range(N) if (r + c) % 2 == q]
        ps = [g[r][c] for r, c in cells]
        val = [[best_fit(g, r, c, t) for t in ps] for r, c in cells]
        cost = [[(10 ** 7 if val[a][b] < 0 else -(1000 * val[a][b] + (1 if a == b else 0))) for b in range(len(ps))] for a in range(len(cells))]
        ans = hungarian(cost)
        opt = sum(val[a][ans[a]] for a in range(len(cells)))
        inc = sum(val[a][a] for a in range(len(cells)))
        moved = sum(1 for a in range(len(cells)) if ans[a] != a)
        # degeneracy: other pieces of the class that fit a cell at least as well as the incumbent
        full = sum(1 for a in range(len(cells)) for b in range(len(ps)) if b != a and val[a][b] >= val[a][a] and val[a][a] >= 0)
        perfect_alt = sum(1 for a, (r, c) in enumerate(cells) for b in range(len(ps)) if b != a and val[a][b] == 4 - [r == 0, c == N - 1, r == N - 1, c == 0].count(True))
        deficient = sum(1 for a, (r, c) in enumerate(cells) if val[a][a] < 4 - [r == 0, c == N - 1, r == N - 1, c == 0].count(True))
        print("%-26s q=%d incumbent %d (board %d) optimum %d gain %d cells moved %d | cells with a mismatch %d, (cell,other piece) pairs fitting as well as incumbent %d, perfect-fit alternatives %d" % (
            i, q, inc, base, opt, opt - inc, moved, deficient, full, perfect_alt))

print("\n== 4. first-moment null model for the committed halo SAT verdicts ==")
halo = json.load(open(os.path.join(ROOT, "research", "topics", "rigidity-sat-halo", "results", "halo-sat.json")))["results"]
committed = {(h["board"], h["radius"]): h for h in halo}
for i in ids:
    g = boards[i]
    seeds = set()
    for a, sa, b, sb in defects(g):
        seeds.add(a); seeds.add(b)
    for R in (1, 2, 3, 4):
        free = {(r, c) for r in range(N) for c in range(N) if any(max(abs(r - a), abs(c - b)) <= R for a, b in seeds)}
        hc = sum(1 for r, c in free if (r in (0, N - 1)) and (c in (0, N - 1)))
        he = sum(1 for r, c in free if ((r in (0, N - 1)) != (c in (0, N - 1))))
        hi = len(free) - hc - he
        Ei = Ef = 0
        for a, sa, b, sb in internal_edges():
            if a in free or b in free:
                on_rim_a = a[0] in (0, N - 1) or a[1] in (0, N - 1)
                on_rim_b = b[0] in (0, N - 1) or b[1] in (0, N - 1)
                along = on_rim_a and on_rim_b and ((a[0] == b[0] and a[0] in (0, N - 1)) or (a[1] == b[1] and a[1] in (0, N - 1)))
                if along:
                    Ef += 1
                else:
                    Ei += 1
        lg = lg_fact(hi) + hi * math.log10(4) + lg_fact(he) + lg_fact(hc) + Ei * math.log10(p_i) + Ef * math.log10(p_f)
        cm = committed.get((i, R))
        print("%-26s R=%d freed %3d (committed %s) interior %3d edge %2d corner %d | edges to satisfy: interior-colour %3d frame %2d | log10 E[perfect refills] = %7.1f | committed verdict %s in %s s" % (
            i, R, len(free), cm["freedCells"] if cm else "?", hi, he, hc, Ei, Ef, lg, cm["verdict"] if cm else "?", cm["seconds"] if cm else "?"))

print("\n== 5. parity ledger bound across straight cuts ==")
print("for a half-board region with its own pieces and the fixed seam demand: breaks >= ceil(#colours with odd half-edge total / 2)")
for i in ids:
    g = boards[i]
    best = (0, None)
    rows_out = []
    for axis in (0, 1):
        for cut in range(1, N):
            for side in (0, 1):
                def inreg(r, c):
                    x = r if axis == 0 else c
                    return (x >= cut) if side else (x < cut)
                tot = Counter()
                for r in range(N):
                    for c in range(N):
                        if inreg(r, c):
                            for col in g[r][c]:
                                if col:
                                    tot[col] += 1
                actual = 0
                for a, sa, b, sb in internal_edges():
                    ia, ib = inreg(*a), inreg(*b)
                    if ia != ib:
                        # seam demand comes from the fixed outside cell
                        col = g[b[0]][b[1]][sb] if ia else g[a[0]][a[1]][sa]
                        tot[col] += 1
                    if (ia or ib) and g[a[0]][a[1]][sa] != g[b[0]][b[1]][sb]:
                        actual += 1
                lb = (sum(1 for c in tot if tot[c] % 2) + 1) // 2
                rows_out.append((lb, actual))
                if lb > best[0]:
                    best = (lb, (axis, cut, side, actual))
    print("%-26s max parity bound over 60 half-board regions: %d (region %s); regions where bound > 0: %d of 60; regions where bound == actual breaks > 0: %d" % (
        i, best[0], best[1], sum(1 for lb, a in rows_out if lb > 0), sum(1 for lb, a in rows_out if lb == a and a > 0)))
