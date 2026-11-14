"""TOY. re-check tally of theory C's low-rank suite (scripts/theory-c/raw/suite/*.jsonl), standard
library only: per mode, runs, solved inside 12,000 evaluations, and the paired comparison with the
baseline on the same (board, start), with an exact two-sided sign test on the discordant pairs."""
import json, glob, os, math, sys
H = os.path.dirname(os.path.abspath(__file__))
rows = []
for f in sorted(glob.glob(os.path.join(H, "..", "theory-c", "raw", "suite", "*.jsonl"))):
    for l in open(f):
        l = l.strip()
        if l: rows.append(json.loads(l))
print("rows", len(rows), "keys", sorted(rows[0].keys()))
if "--keys" in sys.argv: sys.exit()
CAP = 12000
def key(r): return (tuple(r["board"]) if isinstance(r["board"], list) else r["board"], r["start"])
def ev(r):
    e = r.get("evals_first_valid") or r.get("evals_solution") or r.get("first_valid_evals")
    return e
modes = {}
for r in rows: modes.setdefault(r.get("mode", "?"), {})[key(r)] = r
print({m: len(v) for m, v in modes.items()})
def solved(r): return bool(r["solved"]) and r["evals_solution"] <= CAP
base = modes["l2"]
print("baseline l2: runs", len(base), "solved inside 12,000:", sum(solved(r) for r in base.values()),
      "| by start:", {s: (sum(solved(r) for k, r in base.items() if k[1] == s), sum(1 for k in base if k[1] == s)) for s in sorted({k[1] for k in base})},
      "| by board:", {k0[3]: (sum(solved(r) for k, r in base.items() if k[0] == k0), sum(1 for k in base if k[0] == k0)) for k0 in sorted({k[0] for k in base})})
print("baseline solved evals:", sorted(r["evals_solution"] for r in base.values() if solved(r)))
print("statuses of unsolved baseline runs:", sorted((r["status"], r["evals"]) for r in base.values() if not solved(r)))
for m, d in modes.items():
    if m == "l2": continue
    both = [k for k in d if k in base]
    a = sum(solved(d[k]) for k in both); b = sum(solved(base[k]) for k in both)
    win = sum(1 for k in both if solved(d[k]) and not solved(base[k])); lose = sum(1 for k in both if solved(base[k]) and not solved(d[k]))
    n = win + lose
    p = min(1.0, 2 * sum(math.comb(n, i) for i in range(min(win, lose) + 1)) / 2 ** n) if n else 1.0
    print(f"{m:8s} pairs {len(both):2d}: mode solved {a}, baseline solved {b}; discordant: mode only {win}, baseline only {lose}; exact sign test p = {p:.3f}; starts used {sorted({k[1] for k in both})}")
