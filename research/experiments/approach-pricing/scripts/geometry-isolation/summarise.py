"""TOY BOARDS ONLY. Reads raw/*.jsonl from run_all.sh, prints the tables of the memo
(summary.txt) and draws fig_mismatch_vs_distance.png.
Usage: uv run --with numpy --with matplotlib python -I summarise.py > summary.txt
"""
import json, os, glob, statistics as st
import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
RAW = os.path.join(HERE, "raw")
BOARDS = ["5_3_4_%d" % s for s in range(5)] + ["6_4_5_%d" % s for s in range(5)] + ["7_4_7_%d" % s for s in range(2)]


def lines(fn):
    p = os.path.join(RAW, fn)
    if not os.path.exists(p):
        return []
    out = []
    for l in open(p):
        l = l.strip()
        if l.startswith("{"):
            try:
                out.append(json.loads(l))
            except json.JSONDecodeError:
                pass
    return out


def pearson(a, b):
    a, b = np.asarray(a, float), np.asarray(b, float)
    if len(a) < 3 or a.std() == 0 or b.std() == 0:
        return float("nan")
    return float(np.corrcoef(a, b)[0, 1])


def spearman(a, b):
    ra = np.argsort(np.argsort(a)); rb = np.argsort(np.argsort(b))
    return pearson(ra, rb)


fig_data = {}
print("TOY BOARDS ONLY. Generated boards; never Eternity II progress.\n")
for b in BOARDS:
    n = int(b.split("_")[0]); N = n * n
    print("=" * 100)
    enum = None
    print(f"board {b} (n, cf, ci, seed)")
    # (a) and (b)
    for ms in (1, 2, 3):
        L = lines(f"basin_{b}_m{ms}.jsonl")
        if not L:
            print(f"  M{ms}: no basin output"); continue
        enum = L[0]
        nsol = enum["sols"]
        comps = [r for r in L if r.get("cmd") == "basin"]
        kmax_done = max((r["k"] for r in comps), default=-1)
        # per-solution component size at each k
        for k in range(0, 5):
            ck = [r for r in comps if r["k"] == k]
            if not ck:
                print(f"  M{ms} k={k}: not finished"); continue
            size = {}; capped = {}; other = {}; far = {}
            for r in ck:
                for s, mh in zip(r["sols_in"], r["maxh"]):
                    size[s] = r["size"]; capped[s] = r["capped"]; other[s] = len(r["sols_in"]) > 1; far[s] = mh > N / 2
            covered = len(size)
            sz = [size[s] for s in size]
            n_small = sum(1 for v in sz if v < 100)
            print(f"  M{ms} k={k}: solutions {covered}/{nsol}; component size median {st.median(sz):.0f} "
                  f"[min {min(sz)}, max {max(sz)}]; <100 boards: {n_small}/{covered}; capped: {sum(capped.values())}; "
                  f"with another solution: {sum(other.values())}; reaching Hamming > N/2: {sum(far.values())}; "
                  f"components: {len(ck)}")
        # (b) barrier from component structure
        if nsol >= 2:
            first_pair = None; all_one = None
            for k in range(0, kmax_done + 1):
                ck = [r for r in comps if r["k"] == k]
                if first_pair is None and any(len(r["sols_in"]) > 1 for r in ck):
                    first_pair = k
                if all_one is None and len(ck) == 1 and len(ck[0]["sols_in"]) == nsol:
                    all_one = k
            anycap = any(r["capped"] for r in comps)
            print(f"  M{ms} barrier: first k joining two solutions = {first_pair}; k joining all = {all_one}"
                  f"{' (some components capped; a None is then not exact)' if anycap else ''}")
    if enum:
        print(f"  solutions {enum['sols']}, inner edges {enum['E']}, DFS nodes {enum['dfs_nodes']}")
    B = lines(f"beam_{b}.jsonl")
    B = [r for r in B if r.get("cmd") == "beam"]
    if B:
        ubs = [r["ub"] for r in B]; hams = [r["ham"] for r in B]
        print(f"  beam M1 barrier upper bound over {len(B)} pairs: min {min(ubs)}, median {st.median(ubs)}, max {max(ubs)};"
              f" pair Hamming median {st.median(hams)} of {N}")
    # (c)
    A = lines(f"anneal_{b}.jsonl")
    rnd = [r for r in A if r.get("cmd") == "random"]
    runs = [r for r in A if r.get("cmd") == "traj"]
    if runs:
        solved = sum(r["solved_at"] >= 0 for r in runs)
        fin = [r["final_mm"] for r in runs]
        near = [r for r in runs if 1 <= r["final_mm"] <= 4]
        rd = [r["dist"] for r in rnd]
        print(f"  anneal: {len(runs)} runs, solved {solved} ({100*solved/len(runs):.1f}%); final mismatches median {st.median(fin)}, "
              f"histogram " + str({m: fin.count(m) for m in sorted(set(fin))}))
        if near:
            nd = [r["final_dist"] for r in near]
            print(f"  runs ending at 1-4 mismatches: {len(near)}; distance to nearest solution median {st.median(nd)} "
                  f"[min {min(nd)}, max {max(nd)}]; random boards: median {st.median(rd)} [min {min(rd)}]")
        tr = [r for r in runs if "trapK" in r]
        if tr:
            reach = sum(r["trap_reached_sol"] for r in tr); capd = sum(r["trap_capped"] for r in tr)
            szs = [r["trap_size"] for r in tr if not r["trap_reached_sol"]]
            print(f"  trap check (M1 BFS inside <= 4 mismatches from each final board at 1-4): {len(tr)} boards, "
                  f"{reach} connect to a solution, {capd} capped; component size of the others median "
                  f"{st.median(szs) if szs else 'n/a'}, max {max(szs) if szs else 'n/a'}")
        xs, ys = [], []
        for r in runs:
            for _, m, d in r["pts"]:
                xs.append(m); ys.append(d)
            xs.append(r["final_mm"]); ys.append(r["final_dist"])
        for r in rnd:
            xs.append(r["mm"]); ys.append(r["dist"])
        xs = np.array(xs); ys = np.array(ys)
        lo = xs <= 8
        print(f"  FDC over {len(xs)} sampled boards (trajectory snapshots, final boards, 1000 random): "
              f"Pearson {pearson(xs, ys):.3f}, Spearman {spearman(xs, ys):.3f}; "
              f"restricted to <= 8 mismatches ({lo.sum()} boards): Pearson {pearson(xs[lo], ys[lo]):.3f}")
        fx = [r["final_mm"] for r in runs]; fy = [r["final_dist"] for r in runs]
        print(f"  FDC over final boards only: Pearson {pearson(fx, fy):.3f}")
        fig_data.setdefault(n, []).append((xs, ys, np.array(rd)))
    # (d)
    C = lines(f"count_{b}.jsonl") + lines(f"count_{b}_knuth.jsonl")
    for r in C:
        if r.get("cmd") == "count":
            print(f"  count (exact row-scan DFS, K={r['K']}): aborted={r['aborted']} nodes={r['nodes']} counts j=0..: {r['counts']} ({r['sec']} s)")
        if r.get("cmd") == "knuth":
            print(f"  count (Knuth estimator, K={r['K']}, {r['probes']} probes): est {r['est']} se {r['se']}")

try:
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    fig, axes = plt.subplots(1, len(fig_data), figsize=(4.2 * len(fig_data), 3.6), sharey=False)
    if len(fig_data) == 1:
        axes = [axes]
    for ax, n in zip(axes, sorted(fig_data)):
        X = np.concatenate([d[0] for d in fig_data[n]]); Y = np.concatenate([d[1] for d in fig_data[n]])
        R = np.concatenate([d[2] for d in fig_data[n]])
        ms = np.unique(X)
        mean = [Y[X == m].mean() for m in ms]
        lo = [np.percentile(Y[X == m], 10) for m in ms]; hi = [np.percentile(Y[X == m], 90) for m in ms]
        ax.fill_between(ms, lo, hi, color="#9db7d5", alpha=0.6, label="10th to 90th percentile")
        ax.plot(ms, mean, color="#1f4e79", lw=1.6, label="mean")
        ax.axhline(R.mean(), color="#b5522b", ls="--", lw=1.2, label="random board, mean")
        ax.set_xlim(0, min(ms.max(), 40))
        ax.set_ylim(0, n * n + 1)
        ax.set_title(f"{n}x{n} toys (all boards of this size pooled)", fontsize=9)
        ax.set_xlabel("mismatched edges")
        ax.set_ylabel("Hamming distance to nearest solution (cells)")
        ax.legend(fontsize=7, loc="lower right")
    fig.suptitle("TOY BOARDS: annealer snapshots plus random boards", fontsize=10)
    fig.tight_layout()
    fig.savefig(os.path.join(HERE, "fig_mismatch_vs_distance.png"), dpi=130)
except ImportError:
    pass
