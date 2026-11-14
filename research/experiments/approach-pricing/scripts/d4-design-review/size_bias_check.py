"""Design review D4: is the S6 tilt the size-bias identity?
If the true prefix is distributed as (uniform prefix) x exp(S6) and S6 is
roughly normal with sd sigma (nats) over the population, then
E[S6(true)] - mean = sigma^2, so delta = sigma. Reads calibration rows only.
Usage: python3 -I size_bias_check.py <results/calib dir>"""
import json, sys, glob, math, os
d = sys.argv[1]
rows = []
for f in sorted(glob.glob(os.path.join(d, "board-*.jsonl"))):
    for l in open(f):
        r = json.loads(l)
        if "depth" in r and not r.get("extinct"):
            rows.append(r)
S6, S9, S4 = 5, 8, 3
print("depth n  pop_med  log10W  | S6: sd(nats) shift  shift/sd^2  delta  | work: sd shift delta ess_med | S9 delta | corr(S6,S9 true z)")
for k in sorted({r["depth"] for r in rows}):
    R = [r for r in rows if r["depth"] == k and r["sd"][S6] > 0]
    if not R: continue
    n = len(R)
    sd = [r["sd"][S6] for r in R]
    sh = [r["true"][S6] - r["mean"][S6] for r in R]
    de = [s / q for s, q in zip(sh, sd)]
    msd2 = sum(x * x for x in sd) / n
    wr = [r for r in R if r.get("work_ess", 0) >= 10 and r["work_sd"][S6] > 0]
    wsd = [r["work_sd"][S6] for r in wr]; wsh = [r["true"][S6] - r["work_mean"][S6] for r in wr]
    ess = sorted(r["work_ess"] for r in R)
    pops = sorted(r["pop"] for r in R)
    z9 = [(r["true"][S9] - r["mean"][S9]) / r["sd"][S9] for r in R]
    mx, my = sum(de) / n, sum(z9) / n
    cov = sum((a - mx) * (b - my) for a, b in zip(de, z9))
    c = cov / math.sqrt(sum((a - mx) ** 2 for a in de) * sum((b - my) ** 2 for b in z9))
    w = sum(r["log10_width"] for r in R) / n
    print(f"{k:4d} {n:3d} {pops[n//2]:7d} {w:6.2f} | {sum(sd)/n:5.2f} {sum(sh)/n:+6.2f} {sum(sh)/n/msd2:5.2f} {sum(de)/n:+5.2f} | "
          + (f"{sum(wsd)/len(wr):5.2f} {sum(wsh)/len(wr):+6.2f} {sum(a/b for a,b in zip(wsh,wsd))/len(wr):+5.2f}" if wr else "   -     -     -  ")
          + f" {ess[n//2]:8.1f} | {my:+5.2f} | {c:+.2f}")
