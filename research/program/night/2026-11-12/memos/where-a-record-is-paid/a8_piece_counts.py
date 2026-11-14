#!/usr/bin/env python3
"""Piece counts behind the feasibility remarks. Run: python3 -I a8_piece_counts.py
How many interior pieces carry none of k given interior colours, and how many border
pieces carry none of 1 or 2 given ring colours, on the official set (data/official.csv)."""
import itertools
import os

H = os.path.dirname(os.path.abspath(__file__))
L = [l for l in open(os.path.join(H, 'data', 'official.csv')).read().splitlines() if l.strip()]
pieces = []
for l in L[1:]:
    c = l.split(',')
    pieces.append([0 if int(x, 2) == 65535 else int(x, 2) for x in c[:4]])
interior = [set(p) for p in pieces if 0 not in p]
border = [set(p) - {0} for p in pieces if p.count(0) == 1]
corner = [set(p) - {0} for p in pieces if p.count(0) == 2]
print('pieces: %d interior, %d border, %d corner' % (len(interior), len(border), len(corner)))
INNER = range(6, 23)
for k in (2, 3, 4, 5, 6):
    vals = [sum(1 for s in interior if not (s & set(cs))) for cs in itertools.combinations(INNER, k)]
    print('interior pieces free of %d given interior colours: min %d, median %d, max %d over %d choices' % (k, min(vals), sorted(vals)[len(vals) // 2], max(vals), len(vals)))
for k in (1, 2):
    vals = [sum(1 for s in border if not (s & set(cs))) for cs in itertools.combinations(range(1, 6), k)]
    print('border pieces free of %d given ring colours: min %d, median %d, max %d' % (k, min(vals), sorted(vals)[len(vals) // 2], max(vals)))
print('interior cells left at depth 160: 70 ; at depth 176: 56 ; at depth 192: 42 ; at depth 201: 34 (row scan)')
print('border cells left at depth 201: 19 border and 2 corners ; border cells already placed: 37 border and 2 corners')
for trip in ((3, 12, 13), (4, 16, 17), (22, 21, 20)):
    n = sum(1 for p in pieces if set(p) & set(trip))
    sides = sum(p.count(c) for p in pieces for c in trip)
    print('privileged %s: %d sides on %d pieces' % (trip, sides, n))
