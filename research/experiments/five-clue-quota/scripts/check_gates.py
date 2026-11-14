#!/usr/bin/env python3
"""Gates of PREREG.md: G-off, G-on-unflagged (capped subsequence test and the
uncapped difference), G-on-flagged. Prints a report; exit 1 if G-off or the
capped test fails."""
import json, os
H = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
R = os.path.join(H, 'results', 'raw')
T14 = os.path.join(H, '..', 'five-clue-keying', 'results', 'raw')
K = ('hist', 'hist_legal', 'nodes', 'nodes_legal', 'deepest')


def ld(d, n):
    return json.loads(open(os.path.join(d, n + '.jsonl')).readline())


def late(r):
    return sum(r['hist_legal'][201:])


def dl(r):
    return max([d for d, v in enumerate(r['hist_legal']) if v] + [0])


ok = True
for t in ('D', '1-7-9', '5-7-20'):
    ref = ld(T14, f'conf-{t}-s401')
    off = ld(R, f'gate-off-{t}-s401')
    g = all(off[k] == ref[k] for k in K)
    ok &= g
    print(f"G-off {t}: {'PASS' if g else 'FAIL'} (nodes {off['nodes']}, deepest {off['deepest']}, {off['seconds']} s)")
    cap = ld(R, f'gate-oncap-{t}-s401')
    g2 = cap['hist'] == ref['hist_legal'] and cap['nodes'] == ref['nodes_legal'] == cap['nodes_legal']
    ok &= g2
    print(f"G-on capped at committed nodes_legal {ref['nodes_legal']}: hist == committed hist_legal: {'PASS' if g2 else 'FAIL'}")
    on = ld(R, f'gate-on-{t}-s401')
    nd = sum(1 for a, b in zip(on['hist'], ref['hist']) if a != b)
    print(f"G-on uncapped {t}: identical to committed row: {on['hist'] == ref['hist']}; depths whose count differs {nd};"
          f" illegal nodes in committed row {ref['nodes'] - ref['nodes_legal']} ({(ref['nodes'] - ref['nodes_legal']) / ref['nodes']:.2%});"
          f" deepest {ref['deepest']} -> {on['deepest']}; deepest legal {dl(ref)} -> {dl(on)};"
          f" legal late nodes {late(ref)} -> {late(on)}; nodes_legal {on['nodes_legal']}; {ref['seconds']} s -> {on['seconds']} s")
for t, ref_name in (('D-s402', 'conf-D-s402'), ('1-8-9-s403', 'conf-1-8-9-s403'), ('2-9-11-s401', 'conf-2-9-11-s401')):
    ref = ld(T14, ref_name)
    on = ld(R, f'gate-on-{t}')
    print(f"G-on-flagged {t}: committed legal nodes {ref['nodes_legal']}, deepest {ref['deepest']};"
          f" fix on: deepest {on['deepest']} ({'left the opening' if on['deepest'] > 60 else 'DID NOT LEAVE'}),"
          f" deepest legal {dl(on)}, legal late {late(on)}, nodes {on['nodes']}, {on['seconds']} s")
for t in ('D', '1-7-9', '5-7-20'):
    p = os.path.join(R, f'gate2-off-{t}-s401.jsonl')
    if os.path.exists(p):
        a, ref = json.loads(open(p).readline()), ld(T14, f'conf-{t}-s401')
        g = all(a[k] == ref[k] for k in K)
        ok &= g
        print(f"G-off (amendment 2 binary) {t}: {'PASS' if g else 'FAIL'}")
print('GATES (G-off, capped) PASS' if ok else 'GATE FAILED')
raise SystemExit(0 if ok else 1)
