#!/usr/bin/env python3
"""Remote one-row gate: exit 0 iff gate-HOST-c5-P-bwf-s101 equals Track 13's committed row."""
import json, os, sys
H = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
a = json.loads(open(os.path.join(H, 'results', 'raw', f'gate-{sys.argv[1]}-c5-P-bwf-s101.jsonl')).readline())
b = json.loads(open(os.path.join(H, '..', 'quota-keying', 'results', 'raw', 'c5-P-bwf-s101.jsonl')).readline())
ok = all(a[k] == b[k] for k in ('hist', 'hist_legal', 'nodes', 'nodes_legal', 'deepest'))
print('PASS' if ok else 'FAIL', sys.argv[1], a['seconds'], 's')
sys.exit(0 if ok else 1)
