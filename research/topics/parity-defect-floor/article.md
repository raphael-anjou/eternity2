---
id: parity-defect-floor
title: The parity gap at 479 and the defect floor of Eternity II
summary: A per-color parity invariant of the official tile set makes a score of 479/480 impossible, forces a minimum defect of 2 for every non-solution, and bounds the number of defect-2 neighbours of any solution by 76.
status: draft
created: 2026-07-22
updated: 2026-07-22
contributors:
  - name: Raphael Anjou
    role: author
credits:
  - Vol-226 extremal-combinatorics study (research vault, papers/vol-226/EXT.md).
tags:
  - theory
  - parity
  - defect-geometry
sources:
  - label: Vol-226 EXT write-up (research vault)
    url: null
reproduce:
  - cd research/topics/parity-defect-floor/compute && cargo run --release > ../results/census.json
results:
  - label: Bag census JSON
    path: results/census.json
site:
  render: false
  dataFile: null
---

# The parity gap at 479 and the defect floor of Eternity II

> **Status:** draft · **Created:** 2026-07-22 · **Updated:** 2026-07-22
> **Authors:** Raphael Anjou

## Summary

The official Eternity II tile set carries 1024 half-edges. The border color
accounts for 64 of them, and every one of the 22 interior colors appears an
even number of times: five colors 24 times each, five colors 48 times each,
and twelve colors 50 times each, for a colored total of 960, exactly twice
the 480 internal joints. This evenness has a sharp consequence. In any legal
full placement, the number of joints showing a given color on exactly one
side is even, because each color's half-edges split into whole matched pairs
plus those one-sided occurrences. A placement scoring 479 would have a single
mismatched joint showing two colors once each, an odd count for both. So a
score of 479/480 is impossible, while every other near-optimal score is
attainable: the score ladder has exactly one gap.

The same evenness, combined with a second verified property of the set (no
tile is fixed by any rotation), forces a defect floor of 2: any legal
placement that is not a solution misses at least two joints. The minimal
defects are realized by a specific mechanism, swapping two near-twin tiles
that agree on three of their four edges, or turning in place a tile with the
right internal repetitions. Counting those opportunities in the bag gives at
most 50 twin-swap pairs, 23 half-turn tiles, and 3 quarter-turn tiles, so any
solution has at most 76 defect-2 neighbours reachable by a single swap or
rotation. Near-misses at the very top of the ladder are sparse, not abundant.

A frame-color argument rounds out the picture. Colors 1 to 5 appear on zero
interior tiles, so in every solution they saturate exactly the 60 border-ring
joints, 12 joints per color, and the 56 inward-facing edges of the frame
present a fixed color-demand vector to the interior that is determined by the
bag alone, before any search.

All of these numbers are properties of the published tile set. They are
computed here directly from the official instance and checked against the
claimed values.

## Reproduction

Every claim reduces to a census of the official bag. The checker in
`compute/` loads the official 256-piece instance through the starter kit and
verifies, in one pass: the half-edge count of each color and its evenness;
the colored total 960; the absence of rotationally self-symmetric tiles; the
4/56/196 corner/edge/interior split; the count of interior near-twin pairs
(two tiles agreeing on three edge positions in some orientations); the count
of interior tiles whose half-turn or quarter-turn breaks exactly two joints;
the absence of colors 1 to 5 on interior tiles; and the inward demand vector
of the 56 edge tiles. It emits a JSON record with each computed value beside
its expected value and an overall pass flag.

The parity theorem and the defect floor are proofs from these censused
properties, not searches, so the reproduction is exact and runs in well under
a second. See `compute/PLAN.md` for the claim-by-claim mapping, the scoring
convention, and the exact steps. The checker uses the starter kit's census
helpers (`e2_kit::analysis::color_half_edge_census`, `orbit_census`,
`piece_classes`) for the shared measurements and computes the defect-2 and
frame censuses directly.

Measured on the official instance (all 17 checks pass, `all_ok: true`, and
the JSON output is byte-stable across runs):

| Quantity | Expected | Measured |
|---|---|---|
| h(0) gray half-edges | 64 | 64 |
| h(1..5) | 24 each | 24 each |
| h(6..10) | 48 each | 48 each |
| h(11..22) | 50 each | 50 each |
| Colored total (exact-fit identity) | 960 | 960 |
| All colored h(c) even (479 gap premise) | true | true |
| Rotationally self-symmetric tiles | 0 | 0 |
| Corner / edge / interior split | 4 / 56 / 196 | 4 / 56 / 196 |
| Interior near-twin pairs | 50 | 50 |
| Interior half-turn defect-2 tiles | 23 | 23 |
| Interior quarter-turn defect-2 tiles | 3 | 3 |
| Defect-2 generator bound | 76 | 76 |
| Interior tiles carrying colors 1-5 | 0 | 0 |
| Frame-color half-edges | 120 | 120 |
| Ring joints per frame color | 12 x 5 | 12 x 5 |
| Inward demand vector (colors 6-22) | 4,5,3,3,1,1,2,3,4,6,4,2,3,6,4,3,2 | identical |
| Inward demand total | 56 | 56 |

The optional small-N score-spectrum probe mentioned in the plan is not part
of the claim and was not run; the exact-tier census above covers every cited
number.

## Notes / caveats

The 76 is an upper bound on defect-2 generators around any solution; the
number realized by a specific solution requires an explicit 480 board and
remains open. The parity argument applies to legal full placements, meaning
all 256 tiles placed with all border-facing edges gray; partial boards and
frame-illegal boards are outside its scope.
