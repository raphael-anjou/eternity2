"""TOY-derived cost model of the best configuration (structured kernel, r = 16, inner 20).
Per evaluation, one core: t = P^2 (a + b r), a and b fitted on the measured 5x5 kernel times
(results/kernels.jsonl, kernel "struct", r = 8, 16, 32). Memory of a P^2-free implementation:
  factor and L-BFGS vectors (2 m + 8) P r doubles, row-block scratch threads x SB x P doubles,
  conflict list 2 K x 12 bytes (index + multiplier), nonnegativity multipliers 2 A x 12 bytes with
  A = af F (af measured at the solve point: 0.84 at r = 16), or 0 when nonnegativity is a pure
  penalty (mult_mode 1).
P, K, F: 5x5 from the exported boards; 6x6, 7x7 and 16x16 from relaxation-engineering.md (section 4)
and relaxation-theory.md (section 4). Expectations, not measurements, beyond 5x5.
Usage: python3 -I py/costmodel.py"""
import json, os
from statistics import median

HERE = os.path.dirname(os.path.abspath(__file__))
t = {}
for l in open(os.path.join(HERE, "..", "results", "kernels.jsonl")):
    d = json.loads(l)
    if d["kernel"] == "struct":
        t.setdefault(d["r"], []).append(d["ms_per_eval_cpu"] * 1e-3)
P5 = 478
xs = sorted(t)
ys = [median(t[r]) / P5 ** 2 for r in xs]
n = len(xs); mx = sum(xs) / n; my = sum(ys) / n
b = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)
a = my - b * mx
print(f"fit: t_eval = P^2 (a + b r), a = {a:.3e} s, b = {b:.3e} s  (points r={xs}: {[f'{median(t[r])*1e3:.2f} ms' for r in xs]})")
m, r, SB, cores, af = 10, 16, 64, 10, 0.84
sizes = [("5x5", 478, 27203), ("6x6", 1290, 1.6e5), ("7x7", 2916, 6.2e5), ("10x10", 17424, 1.1e7),
         ("12x12", 41616, 4.3e7), ("16x16", 156816, 3.3e8)]
print("| board | P | K | F | s per eval, 1 core | s per eval, 10 cores (ideal) | GB vectors+scratch | GB conflict list | GB nonneg multipliers (A = 0.84 F) | GB total, pure penalty |")
print("|---|---|---|---|---|---|---|---|---|---|")
for name, P, K in sizes:
    F = P * (P - 1) / 2 - K
    te = P * P * (a + b * r)
    gv = ((2 * m + 8) * P * r * 8 + cores * SB * P * 8) / 1e9
    gk = 2 * K * 12 / 1e9
    ga = 2 * af * F * 12 / 1e9
    print(f"| {name} | {P:,} | {K:.2g} | {F:.2g} | {te:.3g} | {te / cores:.3g} | {gv:.2g} | {gk:.2g} | {ga:.3g} | {gv + gk:.2g} |")
# where the multiplier store passes 32 GB: 24 af F = 32e9
Pc = (2 * 32e9 / (24 * af)) ** 0.5
print(f"nonnegativity multiplier store passes 32 GB at P ~ {Pc:,.0f}")
