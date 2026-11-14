import json, sys, itertools
data_dir, csv_path = sys.argv[1], sys.argv[2]
rec = json.load(open(f"{data_dir}/record466.json"))
board = rec["board"]
ext = [tuple(map(int, l.split())) for l in open(f"{data_dir}/pieces.txt") if l.strip()]  # file order U D L R
assert len(ext) == 256 and len(board) == 256
ext_urdl = [(u, r, d, l) for (u, d, l, r) in ext]
def rot(e, k):  # clockwise by k: new[(i+k)%4] = old[i]
    return tuple(e[(i - k) % 4] for i in range(4))
# repo official set
rows = [l.strip().split(",") for l in open(csv_path) if l.strip()][1:]
def c(s):
    v = int(s, 2); return 0 if v == 65535 else v
repo = [tuple(c(x) for x in r[:4]) for r in rows]          # T R B L
repo_clues = [(i, int(r[4]), int(r[5]), int(r[6])) for i, r in enumerate(rows) if any(int(x) for x in r[4:7])]

print("== Piece multiset ==")
ids = [s // 4 + 1 for s in board]; rots = [s % 4 for s in board]
print("distinct ids:", len(set(ids)), "range", min(ids), max(ids), "every id once:", sorted(ids) == list(range(1, 257)))

print("== Colour map ext->repo (is ext piece set the official one?) ==")
found = None
for k0 in range(4):
    m, minv, ok = {}, {}, True
    for i in range(256):
        a, b = rot(ext_urdl[i], k0), repo[i]
        for x, y in zip(a, b):
            if m.setdefault(x, y) != y or minv.setdefault(y, x) != x: ok = False
    if ok: found = (k0, m); print("per-piece alignment offset k0 =", k0, "consistent bijection of", len(m), "colours")
if not found: print("NO consistent colour bijection: ext pieces are not the repo's set under any offset")
# multiset up to rotation
canon = lambda e: min(rot(e, k) for k in range(4))
mp = {}
if found:
    k0, m = found
    A = sorted(canon(tuple(m[x] for x in e)) for e in ext_urdl)
    B = sorted(canon(p) for p in repo)
    print("piece multiset equals repo official set up to rotation and colour relabel:", A == B)

print("== Board (ext convention: state=4*(id-1)+rot cw, sides U R D L, row-major) ==")
cell = []
for s in board:
    cell.append(rot(ext_urdl[s // 4], s % 4))
N = 16; matched = 0; bad = []
for y in range(N):
    for x in range(N):
        u, r, d, l = cell[y * N + x]
        if x + 1 < N:
            if r == cell[y * N + x + 1][3]: matched += 1
            else: bad.append((y, x, "H"))
        if y + 1 < N:
            if d == cell[(y + 1) * N + x][0]: matched += 1
            else: bad.append((y, x, "V"))
print("internal edges:", 2 * N * (N - 1), "matched:", matched, "mismatched:", len(bad))
rim = 0
for y in range(N):
    for x in range(N):
        u, r, d, l = cell[y * N + x]
        want = (u == 0) == (y == 0) and (d == 0) == (y == N - 1) and (l == 0) == (x == 0) and (r == 0) == (x == N - 1)
        if not want: rim += 1
print("cells with grey sides not exactly on the rim:", rim)
print("== Clues ==")
js = json.load(open(f"{data_dir}/puzzle.json"))["fixed_clues"]
n_ext = sum(board[c_["cell"]] == c_["state"] for c_ in js)
print("record board matches puzzle.json's own five clue states:", n_ext, "of 5")
if found:
    k0, m = found
    n = 0
    for (pid, x, y, r) in repo_clues:
        cellidx = y * 16 + x; s = board[cellidx]
        got = tuple(m[v] for v in cell[cellidx])
        same_piece = (s // 4 == pid)
        want = repo[pid]
        # repo rot r: clockwise (try both) 
        w_cw, w_ccw = rot(want, r), rot(want, -r)
        ok = same_piece and got == w_cw
        print(f" repo clue piece {pid+1} at cell {cellidx} rot {r}: board piece {s//4+1} state rot {s%4}; oriented edges equal repo(cw)={got==w_cw} repo(ccw)={got==w_ccw}")
        n += ok
    print("clues honoured per repo definition (same piece, same oriented edges, cw):", n, "of 5")
