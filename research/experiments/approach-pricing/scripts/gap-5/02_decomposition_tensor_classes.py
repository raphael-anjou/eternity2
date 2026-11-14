"""Gap 5, script 2. On the official set and on painted boards with the same colour counts:
(a) which coordinates carry the one colour-dependent invariant of the 2 x 2 cover;
(b) sub-family decomposition: for every subset S of the 17 interior colours, the number of
    interior tiles painted only from S and only from its complement (a zone structure needs
    both large);
(c) the interior tile tensor T[t,r,b,l]: ranks of its three flattenings, connected
    components of the (t,r | b,l) bipartite graph;
(d) how many bits the flux and checkerboard conservation laws remove from the space of
    coarse classes (cell parity, rotation) of the 196 interior tiles (Gaussian local limit).
Run: uv run --with numpy python 02_decomposition_tensor_classes.py
"""
import random, math
from collections import Counter
import numpy as np
from e2common import load, rots, paint, official_pools, INNER, FRAME
import importlib
L1 = importlib.import_module("01_lattice_and_automorphisms")

NI = 17


def inner_tiles(T):
    return [t for t in T if 0 not in t]


def decomposition(T):
    it = inner_tiles(T)
    masks = np.array([sum(1 << (c - 6) for c in set(t)) for t in it], dtype=np.int64)
    S = np.arange(1, 1 << (NI - 1), dtype=np.int64)          # up to complement
    full = (1 << NI) - 1
    pure_in = np.zeros(len(S), dtype=np.int32)
    pure_out = np.zeros(len(S), dtype=np.int32)
    for m in masks:
        pure_in += ((m & ~S) == 0)
        pure_out += ((m & S) == 0)
    best = np.minimum(pure_in, pure_out)
    i = int(best.argmax())
    size = bin(int(S[i])).count("1")
    # best split for balanced palettes only (8 or 9 colours)
    pc = np.array([bin(int(s)).count("1") for s in S])
    bal = best[(pc == 8) | (pc == 9)].max()
    return int(best[i]), size, int(pure_in[i]), int(pure_out[i]), int(bal)


def tensor(T):
    it = inner_tiles(T)
    A = np.zeros((NI,) * 4)
    for t in it:
        for r in set(rots(t)):
            A[tuple(c - 6 for c in r)] += 1
    f1 = A.reshape(NI * NI, NI * NI)                           # (t,r | b,l)
    f2 = A.transpose(0, 2, 1, 3).reshape(NI * NI, NI * NI)     # (t,b | r,l)
    f3 = A.reshape(NI, NI ** 3)                                # (t | r,b,l)
    rk = [int(np.linalg.matrix_rank(f)) for f in (f1, f2, f3)]
    # components of the bipartite graph of f1 among non-isolated vertices
    par = list(range(2 * NI * NI))
    def find(x):
        while par[x] != x:
            par[x] = par[par[x]]; x = par[x]
        return x
    rows, cols = np.nonzero(f1)
    for a, b in zip(rows, cols):
        par[find(a)] = find(NI * NI + b)
    comp = len({find(x) for x in set(rows) | {NI * NI + b for b in cols}})
    return rk + [int((f1.sum(1) > 0).sum()), comp]


def class_bits(T):
    """Coarse class of an interior tile: (parity p in {0,1}, rotation). 8 classes, uniform.
    Laws, per interior colour c: E_c(B) - W_c(W), E_c(W) - W_c(B), S_c(B) - N_c(W),
    S_c(W) - N_c(B) are constants. Returns bits removed = -log2 P(all hold), Gaussian local
    limit on the integer lattice, for (i) flux only (E-W, S-N per colour, parity ignored),
    (ii) flux with checkerboard."""
    it = inner_tiles(T)
    out = []
    for mode in ("flux", "flux+checkerboard"):
        d = 2 * NI if mode == "flux" else 4 * NI
        cov = np.zeros((d, d))
        for t in it:
            vs = []
            for k in range(4):
                top, right, bot, left = rots(t)[k]
                for p in (0, 1):
                    v = np.zeros(d)
                    if mode == "flux":
                        v[right - 6] += 1; v[left - 6] -= 1
                        v[NI + bot - 6] += 1; v[NI + top - 6] -= 1
                    else:
                        v[(0 + p) * NI + right - 6] += 1       # E on parity p
                        v[(0 + 1 - p) * NI + left - 6] -= 1    # W on parity p pairs with E on 1-p
                        v[(2 + p) * NI + bot - 6] += 1
                        v[(2 + 1 - p) * NI + top - 6] -= 1
                    vs.append(v)
            vs = np.array(vs)
            cov += np.cov(vs.T, bias=True)
        w = np.linalg.eigvalsh(cov)
        w = w[w > 1e-9]
        bits = 0.5 * np.sum(np.log2(2 * math.pi * w))
        out.append((mode, len(w), float(bits)))
    return out


if __name__ == "__main__":
    P, _ = load()
    print("== (a) support of the colour-dependent invariant, k = 2, official set ==")
    G = L1.gens(P, 2)
    dim = G.shape[1]
    def corank_with_zeroed(cols):
        E = np.zeros((len(cols), dim), dtype=np.int64)
        for i, c in enumerate(cols):
            E[i, c] = 1
        return dim - L1.rank_mod(np.vstack([G, E]), L1.BIG)
    frame_cols = [i for i in range(dim) if i % 23 in FRAME]
    other_cols = [i for i in range(dim) if i % 23 not in FRAME]
    print("corank:", dim - L1.rank_mod(G, L1.BIG),
          "| annihilator vanishing on frame-colour coordinates:", corank_with_zeroed(frame_cols),
          "| annihilator supported on frame-colour coordinates only:", corank_with_zeroed(other_cols))
    Gi = L1.gens([t for t in P if 0 not in t], 2)
    print("interior tiles alone, k=2: corank", dim - L1.rank_mod(Gi, L1.BIG),
          "(coordinates of grey and frame colours are free there: 2*4*6 = 48 of it)")

    print("== (b) (c) (d) official ==")
    print("decomposition (best min(pure_in, pure_out), |S|, pure_in, pure_out, best balanced):",
          decomposition(P))
    print("tensor [rank(tr|bl), rank(tb|rl), rank(t|rbl), nonzero rows of (tr|bl), components]:",
          tensor(P))
    for m in class_bits(P):
        print("class bits removed:", m, "of", 196 * 3, "bits of coarse class")

    print("== null: painted boards ==")
    rng = random.Random(2026111306)
    ring_pool, inner_pool = official_pools(P)
    D, Tn, C = [], [], []
    for b in range(40):
        T = [t for row in paint(rng, ring_pool, inner_pool) for t in row]
        D.append(decomposition(T)); Tn.append(tensor(T)); C.append([m[2] for m in class_bits(T)])
    D = np.array(D); Tn = np.array(Tn); C = np.array(C)
    def rng_(a):
        return f"min {a.min():.1f} mean {a.mean():.2f} max {a.max():.1f}"
    print("decomposition best:", rng_(D[:, 0]), "| best balanced:", rng_(D[:, 4]))
    for j, nm in enumerate(["rank(tr|bl)", "rank(tb|rl)", "rank(t|rbl)", "nonzero rows", "components"]):
        print(nm, rng_(Tn[:, j]))
    print("class bits flux:", rng_(C[:, 0]), "| flux+checkerboard:", rng_(C[:, 1]))
