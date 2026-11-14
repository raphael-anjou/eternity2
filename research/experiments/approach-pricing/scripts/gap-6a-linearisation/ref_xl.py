"""Naive reference XL, written independently of xl.cpp, for small systems (3x3 boards).

Every generator (each linear equation, each forbidden pair product) is multiplied by every
multilinear monomial such that degree(generator) + degree(monomial) <= D; products are reduced
with x^2 = x only; no column is removed. Rows are eliminated mod p with Python integers
(sparse rows as dicts, leading term = largest monomial for the order (degree, variables)).
Prints the rank of the linearised system and the forced variables as JSON.
Usage: python3 -I ref_xl.py system.txt D p
"""
import sys, json
from itertools import combinations

def read(path):
    tok = open(path).read().split(); it = iter(tok)
    n, E = int(next(it)), int(next(it))
    sol = [int(next(it)) for _ in range(n)]
    eqs = []
    for _ in range(E):
        k = int(next(it)); eqs.append([int(next(it)) for _ in range(k)])
    nF = int(next(it)); F = [(int(next(it)), int(next(it))) for _ in range(nF)]
    return n, eqs, F, sol

def main():
    path, D, p = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
    n, eqs, F, sol = read(path)
    gens = [({frozenset([a]): 1 for a in e} | {frozenset(): p - 1}, 1) for e in eqs]
    gens += [({frozenset([a, b]): 1}, 2) for a, b in F]
    monos = {d: [frozenset(c) for c in combinations(range(n), d)] for d in range(D + 1)}
    key = lambda m: (len(m), tuple(sorted(m)))
    piv = {}            # leading monomial -> row dict (leading coefficient 1)
    def reduce(row):
        while row:
            lead = max(row, key=key)
            r = piv.get(lead)
            if r is None: return row, lead
            c = row[lead]
            for m, v in r.items():
                x = (row.get(m, 0) - c * v) % p
                if x: row[m] = x
                else: row.pop(m, None)
        return None, None
    for g, dg in gens:
        for d in range(D - dg + 1):
            for m in monos[d]:
                row = {}
                for t, v in g.items():
                    u = t | m
                    x = (row.get(u, 0) + v) % p
                    if x: row[u] = x
                    else: row.pop(u, None)
                row, lead = reduce(row)
                if row:
                    inv = pow(row[lead], p - 2, p) if p > 2 else 1
                    piv[lead] = {m: v * inv % p for m, v in row.items()}
    rank = len(piv)
    # linear consequences: rows whose leading monomial has degree <= 1; reduce them among themselves
    lin = [dict(r) for lead, r in piv.items() if len(lead) <= 1]
    cols = [frozenset([a]) for a in range(n)] + [frozenset()]
    M = [[r.get(c, 0) for c in cols] for r in lin]
    prow = {}; r0 = 0
    for c in range(n + 1):
        k = next((i for i in range(r0, len(M)) if M[i][c]), None)
        if k is None: continue
        M[r0], M[k] = M[k], M[r0]
        inv = pow(M[r0][c], p - 2, p) if p > 2 else 1
        M[r0] = [v * inv % p for v in M[r0]]
        for i in range(len(M)):
            if i != r0 and M[i][c]:
                f = M[i][c]; M[i] = [(x - f * y) % p for x, y in zip(M[i], M[r0])]
        prow[c] = r0; r0 += 1
    forced = []
    for c in range(n):
        if c in prow and all(M[prow[c]][j] == 0 for j in range(n) if j != c):
            forced.append([c, (-M[prow[c]][n]) % p])
    print(json.dumps(dict(n=n, D=D, p=p, rank_full=rank, forced=sorted(forced), inconsistent=(n in prow))))
main()
