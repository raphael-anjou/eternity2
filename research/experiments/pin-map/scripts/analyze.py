"""pin-map analysis (see ../PREREG.md). Reads results/*.jsonl, writes
results/tables/*.csv, results/summary.json and results/figures/*.{svg,png}.
Run: uv run --with matplotlib python -I scripts/analyze.py"""
import csv
import json
import math
import os
import statistics as st
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import drive as D  # noqa: E402

import matplotlib  # noqa: E402

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

EXP = D.EXP
RES = D.RES
TAB = os.path.join(RES, "tables")
FIG = os.path.join(RES, "figures")
os.makedirs(TAB, exist_ok=True)
os.makedirs(FIG, exist_ok=True)
L2 = math.log2
SUM = {}


def cls(n, q):
    r, c = divmod(q, n)
    on_r, on_c = r in (0, n - 1), c in (0, n - 1)
    if on_r and on_c:
        return "corner"
    if on_r or on_c:
        return "frame"
    return "interior"


def pct(v, p):
    v = sorted(v)
    if not v:
        return float("nan")
    x = p * (len(v) - 1)
    lo = math.floor(x)
    hi = min(lo + 1, len(v) - 1)
    return v[lo] + (v[hi] - v[lo]) * (x - lo)


def pearson(a, b):
    ma, mb = st.mean(a), st.mean(b)
    num = sum((x - ma) * (y - mb) for x, y in zip(a, b))
    den = math.sqrt(sum((x - ma) ** 2 for x in a) * sum((y - mb) ** 2 for y in b))
    return num / den


def save(fig, name):
    fig.savefig(os.path.join(FIG, name + ".svg"), bbox_inches="tight")
    fig.savefig(os.path.join(FIG, name + ".png"), dpi=130, bbox_inches="tight")
    plt.close(fig)


def write_csv(name, header, rowsv):
    with open(os.path.join(TAB, name), "w", newline="") as fh:
        w = csv.writer(fh)
        w.writerow(header)
        w.writerows(rowsv)


# ---------- item 1: single-tile maps ----------

def singles(stage, setname, order):
    s = D.SETS[setname]
    n = s["n"]
    per = {}
    first = {}
    base = {}
    for r in D.rows(stage):
        _, seed, o = r["label"].split(":")
        if o != order:
            continue
        assert r["complete"]
        seed = int(seed)
        b = r["base_nodes"]
        base[seed] = b
        per[seed] = {q: L2(b) - L2(x) for q, x in zip(r["cand"], r["nodes"])}
        first[seed] = {q: (L2(f) if f else None) for q, f in zip(r["cand"], r["first"])}
    if not per:
        return None
    cells = sorted(next(iter(per.values())).keys())
    mean = {q: st.mean(per[b][q] for b in per) for q in cells}
    sd = {q: (st.stdev(per[b][q] for b in per) if len(per) > 1 else 0.0) for q in cells}
    fmean = {q: st.mean(first[b][q] for b in per if first[b][q] is not None) for q in cells}
    rank = {q: i + 1 for i, q in enumerate(sorted(cells, key=lambda q: -mean[q]))}
    write_csv(f"singles_{setname}_{order}.csv", ["cell", "row", "col", "class", "mean_bits", "sd_bits", "rank",
                                                  "mean_log2_first_solution"],
              [[q, q // n, q % n, cls(n, q), f"{mean[q]:.4f}", f"{sd[q]:.4f}", rank[q], f"{fmean[q]:.4f}"]
               for q in cells])
    byc = {}
    for c in ("corner", "frame", "interior"):
        v = [mean[q] for q in cells if cls(n, q) == c]
        byc[c] = st.mean(v) if v else None
    rows_m = [st.mean(mean[q] for q in cells if q // n == r) for r in range(n) if any(q // n == r for q in cells)]
    base_bits = [L2(base[b]) for b in sorted(base)]
    out = dict(boards=len(per), base_log2_median=st.median(base_bits), base_log2_min=min(base_bits),
               base_log2_max=max(base_bits), class_mean_bits=byc, row_mean_bits=rows_m,
               top5=[(q, round(mean[q], 3)) for q in sorted(cells, key=lambda q: -mean[q])[:5]],
               bottom5=[(q, round(mean[q], 3)) for q in sorted(cells, key=lambda q: mean[q])[:5]],
               mean_all=st.mean(mean.values()), median_sd=st.median(sd.values()),
               first_solution_log2_base_median=None)
    return dict(n=n, mean=mean, sd=sd, rank=rank, summary=out, per=per, base=base, clues=s["clues"])


def heat(ax, n, vals, title, clues=(), fmt="{:.1f}", cmap="viridis", vmin=None, vmax=None):
    import numpy as np  # noqa: F401  (matplotlib dependency)
    grid = [[float("nan")] * n for _ in range(n)]
    for q, v in vals.items():
        grid[q // n][q % n] = v
    im = ax.imshow(grid, cmap=cmap, vmin=vmin, vmax=vmax)
    fs = 7 if n <= 8 else 5.5
    finite = [v for v in vals.values() if v == v]
    mid = (min(finite) + max(finite)) / 2 if finite else 0
    for q, v in vals.items():
        ax.text(q % n, q // n, fmt.format(v), ha="center", va="center", fontsize=fs,
                color="white" if v < mid else "black")
    for q in clues:
        ax.text(q % n, q // n, "clue", ha="center", va="center", fontsize=fs, color="red")
    ax.set_title(title, fontsize=9)
    ax.set_xticks(range(n))
    ax.set_yticks(range(n))
    ax.tick_params(labelsize=6)
    return im


S1 = {}
for stage, setname, order in [("s8", "M8", "fwd"), ("s8", "M8", "rev"), ("s8c", "M8C", "fwd"),
                              ("n8", "N8", "fwd"), ("s10", "M10", "fwd"), ("s10", "M10", "rev")]:
    r = singles(stage, setname, order)
    if r:
        S1[f"{setname}-{order}"] = r
SUM["singles"] = {k: v["summary"] for k, v in S1.items()}


def rot180(n, m):
    return {n * n - 1 - q: v for q, v in m.items()}


for key in ("M8", "M10"):
    if f"{key}-fwd" in S1 and f"{key}-rev" in S1:
        n = S1[f"{key}-fwd"]["n"]
        f = S1[f"{key}-fwd"]["mean"]
        rv = S1[f"{key}-rev"]["mean"]
        rr = rot180(n, rv)
        cells = sorted(f)
        SUM["singles"][f"{key}-direction"] = dict(
            r_fwd_vs_rev_rotated180=pearson([f[q] for q in cells], [rr[q] for q in cells]),
            r_fwd_vs_rev_unrotated=pearson([f[q] for q in cells], [rv[q] for q in cells]))
if "M8-fwd" in S1 and "N8-fwd" in S1:
    a, b = S1["M8-fwd"]["mean"], S1["N8-fwd"]["mean"]
    SUM["singles"]["N8-vs-M8-r"] = pearson([a[q] for q in sorted(a)], [b[q] for q in sorted(a)])
if "M8-fwd" in S1 and "M8C-fwd" in S1:
    a, b = S1["M8-fwd"]["mean"], S1["M8C-fwd"]["mean"]
    cc = sorted(b)
    SUM["singles"]["M8C-vs-M8-r"] = pearson([a[q] for q in cc], [b[q] for q in cc])

panels = [k for k in ("M8-fwd", "M8-rev", "M8C-fwd", "N8-fwd", "M10-fwd", "M10-rev") if k in S1]
if panels:
    fig, axs = plt.subplots(2, 3, figsize=(15, 10))
    names = {"M8-fwd": "8x8 ci=11, scan from top", "M8-rev": "8x8 ci=11, scan from bottom",
             "M8C-fwd": "8x8 ci=11 with 5 clues, scan from top", "N8-fwd": "8x8 ci=10, scan from top",
             "M10-fwd": "10x10 ci=18, scan from top", "M10-rev": "10x10 ci=18, scan from bottom"}
    for ax, k in zip(axs.flat, panels):
        d = S1[k]
        im = heat(ax, d["n"], d["mean"], f"{names[k]}\nmean bits per known tile ({d['summary']['boards']} boards)",
                  clues=d["clues"])
        fig.colorbar(im, ax=ax, fraction=0.046)
    for ax in list(axs.flat)[len(panels):]:
        ax.axis("off")
    fig.suptitle("F1. Worth of knowing one tile, by cell: log2(tree) - log2(tree with that tile fixed), "
                 "exact row-scan trees", fontsize=11)
    save(fig, "F1-single-tile-map")
    fig, axs = plt.subplots(1, 3, figsize=(15, 5))
    for ax, k in zip(axs, ["M8-fwd", "M8C-fwd", "M10-fwd"]):
        if k in S1:
            d = S1[k]
            heat(ax, d["n"], {q: float(v) for q, v in d["rank"].items()}, f"{names[k]}\nrank (1 = most worth)",
                 clues=d["clues"], fmt="{:.0f}", cmap="viridis_r")
    fig.suptitle("F1b. Single-tile worth as ranks", fontsize=11)
    save(fig, "F1b-single-tile-rank")

# ---------- item 2: pairs ----------

pr = D.rows("pairs8")
if pr and "M8-fwd" in S1:
    per = S1["M8-fwd"]["per"]
    base = S1["M8-fwd"]["base"]
    groups = {}
    allrows = []
    for r in pr:
        assert r["complete"]
        _, seed, p = r["label"].split(":")
        seed, p = int(seed), int(p)
        for q, x in zip(r["cand"], r["nodes"]):
            wpq = L2(base[seed]) - L2(x)
            ws = per[seed][p] + per[seed][q]
            dist = abs(p // 8 - q // 8) + abs(p % 8 - q % 8)
            same = p // 8 == q // 8
            allrows.append([seed, p, q, dist, int(same), f"{wpq:.4f}", f"{per[seed][p]:.4f}", f"{per[seed][q]:.4f}"])
            groups.setdefault((dist, same), []).append((wpq, ws))
    write_csv("pairs_M8.csv", ["seed", "p", "q", "manhattan", "same_row", "pair_bits", "single_p_bits",
                               "single_q_bits"], allrows)
    gt = []
    for (dist, same), v in sorted(groups.items()):
        ratios = [a / b for a, b in v if b > 0.05]
        gt.append([dist, int(same), len(v), f"{st.median(ratios):.4f}" if ratios else "",
                   f"{sum(a for a, _ in v) / sum(b for _, b in v):.4f}"])
    write_csv("pairs_additivity_M8.csv", ["manhattan", "same_row", "pairs", "median_ratio", "aggregate_ratio"], gt)
    allr = [a / b for v in groups.values() for a, b in v if b > 0.05]
    agg = lambda f: sum(a for k, v in groups.items() if f(k) for a, _ in v) / sum(  # noqa: E731
        b for k, v in groups.items() if f(k) for _, b in v)
    SUM["pairs"] = dict(pairs=len(allrows), median_ratio_all=st.median(allr),
                        aggregate_same_row=agg(lambda k: k[1]), aggregate_other_rows=agg(lambda k: not k[1]),
                        aggregate_adjacent=agg(lambda k: k[0] == 1),
                        aggregate_dist_ge4=agg(lambda k: k[0] >= 4),
                        table=gt)
    fig, ax = plt.subplots(figsize=(7, 4.5))
    for same, lab, col in [(True, "same row", "#c0392b"), (False, "different rows", "#2471a3")]:
        xs = sorted({d for d, s in groups if s == same})
        ys = [sum(a for a, _ in groups[(d, same)]) / sum(b for _, b in groups[(d, same)]) for d in xs]
        ax.plot(xs, ys, "o-", color=col, label=lab)
    ax.axhline(1, color="grey", lw=0.8, ls="--")
    ax.set_xlabel("Manhattan distance between the two known tiles")
    ax.set_ylabel("worth(pair) / (worth(p) + worth(q))\n(sum over pairs, 5 boards)")
    ax.set_title("F4. Additivity of two known tiles, 8x8 ci=11, exact row-scan trees", fontsize=10)
    ax.legend()
    save(fig, "F4-pair-additivity")

# ---------- item 3: policies ----------

POL = ["a-greedy", "b-scan", "c-lattice", "d-cluster", "e-frame", "f-interior", "g-random"]
PCOL = {"a-greedy": "#000000", "b-scan": "#2471a3", "c-lattice": "#27ae60", "d-cluster": "#c0392b",
        "e-frame": "#8e44ad", "f-interior": "#d68910", "g-random": "#7f8c8d"}
PLAB = {"a-greedy": "(a) measured greedy", "b-scan": "(b) spread along scan", "c-lattice": "(c) 2D lattice",
        "d-cluster": "(d) clustered, best rows", "e-frame": "(e) frame only", "f-interior": "(f) interior only",
        "g-random": "(g) random (20 draws)"}


def policy_curves(setname, gstage, pstage, sstage):
    """rem[policy][seed][k] = remaining log2 nodes (random: mean over draws);
    first[policy][seed][k] = log2 nodes to first solution."""
    s = D.SETS[setname]
    K = D.KMAX[setname]
    base = {int(r["label"].split(":")[1]): r["base_nodes"] for r in D.rows(sstage) if r["label"].endswith(":fwd")}
    rem = {p: {} for p in POL}
    fst = {p: {} for p in POL}
    for r in D.rows(gstage):
        _, seed, k = r["label"].split(":")
        seed, k = int(seed), int(k[1:])
        i = min(range(len(r["cand"])), key=lambda i: (r["nodes"][i], r["cand"][i]))
        rem["a-greedy"].setdefault(seed, {})[k] = L2(r["nodes"][i])
        fst["a-greedy"].setdefault(seed, {})[k] = L2(r["first"][i])
    rnd = {}
    rndf = {}
    for r in D.rows(pstage):
        assert r["complete"]
        _, seed, pol, k = r["label"].split(":")
        seed, k = int(seed), int(k[1:])
        if pol.startswith("g-random"):
            rnd.setdefault((seed, k), []).append(L2(r["nodes"]))
            if r.get("first"):
                rndf.setdefault((seed, k), []).append(L2(r["first"]))
        else:
            rem[pol].setdefault(seed, {})[k] = L2(r["nodes"])
            if r.get("first"):
                fst[pol].setdefault(seed, {})[k] = L2(r["first"])
    # A1: keep a board for policies (b)..(g) only when every unit of it is in
    full = set(range(1, K + 1))
    for seed in list(base):
        ok = all(full <= set(rem[p].get(seed, {})) for p in D.DET)
        ok = ok and all(len(rnd.get((seed, k), [])) == D.DRAWS for k in full)
        if not ok:
            for p in D.DET:
                rem[p].pop(seed, None)
                fst[p].pop(seed, None)
            for k in full:
                rnd.pop((seed, k), None)
                rndf.pop((seed, k), None)
    for (seed, k), v in rnd.items():
        rem["g-random"].setdefault(seed, {})[k] = st.mean(v)
    for (seed, k), v in rndf.items():
        fst["g-random"].setdefault(seed, {})[k] = st.mean(v)
    for p in POL:
        for seed in list(rem[p]):
            rem[p][seed][0] = L2(base[seed])
    return dict(rem=rem, fst=fst, base=base, K=K, seeds=s["seeds"])


def thresholds(pc):
    out = []
    for p in POL:
        row = [p]
        for name in ("1e3", "1e6", "under1e4"):
            ks = []
            for seed, m in pc["rem"][p].items():
                b = m[0]
                hit = None
                for k in range(1, pc["K"] + 1):
                    if k not in m:
                        break
                    v = m[k]
                    ok = (b - v >= L2(1e3)) if name == "1e3" else (b - v >= L2(1e6)) if name == "1e6" else (v < L2(1e4))
                    if ok:
                        hit = k
                        break
                ks.append(hit if hit is not None else math.inf)
            if not ks:
                row += ["", ""]
                continue
            med = st.median(sorted(ks))
            never = sum(1 for x in ks if x == math.inf)
            row += [("never" if med == math.inf else f"{med:g}"), never]
        out.append(row)
    return out


PCS = {}
for setname, gs, ps, ss in [("M8", "g8", "p8", "s8"), ("M8C", "g8c", "p8c", "s8c"), ("M10", "g10", "p10", "s10")]:
    if D.rows(gs) or D.rows(ps):
        PCS[setname] = policy_curves(setname, gs, ps, ss)

if PCS:
    fig, axs = plt.subplots(1, len(PCS), figsize=(6.5 * len(PCS), 5.2), squeeze=False)
    for ax, (setname, pc) in zip(axs[0], PCS.items()):
        table = []
        for p in POL:
            seeds = [sd for sd in pc["rem"][p] if len(pc["rem"][p][sd]) > 1]
            if not seeds:
                continue
            ks = sorted(set.intersection(*[set(pc["rem"][p][sd]) for sd in seeds]))
            m = [st.mean(pc["rem"][p][sd][k] for sd in seeds) for k in ks]
            lo = [pct([pc["rem"][p][sd][k] for sd in seeds], 0.1) for k in ks]
            hi = [pct([pc["rem"][p][sd][k] for sd in seeds], 0.9) for k in ks]
            ax.plot(ks, m, "-", color=PCOL[p], label=f"{PLAB[p]} ({len(seeds)} boards)", lw=2 if p == "a-greedy" else 1.3)
            ax.fill_between(ks, lo, hi, color=PCOL[p], alpha=0.12)
            for k, v, a, b in zip(ks, m, lo, hi):
                table.append([p, k, f"{v:.4f}", f"{a:.4f}", f"{b:.4f}", len(seeds)])
        ax.axhline(L2(1e4), color="grey", ls=":", lw=0.8)
        ax.text(0.2, L2(1e4) + 0.3, "1e4 nodes", fontsize=7, color="grey")
        n = D.SETS[setname]["n"]
        lbl = {"M8": "8x8 ci=11, no clues", "M8C": "8x8 ci=11, 5 clues fixed", "M10": "10x10 ci=18, no clues"}[setname]
        ax.set_title(lbl, fontsize=10)
        ax.set_xlabel("k = known tiles placed by the policy")
        ax.set_ylabel("remaining exact row-scan tree, log2 nodes")
        ax.legend(fontsize=7)
        write_csv(f"policies_{setname}.csv", ["policy", "k", "mean_log2_nodes", "p10", "p90", "boards"], table)
        th = thresholds(pc)
        write_csv(f"thresholds_{setname}.csv", ["policy", "k_for_1e3_cut_median", "boards_never_1e3",
                                                "k_for_1e6_cut_median", "boards_never_1e6",
                                                "k_under_1e4_median", "boards_never_under_1e4"], th)
        SUM.setdefault("policies", {})[setname] = dict(thresholds=th)
        # remaining bits at fixed k and nodes-to-first at fixed k
        atk = {}
        for p in POL:
            for k in (1, 2, 4, 8, 12, 16, 24):
                v = [pc["rem"][p][sd][k] for sd in pc["rem"][p] if k in pc["rem"][p][sd]]
                f = [pc["fst"][p][sd][k] for sd in pc["fst"][p] if k in pc["fst"][p][sd]]
                if v:
                    atk.setdefault(p, {})[k] = dict(mean_log2_nodes=round(st.mean(v), 3),
                                                    mean_log2_first=round(st.mean(f), 3) if f else None, boards=len(v))
        SUM["policies"][setname]["at_k"] = atk
        SUM["policies"][setname]["base_log2_mean"] = st.mean(L2(b) for b in pc["base"].values())
    fig.suptitle("F2. Remaining exact search tree after placing k known tiles, by placement rule "
                 "(line = mean over boards, band = 10th to 90th percentile)", fontsize=11)
    save(fig, "F2-remaining-vs-k")

    # greedy choice maps
    sets = [s for s in ("M8", "M8C", "M10") if s in PCS and PCS[s]["rem"]["a-greedy"]]
    gst = {"M8": "g8", "M8C": "g8c", "M10": "g10"}
    fig, axs = plt.subplots(len(sets), 3, figsize=(14, 4.6 * len(sets)), squeeze=False)
    for i, setname in enumerate(sets):
        n = D.SETS[setname]["n"]
        picks = {}
        for r in D.rows(gst[setname]):
            _, seed, k = r["label"].split(":")
            j = min(range(len(r["cand"])), key=lambda j: (r["nodes"][j], r["cand"][j]))
            picks.setdefault(int(seed), {})[int(k[1:])] = r["cand"][j]
        nb = len(picks)
        trows = []
        for j, top in enumerate((4, 8, 16)):
            freq = {q: 0.0 for q in range(n * n) if q not in D.SETS[setname]["clues"]}
            for sd, m in picks.items():
                for k in range(1, top + 1):
                    if k in m:
                        freq[m[k]] += 1
            freq = {q: v / nb for q, v in freq.items()}
            for q, v in freq.items():
                trows.append([top, q, q // n, q % n, f"{v:.3f}"])
            heat(axs[i][j], n, freq, f"{setname}: share of {nb} boards where greedy picks the cell\namong its first {top}",
                 clues=D.SETS[setname]["clues"], fmt="{:.2f}", cmap="magma", vmin=0, vmax=1)
            fr = sum(v for q, v in freq.items() if cls(n, q) != "interior") / top
            rows_top = [sum(v for q, v in freq.items() if q // n == r) / top for r in range(n)]
            SUM.setdefault("greedy_where", {}).setdefault(setname, {})[top] = dict(
                share_on_frame=round(fr, 3), share_by_row=[round(x, 3) for x in rows_top])
        write_csv(f"greedy_choice_{setname}.csv", ["first_k", "cell", "row", "col", "share_of_boards"], trows)
    fig.suptitle("F3. Where measured greedy puts the known tiles (exact row-scan tree, scan from row 0)", fontsize=11)
    save(fig, "F3-greedy-choice-map")

# ---------- item 4: bb-mrv ----------

mr = D.rows("mrv8")
if mr and "M8" in PCS:
    pc = PCS["M8"]
    cap = float(D.CAP_MRV)
    mv = {}
    cens = {}
    for r in mr:
        _, seed, pol, k = r["label"].split(":")
        seed, k = int(seed), int(k[1:])
        p = "g-random" if pol.startswith("g-random") else pol
        v = L2(max(r["nodes"], 1))
        mv.setdefault(p, {}).setdefault(k, {}).setdefault(seed, []).append(v)
        cens.setdefault(p, {}).setdefault(k, []).append(not r["complete"])
    none = mv.get("none", {}).get(0, {})
    table = []
    fig, axs = plt.subplots(1, 2, figsize=(13, 5))
    for p in POL:
        if p not in mv:
            continue
        ks = sorted(mv[p])
        ys, rs_ys = [], []
        for k in ks:
            per_seed = {sd: st.mean(v) for sd, v in mv[p][k].items()}
            m = st.mean(per_seed.values())
            rsv = [pc["rem"][p][sd][k] for sd in per_seed if sd in pc["rem"][p] and k in pc["rem"][p][sd]]
            ncens = sum(cens[p][k])
            table.append([p, k, f"{m:.4f}", ncens, len(cens[p][k]), f"{st.mean(rsv):.4f}" if rsv else ""])
            ys.append(m)
            rs_ys.append(st.mean(rsv) if rsv else float("nan"))
        axs[1].plot(ks, ys, "o-", color=PCOL[p], label=PLAB[p].replace("20 draws", "5 draws"))
        axs[0].plot(ks, rs_ys, "o-", color=PCOL[p], label=PLAB[p])
    write_csv("mrv_vs_rowscan_M8.csv", ["policy", "k", "bbmrv_mean_log2_nodes", "censored_runs", "runs",
                                        "rowscan_mean_log2_nodes_same_boards"], table)
    if none:
        v0 = st.mean(st.mean(v) for v in none.values())
        axs[1].axhline(v0, color="grey", ls=":", lw=0.8)
        axs[1].text(0.2, v0 + 0.2, f"k=0 (>= cap {cap:.0e} on {sum(cens['none'][0])}/{len(cens['none'][0])})",
                    fontsize=7, color="grey")
    axs[1].axhline(L2(cap), color="red", ls="--", lw=0.6)
    axs[0].set_title("row scan (same 10 boards)", fontsize=10)
    axs[1].set_title(f"bb-mrv to exhaustion (cap {cap:.0e} nodes, censored counted as cap)", fontsize=10)
    for ax in axs:
        ax.set_xlabel("k known tiles")
        ax.set_ylabel("log2 nodes to exhaust")
        ax.legend(fontsize=7)
    fig.suptitle("F5. Does an adaptive propagating solver change the placement ranking? 8x8 ci=11, boards 1..10",
                 fontsize=11)
    save(fig, "F5-bbmrv-vs-rowscan")
    SUM["mrv"] = dict(table=table)

with open(os.path.join(RES, "summary.json"), "w") as fh:
    json.dump(SUM, fh, indent=1, default=str)
print(json.dumps(SUM, indent=1, default=str)[:20000])
