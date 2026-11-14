#!/usr/bin/env python3
"""All nodes against legal nodes, per run. Run: python3 -I a7_legal_hist.py
Legal = every placed piece shows grey exactly on the rim sides of its cell (hist_legal,
engine-copy after patch 2). A node is legal only if its parent is."""
import glob
import json
import math
import os

H = os.path.dirname(os.path.abspath(__file__))
for f in sorted(glob.glob(H + '/out/e4-*.jsonl')) + sorted(glob.glob(H + '/out/e5-*.jsonl')) + sorted(glob.glob(H + '/out/e6-*.jsonl')):
    a = json.loads(open(f).read())
    h, g = a['hist'], a['hist_legal']
    print(os.path.basename(f), 'seconds', a['seconds'], 'completions', a['completions'])
    for nm, c in (('all', h), ('legal', g)):
        s = lambda x, y: sum(c[x:y])
        print('  %-5s total %.3g | depth <60 %.3g | 60-160 %.3g | 161-200 %.3g | >=201 %.3g | late share %.3f, with the opening left out %.3f | per arrival: nodes at 60..200 %.3g, late %.3g | deepest %d' % (
            nm, sum(c), s(0, 60), s(60, 161), s(161, 201), s(201, 257), s(201, 257) / sum(c), s(201, 257) / s(60, 257),
            s(60, 201) / c[201], s(201, 257) / c[201], max(d for d in range(257) if c[d])))
    print('  legal share of nodes at depth < 201: %.4f ; at depth >= 201: %.4f ; total nodes per legal node: %.2f' % (
        sum(g[:201]) / sum(h[:201]), sum(g[201:]) / sum(h[201:]), sum(h) / sum(g)))
    r = lambda c: 'row 12 (201-207) %.3g, row 13 (208-223) %.3g, row 14 (224-239) %.3g, row 15 (240-255) %.3g' % (sum(c[201:208]), sum(c[208:224]), sum(c[224:240]), sum(c[240:256]))
    print('  late nodes by row, all: ' + r(h))
    print('  late nodes by row, legal: ' + r(g))
    print('  log10 nodes per arrival, legal / all: ' + ' '.join('%d: %.2f / %.2f' % (d, math.log10(g[d] / g[201]) if g[d] else float('-inf'), math.log10(h[d] / h[201]) if h[d] else float('-inf')) for d in (204, 210, 216, 222, 228, 231, 234, 236, 237, 240, 243)))
    if 'done_legal' in a:
        print('  completed boards: all %d, legal %d' % (sum(a['done_all']), sum(a['done_legal'])))
