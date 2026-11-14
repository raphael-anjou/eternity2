#!/usr/bin/env python3
"""Preregistered job lists (PREREG.md). One TSV line per run: id, stage, args.

    python3 -I scripts/jobs.py gate                > jobs/gate.tsv
    python3 -I scripts/jobs.py gate1               > jobs/gate1.tsv   (one-row remote gate)
    python3 -I scripts/jobs.py screen HOST         > jobs/screen-HOST.tsv
    python3 -I scripts/jobs.py confirm HOST        > jobs/confirm-HOST.tsv  (needs results/screen-arms.json)
HOST is x86, m1 or local; runs are dealt to hosts in proportion 12:7:4,
opening-major, so every host works through seed 1 first, then 2, then 3.
"""
import itertools, json, os, random, sys

U = '201,206,211,216,221,225,229,233,237,239'
FID = '--legalbreak --ringbreakoff --noisolation'
H = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
KEYS = [(r, a, b) for r in range(1, 6) for a, b in itertools.combinations(range(6, 23), 2)]
RANDOM = [KEYS[i] for i in random.Random(20261113).sample(range(len(KEYS)), 5)]
DEAL = ['x86'] * 12 + ['m1'] * 7 + ['local'] * 4
random.Random(7).shuffle(DEAL)


def kid(k):
    return 'D' if k == 'D' else '-'.join(map(str, k))


def args(k, seed, csv='data/official.csv'):
    if k == 'D':
        return f'bw --csv {csv} --unlock {U} --seed {seed} --cap 1e9 --priv 22,21,20 {FID} --label D'
    return f'bw --csv {csv} --unlock {U} --seed {seed} --cap 1e9 --priv {kid(k).replace("-", ",")} {FID} --quotascaled --label {kid(k)}'


def line(i, st, a):
    print(f'{i}\t{st}\t{a}')


st = sys.argv[1]
host = sys.argv[2] if len(sys.argv) > 2 else None
if st == 'gate':
    line('gate-c5-P-bwf-s101', 'gate', f'bw --csv data/official.csv --unlock {U} --seed 101 --cap 1e9 --priv 5,20,7 {FID} --label c5-P-bwf')
    line('gate-c1-P-bwf-s102', 'gate', f'bw --csv data/official-centre.csv --unlock {U} --seed 102 --cap 1e9 --priv 5,20,7 {FID} --label c1-P-bwf')
    line('gate-nc-P-bwf-s101', 'gate', f'bw --csv data/official-noclue.csv --unlock {U} --seed 101 --cap 1e9 --priv 5,20,7 {FID} --label nc-P-bwf')
    line('gate-c5-P-bwf-scaled-s101', 'gate', f'bw --csv data/official.csv --unlock {U} --seed 101 --cap 1e9 --priv 5,20,7 {FID} --quotascaled --label c5-P-bwf')
elif st == 'gate1':
    line(f'gate-{host}-c5-P-bwf-s101', 'gate', f'bw --csv data/official.csv --unlock {U} --seed 101 --cap 1e9 --priv 5,20,7 {FID} --label c5-P-bwf')
elif st == 'screen':
    order = KEYS + ['D']
    random.Random(1).shuffle(order)
    n = 0
    for seed in (1, 2, 3):
        for k in order:
            if DEAL[n % len(DEAL)] == host:
                line(f'scr-{kid(k)}-s{seed}', 'screen', args(k, seed))
            n += 1
elif st == 'confirm':
    arms = json.load(open(os.path.join(H, 'results', 'screen-arms.json')))['arms']
    # Amendment 1: all arms of one opening on one host (pairing by seed and host).
    SEED_HOST = ['x86', 'm1', 'x86', 'local', 'x86', 'm1', 'x86', 'x86', 'm1', 'local',
                 'x86', 'm1', 'x86', 'local', 'x86', 'm1', 'x86', 'x86', 'm1', 'local']
    for seed in range(401, 421):
        if SEED_HOST[seed - 401] != host:
            continue
        for k in arms:
            k = 'D' if k == 'D' else tuple(k)
            line(f'conf-{kid(k)}-s{seed}', 'confirm', args(k, seed))
else:
    sys.exit('unknown stage')
