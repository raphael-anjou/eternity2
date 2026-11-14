import sys
from collections import Counter
d, csv = sys.argv[1], sys.argv[2]
ext = [tuple(map(int, l.split())) for l in open(f"{d}/pieces.txt") if l.strip()]
ext = [(u, r, dn, l) for (u, dn, l, r) in ext]  # URDL
rows = [l.strip().split(",") for l in open(csv) if l.strip()][1:]
cv = lambda s: 0 if int(s, 2) == 65535 else int(s, 2)
repo = [tuple(cv(x) for x in r[:4]) for r in rows]
ce, cr = Counter(c for p in ext for c in p), Counter(c for p in repo for c in p)
print("colour count histograms equal:", sorted(ce.values()) == sorted(cr.values()), len(ce), len(cr))
print("ext", sorted(ce.items())); print("repo", sorted(cr.items()))
canon = lambda e: min(tuple(e[(i - k) % 4] for i in range(4)) for k in range(4))
# backtracking colour bijection making the two multisets (up to rotation) equal
E = Counter(canon(p) for p in ext)
R = [canon(p) for p in repo]
sys.setrecursionlimit(10000)
def key_ok(m, mi, a, b):
    for x, y in zip(a, b):
        if m.get(x, y) != y or mi.get(y, x) != x: return False
    return True
def rots(p): return [tuple(p[(i - k) % 4] for i in range(4)) for k in range(4)]
ext_list = sorted(E.elements(), key=lambda p: -len(set(p)))
def solve(i, m, mi, pool):
    if i == len(ext_list): return m
    p = ext_list[i]
    for q in set(pool):
        for rq in rots(q):
            if key_ok(m, mi, p, rq):
                m2, mi2 = dict(m), dict(mi)
                for x, y in zip(p, rq): m2[x] = y; mi2[y] = x
                pool2 = list(pool); pool2.remove(q)
                r = solve(i + 1, m2, mi2, pool2)
                if r: return r
    return None
m = {0: 0}; res = solve(0, m, {0: 0}, R)
print("colour bijection making multisets equal up to rotation:", "FOUND" if res else "none", res)
