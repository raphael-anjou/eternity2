"""TOY. Probe A: a literal projection of the solution space and of the search tree.

For one generated board with every solution enumerated:
 1. the solution set as points of {0,1}^(cells x tiles x 4): pairwise Hamming distances (cells that
    differ), the backbone (cells equal in every solution), PCA of the solutions;
 2. for several depths d of the row-scan tree: every live partial board at depth d, marked
    'prefix of a solution' or not. Three geometric statistics, each computable from a sample of
    the level (no solution needed):
      centrality  = overlap with the level centroid = sum over cells of the share of live nodes
                    that put the same (tile, rotation) there;
      pc-extreme  = distance from the centroid in the top 5 principal components;
      density     = number of live nodes within Hamming distance 1 (differ in one cell);
    reported as AUC (solution prefixes vs the rest) and enrichment in the top 10% and 1%.
 3. a PNG: solutions in PCA; the peak level in PCA with solution prefixes highlighted.
Usage: probe_a_projection.py n cf ci seed out_prefix
"""
import sys, os, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
from common import gen, enumerate_all

n, cf, ci, seed = map(int, sys.argv[1:5])
out = sys.argv[5]
tiles = gen(n, cf, ci, seed)
N = n * n
base = enumerate_all(n, tiles)
sols = base["sols"]
widths = base["widths"]
peak = widths.index(max(widths))
res = dict(board=[n, cf, ci, seed], n_sols=len(sols), nodes=base["nodes"], peak_depth=peak,
           peak_width=max(widths))

# 1. solution set geometry
S = np.array([[a[0] * 4 + a[1] for a in s] for s in sols])          # code per cell
D = (S[:, None, :] != S[None, :, :]).sum(-1)
iu = np.triu_indices(len(sols), 1)
res["sol_dist_hist"] = np.bincount(D[iu], minlength=N + 1).tolist()
res["sol_dist_mean"] = float(D[iu].mean())
backbone = [x for x in range(N) if len(set(S[:, x])) == 1]
res["backbone_cells"] = len(backbone)
# connected components of the solution set when distance <= k
for k in (2, 4, 6, 8):
    parent = list(range(len(sols)))
    def f(a):
        while parent[a] != a:
            parent[a] = parent[parent[a]]; a = parent[a]
        return a
    for i, j in zip(*iu):
        if D[i, j] <= k:
            parent[f(i)] = f(j)
    res[f"sol_components_le{k}"] = len({f(i) for i in range(len(sols))})


def onehot(codes, ncell):
    M = np.zeros((len(codes), ncell * N * 4), dtype=np.float32)
    for r, row in enumerate(codes):
        for x, v in enumerate(row):
            M[r, x * N * 4 + v] = 1
    return M


def pca(M, k):
    mu = M.mean(0)
    U, s, Vt = np.linalg.svd(M - mu, full_matrices=False)
    var = s ** 2 / max(1, (s ** 2).sum())
    return (M - mu) @ Vt[:k].T, var[:k], mu, Vt[:k]


Ms = onehot(S, N)
Ys, vs, _, _ = pca(Ms, 2)
res["sol_pca_var2"] = vs.tolist()


def auc(pos, neg):
    allv = np.concatenate([pos, neg])
    ranks = allv.argsort().argsort().astype(float) + 1
    # average ties
    order = np.argsort(allv); sv = allv[order]
    i = 0
    while i < len(sv):
        j = i
        while j + 1 < len(sv) and sv[j + 1] == sv[i]:
            j += 1
        ranks[order[i:j + 1]] = (i + j + 2) / 2
        i = j + 1
    rp = ranks[:len(pos)].sum()
    return (rp - len(pos) * (len(pos) + 1) / 2) / (len(pos) * len(neg))


def enrich(score, lab, q):
    k = max(1, int(round(q * len(score))))
    top = np.argsort(-score)[:k]
    return float(lab[top].mean() / lab.mean())


rng = np.random.default_rng(0)
levels = {}
depths = sorted(set([max(2, N // 4), max(3, N // 3), peak, min(N - 2, (2 * N) // 3)]))
for d in depths:
    r = enumerate_all(n, tiles, keep_depth=d)
    live = r["live"]
    codes = np.array([[a[0] * 4 + a[1] for a in p] for p, _ in live])
    lab = np.array([1.0 if b > 0 else 0.0 for _, b in live])
    # centrality (exact, over the whole level)
    cent = np.zeros(len(live))
    for x in range(d):
        vals, inv, cnt = np.unique(codes[:, x], return_inverse=True, return_counts=True)
        cent += cnt[inv] / len(live)
    # density: live nodes differing in exactly one cell (hash with one cell masked)
    dens = np.zeros(len(live))
    w = rng.integers(1, 2 ** 62, size=(d, N * 4), dtype=np.uint64)   # random hash per (cell, value)
    h = np.zeros(len(live), dtype=np.uint64)
    for x in range(d):
        h += w[x, codes[:, x]]
    for x in range(1, d):        # cell 0 is pinned
        hm = h - w[x, codes[:, x]]
        _, inv, cnt = np.unique(hm, return_inverse=True, return_counts=True)
        dens += cnt[inv] - 1
    # PCA on a sample (all positives plus up to 20000 negatives)
    pos_idx = np.where(lab == 1)[0]
    neg_idx = np.where(lab == 0)[0]
    samp_neg = rng.choice(neg_idx, size=min(20000, len(neg_idx)), replace=False)
    idx = np.concatenate([pos_idx, samp_neg])
    M = onehot(codes[idx], d)
    Y, v, mu, Vt = pca(M, 5)
    ext = np.sqrt((Y ** 2).sum(1))
    lab_s = lab[idx]
    entry = dict(depth=d, width=len(live), sol_prefixes=int(lab.sum()), share=float(lab.mean()),
                 pca_var5=[float(a) for a in v])
    for name, sc, lb in (("centrality", cent, lab), ("density", dens, lab), ("pc_extreme", ext, lab_s)):
        entry[name] = dict(auc=float(auc(sc[lb == 1], sc[lb == 0])),
                           enrich10=enrich(sc, lb, 0.10), enrich1=enrich(sc, lb, 0.01),
                           anti_enrich10=enrich(-sc, lb, 0.10))
    levels[d] = entry
    if d == peak:
        peak_plot = (Y[:, :2], lab_s)
    print(json.dumps(entry), flush=True)
res["levels"] = levels

with open(out + ".json", "w") as fh:
    json.dump(res, fh, indent=1)

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
fig, ax = plt.subplots(1, 3, figsize=(16, 5))
ax[0].scatter(Ys[:, 0], Ys[:, 1], s=6, c="#3366aa")
ax[0].set_title(f"{len(sols)} solutions, PCA of one-hot boards\nvar {vs[0]:.2f}, {vs[1]:.2f}")
ax[1].bar(range(N + 1), res["sol_dist_hist"], color="#3366aa")
ax[1].set_title("pairwise distance between solutions (cells differing)")
Y2, lb = peak_plot
ax[2].scatter(Y2[lb == 0, 0], Y2[lb == 0, 1], s=2, c="#bbbbbb", label="dead prefixes (sample)")
ax[2].scatter(Y2[lb == 1, 0], Y2[lb == 1, 1], s=12, c="#cc3311", label="prefixes of solutions")
ax[2].set_title(f"peak level d={peak}, width {max(widths)}: PCA")
ax[2].legend(loc="best", fontsize=8)
fig.suptitle(f"TOY {n}x{n} cf={cf} ci={ci} seed={seed}")
fig.tight_layout()
fig.savefig(out + ".png", dpi=110)
print(json.dumps({k: v for k, v in res.items() if k != "levels"}))
