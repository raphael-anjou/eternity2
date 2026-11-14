"""TOY. re-check: (1) the cost model recomputed; (2) the 6x6 expectation against the growth rows;
(3) the growth of the work per solution from 5x5 to 6x6 with capped runs treated as censored.
Local rows only (raw/s6mac_snapshot.jsonl, a copy of growth-exponent/results/s6m1.jsonl; lowrank-rust
S10 rows). The x86 rows are not on this machine: the project's reading in the run log (01:12) is
entered below as REPORTED, with the third solved count unknown and bracketed.
Standard library only (exact Poisson and Clopper-Pearson bounds by bisection).
Usage: python3 -I 07_growth_and_cost.py"""
import json, os, math
from statistics import median, mean
H = os.path.dirname(os.path.abspath(__file__))
def load(p): return [json.loads(l) for l in open(os.path.join(H, "..", p))]

print("== 1. cost model")
a, b = 3.924e-9, 6.715e-11
for name, P in (("5x5", 478), ("6x6", 1290), ("7x7", 2916), ("12x12", 41616), ("16x16", 156816)):
    print(f"  {name}: P {P}, t = P^2 (a + 16 b) = {P*P*(a+16*b):.4g} s")
for n in (5, 6, 7, 8, 12, 16):
    inner = (n - 2) ** 2; edge = 4 * (n - 2)
    print(f"  placements {n}x{n}: no pin {16 + edge*edge + 4*inner*inner}, first corner pinned {10 + edge*edge + 4*inner*inner}")
P = 156816; K = 3.3e8; F = P*(P-1)/2 - K
print(f"  16x16: pairs {P*(P-1)/2:.3g}, F {F:.3g}; conflict list 2K x 12 B = {2*K*12/1e9:.1f} GB; vectors (2m+8) P r x 8 B = {28*P*16*8/1e9:.2f} GB; scratch 10 x 64 x P x 8 B = {10*64*P*8/1e9:.2f} GB; nonneg multipliers 2 x 0.84 F x 12 B = {2*0.84*F*12/1e9:.0f} GB")
print(f"  16x16 AS CODED (src/board.rs, al.rs, solver.rs: kind u8, conf_idx u32, confm f64, freem f64, pl f64, gram f64 with fast_outer=1): {P*P*(1+4+8+8+8+8)/1e9:.0f} GB")
print(f"  5x5 working set: V {478*16*8/1e3:.0f} kB, 64-row block {64*478*8/1e3:.0f} kB (cache resident); 16x16: V {P*16*8/1e6:.0f} MB, block {64*P*8/1e6:.0f} MB per thread")
print(f"  3,017 evaluations at 12.3 s: {3017*12.3/3600:.1f} h")

print("== 2. 5x5 reference (S10, the rows of the memo)")
s10 = [r for r in load("lowrank-rust/results/s10_nn0_rs1000.jsonl") if "broken" not in r["board"]]
e5 = [r["evals_solution"] for r in s10]; t5 = [r["t_solution"] for r in s10]
print(f"  n {len(e5)}, median evals {median(e5):.0f}, mean (= evals per solution) {mean(e5):.0f}, CPU s per solution {mean(t5):.2f}, s per evaluation {sum(t5)/sum(e5)*1e3:.3f} ms")
few = [r["evals_solution"] for r in s10 if r["nsols"] <= 14]
print(f"  boards with at most 14 solutions (seeds 1, 2, 4; the 6x6 boards have 3 to 14): mean {mean(few):.0f}, median {median(few):.0f}")

print("== 3. 6x6 rows on this Mac")
s6 = [json.loads(l) for l in open(os.path.join(H, "raw", "s6mac_snapshot.jsonl"))]
for r in s6: print(f"  {r['board']} start {r['start']}: {r['status']}, evals {r['evals']}, CPU s {r['seconds']:.0f}, restarts {r['restarts']}, outer {r['outer']}")
ev = sum(r["evals"] for r in s6); cpu = sum(r["seconds"] for r in s6); k = sum(r["solved"] for r in s6)
ms6 = cpu / ev * 1e3
print(f"  rows {len(s6)}, solved {k}, total evals {ev}, CPU s {cpu:.0f}, per evaluation (whole run) {ms6:.2f} ms; model 8.32 ms for the evaluation alone, x 1/0.75 (5x5 share of the evaluation in a run) = {8.32/0.75:.1f} ms")
print(f"  cost per evaluation: observed / model = {ms6/8.32:.2f} (evaluation only), {ms6/(8.32/0.75):.2f} (whole run)")

def bisect(f, lo, hi):
    for _ in range(200):
        mid = (lo + hi) / 2
        if f(mid) > 0: lo = mid
        else: hi = mid
    return (lo + hi) / 2
def pcdf(k, lam): return sum(math.exp(-lam) * lam ** i / math.factorial(i) for i in range(k + 1))
def bcdf(k, n, p): return sum(math.comb(n, i) * p ** i * (1 - p) ** (n - i) for i in range(k + 1))
def pois(k, cl=0.95):
    a = (1 - cl) / 2
    lo = bisect(lambda l: a - (1 - pcdf(k - 1, l)), 0, 1000) if k else 0.0     # P(X >= k) = a
    hi = bisect(lambda l: pcdf(k, l) - a, 0, 1000)                             # P(X <= k) = a
    return lo, hi
def cp(k, n, cl=0.95):
    a = (1 - cl) / 2
    lo = bisect(lambda p: a - (1 - bcdf(k - 1, n, p)), 0, 1) if k else 0.0
    hi = bisect(lambda p: bcdf(k, n, p) - a, 0, 1) if k < n else 1.0
    return lo, hi
ratio = math.log(1290/478)
def report(label, ev, k):
    lo, hi = pois(k)
    eps, e_lo, e_hi = ev/k, ev/hi, ev/lo
    print(f"  {label}: {k} solutions in {ev:.4g} evaluations: {eps:.3g} per solution, 95% [{e_lo:.3g}, {e_hi:.3g}] (Poisson, constant rate)")
    for base, bl in ((mean(e5), "all five 5x5 boards"), (mean(few), "5x5 boards with <= 14 solutions")):
        print(f"     against {bl} ({base:.0f}): factor {eps/base:.0f} [{e_lo/base:.0f}, {e_hi/base:.0f}], two-point exponent in P {math.log(eps/base)/ratio:.1f} [{math.log(e_lo/base)/ratio:.1f}, {math.log(e_hi/base)/ratio:.1f}],"
              f" per added cell x{(eps/base)**(1/11):.2f} [{(e_lo/base)**(1/11):.2f}, {(e_hi/base)**(1/11):.2f}]")
    return eps, e_lo, e_hi
print("== 4. work per solution at 6x6, capped runs as censored exposure")
m = report("this Mac (read)", ev, k)
# x86 as REPORTED by the project: 14 finished, 3 solved (15,604 and 30,006 and one unknown), 11 capped near 35,000
for third in (5000, 35000):
    report(f"x86 (REPORTED, third solve at {third})", 15604 + 30006 + third + 11*35000, 3)
    report(f"pooled (third solve at {third})", ev + 15604 + 30006 + third + 11*35000, k + 3)
print("== 5. medians: fraction solved inside a budget")
print(f"  5x5: inside 35,000 evaluations {sum(1 for v in e5 if v <= 35000)}/50; inside 25,000 {sum(1 for v in e5 if v <= 25000)}/50")
m35 = sum(1 for r in s6 if r["solved"] and r["evals_solution"] <= 35000)
for lab, kk, nn in (("Mac, inside 35,000", m35, len(s6)), ("Mac, inside its cap (about 188,000)", k, len(s6)), ("x86 REPORTED, inside about 35,000", 3, 14), ("pooled, inside about 35,000", m35 + 3, len(s6) + 14)):
    lo, hi = cp(kk, nn)
    print(f"  6x6 {lab}: {kk}/{nn} = {kk/nn:.0%}, 95% [{lo:.1%}, {hi:.1%}]" + ("  => median above this budget with 95% confidence" if hi < 0.5 else "  => the interval contains one half: the median is not bounded with confidence"))
for bud in (35000, 188000):
    print(f"  if the 6x6 median exceeds {bud}: factor over the 5x5 median 3,017 above {bud/3017:.1f}, two-point exponent above {math.log(bud/3017)/ratio:.2f}")
print("== 6. the memo's expectation")
for g in (1, 2):
    e = 3017 * (1290/478) ** g
    print(f"  growth P^{g}: {e:.0f} evaluations, {e*8.32e-3:.0f} s at the model cost")
print(f"  observed on this Mac: CPU s per solution {cpu/k:.0f} (95% [{m[1]*ms6/1e3:.0f}, {m[2]*ms6/1e3:.0f}]); against 70 to 190 s: factor {cpu/k/190:.0f} to {cpu/k/70:.0f}")
print(f"  of which cost per evaluation {ms6/8.32:.2f}, the rest is the evaluation count")
print("== 7. segments (a restart every 1,000 outer steps, about 25,500 evaluations)")
seg_s = k; seg_f = sum(r["restarts"] for r in s6)
lo, hi = cp(seg_s, seg_s + seg_f)
print(f"  Mac 6x6: {seg_s} successful segments, {seg_f} abandoned at the restart: success per segment {seg_s/(seg_s+seg_f):.1%}, 95% [{lo:.1%}, {hi:.1%}] (the partial last segment of each capped run is left out)")
s9 = [r for r in load("lowrank-rust/results/s9_nn0_r16.jsonl") if "broken" not in r["board"]]
k9 = sum(1 for r in s9 if r["solved"] and r["outer"] <= 1000)
lo, hi = cp(k9, len(s9)); print(f"  5x5 (S9, no restart): solved inside 1,000 outer steps {k9}/{len(s9)}, 95% [{lo:.1%}, {hi:.1%}]")
for r in s6:
    if r["solved"]: print(f"  6x6 solved run: outer step of the solution {r['outer_solution']}, restarts {r['restarts']} -> outer steps into its segment {r['outer_solution'] - 1000*r['restarts']}")
