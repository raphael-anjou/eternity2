#!/usr/bin/env python3
"""Per-host G-off row: gate1-HOST-off-5-7-20-s401 against Track 14's conf-5-7-20-s401."""
import json, os, sys
H = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
h = sys.argv[1]
a = json.loads(open(os.path.join(H, 'results', 'raw', f'gate1-{h}-off-5-7-20-s401.jsonl')).readline())
b = json.loads(open(os.path.join(H, '..', 'five-clue-keying', 'results', 'raw', 'conf-5-7-20-s401.jsonl')).readline())
ok = all(a[k] == b[k] for k in ('hist', 'hist_legal', 'nodes', 'nodes_legal', 'deepest'))
print('PASS' if ok else 'FAIL', h)
sys.exit(0 if ok else 1)
