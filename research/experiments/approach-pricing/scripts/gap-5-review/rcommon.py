"""Second analysis helpers for gap 5. Stdlib only. Written independently of gap-5/e2common.py.
Tiles are (top, right, bottom, left); 0 = grey. Rotation r sends (t,r,b,l) to (l,t,r,b) r times
(the engine's convention, crates/core/src/piece.rs). Clues are (tile index, row, col, tuple as placed)."""
import os, random

N = 16
CSV = os.environ.get("E2_CSV", "research/experiments/"
                     "single-core-benchmark/engine/data/puzzles/size_16_official_eternity.csv")


def rot(t, r):
    r %= 4
    return t[4 - r:] + t[:4 - r] if r else t


def rots(t):
    return [rot(t, r) for r in range(4)]


def load():
    rows = [l.strip() for l in open(CSV) if l.strip()]
    P, clues = [], []
    for i, l in enumerate(rows[1:]):
        c = l.split(",")
        e = tuple(0 if int(x, 2) == 65535 else int(x, 2) for x in c[:4])
        P.append(e)
        x, y, r = int(c[4]), int(c[5]), int(c[6])
        if x or y or r:
            clues.append((i, y, x, rot(e, r)))
    assert len(P) == 256
    return P, clues


def paint_board(rng, n, pool_of):
    """Solved n x n board. pool_of(kind, r, c) names a pool key for each joint; each key's pool is
    a list of colours, shuffled and dealt. kind 'H' = joint between (r,c),(r,c+1); 'V' = (r,c),(r+1,c)."""
    raise NotImplementedError


def tiles_from_joints(n, H, V):
    out = []
    for r in range(n):
        for c in range(n):
            t = V[r - 1][c] if r > 0 else 0
            b = V[r][c] if r < n - 1 else 0
            l = H[r][c - 1] if c > 0 else 0
            rt = H[r][c] if c < n - 1 else 0
            out.append((t, rt, b, l))
    return out


# ---------- exact integer linear algebra ----------
def hermite_basis(rows):
    """Row-style integer elimination. Returns a list of rows spanning the same lattice (echelon)."""
    rows = [list(r) for r in rows if any(r)]
    basis = []
    ncol = len(rows[0]) if rows else 0
    for c in range(ncol):
        cand = [r for r in rows if r[c]]
        rest = [r for r in rows if not r[c]]
        while len(cand) > 1:
            cand.sort(key=lambda r: abs(r[c]))
            p = cand[0]
            nxt = [p]
            for r in cand[1:]:
                q = r[c] // p[c]
                r2 = [x - q * y for x, y in zip(r, p)]
                if r2[c]:
                    nxt.append(r2)
                elif any(r2):
                    rest.append(r2)
            cand = nxt
        if cand:
            basis.append(cand[0])
        rows = rest
    return basis


def smith_diagonal(rows):
    """Elementary divisors of the lattice spanned by integer rows. Pure python, exact."""
    M = hermite_basis(rows)
    M = [r[:] for r in M]
    m = len(M)
    n = len(M[0]) if M else 0
    diag = []
    top = 0
    while top < m and top < n:
        # find the nonzero entry of least absolute value in the remaining block
        best = None
        for i in range(top, m):
            for j in range(top, n):
                v = M[i][j]
                if v and (best is None or abs(v) < abs(M[best[0]][best[1]])):
                    best = (i, j)
        if best is None:
            break
        i, j = best
        M[top], M[i] = M[i], M[top]
        for r in M:
            r[top], r[j] = r[j], r[top]
        while True:
            p = M[top][top]
            dirty = False
            for i in range(top + 1, m):
                if M[i][top]:
                    q = M[i][top] // p
                    M[i] = [a - q * b for a, b in zip(M[i], M[top])]
                    if M[i][top]:
                        M[top], M[i] = M[i], M[top]
                        dirty = True
                        break
            if dirty:
                continue
            for j in range(top + 1, n):
                if M[top][j]:
                    q = M[top][j] // p
                    for r in M:
                        r[j] -= q * r[top]
                    if M[top][j]:
                        for r in M:
                            r[top], r[j] = r[j], r[top]
                        dirty = True
                        break
            if dirty:
                continue
            # divisibility
            bad = None
            for i in range(top + 1, m):
                for j in range(top + 1, n):
                    if M[i][j] % p:
                        bad = i
                        break
                if bad is not None:
                    break
            if bad is None:
                break
            M[top] = [a + b for a, b in zip(M[top], M[bad])]
        diag.append(abs(M[top][top]))
        top += 1
    return diag


def rank_mod(rows, p):
    """Rank over GF(p) of a list of integer rows, pure python with dict pivots."""
    piv = {}
    for r in rows:
        r = [x % p for x in r]
        for c in range(len(r)):
            if r[c]:
                if c in piv:
                    f = r[c]
                    b = piv[c]
                    r = [(x - f * y) % p for x, y in zip(r, b)]
                else:
                    inv = pow(r[c], -1, p)
                    piv[c] = [(x * inv) % p for x in r]
                    break
    return len(piv)
