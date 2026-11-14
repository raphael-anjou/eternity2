"""Second analysis script 8. Is the flux family COMPLETE among count invariants? Measured, not argued.

For one region (the empty cells below a prefix), sample many fillings with tiles reusable (any
of the 256). Record each filling as a count vector at four resolutions:
  tile | tile x cell colour (checkerboard) | tile x rotation | tile x rotation x cell colour.
D = lattice of differences. Z^n / D bounds from above the group of ALL linear and modular
relations between these counts that hold over every tiling of the region (Pak's group of tile
invariants, refined). It is compared with the relations the second analysis can name:
  rim-pattern counts (geometry); for each colour, east faces = west faces and south faces =
  north faces up to the frontier (flux), the same split by cell colour (checkerboard); the
  directed flow of frame colours along the rim (at the rotation-aware levels: the flux of the
  left rim column alone); the parity of the number of sides per colour.
"Unexplained" = independent relations in the sample that the named ones do not span. A sample
can only show too MANY relations (too few fillings), never too few.
Run: uv run --with python-flint python r8_count_groups.py [depth] [path] [oversample]
"""
import sys, os, json, random, time
from collections import Counter, defaultdict
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from flint import fmpz_mat, nmod_mat
from rcommon import load, rots, N
from rfill import fill
from r4_tile_count_invariants import gf2_rank

NC = 23


def canon_border(t):
    g = t.count(0)
    for rt in rots(t):
        if rt[0] == 0 and (g == 1 or rt[1] == 0):
            return rt
    return None


def features(P, level):
    """key(cell, tile, rt) and the named functionals as functions of a key."""
    def key(cell, i, rt):
        p = (cell[0] + cell[1]) % 2
        return {"tile": (i,), "tile x colour": (i, p), "tile x rot": (i, rt),
                "tile x rot x colour": (i, rt, p)}[level]
    Z = {}   # name -> function(key) -> int
    M2 = {}
    def tile_of(k): return P[k[0]]
    has_rot = "rot" in level
    has_col = "colour" in level
    par = (lambda k: k[-1]) if has_col else (lambda k: None)
    rt_of = (lambda k: k[1]) if has_rot else (lambda k: None)
    pars = (0, 1) if has_col else (None,)
    # class counts
    for z in (0, 1, 2):
        for p in pars:
            Z[("class", z, p)] = (lambda k, z=z, p=p: int(tile_of(k).count(0) == z and par(k) == p))
    if has_rot:
        pats = {tuple(x == 0 for x in rt) for t in P for rt in rots(t)}
        for g in pats:
            for p in pars:
                Z[("pattern", g, p)] = (lambda k, g=g, p=p: int(tuple(x == 0 for x in rt_of(k)) == g and par(k) == p))
        for c in range(1, NC):
            if has_col:
                for p in (0, 1):
                    Z[("EW", c, p)] = (lambda k, c=c, p=p: int(rt_of(k)[1] == c and par(k) == p) - int(rt_of(k)[3] == c and par(k) == 1 - p))
                    Z[("NS", c, p)] = (lambda k, c=c, p=p: int(rt_of(k)[2] == c and par(k) == p) - int(rt_of(k)[0] == c and par(k) == 1 - p))
            else:
                Z[("EW", c)] = (lambda k, c=c: int(rt_of(k)[1] == c) - int(rt_of(k)[3] == c))
                Z[("NS", c)] = (lambda k, c=c: int(rt_of(k)[2] == c) - int(rt_of(k)[0] == c))
        # flux along the LEFT rim column alone (the joints between left-edge cells); the right
        # column then follows from the total. Added after the first run showed 4 relations
        # (8 with cell colours) that the list above did not span.
        LEFTP = (False, False, False, True); BLP = (False, False, True, True)
        def patt(k): return tuple(x == 0 for x in rt_of(k))
        for c in range(1, 6):
            if has_col:
                for p in (0, 1):
                    Z[("NSleft", c, p)] = (lambda k, c=c, p=p: int(patt(k) == LEFTP and rt_of(k)[2] == c and par(k) == p)
                                           - int(patt(k) in (LEFTP, BLP) and rt_of(k)[0] == c and par(k) == 1 - p))
            else:
                Z[("NSleft", c)] = (lambda k, c=c: int(patt(k) == LEFTP and rt_of(k)[2] == c)
                                    - int(patt(k) in (LEFTP, BLP) and rt_of(k)[0] == c))
    else:
        for c in range(1, 6):
            def flow(k, c=c, p=None):
                t = tile_of(k); g = t.count(0)
                if not g:
                    return 0, 0
                rt = canon_border(t)
                return int((rt[1] if g == 1 else rt[2]) == c), int(rt[3] == c)
            if has_col:
                for p in (0, 1):
                    Z[("ring", c, p)] = (lambda k, c=c, p=p, flow=flow: flow(k)[0] * int(par(k) == p) - flow(k)[1] * int(par(k) == 1 - p))
            else:
                Z[("ring", c)] = (lambda k, c=c, flow=flow: flow(k)[0] - flow(k)[1])
        if has_col:
            for c in range(1, NC):
                Z[("bw", c)] = (lambda k, c=c: tile_of(k).count(c) * (1 if par(k) == 0 else -1))
    for c in range(NC):
        M2[("parity", c)] = (lambda k, c=c: tile_of(k).count(c) % 2)
    return key, Z, M2


def analyse(P, pref, level, fillings):
    key, Z, M2 = features(P, level)
    vec = []
    coords = {}
    for pl in fillings:
        v = Counter()
        for cell, i, rt in pl:
            k = key(cell, i, rt)
            coords.setdefault(k, len(coords))
            v[coords[k]] += 1
        vec.append(v)
    n = len(coords)
    keys = sorted(coords, key=coords.get)
    Zm = [[f(k) for k in keys] for f in Z.values()]
    M2m = [[f(k) for k in keys] for f in M2.values()]

    def diff_rows(m):
        rows = []
        for v in vec[1:m]:
            r = [0] * n
            for j, x in v.items(): r[j] += x
            for j, x in vec[0].items(): r[j] -= x
            rows.append(r)
        return rows

    def rk(rows, p):
        return nmod_mat(len(rows), len(rows[0]), [x % p for r in rows for x in r], p).rank()
    rows = diff_rows(len(vec))
    BIG = 2147483629
    rq80 = rk(diff_rows(int(len(vec) * 0.8)), BIG)
    rq = rk(rows, BIG)
    okZ = all(sum(a * b for a, b in zip(r, d)) == 0 for r in Zm for d in rows[:150])
    ok2 = all(sum(a * b for a, b in zip(r, d)) % 2 == 0 for r in M2m for d in rows[:150])
    res = dict(level=level, n=n, fillings=len(vec), rank_at_80pct=rq80, rank=rq, free=n - rq,
               named_hold=(okZ and ok2), named_rational_rank=rk(Zm, BIG),
               unexplained_free=n - rq - rk(Zm, BIG))
    # relations mod p: Hom(Z^n / D, Z/p) has dimension n - rank_p(D); the named ones span
    # rank_p(named) of it (mod 2 the colour parities are added to the named set)
    for p in (2, 3, 5, 7):
        rp = rk(rows, p)
        named_p = rk(Zm + (M2m if p == 2 else []), p)
        res[f"mod{p}"] = dict(relations=n - rp, named=named_p, unexplained=n - rp - named_p)
    if n <= 600:
        S = fmpz_mat(rows).snf()
        d = [abs(int(S[i, i])) for i in range(min(S.nrows(), S.ncols()))]
        res["exact_torsion"] = dict(Counter(x for x in d if x > 1))
    return res


if __name__ == "__main__":
    depth = int(sys.argv[1]) if len(sys.argv) > 1 else 16
    path = int(sys.argv[2]) if len(sys.argv) > 2 else 0
    over = float(sys.argv[3]) if len(sys.argv) > 3 else 1.35
    levels = sys.argv[4].split(",") if len(sys.argv) > 4 else ["tile", "tile x colour", "tile x rot", "tile x rot x colour"]
    P, clues = load()
    allp = json.load(open(os.path.join(HERE, "prefixes_noclue.json")))
    pref = next([(i, tuple(rt)) for i, rt in pf] for s, d, pf in allp if s == path and d == depth)
    t0 = time.time()
    need = {"tile": 256, "tile x colour": 512, "tile x rot": 1004, "tile x rot x colour": 2008}
    M = int(max(need[l] for l in levels) * over) + 50
    fills = []
    seed = 0
    while len(fills) < M:
        st, _, pl = fill(P, pref, range(len(P)), random.Random(600000 + seed), 400000)
        seed += 1
        if st == "filled":
            fills.append(pl)
    print(f"path {path}, depth {depth}: {len(fills)} fillings with reuse, {time.time() - t0:.0f}s")
    for level in levels:
        res = analyse(P, pref, level, fills[:int(need[level] * over) + 50])
        print(res, f"{time.time() - t0:.0f}s", flush=True)
