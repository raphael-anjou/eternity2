"""Kill-criterion evaluation and summary (see PREREG.md, PREREG_AMENDMENTS.md).

    python scripts/analyze.py   # reads results/raw/*, writes results/summary.json
"""
import json
import math
import os
import statistics as st
import sys

from scipy.stats import mannwhitneyu, wilcoxon

sys.path.insert(0, os.path.dirname(__file__))
from cpsat_spectrum import CONTRAST, PRIMARY, bands  # noqa: E402

R = os.path.join(os.path.dirname(__file__), "..", "results")


def load(name):
    p = os.path.join(R, "raw", name)
    if not os.path.exists(p):
        return []
    with open(p) as f:
        return [json.loads(l) for l in f if l.strip()]


def med(v):
    return st.median(v) if v else None


def iqr(v):
    if len(v) < 2:
        return None
    q = st.quantiles(v, n=4, method="inclusive")
    return [q[0], q[2]]


def interval(rec):
    """[LB, UB] of a band solve (None if no solution found)."""
    if rec is None or rec["ub"] is None:
        return None
    lb = max(0, rec["lb"]) if rec["lb"] is not None else 0
    return [lb, rec["ub"]]


def index(recs):
    return {(r["n"], r["colors"], r["seed"], r["kind"]): r for r in recs}


def band_table(idx, n, c, seeds, kinds):
    rows = []
    for s in seeds:
        row = {"seed": s}
        for k in kinds:
            r = idx.get((n, c, s, k))
            row[k] = interval(r)
            row[k + "_status"] = r["status"] if r else None
            row[k + "_wall_s"] = r["wall_s"] if r else None
        rows.append(row)
    return rows


def verdict(fire_lo, fire_hi):
    """fire_lo/fire_hi: verdict at the two ends. Returns (decided, fires)."""
    if fire_lo == fire_hi:
        return True, fire_lo
    return False, None


def band_stats(rows, kinds):
    out = {}
    for k in kinds:
        lo = [r[k][0] for r in rows if r[k]]
        hi = [r[k][1] for r in rows if r[k]]
        out[k] = {"median_lb": med(lo), "median_ub": med(hi), "iqr_ub": iqr(hi),
                  "closed": sum(1 for r in rows if r[k] and r[k][0] == r[k][1]),
                  "n": len(rows)}
    return out


def main():
    s1 = index(load("stage1.jsonl"))
    s2 = index(load("stage2.jsonl"))
    curve = index(load("curve.jsonl"))
    s3 = load("stage3.jsonl")
    ann = {(d["n"], d["colors"]): d for d in load("annealed.jsonl")}
    summary = {"stages": {}}
    census = {}
    for d in load("../instances.jsonl"):
        census.setdefault((d["n"], d["colors"]), set()).add((round(d["p_f"], 12), round(d["p_i"], 12)))
    summary["collision_probs_seed_independent"] = {f"{n}x{n}_C{c}": len(v) == 1 for (n, c), v in sorted(census.items())}
    summary["rotation_duplicates_total"] = sum(d["rotation_duplicates"] for d in load("../instances.jsonl"))

    # ---- stage 1 tables
    tabs = {}
    for n in (5, 6):
        for c, seeds in ((PRIMARY[n], range(1, 31)), (CONTRAST[n], range(1, 11))):
            rows = band_table(s1, n, c, seeds, ("near", "mid", "far", "k2"))
            for r in rows:
                if r["mid"] and r["far"]:
                    r["B"] = [r["mid"][0] - r["far"][1], r["mid"][1] - r["far"][0]]
                else:
                    r["B"] = None
                if r["mid"] and r["near"] and r["far"]:
                    r["mid_strictly_above"] = r["mid"][0] > max(r["near"][1], r["far"][1])
            tabs[n, c] = rows
            summary["stages"][f"stage1_{n}x{n}_C{c}"] = {
                "bands": band_stats(rows, ("near", "mid", "far", "k2")),
                "barrier_median_lo": med([r["B"][0] for r in rows if r["B"]]),
                "barrier_median_hi": med([r["B"][1] for r in rows if r["B"]]),
                "barrier_iqr_exact_or_ub": iqr([r["B"][1] for r in rows if r["B"]]),
                "share_mid_strictly_above_near_and_far_certified": (
                    sum(1 for r in rows if r.get("mid_strictly_above")) / len(rows)),
                "rows": rows,
            }

    # ---- K1a
    k1a = {}
    for n in (5, 6):
        rows = tabs[n, PRIMARY[n]]
        if any(r["B"] is None for r in rows) or len(rows) < 30:
            k1a[n] = {"decided": False, "fires": None, "reason": "missing solves"}
            continue
        lo = med([r["B"][0] for r in rows])
        hi = med([r["B"][1] for r in rows])
        d, f = verdict(lo <= 0, hi <= 0)
        k1a[n] = {"median_B_unfavourable": lo, "median_B_favourable": hi, "decided": d, "fires": f}

    # ---- K1b
    r5, r6 = tabs[5, PRIMARY[5]], tabs[6, PRIMARY[6]]
    k1b = {"decided": False, "fires": None}
    if all(r["B"] for r in r5 + r6) and len(r5) == len(r6) == 30:
        def test(d):
            if all(x == 0 for x in d):
                return 1.0
            return float(wilcoxon(d, alternative="greater", zero_method="wilcox",
                                  method="approx", correction=True).pvalue)
        d_unf = [b6["B"][0] - b5["B"][1] for b5, b6 in zip(r5, r6)]
        d_fav = [b6["B"][1] - b5["B"][0] for b5, b6 in zip(r5, r6)]
        p_unf, p_fav = test(d_unf), test(d_fav)
        dd, f = verdict(p_unf > 0.1, p_fav > 0.1)
        B5 = [r["B"][1] for r in r5]
        B6 = [r["B"][1] for r in r6]
        k1b = {"p_unfavourable": p_unf, "p_favourable": p_fav, "decided": dd, "fires": f,
               "d_ub_side": [b6 - b5 for b5, b6 in zip(B5, B6)],
               "mean_B_5x5_ub": st.mean(B5), "mean_B_6x6_ub": st.mean(B6),
               "secondary_mannwhitney_p_ub": float(mannwhitneyu(B6, B5, alternative="greater").pvalue)}

    # ---- stage 3 per-a minima, 8x8 seeds 1..10 for K1c
    s3min = {}
    for r in s3:
        key = (r["n"], r["colors"], r["seed"])
        cur = s3min.setdefault(key, [-1] * len(r["min_b_by_a"]))
        for a, b in enumerate(r["min_b_by_a"]):
            if b >= 0 and (cur[a] < 0 or b < cur[a]):
                cur[a] = b

    # ---- stage 2 and K1c
    k1c = {}
    for n in (7, 8):
        c = PRIMARY[n]
        b = bands(n)
        rows = band_table(s2, n, c, range(1, 11), ("near", "mid", "far"))
        count = 0
        for r in rows:
            mids = [r["mid"][1]] if r["mid"] else []
            fars = [r["far"][1]] if r["far"] else []
            if n == 8 and (n, c, r["seed"]) in s3min:
                v = s3min[n, c, r["seed"]]
                mids += [v[a] for a in range(b["mid"][0], b["mid"][1] + 1) if v[a] >= 0]
                fars += [v[a] for a in range(b["far"][0], b["far"][1] + 1) if v[a] >= 0]
            r["best_mid"] = min(mids) if mids else None
            r["best_far"] = min(fars) if fars else None
            r["mid_le_far"] = (r["best_mid"] is not None and r["best_far"] is not None
                               and r["best_mid"] <= r["best_far"])
            count += r["mid_le_far"]
        missing = sum(1 for r in rows if r["best_mid"] is None or r["best_far"] is None)
        fires = count >= 3
        decided = fires or (count + missing < 3)
        k1c[n] = {"instances_mid_le_far": count, "missing": missing, "of": len(rows),
                  "decided": decided, "fires": fires if decided else None}
        summary["stages"][f"stage2_{n}x{n}_C{c}"] = {"bands": band_stats(rows, ("near", "mid", "far")), "rows": rows}
        cc = CONTRAST[n]
        crow = band_table(s2, n, cc, range(1, 6), ("mid", "far"))
        summary["stages"][f"stage2_{n}x{n}_C{cc}"] = {"bands": band_stats(crow, ("mid", "far")), "rows": crow}

    # ---- K2
    k2 = {"decided": False, "fires": None}
    if all(r["k2"] for r in r6) and len(r6) == 30:
        lo = med([r["k2"][0] for r in r6])
        hi = med([r["k2"][1] for r in r6])
        d, f = verdict(hi <= 2, lo <= 2)
        k2 = {"median_m2_lb": lo, "median_m2_ub": hi, "decided": d, "fires": f}

    # ---- K3 and annealed
    k3 = {"decided": False, "fires": None}
    annsum = {}
    for (n, c), d in sorted(ann.items()):
        bm = bands(n)["mid"]
        vals = [d["b_ann"][a] for a in range(bm[0], bm[1] + 1) if d["b_ann"][a] is not None]
        annsum[f"{n}x{n}_C{c}"] = {"b_ann": d["b_ann"], "b_ann_mid": min(vals) if vals else None,
                                    "p_f": d["p_f"], "p_i": d["p_i"],
                                    "b_ann_bands": {k: min((d["b_ann"][a] for a in range(lo, hi + 1)
                                                             if d["b_ann"][a] is not None), default=None)
                                                    for k, (lo, hi) in bands(n).items()}}
    key6 = f"6x6_C{PRIMARY[6]}"
    if key6 in annsum and all(r["mid"] for r in r6) and len(r6) == 30:
        ba = annsum[key6]["b_ann_mid"]
        dlo = med([r["mid"][0] - ba for r in r6])
        dhi = med([r["mid"][1] - ba for r in r6])
        # |median| > 2 evaluated at both ends of the median interval.
        if dlo <= 0 <= dhi:
            closest = 0
        else:
            closest = min(abs(dlo), abs(dhi))
        farthest = max(abs(dlo), abs(dhi))
        d_, f = verdict(closest > 2, farthest > 2)
        k3 = {"b_ann_mid": ba, "median_diff_lb": dlo, "median_diff_ub": dhi, "decided": d_, "fires": f}
    summary["annealed"] = annsum

    # ---- curves and radius
    curves = {}
    for n in (5, 6):
        c = PRIMARY[n]
        per = {}
        for s in range(1, 31):
            pts = [curve.get((n, c, s, f"a{a}")) for a in range(1, n * n)]
            if any(p is None for p in pts):
                continue
            per[s] = [interval(p) for p in pts]
        if not per:
            continue
        N = n * n
        med_ub = [med([per[s][a - 1][1] for s in per if per[s][a - 1]]) for a in range(1, N)]
        med_lb = [med([per[s][a - 1][0] for s in per if per[s][a - 1]]) for a in range(1, N)]
        closed = [sum(1 for s in per if per[s][a - 1] and per[s][a - 1][0] == per[s][a - 1][1]) for a in range(1, N)]
        radius = {}
        consistency = []
        for s, pts in per.items():
            row = tabs[n, c][s - 1]
            if row["mid"] is None:
                continue
            bthr = row["mid"][1] - 1
            r_ = 0
            certain = True
            for a in range(1, N):
                p = pts[a - 1]
                if p is None:
                    certain = False
                    break
                if p[0] > bthr:
                    break
                if p[1] <= bthr:
                    r_ = a
                else:
                    certain = False
                    break
            radius[s] = {"b": bthr, "r": r_, "certain": certain}
            for k, (lo, hi) in bands(n).items():
                seg = pts[lo - 1:hi]
                if row[k] and all(p and p[0] == p[1] for p in seg) and row[k][0] == row[k][1]:
                    consistency.append(min(p[0] for p in seg) == row[k][0])
        curves[f"{n}x{n}_C{c}"] = {"seeds": sorted(per), "median_ub": med_ub, "median_lb": med_lb,
                                  "closed_per_a": closed, "radius": radius,
                                  "median_radius": med([v["r"] for v in radius.values()]),
                                  "band_consistency_checks": len(consistency),
                                  "band_consistency_failures": consistency.count(False)}
    summary["curves"] = curves

    # ---- stage 3 positions
    pos = {}
    for r in s3:
        n = r["n"]
        bd = bands(n)
        key = f"{n}x{n}_C{r['colors']}/{r['sampler']}"
        p = pos.setdefault(key, {"runs": 0, "runs_with_board": 0, "best_breaks": [],
                                 "low_visits_by_band": {k: 0 for k in ("zero", "near", "mid", "far")},
                                 "best_by_band": {k: 0 for k in ("zero", "near", "mid", "far")}})
        p["runs"] += 1
        if r["best_breaks"] is None:
            continue
        p["runs_with_board"] += 1
        p["best_breaks"].append(r["best_breaks"])

        def band_of(a):
            if a == 0:
                return "zero"
            for k in ("near", "mid", "far"):
                if bd[k][0] <= a <= bd[k][1]:
                    return k
            return "far"

        for a, v in enumerate(r["low_by_a"]):
            p["low_visits_by_band"][band_of(a)] += v
        p["best_by_band"][band_of(r["best_a"])] += 1
    for p in pos.values():
        tot = sum(p["low_visits_by_band"].values())
        p["low_visit_share"] = {k: (v / tot if tot else None) for k, v in p["low_visits_by_band"].items()}
        p["median_best_breaks"] = med(p["best_breaks"])
        del p["best_breaks"]
    summary["stage3"] = pos

    # ---- outcome
    crit = {"K1a_5x5": k1a.get(5), "K1a_6x6": k1a.get(6), "K1b": k1b,
            "K1c_7x7": k1c.get(7), "K1c_8x8": k1c.get(8)}
    k1_fires = any(v and v.get("fires") for v in crit.values())
    k1_decided = k1_fires or all(v and v.get("decided") for v in crit.values())
    K = {"K1": {"decided": k1_decided, "fires": k1_fires if k1_decided else None, "parts": crit},
         "K2": k2, "K3": k3}
    if any(v["fires"] for v in K.values()):
        outcome = "killed"
    elif all(v["decided"] for v in K.values()):
        outcome = "survived"
    else:
        outcome = "inconclusive"
    summary["kill_criteria"] = K
    summary["self_assessed_outcome"] = outcome
    with open(os.path.join(R, "summary.json"), "w") as f:
        json.dump(summary, f, indent=1)
    print(json.dumps({"outcome": outcome, "K1": crit, "K2": k2, "K3": k3}, indent=1))


if __name__ == "__main__":
    main()
