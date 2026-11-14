import sys, itertools
d, csv = sys.argv[1], sys.argv[2]
ext = [tuple(map(int, l.split())) for l in open(f"{d}/pieces.txt") if l.strip()]
rows = [l.strip().split(",") for l in open(csv) if l.strip()][1:]
cv = lambda s: 0 if int(s, 2) == 65535 else int(s, 2)
repo = [tuple(cv(x) for x in r[:4]) for r in rows]
# try every permutation of ext field order (24) as the mapping to repo T R B L, per-index alignment, with global rotation
for perm in itertools.permutations(range(4)):
    for k in range(4):
        m, mi, ok = {}, {}, True
        for e, p in zip(ext, repo):
            a = tuple(e[j] for j in perm); a = tuple(a[(i - k) % 4] for i in range(4))
            for x, y in zip(a, p):
                if m.setdefault(x, y) != y or mi.setdefault(y, x) != x: ok = False; break
            if not ok: break
        if ok: print("perm (ext field index for T,R,B,L)", perm, "rot", k, "colours", len(m))
print("done")
