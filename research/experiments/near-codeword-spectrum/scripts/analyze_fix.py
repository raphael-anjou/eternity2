"""Kill criteria after the round-1 judge fixes (PREREG_AMENDMENTS.md A6 to A10).

    python scripts/analyze_fix.py   # reads results/raw/*, writes results/summary_fix.json
"""
import json
import os
import statistics as st
import sys

from scipy.stats import mannwhitneyu, wilcoxon

sys.path.insert(0, os.path.dirname(__file__))
from analyze import R, band_table, index, iqr, load, med, verdict  # noqa: E402
from cpsat_quotient import free_certificates, bands  # noqa: E402

PRIM = {5: 13, 6: 14}
CONTR = {5: 9, 6: 10}
KINDS = ("near", "mid", "far", "k2")


def wilc(d):
    if all(x == 0 for x in d):
        return 1.0
    return float(wilcoxon(d, alternative="greater", zero_method="wilcox", method="approx",
                          correction=True).pvalue)


def growth(r5, r6, key):
    """K1b-style one-sided Wilcoxon, paired by seed, both interval ends."""
    unf = [b6[key][0] - b5[key][1] for b5, b6 in zip(r5, r6)]
    fav = [b6[key][1] - b5[key][0] for b5, b6 in zip(r5, r6)]
    return wilc(unf), wilc(fav)


def main():
    s1 = index(load("star1.jsonl"))
    insts = {(d["n"], d["colors"], d["seed"]): d for d in load("../instances.jsonl")}
    ann = {(d["n"], d["colors"]): d for d in load("annealed.jsonl")}
    out = {"stages": {}}
    tabs = {}
    for n in (5, 6):
        for c, seeds in ((PRIM[n], range(1, 31)), (CONTR[n], range(1, 11))):
            rows = band_table(s1, n, c, seeds, KINDS)
            for r in rows:
                r["B"] = ([r["mid"][0] - r["far"][1], r["mid"][1] - r["far"][0]]
                          if r["mid"] and r["far"] else None)
                if r["mid"] and r["near"] and r["far"]:
                    r["G"] = [r["mid"][0] - max(r["near"][1], r["far"][1]),
                              r["mid"][1] - max(r["near"][0], r["far"][0])]
                    r["mid_strictly_above"] = r["mid"][0] > max(r["near"][1], r["far"][1])
                    r["mid_certified_not_above_far"] = r["mid"][1] <= r["far"][0]
                rec = s1.get((n, c, r["seed"], "far"))
                r["far_zero_break_second_solution"] = bool(rec and rec["ub"] == 0)
            tabs[n, c] = rows
            stats = {}
            for k in KINDS:
                lo = [r[k][0] for r in rows if r[k]]
                hi = [r[k][1] for r in rows if r[k]]
                stats[k] = {"median_lb": med(lo), "median_ub": med(hi), "iqr_ub": iqr(hi),
                            "mean_lb": st.mean(lo) if lo else None, "mean_ub": st.mean(hi) if hi else None,
                            "closed": sum(1 for r in rows if r[k] and r[k][0] == r[k][1]),
                            "found": len(hi), "n": len(rows)}
            out["stages"][f"star1_{n}x{n}_C{c}"] = {
                "bands": stats,
                "B_median": [med([r["B"][0] for r in rows if r["B"]]), med([r["B"][1] for r in rows if r["B"]])],
                "G_median": [med([r["G"][0] for r in rows if r.get("G")]),
                             med([r["G"][1] for r in rows if r.get("G")])],
                "share_mid_strictly_above_near_and_far": sum(1 for r in rows if r.get("mid_strictly_above")) / len(rows),
                "share_mid_certified_le_far": sum(1 for r in rows if r.get("mid_certified_not_above_far")) / len(rows),
                "far_zero_break_count": sum(r["far_zero_break_second_solution"] for r in rows),
                "rows": rows,
            }

    r5, r6 = tabs[5, 13], tabs[6, 14]
    full = lambda rows, k: len(rows) == 30 and all(r[k] for r in rows)  # noqa: E731
    K = {}
    # K1a* (A6)
    for n, rows in ((5, r5), (6, r6)):
        if not full(rows, "B"):
            K[f"K1a_{n}x{n}"] = {"decided": False, "fires": None, "reason": "missing solves"}
            continue
        lo, hi = med([r["B"][0] for r in rows]), med([r["B"][1] for r in rows])
        d, f = verdict(lo <= 0, hi <= 0)
        K[f"K1a_{n}x{n}"] = {"median_Bstar_unfavourable": lo, "median_Bstar_favourable": hi,
                             "decided": d, "fires": f}
    # K1b* (A6)
    if full(r5, "B") and full(r6, "B"):
        pu, pf = growth(r5, r6, "B")
        d, f = verdict(pu > 0.1, pf > 0.1)
        K["K1b"] = {"p_unfavourable": pu, "p_favourable": pf, "decided": d, "fires": f,
                    "mean_Bstar_5x5": [st.mean(r["B"][0] for r in r5), st.mean(r["B"][1] for r in r5)],
                    "mean_Bstar_6x6": [st.mean(r["B"][0] for r in r6), st.mean(r["B"][1] for r in r6)],
                    "secondary_mannwhitney_p_ub": float(mannwhitneyu([r["B"][1] for r in r6],
                                                                     [r["B"][1] for r in r5],
                                                                     alternative="greater").pvalue)}
    else:
        K["K1b"] = {"decided": False, "fires": None, "reason": "missing solves"}
    # secondary: H*_mid growth alone
    if full(r5, "mid") and full(r6, "mid"):
        out["secondary_Hmid_star_growth_wilcoxon"] = dict(zip(("p_unfavourable", "p_favourable"),
                                                              growth(r5, r6, "mid")))

    # K1c as written (A7), d_0, free far certificate + capped mid solve
    k1c = load("k1c.jsonl")
    kc = {}
    for n, c in ((7, 15), (8, 16)):
        rows = []
        for s in range(1, 11):
            rec = next((r for r in k1c if r["n"] == n and r["seed"] == s), None)
            certs = free_certificates(n, insts[n, c, s]["pieces"])
            far = min(br for k, br, a, d in certs if a >= bands(n)["far"][0])
            row = {"seed": s, "far_certificate": far, "certs": certs,
                   "mid_status": rec["status"] if rec else None,
                   "mid_ub": rec["ub"] if rec else None, "mid_wall_s": rec["wall_s"] if rec else None}
            row["mid_le_far"] = None if rec is None else rec["ub"] is not None
            rows.append(row)
        cnt = sum(1 for r in rows if r["mid_le_far"])
        missing = sum(1 for r in rows if r["mid_le_far"] is None)
        fires = cnt >= 3
        decided = fires or cnt + missing < 3
        kc[n] = rows
        K[f"K1c_{n}x{n}"] = {"instances_mid_le_far": cnt, "missing": missing, "of": 10,
                             "infeasible_proofs": sum(1 for r in rows if r["mid_status"] == "INFEASIBLE"),
                             "decided": decided, "fires": fires if decided else None}
    out["k1c_rows"] = kc

    # K1c* at 7x7 (A9)
    s7 = index(load("star7.jsonl"))
    rows7 = band_table(s7, 7, 15, range(1, 11), ("mid", "far"))
    cnt = sum(1 for r in rows7 if r["mid"] and r["far"] and r["mid"][1] <= r["far"][1])
    missing = sum(1 for r in rows7 if not (r["mid"] and r["far"]))
    fires = cnt >= 3
    decided = fires or cnt + missing < 3
    K["K1c_star_7x7"] = {"instances_mid_le_far": cnt, "missing": missing, "of": 10,
                         "decided": decided, "fires": fires if decided else None}
    out["stages"]["star7_7x7_C15"] = {"rows": rows7, "bands": {
        k: {"median_lb": med([r[k][0] for r in rows7 if r[k]]), "median_ub": med([r[k][1] for r in rows7 if r[k]]),
            "closed": sum(1 for r in rows7 if r[k] and r[k][0] == r[k][1])} for k in ("mid", "far")}}

    # K2* (A6)
    if full(r6, "k2"):
        lo, hi = med([r["k2"][0] for r in r6]), med([r["k2"][1] for r in r6])
        d, f = verdict(hi <= 2, lo <= 2)
        K["K2"] = {"median_m2_lb": lo, "median_m2_ub": hi, "decided": d, "fires": f}
    else:
        K["K2"] = {"decided": False, "fires": None, "reason": "missing solves"}
    # K3* (A6); b_ann reported for near and mid only (A10)
    a6 = ann[6, 14]["b_ann"]
    bm = bands(6)["mid"]
    ba = min(a6[a] for a in range(bm[0], bm[1] + 1) if a6[a] is not None)
    if full(r6, "mid"):
        dlo, dhi = med([r["mid"][0] - ba for r in r6]), med([r["mid"][1] - ba for r in r6])
        closest = 0 if dlo <= 0 <= dhi else min(abs(dlo), abs(dhi))
        d, f = verdict(closest > 2, max(abs(dlo), abs(dhi)) > 2)
        K["K3"] = {"b_ann_mid": ba, "median_diff_lb": dlo, "median_diff_ub": dhi, "decided": d, "fires": f}
    else:
        K["K3"] = {"decided": False, "fires": None, "reason": "missing solves"}
    out["annealed_near_mid"] = {
        f"{n}x{n}_C{c}": {k: min(d["b_ann"][a] for a in range(lo, hi + 1) if d["b_ann"][a] is not None)
                          for k, (lo, hi) in bands(n).items() if k in ("near", "mid")}
        for (n, c), d in sorted(ann.items())}

    k1parts = {k: v for k, v in K.items() if k.startswith("K1")}
    k1f = any(v.get("fires") for v in k1parts.values())
    k1d = k1f or all(v.get("decided") for v in k1parts.values())
    crit = {"K1": {"decided": k1d, "fires": k1f if k1d else None, "parts": k1parts},
            "K2": K["K2"], "K3": K["K3"]}
    if any(v["fires"] for v in crit.values()):
        outcome = "killed"
    elif all(v["decided"] for v in crit.values()):
        outcome = "survived"
    else:
        outcome = "inconclusive"
    out["kill_criteria"] = crit
    out["self_assessed_outcome"] = outcome
    with open(os.path.join(R, "summary_fix.json"), "w") as f:
        json.dump(out, f, indent=1)
    print(json.dumps({"outcome": outcome, **crit,
                      "secondary_Hmid_star_growth": out.get("secondary_Hmid_star_growth_wilcoxon"),
                      "bands": {k: {kk: (vv["median_lb"], vv["median_ub"], vv["closed"])
                                    for kk, vv in v["bands"].items()} for k, v in out["stages"].items()},
                      "B": {k: v.get("B_median") for k, v in out["stages"].items()},
                      "G": {k: v.get("G_median") for k, v in out["stages"].items()}}, indent=1))


if __name__ == "__main__":
    main()
