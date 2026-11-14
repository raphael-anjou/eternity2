"""Prints one line per run log in raw/ (first tag, a few iterations, the final record)."""
import json, sys, glob, os
for f in sorted(glob.glob(os.path.join(os.path.dirname(os.path.abspath(__file__)), "raw", "[apq]*.jsonl"))):
    rows = [json.loads(l) for l in open(f) if l.strip()]
    if not rows:
        continue
    its = [d for d in rows if "it" in d]
    fin = [d for d in rows if d.get("final")]
    print("==", os.path.basename(f), {k: rows[0].get(k) for k in ("knn", "kz", "tilefr", "sides", "n_classes", "n_far_blocks", "n_zero", "n_nonneg", "face_order")})
    for d in its[:: max(1, len(its) // 5)] + its[-1:]:
        print("   it %6d t %6.1f viol %.1e off/cell %.3e obj %.4f npos %d off>1e-3 %d sol<1e-4 %d" % (d["it"], d["t"], d["viol_max"], d["mass_off_per_cell"], d["obj"], d["npos"], d["n_off_above_1e3"], d["n_sol_below_1e4"]))
    if fin:
        print("   final rank", fin[0]["rank_1e3"], "gap to full lift", fin[0]["full_lift_gap"])
