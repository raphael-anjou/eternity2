# Reproduction plan: constraint-immediacy

> **Repro tier: mixed: korder mode exact; solver-order ranking seeded-statistical (ordering, not digits).**
> exact = re-run reproduces committed results byte-for-byte. seeded-statistical =
> committed results are the record of our run (hardware noted in the article);
> reproduction = fresh run agreeing within the stated tolerance. qualitative =
> the effect (sign/ordering/band) reappears; exact digits are lineage-bound.

Source of the claim: research repo,
`research/vault/concepts/constraint-immediacy-principle.md` (vol-216 synthesis,
drawing on vol-14/36 path refutations, vol-213 seam results, the spiral
closure-tax refutation, and the border-first win).

## (a) The exact claim, with expected numbers

Two-part claim.

**Part 1 (theorem, exact).** For any complete visit order of an N x N board,
with k_i = the number of already placed orthogonal neighbours cell i faces when
it is filled, every interior edge is counted exactly once (by the second-placed
endpoint), so

    sum_i k_i = #interior edges  (path-invariant).

On 16x16 that constant is 2 * 16 * 15 = 480. No visit order "adds restriction";
it only chooses when each restriction binds.

**Part 2 (measured ranking, from the vault evidence table).** Fixed-order
search quality tracks immediacy (small decision-to-refutation distance), not
geometric earliness. Vault-cited numbers, all measured in this project on the
official 16x16 puzzle:

| path            | k-profile shape                       | vault result        |
|-----------------|---------------------------------------|---------------------|
| hint-link (vol-36) | k=1 corridors + k>=3 pockets       | 51/480              |
| outer spiral    | closure everywhere                    | 204/480             |
| seam two-front (vol-213) | k=3..4 meeting band          | -3..-5 vs row-major |
| row-major       | uniform k=2                           | 433/480             |
| border-first (vol-36) | most-constrained sub-pool first, binds immediately | 445/480 |

The reproducible claim is the **ordering** (border-first > row-major >> spiral
>> hint-link, with seam slightly below row-major), plus the k-histograms that
explain it. The exact historical values (51, 204, 433, 445) came from the
research repo's engine with its own restart and backtrack policy; a fresh
fixed-order solver in the kit is expected to reproduce the ranking and the
rough magnitudes, not those numbers digit-for-digit. The Part 1 constant (480)
must reproduce exactly for every order.

## (b) Scoring-convention mapping

The vault numbers are **matched interior edges out of 480**, the same
rim-excluding matched-edges convention as the kit's canonical `score_cells` /
`score_board` (MAX_SCORE_16 = 480). No conversion needed. Strict-5/5 hint
compliance is not the metric here; the five official clues matter only as
geometry (the hint-link path is defined by them), and the solver starts from
`instance.seed_board()` so pinned clues are honoured anyway. Part 1's constant
480 is a count of interior adjacencies, which coincides numerically with the
max score because both count interior edges.

## (c) Scale-faithfulness

Part 1 is size-generic: the conservation law holds for any N and the checker
verifies it on 16x16 directly (and trivially extends to any size). No scale
question arises.

Part 2 should be run on the **canonical 16x16**, for three reasons:

1. The hint-link path is defined by the five official clue cells; a generated
   small board has no canonical clue geometry, so the worst-case order in the
   table cannot even be posed faithfully at small N.
2. The immediacy gap is driven by long k=1 corridors (roughly 35-50 candidate
   pieces per under-constrained cell, per the vault page) followed by k>=3
   closures. On a small board corridors are short and candidate pools are
   small, so the fail-last penalty compresses and a small-N test could
   falsely show the orders as near-equal, refuting nothing.
3. The cited evidence is all 16x16; matching the instance removes one source
   of dispute.

Generated 16x16 seeds (kit generator, framed, 22 colours) are a legitimate
**variance supplement** for the non-hint orders (row-major, boustrophedon,
spiral, border-first): run 8+ seeds and report min/median/max, then the
official instance as the headline. Single-seed point estimates are not
results.

## (d) Reproduction steps

From this directory (`compute/`):

1. `cargo run --release -- korder`
   Prints, for each of the five orders on the official instance: the k
   histogram (how many cells were placed facing 0, 1, 2, 3, 4 already placed
   neighbours) and the sum, which must be 480 for every order. Milliseconds.

2. `cargo run --release -- solve 5`
   Runs, per order at 5 seconds each: (i) the greedy best-fit fixed-order pass
   (full board, breaks allowed) and (ii) the perfect-fit fixed-order DFS with
   chronological backtracking (deepest consistent prefix). Prints the canonical
   score, the board URL, and nodes for each. About 10 x 5 s total.
   Increase the per-order budget (e.g. `solve 60`) for the results/ run.

3. Optional variance sweep on generated boards:
   `cargo run --release -- solve 5 --gen-seeds 1..8` (TODO in the skeleton;
   the kit `sweep` runner covers this once the solver is registered there).

Acceptance: step 1 prints `sum(k) = 480` five times with visibly different
histograms (hint-link heavy at k<=1 and k>=3, row-major concentrated at k=2,
border-first rim cells binding immediately). Step 2 reproduces the ranking of
the vault table. Results are written to `results/` only when the run is final.

Caveat recorded up front: the hint-link order in the skeleton is a faithful
*shape* (corridors linking the five official clue cells, then fill), not a
byte-for-byte port of the vol-36 path, which was not preserved as an explicit
cell list in the vault. If the reproduced hint-link score lands far from 51
but still last by a wide margin, the claim stands; the exact 51 belongs to the
lost concrete path.
