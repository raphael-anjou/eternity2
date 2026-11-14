"""Second analysis script 5b. Follow-up to r5.
(1) The shallow clue-pinned prefixes for which NO filling exists even with all 256 tiles
    reusable: is that a local fact at the clue cells? Cross-tabulated against arc consistency
    run with all 256 tiles, and against the two-cell window test at each clue of row 2
    (depth 16: a tile with top = frontier colour and bottom = the clue's top must exist for
    cell (1,2) and (1,13)).
(2) Only 3 of r5's 16 restarts get past row 2. 80 more restarts (seeds 2016..2095); the paths
    that reach depth 48 are kept and tested from depth 48 on (exact, ac, A, B).
(3) The tile-count certificate (r4, version A, pins kept) on r5's three deep paths.
Writes prefixes_clue_more.json.
Run: uv run --with python-flint python r5b_clue_more.py [extra_restarts] [fillings] [parts, e.g. 13 or 2]
(3) is run on one prefix per deep path (depths 48, 96, 144): a filling with the clues of row 13
pinned costs about 150,000 nodes, so 400 fillings take two minutes per prefix.
"""
import sys, os, json, random, time
from collections import Counter, defaultdict
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from rcommon import load, rots, N
from rfill import fill
import r2a_prefix_death as r2a
import r4_tile_count_invariants as r4
import r5_clue_prefixes as r5


def ac_all_tiles(P, pref, pins_d):
    """Arc consistency with every one of the 256 tiles allowed in every empty cell."""
    fake = [(-1 - j, rt) for j, (i, rt) in enumerate(pref)]     # placed tiles do not remove any tile
    return r2a.ac_refutes(P, fake, pins={d: (-999, rt) for d, (i, rt) in pins_d.items()})


if __name__ == "__main__":
    extra = int(sys.argv[1]) if len(sys.argv) > 1 else 80
    M = int(sys.argv[2]) if len(sys.argv) > 2 else 400
    parts = sys.argv[3] if len(sys.argv) > 3 else "123"
    P, clues = load()
    pins_d = {r * N + c: (i, rt) for i, r, c, rt in clues}
    pins_rc = {(r, c): (i, rt) for i, r, c, rt in clues}
    base = [(s, d, [(i, tuple(rt)) for i, rt in pf])
            for s, d, pf in json.load(open(os.path.join(HERE, "prefixes_clue.json")))]
    t0 = time.time()
    print("== (1) shallow prefixes: is 'no filling even with reuse of all tiles' local? ==")
    allrot = {rt for t in P for rt in rots(t)}
    tab = defaultdict(Counter)
    for s, d, pref in base:
        if d > 32 or "1" not in parts:
            continue
        rem_d = {dd: v for dd, v in pins_d.items() if dd >= d}
        rem_rc = {cell: v for cell, v in pins_rc.items() if cell[0] * N + cell[1] >= d}
        st = "cap"
        for attempt in range(3):
            st, _, _ = fill(P, pref, range(len(P)), random.Random(9000 + 17 * s + attempt), 200000, pins=rem_rc)
            if st != "cap":
                break
        ac = ac_all_tiles(P, pref, rem_d)
        if d == 16:
            fr = [pref[c][1][2] for c in range(N)]
            win = all(any(rt[0] == fr[c] and rt[2] == pins_rc[(2, c)][1][0] and 0 not in rt for rt in allrot)
                      for c in (2, 13))
        else:
            fr = [pref[N + c][1][2] for c in range(N)]
            def ok(c):
                cl = pins_rc[(2, c)][1]
                left = any(rt[0] == fr[c - 1] and rt[1] == cl[3] and 0 not in rt for rt in allrot)
                right = any(rt[0] == fr[c + 1] and rt[3] == cl[1] and 0 not in rt for rt in allrot)
                return left and right and cl[0] == fr[c]
            win = ok(2) and ok(13)
        tab[d][(f"A {st}", "AC(all tiles) refutes" if ac else "AC(all tiles) passes",
                "window ok" if win else "window fails")] += 1
    for d in sorted(tab):
        for k, v in sorted(tab[d].items()):
            print(f"  depth {d}: {k}: {v}")
    print("== (3) tile-count certificate on r5's deep paths ==")
    deep = sorted({s for s, d, _ in base if d >= 48})
    want = {(deep[j], dd) for j, dd in enumerate((48, 96, 144)) if j < len(deep)}
    agg = Counter()
    for s, d, pref in base:
        if (s, d) not in want or "3" not in parts:
            continue
        rem_rc = {cell: v for cell, v in pins_rc.items() if cell[0] * N + cell[1] >= d}
        res = r4.run_prefix(P, pref, "A", M, 77 * s + d, pins=rem_rc)
        if res["status"] == "ok":
            agg[(f"Z^{res['free']}+{res['torsion']}", f"explained={res['explained']}", f"member={res['member']}")] += 1
        else:
            agg[res["status"]] += 1
    for k, v in agg.items():
        print("  ", k, v)
    print(f"  elapsed {time.time() - t0:.0f}s", flush=True)
    print("== (2) more restarts ==")
    if "2" not in parts:
        sys.exit(0)
    targets = set(range(16, 256, 16))
    more = []
    reach = Counter()
    for s in range(16, 16 + extra):
        found, nodes = r5.dfs_prefixes_clue(P, pins_d, random.Random(2000 + s), targets, 200000)
        reach[max(found) if found else 0] += 1
        if max(found, default=0) >= 48:
            for d in sorted(found):
                if d >= 48:
                    more.append((s, d, [(i, list(rt)) for i, rt in found[d]]))
    json.dump(more, open(os.path.join(HERE, "prefixes_clue_more.json"), "w"))
    print(f"  deepest row boundary reached, per restart: {dict(sorted(reach.items()))}")
    print(f"  kept {len(more)} prefixes on {len({s for s, _, _ in more})} paths; "
          f"per depth {dict(sorted(Counter(d for _, d, _ in more).items()))}; {time.time() - t0:.0f}s", flush=True)
    tab = defaultdict(Counter)
    for s, d, pf in more:
        pref = [(i, tuple(rt)) for i, rt in pf]
        placed = {i for i, _ in pref}
        rem_d = {dd: v for dd, v in pins_d.items() if dd >= d}
        rem_rc = {cell: v for cell, v in pins_rc.items() if cell[0] * N + cell[1] >= d}
        st, n, _ = r2a.exact_below(P, pref, 300000, pins=rem_d)
        tab[d]["exact:" + st] += 1
        tab[d]["ac:refuted" if r2a.ac_refutes(P, pref, pins=rem_d) else "ac:passes"] += 1
        unpl = [i for i in range(len(P)) if i not in placed and i not in {v[0] for v in rem_rc.values()}]
        for name, allowed in (("A", list(range(len(P)))), ("B", unpl)):
            ok = "cap"
            for attempt in range(4 if name == "A" else 2):
                ok, _, _ = fill(P, pref, allowed, random.Random(9100 + 17 * s + attempt), 200000, pins=rem_rc)
                if ok != "cap":
                    break
            tab[d][name + ":" + {"filled": "filled", "none": "none exists", "cap": "undecided"}[ok]] += 1
    for d in sorted(tab):
        print(f"  depth {d:3d}: {dict(sorted(tab[d].items()))}")
    print(f"elapsed {time.time() - t0:.0f}s")
