"""Second analysis script 7. The non-abelian tile group, as far as finite quotients reach.

Group: G(T) = < h_c, v_c (23 colours each) | h_b v_r h_t^-1 v_l^-1 for every tile (t,r,b,l) of T
in every rotation >. A region tileable by T (tiles reusable) has a boundary word that is trivial
in G(T) (Conway and Lagarias; Thurston p. 757). For a finite group Q, every homomorphism
phi : G(T) -> Q is a boundary invariant; it is non-abelian information when its image is.
This script enumerates ALL homomorphisms into a list of small groups, by backtracking with
unit propagation on the relations, for
  (i)  T = the 256 official tiles;
  (ii) T = the unplaced tiles of each prefix the reuse search (version B) left undecided,
and evaluates phi on the boundary word of the empty region. It also runs the abelian test in
its integer form: is the abelianised boundary word in the lattice spanned by the unplaced tiles?
Controls: on every prefix where a version-A filling exists, all homomorphisms of (i) must give 1
(this also checks the orientation of the word); a planted non-abelian example must be found.
Run: uv run --with python-flint python r7_nonabelian_quotients.py [max_group_order] [paths for (ii)] [groups for (ii)]
"""
import sys, os, json, random, time, itertools
from collections import Counter, defaultdict
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from rcommon import load, rots, N
from rfill import fill

NC = 23


# ---------- small groups as permutation groups ----------
def closure(gens):
    n = len(gens[0])
    e = tuple(range(n))
    seen = {e}; todo = [e]
    while todo:
        x = todo.pop()
        for g in gens:
            y = tuple(x[g[i]] for i in range(n))
            if y not in seen:
                seen.add(y); todo.append(y)
    els = sorted(seen)
    idx = {x: i for i, x in enumerate(els)}
    m = len(els)
    mul = [[idx[tuple(a[b[i]] for i in range(n))] for b in els] for a in els]   # a after b
    inv = [0] * m
    for i in range(m):
        for j in range(m):
            if mul[i][j] == idx[e]:
                inv[i] = j
    return dict(order=m, mul=mul, inv=inv, e=idx[e])


def cyc(n, *cycles):
    p = list(range(n))
    for c in cycles:
        for a, b in zip(c, c[1:] + c[:1]):
            p[a] = b
    return tuple(p)


GROUPS = {
    "S3": closure([cyc(3, (0, 1)), cyc(3, (0, 1, 2))]),
    "D4": closure([cyc(4, (0, 1, 2, 3)), cyc(4, (0, 2))]),
    "Q8": closure([cyc(8, (0, 1, 2, 3), (4, 5, 6, 7)), cyc(8, (0, 4, 2, 6), (1, 7, 3, 5))]),
    "D5": closure([cyc(5, (0, 1, 2, 3, 4)), cyc(5, (1, 4), (2, 3))]),
    "A4": closure([cyc(4, (0, 1, 2)), cyc(4, (0, 1), (2, 3))]),
    "D6": closure([cyc(6, (0, 1, 2, 3, 4, 5)), cyc(6, (1, 5), (2, 4))]),
    "S4": closure([cyc(4, (0, 1)), cyc(4, (0, 1, 2, 3))]),
    "Heis3": closure([cyc(9, (0, 1, 2), (3, 4, 5), (6, 7, 8)), cyc(9, (0, 3, 6), (1, 4, 7), (2, 5, 8)),
                      cyc(9, (3, 4, 5), (6, 8, 7))]),
}


def relations(tiles):
    """Each relation (x1,x2,x3,x4) means a[x1] a[x2] = a[x3] a[x4]; h_c is variable c, v_c is
    NC + c. h_b v_r = v_l h_t."""
    rel = set()
    for t in tiles:
        for (top, right, bot, left) in set(rots(t)):
            rel.add((bot, NC + right, NC + left, top))
    return sorted(rel)


def all_homs(rel, G, limit=2000000):
    mul, inv, m = G["mul"], G["inv"], G["order"]
    touch = defaultdict(list)
    for ri, r in enumerate(rel):
        for x in set(r):
            touch[x].append(ri)
    vars_ = sorted(touch, key=lambda x: -len(touch[x]))
    a = {}
    out = []
    nodes = [0]

    def assign(x, val, trail):
        """Assign with propagation. Returns False on conflict."""
        queue = [(x, val)]
        while queue:
            x, val = queue.pop()
            if x in a:
                if a[x] != val:
                    return False
                continue
            a[x] = val; trail.append(x)
            for ri in touch[x]:
                x1, x2, x3, x4 = rel[ri]
                un = [y for y in (x1, x2, x3, x4) if y not in a]
                if not un:
                    if mul[a[x1]][a[x2]] != mul[a[x3]][a[x4]]:
                        return False
                elif len(un) == 1:
                    y = un[0]
                    if y == x1: v = mul[mul[a[x3]][a[x4]]][inv[a[x2]]]
                    elif y == x2: v = mul[inv[a[x1]]][mul[a[x3]][a[x4]]]
                    elif y == x3: v = mul[mul[a[x1]][a[x2]]][inv[a[x4]]]
                    else: v = mul[inv[a[x3]]][mul[a[x1]][a[x2]]]
                    queue.append((y, v))
        return True

    def rec():
        if len(out) >= limit:
            return
        nodes[0] += 1
        # dynamic choice: a variable of a relation with the fewest unassigned variables, so that
        # the next assignment makes propagation fire
        x = None; bestk = 9
        for r in rel:
            un = [y for y in r if y not in a]
            if un and len(set(un)) < bestk:
                bestk = len(set(un)); x = max(un, key=lambda y: len(touch[y]))
                if bestk == 1:
                    break
        if x is None:
            out.append(dict(a)); return
        for val in range(m):
            trail = []
            if assign(x, val, trail):
                rec()
            for y in trail:
                del a[y]
    sys.setrecursionlimit(10000)
    rec()
    return out, nodes[0]


def word_value(hom, word, G):
    """word: list of (variable, exponent +1/-1). Variables absent from the relations are free;
    returns None if the word uses one (then the region is trivially untileable or unconstrained)."""
    mul, inv = G["mul"], G["inv"]
    x = G["e"]
    for v, s in word:
        if v not in hom:
            return None
        g = hom[v] if s > 0 else inv[hom[v]]
        x = mul[x][g]
    return x


def region_word(pref):
    k = len(pref) // N
    frontier = [pref[(k - 1) * N + c][1][2] for c in range(N)]
    w = [(0, 1)] * N + [(NC + 0, 1)] * (N - k)
    w += [(frontier[c], -1) for c in range(N - 1, -1, -1)]
    w += [(NC + 0, -1)] * (N - k)
    return w


def nonabelian_image(hom, G):
    vals = sorted(set(hom.values()))
    mul = G["mul"]
    return any(mul[x][y] != mul[y][x] for x in vals for y in vals)


def abelian_member(tiles, pref):
    from flint import fmpz_mat
    gens = set()
    for t in tiles:
        for (top, right, bot, left) in set(rots(t)):
            v = [0] * (2 * NC)
            v[bot] += 1; v[top] -= 1; v[NC + right] += 1; v[NC + left] -= 1
            gens.add(tuple(v))
    gens = [list(g) for g in sorted(gens)]
    w = [0] * (2 * NC)
    for v, s in region_word(pref):
        w[v] += s
    def summ(rows):
        S = fmpz_mat(rows).snf()
        d = sorted(abs(int(S[i, i])) for i in range(min(S.nrows(), S.ncols())) if int(S[i, i]))
        return d
    return summ(gens) == summ(gens + [w])


if __name__ == "__main__":
    maxord = int(sys.argv[1]) if len(sys.argv) > 1 else 12
    P, clues = load()
    names = [n for n in GROUPS if GROUPS[n]["order"] <= maxord]
    t0 = time.time()
    print("== control: a planted non-abelian quotient (toy, not Eternity II) ==")
    # Plant a homomorphism to S3: give each of 6 colours random values h_c, v_c in S3, keep every
    # tile (t,r,b,l) over those colours whose four rotations all satisfy the relation, and ask
    # the search for it back. Repeated until the planted map has a non-abelian image on the
    # colours actually used.
    G = GROUPS["S3"]; mul = G["mul"]
    rngc = random.Random(11)
    while True:
        hv = {c: rngc.randrange(6) for c in range(1, 7)}
        vv = {c: rngc.randrange(6) for c in range(1, 7)}
        toy = [t for t in itertools.product(range(1, 7), repeat=4)
               if all(mul[hv[b]][vv[r]] == mul[vv[l]][hv[tp]] for (tp, r, b, l) in rots(t))]
        usedc = {c for t in toy for c in t}
        planted = {c: hv[c] for c in usedc}; planted.update({NC + c: vv[c] for c in usedc})
        if toy and nonabelian_image(planted, G):
            break
    hs, _ = all_homs(relations(toy), G)
    print(f"  planted S3 map on {len(usedc)} colours, {len(toy)} tiles: homomorphisms found {len(hs)}, "
          f"with non-abelian image {sum(nonabelian_image(h, G) for h in hs)}, planted one among them: "
          f"{planted in hs}")
    print("== (i) the 256 official tiles ==")
    rel = relations(P)
    print(f"generators used: {len({x for r in rel for x in r})} of 46; relations: {len(rel)}")
    full = {}
    for n in names:
        hs, nodes = all_homs(rel, GROUPS[n])
        full[n] = hs
        shapes = Counter()
        for h in hs:
            if nonabelian_image(h, GROUPS[n]):
                cls = lambda c: "grey" if c == 0 else ("frame" if c <= 5 else "interior")
                const = all(len({h[v] for v in h if (v < NC) == hv and cls(v % NC) == k}) == 1
                            for hv in (True, False) for k in ("grey", "frame", "interior"))
                shapes["constant on (h or v) x (grey, frame, interior)" if const else "NOT constant on colour classes"] += 1
        if shapes:
            print(f"    non-abelian ones: {dict(shapes)}")
        print(f"  {n} (order {GROUPS[n]['order']}): homomorphisms {len(hs)}, non-abelian image "
              f"{sum(nonabelian_image(h, GROUPS[n]) for h in hs)}, search nodes {nodes}, "
              f"{time.time() - t0:.0f}s", flush=True)
    allp = [(s, d, [(i, tuple(rt)) for i, rt in pf])
            for s, d, pf in json.load(open(os.path.join(HERE, "prefixes_noclue.json")))]
    bad = 0
    for s, d, pref in allp:
        w = region_word(pref)
        for n in names:
            for h in full[n]:
                if word_value(h, w, GROUPS[n]) != GROUPS[n]["e"]:
                    bad += 1
    print(f"control: (prefix, homomorphism) pairs of the full set with a non-trivial value on the "
          f"region word: {bad} (must be 0: every prefix has a version-A filling)")
    print("== (ii) unplaced tiles only ==")
    tab = defaultdict(Counter)
    npaths = int(sys.argv[2]) if len(sys.argv) > 2 else 16
    names2 = [n for n in names if n in (sys.argv[3].split(",") if len(sys.argv) > 3 else names)]
    print(f"paths 0..{npaths - 1}; groups for part (ii): {names2}")
    for s, d, pref in allp:
        if s >= npaths:
            continue
        placed = {i for i, _ in pref}
        unpl = [i for i in range(len(P)) if i not in placed]
        tiles = [P[i] for i in unpl]
        if not abelian_member(tiles, pref):
            tab[d]["abelian integer test REFUTES"] += 1
        else:
            tab[d]["abelian integer test passes"] += 1
        st = "cap"
        for attempt in range(3):
            st, _, _ = fill(P, pref, unpl, random.Random(4000 + 13 * s + attempt), 200000)
            if st != "cap":
                break
        tab[d]["B " + {"filled": "filled", "none": "none exists", "cap": "undecided"}[st]] += 1
        if st == "filled":
            continue
        rel = relations(tiles)
        w = region_word(pref)
        for n in names2:
            hs, nodes = all_homs(rel, GROUPS[n], limit=200000)
            print(f"  path {s} depth {d} B {st}: {n}: homs {len(hs)}, non-abelian image "
                  f"{sum(nonabelian_image(h, GROUPS[n]) for h in hs)}, nodes {nodes}, {time.time() - t0:.0f}s", flush=True)
            vals = [word_value(h, w, GROUPS[n]) for h in hs]
            ref = sum(1 for v in vals if v is not None and v != GROUPS[n]["e"])
            key = f"{n}: " + ("REFUTES" if ref else "no refutation")
            tab[d][key + (" (B none exists)" if st == "none" else " (B undecided)")] += 1
            if ref:
                na = sum(1 for h, v in zip(hs, vals) if v not in (None, GROUPS[n]["e"]) and nonabelian_image(h, GROUPS[n]))
                print(f"  path {s} depth {d}: {n} refutes; homs {len(hs)}, refuting {ref}, of which "
                      f"non-abelian image {na}", flush=True)
    print(f"elapsed {time.time() - t0:.0f}s")
    for d in sorted(tab):
        print(f"depth {d:3d}: {dict(sorted(tab[d].items()))}")
