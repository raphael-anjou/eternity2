#!/usr/bin/env python3
"""Screening statistic (PREREG.md stage 1). Writes results/screen.txt and
results/screen-arms.json (the confirmation arms).

    python3 -I scripts/analyze_screen.py [--openings 1,2,3]

Without --openings, uses the openings complete for all 681 keyings (the
preregistered reduction at the 04:30 stop).
"""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from jobs_keys import KEYS, RANDOM, kid  # noqa: E402

H = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
RAW = os.path.join(H, 'results', 'raw')
ALL = KEYS + ['D']


def load(k, s):
    p = os.path.join(RAW, f'scr-{kid(k)}-s{s}.jsonl')
    if not os.path.exists(p) or os.path.getsize(p) == 0:
        return None
    return json.loads(open(p).readline())


def deepest_legal(r):
    return max([d for d, v in enumerate(r['hist_legal']) if v] + [0])


rows = {(kid(k), s): load(k, s) for k in ALL for s in (1, 2, 3)}
complete = [s for s in (1, 2, 3) if all(rows[(kid(k), s)] is not None for k in ALL)]
if '--openings' in sys.argv:
    S = [int(x) for x in sys.argv[sys.argv.index('--openings') + 1].split(',')]
else:
    S = complete
done = sum(r is not None for r in rows.values())
out = [f'# screening: {done} of {len(rows)} runs present; complete openings {complete}; used {S}']
stats = []
for i, k in enumerate(ALL):
    rs = [rows[(kid(k), s)] for s in S]
    late = sum(sum(r['hist_legal'][201:]) for r in rs)
    n = sum(r['nodes'] for r in rs)
    sec = sum(r['seconds'] for r in rs)
    dl = max(deepest_legal(r) for r in rs)
    l230 = sum(sum(r['hist_legal'][230:]) for r in rs)
    arr = sum(r['hist'][201] for r in rs)
    flagged = sum(r['nodes_legal'] < 100 for r in rs)
    stats.append(dict(k=kid(k), idx=i, Ys=late / n, Ysec=late / sec if sec else 0, late=late, nodes=n, sec=sec,
                      dl=dl, l230=l230, arr=arr, flagged=flagged, per_seed=[sum(r['hist_legal'][201:]) / r['nodes'] for r in rs]))
cand = sorted([x for x in stats if x['k'] != 'D'], key=lambda x: (-x['Ys'], -x['dl'], x['idx']))
for r, x in enumerate(cand, 1):
    x['rank'] = r
Dx = next(x for x in stats if x['k'] == 'D')
Dx['rank'] = 1 + sum(1 for x in cand if (-x['Ys'], -x['dl']) < (-Dx['Ys'], -Dx['dl']))
pub = next(x for x in cand if x['k'] == '5-7-20')
nz = sum(1 for x in cand if x['late'] > 0)
out.append(f'keyings (of 680) with any legal node at depth >= 201: {nz}; with deepest legal >= 201: {sum(1 for x in cand if x["dl"] >= 201)}')
hdr = '| rank | keying (r, a, b) | Ys legal late per node | per second | legal late nodes | arrivals | legal >= 230 | deepest legal | flagged runs | per-seed Ys |'
out += ['', hdr, '|' + '---|' * 10]
fmt = lambda x: f"| {x['rank']} | {x['k']} | {x['Ys']:.3g} | {x['Ysec']:.3g} | {x['late']} | {x['arr']} | {x['l230']} | {x['dl']} | {x['flagged']} | {' '.join('%.2g' % v for v in x['per_seed'])} |"
for x in cand[:20]:
    out.append(fmt(x))
out += ['', 'Default and published:', fmt(Dx) + ' (D, ranked among the 680)', fmt(pub) + ' (P)']
out += ['', 'Random control keys:'] + [fmt(next(x for x in cand if x['k'] == kid(k))) for k in RANDOM]
by_ring = {}
for x in cand:
    by_ring.setdefault(x['k'].split('-')[0], []).append(x)
out += ['', 'By ring colour: keyings with legal late nodes, best rank, sum of legal late nodes']
for rc in sorted(by_ring):
    xs = by_ring[rc]
    out.append(f"ring {rc}: {sum(1 for x in xs if x['late'] > 0)} of {len(xs)}, best rank {min(x['rank'] for x in xs)}, late {sum(x['late'] for x in xs)}")
dls = sorted(x['dl'] for x in cand)
out.append(f'deepest legal over the 680: min {dls[0]}, median {dls[len(dls) // 2]}, max {dls[-1]}')
top5 = [tuple(map(int, x['k'].split('-'))) for x in cand[:5]]
arms = []
for k in top5 + ['D', (5, 7, 20)] + RANDOM:
    k = 'D' if k == 'D' else list(k)
    if k not in arms:
        arms.append(k)
out += ['', f'confirmation arms ({len(arms)}): top5 {[kid(k) for k in top5]}, D, P 5-7-20, random {[kid(k) for k in RANDOM]}']
txt = '\n'.join(out) + '\n'
open(os.path.join(H, 'results', 'screen.txt'), 'w').write(txt)
json.dump({'openings': S, 'top5': [list(k) for k in top5], 'best': list(top5[0]), 'arms': arms,
           'random': [list(k) for k in RANDOM], 'stats': stats}, open(os.path.join(H, 'results', 'screen-arms.json'), 'w'))
print(txt)
