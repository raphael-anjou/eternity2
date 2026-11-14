"""Second analysis script 2a. What share of the memo's 184 prefixes is dead, and would a test with real
power have refuted them?

Regenerates the memo's prefixes with the memo's own generator and seeds (script 04,
dfs_prefixes, seeds 1000..1015, cap 200,000), then for each prefix:
  exact : row-scan depth-first search below the prefix, each tile once, node cap. Exhausted =
          proven dead.
  ac    : arc consistency on the empty cells with the unplaced tiles (no all-different), run to
          a fixed point from the prefix, no search. Refuted = some cell has no candidate left.
  row1  : can the NEXT row alone be completed with unplaced tiles, each once (exact search)?
Also a Knuth estimate of the clue-free level sizes at depths 16 and 32, to price the share of
prefixes that can be alive at all.
Writes prefixes_noclue.json for the other second analysis scripts.
Run: python3 -I r2a_prefix_death.py [exact_cap]
"""
import sys, os, json, random, time, math
from collections import defaultdict, Counter
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.join(HERE, "..", "gap-5"))
os.environ.setdefault("E2_CSV", "research/experiments/"
                      "single-core-benchmark/engine/data/puzzles/size_16_official_eternity.csv")
import importlib
memo4 = importlib.import_module("04_reuse_completion")
from rcommon import load, rots, N


def index_by(P, ids):
    by = defaultdict(list)
    for i in ids:
        for rt in set(rots(P[i])):
            by[(rt[3], rt[0])].append((i, rt))
    return by


def exact_below(P, pref, cap, pins=None, stop_row=None):
    """Row scan from the end of the prefix, each tile once. pins: {cell index: (tile, tuple)}.
    Returns ('dead'|'cap'|'solved', nodes, deepest)."""
    pins = pins or {}
    placed = {i for i, _ in pref}
    pinned_ids = {i for i, _ in pins.values()}
    free = [i for i in range(len(P)) if i not in placed and i not in pinned_ids]
    by = index_by(P, free)
    used = set()
    d0 = len(pref)
    end = N * N if stop_row is None else stop_row * N
    board = {d: rt for d, (_, rt) in enumerate(pref)}
    def cands(d):
        r, c = divmod(d, N)
        left = board[d - 1][1] if c else 0
        top = board[d - N][2] if r else 0
        if d in pins:
            i, rt = pins[d]
            return [(i, rt)] if rt[3] == left and rt[0] == top else []
        out = []
        for (i, rt) in by.get((left, top), ()):
            if i in used or (rt[1] == 0) != (c == N - 1) or (rt[2] == 0) != (r == N - 1):
                continue
            # look ahead to a pinned neighbour on the right or below
            if d + 1 in pins and c < N - 1 and pins[d + 1][1][3] != rt[1]:
                continue
            if d + N in pins and pins[d + N][1][0] != rt[2]:
                continue
            out.append((i, rt))
        return out
    if d0 >= end:
        return "solved", 0, d0
    stack = [cands(d0)]
    who = {}
    d = d0
    nodes = 0
    deepest = d0
    while stack:
        if nodes >= cap:
            return "cap", nodes, deepest
        if stack[-1]:
            i, rt = stack[-1].pop()
            board[d] = rt; who[d] = i; used.add(i); nodes += 1
            d += 1
            deepest = max(deepest, d)
            if d == end:
                return "solved", nodes, deepest
            stack.append(cands(d))
        else:
            stack.pop(); d -= 1
            if d >= d0 and d in who:
                used.discard(who.pop(d))
    return "dead", nodes, deepest


def ac_refutes(P, pref, pins=None):
    """Arc consistency without all-different on the empty cells. True if a domain empties."""
    pins = pins or {}
    placed = {i for i, _ in pref}
    pinned_ids = {i for i, _ in pins.values()}
    free = [i for i in range(len(P)) if i not in placed and i not in pinned_ids]
    opts = set()
    for i in free:
        for rt in set(rots(P[i])):
            opts.add(rt)
    d0 = len(pref)
    board = {d: rt for d, (_, rt) in enumerate(pref)}
    dom = {}
    for d in range(d0, N * N):
        r, c = divmod(d, N)
        if d in pins:
            dom[d] = {pins[d][1]}
            continue
        pat = (r == 0, c == N - 1, r == N - 1, c == 0)
        dom[d] = {rt for rt in opts if tuple(x == 0 for x in rt) == pat}
    # frontier
    for d in range(d0, N * N):
        r, c = divmod(d, N)
        if d - N in board:
            dom[d] = {rt for rt in dom[d] if rt[0] == board[d - N][2]}
        if c and d - 1 in board:
            dom[d] = {rt for rt in dom[d] if rt[3] == board[d - 1][1]}
    changed = True
    while changed:
        changed = False
        for d in range(d0, N * N):
            if not dom[d]:
                return True
            r, c = divmod(d, N)
            for (e, mine, theirs) in ((d + 1, 1, 3), (d + N, 2, 0)):
                if e >= N * N or (mine == 1 and c == N - 1):
                    continue
                a = {rt[mine] for rt in dom[d]}
                b = {rt[theirs] for rt in dom[e]}
                both = a & b
                if len(both) < len(a):
                    dom[d] = {rt for rt in dom[d] if rt[mine] in both}; changed = True
                if len(both) < len(b):
                    dom[e] = {rt for rt in dom[e] if rt[theirs] in both}; changed = True
    return any(not v for v in dom.values())


def knuth_level(P, depth, probes, rng):
    by = index_by(P, range(len(P)))
    tot = 0.0
    for _ in range(probes):
        used = set(); board = []; w = 1.0
        for d in range(depth):
            r, c = divmod(d, N)
            left = board[d - 1][1] if c else 0
            top = board[d - N][2] if r else 0
            cs = [(i, rt) for (i, rt) in by.get((left, top), ())
                  if i not in used and (rt[1] == 0) == (c == N - 1) and (rt[2] == 0) == (r == N - 1)]
            if not cs:
                w = 0.0; break
            w *= len(cs)
            i, rt = rng.choice(cs)
            used.add(i); board.append(rt)
        tot += w
    return tot / probes


if __name__ == "__main__":
    cap = int(sys.argv[1]) if len(sys.argv) > 1 else 400000
    P, clues = load()
    Pm, _ = memo4.load()
    assert list(P) == list(Pm), "memo loader and second analysis loader disagree"
    targets = set(range(16, 256, 16))
    allp = []
    for s in range(16):
        found, _ = memo4.dfs_prefixes(Pm, random.Random(1000 + s), targets, 200000)
        for d in sorted(found):
            allp.append((s, d, [(i, list(rt)) for i, rt in found[d]]))
    json.dump(allp, open(os.path.join(HERE, "prefixes_noclue.json"), "w"))
    print(f"prefixes regenerated: {len(allp)}; per depth: "
          f"{dict(sorted(Counter(d for _, d, _ in allp).items()))}")
    # nesting: is the depth-d prefix of a path an extension of its depth-(d-16) prefix?
    bypath = defaultdict(dict)
    for s, d, pf in allp:
        bypath[s][d] = pf
    nested = sum(1 for s in bypath for d in bypath[s] if d > 16 and bypath[s][d][:d - 16] == bypath[s][d - 16])
    print(f"prefixes that extend the same path's previous prefix: {nested} of {len(allp) - 16}")
    t0 = time.time()
    tab = defaultdict(Counter); nodes_dead = defaultdict(list)
    for s, d, pf in allp:
        pref = [(i, tuple(rt)) for i, rt in pf]
        st, n, deep = exact_below(P, pref, cap)
        tab[d]["exact:" + st] += 1
        if st == "dead":
            nodes_dead[d].append(n)
        a = ac_refutes(P, pref)
        tab[d]["ac:refuted" if a else "ac:passes"] += 1
        if st == "dead":
            tab[d]["dead and ac refutes" if a else "dead and ac passes"] += 1
        st1, n1, _ = exact_below(P, pref, 200000, stop_row=d // N + 1)
        tab[d]["row1:" + {"solved": "fillable", "dead": "not fillable", "cap": "cap"}[st1]] += 1
    print(f"exact cap {cap} nodes; elapsed {time.time() - t0:.0f}s")
    for d in sorted(tab):
        nd = sorted(nodes_dead[d])
        med = nd[len(nd) // 2] if nd else "-"
        print(f"depth {d:3d}: {dict(sorted(tab[d].items()))} | dead subtree median nodes {med}")
    rng = random.Random(7)
    for depth, probes in ((16, 20000), (32, 20000)):
        est = knuth_level(P, depth, probes, rng)
        print(f"Knuth estimate, clue-free level {depth}: 10^{math.log10(est):.2f} nodes ({probes} probes)")
