"""Toy probes for notes/far-ideas-other-fields.md. TOYS ONLY: generated boards, never Eternity II progress.

Run: nice -n 10 timeout 900 python3 -I far_probes.py <probe> [args]
  consensus   : colour-merge relaxation (renormalisation / homotopy). Do coarse solutions point at the truth?
  rotation    : value of knowing every tile's rotation (print-sheet side channel), DFS node counts.
  classprod   : Conway-Lagarias class-product refuter in S_6 on row-scan prefixes.
Stdlib only, deterministic per seed.
"""
import random
import sys
from itertools import permutations

GREY = 0


def rot(t, r):
    n, e, s, w = t
    for _ in range(r):
        n, e, s, w = w, n, e, s
    return (n, e, s, w)


def paint(n, F, C, rng):
    """Planted board: rim joints use colours 1..F, every other inner joint F+1..F+C. Returns tiles (solution order, scrambled rotation) and truth."""
    h = [[0] * n for _ in range(n + 1)]  # h[i][j]: colour on the horizontal joint above row i, column j
    v = [[0] * (n + 1) for _ in range(n)]  # v[i][j]: colour on the vertical joint left of column j, row i
    for i in range(1, n):
        for j in range(n):
            rim = j == 0 or j == n - 1
            h[i][j] = rng.randint(1, F) if rim else rng.randint(F + 1, F + C)
    for i in range(n):
        for j in range(1, n):
            rim = i == 0 or i == n - 1
            v[i][j] = rng.randint(1, F) if rim else rng.randint(F + 1, F + C)
    board = []
    for i in range(n):
        for j in range(n):
            board.append((h[i][j], v[i][j + 1], h[i + 1][j], v[i][j]))
    # shuffle tile ids, keep truth as (tile id, rotation) per cell
    ids = list(range(n * n))
    rng.shuffle(ids)
    tiles = [None] * (n * n)
    truth = [None] * (n * n)
    for cell, tid in enumerate(ids):
        r = rng.randrange(4)
        tiles[tid] = rot(board[cell], (4 - r) % 4)  # stored so that rot(stored, r) == board cell
        truth[cell] = (tid, r)
    return tiles, truth


class Solver:
    def __init__(self, n, tiles, clue_cell=None, clue=None, fixed_rot=None):
        self.n, self.tiles = n, tiles
        self.opts = {}
        for tid, t in enumerate(tiles):
            rots = [fixed_rot[tid]] if fixed_rot is not None else range(4)
            seen = set()
            for r in rots:
                rt = rot(t, r)
                if rt in seen:
                    continue
                seen.add(rt)
                self.opts.setdefault((rt[0], rt[3]), []).append((tid, r, rt[1], rt[2]))
        self.clue_cell, self.clue = clue_cell, clue

    def run(self, cap_solutions=10**9, cap_nodes=10**9, record=True, stop_first=False):
        n = self.n
        N = n * n
        used = [False] * len(self.tiles)
        if self.clue is not None:
            used[self.clue[0]] = True
        place = [None] * N
        east = [GREY] * N
        south = [GREY] * N
        sols = []
        nodes = [0]
        clue_cell, clue = self.clue_cell, self.clue
        opts = self.opts
        tiles = self.tiles

        def rec(k):
            if k == N:
                if record:
                    sols.append(tuple(place))
                else:
                    sols.append(None)
                return len(sols) >= cap_solutions or stop_first
            if nodes[0] >= cap_nodes:
                return True
            i, j = divmod(k, n)
            needN = south[k - n] if i > 0 else GREY
            needW = east[k - 1] if j > 0 else GREY
            lastrow, lastcol = i == n - 1, j == n - 1
            if k == clue_cell:
                tid, r = clue
                rt = rot(tiles[tid], r)
                cands = [(tid, r, rt[1], rt[2])] if (rt[0], rt[3]) == (needN, needW) else []
                free = True
            else:
                cands = opts.get((needN, needW), ())
                free = False
            for tid, r, e, s in cands:
                if (not free and used[tid]) or ((e == GREY) != lastcol) or ((s == GREY) != lastrow):
                    continue
                nodes[0] += 1
                used[tid] = True
                place[k] = (tid, r)
                east[k], south[k] = e, s
                if rec(k + 1):
                    if not free:
                        used[tid] = False
                    return True
                if not free:
                    used[tid] = False
            return False

        rec(0)
        return sols, nodes[0]


def content(tiles, cell_val):
    tid, r = cell_val
    return rot(tiles[tid], r)


# ---------------------------------------------------------------- probe 1: consensus of a coarse (merged-colour) ensemble
def probe_consensus(n=6, F=2, C=4, seeds=20, merges=1, cap=200000):
    print(f"# consensus probe: n={n} F={F} C={C} merges={merges} (merge interior colours pairwise), clue at centre")
    print("seed fine_sols coarse_sols nodes_fine nodes_coarse_all truth_mean_overlap others_mean_overlap truth_rank_pct vote_acc_truth vote_acc_truth_percentile")
    rows = []
    for seed in range(seeds):
        rng = random.Random(1000 + seed)
        tiles, truth = paint(n, F, C, rng)
        cc = (n // 2) * n + n // 2
        fine = Solver(n, tiles, cc, truth[cc])
        fsols, fnodes = fine.run(cap_solutions=cap)
        # coarse map: merge interior colours F+1<-F+2, F+3<-F+4 ... (merges pairs)
        m = {c: c for c in range(F + C + 1)}
        for q in range(merges):
            m[F + 2 + 2 * q] = F + 1 + 2 * q
        ctiles = [tuple(m[x] for x in t) for t in tiles]
        coarse = Solver(n, ctiles, cc, truth[cc])
        csols, cnodes = coarse.run(cap_solutions=cap)
        if len(csols) >= cap or len(csols) < 3:
            print(seed, len(fsols), len(csols), "skipped (cap or too few)", flush=True)
            continue
        cells = [k for k in range(n * n) if k != cc]
        truthC = [content(tiles, truth[k]) for k in range(n * n)]
        S = [[content(tiles, s[k]) for k in range(n * n)] for s in csols]

        def ov(a, b):
            return sum(a[k] == b[k] for k in cells) / len(cells)

        # sample pairs for centrality to keep cost bounded
        M = len(S)
        idx = list(range(M))
        sample = idx if M <= 400 else rng.sample(idx, 400)
        cent = {}
        for a in sample:
            cent[a] = sum(ov(S[a], S[b]) for b in sample if b != a) / (len(sample) - 1)
        truth_cent = sum(ov(truthC, S[b]) for b in sample) / len(sample)
        # the truth is itself one of the coarse solutions; rank its centrality among the sampled
        rank = sum(c < truth_cent for c in cent.values()) / len(cent)
        others = sum(cent.values()) / len(cent)
        # majority vote per cell over the coarse ensemble
        vote = []
        for k in range(n * n):
            cnt = {}
            for s in S:
                cnt[s[k]] = cnt.get(s[k], 0) + 1
            vote.append(max(cnt, key=cnt.get))
        acc_truth = ov(vote, truthC)
        # null: the vote's accuracy against every sampled coarse solution; report the truth's percentile
        accs = [ov(vote, S[b]) for b in sample]
        acc_null = sum(a < acc_truth for a in accs) / len(accs)
        rows.append((len(fsols), M, fnodes, cnodes, truth_cent, others, rank, acc_truth, acc_null))
        print(seed, len(fsols), M, fnodes, cnodes, f"{truth_cent:.3f}", f"{others:.3f}", f"{rank:.2f}", f"{acc_truth:.3f}", f"{acc_null:.2f}", flush=True)
    if rows:
        k = len(rows)
        mean = lambda i: sum(r[i] for r in rows) / k
        print(f"# boards used {k}; mean truth overlap {mean(4):.3f} vs others {mean(5):.3f}; mean truth centrality rank {mean(6):.2f} (0.5 = no signal);"
              f" vote accuracy truth {mean(7):.3f}, its percentile among coarse solutions {mean(8):.2f} (0.5 = no signal); mean nodes fine {mean(2):.0f} coarse(all sols) {mean(3):.0f}")


# ---------------------------------------------------------------- probe 2: value of known rotations
def probe_rotation(n=7, F=3, C=6, seeds=10, cap_nodes=5 * 10**7):
    print(f"# rotation probe: n={n} F={F} C={C}, full enumeration with centre clue; nodes with rotations unknown vs known")
    print("seed sols_unknown nodes_unknown sols_known nodes_known log2_ratio")
    import math
    tot = []
    for seed in range(seeds):
        rng = random.Random(2000 + seed)
        tiles, truth = paint(n, F, C, rng)
        cc = (n // 2) * n + n // 2
        fixed = [None] * len(tiles)
        for cell, (tid, r) in enumerate(truth):
            fixed[tid] = r
        s0, n0 = Solver(n, tiles, cc, truth[cc]).run(record=False, cap_nodes=cap_nodes)
        s1, n1 = Solver(n, tiles, cc, truth[cc], fixed_rot=fixed).run(record=False, cap_nodes=cap_nodes)
        capped = " (capped)" if n0 >= cap_nodes else ""
        lr = math.log2(n0 / max(n1, 1))
        tot.append(lr)
        print(seed, len(s0), n0, len(s1), n1, f"{lr:.1f}{capped}")
    print(f"# mean log2(nodes unknown / nodes known) = {sum(tot) / len(tot):.1f} bits over {n * n} cells")


# ---------------------------------------------------------------- probe 3: Conway-Lagarias class-product refuter
def cycle_type(p):
    seen = [False] * len(p)
    ct = []
    for i in range(len(p)):
        if not seen[i]:
            L = 0
            j = i
            while not seen[j]:
                seen[j] = True
                j = p[j]
                L += 1
            ct.append(L)
    return tuple(sorted(ct))


def compose(a, b):  # apply a then b
    return tuple(b[a[i]] for i in range(len(a)))


def probe_classprod(n=6, F=2, C=4, seeds=10, deg=6, depth_samples=400):
    print(f"# class-product refuter: n={n} F={F} C={C}, colours -> random involutions of S_{deg}; grey -> a fixed involution")
    elems = list(permutations(range(deg)))
    classes = sorted(set(cycle_type(p) for p in elems))
    cid = {c: i for i, c in enumerate(classes)}
    reps = {}
    members = {}
    for p in elems:
        c = cid[cycle_type(p)]
        reps.setdefault(c, p)
        members.setdefault(c, []).append(p)
    K = len(classes)
    prod = [[0] * K for _ in range(K)]  # bitmask of classes in C_a * C_b
    for a in range(K):
        x = reps[a]
        for b in range(K):
            mask = 0
            for y in members[b]:
                mask |= 1 << cid[cycle_type(compose(x, y))]
            prod[a][b] = mask
    sign = [(sum(L - 1 for L in classes[c]) % 2) for c in range(K)]
    invols = [p for p in elems if all(p[p[i]] == i for i in range(deg)) and p != tuple(range(deg))]
    N = n * n
    agg = {}
    sound_fail = 0
    for seed in range(seeds):
        rng = random.Random(3000 + seed)
        tiles, truth = paint(n, F, C, rng)
        g = {c: rng.choice(invols) for c in range(1, F + C + 1)}
        g[GREY] = rng.choice(invols)
        tclass = []
        for t in tiles:
            w = tuple(range(deg))
            for c in t:  # clockwise word N E S W; rotation only conjugates
                w = compose(w, g[c])
            tclass.append(cid[cycle_type(w)])

        def boundary_class(placed_cols, k):
            """placed_cols(cell, side) gives colour seen from the unfilled region; region = cells k..N-1 in row-major order."""
            r0, c0 = divmod(k, n)
            word = []
            # clockwise from top-left corner of cell (r0,c0)
            for j in range(c0, n):
                word.append(placed_cols((r0, j), 'N'))
            for i in range(r0, n):
                word.append(GREY)  # east rim
            if r0 == n - 1:  # region is the tail of the last row
                for j in range(n - 1, c0 - 1, -1):
                    word.append(GREY)
                word.append(placed_cols((r0, c0), 'W') if c0 > 0 else GREY)
                w = tuple(range(deg))
                for c in word:
                    w = compose(w, g[c])
                return cid[cycle_type(w)]
            for j in range(n - 1, -1, -1):
                word.append(GREY)  # south rim
            for i in range(n - 1, r0, -1):
                word.append(GREY)  # west rim
            if c0 > 0:
                for j in range(0, c0):
                    word.append(placed_cols((r0 + 1, j), 'N') if r0 + 1 < n else GREY)
                word.append(placed_cols((r0, c0), 'W'))
            else:
                word.append(GREY)  # west side of (r0,0)
            w = tuple(range(deg))
            for c in word:
                w = compose(w, g[c])
            return cid[cycle_type(w)]

        def test(place, k):
            """True if refuted. place: list of (tid, r) for cells < k."""
            def seen(cell, side):
                i, j = cell
                if side == 'N':
                    if i == 0:
                        return GREY
                    return rot(tiles[place[(i - 1) * n + j][0]], place[(i - 1) * n + j][1])[2]
                if side == 'W':
                    if j == 0:
                        return GREY
                    return rot(tiles[place[i * n + j - 1][0]], place[i * n + j - 1][1])[1]
            if k == 0 or k >= N:
                return False, False
            bc = boundary_class(seen, k)
            usedset = set(p[0] for p in place[:k])
            S = 1  # class 0 = identity (cycle type all ones sorts first)
            assert classes[0] == tuple([1] * deg)
            for tid in range(len(tiles)):
                if tid in usedset:
                    continue
                newS = 0
                a = 0
                m = S
                while m:
                    if m & 1:
                        newS |= prod[a][tclass[tid]]
                    m >>= 1
                    a += 1
                S = newS
            ref_class = not (S >> bc) & 1
            # abelian part only: parity
            par = sum(sign[tclass[t]] for t in range(len(tiles)) if t not in usedset) % 2
            ref_sign = par != sign[bc]
            return ref_class, ref_sign

        # soundness on the truth at every depth
        tp = [truth[k] for k in range(N)]
        for k in range(1, N):
            rc, rs = test(tp, k)
            if rc or rs:
                sound_fail += 1
        # wrong prefixes: random valid prefixes (no clue), sampled by random DFS dives
        solver = Solver(n, tiles)
        for _ in range(depth_samples):
            place = []
            used = set()
            east, south = [], []
            k = 0
            while k < N:
                i, j = divmod(k, n)
                needN = south[k - n] if i > 0 else GREY
                needW = east[k - 1] if j > 0 else GREY
                cands = [c for c in solver.opts.get((needN, needW), ()) if c[0] not in used and ((c[2] == GREY) == (j == n - 1)) and ((c[3] == GREY) == (i == n - 1))]
                if not cands:
                    break
                tid, r, e, s = rng.choice(cands)
                place.append((tid, r))
                used.add(tid)
                east.append(e)
                south.append(s)
                k += 1
                if place[:k] == tp[:k]:
                    continue
                rc, rs = test(place, k)
                d = agg.setdefault(k, [0, 0, 0])
                d[0] += 1
                d[1] += rc
                d[2] += rs
    print(f"# soundness: truth prefixes refuted {sound_fail} times (must be 0)")
    print("depth tested refuted_by_class refuted_by_sign_only extra_from_nonabelian")
    for k in sorted(agg):
        t, rc, rs = agg[k]
        print(k, t, rc, rs, rc - rs)


if __name__ == "__main__":
    probe = sys.argv[1]
    args = [int(a) for a in sys.argv[2:]]
    {"consensus": probe_consensus, "rotation": probe_rotation, "classprod": probe_classprod}[probe](*args)
