"""Second analysis script 4. Tile invariants in Pak's sense: linear or modular relations between the
NUMBERS of copies of each tile, constant over all tilings of one region. The memo's D3 says a
boundary invariant cannot count uses; tile invariants do, and they come from the same theory
(Conway and Lagarias; Pak's survey, opened by the second analysis).

For a prefix with empty region R: let A0, A1, ... be fillings of R with tiles REUSED
(version A: any of the 256 tiles; version B: only the unplaced tiles), and a(Aj) in Z^256 the
count vector (one coordinate per tile, rotations merged). D(R) = lattice spanned by the
differences a(Aj) - a(A0). Every tile invariant of R, of any origin, is a homomorphism of Z^256
that vanishes on D(R). An each-tile-once completion has count vector u = indicator of the
unplaced tiles. So if u - a(A0) lies in D(R), NO tile invariant can refute the prefix. A sampled
D is a sublattice of the true D(R), so membership in the sampled lattice is a proof.
Also compares D with the lattice of the obvious invariants: number of corner, edge and interior
tiles, and for each colour the parity of the number of sides (memo D1).
Run: uv run --with python-flint python r4_tile_count_invariants.py [paths_end] [fillings] [A|B] [paths_start]
"""
import sys, os, json, random, time
from collections import Counter, defaultdict
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from flint import fmpz_mat
from rcommon import load, N
from rfill import fill, check_filling


def snf_summary(rows):
    if not rows:
        return 0, Counter()
    S = fmpz_mat(rows).snf()
    d = [abs(int(S[i, i])) for i in range(min(S.nrows(), S.ncols()))]
    d = [x for x in d if x]
    return len(d), Counter(x for x in d if x != 1)


def named_invariants(P):
    """The invariants the second analysis can name. Z-valued: number of corner, edge, interior tiles;
    for each frame colour c, (border tiles leaving through c) - (border tiles entering through
    c), walking the rim clockwise (the rim is one path, so this is fixed by its two ends).
    Mod 2: for each of the 23 colours, the number of sides of that colour."""
    Z = []
    for z in (2, 1, 0):
        Z.append([1 if t.count(0) == z else 0 for t in P])
    for c in range(1, 6):
        row = []
        for t in P:
            g = t.count(0)
            if g == 0:
                row.append(0); continue
            # rotate so that grey is on top (and on the right too for a corner)
            from rcommon import rots
            for rt in rots(t):
                if rt[0] == 0 and (g == 1 or rt[1] == 0):
                    break
            ent = rt[3]; out = rt[1] if g == 1 else rt[2]
            row.append((1 if out == c else 0) - (1 if ent == c else 0))
        Z.append(row)
    M2 = [[t.count(c) % 2 for t in P] for c in range(23)]
    return Z, M2


def gf2_rank(rows):
    piv = {}
    for r in rows:
        v = 0
        for i, x in enumerate(r):
            if x % 2:
                v |= 1 << i
        while v:
            b = v.bit_length() - 1
            if b in piv:
                v ^= piv[b]
            else:
                piv[b] = v; break
    return len(piv)


def named_report(P, D, support):
    Z, M2 = named_invariants(P)
    sup = set(support)
    Zs = [[x if i in sup else 0 for i, x in enumerate(r)] for r in Z]
    ok_Z = all(sum(a * b for a, b in zip(r, d)) == 0 for r in Z for d in D)
    ok_2 = all(sum(a * b for a, b in zip(r, d)) % 2 == 0 for r in M2 for d in D)
    cols = sorted(sup)
    rkZ = fmpz_mat([[r[i] for i in cols] for r in Z]).rank()
    rk2 = gf2_rank([[r[i] for i in cols] for r in Z + M2])
    return ok_Z, ok_2, rkZ, rk2


def run_prefix(P, pref, version, M, seed, pins=None, cap=400000):
    placed = {i for i, _ in pref}
    unplaced = [i for i in range(len(P)) if i not in placed]
    allowed = list(range(len(P))) if version == "A" else unplaced
    if pins and version == "A":
        pass
    vecs = []
    tries = 0
    while len(vecs) < M and tries < M + 20:
        st, nodes, pl = fill(P, pref, allowed, random.Random(seed * 100003 + tries), cap, pins=pins)
        tries += 1
        if st != "filled":
            if st == "none":
                return dict(status="no reuse filling exists")
            continue
        if len(vecs) < 3:
            check_filling(P, pref, pl, set(allowed) | ({i for i, _ in pins.values()} if pins else set()))
        a = [0] * len(P)
        for _, i, _ in pl:
            a[i] += 1
        vecs.append(a)
    if len(vecs) < 2:
        return dict(status=f"fewer than 2 fillings in {tries} tries")
    a0 = vecs[0]
    D = [[x - y for x, y in zip(v, a0)] for v in vecs[1:]]
    u = [1 if i in set(unplaced) else 0 for i in range(len(P))]
    t = [x - y for x, y in zip(u, a0)]
    rk, tors = snf_summary(D)
    rk2, tors2 = snf_summary(D + [t])
    member = (rk == rk2 and tors == tors2)
    support = sorted({i for v in vecs for i, x in enumerate(v) if x})
    okZ, ok2, rkZ, rk2 = named_report(P, D, support)
    # Z^n / D = Z^(n - rank) + torsion. If the torsion is (Z/2)^j, Hom(Z^n/D, Z/2) has dimension
    # n - rank + j. The named invariants explain everything when they vanish on D, their rational
    # rank is n - rank, and their rank mod 2 is n - rank + j.
    n = len(support)
    j2 = tors.get(2, 0)
    explained = (okZ and ok2 and set(tors) <= {2} and rkZ == n - rk and rk2 == n - rk + j2)
    return dict(status="ok", fillings=len(vecs), rank=rk, torsion=dict(tors), member=member,
                tiles_seen=n, free=n - rk, named=(rkZ, rk2), explained=explained)


if __name__ == "__main__":
    paths = int(sys.argv[1]) if len(sys.argv) > 1 else 3
    first = int(sys.argv[4]) if len(sys.argv) > 4 else 0
    M = int(sys.argv[2]) if len(sys.argv) > 2 else 400
    version = sys.argv[3] if len(sys.argv) > 3 else "A"
    P, clues = load()
    allp = json.load(open(os.path.join(HERE, "prefixes_noclue.json")))
    t0 = time.time()
    print(f"version {version}: fillings with reuse of "
          f"{'any of the 256 tiles' if version == 'A' else 'the unplaced tiles only'}; "
          f"{M} fillings per prefix; paths {first}..{paths - 1}")
    print("path depth | fillings, tiles seen n, rank of D, torsion of Z^n/D | named invariants: "
          "rational rank, rank mod 2 | all of Z^n/D explained by named ones | u - a(A0) in D")
    agg = Counter()
    for s, d, pf in allp:
        if s >= paths or s < first:
            continue
        pref = [(i, tuple(rt)) for i, rt in pf]
        res = run_prefix(P, pref, version, M, 31 * s + d)
        if res["status"] != "ok":
            print(f"{s:4d} {d:5d} | {res['status']}"); agg[res["status"]] += 1
            continue
        print(f"{s:4d} {d:5d} | {res['fillings']:4d} {res['tiles_seen']:4d} {res['rank']:4d} "
              f"{res['torsion']} | {res['named']} | {res['explained']} | {res['member']}", flush=True)
        agg["member" if res["member"] else "NOT member"] += 1
        agg["explained" if res["explained"] else "not explained"] += 1
    print(dict(agg), f"elapsed {time.time() - t0:.0f}s")
