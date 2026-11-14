"""TOY. Near-solution controls through nearctl.c (C port of probe B's two generators).
Usage: controls_c.py n cf ci seed anneal_steps anneal_seconds dfs_budget dfs_seconds out.json
Writes the board model to raw/model_<board>.txt, runs the annealer then the DFS sampler (one process
at a time), checks every returned board in Python (mismatches <= 2, >= 8 cells from every solution,
each tile used once) and writes out.json.
"""
import os, sys, json, subprocess
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from board import Board

n, cf, ci, seed, steps = map(int, sys.argv[1:6])
asec = int(sys.argv[6]); budget = int(sys.argv[7]); dsec = int(sys.argv[8]); out = sys.argv[9]
B = Board(n, cf, ci, seed)
tag = f"{n}_{cf}_{ci}_{seed}"
model = os.path.join(HERE, "raw", f"model_{tag}.txt")
pin_p = B.index[(0, B.pin[0], B.pin[1])]
with open(model, "w") as f:
    f.write(f"{B.N} {B.P} {B.T} {n} {len(B.sols)} {len(B.ctrl_idx)} {pin_p}\n")
    for j in range(B.P):
        f.write(f"{B.cellOf[j]} {B.tileOf[j]} " + " ".join(map(str, B.cols[j])) + "\n")
    for x in range(B.N):
        f.write(" ".join(map(str, B.nb[x])) + "\n")
    for s in B.sol_idx:
        f.write(" ".join(map(str, s)) + "\n")
    for s in B.ctrl_idx:
        f.write(" ".join(map(str, s)) + "\n")
    f.write("3\n")
    for k in range(3):
        cells = [x for x in B.rim_cells[k] if x != 0]
        f.write(f"{len(cells)} " + " ".join(map(str, cells)) + "\n")
exe = os.path.join(HERE, "raw", "nearctl")
if not os.path.exists(exe) or os.path.getmtime(exe) < os.path.getmtime(os.path.join(HERE, "nearctl.c")):
    subprocess.run(os.environ.get("CC", "cc").split() + ["-O2", "-o", exe, os.path.join(HERE, "nearctl.c"), "-lm"], check=True)


def run(mode, a, secs):
    r = subprocess.run([exe, model, mode, str(a), str(secs), "20", "2", "7"], capture_output=True, text=True, check=True)
    boards, stats = [], ""
    for line in r.stdout.splitlines():
        if line.startswith("#"):
            stats = line
        elif line.strip():
            boards.append([int(v) for v in line.split()])
    for js in boards:
        assert B.mismatches(js) <= 2 and B.dist_to_sols(js) >= 8
        assert len({int(B.tileOf[j]) for j in js}) == B.N
        assert all(int(B.cellOf[j]) == x for x, j in enumerate(js))
    return boards, stats


ann, sa = run("anneal", steps, asec)
smp, ss = run("dfs", budget, dsec)
res = dict(board=[n, cf, ci, seed], n_sols=len(B.sols), P=B.P, anneal_stats=sa, dfs_stats=ss,
           annealed=ann, sampled=smp,
           annealed_mismatch=[B.mismatches(a) for a in ann], sampled_mismatch=[B.mismatches(s) for s in smp],
           annealed_dist=[B.dist_to_sols(a) for a in ann], sampled_dist=[B.dist_to_sols(s) for s in smp])
json.dump(res, open(out, "w"))
print(json.dumps({k: v for k, v in res.items() if k not in ("annealed", "sampled")}))
