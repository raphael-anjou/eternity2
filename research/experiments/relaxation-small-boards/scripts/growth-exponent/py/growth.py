"""TOY BOARDS ONLY. Growth of evaluations-to-solution with the number of placements P.

Each argument is label=tag[,tag...]: the rows of results/<tag>.jsonl (one row per start, from
run.sh or run_mac.sh) pooled under one label, one label per (size, machine). Capped starts are
CENSORED: they enter as "no solution within the evaluations they made", never as failures or
successes. Printed, per board and per label:
  - solved / starts;
  - expected evaluations and expected CPU seconds per verified solution = everything spent (capped
    starts in full, solved starts up to their solution) / solutions found. Under a constant hazard
    (the restart portfolio below makes that the natural model) this is the maximum-likelihood mean
    time to a solution with censoring;
  - Kaplan-Meier median of evaluations to a solution (capped starts censored at their count);
  - attempts: the solver restarts from a fresh random point every RESTART outer steps without a
    solution, so a start is a sequence of attempts. Per solve, the outer step inside its attempt
    at which it landed; per label, solutions per attempt-equivalent (outer steps / RESTART).
  - broken control: valid roundings (must be 0).
Then the exponent of both statistics from the smallest to the largest P of each machine group,
with a bootstrap over boards (boards resampled with replacement within each size, 10,000 draws),
and the EXTRAPOLATION to P = 156,816 (16x16) as evaluations and as days at 12 s per evaluation.

Usage: uv run --with numpy python -I py/growth.py 5mac=s5_memo_s10 6mac=s6mac,s6mac_broken ...
       [fit=5mac:6mac fit=5x86:6x86]
"""
import json
import os
import sys

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
P16 = 156816
SEC16 = 12.0
RESTART = 1000


def load(tags):
    rows = []
    for t in tags:
        rows += [json.loads(l) for l in open(os.path.join(HERE, "..", "results", f"{t}.jsonl"))]
    return rows


def spent(r):
    return (r["evals_solution"], r["t_solution"]) if r["solved"] else (r["evals"], r["seconds"])


def eps(rows):
    """(expected evaluations per solution, expected CPU s per solution, solutions)."""
    s = sum(r["solved"] for r in rows)
    e = sum(spent(r)[0] for r in rows)
    t = sum(spent(r)[1] for r in rows)
    return (e / s if s else float("inf")), (t / s if s else float("inf")), s


def km_median(rows):
    """Kaplan-Meier median of evaluations to a solution, capped starts censored."""
    ev = sorted((spent(r)[0], r["solved"]) for r in rows)
    n = len(ev)
    surv = 1.0
    for i, (x, d) in enumerate(ev):
        if d:
            surv *= 1 - 1 / (n - i)
            if surv <= 0.5:
                return float(x)
    return float("inf")


def attempts(rows):
    inside = [r["outer_solution"] - RESTART * r["restarts"] for r in rows if r["solved"]]
    steps = sum((r["outer_solution"] if r["solved"] else r["outer"]) for r in rows)
    return inside, steps / RESTART


def show(label, rows):
    good = [r for r in rows if "broken" not in r["board"]]
    broken = [r for r in rows if "broken" in r["board"]]
    P = good[0]["P"]
    boards = sorted({r["board"] for r in good})
    print(f"\n== {label}: P = {P}, kernel {good[0]['cfg']['kernel']}, {len(good)} starts on {len(boards)} boards")
    print("  board (solutions) | solved | E evals per solution | E CPU s per solution | KM median evals | solve step inside its attempt | solutions per attempt")
    for b in boards + ["ALL"]:
        rb = good if b == "ALL" else [r for r in good if r["board"] == b]
        e, t, s = eps(rb)
        ins, att = attempts(rb)
        ns = "" if b == "ALL" else f" ({rb[0]['nsols']})"
        print(f"  {b}{ns} | {s}/{len(rb)} | {e:,.0f} | {t:,.0f} | {km_median(rb):,.0f} | {sorted(ins)} | {s}/{att:.1f} = {s / att:.3f}")
    caps = [r for r in good if not r["solved"]]
    if caps:
        st = sorted({r["status"] for r in caps})
        print(f"  capped starts: {len(caps)}, status {st}; evals at the cap median {np.median([r['evals'] for r in caps]):,.0f};"
              f" viol_max {min(r['viol_max'] for r in caps):.1e} to {max(r['viol_max'] for r in caps):.1e};"
              f" rank (1e-3) {sorted({r['rank_1e3'] for r in caps})}; restarts {sorted({r['restarts'] for r in caps})};"
              f" end mismatches {min(r['end_mismatches'] for r in caps)} to {max(r['end_mismatches'] for r in caps)};"
              f" converged-and-wrong: {sum(r['status'] == 'converged' for r in caps)}")
    sol = [r for r in good if r["solved"]]
    if sol:
        print(f"  solved: viol_max at the solution median {np.median([r['viol_max'] for r in sol]):.1e}, rank (1e-3) {sorted({r['rank_1e3'] for r in sol})},"
              f" all in enumeration: {all(r['solution_in_enumeration'] for r in sol)}; ms per evaluation (CPU) median {np.median([1e3 * r['t_solution'] / r['evals_solution'] for r in sol]):.2f}")
    if broken:
        print(f"  broken control: {sum(r['solved'] for r in broken)}/{len(broken)} valid roundings, status {sorted({r['status'] for r in broken})}, evals median {np.median([r['evals'] for r in broken]):,.0f}")
    return P, {b: [r for r in good if r["board"] == b] for b in boards}


def fit(groups, a, b, rng):
    (pa, da), (pb, db) = groups[a], groups[b]
    lr = np.log(pb / pa)
    for name, f in (("expected evals per solution", lambda rows: eps(rows)[0]), ("KM median evals", km_median)):
        ya, yb = f(sum(da.values(), [])), f(sum(db.values(), []))
        k = np.log(yb / ya) / lr
        boot, bad = [], 0
        for _ in range(10000):
            ra = sum((da[x] for x in rng.choice(list(da), len(da))), [])
            rb = sum((db[x] for x in rng.choice(list(db), len(db))), [])
            u, v = f(ra), f(rb)
            if np.isfinite(u) and np.isfinite(v):
                boot.append(np.log(v / u) / lr)
            else:
                bad += 1
        lo, hi = np.percentile(boot, [2.5, 97.5]) if boot else (float("nan"),) * 2
        x = lambda kk: yb * (P16 / pb) ** kk
        print(f"\nFIT {a} -> {b}, {name}: TWO SIZES (P = {pa:.0f}, {pb:.0f}), values {ya:,.0f} -> {yb:,.0f}")
        print(f"  exponent {k:.2f}, bootstrap 95% over boards [{lo:.2f}, {hi:.2f}] ({len(boot)} draws, {bad} with no solution at one size)")
        print(f"  EXTRAPOLATION to 16x16: {x(k):.3g} evaluations = {x(k) * SEC16 / 86400:.3g} days at 12 s per evaluation;"
              f" interval {x(lo):.3g} to {x(hi):.3g} evaluations = {x(lo) * SEC16 / 86400:.3g} to {x(hi) * SEC16 / 86400:.3g} days")


def main(args):
    groups, fits = {}, []
    for a in args:
        k, v = a.split("=", 1)
        if k == "fit":
            fits.append(v.split(":"))
        else:
            groups[k] = show(k, load(v.split(",")))
    rng = np.random.default_rng(0)
    for a, b in fits:
        fit({k: (float(p), d) for k, (p, d) in groups.items()}, a, b, rng)


if __name__ == "__main__":
    main(sys.argv[1:])
