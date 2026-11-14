"""TOY. re-check tally of every lowrank-rust result row: per file and tag, runs, solved, the
(board,start) set, distinct configurations, evaluation medians, enumeration membership."""
import json, glob, os, sys
from statistics import median
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "lowrank-rust", "results")
files = sorted(glob.glob(R + "/*.jsonl") + glob.glob(R + "/*/*.jsonl"))
cfgs = set()
for f in files:
    if f.endswith("kernels.jsonl"): continue
    rows = [json.loads(l) for l in open(f)]
    tags = {}
    for r in rows: tags.setdefault(r.get("tag", "?"), []).append(r)
    for t, rs in tags.items():
        good = [r for r in rs if "broken" not in r["board"]]
        bro = [r for r in rs if "broken" in r["board"]]
        ok = [r for r in good if r["solved"]]
        keys = sorted({(r["board"], r["start"]) for r in good})
        boards = sorted({r["board"] for r in good})
        starts = sorted({r["start"] for r in good})
        c = {json.dumps({k: v for k, v in r["cfg"].items() if k not in ("time_limit",)}, sort_keys=True) for r in rs}
        cfgs |= c
        es = [r["evals_solution"] for r in ok]
        ts = [r["t_solution"] for r in ok]
        notin = sum(1 for r in ok if r.get("solution_in_enumeration") is False)
        rst = sum(1 for r in good if r.get("restarts", 0) > 0)
        stat = {}
        for r in good: stat[r["status"]] = stat.get(r["status"], 0) + 1
        print(f"{os.path.relpath(f, R):32s} {t:22s} n={len(good):3d} uniq={len(keys):3d} boards={len(boards)} starts={starts[0] if starts else '-'}..{starts[-1] if starts else '-'} "
              f"solved={len(ok):3d} medE={median(es) if es else None} maxE={max(es) if es else None} medT={round(median(ts),2) if ts else None} "
              f"notInEnum={notin} restarted={rst} status={stat} broken={sum(r['solved'] for r in bro)}/{len(bro)} ncfg={len(c)}")
print("distinct configurations over all files (time limit ignored):", len(cfgs))
