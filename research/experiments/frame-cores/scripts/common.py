"""Shared helpers for the frame-cores scripts: puzzle and frame files, an
independent board checker, and an independent frame checker. Stdlib only.
Written separately from the Rust engine on purpose (stage 0 compares them)."""


def rotate(e, r):
    t, ri, b, l = e
    return [(t, ri, b, l), (l, t, ri, b), (b, l, t, ri), (ri, b, l, t)][r % 4]


def load_puz(path):
    n, pieces, fixed, planted = 0, [], [], []
    for line in open(path):
        t = line.split()
        if not t:
            continue
        v = [int(x) for x in t[1:]]
        if t[0] == "n":
            n = v[0]
        elif t[0] == "p":
            pieces.append(tuple(v))
        elif t[0] == "f":
            fixed.append(tuple(v))
        elif t[0] == "s":
            planted.append(tuple(v))
    assert len(pieces) == n * n
    return {"n": n, "pieces": pieces, "fixed": fixed, "planted": planted}


def load_frames(path):
    out = []
    for line in open(path):
        t = line.split()
        if not t or t[0].startswith("#"):
            continue
        v = [int(x) for x in t[1:]]
        out.append((t[0], [tuple(v[i:i + 3]) for i in range(0, len(v), 3)]))
    return out


def nbr(n, pos, side):
    r, c = divmod(pos, n)
    if side == 0 and r > 0:
        return pos - n
    if side == 1 and c + 1 < n:
        return pos + 1
    if side == 2 and r + 1 < n:
        return pos + n
    if side == 3 and c > 0:
        return pos - 1
    return None


def is_border(n, pos):
    r, c = divmod(pos, n)
    return r in (0, n - 1) or c in (0, n - 1)


def check_board(puz, board, frame=None):
    """board: list of (pos, piece, rot) covering every cell. Returns a list of
    problems (empty = valid solution honouring clues and, if given, frame)."""
    n, pieces = puz["n"], puz["pieces"]
    probs = []
    at = {}
    for pos, pid, r in board:
        if pos in at:
            probs.append("cell %d twice" % pos)
        at[pos] = (pid, r)
    if sorted(at) != list(range(n * n)):
        probs.append("cells missing")
        return probs
    if sorted(pid for pid, _ in at.values()) != list(range(n * n)):
        probs.append("pieces not a permutation")
    for pos in range(n * n):
        e = rotate(pieces[at[pos][0]], at[pos][1])
        for s in range(4):
            q = nbr(n, pos, s)
            if q is None:
                if e[s] != 0:
                    probs.append("cell %d side %d not grey" % (pos, s))
            else:
                f = rotate(pieces[at[q][0]], at[q][1])
                if e[s] != f[(s + 2) % 4] or e[s] == 0:
                    probs.append("joint %d/%d mismatch" % (pos, s))
    for pos, pid, r in list(puz["fixed"]) + list(frame or []):
        if at[pos] != (pid, r) and rotate(pieces[pid], r) != rotate(pieces[at[pos][0]], at[pos][1]):
            probs.append("cell %d not as fixed" % pos)
        elif at[pos][0] != pid:
            probs.append("cell %d other piece" % pos)
    return probs


def check_frame(puz, frame):
    """A valid frame: every border cell once, border pieces distinct, grey
    outward, every border-to-border joint matching."""
    n, pieces = puz["n"], puz["pieces"]
    probs = []
    at = {pos: (pid, r) for pos, pid, r in frame}
    border = [p for p in range(n * n) if is_border(n, p)]
    if sorted(at) != border or len(frame) != len(border):
        return ["cells"]
    if len({pid for pid, _ in at.values()}) != len(border):
        probs.append("piece repeated")
    for pos in border:
        e = rotate(pieces[at[pos][0]], at[pos][1])
        for s in range(4):
            q = nbr(n, pos, s)
            if q is None and e[s] != 0:
                probs.append("not grey")
            if q is not None and is_border(n, q):
                f = rotate(pieces[at[q][0]], at[q][1])
                if e[s] != f[(s + 2) % 4]:
                    probs.append("lateral mismatch %d" % pos)
            if q is not None and not is_border(n, q) and e[s] == 0:
                probs.append("grey inward")
    return probs
