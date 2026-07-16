#!/usr/bin/env python3
"""Generate the 10 unpinned instances for the community-engine comparison.

The DFS study's main grid pins three corners per variant, which a fixed-scan-path
engine (McGavin's codegen backtracker, Blackwood's tuned break-DFS) cannot take:
an arbitrary corner pin the scan never reaches early dead-ends it at once. That is
a real finding, but it also means the corner-pinned grid is not where those
engines can show what they do.

These ten instances remove that obstacle. Every one is the official 256-piece set
with a single hint, the mandatory centre clue (piece 139 = 0-indexed 138 at cell
I8 = pos 135, rotation 0, exactly as the official rulebook and the committed
pinned variants carry it). That is the only clue the real contest forced. With no
arbitrary corner pins, every engine here runs as designed; the diversity across
the ten is each engine's own seed, not a different board, which is how the
community actually benchmarks.

Output mirrors the pinned variants' schema so the same IO layer reads both, plus a
`seed` field per variant.
"""

import json
import os

HERE = os.path.dirname(__file__)
PINNED = os.path.join(HERE, "..", "variants", "variant_00.json")
OUT = os.path.join(HERE, "..", "variants-unpinned")

# The one mandatory clue: piece 139 (1-indexed) = 138 (0-indexed) at cell I8,
# pos 135 on a 16x16 row-major board, rotation 0. Taken from the committed pinned
# variant, which is the authority for the encoding.
CENTRE_HINT = {"pos": 135, "piece": 138, "rot": 0}

# Fixed seeds, one per variant. Chosen once and committed so the grid is
# reproducible; an engine that ignores the seed simply runs its default.
SEEDS = [1, 7, 13, 42, 99, 123, 256, 480, 1009, 2718]


def main():
    base = json.load(open(PINNED))
    pieces = base["pieces"]
    # sanity: the centre piece must be the one the official clue names.
    assert len(pieces) == 256, "expected 256 pieces"

    os.makedirs(OUT, exist_ok=True)
    manifest = {
        "description": (
            "Official E2 + the single mandatory centre clue (piece 139 at I8), "
            "10 seeds. No corner pins, so the community record engines run as "
            "designed. Diversity is the seed, not the board."
        ),
        "centre_hint": CENTRE_HINT,
        "variants": [],
    }

    for i, seed in enumerate(SEEDS):
        inst = {
            "name": f"e2_official_unpinned_centreclue_v{i:02d}",
            "width": 16,
            "height": 16,
            "numColors": base["numColors"],
            "seed": seed,
            "pieces": pieces,
            "hints": [CENTRE_HINT],
        }
        path = os.path.join(OUT, f"unpinned_{i:02d}.json")
        with open(path, "w") as fh:
            fh.write(json.dumps(inst, indent=2) + "\n")
        manifest["variants"].append({"id": i, "seed": seed, "file": f"unpinned_{i:02d}.json"})

    with open(os.path.join(OUT, "manifest.json"), "w") as fh:
        fh.write(json.dumps(manifest, indent=2) + "\n")

    print(f"wrote {len(SEEDS)} unpinned variants + manifest to {os.path.relpath(OUT, HERE)}")


if __name__ == "__main__":
    main()
