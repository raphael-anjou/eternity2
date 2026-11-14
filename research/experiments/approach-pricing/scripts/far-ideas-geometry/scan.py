"""TOY. Solution counts and DFS nodes for small (n, cf, ci) to pick boards for the probes."""
import sys, time
from common import gen, enumerate_all

n = int(sys.argv[1])
for cf, ci in [tuple(map(int, a.split(","))) for a in sys.argv[2:]]:
    for seed in range(3):
        t = gen(n, cf, ci, seed)
        if t is None:
            print(n, cf, ci, seed, "no board"); continue
        t0 = time.time()
        r = enumerate_all(n, t)
        print(f"n={n} cf={cf} ci={ci} seed={seed} sols={len(r['sols'])} nodes={r['nodes']} "
              f"peak={max(r['widths'])}@{r['widths'].index(max(r['widths']))} {time.time()-t0:.1f}s", flush=True)
