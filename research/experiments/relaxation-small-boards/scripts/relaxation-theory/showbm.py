import sys, json
for l in open(sys.argv[1]).read().strip().split("\n")[-int(sys.argv[2]) if len(sys.argv) > 2 else -1:]:
    d = json.loads(l)
    print(d['board'], d['variant'], 'rank', d['rank'], 'rng', d.get('rngseed'), 'sols', d['n_sols'], 'P', d['P'], 'numrank', d['numerical_rank_1e_3'], 'sec', d['seconds'])
    print('   final', {k: (round(v, 6) if isinstance(v, float) else v) for k, v in d['final'].items()})
    h = d['history']
    print('   hist(mu,worst,leak/cell,sec)', [(x['mu'], x['worst'], x['leak_per_cell'], x['seconds']) for x in h[::max(1, len(h) // 12)]] + [(h[-1]['mu'], h[-1]['worst'], h[-1]['leak_per_cell'], h[-1]['seconds'])])
