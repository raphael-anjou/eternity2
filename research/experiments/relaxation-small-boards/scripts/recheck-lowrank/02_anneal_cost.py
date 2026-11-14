"""TOY. re-check: annealing baseline per unit of work, from the isolation study's own rows
(scripts/geometry-isolation/raw/anneal_*.jsonl, 200 runs x 2e6 steps per board), and a timing of
the same annealer binary on this machine (20 runs per 5x5 board, one process, nice 10).
Steps per solution = total steps spent (a solved run charged to its solved_at step when recorded,
an unsolved run charged in full) / number of solved runs."""
import json, os, subprocess, sys, time
H = os.path.dirname(os.path.abspath(__file__))
G = os.path.join(H, "..", "geometry-isolation")
STEPS = 2_000_000
for b in ["5_3_4_0", "5_3_4_1", "5_3_4_2", "5_3_4_3", "5_3_4_4", "6_4_5_0", "6_4_5_1", "6_4_5_2", "6_4_5_3", "6_4_5_4"]:
    runs = []; enum = None
    for l in open(os.path.join(G, "raw", f"anneal_{b}.jsonl")):
        d = json.loads(l)
        if d.get("cmd") == "enum": enum = d
        if d.get("cmd") == "traj": runs.append(d)
    ok = [r for r in runs if r["final_mm"] == 0]
    hit = [r["solved_at"] for r in runs if r["solved_at"] >= 0]
    tot = sum(r["solved_at"] if r["solved_at"] >= 0 else STEPS for r in runs)
    line = (f"{b}: enum sols {enum['sols']} dfs_nodes {enum['dfs_nodes']} sec {enum['sec']} | anneal runs {len(runs)} final_mm==0 {len(ok)} "
            f"hit a solution at some step {len(hit)} median solved_at {sorted(hit)[len(hit)//2] if hit else None} steps per solution {tot/len(hit) if hit else float('inf'):.3g}")
    if b.startswith("5") and "time" in sys.argv:
        t = time.time()
        out = subprocess.run(["nice", "-n", "10", os.path.join(G, "iso"), os.path.join(G, "boards", f"b_{b}.txt"), "anneal", "20", str(STEPS), "7", "0"], capture_output=True, text=True).stdout
        dt = time.time() - t
        rr = [json.loads(l) for l in out.splitlines() if '"traj"' in l]
        s = sum(1 for r in rr if r["final_mm"] == 0)
        line += f" | timed here: 20 runs in {dt:.2f} s wall = {dt/20*1e3:.0f} ms per 2e6-step run, {s}/20 solved"
        if hit: line += f" => about {tot/len(hit)/STEPS*dt/20:.3f} s per solution"
    print(line, flush=True)
