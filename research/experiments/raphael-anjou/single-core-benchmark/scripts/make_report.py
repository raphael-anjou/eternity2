#!/usr/bin/env python3
"""Turn a grid run's results.jsonl into a reproducible markdown + CSV report.

Public port of the vault reporter, made repo-relative. One run per (algo,
variant): 10 corner-pinned puzzles => 10 runs per algorithm. Reports, per
algorithm: mean / best / worst / median / std ACROSS THE 10 VARIANTS (that
spread IS the score distribution -- the puzzle is the only diversity axis), plus
the full per-variant score grid. Ranks by mean.
"""
import argparse
import json
import statistics as st
import subprocess
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
EXPERIMENT = HERE.parent


def git_sha(repo):
    try:
        return subprocess.check_output(
            ["git", "-C", str(repo), "rev-parse", "--short", "HEAD"],
            text=True).strip()
    except Exception:
        return "unknown"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--run", default=str(EXPERIMENT / "results"),
                    help="grid run dir with results.jsonl (default: ../results)")
    args = ap.parse_args()

    run = Path(args.run)
    rows = [json.loads(l) for l in (run / "results.jsonl").read_text().splitlines() if l.strip()]
    meta = json.loads((run / "run_meta.json").read_text())

    scores = defaultdict(list)
    by_variant = defaultdict(lambda: defaultdict(list))
    npss = defaultdict(list)
    nps_unit = {}
    best_url = {}
    fail = defaultdict(int)
    for r in rows:
        a = r["algo"]
        if r.get("nps"):
            npss[a].append(r["nps"])
            if r.get("nps_unit"):
                nps_unit[a] = r["nps_unit"]
        if r.get("score") is None:
            fail[a] += 1
            continue
        s = r["score"]
        scores[a].append(s)
        by_variant[a][r["variant"]].append(s)
        if a not in best_url or s > best_url[a][0]:
            best_url[a] = (s, r["url"])

    algos = sorted(scores.keys(), key=lambda a: -st.mean(scores[a]) if scores[a] else 0)
    variant_ids = meta["variant_ids"]

    def stats(xs):
        if not xs:
            return dict(n=0, mean=0, best=0, worst=0, med=0, std=0)
        return dict(n=len(xs), mean=round(st.mean(xs), 1), best=max(xs),
                    worst=min(xs), med=round(st.median(xs), 1),
                    std=round(st.pstdev(xs), 1) if len(xs) > 1 else 0.0)

    sha = git_sha(EXPERIMENT)
    md = []
    md.append("# Single-core Eternity II solver benchmark\n")
    md.append(f"**Grid:** {len(algos)} algorithms x {len(variant_ids)} corner-pinned "
              f"variants (**one run per puzzle**), **{meta['budget_s']}s each, single core**, "
              f"{meta['parallel']} in parallel.\n")
    md.append(f"**Diversity axis = the puzzle.** Each algorithm solves the SAME 10 "
              f"instances (official E2 + 3 pinned corners, distinct corner arrangements). "
              f"The score spread across the 10 variants is the reported distribution.\n")
    md.append(f"**Scoring:** every run emits a bucas `.url`; score is the canonical "
              f"matched-edge count (max 480), the single source of truth (engines' "
              f"self-reports are never trusted).\n")
    md.append(f"**Repro:** git `{sha}`, seed {meta.get('seed', 1)}.\n")

    def fmt_nps(a):
        if not npss.get(a):
            return "-"
        med = int(st.median(npss[a]))
        unit = nps_unit.get(a, "nodes/s")
        if med >= 1_000_000:
            v = f"{med/1e6:.1f}M"
        elif med >= 1_000:
            v = f"{med/1e3:.0f}K"
        else:
            v = str(med)
        return f"{v} {unit}"

    md.append("\n## Ranking (across the 10 corner variants)\n")
    md.append("Throughput (`nps`) is the engine's NATIVE unit -- search-nodes/s "
              "(DFS/CSP + backtrackers), beam-nodes/s (producer), or iters/s "
              "(ALNS). **These units are NOT comparable across families**; nps "
              "ranks throughput *within* a family only.\n")
    md.append("| rank | algorithm | mean | best | worst | median | std | n | fails | nps (native unit) |")
    md.append("|---:|:--|---:|---:|---:|---:|---:|---:|---:|:--|")
    for i, a in enumerate(algos, 1):
        s = stats(scores[a])
        md.append(f"| {i} | `{a}` | {s['mean']} | **{s['best']}** | {s['worst']} | "
                  f"{s['med']} | {s['std']} | {s['n']} | {fail[a]} | {fmt_nps(a)} |")

    md.append("\n## Score per variant (one 60s run each)\n")
    header = "| algorithm | " + " | ".join(f"v{v:02d}" for v in variant_ids) + " | mean |"
    md.append(header)
    md.append("|:--|" + "---:|" * (len(variant_ids) + 1))
    for a in algos:
        cells = []
        for v in variant_ids:
            xs = by_variant[a].get(v, [])
            cells.append(f"{max(xs)}" if xs else "-")
        overall = f"{st.mean(scores[a]):.1f}" if scores[a] else "-"
        md.append(f"| `{a}` | " + " | ".join(cells) + f" | **{overall}** |")

    md.append("\n## Best board found per algorithm\n")
    md.append("| algorithm | best score | bucas url |")
    md.append("|:--|---:|:--|")
    for a in algos:
        if a in best_url:
            s, u = best_url[a]
            md.append(f"| `{a}` | {s} | `{Path(u).name}` |")

    md.append("\n## Reproduction (native family)\n")
    md.append("```bash")
    md.append("cd research/experiments/raphael-anjou/single-core-benchmark")
    md.append("cargo build --release --bin run_algo --manifest-path engine/Cargo.toml")
    md.append("python3 scripts/run_grid.py --variants variants \\")
    md.append(f"  --out results/rerun --budget-s {meta['budget_s']} "
              f"--seed {meta.get('seed', 1)} --parallel {meta['parallel']}")
    md.append("python3 scripts/make_report.py --run results/rerun")
    md.append("```")
    md.append("\nThe four standalone strong engines (`producer`, `blackwood`, "
              "`verhaard`, `alns`) run from the private v2 vault and are not "
              "reproducible from this repo; their committed rows above are the "
              "published result.\n")

    out_md = run / "REPORT.md"
    out_md.write_text("\n".join(md) + "\n")

    import csv
    with open(run / "results.csv", "w", newline="") as f:
        w = csv.writer(f)
        w.writerow(["algo", "variant", "seed", "score", "wall_s", "url"])
        for r in rows:
            w.writerow([r["algo"], r["variant"], r.get("seed"),
                        r.get("score"), r.get("wall_s"), r.get("url")])

    print(f"report  -> {out_md}")
    print(f"csv     -> {run/'results.csv'}")
    print("\n=== RANKING (mean over all runs) ===")
    for i, a in enumerate(algos, 1):
        s = stats(scores[a])
        print(f"{i:2d}. {a:22s} mean={s['mean']:5} best={s['best']:3} "
              f"std={s['std']:4} n={s['n']} fails={fail[a]}")


if __name__ == "__main__":
    main()
