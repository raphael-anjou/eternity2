# Approach pricing: what each family of approaches could save against the row-scan tree

Desk studies and small computations pricing families of approaches against the full row-scan search tree. Results on generated boards are toys, never Eternity II progress. Expected solutions: about 14,702 with the starter clue, about 1 with all five.

## The question and the fixed numbers

Can any family of approaches remove the roughly 56 to 73 bits between the row-scan tree and what is affordable?

- Row-scan tree, real pieces, five clues: 2^134.4 nodes (10^40.457), measured on the real pieces.
- Affordable: about 10^15 nodes (about 2^50) for a modest run, 2^61.5 for 1,000 core-years at 1e8 nodes per second, 2^78 for a very large distributed effort. Gap 56 to 73 bits against the last two.
- Reopening rule: a price of 10 bits or more and a falsifying experiment under a day.

## One row per family

Gains are in bits that survived the independent re-check (`-recheck.md` beside each note), on the row-scan tree unless stated.

| Family | Largest gain surviving re-check | Status | Notes |
|---|---|---|---|
| Search that learns | about 2 bits (ceiling 6.6); clause learning needs 2^5 to 2^6.5 fewer conflicts than tree nodes on 5x5 and 6x6 (two sizes) | "empty" unproven as written; holds for reasons attached to row-scan nodes | `gap-1-search-that-learns.md`, `-recheck` |
| Lossy representations | two-list join 3.0 bits (not 5.9); a sound sketch at most 2.0 | empty for colour-blind sketches and rectangular region lists; the 2.0 bound has no second reader | `gap-2-lossy-representations.md`, `-recheck` |
| Shape of the tree | 1 to 3 bits with oracle cutoffs; a loss when blind | empty for blind restarts and portfolios | `gap-3-shape-of-the-tree.md`, `-recheck` |
| Adaptive orders | under 3 bits; arc consistency 5.5 bits of nodes (costs about 13 bits per node); finishing rows one or two at a time at most 5.3 or 6.7 | empty for row-shaped rules; free smallest-domain is 13 to 49 bits worse | `gap-4-adaptive-orders.md`, `-recheck` |
| Tile-set structure | 0 bits before depth 192 | empty: no colour automorphism, invariant group Z^2 + Z/2 | `gap-5-tile-set-structure.md`, `-recheck` |
| Algebraic reformulation | none found among fifteen; degree-2 linearisation fixes 0 of 10 boards at 4x4 and 0 of 5 at 5x5 | not a barrier; degree 3 at 5x5 did not fit in memory; run not independently checked | `gap-6a-*.md`, `-recheck` |
| Search-space reformulation | row scan within 0.24 bit of the best static order | empty for static reformulations; no certificate | `gap-6b-reformulation-search-space.md`, `-recheck` |
| Far ideas (21, 18 and 32 listed in three studies) | none priced at 10 bits or more with a mechanism | some dismissals inherited from unchecked rows; four statements ruled wrong | `far-ideas-*.md`, `geometry-isolation.md`, `far-ideas-and-isolation-recheck.md` |
| Frame nogoods | about 17.4 bits of cheap information about the frame, all at the four clue corners | independently re-checked twice; interior under a frame that passes the corners costs about 10^13.6 nodes on average (replaces 10^8.3); far stratum undecided | `nogood-frame-search-price.md`, `-recheck` |


## The ceiling of a perfect free cut

If a check removed every wrong node from free depth d onward at no cost, it could save at most:

| Depth | 52 | 64 | 80 | 96 | 112 | 128 | 144 | 158 |
|---|---|---|---|---|---|---|---|---|
| Bits | 57.1 | 44.8 | 29.8 | 17.1 | 10.7 | 6.8 | 2.4 | 0.5 |

Verified: `python3 -I scripts/route-audit/strata_gain.py` prints these rows from the measured level profile (`five-np1e6.jsonl`, beside it) and the stratified-painter prices (23.6 bits for quadrants, derived, closed). Wrong-board tests bite late (arc consistency refutes 57 to 90 percent of sampled wrong prefixes at depths 96 to 128), so the largest gain priced is about 5.5 bits of nodes.

## Route audit and anomaly hunt

The route audit (`notes/route-audit-weak-kills.md`, not independently checked) examined 78 lines: 47 closures (32 sound, 10 weak and bearing on a solve, 5 weak but relevant to records only). Three reopenings, none above 1 percent odds of a solve:
1. Exactness of the convex lift at 6x6 (the only item priced above the gap if it held to 16x16).
2. Growth of non-tree solvers against the tree on the same boards (annealing, difference map, low-rank, message passing).
3. Learning solvers at 7x7 and on real-piece prefixes, with a propagation-only control.

The anomaly hunt (`notes/anomaly-hunt.md`, not independently checked) examined 51 candidates; 14 disagree with their model beyond noise at face value; none survives a painted-board null and a count of comparisons. The three-row staircase (+2.85 bits) is not reproduced at ten times the population (135.2 and 135.7 bits, above row scan): no gain shown.

## Tripwires: reopen the question if

1. A published method shows a class-level gain: price it against `routes.md` and the ceiling table first.
2. Tiles of the solution become available from outside the search: the pin study prices an extra pin at about 10 bits at best and about 6 at the median on painted boards, and a 10^15 run needs more than 10, probably 16 to 24, spread pins.
3. Hardware improves by 15 orders of magnitude or more (every credible multiplier together is about 2.5 orders).
4. A re-check or run overturns a closure: the convex lift is exact at 6x6; the far stratum of frame cores resolves below break-even; a non-tree solver's work grows by less than 0.75 of the tree's from 6x6 to 7x7; a learning solver's lead over propagation alone rises with size.

## Files

- `notes/`: the studies and their re-checks. `question-and-numbers.md`: the question, obstructions and numbers. `routes.md`: the index of routes tried. `surveys/`: eight field surveys plus citation and derivation re-checks.
- `scripts/<study>/`: code and small inputs. `results/<study>/`: raw output text and rows. The study names match the notes (`-review` folders belong to the re-checks).
- Skipped: five compiled binaries (sources present): `anomaly-hunt/dyn16`, `gap-6a-linearisation/xl_p` and `xl_2`, `geometry-isolation/iso`, `review-far-ideas/iso_ref`. Nothing exceeded 5 MB. Excluded: studies on the puzzle's construction and on relaxations (notes may still cite their names). One section of `far-ideas-different-question.md` and a few odds lines about contacting third parties were cut.

## Rerunning

Stdlib Python; run from the script's folder as `python3 -I <script>.py`, with `E2_CSV` set to `engine/data/official_eternity2.csv` where a script asks. Checked: `route-audit/strata_gain.py`, `route-audit/propagation_control.py`, `far-ideas-different-question/01_blocks_2x2.py`. Many others import tools from experiment folders not packaged here, or run for minutes to hours, and were not rerun. Recipe: `just experiments approach-pricing` (see `RECIPE.txt`).
