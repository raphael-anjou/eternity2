"""TOY. One row per (board, mode, r) from every raw/<stage>/*.jsonl, as markdown tables.
Feasible: max violation of every constraint of lift.py's relaxation <= FEAS (fixed before stage 1).
Success (kill line): feasible AND mass off solution placements per cell < 0.05.
Usage: python3 -I summarise.py raw/stage1 [raw/stage2 ...] > summary_<stage>.txt"""
import glob, json, os, sys
from statistics import median

FEAS = 1e-3


def med(v):
    v = [a for a in v if a is not None]
    return median(v) if v else None


def fmt(v, nd=3):
    if v is None:
        return "n/a"
    if isinstance(v, float):
        return f"{v:.{nd}g}"
    return str(v)


rows = {}
for d in sys.argv[1:]:
    for f in sorted(glob.glob(os.path.join(d, "*.jsonl"))):
        for line in open(f):
            r = json.loads(line)
            rows.setdefault((tuple(r["board"]), r["mode"], r["r"]), []).append(r)

print(f"feasible = max violation <= {FEAS:g}; success = feasible and mass off < 0.05\n")
print("| board | mode | r | starts | feasible | viol<=1e-4 | success | median viol | median mass off | min mass off | median hull dist | median Linf vs full | median obj (full) | greedy solved | Hungarian solved | median mismatches greedy | median s | peak MB |")
print("|" + "---|" * 18)
for key in sorted(rows):
    rs = rows[key]
    b, mode, r = key
    feas = [x for x in rs if x["viol"]["max"] <= FEAS]
    succ = [x for x in feas if x.get("mass_off_solution_placements_per_cell", 1) < 0.05]
    hs = sum(1 for x in rs if (x.get("hungarian_round") or {}).get("is_solution"))
    print("| " + " | ".join([
        "x".join(map(str, b[:1] * 2)) + f" {b[1]}/{b[2]} s{b[3]}", mode, str(r), str(len(rs)),
        str(len(feas)), str(sum(1 for x in rs if x["viol"]["max"] <= 1e-4)), str(len(succ)),
        fmt(med([x["viol"]["max"] for x in rs])),
        fmt(med([x.get("mass_off_solution_placements_per_cell") for x in rs])),
        fmt(min([x.get("mass_off_solution_placements_per_cell", 9) for x in rs])) if "mass_off_solution_placements_per_cell" in rs[0] else "n/a",
        fmt(med([x.get("hull_distance_Linf") for x in rs])),
        fmt(med([x.get("linf_vs_full_sdp") for x in rs])),
        fmt(med([x["obj"] for x in rs]), 4) + (f" ({rs[0]['obj_full_sdp']:.4g})" if "obj_full_sdp" in rs[0] else ""),
        str(sum(1 for x in rs if x["greedy_round"]["is_solution"])), str(hs),
        fmt(med([x["greedy_round"]["mismatches"] for x in rs])),
        fmt(med([x["seconds"] for x in rs])), fmt(max(x["peak_rss_mb_process"] for x in rs)),
    ]) + " |")
