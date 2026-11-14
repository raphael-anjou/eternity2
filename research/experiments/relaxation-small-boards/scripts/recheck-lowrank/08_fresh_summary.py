"""TOY. re-check: the best configuration (S10, unchanged) on starts and boards it was not selected
on. Rows produced by a scratch build of the crate's own source (see README.txt), copied to raw/.
Usage: python3 -I 08_fresh_summary.py"""
import json, os, glob, math
from statistics import median, mean
H = os.path.dirname(os.path.abspath(__file__))
def bcdf(k, n, p): return sum(math.comb(n, i) * p ** i * (1 - p) ** (n - i) for i in range(k + 1))
def bis(f, lo=0.0, hi=1.0):
    for _ in range(100):
        m = (lo + hi) / 2
        if f(m) > 0: lo = m
        else: hi = m
    return lo
def cp(k, n):
    return (bis(lambda p: 0.025 - (1 - bcdf(k - 1, n, p))) if k else 0.0, bis(lambda p: bcdf(k, n, p) - 0.025) if k < n else 1.0)
for f in sorted(glob.glob(os.path.join(H, "raw", "fresh_*.jsonl"))):
    rows = [json.loads(l) for l in open(f)]
    ok = [r for r in rows if r["solved"]]
    print(f"== {os.path.basename(f)}: runs {len(rows)}, cap {rows[0]['cfg']['time_limit']} CPU s, cfg r={rows[0]['cfg']['r']} inner={rows[0]['cfg']['inner_iters']} mult_mode={rows[0]['cfg']['mult_mode']} restart_outer={rows[0]['cfg']['restart_outer']} kernel={rows[0]['cfg']['kernel']}")
    lo, hi = cp(len(ok), len(rows))
    e = [r["evals_solution"] for r in ok]
    print(f"   solved inside the cap {len(ok)}/{len(rows)} = {len(ok)/len(rows):.1%}, 95% [{lo:.1%}, {hi:.1%}]; not in enumeration {sum(1 for r in ok if r['solution_in_enumeration'] is False)}; runs that restarted {sum(1 for r in rows if r['restarts'])}")
    print(f"   evaluations to the solution: median {median(e):.0f}, mean {mean(e):.0f}, p90 {sorted(e)[int(0.9*(len(e)-1))]}, max {max(e)}; CPU s median {median(r['t_solution'] for r in ok):.2f}, mean {mean(r['t_solution'] for r in ok):.2f}")
    for B in (3000, 5000, 10000, 25000):
        k = sum(1 for r in rows if r["solved"] and r["evals_solution"] <= B); lo, hi = cp(k, len(rows))
        print(f"   solved inside {B} evaluations: {k}/{len(rows)} = {k/len(rows):.0%} [{lo:.0%}, {hi:.0%}]")
    k1 = sum(1 for r in rows if r["solved"] and r["restarts"] == 0); lo, hi = cp(k1, len(rows))
    print(f"   solved before any restart (first 1,000 outer steps): {k1}/{len(rows)} = {k1/len(rows):.1%} [{lo:.1%}, {hi:.1%}]")
    for b in sorted({r["board"] for r in rows}, key=lambda s: int(s.split("_s")[1])):
        rs = [r for r in rows if r["board"] == b]; o = [r for r in rs if r["solved"]]
        print(f"     {b} (solutions {rs[0]['nsols']}): {len(o)}/{len(rs)}, median evals {median(r['evals_solution'] for r in o) if o else float('nan'):.0f}, max {max((r['evals_solution'] for r in o), default=0)}, restarted {sum(1 for r in rs if r['restarts'])}")
    for r in rows:
        if not r["solved"]: print("     UNSOLVED", r["board"], r["start"], r["status"], r["evals"], "evals, restarts", r["restarts"], "viol", r["viol_max"], "rank", r["rank_1e2"])
