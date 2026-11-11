#!/usr/bin/env python3
"""Hard-ladder analysis (Python 3 standard library only).

  analyze.py calibration                 rank calibration candidates (PREREG selection rule)
  analyze.py baseline                    baseline curves, growth fits, kill criterion
  analyze.py compare CAND.jsonl BASE.jsonl [--out F]
                                         PROTOCOL.md G1/G2 for a candidate vs a baseline

Paths are relative to the hard-ladder directory. Unsolved runs are
right-censored at the common cap and are never dropped: a censored value is
+infinity in order statistics, so the median is the Kaplan-Meier median
(the ceil(N/2)-th order statistic; "> cap" when fewer than half are solved).
"""
import json
import math
import os
import random
import sys

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
SIZES = [6, 7, 8, 9, 10]
B = 10000
BOOT_SEED = 20261111
INF = math.inf


def load(path):
    with open(os.path.join(ROOT, path) if not os.path.isabs(path) else path) as f:
        return [json.loads(l) for l in f if l.strip()]


def obs(r):
    return r["nodes"] if r["status"] == "solved" else INF


def km_quantile(vals, q):
    """Order statistic ceil(N*q) with censored = inf (exact KM quantile under a common cap)."""
    s = sorted(vals)
    k = max(1, math.ceil(len(s) * q))
    return s[k - 1]


def upper_median(vals):
    s = sorted(vals)
    return s[len(s) // 2]


def wilson(k, n, z=1.959964):
    if n == 0:
        return (0.0, 1.0)
    p = k / n
    d = 1 + z * z / n
    c = (p + z * z / (2 * n)) / d
    h = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / d
    return (max(0.0, c - h), min(1.0, c + h))


def ols(xs, ys):
    n = len(xs)
    if n < 2:
        return None
    mx, my = sum(xs) / n, sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs)
    sxy = sum((x - mx) * (y - my) for x, y in zip(xs, ys))
    slope = sxy / sxx
    return slope, my - slope * mx


def pct(vals, p):
    s = sorted(vals)
    i = (len(s) - 1) * p
    lo, hi = math.floor(i), math.ceil(i)
    if s[lo] == s[hi] or math.isinf(s[lo]) or math.isinf(s[hi]):
        return s[hi] if p > 0.5 else s[lo]
    return s[lo] + (s[hi] - s[lo]) * (i - lo)


def fmt(v):
    return "> cap" if math.isinf(v) else f"{v:,.0f}"


def by_size(rows):
    d = {}
    for r in rows:
        d.setdefault(r["n"], []).append(r)
    for v in d.values():
        v.sort(key=lambda r: r["seed"])
    return d


def size_stats(rows):
    vals = [obs(r) for r in rows]
    k = sum(1 for r in rows if r["status"] == "solved")
    errs = sum(1 for r in rows if r["status"] == "error")
    lo, hi = wilson(k, len(rows))
    walls = sorted(r["wall_ms"] for r in rows)
    return {
        "instances": len(rows),
        "solved": k,
        "errors": errs,
        "solve_fraction": k / len(rows),
        "solve_wilson95": [round(lo, 4), round(hi, 4)],
        "cap": rows[0]["cap"],
        "colours": rows[0]["c"],
        "median_nodes": km_quantile(vals, 0.5),
        "q1_nodes": km_quantile(vals, 0.25),
        "q3_nodes": km_quantile(vals, 0.75),
        "median_wall_ms": walls[len(walls) // 2],
        "total_wall_s": sum(walls) / 1000,
        "inconsistent_tries": sum(r["inconsistent"] for r in rows),
        "equals_planted": sum(1 for r in rows if r["equals_planted"]),
        "nodes_sorted": sorted(vals),
    }


def growth(meds):
    """OLS of log10(median) on cells and on n over sizes with finite medians."""
    pts = [(n, m) for n, m in meds if not math.isinf(m)]
    out = {"sizes_used": [n for n, _ in pts]}
    for name, f in (("per_cell", lambda n: n * n), ("per_side", lambda n: n)):
        fit = ols([f(n) for n, _ in pts], [math.log10(m) for _, m in pts])
        out[name] = None if fit is None else {"slope_log10": fit[0], "intercept_log10": fit[1], "factor_per_unit": 10 ** fit[0]}
    return out


def strictly_increasing(ms):
    return all(a < b for a, b in zip(ms, ms[1:]))


def kill_eval(stats_by_n, per_size_vals):
    sizes = sorted(stats_by_n)
    ms = [stats_by_n[n]["median_nodes"] for n in sizes]
    mono = strictly_increasing(ms)
    s8 = stats_by_n[8]["solved"]
    rate_ok = s8 >= 15
    rng = random.Random(BOOT_SEED)
    mono_hits, slopes = 0, []
    for _ in range(B):
        bm = []
        for n in sizes:
            v = per_size_vals[n]
            bm.append(km_quantile([v[rng.randrange(len(v))] for _ in v], 0.5))
        mono_hits += strictly_increasing(bm)
        pts = [(n * n, math.log10(m)) for n, m in zip(sizes, bm) if not math.isinf(m)]
        fit = ols([p[0] for p in pts], [p[1] for p in pts])
        if fit is not None and len(pts) == len(sizes):
            slopes.append(fit[0])
    g = growth(list(zip(sizes, ms)))
    return {
        "sizes_evaluated": sizes,
        "complete": sizes == SIZES,
        "medians": [None if math.isinf(m) else m for m in ms],
        "monotone_strict": mono,
        "solved_8x8": s8,
        "solve_rate_8x8_ok": rate_ok,
        "killed": (not rate_ok) or (not mono),
        "monotone_note": None if sizes == SIZES else "strict increase checked on measured sizes only; rest deferred (PREREG_AMENDMENTS.md A2)",
        "fired": [c for c, ok in (("monotonicity", mono), ("solve_rate_8x8", rate_ok)) if not ok],
        "support": {
            "bootstrap_B": B,
            "bootstrap_seed": BOOT_SEED,
            "fraction_resamples_monotone": mono_hits / B,
            "slope_per_cell_log10": g["per_cell"]["slope_log10"] if g["per_cell"] else None,
            "slope_per_cell_ci95": [pct(slopes, 0.025), pct(slopes, 0.975)] if len(slopes) > 100 else None,
            "resamples_with_all_medians_finite": len(slopes),
        },
    }


def cmd_calibration():
    d = os.path.join(ROOT, "results/calibration/primary")
    table = []
    for fn in sorted(os.listdir(d)):
        if not fn.endswith(".jsonl"):
            continue
        rows = load(os.path.join(d, fn))
        bs = by_size(rows)
        eligible = all(r["status"] != "error" and r["inconsistent"] == 0 for r in rows)
        meds = {n: km_quantile([obs(r) for r in bs[n]], 0.5) for n in SIZES}
        table.append({
            "solver": rows[0]["solver"],
            "eligible": eligible,
            "errors": sum(r["status"] == "error" for r in rows),
            "inconsistent": sum(r["inconsistent"] for r in rows),
            "solved_total": sum(r["status"] == "solved" for r in rows),
            "solved_by_size": {n: sum(r["status"] == "solved" for r in bs[n]) for n in SIZES},
            "median_by_size": {n: (None if math.isinf(m) else m) for n, m in meds.items()},
            "wall_s_total": sum(r["wall_ms"] for r in rows) / 1000,
            "_key": (-sum(r["status"] == "solved" for r in rows),) + tuple(meds[n] for n in reversed(SIZES)),
        })
    ranked = sorted([t for t in table if t["eligible"]], key=lambda t: t["_key"]) + [t for t in table if not t["eligible"]]
    for i, t in enumerate(ranked):
        t["rank"] = i + 1 if t["eligible"] else None
        del t["_key"]
    best = ranked[0]["solver"]
    out = {"cap": rows[0]["cap"], "selected": best, "ranking": ranked}
    with open(os.path.join(ROOT, "results/calibration/ranking.json"), "w") as f:
        json.dump(out, f, indent=1)
    print("| rank | solver | eligible | solved/50 | " + " | ".join(f"solved n={n}" for n in SIZES) + " | " + " | ".join(f"median n={n}" for n in SIZES) + " | wall s |")
    print("|" + "---|" * (5 + 2 * len(SIZES)))
    for t in ranked:
        meds = [("> cap" if t["median_by_size"][n] is None else f"{t['median_by_size'][n]:,}") for n in SIZES]
        print(f"| {t['rank'] or '-'} | {t['solver']} | {'yes' if t['eligible'] else 'no'} | {t['solved_total']} | "
              + " | ".join(str(t["solved_by_size"][n]) for n in SIZES) + " | " + " | ".join(meds) + f" | {t['wall_s_total']:.0f} |")
    print(f"\nselected: {best}")


def summarize(path):
    rows = load(path)
    bs = by_size(rows)
    st = {n: size_stats(bs[n]) for n in sorted(bs)}
    vals = {n: [obs(r) for r in bs[n]] for n in sorted(bs)}
    return rows, st, vals


def cmd_baseline():
    best = json.load(open(os.path.join(ROOT, "results/calibration/ranking.json")))["selected"]
    summary = {"selected_baseline": best, "ladders": {}}
    surv = ["ladder,solver,n,c,nodes,fraction_unsolved"]
    for ladder in ("primary", "secondary"):
        summary["ladders"][ladder] = {}
        for solver in (best, "hp-dfs"):
            p = f"results/test/{ladder}/{solver}.jsonl"
            if not os.path.exists(os.path.join(ROOT, p)):
                continue
            rows, st, vals = summarize(p)
            ent = {"file": p, "per_size": {}, "growth": growth([(n, st[n]["median_nodes"]) for n in st])}
            for n in st:
                s = dict(st[n])
                fin = [v for v in s.pop("nodes_sorted") if not math.isinf(v)]
                for i, v in enumerate(fin):
                    surv.append(f"{ladder},{solver},{n},{s['colours']},{v},{(len(vals[n]) - i - 1) / len(vals[n]):.4f}")
                for k in ("median_nodes", "q1_nodes", "q3_nodes"):
                    s[k] = None if math.isinf(s[k]) else s[k]
                ent["per_size"][n] = s
            ent["kill"] = kill_eval(st, vals)
            summary["ladders"][ladder][solver] = ent
            print(f"\n### {ladder} ladder, {solver}\n")
            print("| n | c | solved/30 | Wilson 95% | median nodes | Q1 | Q3 | median wall ms |")
            print("|---|---|---|---|---|---|---|---|")
            for n in st:
                s = st[n]
                print(f"| {n} | {s['colours']} | {s['solved']} | {s['solve_wilson95'][0]:.2f}-{s['solve_wilson95'][1]:.2f} | "
                      f"{fmt(s['median_nodes'])} | {fmt(s['q1_nodes'])} | {fmt(s['q3_nodes'])} | {s['median_wall_ms']:.0f} |")
            k = ent["kill"]
            g = ent["growth"]["per_cell"]
            print(f"\nmonotone={k['monotone_strict']} solved8={k['solved_8x8']} killed={k['killed']} fired={k['fired']} "
                  f"boot_monotone={k['support']['fraction_resamples_monotone']:.3f} "
                  f"slope/cell={'n/a' if g is None else f'{g['slope_log10']:.4f} (x{g['factor_per_unit']:.3f}/cell)'} "
                  f"CI={k['support']['slope_per_cell_ci95']}")
    with open(os.path.join(ROOT, "results/summary.json"), "w") as f:
        json.dump(summary, f, indent=1)
    with open(os.path.join(ROOT, "results/survival.csv"), "w") as f:
        f.write("\n".join(surv) + "\n")


def log_ratio(c, b):
    cs, bs = c["status"] == "solved", b["status"] == "solved"
    if cs and bs:
        return math.log(c["nodes"] / b["nodes"])
    if cs and not bs:
        return math.log(c["nodes"] / b["cap"])  # upper bound on the true ratio
    if not cs and bs:
        return INF  # candidate loses
    return 0.0  # both censored: tie


def cmd_compare(cand_path, base_path, out=None):
    cand, base = load(cand_path), load(base_path)
    bmap = {r["instance"]: r for r in base}
    for r in cand:
        b = bmap.get(r["instance"])
        if b is None or b["cap"] != r["cap"] or b["ladder"] != r["ladder"]:
            sys.exit(f"instance {r['instance']} missing from baseline or different cap/ladder")
        if r["status"] == "error":
            sys.exit(f"candidate error on {r['instance']}: {r['error']}")
    cb, bb = by_size(cand), by_size(base)
    rng = random.Random(BOOT_SEED)
    res = {"candidate": cand[0]["solver"], "baseline": base[0]["solver"], "ladder": cand[0]["ladder"], "rungs": {}}
    for n in sorted(cb):
        pairs = [(c, bmap[c["instance"]]) for c in cb[n]]
        lr = [log_ratio(c, b) for c, b in pairs]
        med = upper_median(lr)
        boots = [upper_median([lr[rng.randrange(len(lr))] for _ in lr]) for _ in range(B)]
        hi = pct(boots, 0.975)
        lo = pct(boots, 0.025)
        g1 = math.exp(med) <= 0.5 and hi < 0
        res["rungs"][n] = {
            "pairs": len(lr),
            "median_ratio": math.exp(med) if not math.isinf(med) else None,
            "median_log_ratio": None if math.isinf(med) else med,
            "ci95_median_log_ratio": [None if math.isinf(lo) else lo, None if math.isinf(hi) else hi],
            "G1_pass": g1,
            "cand_solved": sum(c["status"] == "solved" for c, _ in pairs),
            "base_solved": sum(b["status"] == "solved" for _, b in pairs),
            "cand_median_wall_ms": sorted(c["wall_ms"] for c, _ in pairs)[len(pairs) // 2],
            "base_median_wall_ms": sorted(b["wall_ms"] for _, b in pairs)[len(pairs) // 2],
            "cand_total_wall_s": sum(c["wall_ms"] for c, _ in pairs) / 1000,
            "base_total_wall_s": sum(b["wall_ms"] for _, b in pairs) / 1000,
        }
    dropped = None
    for n in (6, 8, 10):
        if n in res["rungs"] and not res["rungs"][n]["G1_pass"]:
            dropped = n
            break
    res["dropped_at"] = dropped

    sizes = [n for n in SIZES if n in cb]

    def slope_diff(cvals, bvals):
        cm = [km_quantile(cvals[n], 0.5) for n in sizes]
        bm = [km_quantile(bvals[n], 0.5) for n in sizes]
        if any(math.isinf(m) for m in cm):
            return INF
        bm = [cap if math.isinf(m) else m for m, cap in zip(bm, [bb[n][0]["cap"] for n in sizes])]
        sc = ols([n * n for n in sizes], [math.log10(m) for m in cm])[0]
        sb = ols([n * n for n in sizes], [math.log10(m) for m in bm])[0]
        return sc - sb

    cv = {n: [obs(r) for r in cb[n]] for n in sizes}
    bv = {n: [obs(bmap[r["instance"]]) for r in cb[n]] for n in sizes}
    d0 = slope_diff(cv, bv)
    boots = []
    for _ in range(B):
        ci, bi = {}, {}
        for n in sizes:
            idx = [rng.randrange(len(cv[n])) for _ in cv[n]]
            ci[n] = [cv[n][i] for i in idx]
            bi[n] = [bv[n][i] for i in idx]
        boots.append(slope_diff(ci, bi))
    hi = pct(boots, 0.975)
    res["G2"] = {
        "sizes": sizes,
        "slope_diff_log10_per_cell": None if math.isinf(d0) else d0,
        "ci95": [None if math.isinf(pct(boots, 0.025)) else pct(boots, 0.025), None if math.isinf(hi) else hi],
        "pass": (not math.isinf(d0)) and d0 < 0 and hi < 0,
    }
    txt = json.dumps(res, indent=1)
    if out:
        with open(out, "w") as f:
            f.write(txt + "\n")
    print(txt)


if __name__ == "__main__":
    a = sys.argv[1:]
    if a[:1] == ["calibration"]:
        cmd_calibration()
    elif a[:1] == ["baseline"]:
        cmd_baseline()
    elif a[:1] == ["compare"] and len(a) >= 3:
        cmd_compare(a[1], a[2], a[a.index("--out") + 1] if "--out" in a else None)
    else:
        sys.exit(__doc__)
