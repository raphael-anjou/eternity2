#!/usr/bin/env python3
"""Job lists (PREREG.md): python3 -I scripts/jobs.py gate|gate1|calib|test HOST
One line per run: id <TAB> stage <TAB> fcq arguments."""
import sys
U = '201,206,211,216,221,225,229,233,237,239'
FID = '--legalbreak --ringbreakoff --noisolation'
CSV = 'data/official.csv'
KEY = {'D': ('22,21,20', ''), 'P': ('5,20,7', ' --quotascaled'), 'K1': ('1,7,9', ' --quotascaled'),
       'R1': ('2,9,11', ' --quotascaled')}
Q = {'Q0': '', 'Q1': ' --noquota', 'Q2': ' --quotamode 2', 'Q3': ' --quotamode 3', 'QS': '', 'QM': ''}
ARMS = [(k, f, q) for k in ('D', 'P', 'K1') for f in ('off', 'on') for q in ('Q0', 'Q1', 'Q2', 'Q3')]
ARMS += [('R1', 'off', 'Q0'), ('R1', 'on', 'Q0')]
# amendment 2 (deconfounding): D with the scaled quota (QS), one-ring keyings slack-matched (QM)
ARMS += [('D', f, 'QS') for f in ('off', 'on')] + [(k, f, 'QM') for k in ('P', 'K1') for f in ('off', 'on')]
DROP_Q3 = False      # amendment 2 reduction order: K1, then Q3, then 10 openings
SEEDS = {'x86': range(501, 509), 'm1': range(509, 514), 'local': range(514, 517)}
CALIB = {'x86': [1], 'm1': [2], 'local': []}
DROP_K1 = False      # reduction (1), set only by a PREREG_AMENDMENTS.md entry
REDUCED = False      # reduction (2)
if REDUCED:
    SEEDS = {'x86': range(501, 507), 'm1': range(507, 511), 'local': range(0)}


def name(k, f, q):
    return f'{k}-{f}-{q}'


def args(k, f, q, seed, cap='1e9'):
    pr, sc = KEY[k]
    if q in ('QS', 'QM'):
        sc = ' --quotascaled' if q == 'QS' else ' --quotaslack'
    fx = ' --orientfix' if f == 'on' else ''
    return f'bw --csv {CSV} --unlock {U} --seed {seed} --cap {cap} --priv {pr} {FID}{sc}{Q[q]}{fx} --label {name(k, f, q)}'


def line(i, st, a):
    print(f'{i}\t{st}\t{a}')


st, host = sys.argv[1], (sys.argv[2] if len(sys.argv) > 2 else 'local')
arms = [a for a in ARMS if not (DROP_K1 and a[0] == 'K1') and not (DROP_Q3 and a[2] == 'Q3')]
if st == 'gate':
    # G-off, G-on-unflagged (cap 1e9 and capped at the committed nodes_legal), G-on-flagged
    legal = {'D': 990081316, 'K1': 960784012, 'P': 979156035}
    for k, t14 in (('D', 'D'), ('K1', '1-7-9'), ('P', '5-7-20')):
        line(f'gate-off-{t14}-s401', 'gate', args(k, 'off', 'Q0', 401))
        line(f'gate-on-{t14}-s401', 'gate', args(k, 'on', 'Q0', 401))
        line(f'gate-oncap-{t14}-s401', 'gate', args(k, 'on', 'Q0', 401, cap=legal[k]))
    line('gate-on-D-s402', 'gate', args('D', 'on', 'Q0', 402))
    line('gate-on-1-8-9-s403', 'gate', args('P', 'on', 'Q0', 403).replace('--priv 5,20,7', '--priv 1,8,9').replace('P-on-Q0', '1-8-9-on-Q0'))
    line('gate-on-2-9-11-s401', 'gate', args('R1', 'on', 'Q0', 401))
elif st == 'gate2':
    # amendment 2: G-off rerun with the binary that has --quotaslack
    for k, t14 in (('D', 'D'), ('K1', '1-7-9'), ('P', '5-7-20')):
        line(f'gate2-off-{t14}-s401', 'gate', args(k, 'off', 'Q0', 401))
elif st == 'gate1':
    line(f'gate1-{host}-off-5-7-20-s401', 'gate', args('P', 'off', 'Q0', 401))
elif st == 'calib':
    for s in CALIB[host]:
        for a in arms:
            line(f'calib-{name(*a)}-s{s}', 'calib', args(*a, s))
elif st == 'test':
    # seed-major so that a host finishes whole seeds first
    for s in SEEDS[host]:
        for a in arms:
            line(f'test-{name(*a)}-s{s}', 'test', args(*a, s))
