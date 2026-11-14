"""Analysis of every raw file of orders-and-pins. Prints every number of the
README and writes results/summary.json. Stdlib only.

    python3 -I scripts/analyze.py

Rules are the ones in PREREG.md (and PREREG_AMENDMENTS.md if present); the
text governs if code and text disagree.
"""
import json
import math
import os
import statistics as st
from collections import defaultdict

EXP = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
R = os.path.join(EXP, "results")
T95 = {1: 12.706, 2: 4.303, 3: 3.182, 4: 2.776, 5: 2.571, 6: 2.447, 7: 2.365, 8: 2.306, 9: 2.262,
       10: 2.228, 14: 2.145, 19: 2.093, 29: 2.045, 199: 1.972}
L2 = math.log2(10)
B615 = 61.5 / L2
B78 = 78 / L2
BUDGETS = (("2^78", B78), ("2^61.5", B615), ("1e16", 16.0), ("1e15", 15.0))  # A6 adds 1e16 and 1e15
T10 = (40.457, 40.440, 40.473)
summary = {}


def jl(path):
    p = os.path.join(R, path)
    if not os.path.exists(p):
        return []
    return [json.loads(l) for l in open(p) if l.strip()]


def lg(x):
    return math.log10(x) if x > 0 else float("-inf")


def interval(tots):
    """log10 of the mean of the totals, 95% t interval (log10), span of reps (dex)."""
    n = len(tots)
    m = st.mean(tots)
    if n < 2:
        return {"n": n, "log10": lg(m), "lo": None, "hi": None, "span": 0.0}
    se = st.stdev(tots) / math.sqrt(n)
    h = T95.get(n - 1, 1.96) * se
    pos = [t for t in tots if t > 0]
    span = (lg(max(pos)) - lg(min(pos))) if len(pos) == n else float("inf")
    return {"n": n, "log10": lg(m), "lo": lg(m - h) if m - h > 0 else float("-inf"), "hi": lg(m + h),
            "span": span, "mean": m, "se": se}


def group(rows, key):
    g = defaultdict(list)
    for r in rows:
        g[key(r)].append(r)
    return g


def unit_parts(u):
    return u.split(":")


# ---------------- gates ----------------
def check_rows(rows):
    """Track 10's per-depth 3-se test and total error (real-puzzle-profile/scripts/analyze.py)."""
    res = []
    for r in rows:
        if not r["exact_complete"]:
            res.append({"label": r["label"], "complete": False})
            continue
        ex, mean, sd, n = r["exact"], r["mean"], r["sd"], r["reps"]
        out3 = 0
        for k in range(1, len(ex)):
            se = sd[k] / math.sqrt(n)
            if se == 0:
                bad = abs(mean[k] - ex[k]) > 1e-9 * max(1, ex[k])
            else:
                bad = abs(mean[k] - ex[k]) > 3 * se
            out3 += bad
        T = sum(ex[1:])
        m = st.mean(r["totals"])
        res.append({"label": r["label"], "complete": True, "depths": len(ex) - 1, "out3": out3, "T": T,
                    "err_log10": abs(lg(m / T)) if T > 0 else 0.0})
    return res


def gate6():
    rows = jl("gate/gate6.jsonl")
    out = {}
    for unit, rs in group(rows, lambda r: r["unit"]).items():
        _, o, tag = unit_parts(unit)
        res = check_rows(rs)
        done = [x for x in res if x["complete"]]
        pairs = sum(x["depths"] for x in done)
        out3 = sum(x["out3"] for x in done)
        med = st.median(x["err_log10"] for x in done) if done else None
        ok = len(done) == len(res) == 30 and pairs > 0 and out3 / pairs <= 0.02 and med <= 0.05
        ok_total = len(done) == len(res) == 30 and med is not None and med <= 0.05  # A4 (M1) G6-total
        out.setdefault(o, {})[tag] = {"pass_total": ok_total, "instances": len(res), "complete": len(done), "pairs": pairs, "out3": out3,
                                      "share_out3": out3 / pairs if pairs else None, "median_abs_log10_err": med,
                                      "max_abs_log10_err": max((x["err_log10"] for x in done), default=None),
                                      "T_range": [min(x["T"] for x in done), max(x["T"] for x in done)] if done else None,
                                      "pass": ok}
    for o in out:
        out[o]["pass"] = all(out[o].get(t, {}).get("pass", False) for t in ("noclue", "clue5"))  # G6-profile
        out[o]["pass_total"] = all(out[o].get(t, {}).get("pass_total", False) for t in ("noclue", "clue5"))
    return out


def gate_gt():
    """GT: official set, tree truncated at free depth K, exact widths against the
    SMC per-depth mean of 10 repetitions."""
    rows = jl("gate/gt.jsonl")
    enum, reps = {}, defaultdict(list)
    for r in rows:
        p = unit_parts(r["unit"])
        if p[2] == "enum":
            enum[p[1]] = r
        else:
            reps[p[1]].append(r)
    out = {}
    for o, e in enum.items():
        K = e["maxd"]
        ex = e["widths"]
        rs = reps.get(o, [])
        if not e["complete"] or len(rs) < 2:
            out[o] = {"K": K, "complete": e["complete"], "pass": False}
            continue
        n = len(rs)
        bad = 0
        for d in range(1, K + 1):
            v = [10 ** x["log10_width"][d] if x["log10_width"][d] is not None else 0.0 for x in rs]
            m = st.mean(v)
            se = st.stdev(v) / math.sqrt(n)
            if se == 0:
                # SMC kept the whole level; widths are stored as log10 with 6
                # decimals, so equality holds to a relative 1e-5 (amendment A2)
                bad += abs(m - ex[d]) > 1e-5 * max(1, ex[d])
            else:
                bad += abs(m - ex[d]) > 3 * se
        mK = st.mean(10 ** x["log10_width"][K] for x in rs)
        msum = st.mean(sum(10 ** x["log10_width"][d] for d in range(1, K + 1)) for x in rs)
        errK = lg(mK / ex[K])
        errS = lg(msum / sum(ex[1:K + 1]))
        out[o] = {"K": K, "complete": True, "exact_sum_log10": lg(sum(ex[1:K + 1])), "depths_out3": bad,
                  "share_out3": bad / K, "err_log10_WK": errK, "err_log10_sum": errS,
                  "pass": bad / K <= 0.10 and abs(errK) <= 0.05 and abs(errS) <= 0.05}
    return out


# ---------------- Part A ----------------
WALL_DEPTH = 150  # A8, the judge's threshold
def part_a(man, gates6, gt):
    """A4 (M1 to M3): status per order, three-way reading, kill line."""
    rows = jl("a/a-official.jsonl")
    g6_, g5_ = defaultdict(list), defaultdict(list)
    for r in rows:
        p = unit_parts(r["unit"])
        (g5_ if len(p) == 4 else g6_)[p[1]].append(r["total"])
    res = {}
    for o, meta in man["orders"].items():
        tots = g6_.get(o, [])
        if not tots:
            continue
        iv = interval(tots)
        sd_log = st.stdev(lg(t) for t in tots) if len(tots) > 1 and min(tots) > 0 else float("inf")
        lad = interval(g5_[o]) if len(g5_.get(o, [])) >= 2 else None
        ladder_diff = (lad["log10"] - iv["log10"]) if lad else None
        g6ok = meta["kind"] != "named" or gates6.get(o, {}).get("pass_total", False)
        gtok = gt.get(o, {}).get("pass", False)
        ext_min = min(r["extinction_depth"] for r in rows
                      if len(unit_parts(r["unit"])) == 3 and unit_parts(r["unit"])[1] == o)
        # A8: runs that die well before the last depth (here below 150 of 251) saw only a prefix of the tree.
        walled = ext_min < WALL_DEPTH
        measured = (g6ok and gtok and sd_log <= 0.10 and ladder_diff is not None and abs(ladder_diff) <= 0.10
                    and not walled)
        res[o] = {"kind": meta["kind"], "model": meta["model_log10_five"], **iv, "sd_log10_reps": sd_log,
                  "ladder_np1e5": lad, "ladder_minus_np1e6": ladder_diff, "min_extinction_depth": ext_min, "g6_total": g6ok,
                  "g6_profile": gates6.get(o, {}).get("pass") if meta["kind"] == "named" else None, "gt": gtok,
                  "status": "measured" if measured else "bounded below",
                  "markov_lb": iv["log10"] - 1.30, "measured_minus_model": iv["log10"] - meta["model_log10_five"]}
    out = {"orders": res}
    base = res.get("rows-b-lr")
    if not base:
        return out
    out["baseline"] = {"track10": T10, "diff_from_track10": base["log10"] - T10[0],
                       "reproduction_ok": base["status"] == "measured" and abs(base["log10"] - T10[0]) <= 0.1}
    reading = {}
    for o, v in res.items():
        if o == "rows-b-lr":
            continue
        rule = v["log10"] < base["log10"] - 1.0 and v["hi"] < base["lo"]
        if v["status"] == "measured" and rule:
            reading[o] = "beats"
        elif (v["status"] == "measured" and not rule) or v["markov_lb"] > base["hi"] - 1.0:
            reading[o] = "does not beat"
        else:
            reading[o] = "undecided"
        v["reading"] = reading[o]
        v["gain_vs_rows"] = base["log10"] - v["log10"]
        v["gain_interval"] = [base["lo"] - v["hi"], base["hi"] - v["lo"]]
    beats = [o for o, x in reading.items() if x == "beats"]
    und = [o for o, x in reading.items() if x == "undecided"]
    out["beats_row_scan"] = beats
    out["undecided"] = und
    out["kill_line"] = ("reached" if not beats and not und else
                        "not reached (an order beats row scan)" if beats else
                        f"reached for {len(reading) - len(und)} of {len(reading)} compared orders, undecided for {und}")
    meas = {o: v for o, v in res.items() if v["status"] == "measured"}
    if meas:
        best = min(meas, key=lambda o: meas[o]["log10"])
        out["best_measured"] = {"order": best, "log10": meas[best]["log10"],
                                "gain_vs_rows": base["log10"] - meas[best]["log10"],
                                "gain_interval": [base["lo"] - meas[best]["hi"], base["hi"] - meas[best]["lo"]]}
        lines = [o for o in meas if o.startswith(("rows-", "cols-"))]
        if lines:
            worst_line = max(lines, key=lambda o: meas[o]["log10"])
            best_line = min(lines, key=lambda o: meas[o]["log10"])
            out["line_scans"] = {"measured": len(lines), "best": best_line, "worst": worst_line,
                                 "best_minus_rows_dex": meas[best_line]["log10"] - base["log10"],
                                 "worst_minus_best_bits": (meas[worst_line]["log10"] - meas[best_line]["log10"]) * L2}
        names = sorted(meas)
        if len(names) > 2:
            rho = spearman([meas[o]["model"] for o in names], [meas[o]["log10"] for o in names])
            mbest = min(meas[o]["model"] for o in names)
            c2 = all(meas[o]["log10"] <= meas[best]["log10"] + 1.0 for o in names if meas[o]["model"] <= mbest + 0.5)
            c3 = all(meas[o]["log10"] >= base["log10"] for o in names if meas[o]["model"] > base["model"] + 1.0)
            out["model_vs_measured"] = {"orders": len(names), "spearman": rho, "cond_rho_ge_0.9": rho >= 0.9,
                                        "cond_model_near_best": c2, "cond_no_model_bad_measured_good": c3,
                                        "model_sufficient": rho >= 0.9 and c2 and c3}
        alln = sorted(res)
        out["spearman_model_vs_point_estimates_all_orders"] = spearman([res[o]["model"] for o in alln],
                                                                        [res[o]["log10"] for o in alln])
    nrows = jl("a/a-null.jsonl")
    ng = group(nrows, lambda r: (unit_parts(r["unit"])[1], unit_parts(r["unit"])[2]))
    null = {}
    for bd in sorted({b for (b, _) in ng}):
        per = {o: interval([r["total"] for r in ng[(bb, o)]]) for (bb, o) in ng if bb == bd}
        if "rows-b-lr" not in per:
            continue
        gains = {o: per["rows-b-lr"]["log10"] - v["log10"] for o, v in per.items()}
        common = sorted(set(per) & set(res))
        null[bd] = {"orders": per, "gain_vs_rows": gains, "best": min(per, key=lambda o: per[o]["log10"]),
                    "spearman_with_official": spearman([per[o]["log10"] for o in common],
                                                       [res[o]["log10"] for o in common]) if len(common) > 2 else None}
    out["null"] = null
    if null:
        flags = {}
        for o, v in res.items():
            if o == "rows-b-lr":
                continue
            ng_ = [null[bd]["gain_vs_rows"][o] for bd in null if o in null[bd]["gain_vs_rows"]]
            if ng_:
                g_ = base["log10"] - v["log10"]
                flags[o] = {"official_gain": g_, "null_gains": ng_, "ordinary": min(ng_) - 0.2 <= g_ <= max(ng_) + 0.2}
        out["null_flags"] = flags
    return out


def ranks(v):
    idx = sorted(range(len(v)), key=lambda i: v[i])
    r = [0.0] * len(v)
    i = 0
    while i < len(v):
        j = i
        while j + 1 < len(v) and v[idx[j + 1]] == v[idx[i]]:
            j += 1
        for t in range(i, j + 1):
            r[idx[t]] = (i + j) / 2 + 1
        i = j + 1
    return r


def spearman(a, b):
    ra, rb = ranks(a), ranks(b)
    ma, mb = st.mean(ra), st.mean(rb)
    num = sum((x - ma) * (y - mb) for x, y in zip(ra, rb))
    den = math.sqrt(sum((x - ma) ** 2 for x in ra) * sum((y - mb) ** 2 for y in rb))
    return num / den if den else float("nan")


# ---------------- Part B ----------------
EARLY = 1e-3  # A4 (M5 deviation): an SMC repetition is extinct early if the
# estimated width at its last nonzero depth exceeds 1e-3 of its own total


def smc_early(r):
    lw = [x for x in r["log10_width"] if x is not None]
    if not lw or r["total"] <= 0:
        return True
    return 10 ** lw[-1] > EARLY * r["total"]


def per_depth(rs, D):
    v = [10 ** x["log10_width"][D] if D < len(x["log10_width"]) and x["log10_width"][D] is not None else 0.0
         for x in rs]
    return st.mean(v)


def per_sum(rs, D):
    return st.mean(sum(10 ** x["log10_width"][d] for d in range(1, D + 1)
                       if d < len(x["log10_width"]) and x["log10_width"][d] is not None) for x in rs)


def gate_gb():
    """A4 (M6): truncated exact trees under clustered and spread pins, both calibration boards."""
    rows = jl("gate/gb.jsonl")
    enum, est = {}, defaultdict(list)
    for r in rows:
        p = unit_parts(r["unit"])
        if p[3].startswith("enum"):
            enum[(p[1], p[2], int(p[3][4:]))] = r
        else:
            est[(p[1], p[2], p[3])].append(r)
    out = {}
    for pol in sorted({k[1] for k in enum}):
        Ds = sorted({k[2] for k in enum if k[1] == pol}, reverse=True)
        D = next((d for d in Ds if enum.get(("100149", pol, d), {}).get("complete")), None)
        res = {"D": D}
        for e in ("smc", "ppd"):
            ok_all, det = D is not None, {}
            for bd in ("100149", "100244"):
                en = enum.get((bd, pol, D)) if D else None
                rs = est.get((bd, pol, e), [])
                if not en or not en["complete"] or len(rs) < 2:
                    ok_all = False
                    det[bd] = {"complete": bool(en and en["complete"]), "reps": len(rs)}
                    continue
                ex = en["widths"]
                eD = lg(per_depth(rs, D) / ex[D]) if ex[D] > 0 else float("inf")
                eS = lg(per_sum(rs, D) / sum(ex[1:D + 1]))
                ok = abs(eD) <= 0.10 and abs(eS) <= 0.10
                ok_all = ok_all and ok
                det[bd] = {"exact_WD_log10": lg(ex[D]), "exact_sum_log10": lg(sum(ex[1:D + 1])),
                           "err_WD": eD, "err_sum": eS, "pass": ok}
            res[e] = {"pass": ok_all, "boards": det}
        out[pol] = res
    return out


def gate_g8():
    """A4 (M7): 8x8 implementation gate for both estimators with pins."""
    rows = jl("gate/g8.jsonl")
    enum, est = {}, defaultdict(list)
    for r in rows:
        p = unit_parts(r["unit"])
        (enum.__setitem__((p[1], p[2]), r) if p[3] == "enum" else est[(p[1], p[2], p[3])].append(r["total"]))
    out = {}
    for e in ("smc", "ppd"):
        ok_pairs, errs, n, skipped, low = 0, [], 0, 0, 0
        for key, en in enum.items():
            if not en["complete"]:
                skipped += 1
                continue
            tots = est.get((key[0], key[1], e), [])
            if len(tots) < 2:
                continue
            ex = en["total"]
            m = st.mean(tots)
            se = st.stdev(tots) / math.sqrt(len(tots))
            err = lg(m / ex)
            n += 1
            errs.append(abs(err))
            low += m < ex
            ok_pairs += abs(m - ex) <= 3 * se or abs(err) <= 0.02
        out[e] = {"pairs": n, "skipped_enumeration_past_cap": skipped, "share_ok": ok_pairs / n if n else None,
                  "median_abs_log10_err": st.median(errs) if errs else None, "mean_below_exact": low,
                  "pass": bool(n) and ok_pairs / n >= 0.95 and st.median(errs) <= 0.02}
    return out


def b_configs(stage):
    """{(board, tag): both estimators, pooled value (A4, M4), validation diagnostics (M5)}"""
    g = defaultdict(lambda: defaultdict(list))
    for r in jl(f"b/{stage}.jsonl"):
        p = unit_parts(r["unit"])
        g[(p[1], p[2])][p[3]].append(r)
    out = {}
    for key, ests in g.items():
        v = {}
        for e, rs in ests.items():
            tots = [r["total"] for r in rs]
            v[e] = interval(tots)
            v[e]["sd_log10"] = st.stdev(lg(t) for t in tots) if len(tots) > 1 and min(tots) > 0 else float("inf")
            if e == "smc":
                v["smc_early_extinct"] = sum(smc_early(r) for r in rs)
        if "smc" in v and "ppd" in v:
            pooled = (v["smc"]["mean"] + v["ppd"]["mean"]) / 2
            v["value"] = lg(pooled)
            v["disagree_dex"] = abs(v["smc"]["log10"] - v["ppd"]["log10"])
        else:
            v["value"] = v["smc"]["log10"]
        v["markov_lb"] = v["value"] - 1.30
        out[key] = v
    return out


def validated(v, gb_ok):
    return (gb_ok and "ppd" in v and v.get("smc_early_extinct", 1) == 0 and v["smc"]["sd_log10"] <= 0.30
            and v["ppd"]["sd_log10"] <= 0.30 and v["disagree_dex"] <= 0.50)


E2E_OK = {"spread": False}  # A5: set in part_b from the M8 result


def status(v, bud, gb_ok, policy=None):
    if validated(v, gb_ok) and v["value"] + 0.30 < bud:
        return "under"
    if (policy == "spread" and E2E_OK["spread"] and validated(v, True) and v["value"] + 0.30 < bud
            and v["markov_lb"] <= bud):
        return "under (end-to-end only)"
    if v["markov_lb"] > bud:
        return "over"
    return "neither"


def e2e_pass_any():
    """A5: M8 end-to-end pass at k = 40 or 48 (same rule as the end-to-end block below)."""
    bs = b_configs("bs")
    ok = {}
    for kk in ("k40", "k48"):
        ds = []
        for r in jl("b/b4.jsonl"):
            _, b, k_ = unit_parts(r["unit"])
            if k_ != kk:
                continue
            v = bs.get((b, kk))
            if not (v and r["complete"] and "ppd" in v):
                ds.append((False, 9))
                continue
            T = lg(r["total"])
            fits = all((v[e]["lo"] is not None and v[e]["lo"] <= T <= v[e]["hi"]) or abs(T - v[e]["log10"]) <= 0.10
                       for e in ("smc", "ppd"))
            far = max(abs(v[e]["log10"] - T) for e in ("smc", "ppd"))
            ds.append((r["solutions"] >= 1 and fits, far))
        ok[kk] = bool(ds) and sum(p for p, _ in ds) >= 4 and all(f <= 0.30 for _, f in ds)
    return any(ok.values())


def part_b(pm, sets, gb):
    out = {}
    b0 = b_configs("b0")
    T0 = {b: v["smc"] for (b, _), v in b0.items()}
    if not T0:
        return out
    boards = sorted(T0)
    out["base"] = {b: {"smc": b0[(b, "k0")]["smc"], "ppd": b0[(b, "k0")].get("ppd")} for b in boards}
    vals = [T0[b]["log10"] for b in boards]
    out["base_span_dex"] = max(vals) - min(vals)
    out["pooled"] = out["base_span_dex"] <= 1.0
    b1 = b_configs("b1")
    cells = defaultdict(dict)
    for (b, cell), v in b1.items():
        cells[cell][b] = v
    single = {}
    for cell, per in cells.items():
        if len(per) < len(boards):
            continue
        bits = [(T0[b]["log10"] - per[b]["smc"]["log10"]) * L2 for b in boards]
        d = {"mean_bits": st.mean(bits), "min_bits": min(bits), "max_bits": max(bits), "per_board_bits": bits,
             "model_bits": pm["single_gain_log10"][cell] * L2,
             "smc_early_extinct": sum(per[b]["smc_early_extinct"] for b in boards)}
        if all("ppd" in per[b] for b in boards):
            d["ppd_minus_smc_dex"] = [per[b]["ppd"]["log10"] - per[b]["smc"]["log10"] for b in boards]
        single[cell] = d
    out["single"] = single
    if single:
        best = max(single, key=lambda c: single[c]["mean_bits"])
        worst = min(single, key=lambda c: single[c]["mean_bits"])
        out["single_summary"] = {
            "best_cell": best, "best_bits": single[best]["mean_bits"], "worst_cell": worst,
            "worst_bits": single[worst]["mean_bits"],
            "typical_bits_median": st.median(v["mean_bits"] for v in single.values()),
            "spearman_model_vs_measured": spearman([single[c]["model_bits"] for c in single],
                                                   [single[c]["mean_bits"] for c in single]),
            "mean_measured_minus_model_bits": st.mean(v["mean_bits"] - v["model_bits"] for v in single.values()),
            "max_abs_ppd_minus_smc_dex": max((abs(x) for v in single.values() for x in v.get("ppd_minus_smc_dex", [])),
                                             default=None),
            "cells_with_early_extinction": [c for c, v in single.items() if v["smc_early_extinct"]]}
    # GB pass per policy: greedy needs greedy12 and greedy16 for one estimator; spread needs spread16
    def gbp(pols):
        return any(all(gb.get(p, {}).get(e, {}).get("pass") for p in pols) for e in ("smc", "ppd"))
    gb_ok = {"greedy": gbp(["greedy12", "greedy16"]), "spread": gbp(["spread16"]), "random": False}
    E2E_OK["spread"] = e2e_pass_any()
    out["gb_policy_pass"] = gb_ok
    pol = {}
    for stage, name in (("b2", "greedy"), ("bs", "spread"), ("b3", "random")):
        cf = b_configs(stage)
        byk = defaultdict(list)
        for (b, tag), v in cf.items():
            byk[int(tag.split("-")[0][1:])].append({"board": b, "tag": tag, **v})
        res = {}
        for k in sorted(byk):
            v = byk[k]
            res[k] = {"median_log10_pooled": st.median(x["value"] for x in v),
                      "median_bits_gained": st.median((T0[x["board"]]["log10"] - x["value"]) * L2 for x in v),
                      "validated": sum(validated(x, gb_ok[name]) for x in v), "configs": len(v),
                      "max_disagree_dex": max(x["disagree_dex"] for x in v),
                      "per_config": {x["tag"] + "@" + x["board"]: {
                          "smc": round(x["smc"]["log10"], 3), "ppd": round(x["ppd"]["log10"], 3),
                          "pooled": round(x["value"], 3), "markov_lb": round(x["markov_lb"], 3),
                          "smc_early": x["smc_early_extinct"], "sd_smc": round(x["smc"]["sd_log10"], 3),
                          "sd_ppd": round(x["ppd"]["sd_log10"], 3)} for x in v}}
            for bname, bv in BUDGETS:
                sts = [status(x, bv, gb_ok[name], name) for x in v]
                res[k][f"status_{bname}"] = {s_: sts.count(s_) for s_ in ("under", "under (end-to-end only)", "over", "neither")}
            if name == "greedy":
                res[k]["model_log10"] = pm["greedy_log10_tree"][k]
            if name == "random":
                res[k]["model_median_log10"] = pm["random_log10_tree"].get(str(k), {}).get("median")
        brackets = {}
        for bname, bv in BUDGETS:
            if name in ("greedy", "spread"):
                per_board = {}
                for b in boards:
                    ks = [(k, status(x, bv, gb_ok[name], name)) for k in sorted(byk) for x in byk[k] if x["board"] == b]
                    over = [k for k, s_ in ks if s_ == "over"]
                    und = [k for k, s_ in ks if s_ == "under"]
                    und2 = [k for k, s_ in ks if s_ == "under (end-to-end only)"]
                    per_board[b] = {"largest_k_over": max(over) if over else None,
                                    "smallest_k_under": min(und) if und else "not shown within the grid",
                                    "smallest_k_under_end_to_end_only": min(und2) if und2 else None}
                brackets[bname] = per_board
            else:
                over = [k for k in res if res[k][f"status_{bname}"]["over"] * 2 >= res[k]["configs"]]
                und = [k for k in res if res[k][f"status_{bname}"]["under"] * 2 >= res[k]["configs"]]
                brackets[bname] = {"largest_k_half_over": max(over) if over else None,
                                   "smallest_k_half_under": min(und) if und else "not shown within the grid"}
        pol[name] = {"by_k": res, "brackets": brackets}
        add = {}
        for k in ((4,) if name == "greedy" else (8,) if name == "spread" else ()):
            pins = [f"{r},{c}" for (r, c) in (sets["greedy"][:k] if name == "greedy" else sets["spread"][str(k)])]
            rk = []
            for b in boards:
                x = next((y for y in byk.get(k, []) if y["board"] == b), None)
                if x is None or not all(p in cells and b in cells[p] for p in pins):
                    continue
                den = sum(T0[b]["log10"] - cells[p][b]["smc"]["log10"] for p in pins)
                rk.append((T0[b]["log10"] - x["value"]) / den)
            if len(rk) >= 2:
                m, sd = st.mean(rk), st.stdev(rk)
                h = T95[len(rk) - 1] * sd / math.sqrt(len(rk))
                add[k] = {"per_board": rk, "mean": m, "lo": m - h, "hi": m + h,
                          "reading": "sub-additive" if m + h < 1 else ("super-additive" if m - h > 1 else
                                                                       "additive within resolution")}
        pol[name]["additivity"] = add
    out["policies"] = pol
    # B4 end to end, A4 (M8)
    bs = b_configs("bs")
    e2e = {}
    for r in jl("b/b4.jsonl"):
        _, b, kk = unit_parts(r["unit"])
        v = bs.get((b, kk))
        d = {"complete": r["complete"], "log10_exact": lg(r["total"]), "solutions": r["solutions"],
             "nodes_at_first_solution": r["nodes_at_first_solution"], "seconds": r["seconds"]}
        if v and r["complete"]:
            T = lg(r["total"])
            d.update({"smc_minus_exact": v["smc"]["log10"] - T, "ppd_minus_exact": v["ppd"]["log10"] - T,
                      "pooled_minus_exact": v["value"] - T,
                      "first_over_total": (r["nodes_at_first_solution"] / r["total"]) if r["nodes_at_first_solution"] is not None else None})
            fits = all((v[e]["lo"] is not None and v[e]["lo"] <= T <= v[e]["hi"]) or abs(T - v[e]["log10"]) <= 0.10
                       for e in ("smc", "ppd"))
            d["pass_board"] = r["solutions"] >= 1 and fits
        else:
            d["pass_board"] = False
        e2e[f"{b}:{kk}"] = d
    out["end_to_end"] = e2e
    for kk in ("k40", "k48"):
        ds = [v for key, v in e2e.items() if key.endswith(":" + kk)]
        if ds:
            far = any(abs(v.get(f"{e}_minus_exact", 9)) > 0.30 for v in ds for e in ("smc", "ppd"))
            out[f"end_to_end_pass_{kk}"] = sum(v["pass_board"] for v in ds) >= 4 and not far
    # A4 (M11) leave one clue out and the two transfer checks
    bl = defaultdict(list)
    for r in jl("b/bl.jsonl"):
        p = unit_parts(r["unit"])
        bl[(p[1], p[2])].append(r["total"])
    worth = defaultdict(dict)
    for (src, drop), tots in bl.items():
        if src == "official":
            continue
        worth[drop][src] = interval(tots)["log10"] - T0[src]["log10"]
    off = {}
    arows = jl("a/a-official.jsonl")
    ofl = [r["total"] for r in arows if r["unit"].startswith("a:rows-b-lr:np1e5:")]
    of6 = [r["total"] for r in arows if r["unit"].startswith("a:rows-b-lr:r")]
    if ofl:
        o5 = interval(ofl)["log10"]
        for (src, drop), tots in bl.items():
            if src == "official":
                off[drop] = interval(tots)["log10"] - o5
    tr = {"clue_worth_boards": worth, "clue_worth_official": off}
    if of6 and off:
        o6 = interval(of6)["log10"]
        c1 = min(vals) - 0.2 <= o6 <= max(vals) + 0.2
        fails = [d for d in off if not (min(worth[d].values()) - 0.3 <= off[d] <= max(worth[d].values()) + 0.3)]
        tr.update({"official_five_clue": o6, "check_i": c1, "check_ii_failing_clues": fails,
                   "transfer_allowed": c1 and not fails})
    out["transfer"] = tr
    out["model_base"] = pm["model_log10_base"]
    return out


def main():
    man = json.load(open(os.path.join(EXP, "orders", "manifest.json")))
    pm = json.load(open(os.path.join(EXP, "pins", "model.json")))
    sets = json.load(open(os.path.join(EXP, "plans", "partb-sets.json")))
    g6 = gate6()
    g16 = gate_gt()
    summary["gate6"] = g6
    summary["gate_gt"] = g16
    summary["part_a"] = part_a(man, g6, g16)
    summary["gate_gb"] = gate_gb()
    summary["gate_g8"] = gate_g8()
    summary["part_b"] = part_b(pm, sets, summary["gate_gb"])
    with open(os.path.join(R, "summary.json"), "w") as fh:
        json.dump(summary, fh, indent=1, default=str)
    print_report()


def f(x, d=3):
    return "n/a" if x is None else (f"{x:.{d}f}" if isinstance(x, float) else str(x))


def print_report():
    s = summary
    print("== Gates ==")
    for o in sorted(set(s["gate6"]) | set(s["gate_gt"])):
        a = s["gate6"].get(o, {})
        b = s["gate_gt"].get(o, {})
        print(f"{o:24s} G6 noclue {f(a.get('noclue', {}).get('share_out3'), 4)} / {f(a.get('noclue', {}).get('median_abs_log10_err'), 4)}"
              f" clue5 {f(a.get('clue5', {}).get('share_out3'), 4)} / {f(a.get('clue5', {}).get('median_abs_log10_err'), 4)}"
              f" pass {a.get('pass')} | GT K={b.get('K')} sum 10^{f(b.get('exact_sum_log10'))} out3 {b.get('depths_out3')}"
              f" errK {f(b.get('err_log10_WK'))} errSum {f(b.get('err_log10_sum'))} pass {b.get('pass')}")
    print("GB", json.dumps(s["gate_gb"], default=str))
    print("G8", json.dumps(s["gate_g8"], default=str))
    A = s["part_a"]
    print("== Part A ==")
    for o, v in sorted(A.get("orders", {}).items(), key=lambda kv: kv[1]["log10"]):
        print(f"{o:24s} model {v['model']:7.3f} est {v['log10']:7.3f} [{f(v['lo'])}, {f(v['hi'])}] sd {v['sd_log10_reps']:.3f}"
              f" ladder {f(v['ladder_minus_np1e6'])} m-m {v['measured_minus_model']:+.3f} {v['status']} LB {v['markov_lb']:.2f}"
              f" {v.get('reading', 'baseline')}")
    for k in ("baseline", "beats_row_scan", "undecided", "kill_line", "best_measured", "line_scans", "model_vs_measured",
              "spearman_model_vs_point_estimates_all_orders", "null_flags"):
        if k in A:
            print(k, json.dumps(A[k], default=str))
    for b, v in A.get("null", {}).items():
        print("null", b, "best", v["best"], "spearman", f(v["spearman_with_official"]),
              {o: round(g, 3) for o, g in sorted(v["gain_vs_rows"].items(), key=lambda kv: -kv[1])})
    B = s["part_b"]
    print("== Part B ==")
    for k in ("base", "base_span_dex", "single_summary", "gb_policy_pass", "policies", "end_to_end",
              "end_to_end_pass_k40", "end_to_end_pass_k48", "transfer"):
        if k in B:
            print(k, json.dumps(B[k], default=str))
    if "single" in B:
        for c, v in sorted(B["single"].items(), key=lambda kv: -kv[1]["mean_bits"]):
            print(f"cell {c:6s} mean {v['mean_bits']:6.2f} bits [{v['min_bits']:6.2f}, {v['max_bits']:6.2f}] model {v['model_bits']:6.2f}")


if __name__ == "__main__":
    main()
