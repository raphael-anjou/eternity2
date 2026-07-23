# Reproduction plan: irreducible-hard-region

> **Repro tier: qualitative.**
> exact = re-run reproduces committed results byte-for-byte. seeded-statistical =
> committed results are the record of our run (hardware noted in the article);
> reproduction = fresh run agreeing within the stated tolerance. qualitative =
> the effect (sign/ordering/band) reappears; exact digits are lineage-bound.

Source of the claim: research repo,
`research/vault/concepts/irreducible-hard-region-conjecture.md` (vol-208, the
meta-conclusion of the TRANSEPT strip-assignment investigation). The page is
explicitly a **CONJECTURE with strong empirical support, not a theorem**. It
has three parts:

1. **Localization** — every sequential spatial decomposition fills the top
   freely and concentrates the residual difficulty into a single "hard region"
   at the LAST part of the sweep (bottom rows / meeting ring / last block).
2. **Exceeds the exact window** — that hard region (~96-128 cells) sits at or
   past the ~112-cell exact-solvable boundary measured in vol-208.
3. **Too globally coupled** — piece-uniqueness couples the hard region to the
   whole board, so heuristics cannot complete it to a new high basin.

Parts 2 and 3 are algorithm- and machine-specific empirical ceilings, not
reproducible facts about a piece set: the concept page itself flags "exceeds
the exact window" as "machine/algorithm-specific" and "too coupled for
heuristics" as "the empirical ceiling, not proven". **This topic reproduces
only part 1 (localization)** — the one concrete, board-measurable claim — and
labels parts 2 and 3 as conjecture, not result.

## (a) The exact claim and expected shape

Part 1 (localization), from the concept page's evidence chain:

| # | Claim | Concept's number / shape |
|---|-------|--------------------------|
| 1 | The easy part fills freely | first ~10 rows / ~160 cells fill perfectly; "near-free until depth ~150" |
| 2 | Residual difficulty localizes to the last region | bottom ~6 rows / ~96 cells in a strip/row-major decomposition |

Reproduced measurement: on framed, colour-balanced, **planted-solvable**
boards (a perfect solution is guaranteed to exist, so any stall is the
search's, not the instance's), run a restarting exact-match DFS **from
scratch** in row-major order and record two localization statistics per board:

- **frontier fraction** = deepest fully-filled row / n. Concept part-1 point 1
  predicts this is large (the top fills), well past 0.5, on stalled boards.
- **bottom-half residual fraction** = of the unfilled cells, the fraction in
  rows n/2..n. Concept part-1 point 2 predicts this is near 1.0 (the hard
  region is the bottom band).

Control: the **same boards** filled in a uniformly random cell order, which
has no "last region". Its bottom-half residual fraction should sit near 0.5
(residual scatters), showing the localization is a property of the sequential
decomposition, not of the board.

Expected qualitative result (reproduction passes if):
- row-major stalled boards have median frontier fraction clearly > 0.5 and
  mean bottom-half residual fraction clearly > 0.5 (near 1.0);
- the random control's bottom-half residual fraction sits near 0.5.

The concept gives no per-seed digit table for a kit run, so there is nothing
to match bit-for-bit; this is a qualitative reproduction of the sign/band.

## (b) Scoring-convention mapping

This topic makes no record claim. Scores are reported through the kit's
canonical `score_board`, the same rim-excluding scorer the site uses (interior
edges only; a full NxN solve scores 2N(N-1)). The frontier and residual
statistics are cell-geometry, not edge-score, so no strict-5/5 vs
matched-edges mapping applies. Boards are generated **without pinned hints** so
the measurement is a clean from-scratch decomposition, matching the concept's
"sequential strip-fill from the rich pool" framing (the original TRANSEPT work
filled from scratch, not from the 5 official clues).

## (c) Scale-faithfulness

The concept is about the canonical 16x16 puzzle's decomposition difficulty. We
run at the real **N=16** side length, so the "Theta(n) = Theta(16)" hard-region
size and the "last 6 rows / ~96 cells" band are at their real scale. The single
faithfulness gap is the **instance family**: we use the kit's framed
colour-balanced generator (planted-solvable, plausibly many solutions) rather
than the canonical official 22-colour set (essentially one solution). The
concept's localization argument is a property of any solvable edge-matching
board under a sequential fill (the residual must live somewhere and a
row-major sweep puts it at the bottom), so the generated family is an adequate
and honest test of part 1; it is NOT a test of parts 2-3, which depend on the
official set's specific coupling. This gap is declared in the article.

## (d) Exact reproduction steps

1. `cd research/topics/irreducible-hard-region/compute`
2. `cargo run --release -- --n 16 --seed-lo 1 --seed-hi 40 --node-cap 1500000 --restart-cap 150000 > ../results/hard_region.json`
   (about 3 minutes on Apple Silicon, single core; 40 boards x 2 arms.)
3. Optional budget-insensitivity check (concept: "the failure is
   budget-insensitive", stall row does not climb with more nodes):
   `cargo run --release -- --n 16 --seed-lo 1 --seed-hi 8 --node-cap 4000000 --restart-cap 400000 > ../results/hard_region_budget4m.json`
   Compare the row-major frontier fractions against the same seeds in the main
   run; they should barely move.
4. Read `aggregate.row_major` and `aggregate.random_control` in the JSON. The
   claim reproduces if row-major mean bottom-half residual fraction is clearly
   above the control's (near 1.0 vs near 0.5) and the row-major median frontier
   fraction is above 0.5.

The generator and DFS shuffle are seeded, so a rerun at the same flags is
byte-stable; the tier is `qualitative` only because the *concept's* numbers are
lineage-bound (different instance family, no committed per-seed table in the
source), not because our own run is noisy.

## Checker structure

`src/main.rs`, single file, deps `e2-kit` (path dep on the starter kit) and
`serde_json`:

- for each generator seed: build a framed planted-solvable NxN instance
  (no hints), run the restarting exact-match DFS twice — row-major slot order
  and a seeded random slot order (control) — on the same board;
- from each final (deepest) board compute the frontier row and the
  bottom-half residual fraction directly from cell occupancy (correct for any
  slot order);
- aggregate over the unsolved boards and emit one JSON object: config, the
  row-major vs control aggregate, and the full per-board record.
