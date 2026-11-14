#!/usr/bin/env python3
"""Port gate (PREREG.md): fck rows against Track 13's committed fidelity rows."""
import glob, json, os
H = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
T13 = os.path.join(H, '..', 'quota-keying', 'results', 'raw')
ok_all = True
KEYS = ('hist', 'hist_legal', 'nodes', 'nodes_legal', 'deepest')
pairs = [('gate-c5-P-bwf-s101', 'c5-P-bwf-s101'), ('gate-c1-P-bwf-s102', 'c1-P-bwf-s102'),
         ('gate-nc-P-bwf-s101', 'nc-P-bwf-s101'), ('gate-c5-P-bwf-scaled-s101', 'c5-P-bwf-s101')]
pairs += [(os.path.basename(f)[:-6], 'c5-P-bwf-s101') for f in sorted(glob.glob(os.path.join(H, 'results', 'raw', 'gate-*-c5-P-bwf-s101.jsonl')))
          if 'scaled' not in f and os.path.basename(f) != 'gate-c5-P-bwf-s101.jsonl']
for mine, ref in pairs:
    p = os.path.join(H, 'results', 'raw', mine + '.jsonl')
    if not os.path.exists(p):
        print('MISSING', mine); ok_all = False; continue
    a, b = json.loads(open(p).readline()), json.loads(open(os.path.join(T13, ref + '.jsonl')).readline())
    ok = all(a[k] == b[k] for k in KEYS)
    ok_all &= ok
    print('PASS' if ok else 'FAIL', mine, 'vs', ref, f"nodes {a['nodes']} deepest {a['deepest']} {a['seconds']}s")
print('ALL PASS' if ok_all else 'SOME GATE FAILED')
