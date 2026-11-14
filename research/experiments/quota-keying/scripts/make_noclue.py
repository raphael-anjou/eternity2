#!/usr/bin/env python3
"""Write data/official.csv (copy of the Track 10 piece file) and
data/official-noclue.csv (same, the three hint columns zeroed) and
data/official-centre.csv (only the centre clue kept: piece 139, x=7, y=8)."""
import os
H = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
src = os.path.join(H, '..', 'single-core-benchmark', 'engine', 'data', 'puzzles', 'size_16_official_eternity.csv')
lines = open(src).read().splitlines()
open(os.path.join(H, 'data', 'official.csv'), 'w').write('\n'.join(lines) + '\n')
out = [lines[0]]
for l in lines[1:]:
    c = l.split(',')
    if len(c) >= 7:
        c[4:7] = ['0', '0', '0']
    out.append(','.join(c))
open(os.path.join(H, 'data', 'official-noclue.csv'), 'w').write('\n'.join(out) + '\n')
cen = [lines[0]]
for l in lines[1:]:
    c = l.split(',')
    if len(c) >= 7 and c[4:6] != ['7', '8']:
        c[4:7] = ['0', '0', '0']
    cen.append(','.join(c))
open(os.path.join(H, 'data', 'official-centre.csv'), 'w').write('\n'.join(cen) + '\n')
