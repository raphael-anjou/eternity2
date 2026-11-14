"""Review check 2 (stdlib): bag counts behind parity-defect-floor and
why-e2-is-hard, a brute-force 478 shell on fresh painted boards, the
expected-one-solution crossing, and the random-permutation null for
sigma-cycles. Written from scratch; rv_lib is used only for the crossing
(its first-moment model was refereed tonight).

Run: python3 -I 02_piece_facts_shell_crossing.py
"""
import math
import os
import random
import statistics
import sys
import time
from itertools import combinations

HERE = os.path.dirname(os.path.abspath(__file__))
CSV = os.path.abspath(os.path.join(HERE, *[".."] * 6, "engine", "data", "official_eternity2.csv"))
T0 = time.time()


def rot(p, k):
    """turn p clockwise k quarter turns; p = (top, right, bottom, left)"""
    return p[4 - k:] + p[:4 - k] if k else p


def load():
    out = []
    for l in [l.strip() for l in open(CSV) if l.strip()][1:]:
        c = l.split(",")
        out.append(tuple(0 if int(x, 2) == 65535 else int(x, 2) for x in c[:4]))
    return out


def bag_counts(pieces):
    """near-twin pairs up to rotation (agree on exactly 3 aligned sides, grey
    aligned with grey), by kind; stored-orientation near-twins; turn moves."""
    kinds = [p.count(0) for p in pieces]
    twin = {0: 0, 1: 0, 2: 0}
    cross_kind = 0
    stored = 0
    twin_pairs = []
    for a, b in combinations(range(len(pieces)), 2):
        pa, pb = pieces[a], pieces[b]
        if sum(1 for x, y in zip(pa, pb) if x == y) == 3:
            stored += 1
        if kinds[a] != kinds[b]:
            continue
        hit = False
        for k in range(4):
            q = rot(pb, k)
            if all((x == 0) == (y == 0) for x, y in zip(pa, q)) and sum(1 for x, y in zip(pa, q) if x == y) == 3:
                hit = True
        if hit:
            twin[kinds[a]] += 1
            twin_pairs.append((a, b))
    turn_tiles = set()
    half = quarter = 0
    moves = 0
    for i, p in enumerate(pieces):
        if kinds[i]:
            continue
        for k in (1, 2, 3):
            kept = sum(1 for x, y in zip(p, rot(p, k)) if x == y)
            if kept == 2:
                moves += 1
                turn_tiles.add(i)
        if sum(1 for x, y in zip(p, rot(p, 2)) if x == y) == 2:
            half += 1
        if any(sum(1 for x, y in zip(p, rot(p, k)) if x == y) == 2 for k in (1, 3)):
            quarter += 1
    return twin, stored, len(turn_tiles), half, quarter, moves, twin_pairs


pieces = load()
tw, stored, ntt, half, quarter, moves, _ = bag_counts(pieces)
print("== official set, bag counts ==")
print("  near-twin pairs up to rotation: interior %d, edge %d, corner %d, total %d" % (tw[0], tw[1], tw[2], sum(tw.values())))
print("  near-twin pairs in the stored orientation (3 of 4 same-position sides): %d" % stored)
print("  interior tiles with a turn that keeps exactly two sides: %d (half turn %d, quarter turn %d); (tile, turn) moves: %d" % (ntt, half, quarter, moves))
print("  page: 50 + 23 + 3 = 76 as an upper bound; memo: at least 125 + 29 = 154")

# fixed-orientation matching cap
def cap(ps):
    e, w, n, s = {}, {}, {}, {}
    for p in ps:
        for d, v in ((n, p[0]), (e, p[1]), (s, p[2]), (w, p[3])):
            if v:
                d[v] = d.get(v, 0) + 1
    return sum(min(e.get(c, 0), w.get(c, 0)) for c in set(e) | set(w)) + sum(min(n.get(c, 0), s.get(c, 0)) for c in set(n) | set(s))


print("\n== fixed-orientation cap (why-e2-is-hard, 307 of 480) ==")
print("  stored orientation: %d" % cap(pieces))
rng = random.Random(1)
vals = [cap([rot(p, rng.randrange(4)) for p in pieces]) for _ in range(2000)]
print("  2000 random orientations of every piece: mean %.1f, sd %.1f, min %d, max %d" % (statistics.mean(vals), statistics.pstdev(vals), min(vals), max(vals)))
# stored-orientation convention
int_min_top = sum(1 for p in pieces if 0 not in p and p[0] == min(p))
edge_grey_top = sum(1 for p in pieces if p.count(0) == 1 and p[0] == 0)
print("  stored orientation: interior pieces with their smallest colour on top %d of 196 ; edge pieces with grey on top %d of 56" % (int_min_top, edge_grey_top))


# ------------------------------------------------------------- 478 shell
def painted(n, cf, ci, rng):
    col = {}
    for r in range(n):
        for c in range(n):
            if c < n - 1:
                col[(r, c, "h")] = (rng.randint(1, cf) if r in (0, n - 1) else cf + rng.randint(1, ci))
            if r < n - 1:
                col[(r, c, "v")] = (rng.randint(1, cf) if c in (0, n - 1) else cf + rng.randint(1, ci))
    board = {}
    for r in range(n):
        for c in range(n):
            board[(r, c)] = (col[(r - 1, c, "v")] if r else 0, col[(r, c, "h")] if c < n - 1 else 0,
                             col[(r, c, "v")] if r < n - 1 else 0, col[(r, c - 1, "h")] if c else 0)
    return board


def shell(board, n):
    """all boards at exactly two broken joints reachable from the solved board
    by one swap of two cells (any legal rotations) or one in-place turn."""
    cells = sorted(board)
    NBR = ((-1, 0), (0, 1), (1, 0), (0, -1))
    OPP = (2, 3, 0, 1)

    def broken_at(cell, q, other=None):
        """broken joints around `cell` if it held q; None if grey is misplaced.
        `other` = (cell2, q2) overrides the content of one neighbour."""
        r, c = cell
        b = 0
        for s in range(4):
            x = (r + NBR[s][0], c + NBR[s][1])
            if x not in board:
                if q[s] != 0:
                    return None
                continue
            if q[s] == 0:
                return None
            v = other[1][OPP[s]] if other and other[0] == x else board[x][OPP[s]]
            if q[s] != v:
                b += 1
        return b

    turn = 0
    for cell in cells:
        for k in (1, 2, 3):
            if broken_at(cell, rot(board[cell], k)) == 2:
                turn += 1
    # table of legal rotations of every piece at every cell
    tab = {}
    for x in cells:
        for y in cells:
            tab[(x, y)] = [(k, broken_at(x, rot(board[y], k))) for k in range(4)]
    swap = adj_swap = 0
    for x, y in combinations(cells, 2):
        adjacent = abs(x[0] - y[0]) + abs(x[1] - y[1]) == 1
        if not adjacent:
            for ka, ba in tab[(x, y)]:
                if ba is None or ba > 2:
                    continue
                for kb, bb in tab[(y, x)]:
                    if bb is not None and ba + bb == 2:
                        swap += 1
        else:
            for ka in range(4):
                qa = rot(board[y], ka)  # piece of y now at x
                for kb in range(4):
                    qb = rot(board[x], kb)  # piece of x now at y
                    b1 = broken_at(x, qa, (y, qb))
                    b2 = broken_at(y, qb, (x, qa))
                    if b1 is None or b2 is None:
                        continue
                    shared = 1 if qa[[s for s in range(4) if (x[0] + NBR[s][0], x[1] + NBR[s][1]) == y][0]] != qb[[s for s in range(4) if (y[0] + NBR[s][0], y[1] + NBR[s][1]) == x][0]] else 0
                    if b1 + b2 - shared == 2:
                        adj_swap += 1
    return turn, swap, adj_swap


print("\n== 478 shell around a solution: brute force on fresh painted 16x16 boards (5 frame colours, 17 interior) ==")
print("  board | turn neighbours at 478 | bag (tile, turn) moves | swap neighbours at 478 (non-adjacent + adjacent) | bag near-twin pairs up to rotation (interior, edge, corner)")
for seed in (11, 12, 13):
    rng = random.Random(seed)
    bd = painted(16, 5, 17, rng)
    ps = [bd[c] for c in sorted(bd)]
    tw, st, ntt, hf, qt, mv, _ = bag_counts(ps)
    turn, swap, adj = shell(bd, 16)
    print("  seed %d | %d | %d | %d + %d = %d | %d (%d, %d, %d)   => shell size %d, page-style count 'interior twins + half-turn tiles + quarter-turn tiles' = %d"
          % (seed, turn, mv, swap, adj, swap + adj, sum(tw.values()), tw[0], tw[1], tw[2], turn + swap + adj, tw[0] + hf + qt))
    sys.stdout.flush()

# ------------------------------------------------------------- crossing
print("\n== expected-one-solution crossing at 17 interior + 5 frame colours (how-hard-is-this-instance) ==")
lf = lambda k: math.lgamma(k + 1) / math.log(10)
print("  N | page formula m! 4^m / 17^j, j = 2(N-2)(N-3) | same with the 4(N-2) seam joints and the frame, independent joints | refereed fixed-census model on a generated balanced board (rv_lib)")
sys.path.insert(0, os.path.join(HERE, "..", "price-and-ledger-review"))
import rv_lib

for N in range(10, 17):
    m = (N - 2) ** 2
    e = 4 * (N - 2)
    j = 2 * (N - 2) * (N - 3)
    page = lf(m) + m * math.log10(4) - j * math.log10(17)
    full = math.log10(24) + lf(e) + lf(m) + m * math.log10(4) - (j + e) * math.log10(17) - 4 * (N - 1) * math.log10(5)
    rng = random.Random(100 + N)
    ps, sol = rv_lib.gen_planted(N, 5, 17, rng, balanced=True)
    M = rv_lib.Model(N, rv_lib.census(ps))
    order = rv_lib.ring(N) + [(r, c) for r in range(1, N - 1) for c in range(1, N - 1)]
    pooled = M.profile(order)[-1]
    # five clues at interior cells spread like the official ones
    q = max(2, N // 8)
    hints = [(q, q), (q, N - 1 - q), (N - 1 - q, q), (N - 1 - q, N - 1 - q), (N // 2, N // 2 - 1)]
    pooled5 = M.profile(order, hints)[-1]
    print("  %2d | %7.1f | %7.1f | no clue %7.1f ; five clues %7.1f" % (N, page, full, pooled, pooled5))
print("  (log10 of the expected number of solutions; the planted solution itself is not counted)")

# ------------------------------------------------------------- sigma-cycles null
print("\n== sigma-cycles: largest cycle of a uniform class-preserving permutation (196 + 56 + 4) ==")
rng = random.Random(5)


def largest(n):
    p = list(range(n))
    rng.shuffle(p)
    seen = [False] * n
    best = 0
    for i in range(n):
        if not seen[i]:
            l = 0
            j = i
            while not seen[j]:
                seen[j] = True
                j = p[j]
                l += 1
            best = max(best, l)
    return best


xs = sorted(max(largest(196), largest(56), largest(4)) for _ in range(20000))
print("  median %d, 5%% %d, 95%% %d, mean %.1f ; page: corpus largest loop 6 to 195, median 119" % (xs[10000], xs[1000], xs[19000], statistics.mean(xs)))
print("[%.0fs]" % (time.time() - T0))
