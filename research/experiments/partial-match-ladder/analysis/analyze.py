#!/usr/bin/env python3
"""Partial-match ladder analysis. Stdlib only; reads raw jsonl, writes JSON + markdown.

    python3 -I analysis/analyze.py probe      # rho from results/probe/*/*.jsonl
    python3 -I analysis/analyze.py calibrate  # results/calibration/*/*.jsonl -> results/calibration.json, tables
    python3 -I analysis/analyze.py test       # results/test/*/*.jsonl -> results/summary_corrected_after_judge.json, tables
    python3 -I analysis/analyze.py determinism  # results/determinism/*/*.jsonl

Every rule is the one written in PREREG.md (and PREREG_AMENDMENTS.md).
"""
import glob
import json
import math
import os
import random
import statistics
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ENTRANTS = ["break-r1", "break-r2", "sa", "pt", "alns"]
BAND = (0.02, 0.04)
KS = [1, 2, 3, 4, 5, 6]
# Family-level established pairs (PREREG.md, criterion (3)): higher member first.
FAMILY_PAIRS = [("break-r1", "sa"), ("break-r2", "sa"), ("alns", "sa")]
IMPLEMENTATION_PAIRS = []  # none: no entrant has a committed 16x16 result from its own implementation
FLIPS = 100_000
FLIP_SEED = 20261112


def _counts(E, k):
    b, x = divmod(E, k)
    return [b + (1 if i < x else 0) for i in range(k)]


def _ln_dfact_even(e):
    return math.lgamma(2 * e + 1) - e * math.log(2) - math.lgamma(e + 1)


def ln_expected_solutions(n, c, cf=5):
    """Balanced first moment of the generator's ensemble (exact colour counts per band; ring seams
    pair a clockwise side with an anticlockwise side), all four turns counted. Formula copied from
    the refereed review of the night memo (growth-law.REVIEW.md, growth-law-review/rvlib.py,
    lnS_balanced) on branch night/2026-11-12-lead. Recorded per rung, never gating."""
    w = n - 2
    P = w * w
    arr = math.log(24) + math.lgamma(4 * w + 1) + math.lgamma(P + 1) + P * math.log(4)
    EF = 4 * (n - 1)
    frame = sum(math.lgamma(f + 1) for f in _counts(EF, cf)) - math.lgamma(EF + 1)
    EI = 2 * w * (w + 1)
    if abs(c - round(c)) < 1e-12:
        inner = sum(_ln_dfact_even(e) for e in _counts(EI, int(round(c)))) - _ln_dfact_even(EI)
    else:
        inner = c * _ln_dfact_even(EI / c) - _ln_dfact_even(EI)
    return arr + frame + inner


def critical_colours(n, cf=5):
    lo, hi = 1.2, 60.0
    for _ in range(100):
        mid = (lo + hi) / 2
        if ln_expected_solutions(n, mid, cf) > 0:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2


def regime(n, c):
    ls = ln_expected_solutions(n, c)
    return {"ln_expected_solutions": ls, "expected_solutions": math.exp(ls),
            "x": c * c / (4 * (n - 2) ** 2), "c_crit_whole_board": critical_colours(n),
            "c_minus_c_crit": c - critical_colours(n)}


def load(phase):
    rows = []
    for f in sorted(glob.glob(os.path.join(ROOT, "results", phase, "*", "*.jsonl"))):
        with open(f) as fh:
            for line in fh:
                if line.strip():
                    r = json.loads(line)
                    r["_file"] = os.path.relpath(f, ROOT)
                    rows.append(r)
    return rows


def dedupe(rows):
    """One row per (entrant, instance, budget, run_seed); a duplicate across machines must agree."""
    out, seen = {}, {}
    conflicts = []
    for r in rows:
        k = (r["entrant"], r["instance"], r["budget"], r["run_seed"])
        if k in out:
            a = out[k]
            if (a.get("best_score"), a.get("work_used")) != (r.get("best_score"), r.get("work_used")):
                conflicts.append({"key": list(k), "a": [a["machine"], a.get("best_score"), a.get("work_used")],
                                  "b": [r["machine"], r.get("best_score"), r.get("work_used")]})
            continue
        out[k] = r
    return list(out.values()), conflicts


def gap(r):
    if r["status"] != "ok" or r["best_score"] is None:
        return 1.0
    return (r["optimum"] - r["best_score"]) / r["optimum"]


def int_score(r):
    """Verified score in integer units; 0 when there is no complete board (gap 1.0)."""
    return 0 if r["status"] != "ok" or r["best_score"] is None else r["best_score"]


def share(r):
    return 1.0 - gap(r)


def quart(xs):
    xs = sorted(xs)
    if len(xs) < 2:
        return [xs[0], xs[0], xs[0]] if xs else [None] * 3
    q = statistics.quantiles(xs, n=4, method="inclusive")
    return [q[0], statistics.median(xs), q[2]]


def work_to(r, k):
    target = r["optimum"] - k
    for w, s in r.get("trajectory") or []:
        if s >= target:
            return w
    return math.inf


def km_median(vals):
    v = sorted(vals)
    return v[math.ceil(len(v) / 2) - 1]


# ------------------------------------------------------------------ probe

def cmd_probe():
    rows = load("probe")
    rates = {}
    for e in ("break-r1", "pt"):
        rs = [r for r in rows if r["entrant"] == e and r["machine"] == "x86"]
        full = [r for r in rs if r.get("work_used") == r["budget"]]
        use = full if (e != "break-r1" or full) else rs
        rates[e] = {
            "runs": len(rs), "runs_full_budget": len(full),
            "median_rate_per_s": statistics.median(r["work_used"] / (r["wall_ms"] / 1000) for r in use),
        }
    raw = rates["break-r1"]["median_rate_per_s"] / rates["pt"]["median_rate_per_s"]
    mag = 10 ** math.floor(math.log10(raw))
    rho = round(raw / mag) * mag
    out = {"rates": rates, "rho_raw": raw, "rho": rho}
    json.dump(out, open(os.path.join(ROOT, "results", "probe.json"), "w"), indent=1)
    print(json.dumps(out, indent=1))


# ------------------------------------------------------------------ calibration

def cmd_calibrate():
    rows, conflicts = dedupe(load("calibration"))
    cells = {}
    for r in rows:
        cells.setdefault((r["n"], r["c"]), {}).setdefault(r["entrant"], []).append(r)
    table = []
    for (n, c), by in sorted(cells.items()):
        ent = {}
        for e in ENTRANTS:
            rs = by.get(e, [])
            if not rs:
                continue
            gaps = [gap(r) for r in rs]
            ent[e] = {
                "runs": len(rs), "median_gap": statistics.median(gaps),
                "median_share": statistics.median(share(r) for r in rs),
                "below_opt": sum(1 for r in rs if r["status"] == "ok" and r["best_score"] < r["optimum"]),
                "no_complete": sum(1 for r in rs if r["status"] == "no_complete_board"),
                "at_opt": sum(1 for r in rs if r["status"] == "ok" and r["best_score"] == r["optimum"]),
                "errors": sum(1 for r in rs if r["status"] == "error"),
                "scores": [r["best_score"] for r in sorted(rs, key=lambda x: x["seed"])],
                "median_wall_s": statistics.median(r["wall_ms"] for r in rs) / 1000,
            }
        complete = all(e in ent and ent[e]["runs"] == 4 for e in ENTRANTS)
        best = max(ent, key=lambda e: (ent[e]["median_share"], -ENTRANTS.index(e))) if ent else None
        bg = ent[best]["median_gap"] if best else None
        best_below = ent[best]["runs"] - ent[best]["at_opt"] if best else 0
        in_band = [e for e in ent if BAND[0] <= ent[e]["median_gap"] <= BAND[1]]
        table.append({
            "n": n, "c": c, "complete": complete, "entrants": ent, "best": best,
            "best_median_gap": bg, "best_below_opt": best_below,
            "qualifies": complete and bg is not None and BAND[0] <= bg <= BAND[1] and best_below >= 3,
            "in_band": in_band, "regime": regime(n, c),
        })

    def rank_key(t):
        return (-len(t["in_band"]), abs(t["best_median_gap"] - 0.03), -t["n"])

    q = sorted([t for t in table if t["qualifies"]], key=rank_key)
    choice = {}
    if q:
        choice["A"] = (q[0]["n"], q[0]["c"], "in band")
        other = [t for t in q if t["n"] != q[0]["n"]]
        if other:
            choice["B"] = (other[0]["n"], other[0]["c"], "in band")
    else:
        fb = sorted([t for t in table if t["complete"] and t["best_below_opt"] >= 3],
                    key=lambda t: (abs(t["best_median_gap"] - 0.03), -t["n"]))
        if fb:
            choice["A"] = (fb[0]["n"], fb[0]["c"], "outside band")
    out = {"cells": table, "choice": choice, "duplicate_conflicts": conflicts}
    json.dump(out, open(os.path.join(ROOT, "results", "calibration.json"), "w"), indent=1)
    lines = ["| n | c | " + " | ".join(f"{e} median gap (scores)" for e in ENTRANTS) + " | best | best below opt | in band | qualifies | E[solutions] | x | c - c_crit |",
             "|---|---|" + "---|" * len(ENTRANTS) + "---|---|---|---|---|---|---|"]
    for t in table:
        cellv = []
        for e in ENTRANTS:
            x = t["entrants"].get(e)
            cellv.append("-" if not x else f"{100 * x['median_gap']:.2f}% ({', '.join('none' if s is None else str(s) for s in x['scores'])})")
        lines.append(f"| {t['n']} | {t['c']} | " + " | ".join(cellv) +
                     f" | {t['best']} | {t['best_below_opt']}/4 | {len(t['in_band'])} | {'yes' if t['qualifies'] else 'no'}"
                     f" | {t['regime']['expected_solutions']:.3g} | {t['regime']['x']:.3f} | {t['regime']['c_minus_c_crit']:+.2f} |")
    lines.append("")
    lines.append(f"Choice: {choice}")
    md = "\n".join(lines)
    open(os.path.join(ROOT, "results", "calibration_table.md"), "w").write(md + "\n")
    print(md)


# ------------------------------------------------------------------ test

def ranks_abs(d):
    order = sorted(range(len(d)), key=lambda i: abs(d[i]))
    rk = [0.0] * len(d)
    i = 0
    while i < len(order):
        j = i
        while j + 1 < len(order) and abs(d[order[j + 1]]) == abs(d[order[i]]):
            j += 1
        avg = (i + j) / 2 + 1
        for t in range(i, j + 1):
            rk[order[t]] = avg
        i = j + 1
    return rk


def wilcoxon_flip(d):
    d = [x for x in d if x != 0]
    if not d:
        return 0.0, 1.0
    rk = ranks_abs(d)
    obs = sum(r if x > 0 else -r for x, r in zip(d, rk))
    rng = random.Random(FLIP_SEED)
    hits = 0
    for _ in range(FLIPS):
        s = 0.0
        for r in rk:
            s += r if rng.random() < 0.5 else -r
        if abs(s) >= abs(obs) - 1e-9:
            hits += 1
    return obs, hits / FLIPS


def holm(ps, alpha=0.05):
    idx = sorted(range(len(ps)), key=lambda i: ps[i])
    rej = [False] * len(ps)
    m = len(ps)
    for j, i in enumerate(idx):
        if ps[i] <= alpha / (m - j):
            rej[i] = True
        else:
            break
    return rej


def analyse_rung(rows):
    by = {}
    for r in rows:
        by.setdefault(r["entrant"], {})[r["instance"]] = r
    insts = sorted(set().union(*[set(v) for v in by.values()]))
    res = {"instances": len(insts), "entrants": {}}
    for e in ENTRANTS:
        rs = [by.get(e, {}).get(i) for i in insts]
        if any(r is None for r in rs):
            res["entrants"][e] = {"missing": sum(r is None for r in rs)}
            continue
        sh = [share(r) for r in rs]
        g = [gap(r) for r in rs]
        wk = {}
        for k in KS:
            v = [work_to(r, k) for r in rs]
            m = km_median(v)
            wk[k] = {"km_median": None if m == math.inf else m, "reached": sum(1 for x in v if x != math.inf)}
        res["entrants"][e] = {
            "unit": rs[0]["unit"], "budget": rs[0]["budget"],
            "share_q1_med_q3": quart(sh), "share_mean": statistics.mean(sh),
            "median_gap": statistics.median(g),
            "at_opt": sum(1 for r in rs if r["status"] == "ok" and r["best_score"] == r["optimum"]),
            "no_complete": sum(1 for r in rs if r["status"] == "no_complete_board"),
            "errors": sum(1 for r in rs if r["status"] == "error"),
            "work_to_opt_minus_k": wk,
            "median_wall_s": statistics.median(r["wall_ms"] for r in rs) / 1000,
            "machines": sorted(set(r["machine"] for r in rs)),
            "in_band": BAND[0] <= statistics.median(g) <= BAND[1],
        }
    complete = all("missing" not in res["entrants"][e] for e in ENTRANTS)
    # criterion (1)
    best_below = 0
    for i in insts:
        best = max((by[e][i]["best_score"] or 0) if by[e][i]["status"] == "ok" else 0 for e in ENTRANTS if e in by and i in by[e])
        opt = next(by[e][i]["optimum"] for e in ENTRANTS if e in by and i in by[e])
        best_below += best < opt
    n_band = sum(1 for e in ENTRANTS if res["entrants"][e].get("in_band"))
    res["criterion1"] = {"entrants_in_band": n_band, "best_below_opt": best_below,
                         "pass": complete and n_band >= 3 and best_below >= 25}
    # pairwise tests
    pairs = {}
    for a_i, a in enumerate(ENTRANTS):
        for b in ENTRANTS[a_i + 1:]:
            if not complete:
                continue
            # Corrected after the judgment (finding 1): rank integer score differences, so equal
            # differences tie exactly. The optimum is the same for both entrants on an instance.
            di = [int_score(by[a][i]) - int_score(by[b][i]) for i in insts]
            d = [share(by[a][i]) - share(by[b][i]) for i in insts]
            w, p = wilcoxon_flip(di)
            pairs[f"{a} vs {b}"] = {"W": w, "p": p, "median_diff": statistics.median(d),
                                    "a_wins": sum(x > 0 for x in d), "b_wins": sum(x < 0 for x in d)}
    res["pairs"] = pairs

    def crit(est, label):
        if not est or not complete:
            return {"level": label, "verdict": "undetermined", "pairs": []}
        ps = [pairs[f"{a} vs {b}"]["p"] if f"{a} vs {b}" in pairs else pairs[f"{b} vs {a}"]["p"] for a, b in est]
        rej = holm(ps)
        det = []
        for (a, b), p, rj in zip(est, ps, rej):
            key = f"{a} vs {b}" if f"{a} vs {b}" in pairs else f"{b} vs {a}"
            md = pairs[key]["median_diff"] * (1 if key.startswith(a) else -1)
            det.append({"pair": [a, b], "p": p, "holm_significant": rj, "median_diff_expected_positive": md,
                        "agrees": md > 0})
        linked = len({x for ab in est for x in ab})
        c2 = all(x["holm_significant"] for x in det)
        c3 = (linked >= 3 and all(x["agrees"] for x in det))
        return {"level": label, "pairs": det, "linked_entrants": linked,
                "criterion2_pass": c2, "criterion3_pass": c3 if linked >= 3 else "undetermined"}

    res["implementation_level"] = crit(IMPLEMENTATION_PAIRS, "implementation")
    res["family_level"] = crit(FAMILY_PAIRS, "family")
    return res


def cmd_test():
    rows, conflicts = dedupe(load("test"))
    rungs = {}
    for r in rows:
        rungs.setdefault(f"n{r['n']}-c{r['c']}", []).append(r)
    out = {"rungs": {k: analyse_rung(v) for k, v in sorted(rungs.items())}, "duplicate_conflicts": conflicts}
    for k, v in rungs.items():
        out["rungs"][k]["regime"] = regime(v[0]["n"], v[0]["c"])
    json.dump(out, open(os.path.join(ROOT, "results", "summary_corrected_after_judge.json"), "w"), indent=1, default=str)
    md = []
    for k, r in out["rungs"].items():
        rg = r["regime"]
        md.append(f"### Rung {k} ({r['instances']} test instances)\n")
        md.append(f"First-moment expected solutions {rg['expected_solutions']:.3g} (ln {rg['ln_expected_solutions']:.1f}), x = c^2/(4(n-2)^2) = {rg['x']:.3f}, whole-board critical c = {rg['c_crit_whole_board']:.2f} (c - c_crit = {rg['c_minus_c_crit']:+.2f}).\n")
        md.append("| entrant | unit | budget | share Q1 / median / Q3 | mean share | median gap | at optimum | no complete board | errors | median wall s | machines |")
        md.append("|---|---|---|---|---|---|---|---|---|---|---|")
        for e in ENTRANTS:
            x = r["entrants"][e]
            if "missing" in x:
                md.append(f"| {e} | missing {x['missing']} rows |")
                continue
            q = x["share_q1_med_q3"]
            md.append(f"| {e} | {x['unit']} | {x['budget']:.3g} | {100*q[0]:.2f}% / {100*q[1]:.2f}% / {100*q[2]:.2f}% | {100*x['share_mean']:.2f}% | {100*x['median_gap']:.2f}% | {x['at_opt']} | {x['no_complete']} | {x['errors']} | {x['median_wall_s']:.1f} | {','.join(x['machines'])} |")
        md.append("")
        md.append("Work to optimum minus k (Kaplan-Meier median, censored at the budget; reached / 30):\n")
        md.append("| entrant | " + " | ".join(f"k={k}" for k in KS) + " |")
        md.append("|---|" + "---|" * len(KS))
        for e in ENTRANTS:
            x = r["entrants"][e]
            if "missing" in x:
                continue
            cells = []
            for k in KS:
                w = x["work_to_opt_minus_k"][k]
                cells.append(("> budget" if w["km_median"] is None else f"{w['km_median']:.3g}") + f" ({w['reached']})")
            md.append(f"| {e} | " + " | ".join(cells) + " |")
        md.append("")
        c1 = r["criterion1"]
        md.append(f"Criterion (1): entrants in band {c1['entrants_in_band']}, best below optimum on {c1['best_below_opt']}/30, pass = {c1['pass']}\n")
        md.append("| pair | median diff of share | a wins / b wins | Wilcoxon W | p (sign flips) |")
        md.append("|---|---|---|---|---|")
        for p, v in r["pairs"].items():
            md.append(f"| {p} | {100*v['median_diff']:.2f} pt | {v['a_wins']} / {v['b_wins']} | {v['W']:.1f} | {v['p']:.5f} |")
        md.append("")
        for lvl in ("implementation_level", "family_level"):
            v = r[lvl]
            md.append(f"{lvl}: {json.dumps(v)}\n")
    text = "\n".join(md)
    open(os.path.join(ROOT, "results", "test_tables_corrected_after_judge.md"), "w").write(text + "\n")
    print(text)


def cmd_determinism():
    rows = load("determinism")
    by = {}
    for r in rows:
        by.setdefault((r["entrant"], r["instance"], r["budget"], r["run_seed"]), {})[r["machine"]] = r
    out = []
    for k, m in sorted(by.items()):
        if len(m) < 2:
            out.append({"key": list(k), "machines": sorted(m), "identical": None})
            continue
        a, b = m["x86"], m["m1"]
        same = all(a.get(f) == b.get(f) for f in ("best_score", "work_used", "trajectory", "board", "status"))
        out.append({"key": list(k), "identical": same,
                    "x86": [a.get("best_score"), a.get("work_used")], "m1": [b.get("best_score"), b.get("work_used")]})
    json.dump(out, open(os.path.join(ROOT, "results", "determinism.json"), "w"), indent=1)
    for o in out:
        print(o)


if __name__ == "__main__":
    {"probe": cmd_probe, "calibrate": cmd_calibrate, "test": cmd_test, "determinism": cmd_determinism}[sys.argv[1]]()
