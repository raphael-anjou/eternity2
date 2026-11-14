#!/usr/bin/env python3
"""Referee script 5. Run: python3 -I r5_gate_and_469.py (seconds, stdlib only)

 a  Claim D: the best SINGLE gate (all ten breaks free from depth g) against the
    cumulative list, so that the 22 decades are split into what any late rule
    earns and what the list's shape earns.
 b  Claim C, absolute level: nodes per 469 board (11 breaks) in the fewer-colour
    tail, for several prefix costs, against the reported 10^15.
"""
import os
import sys
import importlib.util
from math import log

sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location('r3', os.path.join(HERE, 'r3_sensitivity.py'))
r3 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r3)
LN10 = r3.LN10

print('a. single gate at depth g, ten breaks free after it (17 colours, whole tree): log10 nodes per board')
best = None
for g in range(197, 250, 4):
    q = r3.nodes_per_board([g] * 10)[0]
    best = min(best, (q, g)) if best else (q, g)
    print('   g = %d: %.2f' % (g, q))
print('   best single gate: depth %d, 10^%.2f; anywhere 10^45.44; Blackwood list 10^23.34; model-optimal list 10^22.58' % (best[1], best[0]))

print('b. nodes per 469 board (11 breaks), tail below a perfect depth-160 prefix')
for ci, cr in ((17, 5), (15, 4)):
    for name, ul in (('196 + list', [196] + r3.BW), ('list + 241', r3.BW + [241])):
        print('   %2d + %d colours, %-10s: ' % (ci, cr, name) + '  '.join(
            'c0=10^%d: 10^%.2f (late share %.2g)' % (c, r3.tail_summary(160, ul, ci, cr, c * LN10)['q'], r3.tail_summary(160, ul, ci, cr, c * LN10)['late_share'])
            for c in (0, 3, 6)))
print('   reported: about 10^15 nodes for the first 469 (memo, section 2.5)')
