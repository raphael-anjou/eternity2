"""TOY. Stage 2 reading. These rules were fixed before any 6x6 or 7x7 row was read.
  solved               the greedy rounding (or probe B's Hungarian rounding) is a solution;
  converged and wrong  not solved, and either the run reached feasibility (max violation <= FEAS2,
                       the largest violation among the 5x5 runs that were successes and rounded to
                       a solution) or its violation had stalled;
  not converged        everything else (still improving when the clock stopped).
Stalled: rho at its cap (1e5) for the last third of the run's wall time, and the smallest violation
in that last third is at least 0.8 times the smallest violation in the middle third (less than
20 percent progress over the final third).
Usage: python3 -I classify.py raw/stage1 raw/stage2 > raw/stage2_summary.txt"""
import glob, json, os, sys
from statistics import median


def rows(d):
    out = []
    for f in sorted(glob.glob(os.path.join(d, "*.jsonl"))):
        out += [json.loads(l) for l in open(f)]
    return out


def solved(r):
    return r["greedy_round"]["is_solution"] or bool((r.get("hungarian_round") or {}).get("is_solution"))


s1 = [r for r in rows(sys.argv[1]) if r["mode"] == "random"]
ok5 = [r for r in s1 if r["viol"]["max"] <= 1e-3 and r["mass_off_solution_placements_per_cell"] < 0.05 and solved(r)]
FEAS2 = max(r["viol"]["max"] for r in ok5)
print(f"FEAS2 = {FEAS2:.3g} (largest violation among {len(ok5)} 5x5 runs that were successes and rounded to a solution)\n")


def stalled(r):
    h = r.get("hist")
    if not h or len(h) < 6:
        return None
    T = h[-1]["t"]
    mid = [e for e in h if T / 3 <= e["t"] < 2 * T / 3]
    last = [e for e in h if e["t"] >= 2 * T / 3]
    if not mid or not last or any(e["rho"] < 1e5 for e in last):
        return False
    return min(e["viol"] for e in last) >= 0.8 * min(e["viol"] for e in mid)


groups = {}
for r in rows(sys.argv[2]):
    groups.setdefault((tuple(r["board"]), r["r"]), []).append(r)
print("| board | r | runs | solved | converged and wrong | not converged | median final violation | median mass off | median greedy mismatches | median evals | host | median s |")
print("|" + "---|" * 12)
tot = {}
for key in sorted(groups):
    rs = groups[key]
    c = dict(solved=0, wrong=0, nc=0)
    for r in rs:
        if solved(r):
            c["solved"] += 1
        elif r["viol"]["max"] <= FEAS2 or stalled(r):
            c["wrong"] += 1
        else:
            c["nc"] += 1
    b, rr = key
    t = tot.setdefault((b[0], rr), [0, 0, 0, 0]); t[0] += len(rs); t[1] += c["solved"]; t[2] += c["wrong"]; t[3] += c["nc"]
    print("| " + " | ".join([f"{b[0]}x{b[0]} {b[1]}/{b[2]} s{b[3]} ({rs[0]['n_sols']} sols)", str(rr), str(len(rs)), str(c["solved"]), str(c["wrong"]), str(c["nc"]),
                             f"{median(r['viol']['max'] for r in rs):.3g}", f"{median(r['mass_off_solution_placements_per_cell'] for r in rs):.3g}",
                             f"{median(r['greedy_round']['mismatches'] for r in rs):g}", f"{median(r['evals'] for r in rs):g}",
                             ",".join(sorted({r.get('host', '?') for r in rs})), f"{median(r['seconds'] for r in rs):.0f}"]) + " |")
print()
for (n, rr), t in sorted(tot.items()):
    print(f"{n}x{n} r={rr}: runs {t[0]}, solved {t[1]}, converged and wrong {t[2]}, not converged {t[3]}")
