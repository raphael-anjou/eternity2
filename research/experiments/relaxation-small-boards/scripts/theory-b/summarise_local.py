"""Table of raw/loc/bm_local.jsonl (experiment 5): per distance d and board, solved of runs,
evaluations to the verified solution, and the state of the factor at that moment."""
import json, os, statistics as st
D = os.path.join(os.path.dirname(os.path.abspath(__file__)), "raw", "loc")
rs = [json.loads(l) for l in open(os.path.join(D, "bm_local.jsonl"))]
by = {}
for r in rs:
    by.setdefault((r["d"], "broken" if r["broken"] else f"s{r['board'][3]}"), []).append(r)
    if not r["broken"]:
        by.setdefault((r["d"], "all five"), []).append(r)
for k in sorted(by, key=lambda k: (k[0], k[1] == "broken", k[1] == "all five", k[1])):
    g = by[k]; s = [r for r in g if r["solved"]]
    ev = sorted(r["evals"] for r in s)
    line = (f"d={k[0]:2d} {k[1]:8s} pairs {g[0]['masked_free_pairs']:6d}/{g[0]['free_pairs']}  solved {len(s):3d}/{len(g):3d}  "
            f"evals median {st.median(ev) if ev else '-'} min {ev[0] if ev else '-'} p90 {ev[int(0.9 * (len(ev) - 1))] if ev else '-'} max {ev[-1] if ev else '-'}  "
            f"median s {st.median(r['seconds'] for r in g):.1f}  "
            f"at stop: viol {st.median(r['viol']['max'] for r in g):.3f}  most negative free entry {st.median(r['most_negative_free_entry'] for r in g):.4f}  "
            f"share of free pairs < -1e-6 {st.median(r['negative_free_share'] for r in g):.3f}")
    if g[0]["mass_off_solutions_per_cell"] is not None:
        line += f"  mass off {st.median(r['mass_off_solutions_per_cell'] for r in g):.2f}"
    if len(s) < len(g):
        line += f"  unsolved mismatches {sorted(r['mismatches'] for r in g if not r['solved'])}"
    print(line)
