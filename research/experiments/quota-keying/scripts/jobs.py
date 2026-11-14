#!/usr/bin/env python3
"""Write the preregistered job lists (PREREG.md) as TSV: id, stage, binary, args.

    python3 -I scripts/jobs.py gate|main|m3scan > jobs/<stage>.tsv
    python3 -I scripts/jobs.py m3 SEED[,SEED...] [TRIPLE] > jobs/m3.tsv
    python3 -I scripts/jobs.py gate2|part2 > jobs/<stage>.tsv   (PREREG_AMENDMENTS.md)
"""
import sys
U = '201,206,211,216,221,225,229,233,237,239'
NC, FC, CC = 'data/official-noclue.csv', 'data/official.csv', 'data/official-centre.csv'
SEEDS = range(101, 111)
def line(i, st, b, a):
    print(f'{i}\t{st}\t{b}\t{a}')
st = sys.argv[1]
if st == 'gate':
    for s in (1, 2, 3):
        line(f'gate-port-rpp-s{s}', 'gate', 'rpp', f'bw --csv {FC} --unlock {U} --seed {s} --cap 1e8 --label gate-port-rpp')
        line(f'gate-port-qk-s{s}', 'gate', 'qk', f'bw --csv {FC} --unlock {U} --seed {s} --cap 1e8 --stage 0 --legalcap 1e18 --label gate-port-qk')
        for v, f in (('plain', ''), ('lb', ' --legalbreak'), ('bw', ' --legalbreak --ringbreakoff')):
            line(f'gate-nobreak-{v}-s{s}', 'gate', 'qk', f'bw --csv {NC} --unlock 999 --seed {s} --cap 1e8 --priv 5,20,7{f} --label gate-nobreak-{v}')
    line('gate-memo-4-16-17-s1', 'gate', 'qk', f'bw --csv {NC} --unlock {U} --seed 1 --cap 1e8 --priv 4,16,17 --label gate-memo')
elif st == 'main':
    arms = [  # id, csv, priv, flags
        ('m1-P', NC, '5,20,7', ''), ('m1-M4', NC, '4,16,17', ''), ('m1-M3', NC, '3,12,13', ''),
        ('m2-P-lb', NC, '5,20,7', ' --legalbreak'), ('m2-M4-lb', NC, '4,16,17', ' --legalbreak'),
        ('m2-M3-lb', NC, '3,12,13', ' --legalbreak'), ('m2-P-bw', NC, '5,20,7', ' --legalbreak --ringbreakoff'),
        ('m1-D', NC, '22,21,20', ''), ('m1-L', NC, '13,16,10', ''),
        ('c5-D', FC, '22,21,20', ''), ('c5-P', FC, '5,20,7', ''),
    ]
    for a, csv, p, f in arms:
        for s in SEEDS:
            line(f'{a}-s{s}', 'test-main', 'qk', f'bw --csv {csv} --unlock {U} --seed {s} --cap 1e9 --priv {p}{f} --label {a}')
elif st == 'm3scan':
    for tri in ('5,20,7', '4,16,17'):
        for s in range(301, 341):
            t = tri.replace(',', '-')
            line(f'm3scan-{t}-s{s}', 'test-m3scan', 'qk', f'bw --csv {NC} --unlock {U} --seed {s} --cap 1e8 --priv {tri} --legalbreak --stage 0 --label m3scan')
elif st == 'm3':
    seeds = [int(x) for x in sys.argv[2].split(',')]
    tri = sys.argv[3] if len(sys.argv) > 3 else '5,20,7'
    t = tri.replace(',', '-')
    for s in seeds:
        for arm in (0, 1, 2):
            line(f'm3-{t}-A{arm}-s{s}', 'test-m3', 'qk', f'bw --csv {NC} --unlock {U} --seed {s} --cap 1e12 --legalcap 1e9 --priv {tri} --legalbreak --stage {arm} --label m3-A{arm}')
elif st == 'gate2':
    for s in (1, 2, 3):
        line(f'gate2-port-rpp-s{s}', 'gate2', 'rpp', f'bw --csv {FC} --unlock {U} --seed {s} --cap 1e8 --label gate2-port-rpp')
        line(f'gate2-port-qk-s{s}', 'gate2', 'qk', f'bw --csv {FC} --unlock {U} --seed {s} --cap 1e8 --stage 0 --legalcap 1e18 --label gate2-port-qk')
        for v, f in (('plain', ''), ('lb', ' --legalbreak'), ('bw', ' --legalbreak --ringbreakoff'), ('bwf', ' --legalbreak --ringbreakoff --noisolation')):
            line(f'gate2-nobreak-{v}-s{s}', 'gate2', 'qk', f'bw --csv {NC} --unlock 999 --seed {s} --cap 1e8 --priv 5,20,7{f} --label gate2-nobreak-{v}')
elif st == 'part2':
    BWF = ' --legalbreak --ringbreakoff --noisolation'
    arms = [
        ('c1-P-bwf', CC, '5,20,7', BWF), ('c1-P-bwi', CC, '5,20,7', ' --legalbreak --ringbreakoff'),
        ('c1-P-lb', CC, '5,20,7', ' --legalbreak'), ('nc-P-bwf', NC, '5,20,7', BWF),
        ('c1-P', CC, '5,20,7', ''), ('c1-M4', CC, '4,16,17', ''), ('c1-M3', CC, '3,12,13', ''),
        ('c1-D', CC, '22,21,20', ''), ('c1-L', CC, '13,16,10', ''), ('c5-P-bwf', FC, '5,20,7', BWF),
    ]
    for a, csv, p, f in arms:
        for s in SEEDS:
            line(f'{a}-s{s}', 'test-part2', 'qk', f'bw --csv {csv} --unlock {U} --seed {s} --cap 1e9 --priv {p}{f} --label {a}')
