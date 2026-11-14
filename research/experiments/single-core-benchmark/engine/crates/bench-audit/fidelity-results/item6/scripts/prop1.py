import json, sys, glob, os, re
d = sys.argv[1]
seeds = sorted({int(re.search(r'-(\d+)\.json$', f).group(1)) for f in glob.glob(d + '/off-*.json')})
for s in seeds:
    off = json.load(open(f'{d}/off-{s}.json')); on = json.load(open(f'{d}/on-{s}.json')); onl = json.load(open(f'{d}/onl-{s}.json'))
    txt = open(f'{d}/bw-{s}.txt').read()
    m = re.search(r'# node-hist (.*)', txt)
    bw = [0] * 257
    for cell in m.group(1).split():
        a, b = cell.split(':'); bw[int(a)] = int(b)
    ill = re.search(r'illegal_placements=(\d+)', txt).group(1)
    deep = re.search(r'deepest_depth=(\d+)', txt).group(1)
    late = lambda h: sum(h[201:])
    print(f"seed {s}: fcq-off legal nodes {off['nodes_legal']} deepest {off['deepest']} | fcq-on nodes {on['nodes']} legal {on['nodes_legal']} | "
          f"off.hist_legal==onl.hist_legal {off['hist_legal'] == onl['hist_legal']} late {late(off['hist_legal'])}/{late(onl['hist_legal'])} off-nodes {off['nodes']} | "
          f"bw==fcq-on hist {bw == on['hist']} bw deepest {deep} illegal {ill}")
