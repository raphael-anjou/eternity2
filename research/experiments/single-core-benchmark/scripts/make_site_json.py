#!/usr/bin/env python3
"""Generate the website's benchmark data from the committed results.

  results/results.jsonl  ->  web/src/data/single-core-benchmark.json

The site's leaderboard component reads ONLY that JSON, so this script is what
keeps the published page from drifting from what was actually measured. Run it
whenever results.jsonl changes (it is wired into the experiments justfile).

Family, the anjou- ownership prefix, and each engine's native throughput unit all
come from the manifest (../solvers.toml, via manifest.py) — the single source of
truth. This script does NOT re-declare those tables; it looks them up per solver:

  * family      -- categorical grouping for bar COLOUR + legend (producer / alns /
                   backtracker / csp / naive). From the manifest `family` field.
  * anjou-      -- site ownership tag. From the manifest `author` field:
                   raphael-anjou -> `anjou-<name>`, community -> bare. Site-side
                   only; the engine, `run_algo --list` and results.jsonl keep the
                   canonical unprefixed names, so repro stays intact.
  * nps unit    -- the engine's native throughput unit, used as the fallback when
                   a run reported no nps. From the manifest `nps_unit` field.

Any algo in results.jsonl with no manifest entry is a hard error (no silent
"unknown" family/unit). Per-algo stats (mean / best / worst / std / nps) are
computed exactly as make_report.py does: over the 10 corner-pinned variants.
"""
import argparse
import json
import statistics as st
import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import manifest  # noqa: E402  (local module, sibling of this script)

HERE = Path(__file__).resolve().parent
EXPERIMENT = HERE.parent            # research/experiments/single-core-benchmark
REPO = EXPERIMENT.parents[2]        # ...<exp> -> experiments -> research -> <repo>
DEFAULT_OUT = REPO / "web" / "src" / "data" / "single-core-benchmark.json"

# Ceiling markers the page annotates (not measured here; puzzle-level facts).
MAX_SCORE = 480
COMMUNITY_RECORD_5CLUE = 464


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--run", default=str(EXPERIMENT / "results"),
                    help="grid run dir with results.jsonl (default: ../results)")
    ap.add_argument("--out", default=str(DEFAULT_OUT),
                    help="site JSON to write (default: web/src/data/...)")
    args = ap.parse_args()

    run = Path(args.run)
    meta = json.loads((run / "run_meta.json").read_text())
    rows = [json.loads(l) for l in (run / "results.jsonl").read_text().splitlines() if l.strip()]
    variant_ids = meta["variant_ids"]

    spec = {s["name"]: s for s in manifest.load()}  # canonical name -> manifest entry

    by_variant = defaultdict(dict)   # algo -> {variant: score}
    npss = defaultdict(list)
    nps_unit = {}
    for r in rows:
        a = r["algo"]
        if r.get("nps"):
            npss[a].append(r["nps"])
            if r.get("nps_unit"):
                nps_unit[a] = r["nps_unit"]
        if r.get("score") is not None:
            by_variant[a][r["variant"]] = r["score"]

    algos = []
    for a in by_variant:
        if a not in spec:
            raise SystemExit(
                f"algo '{a}' in results.jsonl has no entry in solvers.toml. "
                f"Add a [[solver]] entry for it.")
        s = spec[a]
        variants = [by_variant[a].get(v) for v in variant_ids]
        present = [x for x in variants if x is not None]
        nps_med = int(st.median(npss[a])) if npss.get(a) else None
        algos.append({
            "algo": manifest.site_name(s),
            "family": s["family"],
            "mean": round(st.mean(present), 1),
            "best": max(present),
            "worst": min(present),
            "std": round(st.pstdev(present), 1) if len(present) > 1 else 0.0,
            "nps": nps_med,
            "npsUnit": nps_unit.get(a, s["nps_unit"]),
            "variants": variants,
            # "contender" = competes on score, earns a headline-leaderboard row;
            # rows without a role are the CSP-preset study (their own page).
            **({"role": s["role"]} if s.get("role") else {}),
        })

    # leaderboard order: best mean first (matches make_report.py ranking)
    algos.sort(key=lambda r: -r["mean"])

    # The "throughput != score" paradox tiles compare two named rows. Which rows
    # is a manifest fact (the `paradox` tag), not a component hard-code: emit the
    # SITE names here so the component never string-matches an algo name.
    paradox = {}
    for s in spec.values():
        tag = s.get("paradox")
        if tag:
            paradox[tag] = manifest.site_name(s)

    out = {
        "budgetS": meta["budget_s"],
        "maxScore": MAX_SCORE,
        "community": COMMUNITY_RECORD_5CLUE,
        "variantIds": variant_ids,
        "paradox": paradox,
        "algos": algos,
    }
    Path(args.out).write_text(json.dumps(out, indent=1) + "\n")
    print(f"wrote {len(algos)} algos -> {args.out}")
    for r in algos:
        print(f"  {r['mean']:6}  {r['algo']:24} ({r['family']})")


if __name__ == "__main__":
    main()
