"""Row-major DFS in Python on the committed ladder instances.

Stdlib only. Run: python3 -I dfs_check.py <ladder> <n> [full_cap]
  1. first-solution node count, same candidate order as hp-dfs (piece index,
     then rotation, identical orientations skipped), compared with the
     committed hp-dfs rows where they exist;
  2. the whole tree (no stop at a solution): consistent partial boards per
     depth Z_d and the number of solutions, up to `full_cap` nodes.
Prints one JSON line per instance.
"""
import json, os, sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "../../../../../.."))
HL = os.path.join(ROOT, "research/experiments/hard-ladder")


def rotated(e, r):
    return tuple(e[(i - r) % 4] for i in range(4))


def build(pieces):
    """table[(top, left, right_is_rim, bottom_is_rim)] -> [(piece, right, bottom)] in hp-dfs order."""
    table = {}
    for p, e in enumerate(pieces):
        seen = []
        for r in range(4):
            o = rotated(e, r)
            if o in seen:
                continue
            seen.append(o)
            t, rt, b, l = o
            table.setdefault((t, l, rt == 0, b == 0), []).append((p, rt, b))
    return table


def search(n, pieces, stop_at_first, cap):
    table = build(pieces)
    N = n * n
    used = [False] * len(pieces)
    right = [0] * N
    bottom = [0] * N
    Z = [0] * (N + 1)
    nodes = 0
    sols = 0
    # explicit stack of candidate iterators
    def cands(pos):
        x, y = pos % n, pos // n
        t = 0 if y == 0 else bottom[pos - n]
        l = 0 if x == 0 else right[pos - 1]
        return iter(table.get((t, l, x == n - 1, y == n - 1), ()))
    stack = [cands(0)]
    placed = []
    while stack:
        pos = len(stack) - 1
        it = stack[-1]
        adv = False
        for p, rt, b in it:
            if used[p]:
                continue
            nodes += 1
            Z[pos + 1] += 1
            if nodes > cap:
                return nodes, Z, sols, True
            if pos + 1 == N:
                sols += 1
                if stop_at_first:
                    return nodes, Z, sols, False
                continue
            used[p] = True
            right[pos] = rt
            bottom[pos] = b
            placed.append(p)
            stack.append(cands(pos + 1))
            adv = True
            break
        if not adv:
            stack.pop()
            if placed:
                used[placed.pop()] = False
    return nodes, Z, sols, False


def main():
    ladder, n = sys.argv[1], int(sys.argv[2])
    full_cap = int(sys.argv[3]) if len(sys.argv) > 3 else 0
    seeds = None
    if len(sys.argv) > 4:
        lo, hi = sys.argv[4].split("-")
        seeds = range(int(lo), int(hi) + 1)
    committed = {}
    f = os.path.join(HL, "results/test", ladder, "hp-dfs.jsonl")
    if os.path.exists(f):
        for line in open(f):
            r = json.loads(line)
            committed[r["instance"]] = (r["status"], r["nodes"])
    for line in open(os.path.join(HL, "instances", ladder + "-test.jsonl")):
        inst = json.loads(line)
        if inst["n"] != n or (seeds and inst["seed"] not in seeds):
            continue
        pieces = [tuple(p) for p in inst["puzzle"]["pieces"]]
        first, _, _, capped = search(n, pieces, True, 1000000)
        out = {"instance": inst["id"], "n": n, "c": inst["c"], "first": None if capped else first,
               "committed": committed.get(inst["id"])}
        if full_cap:
            nodes, Z, sols, capped = search(n, pieces, False, full_cap)
            out.update({"full": None if capped else nodes, "Z": None if capped else Z, "solutions": None if capped else sols})
        print(json.dumps(out))


if __name__ == "__main__":
    main()
