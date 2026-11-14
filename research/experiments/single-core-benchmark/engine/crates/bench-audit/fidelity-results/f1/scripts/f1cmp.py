import re, sys, os
d = sys.argv[1]
def summ(f):
    t = open(f).read()
    keys = []
    for pat in [r'nodes=(\d+)', r'best_complete_score=(\d+)', r'best_native_score=(\d+)', r'deepest_depth=(\d+)', r'complete-score histogram \(score:count\): (.*)']:
        m = re.search(pat, t); keys.append(m.group(1).strip() if m else None)
    return keys
def url(tag):
    p = os.path.join(d, f'url-{tag}.txt')
    return open(p).read().strip() if os.path.exists(p) else None
same = 0; tot = 0
for kind, seeds in [('nc', [101, 102, 103, 104, 108]), ('of', [501, 502, 503, 504, 505, 506])]:
    for s in seeds:
        a = summ(f'{d}/base-{kind}-{s}.txt'); b = summ(f'{d}/head-{kind}-{s}.txt')
        ua = url(f'b-{kind}-{s}'); ub = url(f'h-{kind}-{s}')
        ok = a == b and ua == ub
        same += ok; tot += 1
        print(f'legacy {kind} {s}: identical={ok} nodes={a[0]} best={a[1]} deepest={a[3]}/{b[3]}')
print(f'legacy pairs identical: {same}/{tot}')
for s in [1, 2, 3, 4]:
    h = lambda f: re.search(r'# node-hist (.*)', open(f).read()).group(1)
    a = h(f'{d}/pre-ce-{s}.txt'); b = h(f'{d}/post-ce-{s}.txt')
    print(f'default centre seed {s}: histogram unchanged={a == b}')
