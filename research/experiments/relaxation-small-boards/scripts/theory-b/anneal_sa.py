"""TOY BOARDS ONLY. Plain annealing: probe B's annealer (board.Board.anneal: same moves, same
temperature schedule 1.5 (1 - t) + 0.05 over `steps` proposals, same random start), copied here
only to stop at the first board with 0 mismatches and report the step at which it happened.
One sweep = N proposals (N cells). Usage: anneal_sa.py n cf ci seed start steps out.jsonl [broken]
"""
import os, sys, json, time, random
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
from cp_common import load

n, cf, ci, seed, st, steps = map(int, sys.argv[1:7])
out = sys.argv[7]
broken = len(sys.argv) > 8 and sys.argv[8] == "broken"
B, S, C = load(n, cf, ci, seed, broken)
R = random.Random(9000 + st)
t0 = time.time()
js = [int(v) for v in B.ctrl_idx[R.randrange(len(B.ctrl_idx))]]
kinds_l = [[x for x in B.rim_cells[k] if x != 0] for k in range(3)]
NB, TILE, OPT = B.NB, B.TILE, B.OPT
cur = B.mismatches(js); best = cur; hit = None; k = 0
for k in range(steps):
    if cur == 0:
        hit = k
        break
    temp = 1.5 * (1 - k / steps) + 0.05
    cells = kinds_l[R.randrange(3)]
    if len(cells) < 2:
        continue
    x, y = R.sample(cells, 2)
    if cells is kinds_l[0] and R.random() < 0.3:
        y = x
    aff = {x, y} | {z for z in NB[x] if z >= 0} | {z for z in NB[y] if z >= 0}
    before = sum(B._mism_cell(z, js) for z in aff)
    old = (js[x], js[y])
    tx, ty = TILE[js[x]], TILE[js[y]]
    if x == y:
        js[x] = R.choice(OPT[(x, tx)])
    else:
        js[x] = R.choice(OPT[(x, ty)])
        js[y] = R.choice(OPT[(y, tx)])
    delta = (sum(B._mism_cell(z, js) for z in aff) - before) / 2
    if not (delta <= 0 or R.random() < np.exp(-delta / temp)):
        js[x], js[y] = old
    else:
        cur = B.mismatches(js)
        best = min(best, cur)
final = B.mismatches(js)
rec = dict(method="sa", board=[n, cf, ci, seed], broken=broken, start=st, steps_budget=steps, solved=bool(final == 0),
           steps=hit if hit is not None else steps, sweeps=round((hit if hit is not None else steps) / B.N, 1),
           mismatches=int(final), best=int(best), seconds=round(time.time() - t0, 2))
with open(out, "a") as f:
    f.write(json.dumps(rec) + "\n")
print(json.dumps(rec))
