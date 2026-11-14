"""Shared helpers for the solve-path-4 memo. Stdlib only. Run with python3 -I."""
import json, math, os

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *[".."] * 6))
BOARDS = os.path.join(ROOT, "research", "topics", "rigidity-sat-halo", "compute", "boards.json")
N = 16
L10 = math.log(10.0)


def lg_fact(n):
    return math.lgamma(n + 1) / L10


def lg_binom(n, k):
    if k < 0 or k > n:
        return float("-inf")
    return lg_fact(n) - lg_fact(k) - lg_fact(n - k)


def lg_add(a, b):
    if a == float("-inf"):
        return b
    if b == float("-inf"):
        return a
    m = max(a, b)
    return m + math.log10(10 ** (a - m) + 10 ** (b - m))


def lg_binom_cdf(E, k, p_match):
    """log10 P(at most k mismatches among E independent edges), each matching w.p. p_match."""
    if E <= 0:
        return 0.0
    lp, lq = math.log10(p_match), math.log10(1 - p_match)
    tot = float("-inf")
    for j in range(0, min(k, E) + 1):
        tot = lg_add(tot, lg_binom(E, j) + j * lq + (E - j) * lp)
    return min(tot, 0.0)


CSV = os.path.join(ROOT, "engine", "data", "official_eternity2.csv")


def official_pieces():
    lines = [l.strip() for l in open(CSV) if l.strip()]
    ps = []
    for l in lines[1:]:
        c = l.split(",")
        ps.append(tuple(0 if int(x, 2) == 65535 else int(x, 2) for x in c[:4]))
    return ps


def _signatures(tuples):
    """Colour-refinement signatures of each colour of a piece set (relabeling-invariant)."""
    cols = sorted({c for t in tuples for c in t})
    sig = {c: (0 if c == 0 else 1) for c in cols}
    for _ in range(6):
        new = {}
        for c in cols:
            occ = []
            for t in tuples:
                for j in range(4):
                    if t[j] == c:
                        occ.append(tuple(sig[t[(j + k) % 4]] for k in range(4)))
            new[c] = hash((sig[c], tuple(sorted(occ))))
        sig = new
    return sig


def to_official(grid):
    """Relabel a board's colours to the official numbering. Asserts the result is the official piece set."""
    off = official_pieces()
    so = _signatures(off)
    tuples = [grid[r][c] for r in range(N) for c in range(N)]
    sb = _signatures(tuples)
    inv = {}
    for c, s_ in so.items():
        assert s_ not in inv, "ambiguous colour signature"
        inv[s_] = c
    m = {c: inv[s_] for c, s_ in sb.items()}
    assert m[0] == 0
    g = [[tuple(m[x] for x in grid[r][c]) for c in range(N)] for r in range(N)]
    a = sorted(canon(t) for row in g for t in row)
    b = sorted(canon(t) for t in off)
    assert a == b, "board is not a rearrangement of the official piece set"
    return g


def load_boards():
    """Return {id: grid}, grid[r][c] = (up, right, down, left) as ints in official colours, 0 = grey rim."""
    out = {}
    for b in json.load(open(BOARDS))["boards"]:
        s = b["edges"]
        assert len(s) == 4 * N * N
        g = [[tuple(ord(ch) - 97 for ch in s[4 * (r * N + c):4 * (r * N + c) + 4]) for c in range(N)] for r in range(N)]
        out[b["id"]] = to_official(g)
    return out


def canon(t):
    return min(t[i:] + t[:i] for i in range(4))


def internal_edges(n=N):
    """Yield (cellA, sideA, cellB, sideB) for every internal edge. Sides: 0 up, 1 right, 2 down, 3 left."""
    for r in range(n):
        for c in range(n):
            if c + 1 < n:
                yield (r, c), 1, (r, c + 1), 3
            if r + 1 < n:
                yield (r, c), 2, (r + 1, c), 0


def defects(g, n=N):
    return [(a, sa, b, sb) for a, sa, b, sb in internal_edges(n) if g[a[0]][a[1]][sa] != g[b[0]][b[1]][sb]]
