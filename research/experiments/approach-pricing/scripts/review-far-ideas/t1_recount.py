"""TOY BOARDS ONLY. Second analysis recount of T1 (anneal-vs-relaxation): independent verification of
every solved board from its tile list, CPU seconds per verified solution, relaxation rows."""
import json, glob, os, collections
H = os.path.dirname(os.path.abspath(__file__)); A = os.path.join(H, "..", "anneal-vs-relaxation")
def load(b):
    L = open(os.path.join(A, "boards", f"b_{b}.txt")).read().split()
    n, T = int(L[0]), int(L[1]); v = list(map(int, L[2:]))
    return n, [v[4*i:4*i+4] for i in range(T)]
def verify(n, tiles, board):
    if sorted(t for t, r in board) != list(range(n*n)): return False
    c = lambda x, d: tiles[board[x][0]][(d + board[x][1]) & 3]      # sides N E S W, as iso.c col()
    for x in range(n*n):
        r, k = divmod(x, n)
        if (c(x, 0) == 0) != (r == 0) or (c(x, 1) == 0) != (k == n-1): return False
        if (c(x, 2) == 0) != (r == n-1) or (c(x, 3) == 0) != (k == 0): return False
        if k < n-1 and c(x, 1) != c(x+1, 3): return False
        if r < n-1 and c(x, 2) != c(x+n, 0): return False
    return True
for sub in ("raw_m1", "raw_calib_m4"):
    agg = collections.OrderedDict()
    for fn in sorted(glob.glob(os.path.join(A, sub, "*.jsonl"))):
        tag, a, b_, c_, d_, seed = os.path.basename(fn)[:-6].split("_"); b = f"{a}_{b_}_{c_}_{d_}"
        n, tiles = load(b)
        g = agg.setdefault((tag, b), dict(runs=0, claimed=0, ok=0, cpu=0.0, steps=0, fin=collections.Counter(), tsol=[], seeds=set(), bad=0))
        g["seeds"].add(seed)
        for l in open(fn):
            if '"traj"' not in l: continue
            d = json.loads(l); g["runs"] += 1; g["cpu"] += d["cpu_s"]; g["steps"] += d["steps_done"]; g["fin"][d["final_mm"]] += 1
            board = [tuple(p) for p in d["board"]]
            v = verify(n, tiles, board)
            if d["solved_at"] >= 0:
                g["claimed"] += 1; g["ok"] += v; g["tsol"].append(d["cpu_s"])
            elif v: g["bad"] += 1
    print("==", sub)
    for (tag, b), g in agg.items():
        e = g["cpu"] / g["ok"] if g["ok"] else float("inf")
        print(f"{tag:9s} {b}: runs {g['runs']:3d} claimed {g['claimed']:2d} verified {g['ok']:2d} cpu {g['cpu']:8.1f} s  s/sol {e:8.1f}  steps/s {g['steps']/g['cpu']:.3e}  seeds {len(g['seeds'])}  unsolved-but-valid {g['bad']}  final {dict(sorted(g['fin'].items()))}")
print("== relaxation rows")
for name in ("../growth-exponent/results/s6m1.jsonl", "relaxation_rows/s6_x86.jsonl", "../growth-exponent/results/s6x86_partial.jsonl"):
    p = os.path.join(A, name) if not name.startswith("..") else os.path.join(H, name)
    rows = [json.loads(l) for l in open(p) if l.strip()]
    by = collections.OrderedDict()
    for r in rows: by.setdefault(r["board"], []).append(r)
    print(name, len(rows), "rows; keys:", sorted(rows[0].keys())[:40])
    for b in sorted(by):
        R = by[b]; sol = [r for r in R if r.get("solved")]
        tot = sum(r["seconds"] for r in R)
        print(f"  {b}: n {len(R)} solved {len(sol)} statuses {dict(collections.Counter(r['status'] for r in R))} total s {tot:8.0f} s/sol {tot/len(sol) if sol else float('inf'):8.0f}  solved secs {[round(r['seconds']) for r in sol]}  unsolved secs {[round(r['seconds']) for r in R if not r.get('solved')]}  starts {[r['start'] for r in R]}")
