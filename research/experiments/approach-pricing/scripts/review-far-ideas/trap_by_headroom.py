"""TOY BOARDS ONLY. Second analysis check on geometry-isolation raw rows: totals, and the trap check split
by the endpoint's mismatch count (headroom inside the level set {<= 4})."""
import json, glob, os, collections
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "geometry-isolation", "raw")
tot = collections.defaultdict(lambda: [0, 0])          # size -> [solved, runs]
trap = collections.defaultdict(lambda: [0, 0, 0])      # (size, final_mm) -> [reach, n, capped]
best = collections.defaultdict(collections.Counter)
hit = collections.defaultdict(list)
for fn in sorted(glob.glob(os.path.join(R, "anneal_*.jsonl"))):
    size = os.path.basename(fn).split("_")[1]
    for l in open(fn):
        d = json.loads(l)
        if d["cmd"] != "traj": continue
        tot[size][1] += 1; tot[size][0] += d["final_mm"] == 0
        best[size][d["best_mm"] - d["final_mm"]] += 1
        if d["solved_at"] >= 0: hit[size].append(d["solved_at"])
        if "trapK" in d:
            t = trap[(size, d["final_mm"])]; t[1] += 1; t[0] += d["trap_reached_sol"]; t[2] += d["trap_capped"]
for s in sorted(tot): print("size", s, "solved", tot[s])
for k in sorted(trap): print("size %s final_mm %d: reach %d of %d (capped %d)" % (k[0], k[1], *trap[k]))
for s in sorted(hit):
    h = sorted(hit[s]); print("size", s, "solved_at step quantiles 10/50/90:", h[len(h)//10], h[len(h)//2], h[9*len(h)//10], "max", h[-1])
for s in sorted(best): print("size", s, "best_mm - final_mm:", dict(best[s]))
