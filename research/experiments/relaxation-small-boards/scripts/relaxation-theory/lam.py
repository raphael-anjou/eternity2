"""How much one side of an interior tile says about another side (numpy only; toys and the official pieces).

For the interior tiles of a set, C[a, b] counts the (tile, rotation) pairs showing colour a on one
side and colour b on the next side clockwise (adjacent) or on the facing side (opposite). The
normalised matrix D^-1/2 C D^-1/2 has top singular value 1; its second singular value lam is the
correlation carried across one cell. Random tiles give lam^2 of order (colours / interior tiles).
Usage: uv run --with numpy python -I -B lam.py
"""
import os, sys
import numpy as np
here = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(here, "..", "far-ideas-geometry"))
from common import gen


def lam(tiles):
    inner = [t for t in tiles if 0 not in t]
    cols = sorted({c for t in inner for c in t})
    ix = {c: i for i, c in enumerate(cols)}
    out = []
    for step in (1, 2):
        C = np.zeros((len(cols), len(cols)))
        for t in inner:
            for k in range(4):
                C[ix[t[k]], ix[t[(k + step) % 4]]] += 1
        d = C.sum(1)
        S = C / np.sqrt(np.outer(d, C.sum(0)))
        sv = np.linalg.svd(S, compute_uv=False)
        out.append(sv[1])
    return len(inner), len(cols), out


for (n, cf, ci) in ((4, 2, 4), (5, 3, 4), (6, 4, 5), (7, 4, 7), (8, 5, 8)):
    r = [lam(gen(n, cf, ci, s)) for s in range(10)]
    a = np.mean([x[2][0] for x in r]); o = np.mean([x[2][1] for x in r])
    print(f"toy {n}x{n} ({cf},{ci}): interior tiles {r[0][0]}, colours {ci}, colours/tiles {ci/r[0][0]:.3f}, "
          f"lam adjacent {a:.3f} (lam^2 {a*a:.3f}), lam opposite {o:.3f} (lam^2 {o*o:.3f}), mean of 10 seeds")
p = os.path.join(here, "..", "..", "..", "..", "experiments", "single-core-benchmark", "engine", "data", "puzzles", "size_16_official_eternity.csv")
L = open(p).read().split()
tiles = []
for line in L[1:]:
    f = line.split(",")
    t = tuple(0 if s == "1" * 16 else int(s, 2) for s in f[:4])
    tiles.append(t)
T, c, (a, o) = lam(tiles)
print(f"official 16x16: interior tiles {T}, colours on them {c}, colours/tiles {c/T:.3f}, lam adjacent {a:.3f} (lam^2 {a*a:.3f}), "
      f"lam opposite {o:.3f} (lam^2 {o*o:.3f})")
