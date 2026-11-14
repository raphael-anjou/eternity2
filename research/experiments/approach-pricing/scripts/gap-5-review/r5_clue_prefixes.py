"""Second analysis script 5. Prefixes WITH the five clues pinned (the real puzzle), which the memo did
not sample. Randomised row-scan depth-first search, the five clue tiles reserved and forced at
their cells in their rotations, with a one-cell look-ahead to a pinned neighbour. First arrival
at each row boundary, 16 restarts of 200,000 nodes (the memo's protocol otherwise).
Per prefix:
  exact : each-once search below, pins kept, node cap (exhausted = proven dead)
  ac    : arc consistency, no all-different, pins kept
  A     : filling with any of the 256 tiles reusable, pins kept (kills every boundary invariant
          of the tile set, by the memo's D3)
  B     : filling with only unplaced tiles reusable, pins kept
  count : the tile-count certificate of r4 (version A), pins kept
Writes prefixes_clue.json.
Run: uv run --with python-flint python r5_clue_prefixes.py [restarts] [count_paths] [fillings]
"""
import sys, os, json, random, time
from collections import Counter, defaultdict
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
os.environ.setdefault("E2_CSV", "research/experiments/"
                      "single-core-benchmark/engine/data/puzzles/size_16_official_eternity.csv")
from rcommon import load, rots, N
from rfill import fill, check_filling
import r2a_prefix_death as r2a
import r4_tile_count_invariants as r4


def dfs_prefixes_clue(P, pins, rng, targets, node_cap):
    """pins: {cell index: (tile, tuple)}."""
    pinned_ids = {i for i, _ in pins.values()}
    by = r2a.index_by(P, [i for i in range(len(P)) if i not in pinned_ids])
    used = set()
    board = [None] * (N * N)
    found = {}
    nodes = 0
    def cands(d):
        r, c = divmod(d, N)
        left = board[d - 1][1][1] if c else 0
        top = board[d - N][1][2] if r else 0
        if d in pins:
            i, rt = pins[d]
            return [(i, rt)] if rt[3] == left and rt[0] == top else []
        out = []
        for (i, rt) in by.get((left, top), ()):
            if i in used or (rt[1] == 0) != (c == N - 1) or (rt[2] == 0) != (r == N - 1):
                continue
            if d + 1 in pins and c < N - 1 and pins[d + 1][1][3] != rt[1]:
                continue
            if d + N in pins and pins[d + N][1][0] != rt[2]:
                continue
            out.append((i, rt))
        rng.shuffle(out)
        return out
    stack = [cands(0)]
    d = 0
    while stack and nodes < node_cap:
        if stack[-1]:
            i, rt = stack[-1].pop()
            board[d] = (i, rt); used.add(i); nodes += 1
            d += 1
            if d in targets and d not in found:
                found[d] = list(board[:d])
                if len(found) == len(targets):
                    break
            if d == N * N:
                break
            stack.append(cands(d))
        else:
            stack.pop(); d -= 1
            if d >= 0 and board[d] is not None and stack:
                used.discard(board[d][0]); board[d] = None
    return found, nodes


if __name__ == "__main__":
    restarts = int(sys.argv[1]) if len(sys.argv) > 1 else 16
    count_paths = int(sys.argv[2]) if len(sys.argv) > 2 else 2
    M = int(sys.argv[3]) if len(sys.argv) > 3 else 400
    P, clues = load()
    print("clues (tile number, row, col, tuple as placed top,right,bottom,left):",
          [(i + 1, r, c, rt) for i, r, c, rt in clues])
    pins_d = {r * N + c: (i, rt) for i, r, c, rt in clues}
    pins_rc = {(r, c): (i, rt) for i, r, c, rt in clues}
    targets = set(range(16, 256, 16))
    allp = []
    for s in range(restarts):
        found, nodes = dfs_prefixes_clue(P, pins_d, random.Random(2000 + s), targets, 200000)
        for d in sorted(found):
            allp.append((s, d, [(i, list(rt)) for i, rt in found[d]]))
    json.dump(allp, open(os.path.join(HERE, "prefixes_clue.json"), "w"))
    print(f"clue-pinned prefixes: {len(allp)}; per depth "
          f"{dict(sorted(Counter(d for _, d, _ in allp).items()))}")
    t0 = time.time()
    tab = defaultdict(Counter)
    for s, d, pf in allp:
        pref = [(i, tuple(rt)) for i, rt in pf]
        placed = {i for i, _ in pref}
        for dd, (i, rt) in pins_d.items():
            if dd < d:
                assert pref[dd] == (i, rt)
        rem_pins_d = {dd: v for dd, v in pins_d.items() if dd >= d}
        rem_pins_rc = {cell: v for cell, v in pins_rc.items() if cell[0] * N + cell[1] >= d}
        st, n, _ = r2a.exact_below(P, pref, 300000, pins=rem_pins_d)
        tab[d]["exact:" + st] += 1
        a = r2a.ac_refutes(P, pref, pins=rem_pins_d)
        tab[d]["ac:refuted" if a else "ac:passes"] += 1
        unpl = [i for i in range(len(P)) if i not in placed and i not in {v[0] for v in rem_pins_rc.values()}]
        for name, allowed in (("A", list(range(len(P)))), ("B", unpl)):
            ok = "cap"
            for attempt in range(3):
                stf, nf, pl = fill(P, pref, allowed, random.Random(9000 + 17 * s + attempt), 200000,
                                   pins=rem_pins_rc)
                if stf != "cap":
                    ok = stf; break
            if ok == "filled":
                check_filling(P, pref, pl)
            tab[d][name + ":" + {"filled": "filled", "none": "none exists", "cap": "undecided"}[ok]] += 1
        if s < count_paths:
            res = r4.run_prefix(P, pref, "A", M, 77 * s + d, pins=rem_pins_rc)
            if res["status"] == "ok":
                tab[d]["count:member" if res["member"] else "count:NOT member"] += 1
                tab[d][f"count:group Z^{res['free']}+{res['torsion']} explained={res['explained']}"] += 1
            else:
                tab[d]["count:" + res["status"]] += 1
    print(f"elapsed {time.time() - t0:.0f}s")
    for d in sorted(tab):
        print(f"depth {d:3d}: {dict(sorted(tab[d].items()))}")
