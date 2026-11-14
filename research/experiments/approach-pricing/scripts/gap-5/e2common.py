"""Shared helpers. Stdlib only. Pieces are 4-tuples (top, right, bottom, left), 0 = grey rim.
The piece file is engine/data/official_eternity2.csv of the checkout this folder sits in
(five levels up), or the path in the environment variable E2_CSV."""
import os

N = 16
FRAME = (1, 2, 3, 4, 5)
INNER = tuple(range(6, 23))
HERE = os.path.dirname(os.path.abspath(__file__))
CANDIDATES = [
    os.environ.get("E2_CSV", ""),
    os.path.join(HERE, "..", "..", "..", "..", "..", "engine", "data", "official_eternity2.csv"),
]


def csv_path():
    for p in CANDIDATES:
        if p and os.path.exists(p):
            return p
    raise SystemExit("official_eternity2.csv not found; set E2_CSV")


def load():
    """Returns (pieces, hints). pieces[i] is piece number i+1. hints: list of (number, x, y, rot)."""
    rows = [l.strip() for l in open(csv_path()) if l.strip()]
    pieces, hints = [], []
    for i, l in enumerate(rows[1:]):
        c = l.split(",")
        e = [int(x, 2) for x in c[:4]]
        pieces.append(tuple(0 if v == 65535 else v for v in e))
        x, y, r = int(c[4]), int(c[5]), int(c[6])
        if x or y or r:
            hints.append((i + 1, x, y, r))
    return pieces, hints


def rots(p):
    return [p[i:] + p[:i] for i in range(4)]


def canon(p):
    return min(rots(p))


def klass(p):
    z = p.count(0)
    return {2: "corner", 1: "edge", 0: "inner"}[z]


def paint(rng, ring_pool, inner_pool):
    """Exchangeable painter: a solved 16x16 whose ring joints carry a uniform shuffle of
    ring_pool (60 colours) and whose seam + inner joints carry a uniform shuffle of
    inner_pool (420 colours). Returns board[r][c] = (top, right, bottom, left)."""
    ring = ring_pool[:]
    rng.shuffle(ring)
    inner = inner_pool[:]
    rng.shuffle(inner)
    ri = ii = 0
    H = [[0] * (N - 1) for _ in range(N)]   # joint between (r,c) and (r,c+1)
    V = [[0] * N for _ in range(N - 1)]     # joint between (r,c) and (r+1,c)
    for r in range(N):
        for c in range(N - 1):
            if r == 0 or r == N - 1:
                H[r][c] = ring[ri]; ri += 1
            else:
                H[r][c] = inner[ii]; ii += 1
    for r in range(N - 1):
        for c in range(N):
            if c == 0 or c == N - 1:
                V[r][c] = ring[ri]; ri += 1
            else:
                V[r][c] = inner[ii]; ii += 1
    assert ri == 60 and ii == 420
    board = []
    for r in range(N):
        row = []
        for c in range(N):
            t = V[r - 1][c] if r > 0 else 0
            b = V[r][c] if r < N - 1 else 0
            l = H[r][c - 1] if c > 0 else 0
            rt = H[r][c] if c < N - 1 else 0
            row.append((t, rt, b, l))
        board.append(row)
    return board


def official_pools(pieces):
    """Joint pools implied by the official side counts."""
    from collections import Counter
    cnt = Counter(x for p in pieces for x in p if x)
    ring_pool = [c for c in FRAME for _ in range(cnt[c] // 2)]
    inner_pool = [c for c in INNER for _ in range(cnt[c] // 2)]
    return ring_pool, inner_pool


def file_order(board_pieces):
    """The official file's rule applied to any piece multiset: corners, edges, inner;
    each piece in its lexicographically smallest rotation (grey counts as 0, so the rim
    sorts first: corners come out as (0,0,a,b), edges as (0,a,b,c)); ascending."""
    cs = [canon(p) for p in board_pieces]
    key = {"corner": 0, "edge": 1, "inner": 2}
    return sorted(cs, key=lambda p: (key[klass(p)], p))
