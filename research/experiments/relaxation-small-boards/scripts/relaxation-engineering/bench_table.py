"""Print the kernel benchmark as a table of best times and ratios. Usage: python3 -I bench_table.py"""
import json, os
rows = [json.loads(l) for l in open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "raw", "kernel_bench.jsonl")) if l.startswith("{")]
key = lambda r: (r["P"], r["r"], r["prec"])
cfgs = sorted({key(r) for r in rows})
print("P r prec | numpy pairs, allpairs, one-pass (ms) | rust pairs_half, pairs_rows 1t, 2t | rust fused 1t, 2t | rust blas 1t, 2t | ratios: pairs numpy/rust, allpairs numpy/rust-blas, numpy/rust-fused, eval numpy(bm)/rust(blas+pairs)")
for c in cfgs:
    d = {(r["lang"], r["kernel"], r["threads"]): r["seconds"] * 1e3 for r in rows if key(r) == c}
    if len(d) < 10:
        print(c, "incomplete"); continue
    th = max(t for (l, k, t) in d if l == "rust")
    npair, nall, none = d[("numpy", "pairs_bm_style", 1)], d[("numpy", "allpairs_bm_style", 1)], d[("numpy", "both_dense_one_pass", 1)]
    ph, pr1, pr2 = d[("rust", "pairs_half", 1)], d[("rust", "pairs_rows", 1)], d[("rust", "pairs_rows", th)]
    f1, f2 = d[("rust", "allpairs_fused", 1)], d[("rust", "allpairs_fused", th)]
    b1, b2 = d[("rust", "allpairs_blas", 1)], d[("rust", "allpairs_blas", th)]
    print(f"{c[0]} {c[1]} {c[2]} | {npair:.3g} {nall:.3g} {none:.3g} | {ph:.3g} {pr1:.3g} {pr2:.3g} | {f1:.3g} {f2:.3g} | {b1:.3g} {b2:.3g} | "
          f"{npair/ph:.2f} {nall/b1:.2f} {nall/f1:.2f} {(npair+nall)/(b1+ph):.2f} | one-pass numpy vs bm-style numpy {(npair+nall)/none:.2f}, rust(blas+pairs) vs one-pass numpy {none/(b1+ph):.2f}, 2 threads vs 1: pairs {pr1/pr2:.2f} fused {f1/f2:.2f} blas {b1/b2:.2f}")
