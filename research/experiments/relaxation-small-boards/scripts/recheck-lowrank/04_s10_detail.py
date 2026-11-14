"""TOY. re-check reading of the step rows: S9 against S10 run by run, the stuck starts, work per
solution, state at the first valid rounding, S0 found-then-lost, NumPy stage 1 tally."""
import json, os, glob
from statistics import median, mean
H = os.path.dirname(os.path.abspath(__file__))
R = os.path.join(H, "..", "lowrank-rust", "results")
def load(f): return [json.loads(l) for l in open(os.path.join(R, f))]
s9, s10 = load("s9_nn0_r16.jsonl"), load("s10_nn0_rs1000.jsonl")
k = lambda r: (r["board"], r["start"])
d9 = {k(r): r for r in s9}; d10 = {k(r): r for r in s10}
same = sum(1 for q in d9 if "broken" not in q[0] and d9[q]["solved"] and d9[q]["evals_solution"] == d10[q]["evals_solution"])
print("S9 vs S10: runs with identical evaluation count to the solution:", same, "of", sum(1 for q in d9 if "broken" not in q[0]))
for q in d9:
    if "broken" in q[0]: continue
    if not d9[q]["solved"] or d10[q]["restarts"]:
        a, b = d9[q], d10[q]
        print("  differs:", q, "S9:", a["status"], a["evals"], "evals", a["seconds"], "s, viol", a["viol_max"], "rank", a["rank_1e2"], "mm", a["end_mismatches"],
              "| S10:", b["status"], b["evals_solution"], "evals", b["t_solution"], "s restarts", b["restarts"])
for f in ["s4_nn0.jsonl", "x86/sweep1.jsonl"]:
    for r in load(f):
        if "broken" not in r["board"] and not r["solved"]:
            print("  unsolved in", f, r["tag"], k(r), r["status"], r["evals"], "evals r", r["cfg"]["r"])
good = [r for r in s10 if "broken" not in r["board"]]
bro = [r for r in s10 if "broken" in r["board"]]
print("S10 pooled: evals to solution median", median(r["evals_solution"] for r in good), "mean", round(mean(r["evals_solution"] for r in good)),
      "max", max(r["evals_solution"] for r in good), "| CPU s median", median(r["t_solution"] for r in good), "mean", round(mean(r["t_solution"] for r in good), 2))
for B in (3000, 5000, 10000, 25000):
    print(f"  solved within {B} evaluations: {sum(1 for r in good if r['evals_solution'] <= B)}/50")
for b in sorted({r["board"] for r in good}):
    rs = [r for r in good if r["board"] == b]
    print(f"  {b} sols {rs[0]['nsols']}: mean evals/solution {mean(r['evals_solution'] for r in rs):.0f}, mean CPU s/solution {mean(r['t_solution'] for r in rs):.2f}, max {max(r['t_solution'] for r in rs):.1f}")
print("state at the first valid rounding (S10, 50 runs): viol_max median", median(r["viol_max"] for r in good), "min", min(r["viol_max"] for r in good), "max", max(r["viol_max"] for r in good),
      "| mass off per cell median", median(r["mass_off_per_cell"] for r in good), "| rank_1e2 median", median(r["rank_1e2"] for r in good), "min", min(r["rank_1e2"] for r in good),
      "| final rho median", median(r["final_rho"] for r in good), "| roundings checked per run median", median(r["n_rounds"] for r in good), "total", sum(r["n_rounds"] for r in good))
print("broken control S10: roundings checked", sum(r["n_rounds"] for r in bro), "evals", sum(r["evals"] for r in bro), "viol_max", sorted(r["viol_max"] for r in bro)[0], "to", sorted(r["viol_max"] for r in bro)[-1], "statuses", {r["status"] for r in bro})
tot = 0
for f in glob.glob(R + "/*.jsonl") + glob.glob(R + "/*/*.jsonl"):
    if "kernels" in f: continue
    for l in open(f):
        r = json.loads(l)
        if "broken" in r["board"]: tot += r["n_rounds"]; assert not r["solved"]
print("broken control, every file: roundings checked and rejected", tot)
s0 = load("s0.jsonl")
print("S0 rows", len(s0), "found a valid rounding at some point", sum(r["solved"] for r in s0), "still valid at the end", sum(r["end_is_solution"] for r in s0),
      "by board", {b: (sum(r["end_is_solution"] for r in s0 if r["board"] == b), sum(1 for r in s0 if r["board"] == b)) for b in sorted({r["board"] for r in s0})},
      "status", {r["status"] for r in s0}, "median evals whole run", median(r["evals"] for r in s0))
# NumPy stage 1: greedy alone, Hungarian alone, either
S1 = os.path.join(H, "..", "lowrank-relaxation", "raw", "stage1")
c = {}
for f in sorted(glob.glob(S1 + "/*.jsonl")):
    for l in open(f):
        r = json.loads(l)
        g = r["greedy_round"]; h = r["hungarian_round"] or {}
        gs = g.get("mismatches") == 0; hs = bool(h.get("is_solution"))
        d = c.setdefault((r["mode"], r["r"]), dict(runs=0, greedy=0, hungarian=0, either=0))
        d["runs"] += 1; d["greedy"] += gs; d["hungarian"] += hs; d["either"] += (gs or hs)
print("NumPy stage 1 rows by (mode, rank):")
for k2, v in sorted(c.items()): print("  ", k2, v)
