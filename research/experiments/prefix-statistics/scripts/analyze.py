#!/usr/bin/env python3
"""Prefix statistics: per-board deltas, bootstrap intervals, readings.

    uv run --with numpy python3 -I scripts/analyze.py <results-dir> [--real results/real/real.jsonl] [--json out.json]

Reads every board-*.jsonl under <results-dir> (rows written by `psx measure`).
Definitions: PREREG.md section 6, amendments A1 and A4. Needs numpy.
Rows written before A4 (no A4 fields) are read with the original fields and
counted as valid.
"""
import glob
import json
import math
import os
import sys

import numpy as np

PEAK = [144, 160, 176]
DEPTHS = [32, 64, 96, 128, 144, 160, 176, 192, 208, 224]
SURV = [128, 144, 160]
VALUE_DEPTHS = [96, 128, 144]
CLIP = 10.0
NBOOT = 100000
NBOOT_VALUE = 4000
BOOT_SEED = 20261113
FAMILY = 30  # 10 statistics x 3 peak depths
ALPHA = 0.05
ESS_FLOOR = 30.0  # A4 change 4 (was 10)
SURV_FLOOR = 200
FAM_FLOOR = 200.0
POP_FLOOR = 1000
CENSUS = {3, 5, 8}  # S4, S6, S9: reference over census-feasible prefixes (A4 change 2)
CENSUS_ALL = [3, 4, 5, 8]  # S4, S5, S6, S9 (Reading 3, usable-tilt class)
S6 = 5
LINE = 1.4
EXCL = 0.7


def delta(t, m, s):
    if t is None or m is None or s is None:
        return None
    if isinstance(m, float) and math.isnan(m):
        return None
    if s > 0:
        d = (t - m) / s
    elif t == m:
        d = 0.0
    else:
        d = math.copysign(CLIP, t - m)
    return max(-CLIP, min(CLIP, d))


def load(d):
    rows, done, prof = [], set(), {}
    names = None
    for f in sorted(glob.glob(os.path.join(d, "board-*.jsonl"))):
        for line in open(f):
            r = json.loads(line)
            if r.get("done"):
                done.add(r["board"])
                names = r["names"]
            elif "profile_log10_width" in r:
                prof[(r["board"], r["dir"])] = r["profile_log10_width"]
            else:
                rows.append(r)
    rows = [r for r in rows if r["board"] in done]
    return rows, sorted(done), names, prof


def valid(r):
    """A4 change 1: a row is valid for decisions if its population is live,
    at least 1,000 particles, with family effective size at least 200."""
    if r["extinct"]:
        return False
    if "fam_ess" not in r:
        return True
    return r["pop"] >= POP_FLOOR and r["fam_ess"] is not None and r["fam_ess"] >= FAM_FLOOR


def fields(r, s, kind):
    """(mean, sd) field names for statistic s in reading `kind`."""
    feas = s in CENSUS and "mean_feas" in r
    if kind == "node":
        return ("mean_feas", "sd_feas") if feas else ("mean", "sd")
    if kind == "work":
        return ("work_mean_feas", "work_sd_feas") if feas else ("work_mean", "work_sd")
    if kind == "surv":
        return ("surv_mean", "surv_sd")
    if kind == "surv2":
        return ("surv2_mean", "surv2_sd")
    raise ValueError(kind)


def row_ok(r, kind):
    if kind == "node":
        return True
    if kind == "work":
        e = r.get("work_ess_feas", r.get("work_ess", 0))
        e2 = r.get("work_ess", 0)
        return min(e, e2) >= ESS_FLOOR
    if kind == "surv":
        return r.get("surv_n", 0) >= SURV_FLOOR
    if kind == "surv2":
        return r.get("surv2_n", 0) >= SURV_FLOOR
    return True


def row_delta(r, s, kind, who="true"):
    fm, fs = fields(r, s, kind)
    if fm not in r:
        return None
    return delta(r[who][s], r[fm][s], r[fs][s])


def per_board(rows, nstat, kind, rowfilter, who="true"):
    """board -> depth -> stat -> mean over directions of the clipped delta."""
    out = {}
    for r in rows:
        if not rowfilter(r) or not row_ok(r, kind):
            continue
        b, k = r["board"], r["depth"]
        for s in range(nstat):
            d = row_delta(r, s, kind, who)
            if d is None:
                continue
            out.setdefault(b, {}).setdefault(k, {}).setdefault(s, []).append(d)
    return {b: {k: {s: sum(v) / len(v) for s, v in ks.items()} for k, ks in dk.items()} for b, dk in out.items()}


_IDX = {}


def boot_ci(xs, level):
    """Percentile bootstrap of the mean; the same resampled index sets are
    used for every cell with the same number of boards."""
    n = len(xs)
    if n not in _IDX:
        rng = np.random.default_rng(BOOT_SEED + n)
        _IDX[n] = rng.integers(0, n, size=(NBOOT, n))
    ms = np.asarray(xs)[_IDX[n]].mean(axis=1)
    return float(np.quantile(ms, (1 - level) / 2)), float(np.quantile(ms, (1 + level) / 2))


def table(pb, boards, names, depths, level):
    res = {}
    for k in depths:
        for s, nm in enumerate(names):
            xs = [pb[b][k][s] for b in boards if b in pb and k in pb[b] and s in pb[b][k]]
            if len(xs) < 3:
                continue
            mu = sum(xs) / len(xs)
            sd = math.sqrt(sum((x - mu) ** 2 for x in xs) / (len(xs) - 1))
            lo, hi = boot_ci(xs, level)
            q = np.quantile(xs, [0.25, 0.5, 0.75])
            sign = 1 if mu >= 0 else -1
            res[(nm, k)] = dict(n=len(xs), mean=mu, sd=sd, lo=lo, hi=hi, q1=float(q[0]), med=float(q[1]), q3=float(q[2]),
                                below07=sum(1 for x in xs if sign * x < EXCL) / len(xs),
                                below0=sum(1 for x in xs if sign * x < 0) / len(xs))
    return res


def passes(e):
    return bool(e) and abs(e["mean"]) >= LINE and ((e["mean"] > 0 and e["lo"] > EXCL) or (e["mean"] < 0 and e["hi"] < -EXCL))


def show(t, names, depths, title, ess=None):
    print(f"\n{title}")
    print("stat".ljust(20) + "".join(f"{k:>19}" for k in depths))
    for nm in names:
        line = nm.ljust(20)
        for k in depths:
            e = t.get((nm, k))
            line += (f"{e['mean']:+6.2f}[{e['lo']:+.1f},{e['hi']:+.1f}]" + (f"{e['n']:>3d}" if True else "")).rjust(19) if e else " " * 19
        print(line)
    if ess:
        print("median work ESS (feasible reference) by depth: " + "  ".join(f"{k}:{ess[k]:.0f}" for k in depths if k in ess))


def main():
    args = sys.argv[1:]
    d = args[0]
    real = args[args.index("--real") + 1] if "--real" in args and os.path.exists(args[args.index("--real") + 1]) else None
    jout = args[args.index("--json") + 1] if "--json" in args else None
    rows, boards, names, prof = load(d)
    nstat = len(names)
    print(f"boards complete: {len(boards)}; depth rows: {len(rows)}; level profiles: {len(prof)}")
    lvl_peak = 1 - ALPHA / FAMILY
    lvl_all = 1 - ALPHA / (FAMILY // 3 * len(DEPTHS))
    out = {"boards": boards}

    # ---- A4 change 1: validity and demotion
    print("\nrow validity by depth (live, population >= 1,000, family ESS >= 200):")
    demoted = set()
    vinfo = {}
    for k in DEPTHS:
        rk = [r for r in rows if r["depth"] == k]
        nv = sum(1 for r in rk if valid(r))
        fam = sorted(r["fam_ess"] for r in rk if not r["extinct"] and "fam_ess" in r)
        pops = sorted(r["pop"] for r in rk if not r["extinct"])
        inval = 1 - nv / len(rk) if rk else 1
        if inval > 0.10:
            demoted.add(k)
        vinfo[k] = dict(rows=len(rk), valid=nv, invalid_share=inval, fam_ess_median=fam[len(fam) // 2] if fam else None,
                        pop_median=pops[len(pops) // 2] if pops else None)
        print(f"  {k:3d}: valid {nv}/{len(rk)} (invalid {inval:.1%}); median family ESS "
              f"{vinfo[k]['fam_ess_median'] if fam else 'n/a'}; median population {vinfo[k]['pop_median']}"
              + ("  -> DEMOTED to descriptive" if k in demoted else ""))
    out["validity"] = vinfo
    out["demoted"] = sorted(demoted)
    decide = lambda r: valid(r)  # noqa: E731

    # ---- A4 change 2: census-dead prefixes, counted as exact pruning
    print("\ncensus-dead sampled prefixes (exact pruning, not in any delta): mean share of the m sampled; mean work share")
    dead = {}
    for k in DEPTHS:
        rk = [r for r in rows if r["depth"] == k and not r["extinct"] and "n_dead" in r]
        if rk:
            dn = sum(r["n_dead"] / r["m"] for r in rk) / len(rk)
            dw = sum(r["work_dead_share"] for r in rk) / len(rk)
            dead[k] = dict(node_share=dn, work_share=dw, rows=len(rk), rows_with_any=sum(1 for r in rk if r["n_dead"] > 0))
            print(f"  {k:3d}: node share {dn:.2e}, work share {dw:.2e}, rows with any {dead[k]['rows_with_any']}/{len(rk)}")
    out["census_dead"] = dead

    # ---- deltas
    pb = per_board(rows, nstat, "node", decide)
    pb_all = per_board(rows, nstat, "node", lambda r: not r["extinct"])
    pw = per_board(rows, nstat, "work", decide)
    ps = per_board(rows, nstat, "surv", decide)
    ps2 = per_board(rows, nstat, "surv2", decide)
    tab = table(pb_all, boards, names, DEPTHS, lvl_all)
    tabp = table(pb, boards, names, PEAK, lvl_peak)
    tabw = table(pw, boards, names, DEPTHS, lvl_all)
    tabwp = table(pw, boards, names, PEAK, lvl_peak)
    tabs = table(ps, boards, names, SURV, lvl_peak)
    tabs2 = table(ps2, boards, names, SURV, lvl_peak)
    ess = {}
    for k in DEPTHS:
        e = sorted(r.get("work_ess_feas", r.get("work_ess", 0)) for r in rows if r["depth"] == k and not r["extinct"])
        if e:
            ess[k] = e[len(e) // 2]
    show(tab, names, DEPTHS, f"node delta, all live rows, mean over boards [{lvl_all:.4f} interval] n boards")
    show(tabp, names, PEAK, f"node delta, valid rows, peak [{lvl_peak:.5f}]")
    show(tabw, names, DEPTHS, f"work delta (work ESS >= {ESS_FLOOR:g}, valid rows) [{lvl_all:.4f}]", ess)
    show(tabs, names, SURV, f"Reading 3: delta among prefixes alive one row ahead (>= {SURV_FLOOR} survivors) [{lvl_peak:.5f}]")
    show(tabs2, names, SURV, f"Reading 3, two rows ahead (>= {SURV_FLOOR} survivors) [{lvl_peak:.5f}]")
    sv = {}
    for k in SURV:
        rk = [r for r in rows if r["depth"] == k and "surv_scanned" in r]
        if rk:
            sv[k] = dict(surv_frac_mean=sum(r["surv_frac"] for r in rk) / len(rk),
                         rows_measured=sum(1 for r in rk if r["surv_n"] >= SURV_FLOOR),
                         rows_r2_measured=sum(1 for r in rk if r["surv2_n"] >= SURV_FLOOR),
                         r2_pass_share=sum(r["surv2_pass"] / max(r["surv2_tested"], 1) for r in rk) / len(rk),
                         r2_capped=sum(r["surv2_capped"] for r in rk), rows=len(rk))
            print(f"  {k}: surviving fraction one row ahead {sv[k]['surv_frac_mean']:.3e}; rows measured {sv[k]['rows_measured']}/{len(rk)};"
                  f" two rows: pass share {sv[k]['r2_pass_share']:.3f}, rows measured {sv[k]['rows_r2_measured']}, capped {sv[k]['r2_capped']}")
    out["survivors"] = sv
    print("\nNote (A4 change 4): at 160 and 176 the work weights are close to uniform because the tree no longer grows,"
          " so Reading 2 cannot differ from Reading 1 there; its informative depths are 128 and 144.")

    # ---- readings
    out["reading1"], out["reading2"], out["reading3"], out["usable_census"] = {}, {}, {}, {}
    print(f"\nREADING 1 (lead's line): |mean| >= {LINE}, {lvl_peak:.5f} interval excludes {EXCL}; demoted depths excluded")
    for nm in names[:-1]:
        hits = [k for k in PEAK if k not in demoted and passes(tabp.get((nm, k)))]
        out["reading1"][nm] = hits
        print(f"  {nm:20s} {'SIGNAL at ' + str(hits) if hits else 'no'}")
    print("READING 2 (work-weighted, same line):")
    for nm in names[:-1]:
        hits = [k for k in PEAK if k not in demoted and passes(tabwp.get((nm, k)))]
        out["reading2"][nm] = hits
        print(f"  {nm:20s} {'SIGNAL at ' + str(hits) if hits else 'no'}")
    print("READING 3 (survivors one row ahead, same line, depths 128, 144, 160):")
    for nm in names[:-1]:
        hits = [k for k in SURV if k not in demoted and passes(tabs.get((nm, k)))]
        out["reading3"][nm] = hits
        print(f"  {nm:20s} {'SIGNAL at ' + str(hits) if hits else 'no'}")
    print("USABLE CENSUS TILT (A4 change 3: Readings 1, 2 and 3 at the same depth; S4, S5, S6, S9):")
    for s in CENSUS_ALL:
        nm = names[s]
        both = sorted(set(out["reading1"][nm]) & set(out["reading2"][nm]) & set(out["reading3"][nm]))
        out["usable_census"][nm] = both
        print(f"  {nm:20s} {both if both else 'no'}")
    print("  (a pass of S6 and S9 together is one finding; S6 is the primary census statistic)")

    # ---- controls
    print("\nPOSITIVE CONTROL (S10, S8 node delta at 176, all live rows, interval above 0.7):")
    tab176 = table(pb_all, boards, names, [176], lvl_peak)
    pos_ok = False
    for nm in ("s10_row_lookahead", "s8_block4_fills"):
        e = tab176.get((nm, 176))
        ok = bool(e) and e["mean"] >= LINE and e["lo"] > EXCL
        pos_ok |= ok
        print(f"  {nm:20s} " + (f"{e['mean']:+.2f} [{e['lo']:+.2f}, {e['hi']:+.2f}] {'pass' if ok else 'fail'}" if e else "missing"))
    print("  positive control: " + ("PASS" if pos_ok else "FAIL (pipeline broken)"))
    tabpa = table(pb_all, boards, names, PEAK, lvl_peak)
    neg1 = True
    print("NEGATIVE CONTROL 1 (H0, peak, all live rows, interval contains 0):")
    for k in PEAK:
        e = tabpa.get(("h0_hash", k))
        ok = bool(e) and e["lo"] <= 0 <= e["hi"]
        neg1 &= ok
        print(f"  {k}: " + (f"{e['mean']:+.2f} [{e['lo']:+.2f}, {e['hi']:+.2f}] {'pass' if ok else 'FAIL'}" if e else "missing"))
    pr = {}
    for r in rows:
        if r["extinct"] or r["depth"] not in PEAK:
            continue
        for s in range(nstat):
            b_, e_ = r["decoy_below"][s], r["decoy_equal"][s]
            if b_ is None or e_ is None:
                continue
            pr.setdefault(r["board"], {}).setdefault(r["depth"], {}).setdefault(s, []).append(b_ + 0.5 * e_ - 0.5)
    pr = {b: {k: {s: sum(v) / len(v) for s, v in ks.items()} for k, ks in dk.items()} for b, dk in pr.items()}
    tabdr = table(pr, boards, names, PEAK, lvl_peak)
    fails = [(nm, k, round(tabdr[(nm, k)]["mean"], 3)) for nm in names[:-1] for k in PEAK
             if (nm, k) in tabdr and not (tabdr[(nm, k)]["lo"] <= 0 <= tabdr[(nm, k)]["hi"])]
    neg2 = not fails
    print("NEGATIVE CONTROL 2 (A1 decoy rank, 10 statistics x peak): " + ("PASS" if neg2 else f"FAIL {fails}"))
    out.update(positive_control=pos_ok, negative_hash=neg1, negative_decoy=neg2)

    # ---- A4 change 6: per-board distribution, census correlation
    print("\nper-board distribution of the node delta (valid rows), peak: median [q1, q3]; share with oriented delta < 0.7, < 0")
    for nm in names:
        print(f"  {nm:20s} " + "  ".join(
            f"{k}: {tabp[(nm, k)]['med']:+.2f} [{tabp[(nm, k)]['q1']:+.2f},{tabp[(nm, k)]['q3']:+.2f}] {tabp[(nm, k)]['below07']:.2f} {tabp[(nm, k)]['below0']:.2f}"
            for k in PEAK if (nm, k) in tabp))
    print("correlation with S6 of the true prefix's node delta, per row (valid rows), by depth:")
    cc = {}
    for s in (3, 8):
        line = []
        for k in DEPTHS:
            xs, ys = [], []
            for r in rows:
                if r["depth"] != k or not valid(r):
                    continue
                a, b_ = row_delta(r, S6, "node"), row_delta(r, s, "node")
                if a is not None and b_ is not None:
                    xs.append(a)
                    ys.append(b_)
            if len(xs) > 5 and np.std(xs) > 0 and np.std(ys) > 0:
                c = float(np.corrcoef(xs, ys)[0, 1])
                cc[f"{names[s]}@{k}"] = c
                line.append(f"{k}:{c:+.2f}")
        print(f"  {names[s]:20s} " + " ".join(line))
    out["corr_with_s6"] = cc

    # ---- A4 change 7: direction correlation by pair type and depth
    print("\ndirection correlation of the true prefix's node delta (r) and of the kept-tail indicator at f = 0.1 (phi),"
          " opposite (0-2, 1-3) and adjacent pairs, live rows:")
    dc = {}
    for s in [S6, 8, 3, 9, 6]:
        nm = names[s]
        for k in [96, 128, 144, 160, 176]:
            by = {}
            for r in rows:
                if r["depth"] != k or r["extinct"]:
                    continue
                dd = row_delta(r, s, "node")
                if s == S6 and "s6_work_pct_deadlow" in r:
                    q = r["s6_work_pct_deadlow"]
                else:
                    fb, fe = ("work_true_below_feas", "work_true_equal_feas") if (s in CENSUS and "work_true_below_feas" in r) else ("work_true_below", "work_true_equal")
                    q = r[fb][s] + 0.5 * r[fe][s]
                    sign = 1 if tab.get((nm, k), {"mean": 1})["mean"] >= 0 else -1
                    q = q if sign > 0 else 1 - q
                by.setdefault(r["board"], {})[r["dir"]] = (dd, 1.0 if q >= 0.9 else 0.0)
            res = {}
            for typ, pairs in (("opposite", [(0, 2), (1, 3)]), ("adjacent", [(0, 1), (1, 2), (2, 3), (3, 0)])):
                xs, ys, ix, iy = [], [], [], []
                for b, dd in by.items():
                    for i, j in pairs:
                        if i in dd and j in dd and dd[i][0] is not None and dd[j][0] is not None:
                            xs.append(dd[i][0]); ys.append(dd[j][0]); ix.append(dd[i][1]); iy.append(dd[j][1])
                r_ = float(np.corrcoef(xs, ys)[0, 1]) if len(xs) > 5 and np.std(xs) > 0 and np.std(ys) > 0 else float("nan")
                phi = float(np.corrcoef(ix, iy)[0, 1]) if len(ix) > 5 and np.std(ix) > 0 and np.std(iy) > 0 else float("nan")
                res[typ] = dict(r=r_, phi=phi, pairs=len(xs))
            dc[f"{nm}@{k}"] = res
            print(f"  {nm:20s} {k}: opposite r {res['opposite']['r']:+.2f} phi {res['opposite']['phi']:+.2f} ({res['opposite']['pairs']});"
                  f" adjacent r {res['adjacent']['r']:+.2f} phi {res['adjacent']['phi']:+.2f} ({res['adjacent']['pairs']})")
    out["dir_corr"] = dc

    # ---- A4 change 5: value reading for S6
    print("\nVALUE READING (S6, A4 change 5): capped gain G in bits, f chosen as the smallest kept work fraction with p >= target;"
          " bootstrap over boards, 95%")
    val = value_reading(rows, prof, boards)
    out["value"] = val
    usable_value = False
    for k in VALUE_DEPTHS:
        for K in (1, 2, 4):
            for tgt in (0.5, 0.8):
                v = val.get(f"{k}|{K}|{tgt}")
                if not v:
                    continue
                if tgt == 0.5 and v["lo"] is not None and v["lo"] > 1.0:
                    usable_value = True
                print(f"  depth {k} K={K} p>={tgt}: G {v['G']:+.2f} bits [{v['lo']:+.2f}, {v['hi']:+.2f}]  f={v['f']:.4f} p={v['p']:.2f}"
                      f"  cap {v['cap']:.2f}  boards {v['n']}  median work ESS {v['ess']:.0f}")
    print("  'usable' (lower end of G above 1 bit at some depth): " + ("YES" if usable_value else "no"))
    out["usable_value"] = usable_value

    # ---- empirical gain (PREREG section 6)
    print("\nempirical single-presentation gain p/f (work-weighted, feasible reference for census statistics), all live rows:")
    gains = {}
    for s, nm in enumerate(names):
        for k in [96, 128] + PEAK:
            e = tab.get((nm, k))
            if not e:
                continue
            sign = 1 if e["mean"] >= 0 else -1
            pct = []
            for r in rows:
                if r["extinct"] or r["depth"] != k:
                    continue
                fb, fe = ("work_true_below_feas", "work_true_equal_feas") if (s in CENSUS and "work_true_below_feas" in r) else ("work_true_below", "work_true_equal")
                q = r[fb][s] + 0.5 * r[fe][s]
                pct.append(q if sign > 0 else 1 - q)
            cells = []
            for f in (0.5, 0.2, 0.1, 0.05, 0.01):
                p = sum(1 for q in pct if q >= 1 - f) / len(pct)
                cells.append(f"f={f}:{p / f:5.2f}")
                gains[f"{nm}@{k}@{f}"] = dict(p=p, n=len(pct), gain=p / f)
            if s in CENSUS_ALL or s in (6, 9):
                print(f"  {nm:20s} {k} n={len(pct):3d} " + " ".join(cells))
    out["gains"] = gains

    # ---- cost
    print("\nmean ns per evaluation by depth:")
    cost = {}
    for r in rows:
        if r["extinct"]:
            continue
        for s in range(nstat):
            cost.setdefault((s, r["depth"]), []).append(r["ns_per_eval"][s])
    out["cost_ns"] = {}
    for s, nm in enumerate(names):
        cs = {k: sum(cost[(s, k)]) / len(cost[(s, k)]) for k in DEPTHS if (s, k) in cost}
        out["cost_ns"][nm] = cs
        print(f"  {nm:20s} " + " ".join(f"{k}:{v:.0f}" for k, v in cs.items()))

    # ---- null in reverse
    if real:
        print("\nnull in reverse: official pieces' population mean against the generated boards (z over boards, |z| > 3 flagged):")
        gen = {}
        for r in rows:
            if r["extinct"]:
                continue
            for s in range(nstat):
                gen.setdefault((s, r["depth"], r["dir"]), []).append(r["mean"][s])
        zt = {}
        for line in open(real):
            q = json.loads(line)
            if q["extinct"]:
                continue
            zs = []
            for s in range(nstat):
                g = gen.get((s, q["depth"], q["dir"]), [])
                if len(g) < 3:
                    zs.append(None)
                    continue
                mu = sum(g) / len(g)
                sd = math.sqrt(sum((x - mu) ** 2 for x in g) / (len(g) - 1))
                zs.append((q["mean"][s] - mu) / sd if sd > 0 else None)
            zt[f"{q['dir']}@{q['depth']}"] = zs
            print(f"  dir {q['dir']} depth {q['depth']:3d}: " + " ".join("   .  " if z is None else (f"{z:+5.1f}" + ("!" if abs(z) > 3 else " ")) for z in zs))
        out["null_reverse_z"] = zt

    out["tables"] = {kind: {f"{nm}@{k}": v for (nm, k), v in t.items()} for kind, t in
                     (("node_all", tab), ("node_peak_valid", tabp), ("work", tabw), ("work_peak", tabwp), ("surv", tabs),
                      ("surv2", tabs2), ("decoy_rank", tabdr))}
    if jout:
        json.dump(out, open(jout, "w"), indent=1)


def value_reading(rows, prof, boards):
    """Capped gain of pruning once at depth k by S6 (A4 change 5).

    Per board, directions sorted by total tree size T = A + B (A = nodes at
    levels <= k, B = below, from the row's level profile). K = 1, 2, 4
    cheapest directions are run, each pruned at k to the kept work fraction f
    (true prefix kept if its S6 work percentile, census-dead prefixes lowest,
    is >= 1 - f). Success on a board = some run keeps the true prefix.
    G(f) = log2(p) + mean over boards of log2(T_best / sum over runs of
    (A + f B)). f = smallest grid value with p >= target. Bootstrap over
    boards (f re-chosen in each resample).
    """
    grid = np.unique(np.concatenate([np.logspace(-4, 0, 400), [1.0]]))
    res = {}
    for k in VALUE_DEPTHS:
        per = {}
        ess = []
        for r in rows:
            if r["depth"] != k or r["extinct"] or "s6_work_pct_deadlow" not in r:
                continue
            lw = prof.get((r["board"], r["dir"]))
            if lw is None:
                continue
            w = [10 ** x if x is not None else 0.0 for x in lw]
            A, B = sum(w[: k + 1]), sum(w[k + 1:])
            per.setdefault(r["board"], []).append((A + B, A, B, r["s6_work_pct_deadlow"]))
            ess.append(r.get("work_ess", 0))
        bl = [b for b in boards if b in per and len(per[b]) == 4]
        if len(bl) < 5:
            continue
        for K in (1, 2, 4):
            succ = np.zeros((len(bl), len(grid)))
            lcost = np.zeros((len(bl), len(grid)))
            cap = []
            for i, b in enumerate(bl):
                ds = sorted(per[b])[:K]
                Tb = ds[0][0]
                cost = sum(A + grid * B for _, A, B, _ in ds)
                lcost[i] = np.log2(Tb / cost)
                succ[i] = np.max([(q >= 1 - grid).astype(float) for *_, q in ds], axis=0)
                cap.append(math.log2(Tb / sum(A for _, A, _, _ in ds)))

            def g_of(idx, tgt):
                p = succ[idx].mean(axis=0)
                ok = np.nonzero(p >= tgt)[0]
                if len(ok) == 0:
                    return None, None, None
                j = ok[0]
                return float(np.log2(p[j]) + lcost[idx, j].mean()), float(grid[j]), float(p[j])

            rng = np.random.default_rng(BOOT_SEED + k + K)
            for tgt in (0.5, 0.8):
                G, f, p = g_of(np.arange(len(bl)), tgt)
                if G is None:
                    continue
                bs = []
                for _ in range(NBOOT_VALUE):
                    gb = g_of(rng.integers(0, len(bl), len(bl)), tgt)[0]
                    if gb is not None:
                        bs.append(gb)
                lo, hi = (float(np.quantile(bs, 0.025)), float(np.quantile(bs, 0.975))) if bs else (None, None)
                ess_s = sorted(ess)
                res[f"{k}|{K}|{tgt}"] = dict(G=G, lo=lo, hi=hi, f=f, p=p, n=len(bl), cap=float(np.mean(cap)),
                                            ess=ess_s[len(ess_s) // 2])
    return res


if __name__ == "__main__":
    main()
