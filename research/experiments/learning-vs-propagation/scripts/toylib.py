"""Shared by the toy scripts. TOY boards, not Eternity II.
Planted framed edge-matching boards and a direct CNF encoding. Stdlib only."""
import random


def rot(e, r):
    return tuple(e[(i + 4 - r) % 4] for i in range(4))


def generate(n, cf, ci, seed, full=False):
    """Planted board. Joints between two frame cells take colours 1..cf, every
    other joint cf+1..cf+ci, the rim is 0. Returns pieces (shuffled, rotated)
    and the pin (piece, rotation) for cell 0."""
    rng = random.Random(seed * 7919 + n * 104729 + ci * 31 + cf)
    fr = lambda y, x: x in (0, n - 1) or y in (0, n - 1)
    H = [[(rng.randint(1, cf) if y in (0, n - 1) else rng.randint(cf + 1, cf + ci)) for x in range(n - 1)] for y in range(n)]
    V = [[(rng.randint(1, cf) if x in (0, n - 1) else rng.randint(cf + 1, cf + ci)) for x in range(n)] for y in range(n - 1)]
    cells = []
    for y in range(n):
        for x in range(n):
            cells.append((V[y - 1][x] if y else 0, H[y][x] if x < n - 1 else 0, V[y][x] if y < n - 1 else 0, H[y][x - 1] if x else 0))
    perm = list(range(n * n))
    rng.shuffle(perm)
    pieces, where = [None] * (n * n), {}
    for pos, pid in enumerate(perm):
        r = rng.randrange(4)
        pieces[pid] = rot(cells[pos], r)
        where[pos] = (pid, (4 - r) % 4)
    assert rot(pieces[where[0][0]], where[0][1]) == cells[0]
    return (pieces, where) if full else (pieces, where[0])


def encode(n, pieces, pin):
    nv = [0]

    def new():
        nv[0] += 1
        return nv[0]

    cnf = []
    X = {}  # (pos, piece, edges) -> var
    by_cell, by_piece = {}, {}
    for pos in range(n * n):
        y, x = divmod(pos, n)
        for p, e in enumerate(pieces):
            seen = set()
            for r in range(4):
                o = rot(e, r)
                if o in seen:
                    continue
                seen.add(o)
                if (o[0] == 0) != (y == 0) or (o[1] == 0) != (x == n - 1) or (o[2] == 0) != (y == n - 1) or (o[3] == 0) != (x == 0):
                    continue
                if pin is not None and pos == 0 and (p, r) != pin:
                    continue
                v = new()
                X[(pos, p, o)] = v
                by_cell.setdefault(pos, []).append((v, o))
                by_piece.setdefault(p, []).append(v)

    def exactly_one(vs):
        cnf.append(list(vs))
        for i in range(len(vs)):
            for j in range(i + 1, len(vs)):
                cnf.append([-vs[i], -vs[j]])

    for pos in range(n * n):
        exactly_one([v for v, _ in by_cell[pos]])
    for p in range(n * n):
        exactly_one(by_piece[p])
    # joints: side 1 of cell a meets side 3 of cell a+1; side 2 of a meets side 0 of a+n
    for pos in range(n * n):
        y, x = divmod(pos, n)
        for other, sa, sb in ((pos + 1, 1, 3), (pos + n, 2, 0)):
            if (sa == 1 and x == n - 1) or (sa == 2 and y == n - 1):
                continue
            cols = sorted({o[sa] for _, o in by_cell[pos]} | {o[sb] for _, o in by_cell[other]})
            hv = {k: new() for k in cols}
            for i in range(len(cols)):
                for j in range(i + 1, len(cols)):
                    cnf.append([-hv[cols[i]], -hv[cols[j]]])
            for cell, s in ((pos, sa), (other, sb)):
                sup = {k: [] for k in cols}
                for v, o in by_cell[cell]:
                    cnf.append([-v, hv[o[s]]])
                    sup[o[s]].append(v)
                for k in cols:
                    cnf.append([-hv[k]] + sup[k])
    return cnf, X, nv[0]
