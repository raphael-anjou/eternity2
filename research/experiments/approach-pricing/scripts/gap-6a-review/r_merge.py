"""Second analysis re-measurement of merged fillings on the real tiles, written independently of merge_real.py.
Stdlib only. Run: python3 -I r_merge.py <official.json> [region ...]
Region = set of cells (row, col), row 0 at the top; 'rim' sides must be grey, all other sides non-grey.
A filling uses distinct tiles and matches internal edges. Two fillings merge when they have the same
tile set and the same outline. For each merged class the permutation between the two fillings is
classified (which tiles moved, cycle type), to see which mechanisms exist."""
import json, sys, time
from collections import defaultdict, Counter

T = [tuple(p) for p in json.load(open(sys.argv[1]))["pieces"]]      # U,R,D,L
ORI = [(i, t[4 - k:] + t[:4 - k]) for i, t in enumerate(T) for k in range(4)]   # clockwise turns
assert len({(i, o) for i, o in ORI}) == 1024
STEP = {0: (-1, 0), 1: (0, 1), 2: (1, 0), 3: (0, -1)}

def rect(h, w): return [(r, c) for r in range(h) for c in range(w)]
def region(cells, rim): return cells, set(rim)
R = {
 "interior 1x2": region(rect(1, 2), []),
 "interior 1x3": region(rect(1, 3), []),
 "interior L3": region([(0, 0), (0, 1), (1, 0)], []),
 "interior 2x2": region(rect(2, 2), []),
 "rim 1x2": region(rect(1, 2), [((0, c), 2) for c in range(2)]),
 "rim 1x3": region(rect(1, 3), [((0, c), 2) for c in range(3)]),
 "rim 1x4": region(rect(1, 4), [((0, c), 2) for c in range(4)]),
 "rim 1x5": region(rect(1, 5), [((0, c), 2) for c in range(5)]),
 "rim 1x6": region(rect(1, 6), [((0, c), 2) for c in range(6)]),
 "rim 2x2": region(rect(2, 2), [((1, c), 2) for c in range(2)]),
 "rim 2x3": region(rect(2, 3), [((1, c), 2) for c in range(3)]),
 "corner 2x2": region(rect(2, 2), [((1, c), 2) for c in range(2)] + [((r, 0), 3) for r in range(2)]),
 "corner 2x3": region(rect(2, 3), [((1, c), 2) for c in range(3)] + [((r, 0), 3) for r in range(2)]),
 "corner 3x3": region(rect(3, 3), [((2, c), 2) for c in range(3)] + [((r, 0), 3) for r in range(3)]),
}

def enumerate_region(name, keep_all=False):
    cells, rim = R[name]; pos = {c: n for n, c in enumerate(cells)}
    plan = []
    for n, c in enumerate(cells):
        grey = [(c, s) in rim for s in range(4)]
        back = []; free = []
        for s in range(4):
            nb = (c[0] + STEP[s][0], c[1] + STEP[s][1])
            if nb in pos:
                if pos[nb] < n: back.append((s, pos[nb], (s + 2) % 4))
            elif not grey[s]: free.append(s)
        idx = defaultdict(list)
        for i, o in ORI:
            if all((o[s] == 0) == grey[s] for s in range(4)):
                idx[tuple(o[s] for s, _, _ in back)].append((i, o))
        plan.append((back, free, idx))
    classes = defaultdict(list); total = 0
    cur = [None] * len(cells); used = [False] * 256
    def go(n):
        nonlocal total
        if n == len(cells):
            total += 1
            key = (tuple(sorted(i for i, _ in cur)), tuple(cur[m][1][s] for m in range(len(cells)) for s in plan[m][1]))
            classes[key].append(tuple(i for i, _ in cur) if not keep_all else tuple(cur))
            return
        back, free, idx = plan[n]
        for i, o in idx.get(tuple(cur[m][1][t] for _, m, t in back), ()):
            if not used[i]:
                used[i] = True; cur[n] = (i, o); go(n + 1); used[i] = False
    go(0)
    return cells, total, classes

def report(name):
    t0 = time.time()
    cells, total, classes = enumerate_region(name)
    merged = {k: v for k, v in classes.items() if len(v) > 1}
    pairs = sum(len(v) - 1 for v in merged.values())
    kinds = Counter(); triple = 0
    for k, v in merged.items():
        a, b = v[0], v[1]
        moved = [n for n in range(len(cells)) if a[n] != b[n]]
        # cycle type of the tile permutation on cells
        perm = {n: b.index(a[n]) for n in moved}; seen = set(); cyc = []
        for n in moved:
            if n in seen: continue
            m = n; ln = 0
            while m not in seen: seen.add(m); m = perm[m]; ln += 1
            cyc.append(ln)
        kinds[(len(v), tuple(sorted(cyc)), len(cells) - len(moved))] += 1
        if {6, 7, 51} <= {a[n] for n in moved}: triple += 1
    print("%-13s fillings %9d  distinct %9d  merged pairs %4d  m-1 %.2e  classes of size>2: %d  with tiles 6,7,51 moved: %d  (%.0f s)"
          % (name, total, len(classes), pairs, pairs / len(classes), sum(1 for v in merged.values() if len(v) > 2), triple,
             time.time() - t0), flush=True)
    for (sz, cyc, fixed), n in sorted(kinds.items(), key=lambda x: -x[1])[:6]:
        print("      %4d classes: size %d, cycle type %s, cells with the same tile %d" % (n, sz, cyc, fixed))

if __name__ == "__main__":
    for nm in (sys.argv[2:] or list(R)): report(nm)
