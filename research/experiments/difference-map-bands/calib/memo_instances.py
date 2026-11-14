"""Export the solve-path-6 memo's dm.py instances in the ladder jsonl format.

    python3 -I calib/memo_instances.py > calib/memo-instances.jsonl

Reproduces dm.py's gen(n, b, c, random.Random(1000*n + 10*c + seed)) (copied
here so numpy is not needed; the code is the memo's, unchanged in logic).
"""
import json
import random


def gen(n, b, c, rng):
    for _try in range(20000):
        h = [[0] * (n - 1) for _ in range(n)]
        v = [[0] * n for _ in range(n - 1)]
        for r in range(n):
            for j in range(n - 1):
                ring = r in (0, n - 1)
                h[r][j] = rng.randint(1, b) if ring else rng.randint(b + 1, b + c)
        for r in range(n - 1):
            for j in range(n):
                ring = j in (0, n - 1)
                v[r][j] = rng.randint(1, b) if ring else rng.randint(b + 1, b + c)
        sol = []
        for r in range(n):
            for j in range(n):
                t = (v[r - 1][j] if r > 0 else 0, h[r][j] if j < n - 1 else 0,
                     v[r][j] if r < n - 1 else 0, h[r][j - 1] if j > 0 else 0)
                sol.append(t)
        canon = set()
        ok = True
        for t in sol:
            rots = [t[k:] + t[:k] for k in range(4)]
            if len(set(rots)) < 4 and t.count(0) == 0:
                ok = False
                break
            m = min(rots)
            if m in canon:
                ok = False
                break
            canon.add(m)
        if ok:
            pieces = sol[:]
            rng.shuffle(pieces)
            pieces = [p[k:] + p[:k] for p in pieces for k in [rng.randrange(4)]]
            return pieces
    raise SystemExit("no instance")


# (n, b, c, seed) as in the memo's results table
CASES = [(5, 3, 5, 0), (5, 3, 5, 1), (5, 3, 5, 2), (6, 4, 8, 0), (6, 4, 8, 1),
         (6, 4, 7, 0), (6, 4, 7, 1), (7, 4, 11, 1), (8, 5, 12, 0)]

for n, b, c, seed in CASES:
    rng = random.Random(1000 * n + 10 * c + seed)
    pieces = gen(n, b, c, rng)
    row = {"id": f"memo-n{n}-b{b}-c{c}-s{seed}", "n": n, "c": c, "seed": seed,
           "puzzle": {"width": n, "height": n, "pieces": [list(p) for p in pieces]}}
    print(json.dumps(row))
