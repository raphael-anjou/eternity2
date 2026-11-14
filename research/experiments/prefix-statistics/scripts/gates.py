#!/usr/bin/env python3
"""Gate readings (PREREG section 9). Stdlib only.

    python3 -I scripts/gates.py results/gate
"""
import glob
import json
import math
import os
import sys

NAMES = ["s1_next_fits", "s2_frontier_fits", "s3_hall_surplus", "s4_balance_chi2", "s5_goodness_used",
         "s6_first_moment", "s7_ac_domains", "s8_block4_fills", "s9_colour_entropy", "s10_row_lookahead", "h0_hash"]
PEAK = [144, 160, 176]
# A2: G-exact cells need a median live population of at least 50 particles
# (set POP_FLOOR = 0 to read the gate as first preregistered).
POP_FLOOR = int(os.environ.get("POP_FLOOR", "50"))


def d_(t, m, s):
    if s > 0:
        return max(-10, min(10, (t - m) / s))
    return 0.0 if t == m else math.copysign(10, t - m)


def gate_exact(d):
    rows = [json.loads(l) for f in sorted(glob.glob(os.path.join(d, "exact-*.jsonl"))) for l in open(f)]
    rows = [r for r in rows if "skipped" not in r]
    if not rows:
        print("G-exact: no rows")
        return
    cells = {}
    for r in rows:
        cells.setdefault((r["board"], r["depth"]), []).append(r)
    nb, ns, ntot, wb = 0, 0, 0, []
    exact_delta = {}
    worst = []
    for (b, k), rs in sorted(cells.items()):
        live = [r for r in rs if not r.get("extinct")]
        r0 = rs[0]
        for s in range(len(NAMES)):
            esd = r0["exact_sd"][s]
            if esd is None:
                continue
            exact_delta.setdefault((k, s), []).append(d_(r0["true"][s], r0["exact_mean"][s], esd))
            pops = sorted(r["pop"] for r in live)
            if esd <= 0 or len(live) < 15 or (POP_FLOOR and pops[len(pops) // 2] < POP_FLOOR):
                continue
            mu = sum(r["smc_mean"][s] for r in live) / len(live)
            sdm = sum(r["smc_sd"][s] for r in live) / len(live)
            bias = (mu - r0["exact_mean"][s]) / esd
            ratio = sdm / esd
            ntot += 1
            nb += abs(bias) <= 0.2
            ns += 0.8 <= ratio <= 1.25
            worst.append((abs(bias), b, k, NAMES[s], round(bias, 3), round(ratio, 3)))
            ew = r0["exact_work_sd"][s]
            if ew and ew > 0:
                wm = [r["smc_work_mean"][s] for r in live if r.get("work_ess", 0) >= 1]
                if wm:
                    wb.append((sum(wm) / len(wm) - r0["exact_work_mean"][s]) / ew)
    print(f"G-exact: {len(cells)} (board, depth) cells, {ntot} gated (board, depth, statistic) cells")
    print(f"  |bias| <= 0.2: {nb}/{ntot} = {nb / ntot:.3f}  (pass >= 0.90)")
    print(f"  sd ratio in [0.8, 1.25]: {ns}/{ntot} = {ns / ntot:.3f}  (pass >= 0.90)")
    print(f"  verdict: {'PASS' if nb / ntot >= 0.9 and ns / ntot >= 0.9 else 'FAIL'}")
    worst.sort(reverse=True)
    print("  largest |bias|:", worst[:8])
    if wb:
        wb.sort()
        print(f"  work-weighted bias (not gated): median {wb[len(wb) // 2]:+.3f}, 10% {wb[len(wb) // 10]:+.3f}, 90% {wb[9 * len(wb) // 10]:+.3f}, n {len(wb)}")
    print("  exact delta of the planted prefix (SMC-free), mean over boards [min, max]:")
    depths = sorted({k for k, _ in exact_delta})
    print("    " + "stat".ljust(20) + "".join(f"{k:>16}" for k in depths))
    for s, nm in enumerate(NAMES):
        line = "    " + nm.ljust(20)
        for k in depths:
            v = exact_delta.get((k, s), [])
            line += (f"{sum(v) / len(v):+6.2f}[{min(v):+.1f},{max(v):+.1f}]" if v else "").rjust(16)
        print(line)


def gate_rep(d):
    rows = {}
    for f in sorted(glob.glob(os.path.join(d, "rep-*.jsonl"))):
        tag = os.path.basename(f)[4:-6]
        b, np_, s = tag.split("-")
        for l in open(f):
            r = json.loads(l)
            if r.get("done") or r["extinct"]:
                continue
            rows.setdefault((int(b[1:]), np_, r["depth"]), []).append(r)
    if not rows:
        print("G-rep: no rows")
        return
    ok, tot, bad = 0, 0, []
    for (b, np_, k), rs in sorted(rows.items()):
        if np_ != "np1e6" or k not in PEAK:
            continue
        base = rows.get((b, "np1e5", k), [])
        if len(base) < 3:
            continue
        for s in range(len(NAMES) - 1):
            r6 = rs[0]
            if r6["sd"][s] <= 0 or any(r["sd"][s] <= 0 for r in base):
                continue
            m5 = sum(d_(r["true"][s], r["mean"][s], r["sd"][s]) for r in base) / len(base)
            d6 = d_(r6["true"][s], r6["mean"][s], r6["sd"][s])
            tot += 1
            if abs(d6 - m5) <= 0.5:
                ok += 1
            else:
                bad.append((b, k, NAMES[s], round(m5, 2), round(d6, 2)))
    print(f"G-rep: N = 1e6 delta within 0.5 of the N = 1e5 mean in {ok}/{tot} = {ok / max(tot, 1):.3f} cells (pass >= 0.90)"
          f" -> {'PASS' if tot and ok / tot >= 0.9 else 'FAIL'}")
    if bad:
        print("  outside:", bad)
    print("  per board, depth, N: delta of S4, S6, S9, S10 (descriptive)")
    for (b, np_, k), rs in sorted(rows.items()):
        ds = [[round(d_(r["true"][s], r["mean"][s], r["sd"][s]), 2) for s in (3, 5, 8, 9)] for r in rs]
        print(f"    {b} {k} {np_}: {ds}")


if __name__ == "__main__":
    gate_exact(sys.argv[1])
    print()
    gate_rep(sys.argv[1])
