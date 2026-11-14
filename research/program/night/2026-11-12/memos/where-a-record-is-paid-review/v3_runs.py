#!/usr/bin/env python3
"""Referee script 3. One line per engine run in out/*.jsonl (and the memo's runs for reference).
Run: python3 -I v3_runs.py"""
import glob
import json
import os

H = os.path.dirname(os.path.abspath(__file__))
files = sorted(glob.glob(H + '/out/*.jsonl')) + sorted(glob.glob(H + '/../where-a-record-is-paid/out/e4-*.jsonl')) + sorted(glob.glob(H + '/../where-a-record-is-paid/out/e6-*3e8.jsonl'))
print('%-34s %5s %7s %8s %8s %9s %9s %9s | %9s %9s %9s %8s %8s' % (
    'run', 'sec', 'deepest', '<60', '60-200', 'late', 'arrivals', 'n/arr', 'late shr', 'legal shr', 'lgl ex-op', 'lgl deep', 'boards') + '   lgl late/arr')
for f in files:
    t = open(f).read().strip()
    if not t:
        continue
    a = json.loads(t)
    h, g = a['hist'], a['hist_legal']
    s = lambda c, x, y: sum(c[x:y])
    arr = h[201]
    deep_l = max([d for d in range(257) if g[d]] or [0])
    print('%-34s %5.0f %7d %8.2g %8.2g %9.2g %9d %9s | %9.3f %9.3f %9s %8d %8s' % (
        os.path.basename(f)[:-6][:34], a['seconds'], a['deepest'], s(h, 0, 60), s(h, 60, 201), s(h, 201, 257), arr,
        '%.2g' % (s(h, 60, 201) / arr) if arr else '-', s(h, 201, 257) / sum(h), s(g, 201, 257) / sum(g),
        '%.3f' % (s(g, 201, 257) / s(g, 60, 257)) if s(g, 60, 257) else '-', deep_l,
        '%d/%d' % (sum(a.get('done_all', [0])), sum(a.get('done_legal', [0])))) + ('   %.2g' % (s(g, 201, 257) / g[201]) if g[201] else '   -'))
