#!/usr/bin/env python3
"""Arrivals at depth 201, legal boards only. Run: python3 -I a5_probe_legal.py

Input: out/probeL201-*.jsonl (engine-copy after patch 4). Per arrival:
  subl  = legal nodes below it (legal: every placed piece shows grey exactly on the rim)
  deepl = deepest legal depth below it
  pool, front as in a3_probe.py.
Questions: (1) how unequal are arrivals, (2) does the pool statistic G10 rank them,
(3) natural experiment: arrivals where one more interior colour happens to be used up.
"""
import glob
import importlib.util
import json
import math
import os
import sys

sys.dont_write_bytecode = True
H = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location('a3', os.path.join(H, 'a3_probe.py'))
# a3_probe runs its report at import; silence it
import io
import contextlib
a3 = importlib.util.module_from_spec(spec)
with contextlib.redirect_stdout(io.StringIO()):
    spec.loader.exec_module(a3)
G, spearman, sd, quant = a3.G, a3.spearman, a3.sd, a3.quant
INNER = list(range(6, 23))
RING = list(range(1, 6))

for f in sorted(glob.glob(H + '/out/probeL201-*.jsonl')):
    rows = [json.loads(l) for l in open(f) if l.strip()]
    name = os.path.basename(f)[10:-6]
    n = len(rows)
    gs = [G(r) for r in rows]
    g10 = [g['gi'][10] + g['gr'][10] for g in gs]
    g10i = [g['gi'][10] for g in gs]
    subl = [r['subl'] for r in rows]
    lsub = [math.log10(max(1, x)) for x in subl]
    deepl = [r['deepl'] for r in rows]
    print('== %s: %d arrivals ==' % (name, n))
    tot_all = sum(r['sub'] for r in rows)
    tot_leg = sum(subl)
    print('   late nodes below these arrivals: all %.3g, legal %.3g (%.1f%%) ; legal per arrival: mean %.0f, median %.0f, max %.0f' % (
        tot_all, tot_leg, 100.0 * tot_leg / tot_all, tot_leg / n, quant(subl, 0.5), max(subl)))
    top = sorted(subl, reverse=True)[:max(1, n // 10)]
    print('   share of legal late nodes under the top 10%% of arrivals: %.2f ; sd of log10(legal subtree): %.2f decades' % (sum(top) / tot_leg, sd(lsub)))
    print('   deepest legal depth: median %d, 90th %d, 99th %d, max %d ; arrivals with legal depth >= 224 (row 14 reached): %d, >= 230: %d, >= 234: %d' % (
        quant(deepl, 0.5), quant(deepl, 0.9), quant(deepl, 0.99), max(deepl), sum(1 for d in deepl if d >= 224), sum(1 for d in deepl if d >= 230), sum(1 for d in deepl if d >= 234)))
    print('   G10 (interior + ring) sd %.2f decades ; Spearman G10~log(subl) %.2f ; G10~deepl %.2f ; interior part only: %.2f, %.2f' % (
        sd(g10), spearman(g10, lsub), spearman(g10, deepl), spearman(g10i, lsub), spearman(g10i, deepl)))
    order = sorted(range(n), key=lambda i: -g10[i])
    for frac in (0.5, 0.2, 0.1):
        k = max(1, int(n * frac))
        t, rest = order[:k], order[k:]
        hit = lambda idx, D: sum(1 for i in idx if deepl[i] >= D) / max(1, len(idx))
        print('   top %2.0f%% by G10: mean legal subtree %.0f against %.0f ; legal depth >= 224: %.3f against %.3f ; >= 230: %.3f against %.3f ; share of all depth>=230 arrivals caught: %.2f' % (
            100 * frac, sum(subl[i] for i in t) / len(t), sum(subl[i] for i in rest) / max(1, len(rest)),
            hit(t, 224), hit(rest, 224), hit(t, 230), hit(rest, 230),
            sum(1 for i in t if deepl[i] >= 230) / max(1, sum(1 for d in deepl if d >= 230))))
    # natural experiment
    print('   by number of interior colours with no half-edge left at the arrival (pool + frontier):')
    for z in range(0, 5):
        idx = [i for i in range(n) if sum(1 for x in gs[i]['u'] if x == 0) == z]
        if not idx:
            continue
        print('      %d colours gone: %4d arrivals ; mean log10 legal subtree %.2f ; mean legal subtree %.0f ; mean legal depth %.1f ; legal depth >= 224: %.3f ; >= 230: %.3f ; mean G10 %.2f' % (
            z, len(idx), sum(lsub[i] for i in idx) / len(idx), sum(subl[i] for i in idx) / len(idx), sum(deepl[i] for i in idx) / len(idx),
            sum(1 for i in idx if deepl[i] >= 224) / len(idx), sum(1 for i in idx if deepl[i] >= 230) / len(idx), sum(g10[i] for i in idx) / len(idx)))
    print('   by number of ring colours with no half-edge left:')
    for z in range(0, 4):
        idx = [i for i in range(n) if sum(1 for x in gs[i]['r'] if x == 0) == z]
        if not idx:
            continue
        print('      %d ring colours gone: %4d arrivals ; mean log10 legal subtree %.2f ; mean legal depth %.1f ; legal depth >= 224: %.3f ; >= 230: %.3f' % (
            z, len(idx), sum(lsub[i] for i in idx) / len(idx), sum(deepl[i] for i in idx) / len(idx),
            sum(1 for i in idx if deepl[i] >= 224) / len(idx), sum(1 for i in idx if deepl[i] >= 230) / len(idx)))
    # effective number of colours of the tail pool: 1 / sum p_c^2
    eff = []
    for g in gs:
        t = sum(g['u'])
        eff.append(t * t / sum(x * x for x in g['u']))
    print('   effective number of interior colours in the tail (1 / sum of squared shares): median %.2f, min %.2f, max %.2f ; Spearman with legal depth: %.2f' % (
        quant(eff, 0.5), min(eff), max(eff), spearman([-e for e in eff], deepl)))
    print()
