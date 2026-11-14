import json, sys
for f in sys.argv[1:]:
    for l in open(f):
        d = json.loads(l)
        row = [d.get('tag', ''), d['board'], d.get('mode', ''), 'P', d['P'], 'sols', d['n_sols'], 'nodes', d.get('dfs_nodes'), 'solpl', d.get('in_some_solution')]
        for k in ('lp', 'sdp', 'theta'):
            if k in d:
                r = d[k]
                row += ['|', k, r['status'][:22], r.get('per_cell'), r.get('placements_above_1e-3_off_solutions'), r['iters'], r['seconds']]
        print(*row)
