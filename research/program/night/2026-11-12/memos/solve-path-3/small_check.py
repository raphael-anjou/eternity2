"""Exact enumeration on small planted framed boards, against the first-moment model.

Run: python3 -I small_check.py N FRAME_COLOURS INTERIOR_COLOURS SEEDS [node_cap]
Own generator (planted solution, frame colours confined to the ring), own DFS.
Counts, per depth, every valid partial fill in row-major order (labelled pieces,
all rotations, no clue, so every global rotation of a solution is counted).
"""
import sys
import random
from collections import Counter, defaultdict
from math import comb, log10

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from model import interior_match_logp, frame_match_logp, profile, cell_kind, edge_kind, logsum10


def gen(n, cf, ci, seed):
    rnd = random.Random(seed)
    H = [[0] * (n + 1) for _ in range(n)]  # H[r][c]: colour of the vertical edge left of cell (r,c)
    V = [[0] * n for _ in range(n + 1)]    # V[r][c]: colour of the horizontal edge above cell (r,c)
    for r in range(n):
        for c in range(1, n):
            k = edge_kind(n, r, c - 1, r, c)
            H[r][c] = rnd.randint(1, cf) if k == "frame" else cf + rnd.randint(1, ci)
    for r in range(1, n):
        for c in range(n):
            k = edge_kind(n, r - 1, c, r, c)
            V[r][c] = rnd.randint(1, cf) if k == "frame" else cf + rnd.randint(1, ci)
    sol = [(V[r][c], H[r][c + 1], V[r + 1][c], H[r][c]) for r in range(n) for c in range(n)]
    perm = list(range(n * n))
    rnd.shuffle(perm)
    pieces = [None] * (n * n)
    where = {}
    for cell, pid in enumerate(perm):
        k = rnd.randint(0, 3)
        t = sol[cell]
        pieces[pid] = t[k:] + t[:k]
        where[pid] = cell
    return pieces, where


def colour_stats(n, pieces, cf):
    ic, fc = Counter(), Counter()
    for p in pieces:
        for v in p:
            if v == 0:
                continue
            if v <= cf:
                fc[v] += 1
            else:
                ic[v] += 1
    return list(ic.values()), [v // 2 for v in fc.values()]


class Solver:
    def __init__(self, n, pieces, order, allowed=None, preset_frame=None):
        """preset_frame=(where, ids): those pieces sit at their planted cells and
        constrain the search as fixed neighbours; they are not part of `order`."""
        self.n = n
        self.preset = {}
        if preset_frame:
            where, ids = preset_frame
            self.preset = {where[pid]: pid for pid in ids}
        self.order = order
        self.pieces = pieces
        pos = {cell: i for i, cell in enumerate(order)}
        self.plan = []
        orient = []
        for pid, p in enumerate(pieces):
            if allowed is not None and pid not in allowed:
                continue
            for k in range(4):
                orient.append((pid, p[k:] + p[:k]))
        tables = {}
        for i, (r, c) in enumerate(order):
            rim = tuple(int(x) for x in (r == 0, c == n - 1, r == n - 1, c == 0))
            nb = ((r - 1, c), (r, c + 1), (r + 1, c), (r, c - 1))
            known = tuple(int((q in pos and pos[q] < i) or (q[0] * n + q[1]) in self.preset
                              if 0 <= q[0] < n and 0 <= q[1] < n else False) for q in nb)
            key = (rim, known)
            if key not in tables:
                t = defaultdict(list)
                for pid, o in orient:
                    if all((o[s] == 0) == bool(rim[s]) for s in range(4)):
                        t[tuple(o[s] for s in range(4) if known[s])].append((1 << pid, o))
                tables[key] = t
            self.plan.append((r * n + c, [(s, (nb[s][0] * n + nb[s][1])) for s in range(4) if known[s]],
                              tables[key]))

    def run(self, stop, on_leaf=None, cap=None):
        """Enumerate all valid fills of the first `stop` cells. Returns counts per depth."""
        counts = [0] * (stop + 1)
        board = [None] * (self.n * self.n)
        n = self.n
        for cell, pid in self.preset.items():
            r, c = divmod(cell, n)
            rim = (r == 0, c == n - 1, r == n - 1, c == 0)
            p = self.pieces[pid]
            for k in range(4):
                o = p[k:] + p[:k]
                if all((o[s] == 0) == rim[s] for s in range(4)):
                    board[cell] = o
        plan = self.plan
        total = 0

        def rec(d, used):
            nonlocal total
            cell, known, table = plan[d]
            key = tuple(board[q][(s + 2) % 4] for s, q in known)
            for bit, o in table.get(key, ()):
                if used & bit:
                    continue
                counts[d + 1] += 1
                total += 1
                if cap and total > cap:
                    raise OverflowError
                board[cell] = o
                if d + 1 == stop:
                    if on_leaf:
                        on_leaf(used | bit, board)
                else:
                    rec(d + 1, used | bit)
            board[cell] = None

        if stop == 0:
            return counts
        rec(0, 0)
        return counts


def main():
    n, cf, ci = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3])
    seeds = range(1, int(sys.argv[4]) + 1)
    cap = int(float(sys.argv[5])) if len(sys.argv) > 5 else 4_000_000
    h = n // 2
    td = [(r, c) for r in range(n) for c in range(n)]
    bu = [(r, c) for r in reversed(range(n)) for c in range(n)]
    kinds = Counter(cell_kind(n, r, c) for r, c in td)
    pools = dict(kinds)
    agg = defaultdict(list)
    print("board %dx%d, %d frame colours, %d interior colours; pieces %s; 4*n_int/q^2 = %.2f"
          % (n, n, cf, ci, dict(kinds), 4 * kinds["interior"] / ci ** 2))
    for seed in seeds:
        pieces, where = gen(n, cf, ci, seed)
        icnt, fcnt = colour_stats(n, pieces, cf)
        lpi = interior_match_logp(icnt, sum(icnt) // 2)
        lpf = frame_match_logp(fcnt, sum(fcnt))
        mtd = [x[0] for x in profile(n, td, pools, lpi, lpf)]
        mbu = [x[0] for x in profile(n, bu, pools, lpi, lpf)]
        if logsum10(mtd) > log10(cap):
            print("seed %d: model predicts 10^%.2f nodes, above cap, skipped" % (seed, logsum10(mtd)))
            continue
        full = Solver(n, pieces, td)
        try:
            counts = full.run(n * n, cap=cap * 20)
        except OverflowError:
            print("seed %d: node cap hit, skipped" % seed)
            continue
        agg["full_counts"].append(counts[1:])
        agg["full_model"].append(mtd)
        # half lists
        tops, bots = defaultdict(Counter), defaultdict(Counter)

        def leaf_top(used, board):
            tops[tuple(board[(h - 1) * n + c][2] for c in range(n))][used] += 1

        def leaf_bot(used, board):
            bots[tuple(board[h * n + c][0] for c in range(n))][used] += 1

        ct = Solver(n, pieces, td).run(h * n, leaf_top)
        cb = Solver(n, pieces, bu).run((n - h) * n, leaf_bot)
        allmask = (1 << (n * n)) - 1
        nt = sum(sum(v.values()) for v in tops.values())
        nb = sum(sum(v.values()) for v in bots.values())
        st_t = sum(len(v) for v in tops.values())
        st_b = sum(len(v) for v in bots.values())
        sets_t = len({m for v in tops.values() for m in v})
        seam_pairs = sum(sum(v.values()) * sum(bots[s].values()) for s, v in tops.items() if s in bots)
        joined = sum(k * bots[s].get(allmask ^ m, 0) for s, v in tops.items() if s in bots
                     for m, k in v.items())
        disj = sum(k * k2 for s, v in tops.items() if s in bots for m, k in v.items()
                   for m2, k2 in bots[s].items() if not (m & m2))
        assert joined == counts[n * n] == disj, (joined, counts[n * n], disj)
        # all top/bottom pairs irrespective of seam: fraction with complementary sets
        tm, bm = Counter(), Counter()
        for v in tops.values():
            tm.update(v)
        for v in bots.values():
            bm.update(v)
        compl_pairs = sum(k * bm.get(allmask ^ m, 0) for m, k in tm.items())
        top_cells = Counter(cell_kind(n, r, c) for r, c in td[:h * n])
        pc_model = 1.0
        for k, tot in kinds.items():
            pc_model /= comb(tot, top_cells[k])
        # restricted pool: the top half knows its planted piece set
        planted_top = {pid for pid, cell in where.items() if cell < h * n}
        cr = Solver(n, pieces, td, allowed=planted_top).run(h * n)
        rpools = dict(top_cells)
        mr = [x[0] for x in profile(n, td[:h * n], rpools, lpi, lpf)]
        row = dict(
            seed=seed, solutions=counts[n * n], dfs_nodes=sum(counts), dfs_model=10 ** logsum10(mtd),
            peak=max(counts), peak_depth=counts.index(max(counts)), peak_model=10 ** max(mtd),
            peak_model_depth=mtd.index(max(mtd)) + 1,
            tops=nt, tops_model=10 ** mtd[h * n - 1], bottoms=nb, bottoms_model=10 ** mbu[(n - h) * n - 1],
            top_states=st_t, top_sets=sets_t, top_seams=len(tops), seam_space=ci ** (n - 2) * cf ** 2,
            bottom_states=st_b,
            mitm_nodes=sum(ct) + sum(cb), seam_pairs=seam_pairs, joined=joined,
            kill_by_reuse=1 - joined / seam_pairs if seam_pairs else float("nan"),
            compl_frac=compl_pairs / (nt * nb), compl_model=pc_model,
            restricted_nodes=sum(cr), restricted_peak=max(cr), restricted_model_peak=10 ** max(mr),
            free_top_nodes=sum(ct), free_top_peak=max(ct),
        )
        agg["rows"].append(row)
        print("seed %d: " % seed + ", ".join(
            "%s=%s" % (k, ("%.3g" % v) if isinstance(v, float) else v) for k, v in row.items() if k != "seed"))
    rows = agg["rows"]
    if not rows:
        return
    m = len(rows)
    print("\nper-depth mean over %d seeds: depth, mean exact count, model mean (random part), ratio exact/(model+4)" % m)
    for d in range(n * n):
        ex = sum(c[d] for c in agg["full_counts"]) / m
        mo = sum(10 ** x[d] for x in agg["full_model"]) / m
        if (d + 1) % max(1, n // 2) == 0 or d == n * n - 1:
            print("  d=%3d exact=%12.1f model=%12.1f ratio=%.3f" % (d + 1, ex, mo, ex / (mo + 4)))

    def mean(k):
        return sum(r[k] for r in rows) / m
    print("\nsummary (means over seeds):")
    print("  DFS nodes %.4g (model %.4g) ; peak %.4g (model %.4g)" %
          (mean("dfs_nodes"), mean("dfs_model"), mean("peak"), mean("peak_model")))
    print("  top-half fills %.4g (model %.4g) ; bottom-half fills %.4g (model %.4g)" %
          (mean("tops"), mean("tops_model"), mean("bottoms"), mean("bottoms_model")))
    print("  distinct (piece set, seam) states / top fills = %.4f ; distinct piece sets / top fills = %.4f"
          % (mean("top_states") / mean("tops"), mean("top_sets") / mean("tops")))
    print("  distinct top seams %.4g of %d possible" % (mean("top_seams"), rows[0]["seam_space"]))
    print("  two-list generation nodes / DFS nodes = %.3f" % (mean("mitm_nodes") / mean("dfs_nodes")))
    print("  seam-matching pairs %.4g, joined (complementary) %.4g, killed by piece reuse %.6f"
          % (mean("seam_pairs"), mean("joined"), 1 - mean("joined") / mean("seam_pairs")))
    print("  fraction of all (top, bottom) pairs with complementary sets: measured %.3g, model 1/prod C = %.3g"
          % (mean("compl_frac"), mean("compl_model")))
    print("  top half, free pool: nodes %.4g peak %.4g ; planted pool: nodes %.4g peak %.4g (model peak %.4g)"
          % (mean("free_top_nodes"), mean("free_top_peak"), mean("restricted_nodes"),
             mean("restricted_peak"), mean("restricted_model_peak")))


if __name__ == "__main__":
    main()
