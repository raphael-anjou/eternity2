#!/usr/bin/env python3
"""Analysis of PREREG.md: per-arm statistics, ratios to D-off-Q0, readings R1 to R4.
Reads results/raw/test-*.jsonl (and calib-*.jsonl for the cost table), writes
results/analysis.txt and results/summary.json.  Usage: python3 -I scripts/analyze.py"""
import glob, json, os, random, statistics, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.argv = [sys.argv[0], 'none']
from jobs import ARMS, name

H = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
RAW = os.path.join(H, 'results', 'raw')
REF = 'D-off-Q0'


def load(prefix):
    R = {}
    for f in glob.glob(os.path.join(RAW, prefix + '-*.jsonl')):
        b = os.path.basename(f)[len(prefix) + 1:-6]
        arm, s = b.rsplit('-s', 1)
        R[(arm, int(s))] = json.loads(open(f).readline())
    return R


def late(r):
    return sum(r['hist_legal'][201:])


def dlegal(r):
    return max([d for d, v in enumerate(r['hist_legal']) if v] + [0])


def stranded(r):
    return r['nodes_legal'] < 100


def f(x):
    if x == float('inf'):
        return 'inf'
    return f'{x:.3g}'


R = load('test')
arms = [name(*a) for a in ARMS if any(k[0] == name(*a) for k in R)]
seeds = sorted({s for (_, s) in R})
complete = [s for s in seeds if all((a, s) in R for a in arms)]
out = [f'# five-clue-quota analysis (stage test rows only)', '',
       f'arms {len(arms)}, seeds with every arm finished: {len(complete)} ({complete[0] if complete else "-"}..{complete[-1] if complete else "-"})',
       f'rows present: {len(R)}', '']
S = {}
for a in arms:
    rs = [R[(a, s)] for s in complete]
    L = sum(late(r) for r in rs); N = sum(r['nodes'] for r in rs); T = sum(r['seconds'] for r in rs)
    dls = [dlegal(r) for r in rs]
    S[a] = dict(late=L, nodes=N, secs=T, Y=L / N if N else 0, Ysec=L / T if T else 0,
                stranded=sum(stranded(r) for r in rs), n=len(rs), pos=sum(late(r) > 0 for r in rs),
                dl_med=statistics.median(dls) if dls else 0, dl_min=min(dls) if dls else 0, dl_max=max(dls) if dls else 0,
                never=sum(r['deepest'] <= 60 for r in rs),
                per_seed={s: dict(late=late(R[(a, s)]), nodes=R[(a, s)]['nodes'], nodes_legal=R[(a, s)]['nodes_legal'],
                                  seconds=R[(a, s)]['seconds'], deepest=R[(a, s)]['deepest'], dlegal=dlegal(R[(a, s)]))
                          for s in complete})

rng = random.Random(20261113)
B = 10000
idx = [[rng.randrange(len(complete)) for _ in complete] for _ in range(B)] if complete else []


def ratio(a, per):
    def pooled(arm, ii):
        L = sum(S[arm]['per_seed'][complete[i]]['late'] for i in ii)
        D = sum(S[arm]['per_seed'][complete[i]]['nodes' if per == 'node' else 'seconds'] for i in ii)
        return L / D
    def r(ii):
        x, y = pooled(a, ii), pooled(REF, ii)
        if y == 0:
            return 1.0 if x == 0 else float('inf')
        return x / y
    pt = r(range(len(complete)))
    bs = sorted(r(ii) for ii in idx)
    return pt, bs[int(0.025 * B)], bs[int(0.975 * B) - 1]


out += ['## Per arm (pooled over openings; every node charged)', '',
        '| arm | runs | stranded (<100 legal) | positive openings (legal node >= 201) | legal nodes >= 201 | Y per node | Y per second | ratio per node to D-off-Q0 [95%] | ratio per second [95%] | deepest legal median (min..max) | never past depth 60 | seconds (sum) |',
        '|---|---|---|---|---|---|---|---|---|---|---|---|']
RAT = {}
for a in arms:
    d = S[a]
    rn, rs_ = ratio(a, 'node'), ratio(a, 'sec')
    RAT[a] = dict(node=rn, sec=rs_)
    out.append(f"| {a} | {d['n']} | {d['stranded']} ({d['stranded'] / d['n']:.0%}) | {d['pos']} | {d['late']} | {f(d['Y'])} | {f(d['Ysec'])} |"
               f" {f(rn[0])} [{f(rn[1])}, {f(rn[2])}] | {f(rs_[0])} [{f(rs_[1])}, {f(rs_[2])}] | {d['dl_med']:g} ({d['dl_min']}..{d['dl_max']}) | {d['never']} | {d['secs']:.0f} |")

out += ['', '## Per seed: legal nodes >= 201 / deepest legal depth / legal nodes spent (M = 1e6) / seconds', '']
out.append('| arm | ' + ' | '.join(str(s) for s in complete) + ' |')
out.append('|---' * (len(complete) + 1) + '|')
for a in arms:
    ps = S[a]['per_seed']
    out.append(f'| {a} | ' + ' | '.join(f"{ps[s]['late']}/{ps[s]['dlegal']}/{ps[s]['nodes_legal'] / 1e6:.0f}M/{ps[s]['seconds']:.0f}" for s in complete) + ' |')

# readings
one_ring = [a for a in arms if a.split('-')[0] in ('P', 'K1') and a.split('-')[2] in ('Q0', 'Q1', 'Q2', 'Q3')]
def share(fix):
    rs = [a for a in one_ring if a.split('-')[1] == fix]
    n = sum(S[a]['n'] for a in rs); k = sum(S[a]['stranded'] for a in rs)
    return k, n
k0, n0 = share('off'); k1, n1 = share('on')
out += ['', '## Readings (PREREG.md, literal)', '',
        'Scope: at 1e9 nodes per run, one attempt per opening, on this port, with a row scan from the top-left corner without border interleaving.', '']
s0 = k0 / n0 if n0 else 0; s1 = k1 / n1 if n1 else 0
out.append(f'R1: one-ring stranded share, fix off {k0}/{n0} = {s0:.1%}, fix on {k1}/{n1} = {s1:.1%}.')
if s0 > 0.15:
    out.append('R1 reading: ' + ('the defect explains the stranding.' if s1 < 0.05 else 'not supported (fix-on share not below 5%).'))
else:
    out.append('R1 reading: no stranding to explain in these arms (fix-off share 15% or less); supplementary R1\':')
    if 'R1-off-Q0' in S and 'R1-on-Q0' in S:
        a0, a1 = S['R1-off-Q0'], S['R1-on-Q0']
        t0, t1 = a0['stranded'] / a0['n'], a1['stranded'] / a1['n']
        verdict = ('the defect explains the stranding (R1\').' if (t0 > 0.15 and t1 < 0.05)
                   else ('no stranding to explain in R1 either.' if t0 <= 0.15 else 'not supported (fix-on share not below 5%).'))
        out.append(f"R1': R1-off-Q0 {a0['stranded']}/{a0['n']} = {t0:.1%}, R1-on-Q0 {a1['stranded']}/{a1['n']} = {t1:.1%}: {verdict}")
half = (len(complete) + 1) // 2
r2 = []
for k in ('P', 'K1'):
    if f'{k}-on-Q0' not in S:
        continue
    q0pos = S[f'{k}-on-Q0']['pos']
    for q in ('Q1', 'Q2', 'Q3'):
        a = f'{k}-on-{q}'
        if a in S:
            ok = S[a]['pos'] >= half and q0pos == 0
            out.append(f'R2 check {a}: positive openings {S[a]["pos"]} of {S[a]["n"]} (need >= {half}); Q0 twin positive openings {q0pos} (need 0): {"meets" if ok else "does not meet"}')
            if ok:
                r2.append(a)
out.append('R2 reading: ' + (f'the early quota explains the dead one-ring family ({", ".join(r2)}).' if r2 else 'not supported.'))
works = [a for a in arms if a.split('-')[0] in ('P', 'K1') and RAT[a]['sec'][0] >= 1 and RAT[a]['sec'][1] > 1]
dworks = [a for a in arms if a.startswith('D-') and a != REF and RAT[a]['sec'][0] >= 1 and RAT[a]['sec'][1] > 1]
out.append('R3 reading: ' + (f'one-ring configuration(s) work for five clues: {", ".join(works)}.' if works else
                             'no one-ring configuration meets the rule; the default remains the best configuration measured'
                             + (f' (other D arms meeting the same rule against D-off-Q0: {", ".join(dworks)})' if dworks else '') + '.'))
anypos = [a for a in one_ring if a.split('-')[1] == 'on' and S[a]['late'] > 0]
qmpos = [a for a in arms if a.split('-')[0] in ('P', 'K1') and a.split('-')[1] == 'on' and a.split('-')[2] == 'QM' and S[a]['late'] > 0]
out.append('R4: ' + ('one-ring arms with the fix on that placed a legal node at depth >= 201: ' + ', '.join(anypos) + '; R4 does not apply (QM arms, outside the original design: ' + (', '.join(qmpos) or 'none') + ' placed legal nodes at depth >= 201).' if anypos else
                     'no one-ring arm with the fix on placed a legal node at depth >= 201 under any quota: the one-ring family is dead on this port for five clues for a reason not identified here.'))

# R5 (amendment 2)
def pos(a):
    return S[a]['pos'] if a in S else None
n16 = len(complete)
r5 = {}
for fx in ('off', 'on'):
    a = f'D-{fx}-QS'; bs = [x for x in (f'P-{fx}-QM', f'K1-{fx}-QM') if x in S]
    if a not in S or not bs:
        continue
    zero_a = n16 - pos(a)
    slack = zero_a >= (3 * n16 + 3) // 4 and any(pos(b) >= half for b in bs)
    keying = pos(a) >= half and all(S[b]['late'] == 0 for b in bs)
    if slack:
        v = 'slack explains the result'
    elif keying:
        v = 'keying explains the result'
    else:
        a_loses = pos(a) < pos(f'D-{fx}-Q0')
        b_gains = any(pos(b) > pos(b.replace('QM', 'Q0')) for b in bs)
        v = 'both' if (a_loses and b_gains) else 'neither'
        v = f'both or neither: {v} (arm a loses openings against D-{fx}-Q0: {a_loses}; some arm b gains against its Q0 twin: {b_gains})'
    r5[fx] = v
    out.append(f"R5 fix {fx}: arm a {a} positive openings {pos(a)} of {n16} (zero on {zero_a}; D-{fx}-Q0 positive on {pos(f'D-{fx}-Q0')});"
               f" arm b " + ', '.join(f"{b} positive {pos(b)} (Q0 twin {pos(b.replace('QM', 'Q0'))})" for b in bs) + f": {v}.")
if r5:
    out.append('R5 joint: ' + ('; '.join(f'fix {k}: {v}' for k, v in r5.items())) + '.')

# first illegal placement in stranded fix-off runs (judge F4)
fi = {}
for (a, s_), r in R.items():
    if a.split('-')[1] == 'off' and stranded(r):
        fi.setdefault(dlegal(r), []).append(f'{a}/s{s_}')
out += ['', '## Stranded fix-off test runs: depth of the first illegal placement (= deepest legal depth)', '']
for d in sorted(fi):
    out.append(f'depth {d}: {len(fi[d])} runs ({", ".join(sorted(fi[d]))})')
if not fi:
    out.append('none')

C = load('calib')
if C:
    out += ['', '## Calibration (seeds 1 on x86, 2 on m1; never in a statistic)', '', '| arm | seconds s1 | seconds s2 | deepest legal s1 | deepest legal s2 | legal late s1 | legal late s2 |', '|---|---|---|---|---|---|---|']
    for a in [name(*x) for x in ARMS]:
        c1, c2 = C.get((a, 1)), C.get((a, 2))
        g = lambda c, k: (c[k] if k in c else '-') if c else '-'
        out.append(f"| {a} | {g(c1, 'seconds')} | {g(c2, 'seconds')} | {dlegal(c1) if c1 else '-'} | {dlegal(c2) if c2 else '-'} | {late(c1) if c1 else '-'} | {late(c2) if c2 else '-'} |")

open(os.path.join(H, 'results', 'analysis.txt'), 'w').write('\n'.join(out) + '\n')
json.dump({'complete_seeds': complete, 'arms': {a: {k: v for k, v in S[a].items()} for a in arms},
           'ratios': RAT}, open(os.path.join(H, 'results', 'summary.json'), 'w'), indent=1, default=str)
print('\n'.join(out))
