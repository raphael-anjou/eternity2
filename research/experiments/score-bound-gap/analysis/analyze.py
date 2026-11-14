#!/usr/bin/env python3
"""Score-bound gap: analysis of the test rows (PREREG.md, analysis plan).

Reads results/test/measure-*.jsonl, writes results/summary.json and prints the
README tables (markdown). Stdlib only: python3 -I analysis/analyze.py
"""
import glob
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
WIDTHS = [1, 64, 512, 4096]
INF = math.inf


def median(xs):
    xs = sorted(xs)
    k = len(xs)
    if k == 0:
        return None
    if k % 2:
        return xs[k // 2]
    a, b = xs[k // 2 - 1], xs[k // 2]
    if a == INF or b == INF:
        return INF if b == INF and a == INF else (a + b) / 2 if b != INF else INF
    return (a + b) / 2


def quantile(xs, q):
    """Nearest-rank quantile (rank ceil(q*N))."""
    xs = sorted(xs)
    if not xs:
        return None
    return xs[max(0, math.ceil(q * len(xs)) - 1)]


def load():
    rows = []
    for f in sorted(glob.glob(os.path.join(ROOT, "results/test/measure-*.jsonl"))):
        for line in open(f):
            if line.strip():
                rows.append(json.loads(line))
    # Deduplicate by key (a prefix measured on two machines keeps the first).
    seen, out = set(), []
    for r in rows:
        k = (r["id"], r["source"], r["r"], r["draw"])
        if k not in seen:
            seen.add(k)
            out.append(r)
    return out


def bound_of(row, w):
    if w == "trivial":
        return row["trivial"]
    for d in row["dd"]:
        if d["w"] == w:
            return d["bound"]
    raise KeyError(w)


def gaps(rows, w):
    """(finished gaps, lo-variant gaps, hi-variant gaps)."""
    fin, lo, hi = [], [], []
    for r in rows:
        b = bound_of(r, w)
        if r["truth"] is not None:
            g = b - r["truth"]
            fin.append(g)
            lo.append(g)
            hi.append(g)
        else:
            lo.append(b - r["truth_ub"])
            hi.append(INF)
    return fin, lo, hi


def share(xs, pred):
    return sum(1 for x in xs if pred(x)) / len(xs) if xs else None


def fmt(x):
    if x is None:
        return "n/a"
    if x == INF:
        return "inf"
    if isinstance(x, float):
        return f"{x:.2f}".rstrip("0").rstrip(".") if abs(x - round(x)) > 1e-9 else str(int(round(x)))
    return str(x)


def cell_stats(rows, w):
    fin, lo, hi = gaps(rows, w)
    st = {
        "n_prefixes": len(rows),
        "n_finished": len(fin),
        "median_finished": median(fin),
        "q1_finished": quantile(fin, 0.25),
        "q3_finished": quantile(fin, 0.75),
        "M_lo": median(lo),
        "M_hi": median(hi),
        # Shares with censored prefixes counted as not within (conservative).
        "share_gap0": share(hi, lambda g: g == 0),
        "share_le1": share(hi, lambda g: g <= 1),
        "share_le2": share(hi, lambda g: g <= 2),
        "share_le4": share(hi, lambda g: g <= 4),
        "share_gap0_finished": share(fin, lambda g: g == 0),
        "share_le2_finished": share(fin, lambda g: g <= 2),
    }
    prune = {}
    for dk in (0, 1, 2, 4):
        dead, pruned = 0, 0
        for r in rows:
            t = r["max_score"] - dk
            truth_known_below = (r["truth"] is not None and r["truth"] < t) or (
                r["truth"] is None and r["truth_ub"] < t)
            if truth_known_below:
                dead += 1
                if bound_of(r, w) < t:
                    pruned += 1
        prune[f"opt-{dk}"] = {"dead": dead, "pruned": pruned, "share": pruned / dead if dead else None}
    st["prune"] = prune
    if w != "trivial":
        ds = [next(d for d in r["dd"] if d["w"] == w) for r in rows]
        st["dd_nodes_median"] = median([d["nodes"] for d in ds])
        st["dd_transitions_median"] = median([d["transitions"] for d in ds])
        st["dd_wall_ms_median"] = median([d["wall_us"] / 1000 for d in ds])
        st["dd_wall_ms_total"] = sum(d["wall_us"] for d in ds) / 1000
        af = sum(d["absorbed_frontier"] for d in ds)
        ap = sum(d["absorbed_pieces_only"] for d in ds)
        st["frontier_relax_share_absorbed"] = af / (af + ap) if af + ap else 0.0
        lf = sum(d["layers_frontier_relaxed"] for d in ds)
        la = sum(d["layers"] for d in ds)
        st["frontier_relax_share_layers"] = lf / la if la else 0.0
    return st


def main():
    rows = load()
    if not rows:
        print("no rows")
        return 1
    viol = [r for r in rows if r["soundness_violation"]]
    # Re-check soundness from the raw numbers, independently of the flag.
    recheck = 0
    for r in rows:
        for d in r["dd"]:
            if (r["truth"] is not None and d["bound"] < r["truth"]) or d["bound"] > r["trivial"]:
                recheck += 1
    summary = {"n_rows": len(rows), "soundness_violation_rows": len(viol),
               "soundness_recheck_failures": recheck, "machines": sorted({r["machine"] for r in rows}),
               "rungs": {}}
    rungs = sorted({(r["n"], r["c"]) for r in rows})
    for n, c in rungs:
        rk = f"n{n}-c{c}"
        summary["rungs"][rk] = {}
        for rr in sorted({r["r"] for r in rows if (r["n"], r["c"]) == (n, c)}):
            for src in ("pooled", "schedule", "strict"):
                sub = [r for r in rows if (r["n"], r["c"], r["r"]) == (n, c, rr)
                       and (src == "pooled" or r["source"] == src)]
                if not sub:
                    continue
                ent = {"n_prefixes": len(sub),
                       "coverage": sum(r["truth"] is not None for r in sub) / len(sub),
                       "oracle_nodes_median": median([r["oracle_nodes"] for r in sub]),
                       "prefix_mismatches_median": median([r["prefix_mismatches"] for r in sub]),
                       "true_deficit_median_finished": median(
                           [r["max_score"] - r["truth"] for r in sub if r["truth"] is not None]),
                       "widths": {}}
                for w in WIDTHS + ["trivial"]:
                    ent["widths"][str(w)] = cell_stats(sub, w)
                summary["rungs"][rk].setdefault(str(rr), {})[src] = ent

    # Kill criteria on the 7x7 critical rung, r = 3, pooled.
    dec = summary["rungs"].get("n7-c6", {}).get("3", {}).get("pooled")
    verdict = {}
    if dec:
        lo = {w: dec["widths"][str(w)]["M_lo"] for w in WIDTHS}
        hi = {w: dec["widths"][str(w)]["M_hi"] for w in WIDTHS}
        if all(lo[w] > 4 for w in WIDTHS):
            k1 = "fires"
        elif any(hi[w] <= 4 for w in WIDTHS):
            k1 = "does not fire"
        else:
            k1 = "undetermined"
        d_lo = lo[1] - lo[4096]
        d_hi = (hi[1] - hi[4096]) if hi[4096] != INF else None
        k2_lo = d_lo <= 1
        k2_hi = None if d_hi is None else d_hi <= 1
        if k2_lo and k2_hi:
            k2 = "fires"
        elif (not k2_lo) and k2_hi is False:
            k2 = "does not fire"
        else:
            k2 = "undetermined"
        verdict = {"K1": k1, "K2": k2, "M_lo": lo, "M_hi": {w: (None if v == INF else v) for w, v in hi.items()},
                   "K2_diff_lo": d_lo, "K2_diff_hi": d_hi, "n": dec["n_prefixes"], "coverage": dec["coverage"]}
    summary["verdict"] = verdict

    # Exploratory (not preregistered): bootstrap 95% CI of the decision
    # medians (M_hi variant), 10,000 resamples of prefixes, seed 20261112.
    if dec:
        import random
        rng = random.Random(20261112)
        sub = [r for r in rows if (r["n"], r["c"], r["r"]) == (7, 6, 3)]
        ci = {}
        for w in WIDTHS:
            _, _, hi = gaps(sub, w)
            meds = sorted(median([hi[rng.randrange(len(hi))] for _ in hi]) for _ in range(10000))
            ci[str(w)] = [meds[249], meds[9749]]
        summary["exploratory_bootstrap_ci_decision_median"] = ci

    def clean(o):
        if isinstance(o, float) and o == INF:
            return "inf"
        if isinstance(o, dict):
            return {str(k): clean(v) for k, v in o.items()}
        if isinstance(o, list):
            return [clean(v) for v in o]
        return o

    with open(os.path.join(ROOT, "results/summary.json"), "w") as f:
        json.dump(clean(summary), f, indent=1, sort_keys=True)

    # Markdown tables.
    print(f"rows {len(rows)}, soundness violations {len(viol)}, recheck failures {recheck}\n")
    print("| rung | r | prefixes | oracle coverage | W | median gap (finished) | Q1-Q3 | M_lo | M_hi | gap 0 | gap <= 2 | gap <= 4 | prune opt | prune opt-2 | DD nodes (median) | transitions (median) | DD ms (median) | frontier-relax share |")
    print("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
    for rk, rr_map in summary["rungs"].items():
        for rr in sorted(rr_map, key=int):
            ent = rr_map[rr]["pooled"]
            for w in WIDTHS + ["trivial"]:
                s = ent["widths"][str(w)]
                print("| " + " | ".join([
                    rk, rr, str(ent["n_prefixes"]), f"{ent['coverage']:.2f}", str(w),
                    fmt(s["median_finished"]), f"{fmt(s['q1_finished'])}-{fmt(s['q3_finished'])}",
                    fmt(s["M_lo"]), fmt(s["M_hi"]),
                    fmt(s["share_gap0"]), fmt(s["share_le2"]), fmt(s["share_le4"]),
                    fmt(s["prune"]["opt-0"]["share"]), fmt(s["prune"]["opt-2"]["share"]),
                    fmt(s.get("dd_nodes_median")), fmt(s.get("dd_transitions_median")),
                    fmt(s.get("dd_wall_ms_median")), fmt(s.get("frontier_relax_share_absorbed")),
                ]) + " |")
    print()
    # Compact view: median gap (M_hi; M_lo in brackets when different).
    print("| rung | r | cells left | prefixes | coverage | W=1 | W=64 | W=512 | W=4096 | trivial | W=4096 Q1-Q3 (finished) | W=4096 gap per cell left | W=4096 gap 0 | W=4096 gap <= 2 | W=4096 transitions (median) | W=4096 ms (median) | W=4096 frontier-relax share |")
    print("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
    for rk, rr_map in summary["rungs"].items():
        for rr in sorted(rr_map, key=int):
            ent = rr_map[rr]["pooled"]
            nn = int(rk[1:rk.index("-")])
            cells = int(rr) * nn
            def mm(w):
                s = ent["widths"][str(w)]
                return fmt(s["M_hi"]) + ("" if s["M_hi"] == s["M_lo"] else f" [{fmt(s['M_lo'])}]")
            s4 = ent["widths"]["4096"]
            per_cell = s4["M_hi"] / cells if s4["M_hi"] != INF else INF
            print("| " + " | ".join([rk, rr, str(cells), str(ent["n_prefixes"]), f"{ent['coverage']:.3f}"]
                                    + [mm(w) for w in WIDTHS + ["trivial"]]
                                    + [f"{fmt(s4['q1_finished'])}-{fmt(s4['q3_finished'])}",
                                       ("inf" if per_cell == INF else f"{per_cell:.3f}"),
                                       fmt(s4["share_gap0"]), fmt(s4["share_le2"]),
                                       fmt(s4["dd_transitions_median"]), fmt(s4["dd_wall_ms_median"]),
                                       f"{s4['frontier_relax_share_absorbed']:.2f}"]) + " |")
    print()
    # Pruning of dead-for-target prefixes at the decision stratum and its neighbours.
    print("| rung | r | target | dead prefixes | W=1 | W=64 | W=512 | W=4096 | trivial |")
    print("|---|---|---|---|---|---|---|---|---|")
    for rk, rr_map in summary["rungs"].items():
        for rr in sorted(rr_map, key=int):
            if rr not in ("2", "3"):
                continue
            ent = rr_map[rr]["pooled"]
            for t in ("opt-0", "opt-1", "opt-2", "opt-4"):
                dead = ent["widths"]["1"]["prune"][t]["dead"]
                print(f"| {rk} | {rr} | {t.replace('opt-0', 'opt')} | {dead} | " + " | ".join(
                    fmt(ent["widths"][str(w)]["prune"][t]["share"]) for w in WIDTHS + ["trivial"]) + " |")
    print()
    # Trend with board size at fixed rows remaining (M_hi, pooled).
    print("| track | r | W | " + " | ".join(["n=6", "n=7", "n=8"]) + " |")
    print("|---|---|---|---|---|---|")
    tracks = {"critical": {6: "n6-c5", 7: "n7-c6", 8: "n8-c8"},
              "critical+2": {6: "n6-c7", 7: "n7-c8", 8: "n8-c10"}}
    for tname, tmap in tracks.items():
        for rr in ("1", "2", "3"):
            for w in WIDTHS + ["trivial"]:
                vals = []
                for n in (6, 7, 8):
                    e = summary["rungs"].get(tmap[n], {}).get(rr, {}).get("pooled")
                    vals.append(fmt(e["widths"][str(w)]["M_hi"]) + (
                        f" (cov {e['coverage']:.3f})" if e and e["coverage"] < 1 else "") if e else "not run")
                print(f"| {tname} | {rr} | {w} | " + " | ".join(vals) + " |")
    print()
    # Per-source view of the decision stratum.
    if dec:
        print("| source | prefixes | coverage | true deficit (median) | " + " | ".join(f"W={w}" for w in WIDTHS) + " | trivial |")
        print("|---|---|---|---|---|---|---|---|---|")
        for src in ("schedule", "strict"):
            e = summary["rungs"]["n7-c6"]["3"][src]
            print(f"| {src} | {e['n_prefixes']} | {e['coverage']:.3f} | {fmt(e['true_deficit_median_finished'])} | "
                  + " | ".join(fmt(e["widths"][str(w)]["M_hi"]) for w in WIDTHS + ["trivial"]) + " |")
    print()
    print("verdict:", json.dumps(clean(verdict)))
    return 0


if __name__ == "__main__":
    sys.exit(main())
