#!/usr/bin/env python3
"""Fold the community-engine run results into the DFS-study site data.

Reads the two results JSONL files written by run_community.sh, aggregates the
per-(engine,instance) rows, and:

  1. Rewrites results/community.json — the `engines` array (now measured on the
     pinned grid, with the collapse backed by a real rescored run) plus a new
     `unpinned` block for the centre-clue head-to-head.
  2. Patches web/src/data/dfs-study.json in place: replaces `community` and adds
     the top-level `unpinned` block. The existing `variants` (pinned from-scratch
     leaderboard) are left untouched.

Every score here is the canonical rescore emit_result.py produced; nothing is
copied from an engine self-report. Throughput is a per-engine string labelled by
its native unit and is never turned into one cross-engine axis.
"""
import json
import os
import statistics

HERE = os.path.dirname(os.path.abspath(__file__))
DFS = os.path.abspath(os.path.join(HERE, ".."))
REPO = os.path.abspath(os.path.join(DFS, "..", "..", ".."))
COMMUNITY_JSON = os.path.join(DFS, "results", "community.json")
SITE_JSON = os.path.join(REPO, "web", "src", "data", "dfs-study.json")

UNPINNED = os.path.join(HERE, "results-unpinned.jsonl")
PINNED = os.path.join(HERE, "results-pinned-collapse.jsonl")


def load(path):
    if not os.path.exists(path):
        return []
    return [json.loads(ln) for ln in open(path) if ln.strip()]


def fmt_tp(nps, unit):
    if nps is None:
        return None
    if nps >= 1_000_000:
        return f"{nps / 1_000_000:.0f}M {unit}"
    if nps >= 1_000:
        return f"{nps / 1_000:.0f}k {unit}"
    return f"{nps} {unit}"


def by_engine(rows):
    d = {}
    for r in rows:
        d.setdefault(r["engine"], []).append(r)
    return d


def main():
    un = load(UNPINNED)
    pin = load(PINNED)
    un_by = by_engine(un)
    pin_by = by_engine(pin)

    # ---- Unpinned head-to-head rows -------------------------------------------
    # Display metadata per engine (family/kind/note), keyed by the emit label.
    META = {
        "mcgavin-c": dict(display="McGavin (C)", family="community", kind="foreign",
                          instance="centre clue (seed-invariant, one run)",
                          note="Its codegen backtracker specialises to the hint set; "
                               "with only the centre clue it fills deep. Optimises depth, "
                               "not matched-edge score."),
        "blackwood-cs": dict(display="Blackwood (C#)", family="community", kind="foreign",
                             instance="centre clue (unseeded, one run)",
                             note="Break-DFS. Reaches its plateau inside 60 s on a single "
                                  "centre clue; run to run it lands on the same depth."),
        "verhaard-reimpl": dict(display="Verhaard (our reimpl)", family="community", kind="reimpl",
                                instance="centre clue, 3 seeds",
                                note="Our comb-order reimplementation of the Verhaard search "
                                     "(the original binary is Win32-only). Runs on the JSON "
                                     "instance directly."),
        "break-2": dict(display="break-2 (ours)", family="break", kind="engine",
                        instance="centre clue, 3 seeds",
                        note="The DFS study's strongest break-family engine, as the "
                             "from-scratch baseline the community engines are measured "
                             "against on the same grid."),
    }
    order = ["break-2", "verhaard-reimpl", "mcgavin-c", "blackwood-cs"]
    unpinned_rows = []
    for eng in order:
        rows = un_by.get(eng, [])
        if not rows:
            continue
        scores = [r["score"] for r in rows]
        depths = [r["max_depth"] for r in rows if r.get("max_depth") is not None]
        placeds = [r["placed"] for r in rows if r.get("placed") is not None]
        npss = [r["nps"] for r in rows if r.get("nps") is not None]
        m = META[eng]
        unit = rows[0].get("nps_unit", "search-nodes/s")
        row = {
            "name": eng,
            "display": m["display"],
            "family": m["family"],
            "kind": m["kind"],
            "instance": m["instance"],
            "n": len(rows),
            "score": round(statistics.mean(scores), 1) if len(scores) > 1 else scores[0],
            "best": max(scores),
            "worst": min(scores),
            "placed": max(placeds) if placeds else None,
            "max_depth": max(depths) if depths else None,
            "throughput": fmt_tp(round(statistics.median(npss)) if npss else None, unit),
            "nps_unit": unit,
            "note": m["note"],
        }
        unpinned_rows.append(row)

    seeds_used = sorted({r["seed"] for r in un if r.get("seed") is not None})
    unpinned_block = {
        "hint": "centre clue only (piece 139 @ I8)",
        "seeds": seeds_used,
        "budget_s": 60,
        "max_score": 480,
        "rows": unpinned_rows,
    }

    # ---- Pinned-collapse rows (the existing community array, now measured) -----
    def collapse_stat(eng):
        rows = pin_by.get(eng, [])
        if not rows:
            return None
        return {
            "score": min(r["score"] for r in rows),
            "max_depth": min(r["max_depth"] for r in rows if r.get("max_depth") is not None) if any(r.get("max_depth") is not None for r in rows) else None,
            "instances": [r["instance"] for r in rows],
            "nps": statistics.median([r["nps"] for r in rows if r.get("nps") is not None]) if any(r.get("nps") is not None for r in rows) else None,
            "nps_unit": rows[0].get("nps_unit"),
        }

    mcg_pin = collapse_stat("mcgavin-c")
    bw_pin = collapse_stat("blackwood-cs")
    # Unpinned single-run community scores for the narrative on each card.
    mcg_un = un_by.get("mcgavin-c", [{}])[0]
    bw_un = un_by.get("blackwood-cs", [{}])[0]

    engines = [
        {
            "name": "mcgavin-c",
            "display": "MCGAVIN-C",
            "language": "C",
            "instance": "centre clue (unpinned grid)",
            "score": mcg_un.get("score"),
            "max_depth": mcg_un.get("max_depth"),
            "throughput": fmt_tp(mcg_un.get("nps"), mcg_un.get("nps_unit", "tiles/s")),
            "note": "Codegen strict/break backtracker. On the centre-clue grid it reaches "
                    f"depth {mcg_un.get('max_depth')} ({mcg_un.get('score')} matched edges, "
                    "canonically rescored) in 60 s. Optimises depth, not score.",
            "collapse": (
                f"Adding the study's three corner pins collapses its fixed scan path to depth "
                f"{mcg_pin['max_depth']} (canonical score {mcg_pin['score']}), confirmed on "
                f"{len(mcg_pin['instances'])} pinned variants: the scan never reaches a corner "
                "early, so a pinned corner dead-ends it at once."
            ) if mcg_pin else None,
            "collapse_score": mcg_pin["score"] if mcg_pin else None,
            "collapse_depth": mcg_pin["max_depth"] if mcg_pin else None,
        },
        {
            "name": "blackwood-cs",
            "display": "BLACKWOOD-CS",
            "language": "C#",
            "instance": "centre clue (unpinned grid)",
            "score": bw_un.get("score"),
            "max_depth": bw_un.get("max_depth"),
            "throughput": fmt_tp(bw_un.get("nps"), bw_un.get("nps_unit", "search-nodes/s")),
            "note": "Break-DFS (net8.0 retarget, one thread; search unchanged). On the "
                    f"centre-clue grid it reaches depth {bw_un.get('max_depth')} "
                    f"({bw_un.get('score')} matched edges) in 60 s.",
            "collapse": (
                f"It hardcodes its piece set and scan, so it cannot express the study's "
                f"arbitrary corner pins; the matching constrained test is the five official "
                f"clues, where its heuristic phase thrashes to depth {bw_pin.get('max_depth')} "
                f"(canonical score {bw_pin['score']}) in 60 s."
            ) if bw_pin else None,
            "collapse_score": bw_pin["score"] if bw_pin else None,
            "collapse_depth": bw_pin["max_depth"] if bw_pin else None,
        },
    ]

    community_doc = {
        "_comment": "Measured on the M1 (single core, 60 s). Every score is canonically "
                    "rescored from the engine's own board output (community/canonical_rescore.py, "
                    "validated to reproduce the known 469 board); engine self-reports are not "
                    "trusted. Throughput is per-engine and never cross-compared. Rerun with "
                    "community/run_community.sh then community/build_site_data.py.",
        "engines": engines,
        "unpinned": unpinned_block,
    }
    with open(COMMUNITY_JSON, "w") as fh:
        fh.write(json.dumps(community_doc, indent=2) + "\n")
    print(f"wrote {COMMUNITY_JSON}")

    # ---- Patch the site JSON in place -----------------------------------------
    site = json.load(open(SITE_JSON))
    site["community"] = engines
    site["unpinned"] = unpinned_block
    with open(SITE_JSON, "w") as fh:
        fh.write(json.dumps(site, indent=1) + "\n")
    print(f"patched {SITE_JSON}: community ({len(engines)}) + unpinned ({len(unpinned_rows)} rows)")


if __name__ == "__main__":
    main()
