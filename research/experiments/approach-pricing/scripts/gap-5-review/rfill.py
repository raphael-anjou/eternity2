"""Second analysis helper: depth-first filling of the empty cells of a partial board, tiles reusable or
each once, with optional pinned cells (clues). Stdlib only."""
from collections import defaultdict
from rcommon import rots, N


def make_index(P, ids):
    by = defaultdict(list)
    for i in ids:
        for rt in set(rots(P[i])):
            by[(rt[3], rt[0])].append((i, rt))
    return by


def fill_order(d0, pinned_cells=()):
    """Row scan, except that (a) the last two rows are filled together column by column, and
    (b) a row holding a pinned cell is filled together with the row above it, column by column,
    so that a mismatch at the pin is repaired by its neighbours and not by the far end of the
    previous row (chronological backtracking thrashes otherwise)."""
    k = d0 // N
    assert d0 % N == 0
    pin_rows = {r for (r, c) in pinned_cells}
    cells = []
    r = k
    while r < N:
        if r + 1 < N and (r + 1 in pin_rows or r == N - 2):
            cells += [(rr, c) for c in range(N) for rr in (r, r + 1)]
            r += 2
        else:
            cells += [(r, c) for c in range(N)]
            r += 1
    return [x for x in cells if x not in pinned_cells]


def fill(P, pref, allowed, rng, cap, once=False, pins=None):
    """pref: list of (tile, tuple) in row-scan order. pins: {(r,c): (tile, tuple)}.
    Returns (status, nodes, placement) with status 'filled' | 'cap' | 'none';
    placement = list of ((r,c), tile, tuple) for the filled cells (pins included)."""
    pins = pins or {}
    d0 = len(pref)
    board = {divmod(d, N): rt for d, (_, rt) in enumerate(pref)}
    pins = {cell: v for cell, v in pins.items() if cell not in board}
    for cell, (_, rt) in pins.items():
        board[cell] = rt
    # pins must agree with what is already placed
    for (r, c), (_, rt) in pins.items():
        for (dr, dc, mine, theirs) in ((-1, 0, 0, 2), (0, -1, 3, 1), (1, 0, 2, 0), (0, 1, 1, 3)):
            o = board.get((r + dr, c + dc))
            if o is not None and o[theirs] != rt[mine]:
                return "none", 0, None
    order = fill_order(d0, set(pins))
    by = make_index(P, allowed)
    used = set()

    def cands(j):
        r, c = order[j]
        left = board[(r, c - 1)][1] if c else 0
        top = board[(r - 1, c)][2] if r else 0
        rn = board.get((r, c + 1)); bn = board.get((r + 1, c))
        out = []
        for (i, rt) in by.get((left, top), ()):
            if (rt[1] == 0) != (c == N - 1) or (rt[2] == 0) != (r == N - 1):
                continue
            if once and i in used:
                continue
            if rn is not None and rn[3] != rt[1]:
                continue
            if bn is not None and bn[0] != rt[2]:
                continue
            out.append((i, rt))
        if rng is not None:
            rng.shuffle(out)
        return out
    if not order:
        return "filled", 0, [(cell, i, rt) for cell, (i, rt) in pins.items()]
    stack = [cands(0)]
    who = [None] * len(order)
    j = 0
    nodes = 0
    while stack:
        if nodes >= cap:
            return "cap", nodes, None
        if stack[-1]:
            i, rt = stack[-1].pop()
            board[order[j]] = rt; who[j] = (i, rt); used.add(i); nodes += 1
            j += 1
            if j == len(order):
                pl = [(order[a], who[a][0], who[a][1]) for a in range(len(order))]
                pl += [(cell, i, rt) for cell, (i, rt) in pins.items()]
                return "filled", nodes, pl
            stack.append(cands(j))
        else:
            stack.pop(); j -= 1
            if j >= 0 and who[j] is not None:
                used.discard(who[j][0]); who[j] = None
                del board[order[j]]
    return "none", nodes, None


def check_filling(P, pref, pl, allowed=None):
    """Independent verification that a filling matches everywhere."""
    board = {divmod(d, N): rt for d, (_, rt) in enumerate(pref)}
    for cell, i, rt in pl:
        assert rt in rots(P[i])
        assert allowed is None or i in allowed
        board[cell] = rt
    assert len(board) == N * N
    for (r, c), rt in board.items():
        assert (rt[0] == 0) == (r == 0) and (rt[2] == 0) == (r == N - 1)
        assert (rt[3] == 0) == (c == 0) and (rt[1] == 0) == (c == N - 1)
        if c < N - 1:
            assert rt[1] == board[(r, c + 1)][3]
        if r < N - 1:
            assert rt[2] == board[(r + 1, c)][0]
    return True
