---
id: parity-defect-floor
title: The parity gap at 479 and the defect floor of Eternity II
summary: A per-color parity invariant of the official tile set makes a score of 479/480 impossible, forces a minimum defect of 2 for every non-solution, and gives every solution about 154 single-move neighbours at 478 (125 near-twin swaps and 29 turns in place), a floor and not a ceiling.
status: draft
created: 2026-07-22
updated: 2026-11-13
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

> **Status:** draft · **Created:** 2026-07-22 · **Updated:** 2026-11-13
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
defects are reached from any solution by two kinds of single move:
swapping two tiles of the same kind that agree on three of their four sides
once aligned, or turning in place an interior tile with the right internal
repetitions. The bag holds 125 such near-twin pairs (50 interior, 73 edge, 2
corner) and 23 interior tiles that can be turned this way, 3 of them in both
a half turn and a quarter turn, for 29 distinct turns. Each of these 154
moves lands on 478 from every solution (a swapped pair sitting side by side
across its differing side is the one exception), so every solution has
about 154 neighbours at 478 one move away from these two kinds of move alone.
This is a floor: other moves, such as adjacent swaps that change the shared colour, can add more.

An earlier version of this write-up summed 50 interior pairs, 23 half-turn
tiles and 3 quarter-turn tiles to 76 and called it an upper bound on the
defect-2 neighbours of any solution. That reading is withdrawn: it left out
the edge and corner pairs and the second direction of each quarter turn, and
a count of moves that always work bounds the neighbours from below, not from
above.

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
4/56/196 corner/edge/interior split; the count of near-twin pairs of each kind
(two tiles agreeing on three sides once one is turned to keep its gray sides
where the other has them); the count of interior tiles whose half-turn or
quarter-turn breaks exactly two joints, and the turns they give;
the absence of colors 1 to 5 on interior tiles; and the inward demand vector
of the 56 edge tiles. It emits a JSON record with each computed value beside
its expected value and an overall pass flag.

The parity theorem and the defect floor are proofs from these censused
properties, not searches, so the reproduction is exact and runs in well under
a second. See `compute/PLAN.md` for the claim-by-claim mapping, the scoring
convention, and the exact steps. The checker uses the starter kit's census
helpers (`e2_kit::analysis::color_half_edge_census`, `orbit_census`,
`piece_classes`) for the shared measurements and computes the single-move and
frame censuses directly.

Measured on the official instance (all 21 checks pass, `all_ok: true`, and
the JSON output is byte-stable across runs; the 2026-11-13 correction of the
single-move census was recomputed independently by the stdlib script
`compute/scripts/defect2_moves.py`, output committed beside it as
`defect2_moves.out`):

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
| Near-twin pairs, interior / edge / corner | 50 / 73 / 2 | 50 / 73 / 2 |
| Near-twin swaps | 125 | 125 |
| Interior half-turn defect-2 tiles | 23 | 23 |
| Interior quarter-turn defect-2 tiles | 3 | 3 |
| Interior turning tiles (union) | 23 | 23 |
| Interior turns to defect 2 | 29 | 29 |
| Single-move floor at 478 | 154 | 154 |
| Interior tiles carrying colors 1-5 | 0 | 0 |
| Frame-color half-edges | 120 | 120 |
| Ring joints per frame color | 12 x 5 | 12 x 5 |
| Inward demand vector (colors 6-22) | 4,5,3,3,1,1,2,3,4,6,4,2,3,6,4,3,2 | identical |
| Inward demand total | 56 | 56 |

The optional small-N score-spectrum probe mentioned in the plan is not part
of the claim and was not run; the exact-tier census above covers every cited
number.

## Notes / caveats

The 154 is a floor on the single-move neighbours at 478 around any
solution, not a ceiling. The exact number for a specific solution depends on
which twins sit side by side and on moves outside the census (adjacent swaps
among them); it requires an explicit 480 board and remains open. The parity argument applies to legal full placements, meaning
all 256 tiles placed with all border-facing edges gray; partial boards and
frame-illegal boards are outside its scope.
