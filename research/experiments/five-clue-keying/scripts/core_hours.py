#!/usr/bin/env python3
"""Sum run durations (thread-seconds) from every guard log in results/, by stage.
Lines are deduplicated by (run_id, event): the *-confirm logs start with a copy of
this Mac's log (README, "Guard logs")."""
import glob, json, os
from collections import defaultdict
H = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
tot = defaultdict(float); starts = defaultdict(int); ends = defaultdict(int); seen = set()
for f in sorted(glob.glob(os.path.join(H, 'results', 'run-log*.jsonl'))):
    for l in open(f):
        r = json.loads(l)
        if (r['run_id'], r['event']) in seen:
            continue
        seen.add((r['run_id'], r['event']))
        if r['event'] == 'end':
            tot[r['stage']] += r['seconds']; ends[r['stage']] += 1
        else:
            starts[r['stage']] += 1
for k in sorted(tot):
    print(f'{k}: {ends[k]} runs ended of {starts[k]} started, {tot[k] / 3600:.2f} thread-hours')
print(f'total: {sum(tot.values()) / 3600:.2f} thread-hours')
