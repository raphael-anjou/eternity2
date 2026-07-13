#!/usr/bin/env python3
"""Turn a grid run's results.jsonl into a reproducible markdown + CSV report.

One run per (algo, variant): 10 corner-pinned puzzles => 10 runs per algorithm.
Reports, per algorithm: mean / best / worst / median / std ACROSS THE 10
VARIANTS (that spread IS the score distribution — the puzzle is the only
diversity axis), plus the full per-variant score grid. Ranks by mean. Emits the
exact reproduction commands and the git sha.
"""
import argparse
import json
import statistics as st
import subprocess
from collections import defaultdict
from pathlib import Path


def git_sha(repo):
    try:
        return subprocess.check_output(
            ["git", "-C", str(repo), "rev-parse", "--short", "HEAD"],
            text=True).strip()
    except Exception:
        return "unknown"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--run", required=True, help="grid run dir with results.jsonl")
    ap.add_argument("--repo",
                    default="/Users/raphaelanjou/Documents/dev-projects/polytech/eternity2/v2")
    args = ap.parse_args()

    run = Path(args.run)
    rows = [json.loads(l) for l in (run / "results.jsonl").read_text().splitlines() if l.strip()]
    meta = json.loads((run / "run_meta.json").read_text())

    # scores[algo] = list of scores (all variants+rounds); by_variant[algo][vid] = list
    scores = defaultdict(list)
    by_variant = defaultdict(lambda: defaultdict(list))
    npss = defaultdict(list)      # algo -> list of nps
    nps_unit = {}                 # algo -> unit label
    best_url = {}                 # algo -> (score, url)
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

    sha = git_sha(args.repo)
    md = []
    md.append("# Single-core Eternity II solver benchmark\n")
    md.append(f"**Grid:** {len(algos)} algorithms × {len(variant_ids)} corner-pinned "
              f"variants (**one run per puzzle**), **{meta['budget_s']}s each, single core**, "
              f"{meta['parallel']} in parallel.\n")
    md.append(f"**Diversity axis = the puzzle.** Each algorithm solves the SAME 10 "
              f"instances (official E2 + 3 pinned corners, distinct corner arrangements). "
              f"The score spread across the 10 variants is the reported distribution.\n")
    md.append(f"**Scoring:** every run emits a bucas `.url`; score is the canonical "
              f"matched-edge count (max 480), the single source of truth (engines' "
              f"self-reports are never trusted).\n")
    md.append(f"**Repro:** git `{sha}`, variants `{meta['variants_dir']}`, "
              f"seed {meta.get('seed', 1)}.\n")

    def fmt_nps(a):
        if not npss.get(a):
            return "—"
        med = int(st.median(npss[a]))
        unit = nps_unit.get(a, "nodes/s")
        # human-ish: K/M
        if med >= 1_000_000:
            v = f"{med/1e6:.1f}M"
        elif med >= 1_000:
            v = f"{med/1e3:.0f}K"
        else:
            v = str(med)
        return f"{v} {unit}"

    # Main ranking table
    md.append("\n## Ranking (across the 10 corner variants)\n")
    md.append("Throughput (`nps`) is the engine's NATIVE unit — search-nodes/s "
              "(DFS/CSP + backtrackers), beam-nodes/s (producer), or iters/s "
              "(ALNS). **These units are NOT comparable across families**; nps "
              "ranks throughput *within* a family only.\n")
    md.append("| rank | algorithm | mean | best | worst | median | std | n | fails | nps (native unit) |")
    md.append("|---:|:--|---:|---:|---:|---:|---:|---:|---:|:--|")
    for i, a in enumerate(algos, 1):
        s = stats(scores[a])
        md.append(f"| {i} | `{a}` | {s['mean']} | **{s['best']}** | {s['worst']} | "
                  f"{s['med']} | {s['std']} | {s['n']} | {fail[a]} | {fmt_nps(a)} |")

    # Per-variant score grid (one run per cell)
    md.append("\n## Score per variant (one 60s run each)\n")
    header = "| algorithm | " + " | ".join(f"v{v:02d}" for v in variant_ids) + " | mean |"
    md.append(header)
    md.append("|:--|" + "---:|" * (len(variant_ids) + 1))
    for a in algos:
        cells = []
        for v in variant_ids:
            xs = by_variant[a].get(v, [])
            # one run per variant, but be robust if a cell has >1
            cells.append(f"{max(xs)}" if xs else "—")
        overall = f"{st.mean(scores[a]):.1f}" if scores[a] else "—"
        md.append(f"| `{a}` | " + " | ".join(cells) + f" | **{overall}** |")

    # Best board per algo
    md.append("\n## Best board found per algorithm\n")
    md.append("| algorithm | best score | bucas url |")
    md.append("|:--|---:|:--|")
    for a in algos:
        if a in best_url:
            s, u = best_url[a]
            urltxt = Path(u).name
            md.append(f"| `{a}` | {s} | `{urltxt}` |")

    # Community / historical anchors so the single-core numbers have context.
    md.append("\n## Context: where these single-core numbers sit\n")
    md.append("| reference | score | conditions |")
    md.append("|:--|---:|:--|")
    md.append("| Community ceiling (any hints) | 470 | Blackwood, farm-scale compute |")
    md.append("| Community 5-hint record | 464 | Pejic + benj39100, 2026 |")
    md.append("| Community strict 5/5 | 460 | 2023 |")
    md.append("| **Ours** strict 5/5 record | 461 | vol-234 comb-producer + ALNS |")
    md.append("| Ours from-scratch matched (historical) | 460–463 | V155 prior beam / ALNS, minutes |")
    md.append("| Blackwood-from-scratch on OUR 8-core HW | 438 | proves 464-470 is a COMPUTE wall, not cleverness |")
    md.append("\nThis grid caps each engine at **one core for 60s** — far below the "
              "compute that produced the records above. It measures *per-core efficiency* "
              "and heuristic quality, not peak reachable score. An engine that scores well "
              "HERE reaches good boards cheaply; the record numbers need many cores × hours.\n")

    md.append("\n## Reproduction\n")
    md.append("```bash")
    md.append("cd " + args.repo)
    md.append("cargo build -p bench-grid --release")
    md.append("cargo build -p eternity2-bench-audit --release "
              "--bin vol232_w1_producer_trie --bin blackwood_bt "
              "--bin verhaard_faithful_v2 --bin alns_only --bin verify_bucas")
    md.append("# regenerate the identical 10 variants:")
    md.append("./target/release/gen_variants output/vol-235/variants_REGEN")
    md.append("# run the grid:")
    md.append(f"python3 crates/bench-grid/wrappers/run_grid.py \\")
    md.append(f"  --variants {meta['variants_dir']} --out {run} \\")
    md.append(f"  --budget-s {meta['budget_s']} --seed {meta.get('seed', 1)} "
              f"--parallel {meta['parallel']}")
    md.append(f"python3 crates/bench-grid/wrappers/make_report.py --run {run}")
    md.append("```")

    out_md = run / "REPORT.md"
    out_md.write_text("\n".join(md) + "\n")

    # CSV of raw per-run rows
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
