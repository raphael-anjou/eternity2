# Reproduction plan — parity-defect-floor

> **Repro tier: exact.**
> exact = re-run reproduces committed results byte-for-byte. seeded-statistical =
> committed results are the record of our run (hardware noted in the article);
> reproduction = fresh run agreeing within the stated tolerance. qualitative =
> the effect (sign/ordering/band) reappears; exact digits are lineage-bound.

Source of the claim: vol-226 EXT write-up in the research vault
(`vault/papers/vol-226/EXT.md`), theorems 1, 1a, 1b, 2, 3, 5 and the bag
censuses backing them.

## (a) The exact claim, with expected numbers

All numbers below are cited from the vault source and are properties of the
official 256-tile bag.

1. **Half-edge statistics** (EXT section 1, `verify_stats.py`):
   h(0) = 64 (border gray), h(1..5) = 24 each, h(6..10) = 48 each,
   h(11..22) = 50 each. Colored total = 960 = 2 x 480 (the exact-fit
   identity). Every h(c) with c >= 1 is even.
2. **No self-symmetric tile** (EXT, `selfsym.py`): no tile is fixed by any
   nontrivial rotation; every tile has 4 distinct orientations. Tile classes:
   4 corners, 56 edge tiles, 196 interior tiles.
3. **Theorem 1 / 1a (the 479 gap)**: in any legal placement the per-color
   one-sided joint count chi(c) is even for every c >= 1, because
   h(c) = 2 x (monochrome-c joints) + chi(c). A defect of exactly 1 would
   make chi(a) = chi(b) = 1 for the two colors on the broken joint. Hence
   score 479/480 is impossible. This is a proof from item 1's evenness; the
   checker re-establishes the premise (all h(c) even), not the deduction.
4. **Theorem 2 (defect floor = 2)**: for a bag with no self-symmetric tile,
   every non-solution has defect >= 2. Premise re-established by item 2.
5. **Theorem 3 (defect-2 generator census, <= 76 = 50 + 23 + 3)** (EXT,
   `final_bounds.py`, `e2_twins.py`):
   - **50** interior near-twin pairs: unordered pairs of interior tiles that
     in some orientations agree on exactly 3 of 4 edge positions.
   - **23** interior tiles whose half-turn (180 degrees) breaks exactly 2
     joints: exactly one of N=S, E=W holds.
   - **3** interior tiles whose quarter-turn (90 or 270 degrees) breaks
     exactly 2 joints: exactly two edge positions fixed by the turn.
6. **Theorem 5 (frame decomposition)** (EXT, `frame_colors.py`,
   `ring_decomp.py`): colors 1-5 appear on **zero** interior tiles; their
   half-edge total is 120 = 2 x 60, saturating the 60 ring joints at 12 per
   color. The 56 inward frame edges (edge opposite the gray edge of each
   edge tile) have the fixed demand vector

   | color | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 |
   |---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
   | count | 4 | 5 | 3 | 3 | 1 | 1 | 2 | 3 | 4 | 6 | 4 | 2 | 3 | 6 | 4 | 3 | 2 |

   (sum = 56).

## (b) Scoring-convention mapping

The kit's `score_cells` / `score_board` count **matched internal joints,
rim-excluding**, out of `MAX_SCORE_16 = 480` on the official board. The
finding's "score" is exactly this convention: defect M = 480 - score, and
the claim "479 is impossible" is a claim about this scorer's output on
**legal full placements** (all 256 tiles placed once each, every
border-facing edge gray). No strict-5/5 hint condition is involved anywhere
in the claim; hints do not enter the parity argument. One care point: the
kit scorer does not itself enforce frame legality (gray-out edges). A board
that puts colored edges on the rim is outside the theorem's hypothesis, and
479 under the raw scorer is not excluded for such illegal boards. The
checker therefore verifies bag statistics, and any future placement-level
spectrum test must filter to frame-legal boards.

## (c) Scale-faithfulness

Two different regimes, both faithful:

- The **structural theorems** (parity, 479 gap, floor 2) hold for any bag
  with all-even colored half-edge counts and no self-symmetric tile. The
  vault source itself verified them computationally on planted 8x8 and 9x9
  symmetry-free instances (2400 permuted placements, 0 parity violations;
  score-spectrum table with the unique gap at M = 1 on 6/6 instances). Any
  kit-generated instance automatically has all h(c) even, because generated
  bags are read off a solved board where every colored half-edge is paired.
  So a small-N spectrum check is meaningful and cannot falsely refute,
  provided the generated bag is checked for the no-self-symmetric-tile
  property first (the kit generator aims for distinct-up-to-rotation, which
  is stronger, but verify with `pieces_distinct_up_to_rotation`).
- The **censuses** (50/23/3, the inward demand vector, the 24/48/50 h(c)
  profile) are facts about the official bag only and must be reproduced on
  the canonical 16x16 instance. That is what the checker does.

## (d) Exact reproduction steps

1. `cd research/topics/parity-defect-floor/compute`
2. `cargo run --release > ../results/census.json`
   (runtime well under 1 second; the program exits nonzero if any computed
   value disagrees with the expected value cited above)
3. Inspect `../results/census.json`: every entry carries `computed`,
   `expected`, and `ok`; the top-level `all_ok` must be `true`.

Optional extension (not required for the claim, not yet implemented): a
small-N spectrum probe that generates a solved kit instance, applies all
single rotations and swaps, and confirms defect 1 never occurs while
defects 2..8 all do, mirroring the vault's `score_spectrum.py` table. This
stays under a minute at 8x8.
