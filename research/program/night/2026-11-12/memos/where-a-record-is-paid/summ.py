#!/usr/bin/env python3
"""One line per bw run: python3 -I summ.py out/x.jsonl ..."""
import json, sys
for f in sys.argv[1:]:
    for l in open(f):
        r = json.loads(l); h = r['hist']; t = sum(h)
        late = sum(h[201:]); pk = max(range(257), key=lambda d: h[d])
        first = next((d for d in range(256) if h[d] > 1), None)
        print('%-34s nodes %.1e s %6.1f deepest %3d brk %2d done %d best %d | hist[201] %d late(>=201) %.3g peak %d | h160 %d h161 %d h200 %d | shallowest revisited depth %s' % (
            f.split('/')[-1], t, r['seconds'], r['deepest'], r['breaks_at_deepest'], r['completions'], r['best_complete_score'],
            h[201], late / t, pk, h[160], h[161], h[200], first))
