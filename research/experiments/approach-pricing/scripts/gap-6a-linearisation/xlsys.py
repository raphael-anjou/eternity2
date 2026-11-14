"""Board -> 0/1 placement system for xl.cpp (TOY BOARDS; not Eternity II progress).

Variables: one per rim-consistent placement (cell, tile, rotation): grey exactly on the rim sides.
Pinning a cell to placement (tile i, sides p) is substitution: every variable of that cell and
every variable of tile i is removed (the pinned one is 1, the others 0), and so is every variable
of a neighbouring cell whose facing side differs from p (its product with the pinned variable is
a forbidden pair). Nothing else is propagated.

Equations after the pins:
  one per remaining cell:  sum of its variables = 1
  one per remaining tile:  sum of its variables = 1
  forbidden pairs, encoding "minimal":  two placements in adjacent cells with different facing colours
  encoding "enriched" adds:             two placements in the same cell, two placements of the same tile
"""
from boards import placements, pin_tile

OPP = {0: 2, 1: 3, 2: 0, 3: 1}
def neighbours(n, x):
    r, c = divmod(x, n)
    if r > 0: yield x - n, 0
    if c < n - 1: yield x + 1, 1
    if r < n - 1: yield x + n, 2
    if c > 0: yield x - 1, 3

def build(n, tiles, sol, pins, enriched):
    """sol: list per cell of (tile, sides). pins: cells fixed to their value in sol."""
    pl = placements(n, tiles)
    full = len(pl)
    for x in pins:
        i, p = sol[x]
        keep = []
        for (y, j, q) in pl:
            if y == x or j == i: continue
            bad = False
            for (z, side) in neighbours(n, x):
                if z == y and q[OPP[side]] != p[side]: bad = True
            if not bad: keep.append((y, j, q))
        pl = keep
    idx = {v: k for k, v in enumerate(pl)}
    cells = sorted({v[0] for v in pl}); tls = sorted({v[1] for v in pl})
    want_cells = [x for x in range(n * n) if x not in pins]
    want_tiles = [i for i in range(n * n) if i not in {sol[x][0] for x in pins}]
    eqs = [[k for k, v in enumerate(pl) if v[0] == x] for x in want_cells] + \
          [[k for k, v in enumerate(pl) if v[1] == i] for i in want_tiles]
    F = set()
    for a, (x, i, p) in enumerate(pl):
        for b in range(a + 1, len(pl)):
            y, j, q = pl[b]
            if enriched and (x == y or i == j): F.add((a, b)); continue
            for (z, side) in neighbours(n, x):
                if z == y and q[OPP[side]] != p[side]: F.add((a, b))
    solvec = [1 if sol[v[0]] == (v[1], v[2]) else 0 for v in pl]
    return dict(vars=pl, eqs=eqs, F=sorted(F), sol=solvec, full_vars=full, side=n)

def write(sysd, path):
    with open(path, "w") as f:
        f.write(f"{len(sysd['vars'])} {len(sysd['eqs'])}\n")
        f.write(" ".join(map(str, sysd["sol"])) + "\n")
        for e in sysd["eqs"]: f.write(f"{len(e)} " + " ".join(map(str, e)) + "\n")
        f.write(f"{len(sysd['F'])}\n")
        for a, b in sysd["F"]: f.write(f"{a} {b}\n")
        f.write(f"{sysd['side']}\n" + " ".join(str(v[0]) for v in sysd["vars"]) + "\n")   # cell of each variable

def solvec_of(sysd, sol):
    return [1 if sol[v[0]] == (v[1], v[2]) else 0 for v in sysd["vars"]]
