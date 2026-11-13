"""Derived figures for the corrected entropy-area-law results file.

Reads the committed outputs beside this script: strip_entropy.out, and the
two seeded runs of distinct_fraction.js (distinct_fraction.out, seeds 1 to 6;
distinct_fraction.seed100.out, seeds 101 to 106).
Stdlib only:  python3 -I entropy_fields.py

Prints:
  1. h(n) for widths 1 to 6 and the row increments, with the upper bound on
     h_infinity that the infimum gives (the smallest computed h(n));
  2. the sampled distinct-piece fractions and -ln(rho) per cell, per seeded run;
  3. the cell count where the sampled fraction crosses 1e-3, by log-linear
     interpolation between the two sampled sizes that bracket it, per run;
  3b. the spread between the two runs, in units of their combined standard error;
  4. what the withdrawn fit rho = exp(-0.0445 n^2) predicted at the sampled
     sizes, against the samples;
  5. the withdrawn h_infinity 0.67 against h(6).
"""
import math
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))

strip = open(os.path.join(HERE, "strip_entropy.out")).read()
widths = [(int(m[0]), float(m[1]), float(m[2])) for m in re.findall(r"n=(\d) lambda=([\d.]+) h=([\d.]+)", strip)]
lam = {n: l for n, l, _ in widths}
# increments log10(lambda_n / lambda_(n-1)), from the printed eigenvalues
incs = [(n, math.log10(lam[n] / lam[n - 1])) for n in sorted(lam) if n - 1 in lam]
print("width  lambda      h(n)    increment")
inc = dict(incs)
for n, lam, h in widths:
    print(f"{n:5d}  {lam:10.3f}  {h:.4f}  {inc.get(n, float('nan')):.4f}")
hmin = min(h for _, _, h in widths)
print(f"h_infinity = inf h(n) <= {hmin:.4f} (width 6)")
print(f"withdrawn h_infinity 0.67 exceeds h(6) = {widths[5][2]:.4f}: impossible for an infimum")
print(f"h(1) - increment = {widths[0][2] - inc[6]:.4f} (the constant in h(n) ~ 0.435 + c/n)")
print(f"mean-field log10(784/289) = {math.log10(784 / 289):.4f}")

def read_runs(name):
    txt = open(os.path.join(HERE, name)).read()
    return [(int(a), int(b), float(r), float(se), int(ns)) for a, b, r, se, ns in
            re.findall(r"(\d+)x(\d+) \(\d+ cells\): rho = ([\d.]+) \(s.e. ([\d.]+)\), (\d+) samples", txt)]


runs = [("seeds 1-6", read_runs("distinct_fraction.out"))]
if os.path.exists(os.path.join(HERE, "distinct_fraction.seed100.out")):
    runs.append(("seeds 101-106", read_runs("distinct_fraction.seed100.out")))
target = math.log(1e-3)
for label, rows in runs:
    print()
    print(f"[{label}]")
    print("patch  cells  rho        s.e.      -ln(rho)/cell  old fit exp(-0.0445 cells)  fit/sample")
    for a, b, r, se, ns in rows:
        cells = a * b
        fit = math.exp(-0.0445 * cells)
        print(f"{a}x{b:<3d} {cells:5d}  {r:<9.6g}  {se:<8.2g}  {-math.log(r) / cells:13.3f}  {fit:26.4g}  {fit / r:10.1f}")
    pts = [(a * b, r) for a, b, r, _, _ in rows]
    for (c0, r0), (c1, r1) in zip(pts, pts[1:]):
        if math.log(r0) >= target > math.log(r1):
            x = c0 + (c1 - c0) * (target - math.log(r0)) / (math.log(r1) - math.log(r0))
            print(f"sampled rho crosses 1e-3 between {c0} and {c1} cells: {x:.1f} cells (log-linear interpolation)")
if len(runs) == 2:
    print()
    print("spread between the two runs (difference / sqrt(se1^2 + se2^2)):")
    for (a, b, r1, s1, _), (_, _, r2, s2, _) in zip(runs[0][1], runs[1][1]):
        z = (r2 - r1) / math.sqrt(s1 * s1 + s2 * s2)
        print(f"  {a}x{b:<3d} {r1:<9.6g} {r2:<9.6g} difference {r2 - r1:+.3g}  z = {z:+.2f}")
print("withdrawn collapse: 155 cells, from cells = ln(1000)/0.0445 = %.1f" % (math.log(1000) / 0.0445))
