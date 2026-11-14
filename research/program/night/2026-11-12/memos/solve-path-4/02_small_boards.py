"""Score versus distance to the nearest solution, on small planted boards, for boards a simple
annealer stops on.

Puzzle: n x n, 5 frame colours on along-frame joints, ci interior colours on every other joint,
painted uniformly at random from a solved board (the palettes of
research/experiments/near-codeword-spectrum: 5x5 C=13, 6x6 C=14, 7x7 C=15). Instances with two
equal pieces (up to rotation) or a rotation-symmetric piece are rejected, as in the official set.
Every solution is enumerated by exhaustive DFS (all four board turns included), so "nearest
solution" is exact. The annealer swaps two same-class cells (random rotations inside) or rotates
one interior piece; each run is stopped at a fixed move budget and its final board recorded.
Distance = number of cells whose (piece, rotation) differs from the nearest solution.

Stdlib only: python3 -I 02_small_boards.py [seed] [deep]
"deep" reruns 6x6 and 7x7 with longer schedules (60,000 and 120,000 moves per run).
"""
import math, os, random, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib4 import lg_fact, lg_add, canon, internal_edges


def rots(t):
    return [t[4 - k:] + t[:4 - k] if k else t for k in range(4)]


def make(n, cf, ci, rng):
    while True:
        H = [[0] * (n - 1) for _ in range(n)]  # joint between (r,c) and (r,c+1)
        V = [[0] * n for _ in range(n - 1)]    # joint between (r,c) and (r+1,c)
        for r in range(n):
            for c in range(n - 1):
                H[r][c] = rng.randint(1, cf) if r in (0, n - 1) else cf + rng.randint(1, ci)
        for r in range(n - 1):
            for c in range(n):
                V[r][c] = rng.randint(1, cf) if c in (0, n - 1) else cf + rng.randint(1, ci)
        sol = []
        for r in range(n):
            for c in range(n):
                sol.append((V[r - 1][c] if r else 0, H[r][c] if c < n - 1 else 0, V[r][c] if r < n - 1 else 0, H[r][c - 1] if c else 0))
        cs = [canon(t) for t in sol]
        if len(set(cs)) < n * n:
            continue
        if any(len(set(rots(t))) < 4 for t in sol):
            continue
        return sol


def rim_sides(n, r, c):
    return (r == 0, c == n - 1, r == n - 1, c == 0)


def legal(n, t, r, c):
    rs = rim_sides(n, r, c)
    return [q for q in set(rots(t)) if all((q[s] == 0) == rs[s] for s in range(4))]


def enumerate_solutions(n, pieces, cap_nodes):
    cells = [(r, c) for r in range(n) for c in range(n)]
    cand = {}
    for r, c in cells:
        idx = {}
        for i, t in enumerate(pieces):
            for q in legal(n, t, r, c):
                idx.setdefault((q[0], q[3]), []).append((i, q))
        cand[(r, c)] = idx
    used = [False] * len(pieces)
    board = [None] * (n * n)
    out = []
    nodes = [0]

    def rec(k):
        if k == n * n:
            out.append(tuple(board))
            return True
        r, c = cells[k]
        up = board[k - n][2] if r else 0
        left = board[k - 1][1] if c else 0
        for i, q in cand[(r, c)].get((up, left), ()):
            if used[i]:
                continue
            nodes[0] += 1
            if nodes[0] > cap_nodes:
                return False
            used[i] = True
            board[k] = q
            ok = rec(k + 1)
            used[i] = False
            if not ok:
                return False
        return True

    done = rec(0)
    return out if done else None, nodes[0]


def anneal(n, pieces, rng, moves, t0, t1):
    N = n * n
    cells = [(r, c) for r in range(n) for c in range(n)]
    cls = {0: [], 1: [], 2: []}
    for k, (r, c) in enumerate(cells):
        cls[(r in (0, n - 1)) + (c in (0, n - 1))].append(k)
    pcs = {0: [], 1: [], 2: []}
    for t in pieces:
        pcs[t.count(0)].append(t)
    board = [None] * N
    for g in (0, 1, 2):
        ps = pcs[g][:]
        rng.shuffle(ps)
        for k, t in zip(cls[g], ps):
            board[k] = rng.choice(legal(n, t, *cells[k]))
    nb = []
    for k, (r, c) in enumerate(cells):
        l = []
        if r: l.append((0, k - n, 2))
        if c < n - 1: l.append((1, k + 1, 3))
        if r < n - 1: l.append((2, k + n, 0))
        if c: l.append((3, k - 1, 1))
        nb.append(l)

    def local(ks):
        e = 0
        seen = set()
        for k in ks:
            for s, j, sj in nb[k]:
                key = (min(k, j), max(k, j))
                if key in seen:
                    continue
                seen.add(key)
                if board[k][s] != board[j][sj]:
                    e += 1
        return e

    E = local(range(N))
    weights = [len(cls[0]), len(cls[1]), len(cls[2])]
    groups = [0, 1, 2]
    ratio = (t1 / t0) ** (1.0 / moves)
    T = t0
    for _ in range(moves):
        g = rng.choices(groups, weights)[0]
        a = rng.choice(cls[g])
        b = rng.choice(cls[g])
        old_a, old_b = board[a], board[b]
        if a == b:
            if g != 0:
                T *= ratio
                continue
            before = local((a,))
            board[a] = rng.choice(rots(old_a))
            after = local((a,))
        else:
            before = local((a, b))
            board[a] = rng.choice(legal(n, old_b, *cells[a]))
            board[b] = rng.choice(legal(n, old_a, *cells[b]))
            after = local((a, b))
        d = after - before
        if d <= 0 or rng.random() < math.exp(-d / T):
            E += d
        else:
            board[a], board[b] = old_a, old_b
        T *= ratio
    return E, board


def main():
    seed = int(sys.argv[1]) if len(sys.argv) > 1 else 20261112
    rng = random.Random(seed)
    t_start = time.time()
    plan = [(5, 5, 8, 20, 60, 4000), (6, 5, 9, 15, 60, 8000), (7, 5, 10, 8, 40, 16000)]
    hot, cold = 1.0, 0.08
    if len(sys.argv) > 2 and sys.argv[2] == "deep":
        plan = [(6, 5, 9, 10, 30, 60000), (7, 5, 10, 5, 20, 120000)]
        hot, cold = 0.7, 0.12
    for n, cf, ci, n_inst, runs, moves in plan:
        N = n * n
        rows = {}
        nsols = []
        skipped = 0
        t0 = time.time()
        made = 0
        while made < n_inst:
            sol = make(n, cf, ci, rng)
            pieces = sol[:]
            sols, nodes = enumerate_solutions(n, pieces, 3_000_000)
            if sols is None:
                skipped += 1
                if skipped > 6:
                    break
                continue
            made += 1
            nsols.append(len(sols))
            for _ in range(runs):
                E, b = anneal(n, pieces, rng, moves, hot, cold)
                d = min(sum(1 for x, y in zip(b, s) if x != y) for s in sols)
                rows.setdefault(E, []).append(d)
        print("== %dx%d, 5 frame + %d interior colours: %d instances (%d skipped at the enumeration cap), %d runs each of %d moves, %.0f s ==" % (
            n, n, ci, made, skipped, runs, moves, time.time() - t0))
        if not nsols:
            continue
        print("   solutions per instance (4 board turns counted): min %d median %d max %d" % (min(nsols), sorted(nsols)[len(nsols) // 2], max(nsols)))
        near_hi, mid_hi = math.ceil(N / 4) - 1, (3 * N) // 4
        print("   bands: near 1..%d, mid %d..%d, far %d..%d cells" % (near_hi, near_hi + 1, mid_hi, mid_hi + 1, N))
        print("   mismatches  boards  median distance  min  max   near   mid   far")
        for E in sorted(rows):
            ds = sorted(rows[E])
            m = len(ds)
            print("   %10d  %6d  %15d  %3d  %3d  %5.2f %5.2f %5.2f" % (
                E, m, ds[m // 2], ds[0], ds[-1],
                sum(1 for d in ds if 1 <= d <= near_hi) / m, sum(1 for d in ds if near_hi < d <= mid_hi) / m, sum(1 for d in ds if d > mid_hi) / m))
        allp = [(E, d) for E in rows for d in rows[E] if E > 0]
        if len(allp) > 2:
            me = sum(e for e, _ in allp) / len(allp); md = sum(d for _, d in allp) / len(allp)
            cov = sum((e - me) * (d - md) for e, d in allp)
            ve = sum((e - me) ** 2 for e, _ in allp); vd = sum((d - md) ** 2 for _, d in allp)
            print("   Pearson correlation between mismatches and distance over the %d unsolved boards: %.3f" % (len(allp), cov / math.sqrt(ve * vd) if ve and vd else float("nan")))
        sys.stdout.flush()
    print("total %.0f s" % (time.time() - t_start))


main()
