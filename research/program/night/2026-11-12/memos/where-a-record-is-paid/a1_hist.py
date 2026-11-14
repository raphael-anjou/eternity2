#!/usr/bin/env python3
"""Where the nodes are, per run. Reads the committed Track 10 histograms (read-only)
and the runs of this memo (out/). Run: python3 -I a1_hist.py

Depth d = number of cells already filled when the search is called (0..255), as in
Track 10. hist[d] counts calls at depth d, before the quota test. An "arrival" is a
call at depth 201 (the first break unlock); it carries no break.
"""
import glob
import json
import math
import os

H = os.path.dirname(os.path.abspath(__file__))
T10 = '<repo>/research/experiments/real-puzzle-profile/results/m1'


def rows(path):
    return [json.loads(l) for l in open(path) if l.strip()]


def seg(h, a, b):
    return sum(h[a:b])


def line(name, h):
    t = sum(h)
    arr = h[201]
    opening = seg(h, 0, 60)
    mid = seg(h, 60, 161)
    pre = seg(h, 161, 201)
    late = seg(h, 201, 257)
    deepest = max(d for d in range(257) if h[d])
    pk = max(range(257), key=lambda d: h[d])
    f = lambda x: '%.2e' % x if x else '0'
    return '%-40s %8s | open(<60) %5.1f%% mid(60-160) %5.1f%% 161-200 %5.1f%% late(>=201) %8.4f%% | arrivals %6d nodes/arrival %9s late/arrival %9s (60-200)/arrival %9s | h201/h161 %8s | peak %3d deepest %3d' % (
        name, f(t), 100 * opening / t, 100 * mid / t, 100 * pre / t, 100 * late / t, arr,
        f(t / arr) if arr else '-', f(late / arr) if arr else '-', f((mid + pre) / arr) if arr else '-',
        f(h[201] / h[161]) if h[161] else '-', pk, deepest)


print('== A. Track 10 port as committed: privileged colours 22,21,20 (three interior colours, 150 sides), five clues ==')
for f in sorted(glob.glob(T10 + '/record-s*.jsonl')) + sorted(glob.glob(T10 + '/record-1e11-s*.jsonl')) + sorted(glob.glob(T10 + '/record-noquota-s*.jsonl')):
    for r in rows(f):
        print(line(os.path.basename(f)[:-6], r['hist']))
        for n, h in r['checkpoints']:
            if n in (10 ** 9, 10 ** 10) and n < r['nodes']:
                print(line('   checkpoint %.0e' % n, h))

print()
print('== B. Same binary (engine-copy, search unchanged), privileged = one ring colour + two interior colours (124 sides), no clue ==')
for f in sorted(glob.glob(H + '/out/e2-official-noclue-*.jsonl')):
    for r in rows(f):
        print(line(os.path.basename(f)[12:-6], r['hist']))
        for n, h in r['checkpoints']:
            print(line('   checkpoint %.0e' % n, h))

print()
print('== C. Same, five clues on the board ==')
for f in sorted(glob.glob(H + '/out/e2-official-priv*.jsonl')):
    for r in rows(f):
        print(line(os.path.basename(f)[3:-6], r['hist']))

print()
print('== D. Runs that got through the opening: per-depth profile after the opening ==')
for f in sorted(glob.glob(H + '/out/e2-official-noclue-*.jsonl')) + sorted(glob.glob(H + '/out/e3-*.jsonl')):
    for r in rows(f):
        h = r['hist']
        if not h[201]:
            continue
        print(os.path.basename(f), 'unlock', r['unlock'], 'completions', r['completions'], 'best', r['best_complete_score'])
        first_multi = next(d for d in range(256) if h[d] > 1)
        # the prefix the whole run stays under: deepest d with hist == 1
        fixed = max(d for d in range(256) if h[d] == 1 and all(h[e] <= 1 for e in range(d + 1)))
        print('   never backtracked above depth', fixed, '(hist = 1 down to there); first depth revisited', first_multi)
        print('   log10 hist every 8 cells from 96: ' + ' '.join('%d:%.2f' % (d, math.log10(h[d])) for d in range(96, 257, 8) if h[d]))
        print('   log10 hist 196..: ' + ' '.join('%d:%.2f' % (d, math.log10(h[d])) for d in range(196, 257) if h[d]))
        late = seg(h, 201, 257)
        post = seg(h, 60, 257)
        print('   late share of all nodes %.3f ; of nodes at depth >= 60 (opening excluded) %.3f' % (late / sum(h), late / post))
        print('   per arrival at 201: %.3g nodes at 60..200, %.3g nodes at >= 201' % ((seg(h, 60, 201)) / h[201], late / h[201]))
        print('   per call at depth 160: %.3g nodes at 161..200, %.3g arrivals, %.3g late nodes ; nodes at 60..160 per call at 160: %.3g' % (
            seg(h, 161, 201) / h[160], h[201] / h[160], late / h[160], seg(h, 60, 161) / h[160]))
