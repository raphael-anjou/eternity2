"""TOY BOARDS ONLY. The painter's own solution of a generated board, recovered without search, so
an 8x8 board can be measured without the Python enumerator (hours at 8x8).

common.gen paints a board, then shuffles the tile list with random.shuffle. Shuffling a list of
indices of the same length with the same generator state draws the same permutation (shuffle's
draws depend only on the length), so replaying gen with an index list gives where each painted
tile went. The painted board (tile x at cell x, rotation 0) is then turned by quarter turns until
the pinned tile (common.pin_of) sits at cell 0, which is the frame every other script uses.
check(): on boards with a full enumeration, the recovered board must be one of the solutions.
Usage: construct.py   (runs check() on 5x5, 6x6 and 7x7 boards)
"""
import os, random, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
from common import gen, pin_of, rot


def painted(n, cf, ci, seed, max_tries=20000):
    """Replay of common.gen returning (shuffled tiles, idx) with shuffled[j] = painted[idx[j]]."""
    rng = random.Random(seed)
    for _ in range(max_tries):
        hor = [[0] * (n + 1) for _ in range(n)]
        ver = [[0] * n for _ in range(n + 1)]
        for r in range(n):
            for c in range(1, n):
                ring = r in (0, n - 1)
                hor[r][c] = rng.randint(1, cf) if ring else cf + rng.randint(1, ci)
        for r in range(1, n):
            for c in range(n):
                ring = c in (0, n - 1)
                ver[r][c] = rng.randint(1, cf) if ring else cf + rng.randint(1, ci)
        tiles = [(ver[r][c], hor[r][c + 1], ver[r + 1][c], hor[r][c]) for r in range(n) for c in range(n)]
        canon = set(); ok = True
        for t in tiles:
            rots = [t[k:] + t[:k] for k in range(4)]
            if len(set(rots)) < 4 and 0 not in t:
                ok = False
            m = min(rots)
            if m in canon:
                ok = False
            canon.add(m)
        if ok:
            idx = list(range(len(tiles)))
            rng.shuffle(idx)
            return [tiles[i] for i in idx], idx
    return None


def constructed_solution(n, cf, ci, seed):
    """The painted board in the pinned frame, as a tuple of (tile, rotation) per cell."""
    shuffled, idx = painted(n, cf, ci, seed)
    assert shuffled == gen(n, cf, ci, seed)
    where = {x: j for j, x in enumerate(idx)}           # painted cell x -> shuffled tile index
    board = {(x // n, x % n): (where[x], 0) for x in range(n * n)}
    pin_t, pin_k, _ = pin_of(n, shuffled)
    for _ in range(4):
        if board[(0, 0)][0] == pin_t:
            break
        # quarter turn clockwise: (r, c) -> (c, n - 1 - r); sides (N,E,S,W) -> (W,N,E,S) = rot(t, 3)
        board = {(c, n - 1 - r): (t, (k + 3) % 4) for (r, c), (t, k) in board.items()}
    assert board[(0, 0)] == (pin_t, pin_k), (board[(0, 0)], pin_t, pin_k)
    sol = tuple(board[(x // n, x % n)] for x in range(n * n))
    # every inner edge matches
    for x in range(n * n):
        r, c = divmod(x, n)
        p = rot(shuffled[sol[x][0]], sol[x][1])
        if c < n - 1:
            assert p[1] == rot(shuffled[sol[x + 1][0]], sol[x + 1][1])[3]
        if r < n - 1:
            assert p[2] == rot(shuffled[sol[x + n][0]], sol[x + n][1])[0]
    return sol


def check():
    from common import enumerate_all
    for (n, cf, ci, s) in [(5, 3, 4, 0), (5, 3, 4, 4), (6, 4, 5, 1), (7, 4, 7, 0)]:
        sols = enumerate_all(n, gen(n, cf, ci, s))["sols"]
        print(n, cf, ci, s, "painted solution among the enumerated ones:", constructed_solution(n, cf, ci, s) in sols,
              "of", len(sols))


if __name__ == "__main__":
    check()
