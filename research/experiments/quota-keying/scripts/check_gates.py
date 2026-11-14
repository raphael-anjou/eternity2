#!/usr/bin/env python3
"""Check the preregistered gates on results/raw/gate-*.jsonl; print PASS/FAIL lines."""
import json, os
H = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
R = lambda i: json.loads(open(os.path.join(H, 'results', 'raw', i + '.jsonl')).readline())
ok_all = True
def say(name, ok, extra=''):
    global ok_all
    ok_all &= ok
    print(('PASS' if ok else 'FAIL'), name, extra)
for s in (1, 2, 3):
    a, b = R(f'gate-port-rpp-s{s}'), R(f'gate-port-qk-s{s}')
    say(f'G-port seed {s}', a['hist'] == b['hist'] and a['nodes'] == b['nodes'] and a['deepest'] == b['deepest'],
        f"nodes {a['nodes']} deepest {a['deepest']}/{b['deepest']} rpp {a['seconds']}s qk {b['seconds']}s")
    p, l, w = (R(f'gate-nobreak-{v}-s{s}') for v in ('plain', 'lb', 'bw'))
    say(f'G-nobreak seed {s}', p['hist'] == l['hist'] == w['hist'] and p['hist_legal'] == l['hist_legal'] == w['hist_legal'],
        f"deepest {p['deepest']} nodes {p['nodes']}")
M = 'research/program/night/2026-11-12/memos/where-a-record-is-paid/out/e4-legal-noclue-priv4-16-17-s1-1e8.jsonl'
if os.path.exists(M):
    m, q = json.loads(open(M).readline()), R('gate-memo-4-16-17-s1')
    say('G-port memo (4,16,17) seed 1', m['hist'] == q['hist'] and m['hist_legal'] == q['hist_legal'])
else:
    print('SKIP memo comparison (memo output not on this checkout)')
if os.path.exists(os.path.join(H, 'results', 'raw', 'gate2-port-qk-s1.jsonl')):
    for s in (1, 2, 3):
        a, b = R(f'gate2-port-rpp-s{s}'), R(f'gate2-port-qk-s{s}')
        say(f'G2-port seed {s}', a['hist'] == b['hist'] and a['nodes'] == b['nodes'] and a['deepest'] == b['deepest'])
        vs = [R(f'gate2-nobreak-{v}-s{s}') for v in ('plain', 'lb', 'bw', 'bwf')]
        say(f'G2-nobreak seed {s}', all(v['hist'] == vs[0]['hist'] and v['hist_legal'] == vs[0]['hist_legal'] for v in vs))
print('ALL PASS' if ok_all else 'SOME GATE FAILED')
