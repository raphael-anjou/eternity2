"""TOY. Print the probe A table (AUC per score at every recorded depth) and the probe B table."""
import glob, json, os

here = os.path.dirname(os.path.abspath(__file__))
print("probe A: board | sols | mean dist between sols / cells | backbone | depth: AUC centrality, density, pc-extreme")
for f in sorted(glob.glob(os.path.join(here, "out_a_*.json"))):
    d = json.load(open(f))
    n = d["board"][0]
    parts = []
    for dep, e in sorted(d["levels"].items(), key=lambda kv: int(kv[0])):
        tag = "*" if int(dep) == d["peak_depth"] else ""
        parts.append(f"d{dep}{tag}({e['sol_prefixes']}/{e['width']}): "
                     f"{e['centrality']['auc']:.2f} {e['density']['auc']:.2f} {e['pc_extreme']['auc']:.2f}")
    print(d["board"], d["n_sols"], f"{d['sol_dist_mean']:.1f}/{n*n}", d["backbone_cells"], " | ".join(parts))
print()
print("probe B: board | sols | log2 DFS nodes | success | matched | dist rounded->nearest sol vs ->nearest random |"
      " beta, H | bits on sols (mean,max) | near (n, max mismatches, bits mean) | random (mean,max) | z vs random | z vs near")
for f in ("out_b.jsonl", "out_b_dfsnear.jsonl"):
    p = os.path.join(here, f)
    if not os.path.exists(p):
        continue
    for line in open(p):
        try:
            d = json.loads(line)
        except ValueError:
            continue
        b = d["best_rows"][0]
        print(d["board"], d["n_sols"], f"{d['log2_dfs_nodes']:.1f}", d["success"], f"{d['matched_mean']:.3f}",
              d["rounded_dist_to_nearest_solution_median"], d["rounded_dist_to_nearest_control_median"],
              b["beta"], b["H"], (b["bits_sol"], b["bits_sol_max"]),
              (d["near_controls"], d.get("near_max_mismatch", 2), b["bits_near"]),
              (b["bits_ctrl"], b["bits_ctrl_max"]), b["z"], b["z_vs_near"])
