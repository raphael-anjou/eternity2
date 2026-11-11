#!/usr/bin/env python3
"""Two checks on the committed grid, from results/ only (no engine run).

1. Contender separation: per-variant scores of `verhaard` and `blackwood`,
   medians, and a paired bootstrap 95% CI on the mean and median gap.
2. Preset identity: which CSP presets emitted the same board on every variant
   (board field compared, solver name ignored).

Writes results/contender_stats.json. Deterministic (fixed RNG seed).
"""
import json
import random
import statistics as st
from pathlib import Path

HERE = Path(__file__).resolve().parent.parent
RES = HERE / "results"
B = 10000


def scores(algo):
    rows = [json.loads(l) for l in open(RES / "results.jsonl")]
    by = {r["variant"]: r["score"] for r in rows if r["algo"] == algo}
    return [by[v] for v in sorted(by)]


def boot(xs, f, rng):
    vals = sorted(f([rng.choice(xs) for _ in xs]) for _ in range(B))
    return [vals[int(0.025 * B)], vals[int(0.975 * B)]]


def board(algo, v):
    return json.load(open(RES / "boards" / f"{algo}__v{v:02d}.json"))["board"]


def main():
    rng = random.Random(1)
    v, b = scores("verhaard"), scores("blackwood")
    d = [x - y for x, y in zip(v, b)]
    out = {
        "n_variants": len(v),
        "seeds_per_variant": 1,
        "verhaard": {"scores": v, "mean": st.mean(v), "median": st.median(v)},
        "blackwood": {"scores": b, "mean": st.mean(b), "median": st.median(b)},
        "paired_gap": {
            "per_variant": d,
            "verhaard_ahead": sum(x > 0 for x in d),
            "blackwood_ahead": sum(x < 0 for x in d),
            "mean": st.mean(d),
            "mean_ci95": boot(d, st.mean, rng),
            "median": st.median(d),
            "median_ci95": boot(d, st.median, rng),
        },
        "identical_presets": {},
    }
    groups = [
        ["border_first_lcv", "rare_color_first"],
        ["gacolor_ac3", "gacolor_ac3_ns1", "verhaard_preferred"],
        ["joe_depth150", "joe_depth150_bp"],
    ]
    for g in groups:
        same = all(
            all(board(a, i) == board(g[0], i) for a in g[1:]) for i in range(10)
        )
        out["identical_presets"][" = ".join(g)] = same
    (RES / "contender_stats.json").write_text(json.dumps(out, indent=2) + "\n")
    print(json.dumps(out, indent=2))


if __name__ == "__main__":
    main()
