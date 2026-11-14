#!/usr/bin/env python3
"""A20 analysis (PREREG.md). Stdlib only.

    python3 analysis/analyze.py calibrate   # tabu selection, results/calibration/
    python3 analysis/analyze.py test        # tables and verdicts, results/summary.json + results/tables.md
"""

import json
import math
import random
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
sys.path.insert(0, str(ROOT.parent / "hard-ladder" / "analysis"))
from analyze import km_quantile, upper_median, wilson, pct  # noqa: E402

B = 10000
SEED = 20261111
INF = math.inf
ARMS = ["scb-gacolor_ac3_random", "bb-mrv", "bj-blocker", "bj-random"]
SHORT = {"scb-gacolor_ac3_random": "scb", "bb-mrv": "bb-mrv", "bj-blocker": "blocker", "bj-random": "random-twin"}


def load(p):
    p = Path(p)
    if not p.exists():
        return None
    return [json.loads(l) for l in p.read_text().splitlines() if l.strip()]


def solved(r):
    return r["status"] == "solved"


def time_censored(r):
    return r["status"] == "censored" and r["nodes_counted"] < r["cap"]


def lr(c, b):
    """PROTOCOL.md G1 pair score; a time-censored baseline vs solved candidate scores 0 (PREREG)."""
    cs, bs = solved(c), solved(b)
    if cs and bs:
        return math.log(c["nodes"] / b["nodes"])
    if cs and not bs:
        return 0.0 if time_censored(b) else math.log(c["nodes"] / b["cap"])
    if not cs and bs:
        return INF
    return 0.0


def boot_ci(vals):
    rng = random.Random(SEED)
    n = len(vals)
    meds = [upper_median([vals[rng.randrange(n)] for _ in range(n)]) for _ in range(B)]
    return pct(meds, 0.025), pct(meds, 0.975)


def pair(cand, base):
    bm = {r["instance"]: r for r in base}
    ps = [(c, bm[c["instance"]]) for c in cand]
    assert all(c["cap"] == b["cap"] for c, b in ps), "cap mismatch"
    l = [lr(c, b) for c, b in ps]
    med = upper_median(l)
    lo, hi = boot_ci(l)
    w = [c["wall_ms"] / b["wall_ms"] for c, b in ps]
    wlo, whi = boot_ci(w)
    ratio = math.exp(med) if med != INF else INF
    return {
        "pairs": len(ps),
        "median_ln_ratio": med,
        "node_ratio": ratio,
        "ci_ln": [lo, hi],
        "ci_ratio": [math.exp(lo) if lo != INF else INF, math.exp(hi) if hi != INF else INF],
        "g1_pass": ratio <= 0.5 and hi < 0,
        "significantly_lower": hi < 0,
        "cand_solved": sum(solved(c) for c, _ in ps),
        "base_solved": sum(solved(b) for _, b in ps),
        "both_censored": sum(1 for c, b in ps if not solved(c) and not solved(b)),
        "cand_only_solved": sum(1 for c, b in ps if solved(c) and not solved(b)),
        "base_only_solved": sum(1 for c, b in ps if not solved(c) and solved(b)),
        "wall_ratio_median": upper_median(w),
        "wall_ratio_ci": [wlo, whi],
    }


def describe(rows, side):
    n = len(rows)
    k = sum(solved(r) for r in rows)
    nodes = [r["nodes"] if solved(r) else INF for r in rows]
    walls = sorted(r["wall_ms"] for r in rows)
    lo, hi = wilson(k, n)
    d = {
        "N": n,
        "solved": k,
        "wilson": [lo, hi],
        "km_q25": km_quantile(nodes, 0.25),
        "km_median": km_quantile(nodes, 0.5),
        "km_q75": km_quantile(nodes, 0.75),
        "median_wall_ms": walls[n // 2],
        "total_wall_s": sum(walls) / 1000,
        "errors": sum(r["status"] == "error" for r in rows),
        "inconsistent": sum(r["inconsistent"] for r in rows),
        "time_censored": sum(time_censored(r) for r in rows),
    }
    if side:
        de = sum(s["dead_ends"] for s in side)
        d["dead_ends"] = de
        d["fallback_frac"] = sum(s["fallbacks"] for s in side) / max(1, de)
        d["mean_evicted_per_dead_end"] = sum(s["evicted"] for s in side) / max(1, de)
        d["revisit_frac"] = sum(s["repairs_revisiting_state"] for s in side) / max(1, de)
        d["max_state_multiplicity"] = max(s["max_state_multiplicity"] for s in side)
        d["tabu_clears"] = sum(s["tabu_clears"] for s in side)
        d["guard_fired"] = [s["puzzle"] for s in side if s["guard_fired"]]
        cens = {r["instance"] for r in rows if not solved(r)}
        d["cycled_runs"] = sorted(
            s["puzzle"]
            for s in side
            if s["dead_ends"] > 0 and s["repairs_revisiting_state"] / s["dead_ends"] > 0.5
            and s["puzzle"].replace("hard-ladder ", "").replace(" ", "-") in cens
        )
    return d


def fmt(x, nd=0):
    if x is None:
        return "-"
    if isinstance(x, float) and math.isinf(x):
        return "inf"
    return f"{x:,.{nd}f}"


def calibrate():
    cal = ROOT / "results" / "calibration"
    rows = []
    best = None
    for t in (0, 16, 128):
        for arm in ("bj-blocker", "bj-random"):
            rs = load(cal / f"t{t}" / f"{arm}.jsonl")
            if rs is None:
                continue
            for n in (6, 7):
                sub = [r for r in rs if r["n"] == n]
                k = sum(solved(r) for r in sub)
                sl = sum(math.log(min(r["nodes"], r["cap"])) for r in sub)
                rows.append((t, arm, n, k, len(sub), sl, km_quantile([r["nodes"] if solved(r) else INF for r in sub], 0.5)))
            if arm == "bj-blocker":
                k = sum(solved(r) for r in rs)
                sl = sum(math.log(min(r["nodes"], r["cap"])) for r in rs)
                key = (-k, sl, t)
                if best is None or key < best[0]:
                    best = (key, t)
    base = {}
    for arm in ("bb-mrv", "scb-gacolor_ac3_random", "bj-blocker"):
        rs = load(cal / "base" / f"{arm}.jsonl")
        if rs:
            for n in (6, 7):
                sub = [r for r in rs if r["n"] == n]
                base[(arm, n)] = (sum(solved(r) for r in sub), len(sub),
                                  km_quantile([r["nodes"] if solved(r) else INF for r in sub], 0.5),
                                  sum(r["wall_ms"] for r in sub) / 1000)
    lines = ["| tabu | arm | n | solved | KM median | sum ln(min(nodes, cap)) |", "|---|---|---|---|---|---|"]
    for t, arm, n, k, N, sl, med in rows:
        lines.append(f"| {t} | {arm} | {n} | {k}/{N} | {fmt(med)} | {sl:.2f} |")
    lines += ["", "| arm (tabu 16 for bj) | n | solved | KM median | total wall s |", "|---|---|---|---|---|"]
    for (arm, n), (k, N, med, w) in sorted(base.items()):
        lines.append(f"| {arm} | {n} | {k}/{N} | {fmt(med)} | {w:.1f} |")
    lines += ["", f"Selected tabu tenure (bj-blocker, most solved over R6+R7 calibration, then smaller sum ln, then smaller T): **{best[1]}**"]
    (cal / "calibration.md").write_text("\n".join(lines) + "\n")
    (cal / "selected_tabu.txt").write_text(f"{best[1]}\n")
    print("\n".join(lines))


def load_set(d):
    d = ROOT / "results" / "test" / d
    out = {}
    for a in ARMS:
        rs = load(d / f"{a}.jsonl")
        if rs is not None:
            out[a] = (rs, load(d / f"{a}.side.jsonl"))
    return out


def merge(*sets):
    out = {}
    for s in sets:
        for a, (rs, side) in s.items():
            r0, s0 = out.get(a, ([], []))
            out[a] = (r0 + rs, (s0 or []) + (side or []))
    return out


def test():
    rungs = {}
    for name, dirs in (("R6 6x6 c=5", ["r6"]), ("R7 7x7 c=6", ["r7"]), ("R8 8x8 c=10 s21-50", ["r8a", "r8b"])):
        s = merge(*[load_set(d) for d in dirs])
        if s:
            rungs[name] = s
    extra = {}
    for n in (6, 7):
        for rs_ in (2, 3):
            s = load_set(f"r{n}-seed{rs_}")
            if s:
                extra[(n, rs_)] = s
    summary = {"rungs": {}, "verdicts": {}}
    md = ["## Per rung and arm (run seed 1)", "",
          "| rung | arm | solved/N | Wilson 95% | KM q25 | KM median | KM q75 | median wall ms | total wall s | time-cens | fallback frac | revisit frac | cycled runs |",
          "|---|---|---|---|---|---|---|---|---|---|---|---|---|"]
    pairs_md = ["## Paired comparisons (G1 form, run seed 1)", "",
                "| rung | candidate | baseline | pairs | node ratio | 95% CI | G1 | cand-only | base-only | both-cens | wall ratio | wall 95% CI |",
                "|---|---|---|---|---|---|---|---|---|---|---|---|"]
    for name, s in rungs.items():
        R = {"arms": {}, "pairs": {}}
        for a in ARMS:
            if a not in s:
                continue
            rs, side = s[a]
            d = describe(rs, side)
            R["arms"][a] = d
            md.append(
                f"| {name} | {a} | {d['solved']}/{d['N']} | {d['wilson'][0]:.2f}-{d['wilson'][1]:.2f} | {fmt(d['km_q25'])} | "
                f"{fmt(d['km_median'])} | {fmt(d['km_q75'])} | {fmt(d['median_wall_ms'])} | {fmt(d['total_wall_s'], 1)} | "
                f"{d['time_censored']} | {fmt(d.get('fallback_frac'), 3)} | {fmt(d.get('revisit_frac'), 3)} | "
                f"{len(d['cycled_runs']) if 'cycled_runs' in d else '-'} |")
        dec = "bb-mrv" if name.startswith("R8") else "scb-gacolor_ac3_random"
        comps = [("bj-blocker", dec), ("bj-blocker", "bb-mrv"), ("bj-random", dec), ("bj-random", "bb-mrv"),
                 ("bj-blocker", "bj-random"), ("bb-mrv", "scb-gacolor_ac3_random")]
        seen = set()
        for c, b in comps:
            if (c, b) in seen or c not in s or b not in s:
                continue
            seen.add((c, b))
            p = pair(s[c][0], s[b][0])
            R["pairs"][f"{c} vs {b}"] = p
            pairs_md.append(
                f"| {name} | {c} | {b} | {p['pairs']} | {fmt(p['node_ratio'], 3)} | [{fmt(p['ci_ratio'][0], 3)}, {fmt(p['ci_ratio'][1], 3)}] | "
                f"{'PASS' if p['g1_pass'] else 'fail'} | {p['cand_only_solved']} | {p['base_only_solved']} | {p['both_censored']} | "
                f"{fmt(p['wall_ratio_median'], 3)} | [{fmt(p['wall_ratio_ci'][0], 3)}, {fmt(p['wall_ratio_ci'][1], 3)}] |")
        summary["rungs"][name] = R
    # run seeds 2, 3
    seeds_md = ["## Run seeds 1 to 3 (bj arms, solved/30)", "", "| rung | arm | seed 1 | seed 2 | seed 3 | median |", "|---|---|---|---|---|---|"]
    med_solved = {}
    for n, name in ((6, "R6 6x6 c=5"), (7, "R7 7x7 c=6")):
        if name not in rungs:
            continue
        for a in ("bj-blocker", "bj-random"):
            ks = [summary["rungs"][name]["arms"][a]["solved"]]
            for rs_ in (2, 3):
                if (n, rs_) in extra and a in extra[(n, rs_)]:
                    ks.append(sum(solved(r) for r in extra[(n, rs_)][a][0]))
            m = sorted(ks)[len(ks) // 2] if len(ks) == 3 else None
            med_solved[(n, a)] = m
            seeds_md.append(f"| {name} | {a} | " + " | ".join(str(k) for k in ks + [None] * (3 - len(ks))) + f" | {m} |")
    # verdicts
    V = {}
    r6 = summary["rungs"].get("R6 6x6 c=5")
    r7 = summary["rungs"].get("R7 7x7 c=6")
    r8 = summary["rungs"].get("R8 8x8 c=10 s21-50")
    if r6 and r7:
        g6 = r6["pairs"]["bj-blocker vs scb-gacolor_ac3_random"]["g1_pass"]
        g7 = r7["pairs"]["bj-blocker vs scb-gacolor_ac3_random"]["g1_pass"]
        scb7 = r7["arms"]["scb-gacolor_ac3_random"]["solved"]
        bl7 = med_solved.get((7, "bj-blocker"))
        bl7_s1 = r7["arms"]["bj-blocker"]["solved"]
        V["protocol_G1_R6_pass"] = g6
        V["protocol_dropped_at_R6"] = not g6
        V["R7_gate_pass"] = g7 or bl7_s1 >= scb7 + 4
        bl7_used = bl7 if bl7 is not None else bl7_s1
        V["K1_A20_method_kill"] = (not g6) and (not g7) and bl7_used <= scb7 + 3
        V["K1_inputs"] = {"G1_R6": g6, "G1_R7": g7, "blocker_R7_solved_runseed_median": bl7, "blocker_R7_solved_seed1": bl7_s1, "scb_R7_solved": scb7}
        if r8:
            V["G1_R8_vs_bb_mrv_pass"] = r8["pairs"]["bj-blocker vs bb-mrv"]["g1_pass"]
        V["method_killed"] = V["protocol_dropped_at_R6"] or V["K1_A20_method_kill"] or (r8 is not None and not V["G1_R8_vs_bb_mrv_pass"])
        tw7 = med_solved.get((7, "bj-random"))
        tw7_used = tw7 if tw7 is not None else r7["arms"]["bj-random"]["solved"]
        bt = r7["pairs"]["bj-blocker vs bj-random"]
        V["K2_mechanism_kill_R7"] = tw7_used >= bl7_used - 2 and not bt["significantly_lower"]
        V["K2_inputs"] = {"twin_R7_solved": tw7_used, "blocker_R7_solved": bl7_used, "blocker_vs_twin_ci_ln_upper": bt["ci_ln"][1]}
        if r8:
            bt8 = r8["pairs"]["bj-blocker vs bj-random"]
            V["K2_mechanism_kill_R8"] = r8["arms"]["bj-random"]["solved"] >= r8["arms"]["bj-blocker"]["solved"] - 2 and not bt8["significantly_lower"]
    summary["verdicts"] = V
    summary["run_seed_solved_median"] = {f"n{n} {a}": v for (n, a), v in med_solved.items()}
    out = ROOT / "results"
    (out / "summary.json").write_text(json.dumps(summary, indent=1, default=lambda x: None) + "\n")
    text = "\n".join(md + [""] + pairs_md + [""] + seeds_md + ["", "## Verdicts", "", "```", json.dumps(V, indent=1), "```"]) + "\n"
    (out / "tables.md").write_text(text)
    print(text)


if __name__ == "__main__":
    cmd = sys.argv[1] if len(sys.argv) > 1 else "test"
    {"calibrate": calibrate, "test": test}[cmd]()
