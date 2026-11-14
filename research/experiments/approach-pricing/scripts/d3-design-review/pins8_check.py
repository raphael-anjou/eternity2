"""Design-review check (not a test run): on 8x8 hard-ladder CALIBRATION
instances with pins, compare exact enumeration with plain SMC and with the
planted-path decomposition (PPD), clustered against spread pins.
Usage: python3 -I pins8_check.py <experiment dir> <work dir> [n_instances]
One core, run under nice. Uses the experiment's built binary, read only."""
import json, math, os, statistics as st, subprocess, sys

EXP, WORK = sys.argv[1], sys.argv[2]
NI = int(sys.argv[3]) if len(sys.argv) > 3 else 6
BIN = os.path.join(EXP, "engine/target/release/rpp")
SRC = os.path.join(EXP, "../hard-ladder/instances/8-c10-calibration.jsonl")
N = 8
os.makedirs(WORK, exist_ok=True)
order = os.path.join(WORK, "rows-b-lr-8.txt")
with open(order, "w") as fh:
    fh.write(" ".join(str(r * N + c) for r in range(N - 1, -1, -1) for c in range(N)) + "\n")
SETS = {
    "none": [],
    "clustered8": [(5, 1), (5, 3), (5, 5), (6, 2), (6, 4), (7, 1), (7, 3), (7, 5)],
    "spread8": [(6, 2), (6, 5), (4, 2), (4, 5), (3, 3), (2, 2), (2, 5), (1, 3)],
    "clustered4": [(5, 1), (5, 4), (6, 2), (6, 5)],
    "spread4": [(6, 3), (4, 4), (3, 2), (1, 4)],
}
NP_SMC, NP_PPD, REPS, CAP = 100, 10, 200, int(4e8)


def run(args):
    out = subprocess.run([BIN] + args, capture_output=True, text=True, check=True).stdout
    return [json.loads(l) for l in out.splitlines() if l.startswith("{")]


def summ(tots, exact):
    m = st.mean(tots)
    se = st.stdev(tots) / math.sqrt(len(tots))
    med = st.median(tots)
    low = sum(t < exact / 10 for t in tots) / len(tots)
    return (f"mean/exact {m / exact:7.3f} (se {se / exact:6.3f})  median/exact {med / exact:8.4f}  "
            f"share of reps under exact/10: {low:4.2f}")


for li, line in enumerate(open(SRC)):
    if li >= NI:
        break
    d = json.loads(line)
    planted = d.get("planted") or d["puzzle"].get("planted")
    pieces = d["puzzle"]["pieces"]
    bpath = os.path.join(WORK, f"b8-{li}.txt")
    with open(bpath, "w") as fh:
        fh.write(f"{N}\n")
        for p in pieces:
            fh.write(" ".join(map(str, p)) + "\n")
        for v in planted:
            fh.write(f"{v // 4} {v % 4}\n")
    for name, cells in SETS.items():
        fixed = ",".join(str(r * N + c) for r, c in cells)
        base = ["--board", bpath, "--order", order] + (["--fixed", fixed] if cells else [])
        e = run(["tenum"] + base + ["--cap", str(CAP)])[0]
        if not e["complete"]:
            print(f"{d['id']} {name}: enumeration past {CAP:.0e}, skipped", flush=True)
            continue
        ex = e["total"]
        smc = [r["total"] for r in run(["tsmc"] + base + ["--np", str(NP_SMC), "--reps", str(REPS), "--seed0", "5001"])]
        ppd = [r["total"] for r in run(["tppd"] + base + ["--np", str(NP_PPD), "--reps", str(REPS), "--seed0", "7001"])]
        print(f"{d['id']} {name}: exact {ex:.3e} sols {e['solutions']} first at {e['nodes_at_first_solution']}"
              f" ({e['seconds']} s)\n   SMC N={NP_SMC}: {summ(smc, ex)}\n   PPD N={NP_PPD}: {summ(ppd, ex)}", flush=True)
