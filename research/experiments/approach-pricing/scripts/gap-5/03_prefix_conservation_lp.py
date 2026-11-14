"""Gap 5, script 3. Do the region-by-region conservation laws ever refute a row-scan prefix
of the REAL tiles before cell-by-cell matching does?

Test: after k full rows, the unplaced tiles must admit an assignment to coarse classes
(row, column parity, rim pattern, rotation) such that, per colour: east faces of even
columns = west faces of odd columns and conversely in every row, south faces of row r =
north faces of row r+1 per column parity, and the north faces of row k reproduce the frontier
per column parity. This aggregates edge matching over the classes; it contains the flux law,
the checkerboard law and every per-row census. LP relaxation (HiGHS), then the integer
program with a time limit when the LP is feasible.

Prefixes: randomised row-scan depth-first search on the official pieces, NO clue pinned
(the clue-free tree), first arrival at each row boundary, several restarts.
Control (painted 16x16 boards with the official colour counts, planted solution known; these
are generated boards, not Eternity II): the true remainder after k rows with TWO unplaced
interior tiles exchanged for two placed ones carrying the same eight colours, so that the
colour census is untouched. (A first version exchanged ONE tile; that breaks the parity of
two colour counts, which every valid prefix satisfies by construction, so it tested nothing:
70 of 72 refuted, kept on record in the memo.)

Run: uv run --with numpy --with scipy python 03_prefix_conservation_lp.py real [restarts] [node_cap] [ip_secs] [ip_from]
     uv run --with numpy --with scipy python 03_prefix_conservation_lp.py control [ip_secs] [boards]
"""
import sys, random, time
from collections import Counter, defaultdict
import numpy as np
from scipy.optimize import linprog, milp, LinearConstraint, Bounds
from scipy.sparse import coo_matrix
from e2common import load, rots, paint, official_pools, N

NC = 23


def rim_pattern(r, c):
    """(top grey, right grey, bottom grey, left grey) required at cell (r,c)."""
    return (r == 0, c == N - 1, r == N - 1, c == 0)


def conservation_feasible(remaining, k, frontier, integer_seconds=20.0):
    """remaining: list of tiles (4-tuples); k: rows already full; frontier[c] = colour that
    the tile at (k,c) must show on top. Returns (lp_feasible, ip_status)."""
    rows = list(range(k, N))
    # classes: (r, q, pattern) with their cell counts
    cells = Counter()
    for r in rows:
        for c in range(N):
            cells[(r, c % 2, rim_pattern(r, c))] += 1
    var = []            # (tile index, rotation tuple, class)
    for i, t in enumerate(remaining):
        for rt in set(rots(t)):
            g = tuple(x == 0 for x in rt)
            for cl in cells:
                if cl[2] == g:
                    var.append((i, rt, cl))
    nv = len(var)
    R, C, V, rhs = [], [], [], []
    eq = {}
    def row_of(key, b=0):
        if key not in eq:
            eq[key] = len(rhs); rhs.append(b)
        return eq[key]
    for i in range(len(remaining)):
        row_of(("tile", i), 1)
    for cl, n in cells.items():
        row_of(("cells", cl), n)
    fr = Counter((c % 2, frontier[c]) for c in range(N))
    for (q, col), n in fr.items():
        row_of(("NS", k - 1, q, col), -n)      # south of row k-1 is the frontier: S - N = 0
    for j, (i, (top, right, bot, left), (r, q, _)) in enumerate(var):
        R.append(eq[("tile", i)]); C.append(j); V.append(1)
        R.append(eq[("cells", (r, q, _))]); C.append(j); V.append(1)
        if right:
            R.append(row_of(("EW", r, q, right))); C.append(j); V.append(1)
        if left:
            R.append(row_of(("EW", r, 1 - q, left))); C.append(j); V.append(-1)
        if bot:
            R.append(row_of(("NS", r, q, bot))); C.append(j); V.append(1)
        if top:
            R.append(row_of(("NS", r - 1, q, top))); C.append(j); V.append(-1)
    A = coo_matrix((V, (R, C)), shape=(len(rhs), nv)).tocsr()
    b = np.array(rhs, dtype=float)
    res = linprog(np.zeros(nv), A_eq=A, b_eq=b, bounds=(0, 1), method="highs")
    if res.status == 2:
        return False, "lp-infeasible"
    if integer_seconds <= 0:
        return True, "not-run"
    m = milp(np.zeros(nv), constraints=LinearConstraint(A, b, b), integrality=np.ones(nv),
             bounds=Bounds(0, 1), options={"time_limit": integer_seconds})
    st = {0: "ip-feasible", 1: "ip-timeout", 2: "ip-infeasible"}.get(m.status, f"ip-status-{m.status}")
    return True, st


def dfs_prefixes(P, rng, targets, node_cap):
    """Randomised row-scan DFS on the official pieces, no clue. Returns {depth: board list}
    at first arrival, and the node count."""
    by = defaultdict(list)     # (left, top) -> [(tile index, rotation)]
    for i, t in enumerate(P):
        for rt in set(rots(t)):
            by[(rt[3], rt[0])].append((i, rt))
    used = [False] * len(P)
    board = [None] * (N * N)
    found = {}
    nodes = 0
    stack = []                 # per depth: candidate list iterator
    def cands(d):
        r, c = divmod(d, N)
        left = board[d - 1][1][1] if c else 0
        top = board[d - N][1][2] if r else 0
        out = []
        for (i, rt) in by.get((left, top), ()):
            if used[i]:
                continue
            if (rt[1] == 0) != (c == N - 1) or (rt[2] == 0) != (r == N - 1):
                continue
            out.append((i, rt))
        rng.shuffle(out)
        return out
    stack.append(cands(0))
    d = 0
    while stack and nodes < node_cap:
        if stack[-1]:
            i, rt = stack[-1].pop()
            board[d] = (i, rt); used[i] = True; nodes += 1
            d += 1
            if d in targets and d not in found:
                found[d] = list(board[:d])
                if len(found) == len(targets):
                    break
            if d == N * N:
                break
            stack.append(cands(d))
        else:
            stack.pop()
            d -= 1
            if d >= 0 and board[d] is not None and stack:
                used[board[d][0]] = False; board[d] = None
    return found, nodes


def check_prefix(P, pref, secs):
    k = len(pref) // N
    placed = {i for i, _ in pref}
    remaining = [t for i, t in enumerate(P) if i not in placed]
    frontier = [pref[(k - 1) * N + c][1][2] for c in range(N)]
    return conservation_feasible(remaining, k, frontier, secs)


if __name__ == "__main__":
    mode = sys.argv[1] if len(sys.argv) > 1 else "real"
    P, _ = load()
    t0 = time.time()
    if mode == "real":
        restarts = int(sys.argv[2]) if len(sys.argv) > 2 else 4
        node_cap = int(sys.argv[3]) if len(sys.argv) > 3 else 200000
        ip_secs = float(sys.argv[4]) if len(sys.argv) > 4 else 45.0
        ip_from = int(sys.argv[5]) if len(sys.argv) > 5 else 160
        targets = set(range(16, 256, 16))
        tally = defaultdict(Counter)
        for s in range(restarts):
            rng = random.Random(1000 + s)
            found, nodes = dfs_prefixes(P, rng, targets, node_cap)
            for d in sorted(found):
                lp, st = check_prefix(P, found[d], ip_secs if d >= ip_from else 0)
                tally[d][("lp-feasible/" + st) if lp else st] += 1
            print(f"restart {s}: nodes {nodes}, deepest row boundary {max(found)}, "
                  f"elapsed {time.time() - t0:.0f}s", flush=True)
        print("== official pieces, no clue: prefixes at row boundaries ==")
        print(f"(integer program only from depth {ip_from}, {ip_secs:.0f} s limit)")
        print("depth  prefixes  outcome counts")
        for d in sorted(tally):
            print(f"{d:5d}  {sum(tally[d].values()):8d}  {dict(tally[d])}")
    else:
        # Control on painted boards (generated, not Eternity II). The true remainder after k
        # rows, with two unplaced interior tiles exchanged for two placed interior tiles that
        # carry the same eight colours in total. The colour census is unchanged, so only the
        # laws beyond the census can refute.
        ip_secs = float(sys.argv[2]) if len(sys.argv) > 2 else 20.0
        boards = int(sys.argv[3]) if len(sys.argv) > 3 else 10
        rng = random.Random(77)
        ring_pool, inner_pool = official_pools(P)
        ctl = defaultdict(Counter)
        for b in range(boards):
            board = paint(rng, ring_pool, inner_pool)
            for k in (8, 10, 12, 13, 14):
                placed = [board[r][c] for r in range(1, k - 1) for c in range(1, N - 1)]
                rest = [board[r][c] for r in range(k, N - 1) for c in range(1, N - 1)]
                key = defaultdict(list)
                for i in range(len(placed)):
                    for j in range(i):
                        key[tuple(sorted(placed[i] + placed[j]))].append((i, j))
                pairs = []
                for i in range(len(rest)):
                    for j in range(i):
                        kk = tuple(sorted(rest[i] + rest[j]))
                        if kk in key and {tuple(sorted(rest[i])), tuple(sorted(rest[j]))} != \
                                {tuple(sorted(placed[key[kk][0][0]])), tuple(sorted(placed[key[kk][0][1]]))}:
                            pairs.append((i, j, key[kk][0]))
                if not pairs:
                    ctl[k]["no census-preserving exchange"] += 1
                    continue
                i, j, (pi, pj) = rng.choice(pairs)
                remaining = [board[r][c] for r in range(k, N) for c in range(N)]
                remaining.remove(rest[i]); remaining.remove(rest[j])
                remaining += [placed[pi], placed[pj]]
                frontier = [board[k - 1][c][2] for c in range(N)]
                lp, st = conservation_feasible(remaining, k, frontier, ip_secs)
                ctl[k][("lp-feasible/" + st) if lp else st] += 1
            print(f"board {b} done, {time.time() - t0:.0f}s", flush=True)
        print("== control: painted boards, census-preserving two-for-two exchange ==")
        print("rows placed  rows left  outcome counts over", boards, "boards")
        for k in sorted(ctl):
            print(f"{k:11d}  {N - k:9d}  {dict(ctl[k])}")
    print(f"total elapsed {time.time() - t0:.0f}s")
