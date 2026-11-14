#!/usr/bin/env python3
"""Arrivals at depth 201 on the real pieces: is the tail predictable from the pool?
Run: python3 -I a3_probe.py

Input: out/probe201-*.jsonl, one row per arrival (call at depth 201, no break yet):
  pool[c]  sides of colour c among the unused pieces (c = 0 grey, 1..5 ring, 6..22 interior)
  front[c] colours the already placed cells show to the empty cells
  sub      nodes of the subtree below the arrival under the run's unlock list
  deep     deepest depth reached in that subtree ; brk breaks used there ; done completions
Predictor (first moment, the memo's statistic): u_c = pool[c] + front[c] is the number of
half-edges of colour c still to be paired. G_b = log10 P(a uniform perfect matching of the
interior half-edges has at most b two-coloured pairs), exact (model.interior_dist), plus the
same for the ring with each ring colour split evenly between its two facings (approximation).
The number of piece arrangements is the same for every arrival, so G_b ranks the first-moment
expected number of completions with at most b breaks anywhere in the tail.
"""
import glob
import importlib.util
import json
import math
import os
import sys

sys.dont_write_bytecode = True
H = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location('memo_model', os.path.join(H, '..', 'partial-match-regime', 'model.py'))
M = importlib.util.module_from_spec(spec)
spec.loader.exec_module(M)
RING = range(1, 6)
INNER = range(6, 23)
_cache = {}


def cum_log10(D, bs):
    tot = sum(D)
    out = {}
    c = 0
    for k, d in enumerate(D):
        c += d
        if k in bs:
            out[k] = math.log10(c / tot) if c else float('-inf')
    return out


def G(row, bs=(0, 4, 10)):
    u = tuple(row['pool'][c] + row['front'][c] for c in INNER)
    r = tuple(row['pool'][c] + row['front'][c] for c in RING)
    key = (u, r)
    if key in _cache:
        return _cache[key]
    if sum(u) % 2:
        raise SystemExit('odd interior total %s' % (u,))
    gi = cum_log10(M.interior_dist(list(u)), bs)
    n = sum(r) // 2
    cw = [x // 2 for x in r]
    ccw = [x - x // 2 for x in r]
    # make both facings sum to n
    i = 0
    while sum(cw) < n:
        cw[i % 5] += 1
        ccw[i % 5] -= 1
        i += 1
    gr = cum_log10(M.ring_dist(cw, ccw), bs)
    odd = sum(1 for x in u if x % 2)
    out = {'gi': gi, 'gr': gr, 'odd': odd, 'u': u, 'r': r}
    _cache[key] = out
    return out


def ranks(v):
    idx = sorted(range(len(v)), key=lambda i: v[i])
    r = [0.0] * len(v)
    i = 0
    while i < len(v):
        j = i
        while j + 1 < len(v) and v[idx[j + 1]] == v[idx[i]]:
            j += 1
        for k in range(i, j + 1):
            r[idx[k]] = (i + j) / 2.0
        i = j + 1
    return r


def pearson(a, b):
    n = len(a)
    ma, mb = sum(a) / n, sum(b) / n
    sa = math.sqrt(sum((x - ma) ** 2 for x in a))
    sb = math.sqrt(sum((y - mb) ** 2 for y in b))
    return sum((x - ma) * (y - mb) for x, y in zip(a, b)) / (sa * sb) if sa and sb else float('nan')


def spearman(a, b):
    return pearson(ranks(a), ranks(b))


def sd(v):
    m = sum(v) / len(v)
    return math.sqrt(sum((x - m) ** 2 for x in v) / max(1, len(v) - 1))


def quant(v, q):
    s = sorted(v)
    return s[min(len(s) - 1, int(q * len(s)))]


for f in sorted(glob.glob(H + '/out/probe201-*.jsonl')):
    rows = [json.loads(l) for l in open(f) if l.strip()]
    if len(rows) < 10:
        continue
    name = os.path.basename(f)[9:-6]
    gs = [G(r) for r in rows]
    g0 = [g['gi'][0] + g['gr'][0] for g in gs]
    g10 = [g['gi'][10] for g in gs]
    g4 = [g['gi'][4] for g in gs]
    sub = [math.log10(r['sub']) for r in rows]
    deep = [r['deep'] for r in rows]
    odd = [g['odd'] for g in gs]
    print('== %s: %d arrivals ==' % (name, len(rows)))
    ex = rows[0]
    print('   pool sides per colour, first arrival: ring %s interior %s' % ([ex['pool'][c] for c in RING], [ex['pool'][c] for c in INNER]))
    zero = [sum(1 for c in INNER if g['u'][INNER.index(c)] == 0) for g in gs]
    zr = [sum(1 for x in g['r'] if x == 0) for g in gs]
    print('   interior colours with no half-edge left: median %d (min %d, max %d) ; ring colours with none left: median %d' % (
        quant(zero, 0.5), min(zero), max(zero), quant(zr, 0.5)))
    print('   interior half-edges to pair: %d ; ring: %d ; colours with an odd count: median %d (min %d, max %d)' % (
        sum(gs[0]['u']), sum(gs[0]['r']), quant(odd, 0.5), min(odd), max(odd)))
    print('   subtree nodes: median %.0f, mean %.0f, max %.0f ; share of all subtree nodes in the top 10%% of arrivals: %.2f' % (
        10 ** quant(sub, 0.5), sum(r['sub'] for r in rows) / len(rows), max(r['sub'] for r in rows),
        sum(sorted((r['sub'] for r in rows), reverse=True)[:max(1, len(rows) // 10)]) / sum(r['sub'] for r in rows)))
    print('   deepest depth below an arrival: median %d, 90th %d, max %d ; arrivals reaching >= 240: %d ; completions %d' % (
        quant(deep, 0.5), quant(deep, 0.9), max(deep), sum(1 for d in deep if d >= 240), sum(r['done'] for r in rows)))
    print('   predictor spread over arrivals (decades): G0 sd %.2f range %.2f..%.2f ; G4 sd %.2f ; G10 sd %.2f range %.2f..%.2f' % (
        sd(g0), min(g0), max(g0), sd(g4), sd(g10), min(g10), max(g10)))
    print('   Spearman: G0~log(sub) %.2f ; G10~log(sub) %.2f ; G0~deep %.2f ; G10~deep %.2f ; odd~log(sub) %.2f ; log(sub)~deep %.2f' % (
        spearman(g0, sub), spearman(g10, sub), spearman(g0, deep), spearman(g10, deep), spearman(odd, sub), spearman(sub, deep)))
    # split by time: first half against second half of the run (arrivals are time ordered)
    h = len(rows) // 2
    print('   first half / second half of the run: mean log10 sub %.2f / %.2f ; mean G10 %.2f / %.2f ; mean deep %.1f / %.1f' % (
        sum(sub[:h]) / h, sum(sub[h:]) / (len(rows) - h), sum(g10[:h]) / h, sum(g10[h:]) / (len(rows) - h),
        sum(deep[:h]) / h, sum(deep[h:]) / (len(rows) - h)))
    # top fifth by predictor against the rest
    order = sorted(range(len(rows)), key=lambda i: -g10[i])
    top = order[:max(1, len(rows) // 5)]
    rest = order[len(top):]
    mt = sum(rows[i]['sub'] for i in top) / len(top)
    mr = sum(rows[i]['sub'] for i in rest) / len(rest)
    dt = sum(1 for i in top if deep[i] >= 236) / len(top)
    dr = sum(1 for i in rest if deep[i] >= 236) / len(rest)
    print('   top fifth by G10 against the rest: mean subtree %.0f against %.0f ; share reaching depth >= 236: %.3f against %.3f' % (mt, mr, dt, dr))
    print()
