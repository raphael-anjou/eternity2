#!/usr/bin/env python3
"""Confirmation statistics and reading (PREREG.md stage 2, PREREG_AMENDMENTS.md 1).
Writes results/analysis.txt and results/summary.json from results/raw/conf-*.jsonl."""
import json, math, os, random, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from jobs_keys import kid  # noqa: E402

H = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
RAW = os.path.join(H, 'results', 'raw')
T13 = os.path.join(H, '..', 'quota-keying', 'results', 'raw')
sa = json.load(open(os.path.join(H, 'results', 'screen-arms.json')))
arms = [kid('D' if a == 'D' else tuple(a)) for a in sa['arms']]
best = kid(tuple(sa['best']))
top5 = [kid(tuple(k)) for k in sa['top5']]
rnd = [kid(tuple(k)) for k in sa['random']]
SEEDS = list(range(401, 421))
B, BSEED = 10000, 20261113


def load(a, s):
    p = os.path.join(RAW, f'conf-{a}-s{s}.jsonl')
    return json.loads(open(p).readline()) if os.path.exists(p) and os.path.getsize(p) else None


R = {(a, s): load(a, s) for a in arms for s in SEEDS}
done = [s for s in SEEDS if all(R[(a, s)] is not None for a in arms)]
out = [f'# confirmation: openings complete for all {len(arms)} arms: {len(done)} of 20 ({done})',
       f'arms: {arms}; best screened {best}; top5 {top5}; random {rnd}', '']


def late(r, d0=201):
    return sum(r['hist_legal'][d0:])


def dlegal(r):
    return max([d for d, v in enumerate(r['hist_legal']) if v] + [0])


def flagged(r):
    return r['nodes_legal'] < 100


# per arm, per opening: (legal late, nodes, seconds, flagged)
V = {a: [(late(R[(a, s)]), R[(a, s)]['nodes'], R[(a, s)]['seconds'], flagged(R[(a, s)])) for s in done] for a in arms}


def Y(a, idx, per='node', drop=False):
    num = den = 0.0
    for i in idx:
        l, n, sec, f = V[a][i]
        if drop and f:
            continue
        num += l
        den += n if per == 'node' else sec
    return num / den if den else 0.0


def ratio(x, y):
    if x == 0 and y == 0:
        return 1.0
    return math.inf if y == 0 else x / y


def boot(fnum, fden):
    rng = random.Random(BSEED)
    n = len(done)
    full = list(range(n))
    pt = ratio(fnum(full), fden(full))
    vals = []
    for _ in range(B):
        idx = [rng.randrange(n) for _ in range(n)]
        vals.append(ratio(fnum(idx), fden(idx)))
    vals.sort()
    return pt, vals[int(0.025 * B)], vals[int(0.975 * B) - 1]


def f(x):
    return 'inf' if x == math.inf else f'{x:.3g}'


summary = {'done': done, 'arms': {}}
hdr = '| arm | role | Y per node | Y per second | legal late nodes | arrivals | legal >= 230 | deepest legal (max) | deepest (min-max) | never left opening | no node past 200 | flagged (<100 legal) | seconds (sum) | per-seed y min / median / max |'
out += ['## Per arm (pooled over complete openings, cap 1e9 nodes per run)', hdr, '|' + '---|' * 14]
for a in arms:
    rs = [R[(a, s)] for s in done]
    role = []
    if a in top5: role.append(f'screen #{top5.index(a) + 1}')
    if a == 'D': role.append('default')
    if a == '5-7-20': role.append('published')
    if a in rnd: role.append(f'random R{rnd.index(a) + 1}')
    ys = sorted(late(r) / r['nodes'] for r in rs)
    full = list(range(len(done)))
    d = dict(Y=Y(a, full), Ysec=Y(a, full, 'sec'), late=sum(late(r) for r in rs), arr=sum(r['hist'][201] for r in rs),
             l230=sum(late(r, 230) for r in rs), dl=max(dlegal(r) for r in rs), dmin=min(r['deepest'] for r in rs),
             dmax=max(r['deepest'] for r in rs), never=sum(r['deepest'] <= 60 for r in rs),
             nopast=sum(sum(r['hist'][201:]) == 0 for r in rs), flagged=sum(flagged(r) for r in rs),
             sec=sum(r['seconds'] for r in rs), per_seed={s: late(R[(a, s)]) / R[(a, s)]['nodes'] for s in done})
    summary['arms'][a] = d
    med = ys[len(ys) // 2] if ys else 0
    out.append(f"| {a} | {', '.join(role)} | {f(d['Y'])} | {f(d['Ysec'])} | {d['late']} | {d['arr']} | {d['l230']} | {d['dl']} | {d['dmin']}-{d['dmax']} | {d['never']} | {d['nopast']} | {d['flagged']} | {d['sec']:.0f} | {f(ys[0]) if ys else '-'} / {f(med)} / {f(ys[-1]) if ys else '-'} |")

out += ['', '## Per-seed y (legal late per node) and host', '| seed | host | ' + ' | '.join(arms) + ' |', '|' + '---|' * (len(arms) + 2)]
HOSTS = ['x86', 'm1', 'x86', 'local', 'x86', 'm1', 'x86', 'x86', 'm1', 'local',
         'x86', 'm1', 'x86', 'local', 'x86', 'm1', 'x86', 'x86', 'm1', 'local']
for s in done:
    out.append(f'| {s} | {HOSTS[s - 401]} | ' + ' | '.join(f(late(R[(a, s)]) / R[(a, s)]['nodes']) for a in arms) + ' |')

out += ['', '## Ratios to D, paired bootstrap over openings (95%, 10,000 resamples, seed 20261113)',
        '| arm | per node | per second | per node without flagged runs | per second without flagged runs |', '|---|---|---|---|---|']
ratios = {}
for a in arms:
    if a == 'D':
        continue
    cells = []
    for per in ('node', 'sec'):
        for drop in (False, True):
            r = boot(lambda i, a=a, per=per, drop=drop: Y(a, i, per, drop), lambda i, per=per, drop=drop: Y('D', i, per, drop))
            ratios[(a, per, drop)] = r
    out.append(f"| {a} | " + ' | '.join(f'{f(r[0])} [{f(r[1])}, {f(r[2])}]' for r in
                                        (ratios[(a, 'node', False)], ratios[(a, 'sec', False)], ratios[(a, 'node', True)], ratios[(a, 'sec', True)])) + ' |')

out += ['', '## Best screened against best random key (max over R1..R5 inside each resample)']
GR = {}
for per in ('node', 'sec'):
    for drop in (False, True):
        GR[(per, drop)] = boot(lambda i, per=per, drop=drop: Y(best, i, per, drop),
                               lambda i, per=per, drop=drop: max(Y(r, i, per, drop) for r in rnd))
        out.append(f"G_R per {per}{' without flagged' if drop else ''}: {f(GR[(per, drop)][0])} [{f(GR[(per, drop)][1])}, {f(GR[(per, drop)][2])}]")
sec2 = boot(lambda i: max(Y(t, i) for t in top5), lambda i: max(Y(r, i) for r in rnd))
out.append(f'secondary (no reading): best of top5 / best random, per node: {f(sec2[0])} [{f(sec2[1])}, {f(sec2[2])}]')

GD = ratios[(best, 'node', False)]
GRn = GR[('node', False)]
out += ['', '## Reading (PREREG.md, amendment 1)',
        f'G_D = {f(GD[0])} [{f(GD[1])}, {f(GD[2])}]; G_R = {f(GRn[0])} [{f(GRn[1])}, {f(GRn[2])}]']
if GD[0] >= 3 and GRn[0] >= 3 and GD[1] > 1.5 and GRn[1] > 1.5:
    reading = 'keying is a lever for five clues'
    if ratios[(best, 'sec', False)][0] < 1 or GR[('sec', False)][0] < 1:
        reading = 'not a lever: wins per node, loses per second'
elif GRn[1] <= 1 <= GRn[2]:
    reading = 'keying is not a lever'
else:
    reading = 'undecided at this budget'
uninf = len(done) < 10 or all(summary['arms'][a]['late'] == 0 for a in arms)
out.append(f'READING: {reading}' + (' (uninformative: fewer than 10 openings or no legal late node)' if uninf else '') +
           '; at 1e9 nodes per run, one attempt per opening, on this port, with a row scan from the top-left corner without border interleaving.')

# Track 13 centre-clue fidelity runs at the same budget (Track 13's rows)
c1 = [json.loads(open(os.path.join(T13, f'c1-P-bwf-s{s}.jsonl')).readline()) for s in range(101, 111)]
out += ['', '## Against Track 13 centre-clue fidelity runs (c1-P-bwf, seeds 101..110, 1e9 nodes)',
        f"c1-P-bwf: Y {sum(late(r) for r in c1) / sum(r['nodes'] for r in c1):.3g}, deepest legal {min(dlegal(r) for r in c1)}..{max(dlegal(r) for r in c1)}, legal >= 230 {sum(late(r, 230) for r in c1)}"]
mx = max(summary['arms'][a]['dl'] for a in arms)
out.append(f'deepest legal over all five-clue arms: {mx}')
summary['ratios_D'] = {f'{a}|{per}|{drop}': v for (a, per, drop), v in ratios.items()}
summary['G_R'] = {f'{per}|{drop}': v for (per, drop), v in GR.items()}
summary['reading'] = reading
txt = '\n'.join(out) + '\n'
open(os.path.join(H, 'results', 'analysis.txt'), 'w').write(txt)
json.dump(summary, open(os.path.join(H, 'results', 'summary.json'), 'w'), default=str)
print(txt)
