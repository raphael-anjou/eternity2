"""Second analysis script 1b. The k x k covers (period-k weightings), recomputed, and the one extra
invariant the memo left unidentified, named and verified.

Coordinates: h(a,b,c) = top side of a cell of class (a,b) in Z_k^2 carrying colour c;
v(a,b,c) = its left side. A tile (t,r,b,l) at class (a,b), in any rotation, gives
  h(a+1,b,bot) - h(a,b,top) + v(a,b+1,right) - v(a,b,left).
Named candidate (k even): weight (-1)^(a+b) g(c) on h and -(-1)^(a+b) g(c) on v, with
g(grey) = +1, g(frame colour) = 0, g(interior colour) = -1.
Run: uv run --with python-flint python r1b_cover_invariant.py [k ...]   (default 1 2 4)
"""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from flint import nmod_mat, fmpz_mat
from rcommon import load, rots

NC = 23
BIG = 2147483629


def gens(tiles, k):
    def idx(ty, a, b, c):
        return ((ty * k + a % k) * k + b % k) * NC + c
    out = set()
    for t in tiles:
        for (top, right, bot, left) in set(rots(t)):
            for a in range(k):
                for b in range(k):
                    v = [0] * (2 * k * k * NC)
                    v[idx(0, a + 1, b, bot)] += 1; v[idx(0, a, b, top)] -= 1
                    v[idx(1, a, b + 1, right)] += 1; v[idx(1, a, b, left)] -= 1
                    out.add(tuple(v))
    return sorted(out), idx


def rank_p(rows, p):
    m, n = len(rows), len(rows[0])
    return nmod_mat(m, n, [x % p for r in rows for x in r], p).rank()


if __name__ == "__main__":
    P, _ = load()
    g = [1] + [0] * 5 + [-1] * 17
    ks = [int(x) for x in sys.argv[1:]] or [1, 2, 4]
    for k in ks:
        G, idx = gens(P, k)
        dim = 2 * k * k * NC
        rq = rank_p(G, BIG)
        tors = {p: rq - rank_p(G, p) for p in (2, 3, 5, 7, 11, 13)}
        # colour-blind weightings: f_h, f_v on Z_k^2 with f_h(a+1,b)-f_h(a,b)+f_v(a,b+1)-f_v(a,b)=0
        eqs = []
        for a in range(k):
            for b in range(k):
                e = [0] * (2 * k * k)
                e[((a + 1) % k) * k + b] += 1; e[a * k + b] -= 1
                e[k * k + a * k + (b + 1) % k] += 1; e[k * k + a * k + b] -= 1
                eqs.append(e)
        blind = 2 * k * k - fmpz_mat(eqs).rank()
        line = f"k={k}: coordinates {dim}, rank {rq}, corank {dim - rq}, colour-blind weightings {blind}, rank drop mod p {tors}"
        if k % 2 == 0:
            w = [0] * dim
            for a in range(k):
                for b in range(k):
                    for c in range(NC):
                        s = (-1) ** (a + b)
                        w[idx(0, a, b, c)] = s * g[c]
                        w[idx(1, a, b, c)] = -s * g[c]
            ok = all(sum(x * y for x, y in zip(w, v)) == 0 for v in G)
            line += f" | named checkerboard weighting vanishes on every tile: {ok}; so corank = colour-blind + 1 is fully named: {ok and dim - rq == blind + 1}"
        print(line)
