#!/usr/bin/env python3
"""Referee script 1. Colour census of the official piece file and the map between
Blackwood's colour labels and the repo's.

Run: python3 -I v1_colour_map.py <repo csv> <path to Blackwood's Util.cs>
Util.cs is https://github.com/jblackwood345/EternityII_Solver (EternityII_Solver/Util.cs),
downloaded to a scratch folder, not vendored here.
"""
import re
import sys
from collections import Counter

csv_path, util_path = sys.argv[1], sys.argv[2]
L = [l for l in open(csv_path).read().splitlines() if l.strip()]
repo = []
for l in L[1:]:
    c = l.split(',')
    repo.append(tuple(0 if int(x, 2) == 65535 else int(x, 2) for x in c[:4]))  # top, right, bottom, left
src = open(util_path, encoding='utf-8-sig').read()
bw = {}
for m in re.finditer(r'PieceNumber = (\d+), TopSide = (\d+), RightSide = (\d+), BottomSide = (\d+), LeftSide = (\d+)', src):
    n, t, r, b, l = map(int, m.groups())
    bw[n] = (t, r, b, l)
assert len(repo) == 256 and len(bw) == 256

# 1. census of the repo file
sides = Counter(c for p in repo for c in p)
ring = set()
for p in repo:
    if 0 in p:
        for i in range(4):
            # a ring colour sits next to a grey side on a border or corner piece
            if p[i] != 0 and (p[(i + 1) % 4] == 0 or p[(i - 1) % 4] == 0):
                ring.add(p[i])
inner_on_border = set(c for p in repo if 0 in p for c in p if c and c not in ring)
print('repo file: %d pieces ; grey sides %d' % (len(repo), sides[0]))
print('ring colours (sit beside a grey side): %s, sides each %s' % (sorted(ring), [sides[c] for c in sorted(ring)]))
inner = sorted(c for c in sides if c and c not in ring)
print('interior colours: %s' % inner)
print('  sides each: %s' % [sides[c] for c in inner])
print('  ring colour on an interior piece: %s' % any(c in ring for p in repo if 0 not in p for c in p))

# 2. the map. Try: same piece order, same rotation up to a quarter turn per piece.
def rots(p):
    return [p[i:] + p[:i] for i in range(4)]

mp = {}
ok = True
bad = 0
for i, p in enumerate(repo):
    q = bw[i + 1]
    found = False
    for r in rots(q):
        cand = dict(mp)
        good = True
        for a, b in zip(r, p):
            if cand.setdefault(a, b) != b:
                good = False
                break
        if good and len(set(cand.values())) == len(cand):
            found = True
            trial = cand
            break
    if not found:
        bad += 1
    else:
        mp = trial
print()
print('piece i of the repo file against Blackwood piece i: %d of 256 inconsistent with one colour bijection' % bad)
print('Blackwood label -> repo label: %s' % ', '.join('%d->%d' % (a, mp[a]) for a in sorted(mp)))
inv = {v: k for k, v in mp.items()}
for name, trip in (('Blackwood heuristic_sides {13, 16, 10}', [mp[13], mp[16], mp[10]]), ):
    print('%s = repo colours %s ; classes %s ; sides %s, total %d' % (
        name, trip, ['ring' if c in ring else 'interior' for c in trip], [sides[c] for c in trip], sum(sides[c] for c in trip)))
print('Blackwood side_edges {1,5,9,13,17} -> repo %s' % sorted(mp[c] for c in (1, 5, 9, 13, 17)))
for trip in ((13, 16, 10), (22, 21, 20), (3, 12, 13), (4, 16, 17), (12, 14, 15)):
    npieces = sum(1 for p in repo if set(p) & set(trip))
    print('repo triple %s: classes %s, %d sides on %d pieces ; in Blackwood labels %s' % (
        trip, ['ring' if c in ring else 'interior' for c in trip], sum(sides[c] for c in trip), npieces, [inv[c] for c in trip]))
# centre piece
print('piece 139: Blackwood %s, repo %s' % (bw[139], repo[138]))
# pieces per privileged-side count, for the record keying in repo labels
trip = [mp[13], mp[16], mp[10]]
cnt = Counter(sum(1 for c in p if c in trip) for p in repo)
print('record keying %s: pieces by number of privileged sides %s ; border/corner pieces carrying the ring one: %d' % (
    trip, dict(sorted(cnt.items())), sum(1 for p in repo if 0 in p and trip[0] in p)))
