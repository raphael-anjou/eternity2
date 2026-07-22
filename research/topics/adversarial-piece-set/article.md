---
id: adversarial-piece-set
title: The Eternity II piece set is adversarial by design
summary: Structural measurements of the official 256-piece set show no rotation symmetries, only 5 twin multisets, a fixed-orientation matching cap of 307 of 480, and a colour budget with zero slack.
status: draft
created: 2026-07-22
updated: 2026-07-22
contributors:
  - name: Raphael Anjou
    role: author
credits:
  - Structural analysis of the canonical piece set, building on the Selby and Riordan colour-allocation description.
tags:
  - structure
  - piece-set
  - symmetry
sources:
  - label: Selby and Riordan on the Eternity II design
    url: https://www.archduke.org/eternity/
reproduce:
  - cd research/topics/adversarial-piece-set/compute && cargo run --release > ../results/invariants.json
results:
  - label: Piece-set invariants JSON
    path: results/invariants.json
site:
  render: false
  dataFile: null
---

# The Eternity II piece set is adversarial by design

> **Status:** draft · **Created:** 2026-07-22 · **Updated:** 2026-07-22
> **Authors:** Raphael Anjou

## Summary

The official 256-piece Eternity II set is not a random draw from the space of
16 by 16 edge-matching puzzles. Direct measurement of the piece set shows that
every structural shortcut a solver might hope to exploit has been engineered
away. This topic collects the piece-set-level measurements that support that
claim, each one computable in under a second from the official instance.

Four independent measurements are reproduced here. First, all 256 pieces have
a full rotation orbit of size 4: no piece is invariant under any rotation, so
symmetry reduction over piece orientations gains nothing, and the puzzle
genuinely has 1024 distinct piece-rotations. Second, the unordered edge-colour
quadruples of the 256 pieces form 251 distinct multisets: only 5 pairs of
pieces share a colour budget, out of 32640 possible pairs. Third, if every
piece is frozen in its published orientation, a counting bound caps the number
of matchable interior edges at 307, far below the geometric maximum of 480;
rotation is structurally necessary, not merely convenient. Fourth, every
interior colour appears on an even number of piece sides and the per-colour
pairing capacities sum to exactly 480: the colour supply is exactly sufficient
for a perfect board, with zero slack in any colour.

Together these measurements say the hardness of Eternity II is deliberate and
lives in the combinatorial constraint structure, not in any supply imbalance
or hidden symmetry. Related evidence at the level of solution basins, such as
the indecomposability of transitions between high-scoring boards, is out of
scope for this topic because it depends on specific record boards rather than
on the piece set alone; only the piece-set-level claims are reproduced here.

## Reproduction

The `compute/` crate is a self-contained Rust binary that depends on the
research starter kit for the official instance and recomputes each number from
the raw piece set. It reports, as one JSON document: the rotation-orbit size
census, the multiset-twin pairs, the near-twin census (pairs sharing 3 of 4
same-position edges in the stored orientation), the fixed-orientation matching
cap, and the per-colour side counts with their parity and pairing-capacity
sum. Expected values are asserted in the binary, so a run is also a check.
The orbit census and the per-colour half-edge census come from the kit's
`analysis` module; the positional measurements (near-twins and the
fixed-orientation cap) are computed locally because they depend on each
piece's stored orientation, which the kit's rotation-invariant helpers
deliberately abstract away.

Measured on the bundled official instance (256 pieces, no hints); the run is
deterministic, completes in well under a second, and the committed
`results/invariants.json` is byte-stable across runs.

| Claim | Expected | Measured | Match |
|---|---|---|---|
| A1 pieces with full rotation orbit | 256 of 256 | 256 of 256 (0 half, 0 fixed) | yes |
| A1 distinct piece-rotations | 1024 | 1024 | yes |
| A1b distinct edge-colour multisets | 251 | 251 | yes |
| A1b twin pairs | 5 | 5: (2,3), (5,14), (7,51), (109,110), (171,181) | yes |
| A1c near-twin pairs / groups | 114 / 79 | 114 / 79 | yes |
| A2 fixed-orientation matching cap | 307 of 480 | 307 of 480 | yes |
| A3 colours with odd side count | 0 | 0 (22 interior colours) | yes |
| A3 pairing-capacity sum | 480 | 480 | yes |

The five twin pairs match the vault's research-numbering list exactly, so the
bundled instance stores the same canonical rotations and id order as the
research CSV; the orientation and numbering caveats in `compute/PLAN.md` do
not bite here. The axis-4 spectral measurements (algebraic connectivity,
spectral gap) remain deferred as planned and are not claimed by this topic.

See `compute/PLAN.md` for the exact claim being tested, the scoring-convention
mapping, and the known orientation and numbering caveats.
