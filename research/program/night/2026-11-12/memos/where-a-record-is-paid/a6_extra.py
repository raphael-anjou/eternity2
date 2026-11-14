#!/usr/bin/env python3
"""Small extra numbers. Run: python3 -I a6_extra.py
 1  Model price per board for the colour counts that bracket the measured stretch
    (survival from depth 161 to 201), with the measured head cost c0 = 10^2.
 2  Privileged sides still off the board at each arrival (depth 201), per run.
 3  Reported compute of the 470 and the 469, in nodes, for the comparison of levels.
"""
import glob
import importlib.util
import json
import math
import os
import sys

sys.dont_write_bytecode = True
H = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location('r3', os.path.join(H, '..', 'partial-match-regime-review', 'r3_sensitivity.py'))
r3 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r3)
LN10 = r3.LN10
BW = r3.BW

print('== 1. Model, tail below a perfect depth-160 prefix, c0 = 10^2 nodes per prefix: log10 nodes per board (late share, log10 boards per prefix) ==')
for ci, cr in ((17, 5), (16, 5), (16, 4), (15, 5), (15, 4), (14, 4)):
    out = []
    for K in (10, 9):
        s = r3.tail_summary(160, BW[10 - K:], ci, cr, 2 * LN10)
        out.append('K=%d: %.2f (late %.3g, boards %.2f)' % (K, s['q'], s['late_share'], s['boards']))
    print('   %2d + %d : %s' % (ci, cr, ' ; '.join(out)))

print()
print('== 2. Privileged sides not yet on the board at an arrival (pool only) ==')
for f in sorted(glob.glob(H + '/out/probe201-*.jsonl')) + sorted(glob.glob(H + '/out/probeL201-*.jsonl')):
    rows = [json.loads(l) for l in open(f) if l.strip()]
    if len(rows) < 10:
        continue
    name = os.path.basename(f)[:-6]
    if 'port' in name:
        priv = (22, 21, 20)
    else:
        priv = tuple(int(x) for x in name.split('priv')[1].split('-')[:3])
    left = [sum(r['pool'][c] for c in priv) for r in rows]
    per = [[r['pool'][c] for r in rows] for c in priv]
    total = {1: 24, 2: 24, 3: 24, 4: 24, 5: 24}
    print('   %-44s privileged %s: sides left, median %d (min %d, max %d) ; per colour medians %s ; on board (engine counter) median %d' % (
        name, priv, sorted(left)[len(left) // 2], min(left), max(left), [sorted(p)[len(p) // 2] for p in per], sorted(r['priv'] for r in rows)[len(rows) // 2]))

print()
print('== 3. Reported compute, converted (argued: rates are not in the sources) ==')
for name, cores, days in (('470, Blackwood 2021: about a month on a 32-core Threadripper 3970X', 32, 30), ('469, McGavin 2020: a few days on about 200 cores', 200, 3)):
    for rate in (1.9e7, 1e8):
        n = cores * days * 86400 * rate
        print('   %-70s at %.1e nodes/s per core: 10^%.1f nodes' % (name, rate, math.log10(n)))

print()
print('== 4. Frame-last order: the head is a complete 14x14 interior (row scan, no border), 17 colours, independent joints ==')
from math import lgamma, log
best = (-1e9, 0)
tot = []
for d in range(1, 197):
    rows_full, col = divmod(d, 14)
    # joints closed after d cells of a 14-wide strip: left joints + top joints
    left = rows_full * 13 + max(0, col - 1)
    top = max(0, d - 14)
    v = (lgamma(197) - lgamma(197 - d) + d * log(4) - (left + top) * log(17)) / LN10
    tot.append(v)
    best = max(best, (v, d))
m = max(tot)
s = m + math.log10(sum(10 ** (x - m) for x in tot))
print('   widest level 10^%.1f at %d cells ; whole tree 10^%.1f ; complete interiors 10^%.1f ; nodes per complete interior 10^%.1f' % (best[0], best[1], s, tot[-1], s - tot[-1]))
print('   each complete interior then has 10^-34.6 perfect frames (17 + 5 colours, a4 P5)')

print()
print('== 5. Last 16 cells: the rim row against an interior row closed from both sides (15 + 4 colours), log10 perfect fills ==')
rim = (lgamma(15) + log(2) - 15 * log(4) - 14 * log(15)) / LN10
mid = (lgamma(15) + 14 * log(4) + log(2) - (14 + 14 + 15) * log(15) - 4 * log(4)) / LN10
print('   rim row: %.1f ; interior meeting row: %.1f ; difference %.1f decades' % (rim, mid, rim - mid))
