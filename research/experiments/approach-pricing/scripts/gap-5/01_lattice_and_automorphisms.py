"""Gap 5, script 1. Exact structure of the official tile set that is not a statistic:
(a) colour-permutation automorphisms (with and without mirror), by colour refinement;
(b) the abelianised tile group: Z^(2*k*k*23) modulo the lattice L_k spanned by the boundary
    vectors of every tile in every rotation at every cell class of the k x k torus cover
    (k = 1: plain flux; k = 2 contains the checkerboard). Free rank = number of independent
    linear boundary invariants; torsion = modular ones. Compared with painted boards.
Run: uv run --with numpy --with sympy python 01_lattice_and_automorphisms.py
"""
import random, sys, json
from collections import Counter
import numpy as np
from e2common import load, rots, canon, paint, official_pools, N

NC = 23  # colours 0..22, 0 = grey


# ---------- (a) automorphisms ----------
def refine(tiles, tag=""):
    """Colour refinement. Returns dict colour -> stable signature id (comparable across sets
    refined together). tiles: list of 4-tuples."""
    cols = sorted({c for t in tiles for c in t})
    cls = {c: 0 for c in cols}
    for _ in range(50):
        sig = {}
        for c in cols:
            bag = []
            for t in tiles:
                if c in t:
                    # canonical rotation of the tile seen as (class, is-this-colour) pairs
                    bag.append(min(tuple((cls[x], x == c) for x in r) for r in rots(t)))
            sig[c] = (cls[c], tuple(sorted(bag)))
        ids = {s: i for i, s in enumerate(sorted(set(sig.values())))}
        new = {c: ids[sig[c]] for c in cols}
        if len(set(new.values())) == len(set(cls.values())):
            return sig
        cls = new
    return sig


def mirror(t):
    return (t[0], t[3], t[2], t[1])


def automorphism_report(tiles):
    s = refine(tiles)
    nclass = len(set(s.values()))
    sm = refine([mirror(t) for t in tiles])
    # a mirror automorphism needs the same multiset of signatures; signatures embed class
    # ids that are only comparable through the sorted structure, so compare sorted lists
    same = sorted(map(repr, s.values())) == sorted(map(repr, sm.values()))
    return nclass, len(s), same


# ---------- (b) lattice ----------
def gens(tiles, k):
    """Boundary vectors. Coordinates: (type h=0/v=1, a, b, colour), a,b in Z_k."""
    def idx(ty, a, b, c):
        return ((ty * k + a % k) * k + b % k) * NC + c
    out = set()
    for t in tiles:
        for (top, right, bot, left) in set(rots(t)):
            for a in range(k):
                for b in range(k):
                    v = [0] * (2 * k * k * NC)
                    v[idx(0, a, b, top)] += 1
                    v[idx(0, a + 1, b, bot)] -= 1
                    v[idx(1, a, b + 1, right)] += 1
                    v[idx(1, a, b, left)] -= 1
                    out.add(tuple(v))
    return np.array(sorted(out), dtype=np.int64)


def rank_mod(M, p):
    M = M.copy() % p
    r = 0
    rows, cols = M.shape
    for c in range(cols):
        piv = np.nonzero(M[r:, c])[0]
        if len(piv) == 0:
            continue
        i = r + piv[0]
        if i != r:
            M[[r, i]] = M[[i, r]]
        inv = pow(int(M[r, c]), -1, p)
        M[r] = (M[r] * inv) % p
        nz = np.nonzero(M[:, c])[0]
        nz = nz[nz != r]
        M[nz] = (M[nz] - np.outer(M[nz, c], M[r])) % p
        r += 1
        if r == rows:
            break
    return r


BIG = 2147483629
PRIMES = [2, 3, 5, 7, 11, 13]


def lattice_report(tiles, ks=(1, 2)):
    rep = {}
    for k in ks:
        G = gens(tiles, k)
        rq = rank_mod(G, BIG)
        rp = {p: rank_mod(G, p) for p in PRIMES}
        rep[k] = dict(dim=G.shape[1], gens=G.shape[0], rankQ=rq, corank=G.shape[1] - rq,
                      geometric=k * k + 1,
                      torsion_rank={p: rq - rp[p] for p in PRIMES})
    return rep


def exact_snf_k1(tiles):
    from sympy import Matrix, ZZ
    from sympy.matrices.normalforms import smith_normal_form
    G = gens(tiles, 1)
    M = Matrix(G.tolist())
    # reduce to a row basis first (rref over Q gives the row space; for SNF we need the
    # lattice itself, so use integer Hermite reduction by hand)
    rows = [list(map(int, r)) for r in G.tolist()]
    basis = []
    ncol = len(rows[0])
    for c in range(ncol):
        cand = [r for r in rows if r[c] != 0]
        rest = [r for r in rows if r[c] == 0]
        while len(cand) > 1:
            cand.sort(key=lambda r: abs(r[c]))
            p = cand[0]
            nxt = [p]
            for r in cand[1:]:
                q = r[c] // p[c]
                r2 = [x - q * y for x, y in zip(r, p)]
                if r2[c] != 0:
                    nxt.append(r2)
                elif any(r2):
                    rest.append(r2)
            cand = nxt
        if cand:
            basis.append(cand[0])
        rows = rest
    S = smith_normal_form(Matrix(basis), domain=ZZ)
    d = [abs(int(S[i, i])) for i in range(min(S.shape))]
    null = M.nullspace()
    return len(basis), Counter(d), null


def describe_null(null):
    out = []
    for v in null:
        v = v * v.applyfunc(lambda x: x.q).__iter__().__next__() if False else v
        h = [v[c] for c in range(NC)]
        vv = [v[NC + c] for c in range(NC)]
        out.append(("h:" + " ".join(str(x) for x in h), "v:" + " ".join(str(x) for x in vv)))
    return out


if __name__ == "__main__":
    P, hints = load()
    print("== official set ==")
    nclass, ncol, mir = automorphism_report(P)
    print(f"colour refinement: {nclass} classes for {ncol} colours (discrete = no non-trivial "
          f"colour automorphism); mirror image has the same signature multiset: {mir}")
    # mirror automorphism needs every tile's mirror (after a colour permutation) in the set;
    # with discrete refinement the permutation, if any, is forced. Direct test with identity:
    S = Counter(canon(t) for t in P)
    Sm = Counter(canon(mirror(t)) for t in P)
    print("tiles whose mirror image (same colours) is in the set:",
          sum(1 for t in P if canon(mirror(t)) in S and canon(mirror(t)) != canon(t)),
          "; achiral tiles:", sum(1 for t in P if canon(mirror(t)) == canon(t)))
    rep = lattice_report(P, ks=(1, 2, 4))
    for k, r in rep.items():
        print(f"k={k}: {r}")
    nb, divs, null = exact_snf_k1(P)
    print(f"k=1 exact: lattice basis rows {nb}; elementary divisors {dict(divs)}")
    print("k=1 rational annihilator basis (colour 0..22; h block then v block):")
    for h, v in describe_null(null):
        print("  ", h); print("  ", v)

    print("== painted boards, official colour counts (null) ==")
    rng = random.Random(2026111305)
    ring_pool, inner_pool = official_pools(P)
    agg = Counter()
    nb_boards = 30
    for b in range(nb_boards):
        board = paint(rng, ring_pool, inner_pool)
        T = [t for row in board for t in row]
        nclass, ncol, mir = automorphism_report(T)
        rep = lattice_report(T, ks=(1, 2))
        key = (nclass, mir,
               rep[1]["corank"], tuple(rep[1]["torsion_rank"].values()),
               rep[2]["corank"], tuple(rep[2]["torsion_rank"].values()))
        agg[key] += 1
    print("(colour classes, mirror-signature-equal, corank k=1, torsion ranks k=1 at "
          f"{PRIMES}, corank k=2, torsion ranks k=2) -> boards of {nb_boards}")
    for k, v in agg.items():
        print("  ", k, v)
