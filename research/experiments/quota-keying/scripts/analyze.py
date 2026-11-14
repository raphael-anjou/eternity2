#!/usr/bin/env python3
"""Preregistered analysis of quota-keying (see PREREG.md). Reads results/raw/*.jsonl,
writes results/summary.json and prints the tables the README quotes.

    python3 -I scripts/analyze.py
"""
import json
import math
import os
import random
import sys

H = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
RAW = os.path.join(H, 'results', 'raw')
SEEDS = list(range(101, 111))
B, BSEED = 10000, 20261113


def load(i):
    p = os.path.join(RAW, i + '.jsonl')
    if not os.path.exists(p):
        return None
    t = open(p).read().strip()
    return json.loads(t) if t else None


def stats(r):
    h, hl = r['hist'], r['hist_legal']
    return dict(
        N=r['nodes'], NL=sum(hl), A=h[201], AL=hl[201],
        Nopen=sum(h[:60]), Nmid=sum(h[60:201]), late=sum(h[201:]), lateL=sum(hl[201:]),
        L60=sum(hl[60:]), L230=sum(hl[230:]), deepest=r['deepest'],
        deepL=max([d for d, x in enumerate(hl) if x] or [0]),
        left=r['deepest'] > 60, completions=r['completions'], seconds=r['seconds'])


def ratio(a, b):
    if b == 0:
        return math.inf if a > 0 else math.nan
    return a / b


def pct(v, q):
    v = sorted(v)
    k = (len(v) - 1) * q
    lo, hi = math.floor(k), math.ceil(k)
    if v[lo] == math.inf or v[hi] == math.inf:
        return math.inf if v[hi] == math.inf and (v[lo] == math.inf or k > lo) else v[lo]
    return v[lo] + (v[hi] - v[lo]) * (k - lo)


def boot(rows_by_key, fn):
    """Paired percentile bootstrap over the keys (openings): fn(list of keys) -> value."""
    keys = sorted(rows_by_key)
    rng = random.Random(BSEED)
    vals = []
    for _ in range(B):
        ks = [keys[rng.randrange(len(keys))] for _ in keys]
        vals.append(fn(ks))
    vals = [v for v in vals if not math.isnan(v)]
    return (pct(vals, 0.025), pct(vals, 0.975)) if vals else (math.nan, math.nan)


def fmt(x):
    if x is None or (isinstance(x, float) and math.isnan(x)):
        return 'nan'
    if x == math.inf:
        return 'inf'
    return f'{x:.3g}'


def arm(prefix):
    out = {}
    for s in SEEDS:
        r = load(f'{prefix}-s{s}')
        if r:
            out[s] = stats(r)
    return out


def pooled(rows, keys=None):
    keys = keys if keys is not None else list(rows)
    S = lambda f: sum(rows[k][f] for k in keys)
    return dict(
        R_tot=ratio(S('N'), S('A')), R_mid=ratio(S('Nmid'), S('A')),
        S_all=ratio(S('late'), S('N')), S_legal=ratio(S('lateL'), S('NL')),
        Y=ratio(S('lateL'), S('N')), N_per_NL=ratio(S('N'), S('NL')),
        AL_per_N=ratio(S('AL'), S('N')))


summary = {}
out = []
P = lambda *a: out.append(' '.join(str(x) for x in a))

# ---------------- M1 and C5 ----------------
arms = {a: arm(a) for a in ['m1-D', 'm1-P', 'm1-M4', 'm1-M3', 'm1-L', 'c5-D', 'c5-P',
                            'm2-P-lb', 'm2-M4-lb', 'm2-M3-lb', 'm2-P-bw',
                            # part 2 (PREREG_AMENDMENTS.md)
                            'c1-D', 'c1-P', 'c1-M4', 'c1-M3', 'c1-L', 'c1-P-lb', 'c1-P-bwi', 'c1-P-bwf',
                            'nc-P-bwf', 'c5-P-bwf']}
P('## M1 / C5 / M2 per arm (pooled over finished openings; cap 1e9 nodes per run)')
P('| arm | runs | left opening | runs with A=0 | arrivals | R_tot | R_mid | median N/A | S_all | S_legal | Y (legal late per node) | N/NL | deepest (min-max) | deepest legal (max) | completions |')
P('|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|')
for a, rows in arms.items():
    if not rows:
        P(f'| {a} | 0 | | | | | | | | | | | | | |')
        continue
    pl = pooled(rows)
    na = [ratio(r['N'], r['A']) for r in rows.values()]
    med = pct(na, 0.5)
    summary[a] = dict(pooled=pl, runs=len(rows), per_seed=rows, median_N_per_A=med)
    P(f"| {a} | {len(rows)} | {sum(r['left'] for r in rows.values())} | {sum(r['A'] == 0 for r in rows.values())} | "
      f"{sum(r['A'] for r in rows.values())} | {fmt(pl['R_tot'])} | {fmt(pl['R_mid'])} | {fmt(med)} | {fmt(pl['S_all'])} | "
      f"{fmt(pl['S_legal'])} | {fmt(pl['Y'])} | {fmt(pl['N_per_NL'])} | "
      f"{min(r['deepest'] for r in rows.values())}-{max(r['deepest'] for r in rows.values())} | "
      f"{max(r['deepL'] for r in rows.values())} | {sum(r['completions'] for r in rows.values())} |")

P('')
P('## Per-seed spread (min / median / max over finished seeds; N/A infinite when a run never arrives)')
P('| arm | N/A min | N/A median | N/A max | S_all min | S_all median | S_all max | S_legal min | S_legal median | S_legal max | per-seed S_all |')
P('|---|---|---|---|---|---|---|---|---|---|---|')
for a, rows in arms.items():
    if not rows:
        continue
    na = [ratio(r['N'], r['A']) for r in rows.values()]
    sa = [ratio(r['late'], r['N']) for r in rows.values()]
    sl = [ratio(r['lateL'], r['NL']) for r in rows.values()]
    summary[a]['spread'] = dict(N_per_A=[min(na), pct(na, .5), max(na)], S_all=[min(sa), pct(sa, .5), max(sa)],
                                S_legal=[min(sl), pct(sl, .5), max(sl)])
    P(f"| {a} | {fmt(min(na))} | {fmt(pct(na, .5))} | {fmt(max(na))} | {fmt(min(sa))} | {fmt(pct(sa, .5))} | {fmt(max(sa))} | "
      f"{fmt(min(sl))} | {fmt(pct(sl, .5))} | {fmt(max(sl))} | " + ' '.join(f'{k}:{fmt(v)}' for k, v in zip(rows, sa)) + ' |')

P('')
P('## M1 gain G = R(D) / R(arm), paired bootstrap over openings (95%)')
D = arms['m1-D']
for a in ['m1-P', 'm1-M4', 'm1-M3', 'm1-L']:
    X = arms[a]
    common = sorted(set(D) & set(X))
    if not common:
        continue
    for basis in ('R_tot', 'R_mid'):
        g = ratio(pooled(D, common)[basis], pooled(X, common)[basis])
        ci = boot({k: 1 for k in common}, lambda ks: ratio(pooled(D, ks)[basis], pooled(X, ks)[basis]))
        summary.setdefault('M1_gain', {})[f'{a}:{basis}'] = dict(G=g, ci=ci, n=len(common))
        verdict = ''
        if basis == 'R_tot':
            verdict = 'supported' if ci[0] >= 100 else ('contradicted' if ci[1] < 45 else 'not decided')
        P(f'{a} {basis}: G = {fmt(g)}  CI [{fmt(ci[0])}, {fmt(ci[1])}]  n = {len(common)}  {verdict}')

P('')
P('## C5 (five clues) gain, same definitions')
D5, X5 = arms['c5-D'], arms['c5-P']
c = sorted(set(D5) & set(X5))
if c:
    for basis in ('R_tot', 'R_mid'):
        g = ratio(pooled(D5, c)[basis], pooled(X5, c)[basis])
        ci = boot({k: 1 for k in c}, lambda ks: ratio(pooled(D5, ks)[basis], pooled(X5, ks)[basis]))
        summary.setdefault('C5_gain', {})[basis] = dict(G=g, ci=ci, n=len(c))
        P(f'c5-P {basis}: G = {fmt(g)}  CI [{fmt(ci[0])}, {fmt(ci[1])}]  n = {len(c)}')

P('')
P('## M2: Y(arm + legal break) / Y(arm), paired bootstrap over openings (95%)')
for x, y in [('m2-P-lb', 'm1-P'), ('m2-M4-lb', 'm1-M4'), ('m2-M3-lb', 'm1-M3'), ('m2-P-bw', 'm1-P'), ('m2-P-bw', 'm2-P-lb')]:
    X, Yr = arms[x], arms[y]
    common = sorted(set(X) & set(Yr))
    if not common:
        continue
    r = ratio(pooled(X, common)['Y'], pooled(Yr, common)['Y'])
    ci = boot({k: 1 for k in common}, lambda ks: ratio(pooled(X, ks)['Y'], pooled(Yr, ks)['Y']))
    verdict = 'supported' if ci[0] > 1.5 else ('contradicted' if ci[1] < 1.2 else 'not decided')
    if y.startswith('m2-'):
        verdict = '(secondary, no preregistered reading)'
    summary.setdefault('M2', {})[f'{x}/{y}'] = dict(ratio=r, ci=ci, n=len(common))
    P(f'{x} / {y}: {fmt(r)}  CI [{fmt(ci[0])}, {fmt(ci[1])}]  n = {len(common)}  {verdict}')

P('')
P('## Part 2: keying gain at the centre clue, G = R(c1-D) / R(arm), and break-path fidelity')
D1 = arms['c1-D']
for a in ['c1-P', 'c1-M4', 'c1-M3', 'c1-L']:
    X = arms[a]
    common = sorted(set(D1) & set(X))
    if not common:
        continue
    for basis in ('R_tot', 'R_mid'):
        g = ratio(pooled(D1, common)[basis], pooled(X, common)[basis])
        ci = boot({k: 1 for k in common}, lambda ks: ratio(pooled(D1, ks)[basis], pooled(X, ks)[basis]))
        verdict = ''
        if basis == 'R_tot':
            verdict = 'supported' if ci[0] >= 100 else ('contradicted' if ci[1] < 45 else 'not decided')
        summary.setdefault('P2_gain', {})[f'{a}:{basis}'] = dict(G=g, ci=ci, n=len(common))
        P(f'{a} {basis}: G = {fmt(g)}  CI [{fmt(ci[0])}, {fmt(ci[1])}]  n = {len(common)}  {verdict}')
for x, y in [('c1-P-bwf', 'c1-P'), ('c1-P-lb', 'c1-P'), ('c1-P-bwi', 'c1-P'), ('nc-P-bwf', 'm1-P'),
             ('c1-P-bwf', 'c1-P-bwi'), ('nc-P-bwf', 'm2-P-bw')]:
    X, Yr = arms[x], arms[y]
    common = sorted(set(X) & set(Yr))
    if not common:
        continue
    r = ratio(pooled(X, common)['Y'], pooled(Yr, common)['Y'])
    ci = boot({k: 1 for k in common}, lambda ks: ratio(pooled(X, ks)['Y'], pooled(Yr, ks)['Y']))
    summary.setdefault('P2_fidelity', {})[f'{x}/{y}'] = dict(ratio=r, ci=ci, n=len(common))
    P(f'Y {x} / {y}: {fmt(r)}  CI [{fmt(ci[0])}, {fmt(ci[1])}]  n = {len(common)}')

# ---------------- M3 ----------------
P('')
P('## M3 scan')
scan = {}
for t in ('5-20-7', '4-16-17'):
    for s in range(301, 341):
        r = load(f'm3scan-{t}-s{s}')
        if r:
            scan[(t, s)] = r['deepest'] > 60
adm = [k for k in sorted(scan) if scan[k]]
P(f'tried {len(scan)}, passed depth 60: {len(adm)}: ' + ', '.join(f'{t} s{s}' for t, s in adm))
summary['M3_scan'] = dict(tried=len(scan), admitted=[list(k) for k in adm])

m3 = {}
for t, s in [(t, s) for t in ('5-20-7', '4-16-17') for s in range(301, 341)]:
    rs = [load(f'm3-{t}-A{a}-s{s}') for a in (0, 1, 2)]
    if all(rs):
        m3[(t, s)] = rs
incomplete = []
for t, s in adm:
    if (t, s) not in m3 and any(load(f'm3-{t}-A{a}-s{s}') for a in (0, 1, 2)):
        incomplete.append((t, s))
P('')
P(f'## M3 (complete openings: {len(m3)}; incomplete: {incomplete})')


def d230(keys, a):
    num = sum(sum(m3[k][a]['hist_legal'][230:]) for k in keys)
    den = sum(sum(m3[k][a]['hist_legal'][60:]) for k in keys)
    return ratio(num, den)


if m3:
    keys = sorted(m3)
    P('| opening | arm | legal nodes | legal at >=60 | legal at >=230 | legal arrivals | legal late per arrival | deepest legal | completions | c3 chosen (A2) |')
    P('|---|---|---|---|---|---|---|---|---|---|')
    for k in keys:
        for a in range(3):
            r = m3[k][a]
            hl = r['hist_legal']
            c3 = {i: v for i, v in enumerate(r['c3_hist']) if v}
            P(f"| {k[0]} s{k[1]} | A{a} | {sum(hl)} | {sum(hl[60:])} | {sum(hl[230:])} | {hl[201]} | {fmt(ratio(sum(hl[201:]), hl[201]))} | "
              f"{max([d for d, x in enumerate(hl) if x] or [0])} | {r['completions']} | {c3 if a == 2 else ''} |")
    D = [d230(keys, a) for a in range(3)]
    deep0 = sum(sum(m3[k][0]['hist_legal'][230:]) for k in keys)
    r1, r2 = ratio(D[1], D[0]), ratio(D[2], D[0])
    ci1 = boot({k: 1 for k in keys}, lambda ks: ratio(d230(ks, 1), d230(ks, 0)))
    ci2 = boot({k: 1 for k in keys}, lambda ks: ratio(d230(ks, 2), d230(ks, 0)))
    P('')
    P(f'D230: A0 {fmt(D[0])}, A1 {fmt(D[1])}, A2 {fmt(D[2])}; deep nodes in A0 = {deep0}; openings = {len(keys)}')
    P(f'D230(A1)/D230(A0) = {fmt(r1)}  CI [{fmt(ci1[0])}, {fmt(ci1[1])}]')
    P(f'D230(A2)/D230(A0) = {fmt(r2)}  CI [{fmt(ci2[0])}, {fmt(ci2[1])}]')
    if len(keys) < 4 or deep0 < 100:
        kill = 'uninformative'
    elif r1 < 3 and r2 < 3:
        kill = 'dead (both ratios below 3)'
    elif r1 >= 10 or r2 >= 10:
        kill = 'continues (an arm reaches 10)'
    else:
        kill = 'neither dead nor continuing (a ratio in [3, 10))'
    P(f'kill line: {kill}')
    # noise null: first half against second half of A0
    def half(keys, first):
        num = den = 0
        for k in keys:
            r = m3[k][0]
            snaps = r.get('legal_checkpoints') or []
            if not snaps:
                continue
            hl_half = snaps[0][3]
            hl = hl_half if first else [x - y for x, y in zip(r['hist_legal'], hl_half)]
            num += sum(hl[230:]); den += sum(hl[60:])
        return ratio(num, den)
    h1, h2 = half(keys, True), half(keys, False)
    P(f'noise null, A0 first half D230 = {fmt(h1)}, second half = {fmt(h2)}, ratio = {fmt(ratio(h2, h1))}')
    summary['M3'] = dict(openings=[list(k) for k in keys], D230=D, deep_A0=deep0, r1=r1, ci1=ci1, r2=r2, ci2=ci2,
                         kill=kill, null=dict(first=h1, second=h2), incomplete=[list(k) for k in incomplete])

text = '\n'.join(out)
print(text)


def clean(o):
    if isinstance(o, float) and (math.isinf(o) or math.isnan(o)):
        return str(o)
    if isinstance(o, dict):
        return {str(k): clean(v) for k, v in o.items()}
    if isinstance(o, (list, tuple)):
        return [clean(v) for v in o]
    return o


json.dump(clean(summary), open(os.path.join(H, 'results', 'summary.json'), 'w'), indent=1)
open(os.path.join(H, 'results', 'analysis.txt'), 'w').write(text + '\n')
