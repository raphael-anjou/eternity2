# Reproduction plan: ring-purity

> **Repro tier: exact.**
> exact = re-run reproduces committed results byte-for-byte. seeded-statistical =
> committed results are the record of our run (hardware noted in the article);
> reproduction = fresh run agreeing within the stated tolerance. qualitative =
> the effect (sign/ordering/band) reappears; exact digits are lineage-bound.

Source of the claim: research repo, `research/vault/papers/vol-226/FLOW.md`
(vol-226 round 1, track FLOW), Section 6 "The Ring Purity Theorem" plus
Measurements 6.3 and 6.4. This plan cites the exact numbers the checker must
reproduce.

## (a) The exact claim and expected numbers

Deterministic facts about the official 256-piece list (FLOW.md Fact 1.1 and
Theorem 6.1):

| # | Fact | Expected value |
|---|------|----------------|
| 1 | Piece-type histogram by gray-slot count | 196 interior / 56 edge / 4 corner |
| 2 | Frame colors := colors absent from all interior pieces | exactly 5 colors |
| 3 | Half-edge count of each frame color | 24 each, total supply 120 |
| 4 | Non-frame color multiplicities | 5 colors at 48, 12 colors at 50 |
| 5 | Each edge piece: frame slots among its 3 non-zero slots | exactly 2, and the single non-frame slot is opposite the gray slot |
| 6 | Each corner piece: both non-zero slots frame-colored | 4/4, and the two gray slots are cyclically adjacent (Lemma 5.1) |
| 7 | Ring demand vs frame supply | 56x2 + 4x2 = 120 = 5x24, zero slack |
| 8 | Ring multigraph M (5 vertices, one edge per border piece) | 60 edges, every degree exactly 24, connected, hence Eulerian (Cor. 6.2) |
| 9 | Self-loop pieces in M | 14; distinct color-pairs spanned: 15 |
| 10 | Exact first-step branching (Meas. 6.3): distinct border pieces incident to each frame color | multiset {20, 21, 21, 22, 22}, mean 21.2/59, reduction 59/21.2 = 2.78x |

Statistical measure (Measurement 6.4, sequential importance sampling of a
greedy ring build, 1000 trials):

| # | Quantity | Expected value |
|---|----------|----------------|
| 11 | Completion rate of 59 greedy color-matching steps | about 9.4% (94/1000 in the source; a seeded rerun should land in the same few-percent band) |
| 12 | Mean log10 path probability over completed builds | -27.3 with sample sd about 0.8 |
| 13 | Uniform-ordering baseline | log10(1/59!) = -80.1 |

Facts 1-10 are exact: any deviation refutes the claim outright. Facts 11-12
are Monte Carlo estimates; reproduction means agreement within sampling error
(the source reports two independent batches consistent within noise), not
bit-identical values.

Caveat carried from the source: fact 10's per-color values depend on color
labeling, so the checker compares the sorted multiset, not the labeled table.
The source's open problem (exact undirected Eulerian-circuit count of M) is
NOT reproduced; the single-orientation BEST-theorem value 11 was documented as
an error in the source and must not be recomputed as a claim.

## (b) Scoring-convention mapping

This topic makes no score claim, so no strict-5/5 vs matched-edges mapping is
needed. For orientation: the kit's canonical `score_cells` counts the 480
internal edges of the 16x16 board and excludes the 64 perimeter (gray) rim
slots. The source uses the same convention (480 internal joints, perimeter
forced gray), so "border ring" here means the 60 outer cells and their 60
ring-internal joints, all of which ARE part of the kit's 480 scored edges.
The theorem constrains which colors can sit on those 60 ring joints (frame
colors only, all 120 half-edges consumed); it says nothing about score
totals.

Color labels: the checker derives the frame-color set from the data (colors
with zero occurrences on interior pieces) instead of assuming they are labeled
1..5, which makes it robust to any renumbering between the kit's
`official.json` and the source's piece list.

## (c) Scale-faithfulness

The theorem is a property of the official instance's piece design, not of
edge-matching puzzles in general. A generated small-N board is NOT a faithful
test: the kit's framed generator confines border colors to the frame band by
construction, so ring purity would hold there by fiat and verify nothing,
while an unframed generated board mixes frame and interior colors and would
falsely refute the claim. The source itself distinguishes instance facts
(Theorem 6.1 parts 1-3 are "direct exhaustive facts about the supplied
256-piece list") from generic laws it did test on synthetic boards (the
parity laws of its Section 3). Therefore the checker runs exclusively on the
canonical 16x16 official instance via `official_instance(true)`.

Optionally, a framed generated instance (`generate_framed`) can serve as a
positive control (purity holds by construction) and an unframed one as a
negative control (purity fails), but neither bears on the claim itself.

## (d) Exact reproduction steps

1. `cd research/topics/ring-purity/compute`
2. `cargo run --release > ../results/ring_purity.json` (creates `results/`
   next to `article.md`; runtime well under one second for the deterministic
   checks plus about a second for the 1000 SIS trials)
3. Compare the emitted JSON against the expected table above. Every field in
   the `deterministic` block must match exactly; the `sis` block must agree
   within sampling error (completion rate in the mid-single-digit to low-teens
   percent range, mean log10 near -27, baseline exactly -80.1).
4. The binary exits nonzero and prints the first failing check if any
   deterministic clause is violated.

RNG: the SIS uses the kit generator's `XorShift` with a fixed base seed, so
reruns are bit-reproducible; change `--seed` to draw an independent batch and
confirm batch-to-batch consistency the way the source did (200/500/300
split).

## Checker structure

`src/main.rs`, single file, no dependencies beyond `e2-kit` (path dep on the
starter kit) and `serde_json`:

- load `official_instance(true)`, classify pieces by `border_edge_count()`
- derive frame colors, build per-color half-edge histogram
- verify clauses 1-9, exact counts for clause 10
- run 1000 seeded SIS trials for clauses 11-13
- print one JSON object to stdout; nonzero exit on any deterministic failure
