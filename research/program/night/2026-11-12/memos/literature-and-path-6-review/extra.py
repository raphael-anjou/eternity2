"""Referee checks on solve-path-6 (2026-11-12). Imports the memo's own dm.py.

Run: uv run --quiet --with numpy --with scipy python -I extra.py <part>
Parts:
  crit     distance of the memo's instances from the critical colour count
  base     what the naive DFS reference measures, and two fairer references
  near     the map on the near-critical 5x5 c=4 instances the memo skipped
  fixed    independent check of terminal layouts, and what a stall looks like
"""
import sys, os, math, random, statistics, time
import importlib.util

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("dm", os.path.join(HERE, "..", "solve-path-6", "dm.py"))
dm = importlib.util.module_from_spec(spec)
spec.loader.exec_module(dm)
import numpy as np

TABLE = [(5, 3, 5, 0), (5, 3, 5, 1), (5, 3, 5, 2), (6, 4, 8, 0), (6, 4, 8, 1), (6, 4, 7, 0), (7, 4, 11, 1)]
ARK_MED = {(5, 3, 5, 0): 351, (5, 3, 5, 1): 2287, (5, 3, 5, 2): 465, (6, 4, 8, 0): 970,
           (6, 4, 8, 1): 299, (6, 4, 7, 0): 2613, (7, 4, 11, 1): 2515}
A_MED = {(5, 3, 5, 0): 195, (5, 3, 5, 1): 5657, (5, 3, 5, 2): 937, (6, 4, 8, 0): 1246,
         (6, 4, 8, 1): 322, (6, 4, 7, 0): 5506, (7, 4, 11, 1): 17136}


def inst(n, b, c, seed):
    return dm.gen(n, b, c, random.Random(1000 * n + 10 * c + seed))


def lnS(n, b, c):
    """First moment for independent uniform colours (the memo's generator draws
    each seam with randint), all four turns of the board counted."""
    w = n - 2
    P = w * w
    lf = math.lgamma
    return (math.log(24) + lf(4 * w + 1) + lf(P + 1) + P * math.log(4)
            - 4 * (n - 1) * math.log(b) - 2 * w * (w + 1) * math.log(c))


def crit(n, b):
    w = n - 2
    return math.exp(lnS(n, b, 1.0) / (2 * w * (w + 1)))


def part_crit():
    print("n  ring b  other c  critical c (S=1)  offset  ln S   | same board with 5 ring colours: critical c")
    for n, b, cs in ((5, 3, (4, 5)), (6, 4, (6, 7, 8)), (7, 4, (11,)), (8, 5, (12,))):
        for c in cs:
            print(f"{n}  {b}       {c:<7d}  {crit(n, b):6.2f}            {c - crit(n, b):+5.2f}  {lnS(n, b, c):6.1f} | {crit(n, 5):5.2f}")
    print("reviewed balanced-ensemble values, 5 frame colours (growth-law.REVIEW.md line 78): n=6 4.65, n=7 5.99, n=8 7.32, n=9 8.63, n=10 9.94")


def dfs_profile(n, pieces, cap, first=False):
    rots = [[p[k:] + p[:k] for k in range(4)] for p in pieces]
    P = len(pieces)
    used = [False] * P
    board = [None] * (n * n)
    depth = [0] * (n * n)
    st = {"nodes": 0, "sols": 0, "stop": False}

    def rec(pos):
        if st["stop"]:
            return
        if pos == n * n:
            st["sols"] += 1
            if first:
                st["stop"] = True
            return
        r, j = divmod(pos, n)
        rim = dm.rim_sides(n, r, j)
        left = board[pos - 1][1] if j > 0 else None
        up = board[pos - n][2] if r > 0 else None
        for p in range(P):
            if used[p]:
                continue
            seen = set()
            for t in rots[p]:
                if t in seen:
                    continue
                seen.add(t)
                if any((t[s] == 0) != rim[s] for s in range(4)):
                    continue
                if left is not None and t[3] != left:
                    continue
                if up is not None and t[0] != up:
                    continue
                st["nodes"] += 1
                depth[pos] += 1
                if st["nodes"] > cap:
                    st["stop"] = True
                    return
                used[p] = True
                board[pos] = t
                rec(pos + 1)
                used[p] = False
                if st["stop"]:
                    return
    rec(0)
    return st["nodes"], st["sols"], depth


def mrv_first(n, pieces, rng, cap):
    """Smallest-domain cell first, dead end as soon as an open cell next to a
    placed one has no candidate. Nodes = placements. Stops at the first solution."""
    P = len(pieces)
    order = list(range(P))
    rng.shuffle(order)
    rots = [sorted(set(pieces[p][k:] + pieces[p][:k] for k in range(4))) for p in range(P)]
    board = {}
    used = [False] * P
    st = {"nodes": 0, "done": False}
    nb = ((-1, 0, 0, 2), (0, 1, 1, 3), (1, 0, 2, 0), (0, -1, 3, 1))  # dr, dj, my side, their side

    def cands(r, j):
        rim = dm.rim_sides(n, r, j)
        need = [None] * 4
        for dr, dj, s, o in nb:
            q = board.get((r + dr, j + dj))
            if q is not None:
                need[s] = q[o]
        out = []
        for p in order:
            if used[p]:
                continue
            for t in rots[p]:
                ok = True
                for s in range(4):
                    if (t[s] == 0) != rim[s] or (need[s] is not None and t[s] != need[s]):
                        ok = False
                        break
                if ok:
                    out.append((p, t))
        return out

    def rec():
        if st["done"] or st["nodes"] > cap:
            return
        if len(board) == n * n:
            st["done"] = True
            return
        best = None
        front = [(r, j) for r in range(n) for j in range(n) if (r, j) not in board
                 and any((r + dr, j + dj) in board for dr, dj, _, _ in nb)]
        if not front:
            front = [(0, 0)]
        for cell in front:
            cs = cands(*cell)
            if not cs:
                return
            if best is None or len(cs) < len(best[1]):
                best = (cell, cs)
        cell, cs = best
        for p, t in cs:
            st["nodes"] += 1
            used[p] = True
            board[cell] = t
            rec()
            if st["done"] or st["nodes"] > cap:
                return
            used[p] = False
            del board[cell]
    rec()
    return st["nodes"] if st["done"] else None


def part_base():
    print("Naive row scan, whole tree: where the nodes are. Then nodes to a FIRST solution")
    print("(median over shuffled piece orders) for the naive scan and for a smallest-domain-first DFS.")
    print("n b c seed | tree nodes sols nodes/sol | share of nodes in row 0, rows 0-1 | naive first-sol med (9) | mrv first-sol med (5) | ARK med its, ARK/naive-per-sol, ARK/mrv, A/mrv")
    for key in TABLE:
        n, b, c, seed = key
        pieces = inst(*key)
        t0 = time.process_time()
        nodes, sols, depth = dfs_profile(n, pieces, 10 ** 7)
        r0 = sum(depth[:n]) / nodes
        r1 = sum(depth[:2 * n]) / nodes
        firsts = []
        for s in range(9):
            pp = pieces[:]
            random.Random(s).shuffle(pp)
            firsts.append(dfs_profile(n, pp, 10 ** 7, first=True)[0])
        mrv = []
        for s in range(5):
            mrv.append(mrv_first(n, pieces, random.Random(s), 200000))
        mm = statistics.median(x if x is not None else 10 ** 9 for x in mrv)
        print(f"{n} {b} {c} {seed} | {nodes} {sols} {nodes / sols:.0f} | {r0:.2f} {r1:.2f} | {statistics.median(firsts):.0f} "
              f"(min {min(firsts)}, max {max(firsts)}) | {mm:.0f} {mrv} | {ARK_MED[key]} {ARK_MED[key] / (nodes / sols):.3f} "
              f"{ARK_MED[key] / mm:.2f} {A_MED[key] / mm:.2f} | t={time.process_time() - t0:.0f}s", flush=True)


def part_near():
    print("5x5, 3 ring colours, 4 other colours (offset +0.4 from critical): DFS reference existed in results.txt, the map was not run.")
    for seed in (0, 1):
        pieces = inst(5, 3, 4, seed)
        nodes, sols, _ = dfs_profile(5, pieces, 10 ** 7)
        for sets in ("A", "ARK"):
            its = []
            t0 = time.process_time()
            for run in range(5):
                m = dm.DM(5, pieces, 8, sets=sets, form="a", seed=run)
                it, best = m.run(20000, 0.25)
                its.append(it if it is not None else 10 ** 9)
            med = statistics.median(its)
            shown = [x if x < 10 ** 9 else "cap" for x in its]
            print(f"n=5 b=3 c=4 seed={seed} sols={sols} nodes/sol={nodes / sols:.0f} sets={sets} its={shown} "
                  f"median={'cap' if med >= 10 ** 9 else med} ratio={'-' if med >= 10 ** 9 else round(med / (nodes / sols), 2)} "
                  f"t={time.process_time() - t0:.1f}s", flush=True)


def verify(n, pieces, m, lay):
    """Independent of DM.mismatches: rebuild the board from piece tuples."""
    seen = set()
    grid = [[None] * n for _ in range(n)]
    for r in range(n):
        for j in range(n):
            s = int(lay[r, j])
            p, k = divmod(s, 4)
            if p in seen:
                return "piece used twice"
            seen.add(p)
            grid[r][j] = pieces[p][k:] + pieces[p][:k]
    for r in range(n):
        for j in range(n):
            t = grid[r][j]
            rim = dm.rim_sides(n, r, j)
            if any((t[s] == 0) != rim[s] for s in range(4)):
                return "rim wrong"
            if j + 1 < n and t[1] != grid[r][j + 1][3]:
                return "seam wrong"
            if r + 1 < n and t[2] != grid[r + 1][j][0]:
                return "seam wrong"
    return "ok"


def trace(m, max_iter, beta, keep):
    n, C, K = m.n, m.C, len(m.sets)
    X = m.rng.random((K, n, n, 4, C))
    lays, norms, gaps = [], [], []
    for it in range(1, max_iter + 1):
        pc = m.concur(X)
        pd, lay = m.divide(2 * pc - X)
        X = X + beta * (pd - pc)
        if it > max_iter - keep:
            lays.append(lay.tobytes())
            norms.append(float(np.abs(X).max()))
            gaps.append(float(np.linalg.norm(pd - pc)))
        if m.mismatches(lay) == 0:
            return it, lay, lays, norms, [float(np.linalg.norm(pd - pc))]
    return None, lay, lays, norms, gaps


def part_fixed():
    print("1. Terminal layouts checked with an independent verifier (pieces used once, rim, every seam).")
    for key, sets in (((5, 3, 5, 0), "A"), ((5, 3, 5, 2), "ARK"), ((6, 4, 8, 1), "A"), ((6, 4, 8, 1), "ARK")):
        n, b, c, seed = key
        pieces = inst(*key)
        for run in range(3):
            m = dm.DM(n, pieces, b + c + 1, sets=sets, form="a", seed=run)
            it, lay, _, _, gaps = trace(m, 20000, 0.25, 1)
            print(f"   {key} {sets} run {run}: stop at {it}, verifier: {verify(n, pieces, m, lay) if it else 'no stop'}, "
                  f"|PD - PC| at the stop = {gaps[-1]:.3f} (0 would be a fixed point of the map)")
    print("2. A stall: 5x5 c=5 seed 1, layouts only, beta = 1, 20,000 iterations, last 4,000 kept.")
    pieces = inst(5, 3, 5, 1)
    for run in range(3):
        m = dm.DM(5, pieces, 9, sets="A", form="a", seed=run)
        it, lay, lays, norms, gaps = trace(m, 20000, 1.0, 4000)
        distinct = len(set(lays))
        period = next((p for p in range(1, 2000) if all(lays[i] == lays[i - p] for i in range(len(lays) - 2000, len(lays)))), None)
        print(f"   run {run}: stop={it}, distinct layouts in the last 4,000 iterations: {distinct}, exact period: {period}, "
              f"mismatches of the last layout: {m.mismatches(lay)}, max|x| at 16,001 and 20,000: {norms[0]:.1f}, {norms[-1]:.1f}, "
              f"|PD - PC| last: {gaps[-1]:.3f}")


if __name__ == "__main__":
    {"crit": part_crit, "base": part_base, "near": part_near, "fixed": part_fixed}[sys.argv[1]]()
