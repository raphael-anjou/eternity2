"""Refutation-cost and nogood-locality statistics as PREREG.md fixes them.

  python3 -I analyze.py RESULTS_DIR [STAGE0_SUMMARY] > summary.json

RESULTS_DIR holds one directory per set (real, null1101, ...), each with
<set>.puz, <set>.frames, refuters.jsonl, counts.jsonl, base.jsonl.
"""
import json
import math
import os
import random
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common  # noqa: E402

PLAIN = ["R0-rows", "R0-spiral"]
CANDIDATES = ["R1-rows", "R1-spiral", "R2-glucose"]
DFS_CAP = 120.0
CAP = {"R0-rows": 120.0, "R0-spiral": 120.0, "R1-rows": 120.0, "R1-spiral": 120.0, "R2-glucose": 600.0}
ROWSCAN_BITS = 134.4
SEED = 20261113
BOOT = 10000
LOG2_10 = math.log2(10)


def rows(path):
    return [json.loads(line) for line in open(path) if line.strip()]


def lower_median(xs):
    s = sorted(xs)
    return s[(len(s) + 1) // 2 - 1]


def median(xs):
    s = sorted(xs)
    n = len(s)
    if n % 2:
        return s[n // 2]
    a, b = s[n // 2 - 1], s[n // 2]
    return math.inf if math.isinf(a) or math.isinf(b) else (a + b) / 2


def boot_ci(xs, stat, seed=SEED):
    rng = random.Random(seed)
    n = len(xs)
    vals = sorted(stat([xs[rng.randrange(n)] for _ in range(n)]) for _ in range(BOOT))
    return [vals[int(0.025 * BOOT)], vals[int(0.975 * BOOT) - 1]]


def stratum(fid):
    return re.match(r"^(real|n\d+)([HU])", fid).group(2)


def ring_index(n):
    v = list(range(n))
    v += [r * n + n - 1 for r in range(1, n)]
    v += [(n - 1) * n + c for c in range(n - 2, -1, -1)]
    v += [r * n for r in range(n - 2, 0, -1)]
    return {pos: i for i, pos in enumerate(v)}


def geometry(cells, n):
    ri = ring_index(n)
    L = 4 * (n - 1)
    idx = sorted(ri[c] for c in cells)
    gaps = [(idx[(i + 1) % len(idx)] - idx[i]) % L for i in range(len(idx))]
    contiguous = len(idx) <= 1 or sum(1 for g in gaps if g != 1) <= 1
    sides = set()
    for c in cells:
        r, col = divmod(c, n)
        if r == 0:
            sides.add("top")
        if r == n - 1:
            sides.add("bottom")
        if col == 0:
            sides.add("left")
        if col == n - 1:
            sides.add("right")
    corners = [0, n - 1, 2 * (n - 1), 3 * (n - 1)]
    dist = min(min((i - k) % L, (k - i) % L) for i in idx for k in corners)
    has_corner = any(i in corners for i in idx)
    far = max(min(min((i - k) % L, (k - i) % L) for k in corners) for i in idx)
    return {"contiguous": contiguous, "sides": sorted(sides), "has_corner": has_corner, "corner_distance": dist,
            "max_corner_distance": far}


def load_set(d, name, dropped):
    puz = common.load_puz(os.path.join(d, name + ".puz"))
    frames = dict(common.load_frames(os.path.join(d, name + ".frames")))
    ref = {}
    for r in rows(os.path.join(d, "refuters.jsonl")):
        if r["refuter"] in dropped:
            continue
        ref.setdefault(r["id"], {})[r["refuter"]] = r
    counts = {c["id"]: c for c in rows(os.path.join(d, "counts.jsonl"))}
    hp = os.path.join(d, "counts_h.jsonl")
    counts_h = {c["id"]: c for c in rows(hp)} if os.path.exists(hp) else {}
    base = rows(os.path.join(d, "base.jsonl"))
    nf = [b for b in base if b["stratum"] == "U"][0]["log10_mean"]
    nh = [b for b in base if b["stratum"] == "H"][0]["log10_mean"]
    return puz, frames, ref, counts, counts_h, nf, nh


def frame_time(rec, refuter):
    """(seconds, censored) for a refuter on a frame; a missing row is censored."""
    r = rec.get(refuter)
    if r is None or r["verdict"] == "capped":
        return CAP[refuter], True
    return r["secs"], False


def stage1(ids, ref, cands):
    out = {}
    for x in cands:
        ratios, censored_pairs = [], 0
        for fid in ids:
            tp = [frame_time(ref[fid], p) for p in PLAIN]
            dec = [t for t, c in tp if not c]
            p_cens = not dec
            t_p = min(dec) if dec else DFS_CAP
            t_x, x_cens = frame_time(ref[fid], x)
            t_x = max(t_x, 1e-7)
            if not p_cens and not x_cens:
                ratios.append(t_p / t_x)
            elif p_cens and not x_cens:
                ratios.append(DFS_CAP / t_x)
            elif not p_cens and x_cens:
                ratios.append(t_p / CAP[x])
            else:
                ratios.append(1.0)
                censored_pairs += 1
        med = lower_median(ratios) if ratios else None
        ci = boot_ci(ratios, lower_median) if ratios else [None, None]
        # A2: ratio of total times, censored runs entered at their cap
        tot_p = sum(min(frame_time(ref[f], q)[0] for q in PLAIN) for f in ids)
        tot_x = sum(frame_time(ref[f], x)[0] for f in ids)
        out[x] = {"n": len(ratios), "lower_median_ratio": med, "ci95": ci, "censored_pairs": censored_pairs,
                  "passes_100x": bool(ratios) and med >= 100 and ci[0] >= 10,
                  "total_plain_s": tot_p, "total_x_s": tot_x, "total_ratio": tot_p / tot_x if tot_x else None,
                  "plain_censored": sum(1 for f in ids if all(frame_time(ref[f], q)[1] for q in PLAIN)),
                  "x_censored": sum(1 for f in ids if frame_time(ref[f], x)[1])}
    return out


def node_rate(ref):
    rates = [r["R0-rows"]["nodes"] / r["R0-rows"]["secs"] for r in ref.values()
             if "R0-rows" in r and r["R0-rows"]["verdict"] == "capped"]
    if not rates:  # no capped run: fall back to every R0-rows run of at least 1 s
        rates = [r["R0-rows"]["nodes"] / r["R0-rows"]["secs"] for r in ref.values()
                 if "R0-rows" in r and r["R0-rows"]["secs"] >= 1]
    return median(rates) if rates else None, len(rates)


def stage2_frames(name, puz, frames, ref, counts, counts_h, nf, nh, rate, sources):
    """Per frame: every nogood with b_F (among valid frames) and b_H (among
    clue-block-passing frames, amendment A1), C, v_F = b_F + log2 C and
    v_H = b_H + log2 C. The frame's v is min v_H for an H frame, min v_F for
    a U frame; v_F_min is kept for the secondary b_F - beta_H statistic."""
    out = []
    for fid in sorted(frames):
        st = stratum(fid)
        cand = []
        for src in sources:
            r = ref[fid].get(src)
            if r is None or r["verdict"] != "unsat":
                continue
            c = counts.get("%s:%s" % (fid, src))
            if c is None or c["log10_mean"] is None:
                continue
            b_f = (nf - c["log10_mean"]) * LOG2_10
            ch = counts_h.get("%s:%s" % (fid, src))
            b_h = (nh - ch["log10_mean"]) * LOG2_10 if ch and ch["log10_mean"] is not None else None
            if src.startswith("R2"):
                if r.get("core_verdict") != "unsat":
                    continue
                cost = max(r["core_secs"] * rate, 1.0)
                cells = [p for p, _, _ in r["core_placements"]]
            else:
                cost = max(r["nodes"], 1)
                cells = r["read_border"]
            lc = math.log2(cost)
            cand.append({"source": src, "size": len(cells), "b_F_bits": b_f, "b_H_bits": b_h, "log2C": lc,
                         "v_F": b_f + lc, "v_H": None if b_h is None else b_h + lc, "cells": sorted(cells)})
        key = "v_H" if st == "H" else "v_F"
        usable = [x for x in cand if x[key] is not None]
        best = min(usable, key=lambda x: x[key]) if usable else None
        row = {"set": name, "id": fid, "stratum": st, "nogoods": cand,
               "v_F_min": min((x["v_F"] for x in cand), default=math.inf)}
        r2 = ref[fid].get("R2-glucose")
        r2core = [x for x in cand if x["source"] == "R2-glucose"]
        row["r2_core_size"] = r2core[0]["size"] if r2core else math.inf
        if best is None:
            row.update({"v": math.inf, "censored": True})
        else:
            row.update({"v": best[key], "censored": False, "source": best["source"], "size": best["size"],
                        "b_bits": best["b_H_bits"] if st == "H" else best["b_F_bits"], "log2C": best["log2C"]})
            row.update(geometry(best["cells"], puz["n"]))
        if r2:
            row["r2"] = {k: r2.get(k) for k in ("verdict", "secs", "conflicts", "load_secs", "core0", "core", "core_undecided",
                                               "min_calls", "min_secs", "core_verdict", "core_secs", "core_conflicts")}
        row["times"] = {x: ref[fid][x]["secs"] for x in ref[fid]}
        row["verdicts"] = {x: ref[fid][x]["verdict"] for x in ref[fid]}
        row["nodes"] = {x: ref[fid][x].get("nodes") for x in ref[fid] if not x.startswith("R2")}
        out.append(row)
    return out


def stage2_stats(frs, beta):
    vs = [f["v"] for f in frs]
    m = median(vs)
    ci = boot_ci(vs, median)
    r2sizes = [f["r2_core_size"] for f in frs]
    sizes = [f.get("size", math.inf) for f in frs]
    vf_adj = [f["v_F_min"] - beta for f in frs]
    ms = median(r2sizes)
    return {"n": len(frs), "censored": sum(1 for f in frs if f["censored"]), "beta_bits": beta,
            "median_v": m, "ci95": ci,
            "secondary_median_vF_minus_beta": median(vf_adj), "secondary_ci95": boot_ci(vf_adj, median),
            "median_r2_core_size": ms, "r2_core_size_quartiles": [sorted(r2sizes)[len(r2sizes) // 4], sorted(r2sizes)[(3 * len(r2sizes)) // 4]],
            "median_best_nogood_size": median(sizes),
            "reading": ("dead" if (m > ROWSCAN_BITS or ms >= 54) else
                        "passes necessary condition" if (m < 120 and ci[1] < ROWSCAN_BITS) else "undecided")}


ARC_K = (15, 14, 13, 12)
ARC_CAP = {15: 120.0, 14: 450.0, 13: 6400.0, 12: 120.0}  # k = 13: 6400 s on realH0 to realH29, 120 s on the rest
# referee's budgets per block-passing bottom arc (log2 nodes) and log2 counts
ARC_BUDGET = {15: 25.5, 14: 29.0, 13: 32.9, 12: 37.0}
ARC_LOG2COUNT = {16: 107.5, 15: 108.9, 14: 105.4, 13: 101.5, 12: 97.4}


def arcs(d, nf_nh, full_r2, rate):
    """Bottom arcs of the real H frames: A1 counts and readings, and the A2
    nested-cost reading."""
    ref = rows(os.path.join(d, "refuters.jsonl"))
    cnt = rows(os.path.join(d, "counts.jsonl"))
    nf, nh = nf_nh
    bh = {c["id"]: (nh - c["log10_mean"]) * LOG2_10 for c in cnt if c["stratum"] == "H" and c["log10_mean"] is not None}
    bf = {c["id"]: (nf - c["log10_mean"]) * LOG2_10 for c in cnt if c["stratum"] == "U" and c["log10_mean"] is not None}
    out = {}
    byk = {}
    for k in ARC_K:
        cells = 2 * k + 14
        for refuter in ("R0-rowsup-partial", "R2-glucose"):
            rr = [r for r in ref if r["id"].endswith("k%d" % k) and r["refuter"] == refuter]
            if not rr:
                continue
            ids = [r["id"] for r in rr]
            unsat = [r for r in rr if r["verdict"] == "unsat"]
            out["k%d/%s" % (k, refuter)] = {
                "cells": cells, "frames": len(rr), "cap_s": ARC_CAP[k] if refuter.startswith("R2") else 120.0,
                "unsat": len(unsat),
                "unsat_within_60s": sum(1 for r in unsat if r["secs"] <= 60),
                "unsat_within_65s": sum(1 for r in unsat if r["secs"] <= 65),
                "sat": sum(1 for r in rr if r["verdict"] == "sat"),
                "capped": sum(1 for r in rr if r["verdict"] == "capped"),
                "median_secs_unsat": median([r["secs"] for r in unsat]) if unsat else None,
                "unsat_nodes": sorted(r["nodes"] for r in unsat if "nodes" in r),
                "median_b_H_bits": median([bh[i] for i in ids if i in bh]) if any(i in bh for i in ids) else None,
                "median_b_F_bits": median([bf[i] for i in ids if i in bf]) if any(i in bf for i in ids) else None,
            }
            if refuter.startswith("R2"):
                byk[k] = {r["id"][: -len("k%d" % k)]: r for r in rr}
    a1 = {}
    for refuter in ("R0-rowsup-partial", "R2-glucose"):
        below = any(("k%d/%s" % (k, refuter)) in out and
                    out["k%d/%s" % (k, refuter)]["unsat_within_60s"] >= out["k%d/%s" % (k, refuter)]["frames"] / 4
                    for k in (12,))
        k13 = out.get("k13/%s" % refuter)
        killed = k13 is not None and k13["unsat_within_65s"] == 0
        a1[refuter] = ("below the claimed break-even" if below else
                       "prefix route dead at this cap" if killed else
                       "neither line met" if k13 is not None else "k = 13 not run for this refuter")
    out["readings_A1"] = a1
    # A2 nested cost, R2 only
    chain = {}
    frames = sorted(full_r2)
    prev = {}
    up = {15: 16, 14: 15, 13: 14, 12: 13}
    for k in ARC_K:
        e = ARC_LOG2COUNT[up[k]] - ARC_LOG2COUNT[k]
        cs = {}
        for f in frames:
            r = byk.get(k, {}).get(f)
            parent = full_r2[f] if k == 15 else prev[f]
            if r is not None and r["verdict"] == "unsat":
                cs[f] = r["secs"]
            else:
                cap = r["secs"] if r is not None else ARC_CAP[k]
                cs[f] = cap + 2 ** e * parent
        prev = cs
        mean_c = sum(cs.values()) / len(cs)
        log2_nodes = math.log2(mean_c * rate)
        chain["k%d" % k] = {"cells": 2 * k + 14, "exponent": e, "mean_c_s": mean_c, "log2_mean_c_nodes": log2_nodes,
                            "budget_log2": ARC_BUDGET[k], "budget_s": 2 ** ARC_BUDGET[k] / rate,
                            "can_kill_at_cap": 2 ** ARC_BUDGET[k] / rate < ARC_CAP[k],
                            "below_break_even": log2_nodes < ARC_BUDGET[k]}
    out["A2_nested"] = chain
    return out


def floors(frs, rate):
    """A2: lower bounds on log2 of the mean of 2^v. Censored frames enter at
    log2(600 r) (their b taken as 0), a lower bound."""
    lb = math.log2(600 * rate)
    vs = sorted(f["v"] if not f["censored"] else lb for f in frs)
    n = len(vs)
    out = {}
    for p in (0.4, 0.25, 0.1):
        m = math.ceil(p * n)
        out["F(%s)" % p] = vs[n - m] + math.log2(m / n)
    mx = vs[-1]
    out["log2_mean"] = mx + math.log2(sum(2 ** (v - mx) for v in vs) / n)
    best = max(out.values())
    out["best"] = best
    out["reading_A2"] = "fails necessary condition" if best > ROWSCAN_BITS else "no reading (A2 has no pass line)"
    out["sample_cap_bits"] = math.log2(n / 3)
    return out


def near_corner(f):
    return (not f["censored"]) and f["max_corner_distance"] <= 3


def secondary_L(frs, beta, rate):
    lb = math.log2(600 * rate)
    vs = [f["v"] if not f["censored"] else lb for f in frs]
    mx = max(vs)
    return mx + math.log2(sum(2 ** (v - mx) for v in vs) / len(vs)) - beta


def main():
    d = sys.argv[1]
    dropped = set()
    if len(sys.argv) > 2 and os.path.exists(sys.argv[2]):
        s0 = json.load(open(sys.argv[2]))
        dropped = {k for k, v in s0["pass"].items() if not v and k.startswith("R")}
    sets = sorted(x for x in os.listdir(d) if os.path.isdir(os.path.join(d, x)) and not x.endswith("-arcs"))
    loaded = {s: load_set(os.path.join(d, s), s, dropped) for s in sets}
    cands = [x for x in CANDIDATES if x not in dropped]
    sources = [x for x in ["R2-glucose", "R0-rows", "R0-spiral"] if x not in dropped]
    rate_real = node_rate(loaded["real"][2]) if "real" in loaded else (None, 0)
    null_ref = {}
    for s in sets:
        if s.startswith("null"):
            null_ref.update(loaded[s][2])
    rate_null = node_rate(null_ref)
    out = {"dropped_refuters": sorted(dropped), "node_rate": {"real": rate_real, "null": rate_null},
           "stage1": {}, "stage2": {}, "secondary_L": {}, "base": {}, "frames": []}
    pooled = {"H": [], "U": []}
    for s in sets:
        puz, frames, ref, counts, counts_h, nf, nh = loaded[s]
        beta = (nf - nh) * LOG2_10
        out["base"][s] = {"log10_NF": nf, "log10_NH": nh, "beta_H_bits": beta}
        rate = rate_real[0] if s.startswith("real") else rate_null[0]
        frs = stage2_frames(s, puz, frames, ref, counts, counts_h, nf, nh, rate, sources)
        fp = os.path.join(d, s, "flags.jsonl")
        fp = fp if os.path.exists(fp) else os.path.join(d, s, s + ".flags.jsonl")
        if os.path.exists(fp):
            fl = {x["id"]: x for x in rows(fp)}
            for f in frs:
                if f["id"] in fl:
                    f["dead_block3"] = fl[f["id"]]["dead_block3"]
                    f["dead_block3x3"] = fl[f["id"]]["dead_block3x3"]
        out["frames"].extend(frs)
        for st in ("H", "U"):
            ids = [f for f in frames if stratum(f) == st]
            if not ids:
                continue
            out["stage1"]["%s/%s" % (s, st)] = stage1(ids, ref, cands)
            sub = [f for f in frs if f["stratum"] == st]
            b = beta if st == "H" else 0.0
            out["stage2"]["%s/%s" % (s, st)] = stage2_stats(sub, b)
            out["secondary_L"]["%s/%s" % (s, st)] = secondary_L(sub, 0.0, rate)
            out["stage2"]["%s/%s" % (s, st)]["floors_A2"] = floors(sub, rate)
            for part, sel in (("near", True), ("far", False)):
                ps = [f for f in sub if near_corner(f) == sel]
                key = "%s/%s/%s" % (s, st, part)
                if ps:
                    out["stage2"][key] = stage2_stats(ps, b)
                    out["stage2"][key]["floors_A2"] = floors(ps, rate)
                    out["stage1"][key] = stage1([f["id"] for f in ps], ref, cands)
            if s.startswith("null"):
                pooled[st].extend(sub)
    for st, frs in pooled.items():
        if frs:
            out["stage2"]["null-pooled/%s" % st] = stage2_stats(frs, float("nan"))
            ids_by_set = {}
            for f in frs:
                ids_by_set.setdefault(f["set"], []).append(f["id"])
            allref = {}
            for s2, ids in ids_by_set.items():
                for i in ids:
                    allref[i] = loaded[s2][2][i]
            out["stage1"]["null-pooled/%s" % st] = stage1(list(allref), allref, cands)
    ap = os.path.join(d, "real-arcs")
    if os.path.isdir(ap):
        full_r2 = {fid: frame_time(rec, "R2-glucose")[0] for fid, rec in loaded["real"][2].items() if stratum(fid) == "H"}
        out["arcs"] = arcs(ap, (loaded["real"][5], loaded["real"][6]), full_r2, rate_real[0])
    # uniform frames: where the cheapest nogood sits (amendment A1 item 1)
    uf = [f for f in out["frames"] if f["set"] == "real" and f["stratum"] == "U"]
    out["uniform_real"] = {"frames": len(uf), "refuted": sum(1 for f in uf if not f["censored"]),
                           "best_nogood_within_2_of_a_corner": sum(1 for f in uf if not f["censored"] and f["max_corner_distance"] <= 2),
                           "median_best_plain_nodes": median([min(v for k, v in f["nodes"].items() if k.startswith("R0") and v is not None) for f in uf]),
                           "beta_H_bits": out["base"]["real"]["beta_H_bits"]}
    real_h = out["stage1"].get("real/H", {})
    out["verdict"] = {
        "stage1_kill": "dead" if not any(v["passes_100x"] for v in real_h.values()) else "alive",
        "stage2_reading": out["stage2"].get("real/H", {}).get("reading"),
        "arcs_A1": out.get("arcs", {}).get("readings_A1"),
        "arcs_A2": {k: v["below_break_even"] for k, v in out.get("arcs", {}).get("A2_nested", {}).items()},
        "stage2_A2": out["stage2"].get("real/H", {}).get("floors_A2", {}).get("reading_A2"),
    }
    json.dump(out, sys.stdout, indent=1, default=lambda x: None)
    print()


if __name__ == "__main__":
    main()
