import sys, json
d, csv = sys.argv[1], sys.argv[2]
M = {0: 0, 1: 1, 10: 7, 9: 2, 6: 9, 13: 5, 11: 12, 17: 3, 7: 14, 5: 4, 15: 15, 8: 19, 21: 21, 18: 8, 14: 10, 19: 13, 4: 16, 12: 17, 20: 18, 2: 6, 16: 20, 22: 22, 3: 11}
ext = [tuple(map(int, l.split())) for l in open(f"{d}/pieces.txt") if l.strip()]
ext = [(u, r, dn, l) for (u, dn, l, r) in ext]
board = json.load(open(f"{d}/record466.json"))["board"]
rows = [l.strip().split(",") for l in open(csv) if l.strip()][1:]
cv = lambda s: 0 if int(s, 2) == 65535 else int(s, 2)
repo = [tuple(cv(x) for x in r[:4]) for r in rows]
rot = lambda e, k: tuple(e[(i - k) % 4] for i in range(4))
n = 0
for i, r in enumerate(rows):
    x, y, k = int(r[4]), int(r[5]), int(r[6])
    if not (x or y or k): continue
    cell = y * 16 + x; s = board[cell]
    got = tuple(M[c] for c in rot(ext[s // 4], s % 4))
    for name, kk in (("cw", k), ("ccw", -k)):
        print(f"repo clue piece idx {i} cell {cell} rot {k} {name}: board edges {got} vs repo {rot(repo[i], kk)} equal={got == rot(repo[i], kk)}")
    n += got == rot(repo[i], k)
print("clues honoured (repo definition, cw):", n, "of 5")
