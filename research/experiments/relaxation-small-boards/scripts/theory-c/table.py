"""TOY. Markdown tables for the memo from raw/suite/*.jsonl and raw/dr_obj.jsonl.
Every low-rank run is censored at 12,000 evaluations. Each mode is compared with the baseline l2
on the (board, start) pairs both have finished."""
import json, glob
import numpy as np
CAP = 12000
rows = {}
for f in glob.glob("raw/suite/*.jsonl"):
    for line in open(f):
        r = json.loads(line)
        ok = bool(r["solved"] and r["evals_solution"] <= CAP)
        r["ok"] = ok; r["ev"] = r["evals_solution"] if ok else CAP
        if not ok and r["evals"] < CAP:
            continue                      # stopped by the clock before the cap: unusable
        rows[(r["mode"], r["board"][3], r["start"])] = r
modes = sorted({k[0] for k in rows})
print("| mode | pairs | solved, mode | median evaluations, mode | solved, baseline | median evaluations, baseline |")
print("|---|---|---|---|---|---|")
for m in ["l2"] + [x for x in modes if x != "l2"]:
    keys = sorted((b, s) for (mm, b, s) in rows if mm == m and ("l2", b, s) in rows)
    if not keys:
        continue
    a = [rows[(m, b, s)] for b, s in keys]; z = [rows[("l2", b, s)] for b, s in keys]
    def med(v):
        x = np.median([r["ev"] for r in v]); return f"{x:,.0f}" if x < CAP else "over 12,000"
    print(f"| {m} | {len(keys)} | {sum(r['ok'] for r in a)} | {med(a)} | {sum(r['ok'] for r in z)} | {med(z)} |")
print()
print("| board (solutions) | solutions left by the hint | placements deleted | starts | solved, no hint | evaluations, no hint | solved, hint | evaluations, hint |")
print("|---|---|---|---|---|---|---|---|")
for b in range(5):
    keys = sorted(s for (mm, bb, s) in rows if mm == "l2@p0" and bb == b and ("l2", b, s) in rows)
    if not keys:
        continue
    a = [rows[("l2@p0", b, s)] for s in keys]; z = [rows[("l2", b, s)] for s in keys]
    fmt = lambda v: ", ".join(f"{r['ev']:,}" if r["ok"] else "none" for r in v)
    print(f"| seed {b} ({a[0]['n_sols']}) | {a[0]['sols_after_pin']} | {a[0]['frozen']} | {len(keys)} | {sum(r['ok'] for r in z)} | {fmt(z)} | {sum(r['ok'] for r in a)} | {fmt(a)} |")
print()
w = [r for (m, b, s), r in rows.items() if m == "l2" and r["ok"]]
print("baseline at the solution:", len(w), "runs; rank_1e2", sorted(r["rank_1e2"] for r in w), "rho", sorted(round(r["final_rho"]) for r in w),
      "mass off", sorted(r["mass_off_end"] for r in w), "viol", sorted(r["viol_end"] for r in w), "evals", sorted(r["ev"] for r in w))
print("baseline all:", sorted((r["board"][3], r["start"], r["ev"] if r["ok"] else None) for (m, b, s), r in rows.items() if m == "l2"))
print()
ORDER_SC = [1, 1, 1, 3, 0.33, 1, 1, 10, 1]      # the command lines of chain_dr.sh then chain_dr2.sh (see raw/NOTES.txt)
print("| board | objective | scale s | iterations run | first valid rounding | mass off under 1e-2 / 1e-5 | end residual | rank of Z | dual rank | eigenvalues within 1e-6 | free pairs: primal positive / multiplier over 1e-8 / neither |")
print("|---|---|---|---|---|---|---|---|---|---|---|")
for i, line in enumerate(open("raw/dr_obj.jsonl")):
    r = json.loads(line)
    sc = ORDER_SC[i]
    obj = {"feas": "none", "lin": f"linear, tau {r['param']}, draw {r['cseed']}", "l2": f"norm, rho {r['param']:.0f}"}[r["mode"]]
    fm = r["first_mass"]; fe = r["free_entries"]["1e-08"]; sp = r["spec"]["1e-06"]
    print(f"| seed {r['seed']} ({r['n_sols']}) | {obj} | {sc} | {r['iters']:,} | {r['first_round'] or 'never'} | {fm.get('0.01', 'never')} / {fm.get('1e-05', 'never')} | {r['rp']:.0e} | {r['rank_Z_1e6']} | {sp['neg']} | {sp['near0']} | {fe['pos']:,} / {fe['neg']:,} / {fe['near0']:,} |")
