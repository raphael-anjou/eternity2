"""TOY BOARDS ONLY. Exports, from the NumPy low-rank code (bm.py, board.py, common.py copied
unchanged from ../../lowrank-relaxation/), everything the Rust port reads:

  boards/<name>/meta.txt       n cf ci seed P N T K F nsols
  boards/<name>/place.txt      P lines: cell tile rot N E S W (placement list)
  boards/<name>/nb.txt         N lines: neighbour cell N E S W (-1 off board)
  boards/<name>/cells.txt      N lines: placements of each cell   (linear constraint E1 rows)
  boards/<name>/tiles.txt      T lines: placements of each tile   (linear constraint E2 rows)
  boards/<name>/conflicts.txt  K lines: i j, i < j (conflict pairs, E6)
  boards/<name>/free.txt       F lines: i j, i < j (free pairs, I2)
  boards/<name>/sols.txt       one line per enumerated solution: N placement indices
  boards/<name>/init/r<r>_s<s>.bin   U0 = init_U(B, r, default_rng(1000 r + s)), P*r f64 LE,
                               the same start run_unit.py uses for (r, start)
  boards/<name>/equiv/<k>.*    random W, multipliers, rho and NumPy's f, dF/dV, dF/dW (vec=True)

Names: b5_s<seed> (5x5, frame 3, inner 4), b5_broken (run_unit.py's broken control on seed 0),
b6_s<seed> and b6_broken (6x6, frame 4, inner 5) when "6" is passed, b7_s<seed> and b7_broken
(7x7, frame 4, inner 7: the convention of scripts/lowrank-relaxation/scale.py and construct.py)
when "7" is passed. Growth-exponent copy: ranks 16 and 32 only, no equivalence states.
Usage: uv run --with numpy --with scipy python -I py/export.py [5] [6] [7]"""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import common  # noqa: F401  (local copy, loaded before board.py, as run_unit.py does)
from common import gen, enumerate_all
import numpy as np
import scipy.sparse as sp
from board import Board
from bm import Structure, SphereAL, V_of_U, init_U

ROOT = os.path.join(HERE, "..", "boards")
RANKS = (16, 32)  # growth-exponent copy: the best configuration (16) and adjustment 1 (32) only
STARTS = range(10)


def broken_tiles(n, cf, ci, seed):
    """run_unit.py's mode "broken", verbatim."""
    tiles = gen(n, cf, ci, seed)
    for i, t in enumerate(tiles):
        if 0 not in t:
            nt = list(t); nt[0] = cf + 1 if t[0] != cf + 1 else cf + 2; nt = tuple(nt)
            canon = {min(u[k:] + u[:k] for k in range(4)) for j, u in enumerate(tiles) if j != i}
            if min(nt[k:] + nt[:k] for k in range(4)) in canon:
                continue
            tt = tiles[:i] + [nt] + tiles[i + 1:]
            if len(enumerate_all(n, tt)["sols"]) == 0:
                return tt
    raise RuntimeError


def lines(path, rows):
    with open(path, "w") as f:
        for r in rows:
            f.write(" ".join(str(int(a)) for a in r) + "\n")


def export(name, B, equiv=True):
    d = os.path.join(ROOT, name); os.makedirs(os.path.join(d, "init"), exist_ok=True)
    S = Structure(B)
    P = B.P
    free = np.ones((P, P), bool)
    free[S.ci, S.cj] = False
    iu, ju = np.triu_indices(P, 1)
    keep = free[iu, ju]
    fi, fj = iu[keep], ju[keep]
    assert len(fi) == S.F
    with open(os.path.join(d, "meta.txt"), "w") as f:
        f.write(f"{B.n} {B.cf} {B.ci} {B.seed} {P} {B.N} {B.T} {S.K} {S.F} {len(B.sol_idx)}\n")
    lines(os.path.join(d, "place.txt"), [[B.cellOf[p], B.tileOf[p], B.rotOf[p], *B.cols[p]] for p in range(P)])
    lines(os.path.join(d, "nb.txt"), B.nb.tolist())
    lines(os.path.join(d, "cells.txt"), [np.where(B.cellOf == x)[0] for x in range(B.N)])
    lines(os.path.join(d, "tiles.txt"), [np.where(B.tileOf == t)[0] for t in range(B.T)])
    lines(os.path.join(d, "conflicts.txt"), zip(S.ci, S.cj))
    lines(os.path.join(d, "free.txt"), zip(fi, fj))
    lines(os.path.join(d, "sols.txt"), B.sol_idx)
    for r in RANKS:
        for s in STARTS:
            init_U(B, r, np.random.default_rng(1000 * r + s)).astype("<f8").tofile(os.path.join(d, "init", f"r{r}_s{s}.bin"))
    if not equiv:
        return
    ed = os.path.join(d, "equiv"); os.makedirs(ed, exist_ok=True)
    rng = np.random.default_rng(12345)
    for k, r in enumerate((4, 8, 16, 32, 64)):
        al = SphereAL(S, r, rho=float(10 ** rng.uniform(0, 4)), vec=True)
        al.lc = rng.standard_normal((B.N, r)); al.lt = rng.standard_normal((B.T, r)); al.lz = rng.standard_normal(S.K)
        # nonnegative I2 multipliers on a random 30 percent of the free pairs
        on = rng.random(S.F) < 0.3
        mv = np.abs(rng.standard_normal(on.sum()))
        M = sp.coo_matrix((mv, (fi[on], fj[on])), shape=(P, P)).tocsr()
        al.M2 = (M + M.T).tocsr()
        W = rng.standard_normal((P, r)) * rng.uniform(0.5, 2.0, (P, 1))
        f, gW = al.fg(W.ravel())
        nrm = np.linalg.norm(W, axis=1, keepdims=True)
        V = V_of_U(W / nrm)
        al._mu = None
        fV, gV = al.fV(V)
        assert abs(fV - f) <= 1e-12 * abs(f)
        with open(os.path.join(ed, f"{k}.txt"), "w") as fo:
            fo.write(f"{r} {float(al.rho)!r} {int(on.sum())} {float(f)!r}\n")
        W.astype("<f8").tofile(os.path.join(ed, f"{k}_W.bin"))
        al.lc.astype("<f8").tofile(os.path.join(ed, f"{k}_lc.bin"))
        al.lt.astype("<f8").tofile(os.path.join(ed, f"{k}_lt.bin"))
        al.lz.astype("<f8").tofile(os.path.join(ed, f"{k}_lz.bin"))
        np.stack([fi[on].astype(float), fj[on].astype(float), mv], 1).astype("<f8").tofile(os.path.join(ed, f"{k}_mu.bin"))
        gV.astype("<f8").tofile(os.path.join(ed, f"{k}_gV.bin"))
        gW.reshape(P, r).astype("<f8").tofile(os.path.join(ed, f"{k}_gW.bin"))
    print(name, "P", P, "K", S.K, "F", S.F, "sols", len(B.sol_idx), flush=True)


if __name__ == "__main__":
    args = sys.argv[1:] or ["5"]
    if "5" in args:
        for seed in range(5):
            export(f"b5_s{seed}", Board(5, 3, 4, seed), equiv=False)
        export("b5_broken", Board(5, 3, 4, 0, tiles=broken_tiles(5, 3, 4, 0)), equiv=False)
    if "6" in args:
        for seed in range(5):
            export(f"b6_s{seed}", Board(6, 4, 5, seed), equiv=False)
        export("b6_broken", Board(6, 4, 5, 0, tiles=broken_tiles(6, 4, 5, 0)), equiv=False)
    if "7" in args:
        for seed in range(5):
            export(f"b7_s{seed}", Board(7, 4, 7, seed), equiv=False)
        export("b7_broken", Board(7, 4, 7, 0, tiles=broken_tiles(7, 4, 7, 0)), equiv=False)
