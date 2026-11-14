#!/usr/bin/env python3
"""The review's tail model against the measured histograms. Run: python3 -I a2_tail_model.py

Uses, read-only, the memo's model.py through the review's r3_sensitivity.py
(tail_tree: expected partial boards per depth below ONE perfect prefix of depth d0
whose remaining half-edges are spread evenly over ci interior and cr ring colours).

Part 1: survival from depth 161 to 201 and late nodes per arrival, model against
        measured, for the port (three interior colours privileged) and for the
        corrected quota (one ring + two interior).
Part 2: the measured profile after depth 201 against the model, per arrival.
Part 3: where the model says a finished board is lost: survival by stretch.
"""
import glob
import importlib.util
import json
import math
import os
import sys

sys.dont_write_bytecode = True
H = os.path.dirname(os.path.abspath(__file__))
R3 = os.path.join(H, '..', 'partial-match-regime-review', 'r3_sensitivity.py')
spec = importlib.util.spec_from_file_location('r3', R3)
r3 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r3)
LN10 = r3.LN10
lse = r3.lse
BW = r3.BW
T10 = '<repo>/research/experiments/real-puzzle-profile/results/m1'


def prof(d0, ul, ci, cr):
    lv = r3.tail_tree(d0, ul, ci, cr)
    return {d: lse(list(lv[d].values())) / LN10 for d in lv}


def load(path):
    return [json.loads(l) for l in open(path) if l.strip()][0]


print('== 1. Model: below one perfect depth-161 prefix, Blackwood list, log10 ==')
print('%-10s %12s %14s %16s %14s %10s' % ('colours', 'arrivals@201', 'nodes 162..200', 'late nodes total', 'late/arrival', 'boards'))
MODEL = {}
for ci, cr in ((17, 5), (16, 5), (16, 4), (15, 5), (15, 4), (14, 4), (13, 4)):
    p = prof(161, BW, ci, cr)
    pre = lse([p[d] * LN10 for d in range(162, 201)]) / LN10
    late = lse([p[d] * LN10 for d in range(201, 257)]) / LN10
    MODEL[(ci, cr)] = p
    print('%2d + %d     %12.2f %14.2f %16.2f %14.2f %10.2f' % (ci, cr, p[201], pre, late, late - p[201], p[256]))

print()
print('== 1b. Measured, per call at depth 161, log10 ==')
runs = [(T10 + '/record-1e11-s1.jsonl', 'port 22,21,20, five clues, 1e11 s1'), (T10 + '/record-1e11-s2.jsonl', 'port 22,21,20, five clues, 1e11 s2'),
        (T10 + '/record-noquota-s2.jsonl', 'port, quota off, five clues, 1e10 s2')]
for f in sorted(glob.glob(H + '/out/e2-official-noclue-*.jsonl')):
    runs.append((f, os.path.basename(f)[12:-6]))
for f, name in runs:
    h = load(f)['hist']
    if not h[201]:
        continue
    l = lambda x: math.log10(x) if x else float('-inf')
    print('%-44s arrivals@201 %6.2f  nodes 162..200 %6.2f  late %6.2f  late/arrival %5.2f   (h194/h161 %6.2f, before any clue-adjacent cell)' % (
        name, l(h[201] / h[161]), l(sum(h[162:201]) / h[161]), l(sum(h[201:]) / h[161]), l(sum(h[201:]) / h[201]), l(h[194] / h[161])))
print('model h194/h161: ' + '  '.join('%d+%d: %.2f' % (k[0], k[1], MODEL[k][194]) for k in MODEL))

print()
print('== 2. Profile after the first unlock, per arrival at depth 201: log10(nodes at d / arrivals) ==')
meas = {}
for f in sorted(glob.glob(H + '/out/e2-official-noclue-*.jsonl')):
    h = load(f)['hist']
    if h[201]:
        meas[os.path.basename(f)[12:-6]] = h
mods = {}
for ci, cr in ((17, 5), (15, 4), (14, 4)):
    p = prof(201, BW, ci, cr)
    mods['model %d+%d' % (ci, cr)] = p
print('depth ' + ' '.join('%22s' % k[-22:] for k in list(mods) + list(meas)))
for d in list(range(201, 245, 3)) + [248, 252, 256]:
    row = ['%22.2f' % mods[k][d] for k in mods]
    for k in meas:
        h = meas[k]
        row.append('%22s' % ('%.2f' % math.log10(h[d] / h[201]) if h[d] else '-'))
    print('%5d ' % d + ' '.join(row))

print()
print('== 3. Model, 15 + 4 colours, Blackwood list: where a finished board is lost (log10, per depth-161 prefix) ==')
p = MODEL[(15, 4)]
pk = max(range(162, 257), key=lambda d: p[d])
print('161 -> 201: %.2f ; 201 -> peak (depth %d): %+.2f ; peak -> 240: %+.2f ; 240 -> 256: %+.2f ; boards per prefix %.2f' % (
    p[201], pk, p[pk] - p[201], p[240] - p[pk], p[256] - p[240], p[256]))
tot = lse([p[d] * LN10 for d in range(162, 257)]) / LN10
print('tail nodes per prefix %.2f ; nodes per board %.2f ; boards per arrival at 201: %.2f' % (tot, tot - p[256], p[256] - p[201]))
for K in (10, 9, 8):
    ul = BW[10 - K:]
    q = prof(161, ul, 15, 4)
    tot = lse([q[d] * LN10 for d in range(162, 257)]) / LN10
    print('  K = %d (score %d), list = last %d entries: tail nodes %.2f, boards %.2f, nodes per board %.2f' % (K, 480 - K, K, tot, q[256], tot - q[256]))
