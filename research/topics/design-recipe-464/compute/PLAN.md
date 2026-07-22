# Reproduction plan: design-recipe-464

> **Repro tier: exact.**
> exact = re-run reproduces committed results byte-for-byte. seeded-statistical =
> committed results are the record of our run (hardware noted in the article);
> reproduction = fresh run agreeing within the stated tolerance. qualitative =
> the effect (sign/ordering/band) reappears; exact digits are lineage-bound.

Class: analysis (a measurement over fixed public boards; no search is run).

## (a) The exact claim, with expected numbers

Source: research repo, `vault/concepts/modified-blackwood-dfs-basin-discovery.md`
(vol-220, 2026-07-10) and `vault/concepts/community-464-basins.md`.

Claim: the July 2026 community 464 boards were produced by a Blackwood-style
break-scheduled DFS whose primary basin selector is **scan direction**. The
decisive evidence is the break geometry of the boards themselves:

| Board | Score | Breaks | Break rows | Double-break cells |
|---|---|---|---|---|
| b0 (464, basin C) | 464 | 16 | r0..r5 (top band) | 1 |
| b1 (464, basin B) | 464 | 16 | r1..r5 (top band) | 2 |
| b2 (464, basin B) | 464 | 16 | r1..r5 (top band) | 0 |
| public 460 (basin A) | 460 | 20 | r12..r15 (bottom band) | 3 |

Supporting facts to reproduce alongside:

- Break count identity: breaks == 480 - score for every full board.
- Break grid: the 464 (basin C) is perfectly clean on rows 6..15; the public
  460 is perfectly clean on rows 0..11. Mirror images.
- Basin map (pairwise tile-Hamming, piece id differs): b0 vs b1 = 250,
  b0 vs b2 = 250, b1 vs b2 = 6, each 464 vs public 460 = 243..247.
  Three mutually near-orthogonal basins: A = public 460, B = {b1, b2}, C = {b0}.
- Exact break cell indices for basin C (row-major attribution):
  [12, 27, 31, 38, 40, 44, 46, 53, 58, 62, 63, 71, 73, 77, 93] (rows 0..5),
  disjoint from the classic Blackwood 469 break-depth set (rows 12..15).
- All five boards are complete (256 placed) and strict-5/5 clue legal.
- b2 reaches 464 with zero double-break cells, so an at-most-one-break-per-cell
  DFS suffices to represent basin B.

Interpretation being packaged (the recipe, cited from the vault page): scan
direction / bidirectional construction is the primary basin selector (vault
hypothesis M-C, confirmed by the break-grid measurement); per-restart seed
diversity is the secondary within-direction diversifier (M-A); the break budget
lives in the terminal (last-placed) band, so reversing the scan moves the break
positions from the bottom rows to the top rows; anytime capture of the best
complete board across restarts is required to bank the record.

## (b) Scoring-convention mapping

The kit's `score_cells` is the canonical rim-excluding matched-edges scorer:
a right or down adjacency scores 1 iff the facing edges are equal and not the
border colour. Max = 480 on 16x16. This is exactly the convention of every
number above (464/480, 460/480), so no mapping is needed for scores.

Two derived conventions used by the claim:

- **Breaks** = 480 - score on a complete board (equivalently, mismatched
  interior adjacencies). The kit's `SolveOutput.breaks` already computes this.
- **Strict-5/5** = matched-edges score AND all five official clue pieces at
  their published cells with their published rotations. The kit's
  `official_instance(true)` carries the five hints; the checker verifies each
  hint against the recovered placement. All five boards must pass 5/5.
- **Break attribution** (for rows / double-break counts): each mismatched
  interior adjacency is attributed to the row-major-later cell of the pair
  (the right cell of a horizontal adjacency, the lower cell of a vertical
  one). Equivalently: each cell counts its UP and LEFT mismatches. This is the
  convention of the research repo's `verify_bucas` tool that produced the
  numbers in (a); the checker implements the same rule. Rim edges (border
  colour on the outside) are excluded, matching the scorer.

## (c) Scale faithfulness

This is an analysis of five specific, published 16x16 boards against the one
official piece set. It only makes sense on the canonical 16x16 instance: the
break-band geometry is a property of those exact boards, and the basin-map
distances are between those exact boards. There is no small-N surrogate to run
and none is needed; the computation is a few milliseconds per board.

(A follow-up *solver* experiment suggested by the vault page, running a
bottom-to-top break-scheduled DFS to reach a 464-class board, WOULD be
scale-sensitive: the vault's n13-solvability-cliff work shows small-N ladders
stop being representative above N = 12, and the compute regime of
Blackwood-class DFS record runs is documented as 3 to 4 orders of magnitude
beyond an 8-core box. That experiment is out of scope for this topic's
compute; this topic packages the measurement that motivates it.)

## (d) Exact reproduction steps

1. Inputs: `compute/boards.txt`, five labelled viewer URLs (tab separated:
   label, URL). These are the canonical re-encodings of the three 464 boards
   posted to groups.io (thread "Record of Eternity2 with 5 hints?",
   #11906-11919) and the two public 460 postings, as verified in the research
   repo's community-corpus audit (`solver_audit_20260711_community`,
   canonical_boards 464_* and 460_*). Anyone can substitute the URLs straight
   from the forum posts; the checker rescores from scratch either way.
2. Run:

   ```sh
   cd research/topics/design-recipe-464/compute
   cargo run --release > ../results/break-geometry.json
   ```

3. The program, per board: decode `board_edges`; rescore with the canonical
   scorer; recover the placement against `official_instance(true)` (piece set
   is distinct up to rotation, so edges determine placement); check the five
   clues; compute per-cell break counts under the attribution rule of (b),
   per-row totals, double-break cells, and break cell indices. Then the
   pairwise tile-Hamming matrix over the five placements.
4. Accept iff the output matches every number in (a): scores (464, 464, 464,
   460, 460), breaks (= 480 - score), break rows confined to r0..r5 for the
   464s and r12..r15 for the 460(s), double-break counts (1, 2, 0 for
   b0, b1, b2), the basin-C break-index list, 5/5 hints on all boards, and the
   Hamming matrix (250 / 250 / 6, cross distances 243..247).

Expected runtime: under 5 seconds total (IO plus five 256-cell scans).

## Status of this skeleton

The checker builds and is complete; it has not been run to write `results/`
yet (that is the publish step). No solver code ships in this topic.
