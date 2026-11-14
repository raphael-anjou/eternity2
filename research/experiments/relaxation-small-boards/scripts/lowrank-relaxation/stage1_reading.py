"""TOY. Reading of the 5x5 runs against the kill line, plus what the feasible low-rank points are.
Kill line (fixed in the brief): unreliable if, at the smallest r for which any start succeeds,
fewer than half of the starts reach a feasible point with mass off solution placements < 0.05 on
most 5x5 boards. Feasible: max violation <= 1e-3.
Usage: python3 -I stage1_reading.py raw/stage1"""
import glob, json, os, sys
from statistics import median

rows = []
for f in sorted(glob.glob(os.path.join(sys.argv[1], "*.jsonl"))):
    rows += [json.loads(l) for l in open(f)]
rnd = [r for r in rows if r["mode"] == "random"]


def succ(r):
    return r["viol"]["max"] <= 1e-3 and r["mass_off_solution_placements_per_cell"] < 0.05


def solved(r):
    return r["greedy_round"]["is_solution"] or bool((r.get("hungarian_round") or {}).get("is_solution"))


ranks = sorted({r["r"] for r in rnd})
print("ranks run:", ranks)
verdict = []
for seed in range(5):
    b = [r for r in rnd if r["board"][3] == seed]
    rs = [rr for rr in ranks if any(succ(r) for r in b if r["r"] == rr)]
    if not rs:
        print(f"seed {seed}: no success at any rank run"); verdict.append(False); continue
    r0 = rs[0]
    at = [r for r in b if r["r"] == r0]
    k = sum(succ(r) for r in at)
    verdict.append(2 * k >= len(at))
    print(f"seed {seed} ({at[0]['n_sols']} sols): smallest rank run with a success r={r0}: {k}/{len(at)} starts succeed;"
          f" per rank: " + ", ".join(f"r={rr}: {sum(succ(r) for r in b if r['r'] == rr)}/{sum(1 for r in b if r['r'] == rr)}" for rr in ranks))
print(f"boards meeting the bar: {sum(verdict)}/5 -> {'NOT killed' if sum(verdict) >= 3 else 'KILLED'}")
print()
print("rounding of random starts, by rank: solved (greedy or Hungarian) / runs; solved among successes")
for rr in ranks:
    a = [r for r in rnd if r["r"] == rr]
    s = [r for r in a if succ(r)]
    print(f"r={rr}: {sum(solved(r) for r in a)}/{len(a)}; {sum(solved(r) for r in s)}/{len(s)};"
          f" greedy alone {sum(r['greedy_round']['is_solution'] for r in a)}/{len(a)}")
print()
print("what the feasible points are (successes): numerical rank of Z at 1e-3, objective ||x||^2, Linf to the full solve")
for rr in ranks:
    s = [r for r in rnd if r["r"] == rr and succ(r)]
    if not s:
        continue
    li = [r["linf_vs_full_sdp"] for r in s if "linf_vs_full_sdp" in r]
    print(f"r={rr}: rank median {median(r['rank_1e3'] for r in s)} (range {min(r['rank_1e3'] for r in s)}-{max(r['rank_1e3'] for r in s)}),"
          f" obj median {median(r['obj'] for r in s):.2f}, Linf median {median(li):.3f} over {len(li)} runs with a full solve,"
          f" hull distance median {median(r['hull_distance_Linf'] for r in s):.2g}, max {max(r['hull_distance_Linf'] for r in s):.2g}")
full = {}
for r in rnd:
    if "obj_full_sdp" in r:
        full[r["board"][3]] = r["obj_full_sdp"]
print("full-solve ||x||^2 per seed:", {k: round(v, 3) for k, v in sorted(full.items())})
print()
print("time and memory: median seconds, max peak MB, median evaluations, by rank")
for rr in ranks:
    a = [r for r in rnd if r["r"] == rr]
    print(f"r={rr}: {median(r['seconds'] for r in a):.0f} s, {max(r['peak_rss_mb_process'] for r in a):.0f} MB,"
          f" {median(r['evals'] for r in a):.0f} evals, status {sorted({r['status'] for r in a})},"
          f" active nonneg multipliers median {median(r['active_nonneg_multipliers'] for r in a):.0f} of F={a[0]['F']}")
print()
for mode in ("solution", "broken"):
    a = [r for r in rows if r["mode"] == mode]
    print(f"{mode}: {len(a)} runs; feasible <= 1e-3: {sum(r['viol']['max'] <= 1e-3 for r in a)};"
          f" violation range {min(r['viol']['max'] for r in a):.2g}-{max(r['viol']['max'] for r in a):.2g};"
          f" rounding is a solution: {sum(r['greedy_round']['is_solution'] for r in a)}")
