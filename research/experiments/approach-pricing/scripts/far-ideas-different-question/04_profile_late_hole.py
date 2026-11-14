"""Arithmetic on the MEASURED level profile of the real tree (Track 10, commit dc71ad56,
results/m2/{five-np1e6,centre-np1e5,none-np1e5}.jsonl, copied here).
 (1) nodes per solution with fewer clues (does finding ANY of the ~14,702 starter-only
     solutions cost less than the five-clue search?);
 (2) late hole: a "solution with a hole of the last m scanned cells" is a valid partial
     board at free depth 251-m, so their number is the measured width W(d). Cost per
     hole-solution ~ T / W(d). Find the d where that reaches 10^15, extrapolating past the
     extinction depth with the uniform model of script 03, shifted to the measured width
     (bracket: constant shift, and a shift interpolated to the independently re-checked full-board
     anchor log10 E = -7.39 at d = 251).
Run: python3 -I 04_profile_late_hole.py   (stdlib, seconds)"""
import json, math, os
HERE = os.path.dirname(os.path.abspath(__file__))
L10 = math.log10


def mean_profile(fn):
    reps = [json.loads(l) for l in open(os.path.join(HERE, fn))]
    L = max(len(r["log10_width"]) for r in reps)
    out = []
    for d in range(L):
        ws = [r["log10_width"][d] for r in reps if d < len(r["log10_width"]) and r["log10_width"][d] is not None]
        if len(ws) < len(reps):
            break
        m = max(ws)
        out.append(m + L10(sum(10 ** (w - m) for w in ws) / len(ws)))
    tot = [r["total"] for r in reps]
    return out, L10(sum(tot) / len(tot))


five, T5 = mean_profile("five-np1e6.jsonl")
centre, Tc = mean_profile("centre-np1e5.jsonl")
none, Tn = mean_profile("none-np1e5.jsonl")
print("(1) nodes per solution, row scan (measured totals; independently re-checked first-moment solution counts)")
for name, T, E in (("five clues", T5, 1.0), ("centre clue only", Tc, 14702.27), ("no clue", Tn, 1.153e7)):
    print(f"  {name:17s} log10 T {T:.3f}  E[solutions] {E:,.0f}  log10 T/E {T - L10(E):.2f}")

# (2)
ns = {}
src = open(os.path.join(HERE, "03_first_moment_orders_holes.py")).read().split("row = [")[0]
exec(src, ns)
CL, N = ns["CLUES"], ns["N"]
row = [(r, c) for r in range(N) for c in range(N) if (r, c) not in CL]
model = [0.0] + ns["profile"](row)  # model[d] = log10 width at free depth d
dlast = len(five) - 1
shift0 = five[dlast] - model[dlast]
shift_end = -7.39 - model[251]
print(f"\n(2) late hole. measured profile valid to free depth {dlast} (width 10^{five[dlast]:.2f}); "
      f"model shift there {shift0:+.2f}, at d=251 {shift_end:+.2f}")
target = T5 - 15
print(f"  need W(d) >= 10^{target:.2f} for <= 1e15 nodes per hole-solution")
for label, f in (("constant shift", lambda d: shift0),
                 ("interpolated shift", lambda d: shift0 + (shift_end - shift0) * (d - dlast) / (251 - dlast))):
    for d in range(dlast, 252):
        w = model[d] + f(d)
        if w <= target:
            print(f"  {label:20s}: first d with W <= target is {d} (hole of the last {251-d} free cells), "
                  f"W = 10^{w:.2f}; chance that a hole-solution found is the planted board minus the hole: 10^-{w:.1f}")
            break
for d in (160, 176, 190):
    print(f"  measured W({d}) = 10^{five[d]:.2f}: hole of last {251-d} cells, cost per hole-solution 10^{T5-five[d]:.2f} nodes")
