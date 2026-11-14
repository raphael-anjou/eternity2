"""Second analysis script 6. The memo's test 1 (aggregated matching laws on the unplaced tiles), rebuilt
independently, in the forms the memo did not run:
  coarse  : memo's classes (row, column parity, rim pattern). LP (reproduction on a subset) and
            the SIGNED INTEGER form mod 2 on every prefix: is A x = b solvable over GF(2)?
            (A parity obstruction of the Tseitin kind would show here.)
  modp    : same system mod 3, 5 and 7 on one path, exact, and the rank of A mod those primes
            against its rank mod a 31-bit prime (equal ranks = no torsion at that prime).
  ip      : the 0/1 integer program on the deepest prefixes, 60 s each.
  cell    : every cell its own class (the top rung of the memo's ladder): LP and mod 2, on the
            prefixes at depth 144 and more, all of which r2a proved dead.
  clue    : coarse LP and mod 2 on the clue-pinned prefixes of r5, the clue tiles confined to
            their cell's class and rotation.
Run: uv run --with numpy --with scipy --with python-flint python r6_integer_and_power.py MODE [args]
"""
import sys, os, json, time
from collections import Counter, defaultdict
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import numpy as np
from scipy.optimize import linprog, milp, LinearConstraint, Bounds
from scipy.sparse import coo_matrix
from rcommon import load, rots, N


def build(P, pref, level, pins=None):
    """Returns (rows R, cols C, vals V, rhs b, nv). pins: {(r,c): (tile, tuple)} not yet placed."""
    pins = pins or {}
    d0 = len(pref)
    assert d0 % N == 0
    k = d0 // N
    placed = {i for i, _ in pref}
    remaining = [i for i in range(len(P)) if i not in placed]
    pin_of = {i: (cell, rt) for cell, (i, rt) in pins.items()}
    frontier = [pref[(k - 1) * N + c][1][2] for c in range(N)]
    def pat(r, c):
        return (r == 0, c == N - 1, r == N - 1, c == 0)
    if level == "coarse":
        cls = lambda r, c: (r, c % 2, pat(r, c))
        jc = lambda c: c % 2
    else:
        cls = lambda r, c: (r, c, pat(r, c))
        jc = lambda c: c
    cells = Counter()
    members = defaultdict(list)
    for r in range(k, N):
        for c in range(N):
            cells[cls(r, c)] += 1
            members[cls(r, c)].append((r, c))
    var = []
    for i in remaining:
        if i in pin_of:
            (r, c), rt = pin_of[i]
            var.append((i, rt, cls(r, c)))
            continue
        for rt in set(rots(P[i])):
            g = tuple(x == 0 for x in rt)
            for cl in cells:
                if cl[2] != g:
                    continue
                if level == "cell" and members[cl][0] in pins:
                    continue
                var.append((i, rt, cl))
    R, C, V, rhs = [], [], [], []
    eq = {}
    def row_of(key, b=0):
        if key not in eq:
            eq[key] = len(rhs); rhs.append(b)
        return eq[key]
    for i in remaining:
        row_of(("tile", i), 1)
    for cl, n in cells.items():
        row_of(("cells", cl), n)
    fr = Counter((jc(c), frontier[c]) for c in range(N))
    for (q, col), n in fr.items():
        row_of(("NS", k - 1, q, col), -n)
    for j, (i, (top, right, bot, left), cl) in enumerate(var):
        r, q = cl[0], cl[1]
        R.append(eq[("tile", i)]); C.append(j); V.append(1)
        R.append(eq[("cells", cl)]); C.append(j); V.append(1)
        if level == "coarse":
            if right: R.append(row_of(("EW", r, q, right))); C.append(j); V.append(1)
            if left: R.append(row_of(("EW", r, 1 - q, left))); C.append(j); V.append(-1)
        else:
            if right: R.append(row_of(("EW", r, q, right))); C.append(j); V.append(1)
            if left: R.append(row_of(("EW", r, q - 1, left))); C.append(j); V.append(-1)
        if bot: R.append(row_of(("NS", r, q, bot))); C.append(j); V.append(1)
        R.append(row_of(("NS", r - 1, q, top))); C.append(j); V.append(-1)
    return R, C, V, rhs, len(var)


def lp_feasible(sysm):
    R, C, V, rhs, nv = sysm
    A = coo_matrix((V, (R, C)), shape=(len(rhs), nv)).tocsr()
    res = linprog(np.zeros(nv), A_eq=A, b_eq=np.array(rhs, dtype=float), bounds=(0, 1), method="highs")
    return res.status != 2, res.status


def ip_status(sysm, secs):
    R, C, V, rhs, nv = sysm
    A = coo_matrix((V, (R, C)), shape=(len(rhs), nv)).tocsr()
    b = np.array(rhs, dtype=float)
    m = milp(np.zeros(nv), constraints=LinearConstraint(A, b, b), integrality=np.ones(nv),
             bounds=Bounds(0, 1), options={"time_limit": secs})
    return {0: "feasible", 1: "timeout", 2: "infeasible"}.get(m.status, f"status-{m.status}")


def solvable_mod2(sysm):
    """Is A x = b solvable over GF(2)? Exact."""
    R, C, V, rhs, nv = sysm
    rows = [0] * len(rhs)
    for r, c, v in zip(R, C, V):
        if v % 2:
            rows[r] ^= 1 << c
    aug = 1 << nv
    for r, b in enumerate(rhs):
        if b % 2:
            rows[r] |= aug
    piv = {}
    for v in rows:
        while v:
            low = v & -v
            if low == aug:
                return False
            if low in piv:
                v ^= piv[low]
            else:
                piv[low] = v
                break
    return True


def solvable_modp(sysm, p):
    """Is A x = b solvable over GF(p)? Exact: rank of A against rank of [A | b], full matrix,
    transposed so that the elimination runs on the short side."""
    from flint import nmod_mat
    R, C, V, rhs, nv = sysm
    m = len(rhs)
    A = nmod_mat(nv + 1, m, p)
    acc = {}
    for r, c, v in zip(R, C, V):
        acc[(c, r)] = acc.get((c, r), 0) + v
    for (c, r), v in acc.items():
        A[c, r] = v % p
    ra = A.rank()                 # last row still zero: rank of A
    for r, b in enumerate(rhs):
        if b % p:
            A[nv, r] = b % p
    return ra == A.rank(), ra


if __name__ == "__main__":
    mode = sys.argv[1]
    P, clues = load()
    t0 = time.time()
    noclue = [(s, d, [(i, tuple(rt)) for i, rt in pf])
              for s, d, pf in json.load(open(os.path.join(HERE, "prefixes_noclue.json")))]
    tab = defaultdict(Counter)
    if mode == "coarse":
        lp_paths = int(sys.argv[2]) if len(sys.argv) > 2 else 3
        for s, d, pref in noclue:
            sysm = build(P, pref, "coarse")
            tab[d]["mod 2 solvable" if solvable_mod2(sysm) else "mod 2 REFUTED"] += 1
            if s < lp_paths:
                tab[d]["LP feasible" if lp_feasible(sysm)[0] else "LP refuted"] += 1
    elif mode == "modp":
        path = int(sys.argv[2]) if len(sys.argv) > 2 else 0
        for s, d, pref in noclue:
            if s != path:
                continue
            sysm = build(P, pref, "coarse")
            ranks = {}
            for p in (3, 5, 7, 2147483629):
                ok, ra = solvable_modp(sysm, p)
                ranks[p] = ra
                if p < 100:
                    tab[d][f"mod {p} " + ("solvable" if ok else "REFUTED")] += 1
            tab[d][f"rank of A mod 3, 5, 7 equals rank mod a 31-bit prime: {len(set(ranks.values())) == 1}"] += 1
            print(d, ranks, f"{time.time() - t0:.0f}s", flush=True)
    elif mode == "ip":
        dmin = int(sys.argv[2]); secs = float(sys.argv[3]); lim = int(sys.argv[4])
        n = 0
        for s, d, pref in noclue:
            if d < dmin or n >= lim:
                continue
            n += 1
            st = ip_status(build(P, pref, "coarse"), secs)
            tab[d]["IP " + st] += 1
            print(s, d, st, f"{time.time() - t0:.0f}s", flush=True)
    elif mode == "cell":
        dmin = int(sys.argv[2]); dmax = int(sys.argv[3])
        for s, d, pref in noclue:
            if not (dmin <= d <= dmax):
                continue
            sysm = build(P, pref, "cell")
            ok, st = lp_feasible(sysm)
            tab[d]["cell LP feasible" if ok else "cell LP refuted"] += 1
            tab[d]["cell mod 2 solvable" if solvable_mod2(sysm) else "cell mod 2 REFUTED"] += 1
            print(s, d, ok, st, len(sysm[3]), sysm[4], f"{time.time() - t0:.0f}s", flush=True)
    elif mode == "clue":
        pins_all = {(r, c): (i, rt) for i, r, c, rt in clues}
        raw = json.load(open(os.path.join(HERE, "prefixes_clue.json")))
        more = os.path.join(HERE, "prefixes_clue_more.json")
        if os.path.exists(more):
            raw += json.load(open(more))
        cl = [(s, d, [(i, tuple(rt)) for i, rt in pf]) for s, d, pf in raw]
        lp_paths = int(sys.argv[2]) if len(sys.argv) > 2 else 16
        for s, d, pref in cl:
            if d < 48 and s >= 4:
                continue      # shallow prefixes: four paths are enough, the LP is slow there
            pins = {cell: v for cell, v in pins_all.items() if cell[0] * N + cell[1] >= d}
            sysm = build(P, pref, "coarse", pins)
            tab[d]["mod 2 solvable" if solvable_mod2(sysm) else "mod 2 REFUTED"] += 1
            if s < lp_paths:
                tab[d]["LP feasible" if lp_feasible(sysm)[0] else "LP refuted"] += 1
    print(f"mode {mode}; elapsed {time.time() - t0:.0f}s")
    for d in sorted(tab):
        print(f"depth {d:3d}: {dict(sorted(tab[d].items()))}")
