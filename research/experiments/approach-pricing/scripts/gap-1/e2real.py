"""Gap 1 helper: the official pieces, the row-scan order of Track 10 (rows 15 to 0,
columns 0 to 15, clues on the board from the start, grey exactly on the rim), a
candidate index, and the measured level widths. Stdlib only, python3 -I.

Edge order URDL. Rotation r clockwise: new[i] = old[(i + 4 - r) % 4] (engine/src/types.rs).
"""
import json
import os
import subprocess

REPO = "."
CSV = os.path.join(REPO, "engine", "data", "official_eternity2.csv")
PROFILE_REF = "dc71ad56:research/experiments/real-puzzle-profile/results/m2/five-np1e6.jsonl"
N = 16


def rot(e, r):
    return tuple(e[(i + 4 - r) % 4] for i in range(4))


def load():
    lines = [l.strip() for l in open(CSV) if l.strip()]
    pieces, clues = [], {}
    for i, l in enumerate(lines[1:]):
        c = l.split(",")
        pieces.append(tuple(0 if int(x, 2) == 65535 else int(x, 2) for x in c[:4]))
        if int(c[4]) or int(c[5]) or int(c[6]):
            clues[(int(c[5]), int(c[4]))] = (i, int(c[6]))  # (row, col) -> (piece, rot)
    return pieces, clues


PIECES, CLUES = load()
CLUE_IDS = {p for p, _ in CLUES.values()}
CLUE_EDGES = {rc: rot(PIECES[p], r) for rc, (p, r) in CLUES.items()}
KIND = [sum(1 for v in p if v == 0) for p in PIECES]  # 0 interior, 1 edge, 2 corner
INTERIOR_POOL = sum(1 for i, k in enumerate(KIND) if k == 0 and i not in CLUE_IDS)
EDGE_POOL = sum(1 for k in KIND if k == 1)

# all distinct oriented placements, indexed by (left, down)
IDX = {}
for t, p in enumerate(PIECES):
    if t in CLUE_IDS:
        continue
    seen = set()
    for r in range(4):
        e = rot(p, r)
        if e in seen:
            continue
        seen.add(e)
        IDX.setdefault((e[3], e[2]), []).append((t, e[0], e[1], 1 << t))

# free cells in scan order
CELLS = [(r, c) for r in range(N - 1, -1, -1) for c in range(N) if (r, c) not in CLUES]
NFREE = len(CELLS)


def _spec(r, c):
    """Static part of the constraints of cell (r, c):
    (left_src, down_src, need_up, need_right) with src = ('g',) grey, ('s',) from the
    state, ('k', colour) from a clue; need_* = None (any non-grey), or a colour (0 = grey)."""
    if c == 0:
        ls = ("k", 0)
    elif (r, c - 1) in CLUES:
        ls = ("k", CLUE_EDGES[(r, c - 1)][1])
    else:
        ls = ("s",)
    if r == N - 1:
        ds = ("k", 0)
    elif (r + 1, c) in CLUES:
        ds = ("k", CLUE_EDGES[(r + 1, c)][0])
    else:
        ds = ("s",)
    if r == 0:
        nu = 0
    elif (r - 1, c) in CLUES:
        nu = CLUE_EDGES[(r - 1, c)][2]
    else:
        nu = None
    if c == N - 1:
        nr = 0
    elif (r, c + 1) in CLUES:
        nr = CLUE_EDGES[(r, c + 1)][3]
    else:
        nr = None
    return ls, ds, nu, nr


SPEC = [_spec(r, c) for r, c in CELLS]
_cache = {}


def cands(pos, left, down):
    """All oriented placements that fit cell CELLS[pos] with the given left and down
    colours, whatever the used set. Tuples (tile, up, right, bit)."""
    _, _, nu, nr = SPEC[pos]
    key = (nu, nr, left, down)
    v = _cache.get(key)
    if v is None:
        out = []
        for t, up, right, bit in IDX.get((left, down), ()):
            if nu is None:
                if up == 0:
                    continue
            elif up != nu:
                continue
            if nr is None:
                if right == 0:
                    continue
            elif right != nr:
                continue
            out.append((t, up, right, bit))
        v = tuple(out)
        _cache[key] = v
    return v


def context(pos, front, lastright):
    """(left, down) colours of the cell at scan position pos."""
    ls, ds, _, _ = SPEC[pos]
    left = ls[1] if ls[0] == "k" else lastright
    down = ds[1] if ds[0] == "k" else front[CELLS[pos][1]]
    return left, down


def measured_widths():
    """log10 width per free depth, mean over the 5 repetitions (mean of estimates, then log)."""
    from math import log10
    txt = subprocess.run(["git", "-C", REPO, "show", PROFILE_REF], capture_output=True, text=True, check=True).stdout
    reps = [json.loads(l)["log10_width"] for l in txt.splitlines() if l.strip()]
    L = min(len(r) for r in reps)
    out = []
    for d in range(L):
        vals = [r[d] for r in reps if r[d] is not None and r[d] > -1e300]
        if not vals:
            break
        m = max(vals)
        out.append(m + log10(sum(10 ** (v - m) for v in vals) / len(reps)))
    return out
