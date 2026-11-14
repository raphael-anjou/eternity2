"""Gap 5, script 4. Can any boundary invariant (Conway and Lagarias: abelian or not) refute a
row-scan prefix of the real tiles?

A boundary invariant of the tile set cannot tell how often a tile is used. So if the cells
below a prefix can be filled with edges matching when tiles may be REUSED, every boundary
invariant of the frontier is trivial and none can refute the prefix. Two versions:
  A. any of the 256 tiles, reuse allowed (invariants of the tile set alone);
  B. only tiles not yet placed, reuse allowed (invariants of the remaining tiles).
Finding a filling is a proof that no invariant refutes; not finding one within the node cap
decides nothing. Prefixes: same generator and seeds as script 3 (official pieces, no clue).
Run: python3 -I 04_reuse_completion.py [restarts] [prefix_node_cap] [fill_node_cap]
"""
import sys, random, time, importlib.util, os
from collections import defaultdict, Counter
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from e2common import load, rots, N


def dfs_prefixes(P, rng, targets, node_cap):
    by = defaultdict(list)
    for i, t in enumerate(P):
        for rt in set(rots(t)):
            by[(rt[3], rt[0])].append((i, rt))
    used = [False] * len(P)
    board = [None] * (N * N)
    found = {}
    nodes = 0
    def cands(d):
        r, c = divmod(d, N)
        left = board[d - 1][1][1] if c else 0
        top = board[d - N][1][2] if r else 0
        out = [(i, rt) for (i, rt) in by.get((left, top), ())
               if not used[i] and (rt[1] == 0) == (c == N - 1) and (rt[2] == 0) == (r == N - 1)]
        rng.shuffle(out)
        return out
    stack = [cands(0)]
    d = 0
    while stack and nodes < node_cap:
        if stack[-1]:
            i, rt = stack[-1].pop()
            board[d] = (i, rt); used[i] = True; nodes += 1
            d += 1
            if d in targets and d not in found:
                found[d] = list(board[:d])
                if len(found) == len(targets):
                    break
            if d == N * N:
                break
            stack.append(cands(d))
        else:
            stack.pop()
            d -= 1
            if d >= 0 and board[d] is not None and stack:
                used[board[d][0]] = False; board[d] = None
    return found, nodes


def fill_with_reuse(P, pref, allowed, rng, cap, once=False):
    """Depth-first filling of the cells below the prefix, tiles reusable. Order: row scan up
    to row 13, then rows 14 and 15 together column by column, so that a failure on the bottom
    rim is repaired by the cell just above it and not by the far end of row 14."""
    by = defaultdict(list)
    for i in allowed:
        for rt in set(rots(P[i])):
            by[(rt[3], rt[0])].append((i, rt))
    used = set()
    d0 = len(pref)
    k = d0 // N
    order = [(r, c) for r in range(k, min(N - 2, N)) for c in range(N)] if k < N - 2 else []
    if k <= N - 2:
        order += [(r, c) for c in range(N) for r in (N - 2, N - 1)]
    else:
        order += [(N - 1, c) for c in range(N)]
    board = {}
    for d, (_, rt) in enumerate(pref):
        board[divmod(d, N)] = rt
    def cands(j):
        r, c = order[j]
        left = board[(r, c - 1)][1] if c else 0
        top = board[(r - 1, c)][2] if r else 0
        out = [(i, rt) for (i, rt) in by.get((left, top), ())
               if (rt[1] == 0) == (c == N - 1) and (rt[2] == 0) == (r == N - 1)
               and not (once and i in used)]
        rng.shuffle(out)
        return out
    stack = [cands(0)]
    who = [None] * len(order)
    j = 0
    nodes = 0
    while stack and nodes < cap:
        if stack[-1]:
            i, rt = stack[-1].pop()
            board[order[j]] = rt; who[j] = i; used.add(i); nodes += 1
            j += 1
            if j == len(order):
                return True, nodes
            stack.append(cands(j))
        else:
            stack.pop(); j -= 1
            if j >= 0 and who[j] is not None:
                used.discard(who[j]); who[j] = None
    return (False if nodes >= cap else None), nodes     # None = exhausted: no filling exists


if __name__ == "__main__":
    restarts = int(sys.argv[1]) if len(sys.argv) > 1 else 12
    pcap = int(sys.argv[2]) if len(sys.argv) > 2 else 400000
    fcap = int(sys.argv[3]) if len(sys.argv) > 3 else 300000
    P, _ = load()
    targets = set(range(16, 256, 16))
    tally = {"A": defaultdict(Counter), "B": defaultdict(Counter)}
    cost = defaultdict(list)
    t0 = time.time()
    for s in range(restarts):
        rng = random.Random(1000 + s)
        found, _ = dfs_prefixes(P, rng, targets, pcap)
        for d in sorted(found):
            placed = {i for i, _ in found[d]}
            for name, allowed in (("A", range(len(P))), ("B", [i for i in range(len(P)) if i not in placed])):
                ok, nodes = fill_with_reuse(P, found[d], allowed, random.Random(5000 + s), fcap)
                tally[name][d][{True: "filled", False: "cap", None: "none exists"}[ok]] += 1
                if name == "B" and ok is None:
                    ok2, n2 = fill_with_reuse(P, found[d], allowed, random.Random(5000 + s), fcap, once=True)
                    assert ok2 is None
                    cost[d].append((nodes, n2))
        print(f"restart {s} done, {time.time() - t0:.0f}s", flush=True)
    for name in "AB":
        print(f"== version {name} ==")
        for d in sorted(tally[name]):
            print(f"depth {d:3d}: {dict(tally[name][d])}")
    print("== version B refutations: nodes to exhaust with reuse, and nodes of the exact subtree")
    print("   (each tile once, same order) under the same prefix ==")
    for d in sorted(cost):
        a = sorted(x for x, _ in cost[d]); b = sorted(y for _, y in cost[d])
        print(f"depth {d:3d}: n {len(a):2d}  reuse median {a[len(a)//2]:6d} max {a[-1]:6d} | "
              f"exact median {b[len(b)//2]:6d} max {b[-1]:6d}")
