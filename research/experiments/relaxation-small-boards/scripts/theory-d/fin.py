"""One line per finished run in raw/ (the record marked final)."""
import json, glob, os
for f in sorted(glob.glob(os.path.join(os.path.dirname(os.path.abspath(__file__)), "raw", "[apq]*.jsonl"))):
    rows = [json.loads(l) for l in open(f) if l.strip().startswith("{")]
    fin = [d for d in rows if d.get("final")]
    its = [d for d in rows if "it" in d]
    if fin:
        d = fin[0]
        print("%-44s seed %d knn %-2s %-14s obj %-4s nn %6d its %6d viol %.1e off/cell %.3e l2 %.4f rank %3d off>1e-3 %3d" % (
            os.path.basename(f), d["board"][3], d["knn"], d.get("mask", d.get("sides", "")) or "", d["obj"], d["n_nonneg"], d["iters"],
            max(d["viol"].values()), d["mass_off_per_cell"], d["obj_val"], d["rank_1e3"], its[-1]["n_off_above_1e3"]))
