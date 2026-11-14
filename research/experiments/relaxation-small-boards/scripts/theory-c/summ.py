"""TOY. Table of the objective suite: per mode, over the (board, start) pairs that EVERY listed
mode has finished, runs solved and evaluations to the first verified rounding (a run that hit its
cap counts as its evaluation count, marked censored). Usage: summ.py dir mode [mode ...]"""
import sys, json, glob, os
import numpy as np
d = sys.argv[1]; modes = sys.argv[2:]
CAP = 12000        # every run is censored here (the first 33 runs had a cap of 25,000)
rows = {}
for f in glob.glob(os.path.join(d, "*.jsonl")):
    for line in open(f):
        r = json.loads(line)
        r["solved"] = bool(r["solved"] and r["evals_solution"] <= CAP)
        r["evals_solution"] = r["evals_solution"] if r["solved"] else None
        r["evals"] = min(r["evals"], CAP)
        rows[(r["mode"], r["board"][3], r["start"])] = r
keys = sorted({(b, s) for (m, b, s) in rows})
common_keys = [k for k in keys if all((m, k[0], k[1]) in rows for m in modes)]
print("pairs finished by all of", modes, ":", len(common_keys), "per board:",
      {b: sum(1 for k in common_keys if k[0] == b) for b in range(5)})
base = modes[0]
for m in modes:
    rs = [rows[(m, b, s)] for (b, s) in common_keys]
    ev = np.array([r["evals_solution"] if r["solved"] else r["evals"] for r in rs], float)
    solved = sum(r["solved"] for r in rs)
    bs = [rows[(base, b, s)] for (b, s) in common_keys]
    bev = np.array([r["evals_solution"] if r["solved"] else r["evals"] for r in bs], float)
    ratio = bev / ev
    out = [r for r in rs if r["solved"]]
    print(f"{m:8s} runs {len(rs):3d} solved {solved:3d}  evals median {np.median(ev):7.0f} mean {ev.mean():7.0f} "
          f"q25 {np.quantile(ev, .25):6.0f} q75 {np.quantile(ev, .75):6.0f} min {ev.min():6.0f} max {ev.max():6.0f}  "
          f"geo-mean ratio {base}/{m} {np.exp(np.log(ratio).mean()):5.2f}  "
          f"at solution: median viol {np.median([r['viol_end'] for r in out]) if out else float('nan'):.1e} "
          f"mass off {np.median([r['mass_off_end'] for r in out]) if out else float('nan'):.2f} "
          f"rho {np.median([r['final_rho'] for r in out]) if out else float('nan'):.0f} "
          f"outer {np.median([r['outer'] for r in out]) if out else float('nan'):.0f}")
    for b in range(5):
        rb = [rows[(m, bb, s)] for (bb, s) in common_keys if bb == b]
        if rb:
            e = [r["evals_solution"] if r["solved"] else r["evals"] for r in rb]
            print(f"      board {b} ({rb[0]['n_sols']} sols, {rb[0].get('sols_after_pin', rb[0]['n_sols'])} after pin): solved {sum(r['solved'] for r in rb)}/{len(rb)} median {np.median(e):.0f}")
