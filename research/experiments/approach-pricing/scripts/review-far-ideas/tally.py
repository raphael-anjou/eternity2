"""TOY BOARDS ONLY. Tally of the re-check's annealing reruns in raw/ (rows written by iso_ref)."""
import json, glob, os, collections
H = os.path.dirname(os.path.abspath(__file__))
agg = collections.OrderedDict()
for fn in sorted(glob.glob(os.path.join(H, "raw", "r*_*.jsonl"))):
    name = os.path.basename(fn)[:-6]
    tag, board = name.rsplit("_", 4)[0], "_".join(name.rsplit("_", 4)[1:])
    runs = [json.loads(l) for l in open(fn) if '"traj"' in l]
    solved = [r["solved_at"] for r in runs if r["solved_at"] >= 0]
    fin = collections.Counter(r["final_mm"] for r in runs)
    print(f"{tag:22s} {board}: runs {len(runs):3d} solved {len(solved):3d}  final_mm {dict(sorted(fin.items()))}  solved_at {sorted(solved)[:8]}")
    a = agg.setdefault((tag, board.split('_')[0]), [0, 0]); a[0] += len(solved); a[1] += len(runs)
print()
for k, v in agg.items(): print(f"{k[0]:22s} size {k[1]}: solved {v[0]} of {v[1]}")
