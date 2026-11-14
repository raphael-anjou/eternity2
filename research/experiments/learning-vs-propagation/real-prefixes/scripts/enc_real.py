"""learning-real-prefixes: CNF of the board below a prefix. The toy direct encoding of toylib.encode (one variable
per placement, pairwise exactly-one per cell and per piece, one colour variable per joint and colour with support
clauses), with the pins applied before writing: pinned cells and pinned pieces have no variable, a free cell keeps
only the placements that match its pinned neighbours, joints with a pinned side have no colour variable. This is what
unit propagation makes of the full board with the pins as unit clauses. Placement variables come first, cell by cell
in row order (the static order of arm U0). An empty cell or piece gives the empty clause.
usage: python3 -I enc_real.py file.dyn out.cnf"""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from toylib import rot


def read_dyn(path):
    n, pieces, pins = None, [], {}
    for line in open(path):
        if line[0] == "#":
            continue
        if line[0] == "P":
            c, v = line[1:].split()
            pins[int(c)] = (int(v) // 4, int(v) % 4)
        elif n is None:
            n = int(line)
        else:
            pieces.append(tuple(int(t) for t in line.split()))
    assert len(pieces) == n * n
    return n, pieces, pins


def encode(n, pieces, pins, out):
    fixed = {pos: rot(pieces[p], r) for pos, (p, r) in pins.items()}
    gone = {p for p, _ in pins.values()}
    nv, ncl, chunks = 0, 0, []
    by_cell, by_piece = {}, {}
    for pos in range(n * n):
        if pos in fixed:
            continue
        y, x = divmod(pos, n)
        nbs = [(s, fixed[q][(s + 2) % 4]) for s, q, ok in ((0, pos - n, y > 0), (1, pos + 1, x < n - 1), (2, pos + n, y < n - 1), (3, pos - 1, x > 0)) if ok and q in fixed]
        by_cell[pos] = []
        for p, e in enumerate(pieces):
            if p in gone:
                continue
            seen = set()
            for r in range(4):
                o = rot(e, r)
                if o in seen:
                    continue
                seen.add(o)
                if (o[0] == 0) != (y == 0) or (o[1] == 0) != (x == n - 1) or (o[2] == 0) != (y == n - 1) or (o[3] == 0) != (x == 0):
                    continue
                if any(o[s] != k for s, k in nbs):
                    continue
                nv += 1
                by_cell[pos].append((nv, o))
                by_piece.setdefault(p, []).append(nv)
    nx = nv

    def exactly_one(vs):
        nonlocal ncl
        chunks.append(" ".join(map(str, vs)) + " 0\n" if vs else "0\n")
        m = len(vs)
        for i in range(m):
            a = "-%d -" % vs[i]
            chunks.append("".join([a + "%d 0\n" % w for w in vs[i + 1:]]))
        ncl += 1 + m * (m - 1) // 2

    for pos in sorted(by_cell):
        exactly_one([v for v, _ in by_cell[pos]])
    for p in range(n * n):
        if p not in gone:
            exactly_one(by_piece.get(p, []))
    for pos in sorted(by_cell):
        y, x = divmod(pos, n)
        for other, sa, sb in ((pos + 1, 1, 3), (pos + n, 2, 0)):
            if (sa == 1 and x == n - 1) or (sa == 2 and y == n - 1) or other in fixed:
                continue
            cols = sorted({o[sa] for _, o in by_cell[pos]} | {o[sb] for _, o in by_cell[other]})
            hv = {}
            for k in cols:
                nv += 1
                hv[k] = nv
            for i in range(len(cols)):
                for j in range(i + 1, len(cols)):
                    chunks.append("-%d -%d 0\n" % (hv[cols[i]], hv[cols[j]]))
                    ncl += 1
            for cell, s in ((pos, sa), (other, sb)):
                sup = {k: [] for k in cols}
                for v, o in by_cell[cell]:
                    chunks.append("-%d %d 0\n" % (v, hv[o[s]]))
                    ncl += 1
                    sup[o[s]].append(v)
                for k in cols:
                    chunks.append(" ".join(map(str, [-hv[k]] + sup[k])) + " 0\n")
                    ncl += 1
    with open(out, "w") as f:
        f.write("c OFFICIAL pieces below a prefix, %d pins\nc nplacement %d\np cnf %d %d\n" % (len(pins), nx, nv, ncl))
        f.writelines(chunks)
    return nx, nv, ncl


if __name__ == "__main__":
    n, pieces, pins = read_dyn(sys.argv[1])
    print("nplacement %d vars %d clauses %d" % encode(n, pieces, pins, sys.argv[2]))
