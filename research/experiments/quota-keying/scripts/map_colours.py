#!/usr/bin/env python3
"""Map Joshua Blackwood's colour labels to this repo's colour numbers.

Input: his Util.cs (github.com/jblackwood345/EternityII_Solver, Get_Pieces())
given as argv[1]; this repo's piece file data/official.csv. Finds every
colour bijection (grey fixed) under which his 256 pieces, as cyclic edge
tuples up to rotation (both chiralities tried), equal this repo's 256 pieces
as a multiset. Prints the bijection(s) and his heuristic_sides in repo numbers.
"""
import os, re, sys
from collections import Counter
H = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
src = open(sys.argv[1], encoding='utf-8-sig').read()
his = [tuple(int(m.group(k)) for k in ('t', 'r', 'b', 'l')) for m in re.finditer(
    r'TopSide = (?P<t>\d+), RightSide = (?P<r>\d+), BottomSide = (?P<b>\d+), LeftSide = (?P<l>\d+)', src)]
hs = [int(x) for x in re.search(r'heuristic_sides = new List<int>\(\) \{([^}]*)\}', src).group(1).split(',')]
assert len(his) == 256, len(his)
lines = open(os.path.join(H, 'data', 'official.csv')).read().split()[1:]
def col(s):
    v = int(s, 2)
    return 0 if v == 65535 else v
ours = [tuple(col(x) for x in l.split(',')[:4]) for l in lines]
def canon(e):
    return min(tuple(e[(i + k) % 4] for i in range(4)) for k in range(4))
def solve(his_p):
    target = Counter(canon(e) for e in ours)
    cols_h = sorted({c for e in his_p for c in e} - {0})
    cnt_h = Counter(c for e in his_p for c in e)
    cnt_o = Counter(c for e in ours for c in e)
    # candidate images by side count and by co-occurrence with grey (ring colours)
    def prof(pieces, cnt):
        p = {}
        for c in cnt:
            p[c] = (cnt[c], sum(1 for e in pieces if c in e and 0 in e), sum(1 for e in pieces if c in e and list(e).count(0) == 2))
        return p
    ph, po = prof(his_p, cnt_h), prof(ours, cnt_o)
    cand = {c: [d for d in po if d != 0 and po[d] == ph[c]] for c in cols_h}
    order = sorted(cols_h, key=lambda c: len(cand[c]))
    sols = []
    def ok_partial(m):
        # every his piece whose colours are all mapped must map to an existing repo piece with enough multiplicity
        mapped = Counter()
        for e in his_p:
            if all(c == 0 or c in m for c in e):
                mapped[canon(tuple(0 if c == 0 else m[c] for c in e))] += 1
        return all(target[k] >= v for k, v in mapped.items())
    def rec(i, m, used):
        if len(sols) > 3:
            return
        if i == len(order):
            if Counter(canon(tuple(0 if c == 0 else m[c] for c in e)) for e in his_p) == target:
                sols.append(dict(m))
            return
        c = order[i]
        for d in cand[c]:
            if d in used:
                continue
            m[c] = d; used.add(d)
            if ok_partial(m):
                rec(i + 1, m, used)
            del m[c]; used.discard(d)
    rec(0, {}, set())
    return sols
for name, pieces in (('same chirality', his), ('mirrored', [(t, l, b, r) for (t, r, b, l) in his])):
    sols = solve(pieces)
    print(f'{name}: {len(sols)} bijection(s)')
    for s in sols:
        print('  his -> repo:', ', '.join(f'{a}->{s[a]}' for a in sorted(s)))
        print('  heuristic_sides', hs, '-> repo', [s[c] for c in hs])
