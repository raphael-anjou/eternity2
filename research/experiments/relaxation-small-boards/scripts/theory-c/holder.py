"""TOY. From dr_obj.py rows: the slope of log(mass off solutions per placement) against
log(largest violation) along the trajectory, in windows. Usage: holder.py raw/dr_obj.jsonl"""
import sys, json
import numpy as np
for line in open(sys.argv[1]):
    r = json.loads(line)
    rows = [x for x in r["rows"] if x["mass_off"] > 1e-9 and x["viol"] > 0]
    it = np.array([x["it"] for x in rows]); v = np.array([x["viol"] for x in rows])
    m = np.array([x["mass_off"] for x in rows]) * 25 / r["P"]
    vn = np.array([x["vnorm"] for x in rows])
    print(r["mode"], r["param"], "seed", r["seed"], "iters", r["iters"], "first_round", r["first_round"], "first_mass", r["first_mass"],
          "rankZ", r["rank_Z_1e6"], "spec", r["spec"]["1e-06"], "free", r["free_entries"]["1e-06"], "x_int", r["x_int"])
    for a, b in ((100, 500), (500, 1500), (1500, 3000), (3000, 6000)):
        k = (it >= a) & (it <= b)
        if k.sum() > 5:
            sl = np.polyfit(np.log(v[k]), np.log(m[k]), 1)[0]
            print(f"   it {a}-{b}: viol {v[k][0]:.1e} -> {v[k][-1]:.1e}, mass/placement {m[k][0]:.1e} -> {m[k][-1]:.1e}, "
                  f"slope {sl:.2f}, |v| {vn[k][0]:.2f} -> {vn[k][-1]:.2f}")
