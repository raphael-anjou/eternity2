#!/usr/bin/env python3
"""Referee script 4. Legal tail profile per arrival, and total nodes per arrival, for the referee runs.
Run: python3 -I v4_profile.py"""
import glob
import json
import math
import os

H = os.path.dirname(os.path.abspath(__file__))
ld = lambda n: json.loads(open(os.path.join(H, 'out', n)).read())
a, b = ld('check-rebuilt-A-s1.jsonl'), ld('A-noclue-22-21-20-s1.jsonl')
print('binary rebuilt with the RV_NOISO switch reproduces run A seed 1 (hist and hist_legal):', a['hist'] == b['hist'] and a['hist_legal'] == b['hist_legal'])
print()
print('all nodes of the run per arrival, nodes per legal node, legal share of late nodes')
for f in sorted(glob.glob(H + '/out/[A-E]-*.jsonl')):
    r = json.loads(open(f).read())
    h, g = r['hist'], r['hist_legal']
    print('  %-30s %10s %6.2f %6.3f' % (os.path.basename(f)[:-6], '%.3g' % (sum(h) / h[201]) if h[201] else '-', sum(h) / sum(g), sum(g[201:]) / max(1, sum(h[201:]))))
print()
print('log10 nodes per arrival by depth, legal and all')
ds = (204, 210, 216, 222, 228, 231, 234, 236, 237, 238, 240, 243)
print('%-32s' % 'run' + ' '.join('%6d' % d for d in ds))
for n in ('B-noclue-5-20-7-s1', 'E-noiso-noclue-5-20-7-s1', 'B-noclue-5-20-7-s2', 'B-noclue-5-20-7-s3', 'A-noclue-22-21-20-s2', 'A-noclue-22-21-20-s3'):
    r = ld(n + '.jsonl')
    for nm, c in (('legal', r['hist_legal']), ('all', r['hist'])):
        print('%-32s' % (n[:25] + ' ' + nm) + ' '.join('%6s' % ('%.2f' % math.log10(c[d] / c[201]) if c[d] else '-') for d in ds))
