#!/usr/bin/env python3
"""M2: independent exact count on the reduced rings (Python stdlib only).

For each corner assignment sigma, each side is a type-word, i.e. an Euler
trail of its Parikh multigraph P from head(sigma_j) to tail(sigma_{j+1}).
The number of type-words with Parikh matrix P is, by BEST and the
matrix-tree theorem,
    kappa_t(P) * prod_{v != t present} (out_P(v)-1)! * out_P(t)! / prod P_ab!
The four side tables are convolved over all decompositions of the edge-type
multiset M. N = prod M_ab! * sum_sigma W(sigma). Shares no code with M1/M3.

usage: m2_best.py IN.jsonl OUT.jsonl [workers]
"""
import itertools
import json
import sys
import time
from fractions import Fraction
from math import factorial, prod
from multiprocessing import Pool


def det(m):
    m = [[Fraction(x) for x in row] for row in m]
    n = len(m)
    d = Fraction(1)
    for c in range(n):
        p = next((r for r in range(c, n) if m[r][c] != 0), None)
        if p is None:
            return Fraction(0)
        if p != c:
            m[c], m[p] = m[p], m[c]
            d = -d
        d *= m[c][c]
        for r in range(c + 1, n):
            f = m[r][c] / m[c][c]
            if f:
                for k in range(c, n):
                    m[r][k] -= f * m[c][k]
    return d


def words(types, P, s, t, nv):
    """Number of distinct type-words that are Euler trails s -> t of P."""
    out = [0] * nv
    inn = [0] * nv
    for (a, b), k in zip(types, P):
        out[a] += k
        inn[b] += k
    for v in range(nv):
        if out[v] - inn[v] != (v == s) - (v == t):
            return 0
    present = [v for v in range(nv) if out[v] + inn[v] > 0]
    if s not in present or t not in present:
        return 0
    rest = [v for v in present if v != t]
    idx = {v: i for i, v in enumerate(rest)}
    lap = [[0] * len(rest) for _ in rest]
    for (a, b), k in zip(types, P):
        if a == t or k == 0:
            continue
        lap[idx[a]][idx[a]] += k
        if b != t and a != b:
            lap[idx[a]][idx[b]] -= k
        elif a == b:
            lap[idx[a]][idx[a]] -= k
    kappa = det(lap) if rest else Fraction(1)
    assert kappa.denominator == 1
    kappa = int(kappa)
    if kappa == 0:
        return 0
    lab = kappa * factorial(out[t]) * prod(factorial(out[v] - 1) for v in rest)
    den = prod(factorial(k) for k in P)
    assert lab % den == 0
    return lab // den


def vectors(mult, total):
    def rec(i, left):
        if i == len(mult):
            if left == 0:
                yield ()
            return
        for k in range(min(mult[i], left) + 1):
            for r in rec(i + 1, left - k):
                yield (k,) + r
    return list(rec(0, total))


def one(line):
    inst = json.loads(line)
    t0 = time.time()
    nv, L = inst["nv"], inst["side"]
    types = [tuple(x) for x in inst["types"]]
    mult = tuple(inst["mult"])
    corners = [tuple(x) for x in inst["corners"]]
    vecs = vectors(mult, L)
    table = {}

    def side(s, t):
        if (s, t) not in table:
            d = {}
            for P in vecs:
                w = words(types, P, s, t, nv)
                if w:
                    d[P] = w
            table[(s, t)] = d
        return table[(s, t)]

    def pair(sa, ta, sb, tb):
        h = {}
        A, B = side(sa, ta), side(sb, tb)
        for P0, w0 in A.items():
            for P1, w1 in B.items():
                Q = tuple(x + y for x, y in zip(P0, P1))
                if all(q <= m for q, m in zip(Q, mult)):
                    h[Q] = h.get(Q, 0) + w0 * w1
        return h

    pairs = {}
    W = []
    for sig in itertools.permutations(range(4)):
        c = [corners[i] for i in sig]
        st = [(c[j][1], c[(j + 1) % 4][0]) for j in range(4)]
        k01, k23 = st[0] + st[1], st[2] + st[3]
        for k in (k01, k23):
            if k not in pairs:
                pairs[k] = pair(*k)
        h01, h23 = pairs[k01], pairs[k23]
        w = 0
        for Q, a in h01.items():
            R = tuple(m - q for m, q in zip(mult, Q))
            b = h23.get(R)
            if b:
                w += a * b
        W.append(w)
    pf = prod(factorial(m) for m in mult)
    return json.dumps({
        "n": inst["n"], "colors": inst["colors"], "seed": inst["seed"],
        "sigmas": [list(s) for s in itertools.permutations(range(4))],
        "W_sigma": [str(w) for w in W],
        "N_M2": str(pf * sum(W)),
        "M2_seconds": round(time.time() - t0, 3),
    })


if __name__ == "__main__":
    src, dst = sys.argv[1], sys.argv[2]
    workers = int(sys.argv[3]) if len(sys.argv) > 3 else 8
    lines = [l for l in open(src) if l.strip()]
    with Pool(workers) as pool:
        res = pool.map(one, lines, chunksize=1)
    with open(dst, "w") as f:
        for r in res:
            f.write(r + "\n")
