---
id: irreducible-hard-region
title: "The hard region localizes to the last band you fill"
summary: On solvable Eternity II boards, a from-scratch row-major exact-match search fills the top three quarters freely and then stalls; across 40 generated 16x16 boards the entire residual sits in the bottom half of the board every single time, while the same boards filled in a random order scatter their residual evenly, so the hard region is a property of the sequential decomposition, not of the board.
status: draft
created: 2026-07-23
updated: 2026-07-23
contributors:
  - name: Raphael Anjou
    role: author
credits:
  - Reproduces the localization half of the vol-208 irreducible-hard-region conjecture.
tags:
  - structure
  - decomposition
  - methodology
sources: []
repro:
  tier: qualitative
  cmd: cd research/topics/irreducible-hard-region/compute && cargo run --release -- --n 16 --seed-lo 1 --seed-hi 40 --node-cap 1500000 --restart-cap 150000 > ../results/hard_region.json
  scope: 40 generated framed 16x16 boards (colors=22, from-scratch, no hints), restarting exact-match DFS row-major vs random-order control, 1.5M-node budget per board; measures localization only, not the exact-window or coupling halves of the conjecture. About 3 min single core.
reproduce:
  - cd research/topics/irreducible-hard-region/compute && cargo run --release -- --n 16 --seed-lo 1 --seed-hi 40 --node-cap 1500000 --restart-cap 150000 > ../results/hard_region.json
  - cd research/topics/irreducible-hard-region/compute && cargo run --release -- --n 16 --seed-lo 1 --seed-hi 8 --node-cap 4000000 --restart-cap 400000 > ../results/hard_region_budget4m.json
results:
  - label: Localization sweep, 40 boards, row-major vs random control (JSON)
    path: results/hard_region.json
  - label: Budget-insensitivity check, 8 boards at 2.7x the node budget (JSON)
    path: results/hard_region_budget4m.json
site:
  render: false
  dataFile: null
---

# The hard region localizes to the last band you fill

> **Status:** draft · **Created:** 2026-07-23 · **Updated:** 2026-07-23
> **Authors:** Raphael Anjou

## Summary

Two hundred and eight volumes of decomposition work on Eternity II share one
shape of failure. Fill the board in a sequence of regions, whatever the regions
are, and the easy part fills freely while the difficulty piles up in whichever
region you fill last. The vault records this as the irreducible-hard-region
conjecture: every spatial decomposition concentrates the combinatorial
difficulty into a single hard region of size proportional to the board side,
because piece-uniqueness is a global coupling that no local cut can dissolve.

That conjecture has three parts. First, the difficulty localizes to the last
region. Second, that region is too big for the exact-solve window measured at
around 112 cells. Third, it is too globally coupled for a heuristic to finish
to a new high basin. Only the first part is a clean, board-measurable fact; the
other two are algorithm- and machine-specific ceilings that the concept page
itself flags as empirical, not proven. This note reproduces the first part and
labels the rest as conjecture.

The instrument is a restarting exact-match depth-first search run from scratch
on framed, colour-balanced, planted-solvable boards, so a perfect solution is
guaranteed to exist and any stall belongs to the search rather than the
instance. On 40 generated 16x16 boards the row-major search reaches a median
frontier of three quarters of the board, twelve complete rows out of sixteen,
before it can no longer place a perfect match, matching the concept's picture
of the top ten-plus rows filling freely. And on every one of the 40 boards the
entire residual, the unfilled cells, sits in the bottom half. The mean fraction
of residual cells in the bottom half is exactly 1.0 across all 40 boards.

The control makes the point sharp. Take the same boards and fill them in a
uniformly random cell order instead of row-major. Now there is no last region,
and the residual scatters: the mean bottom-half residual fraction is 0.503,
indistinguishable from an even split. The hard region is not somewhere on the
board waiting to be found. It is manufactured by the sequential decomposition,
and it lands wherever that decomposition finishes.

The stall is a wall, not a budget shortfall. Rerunning eight of the boards at
2.7 times the node budget moves the frontier by at most one row and leaves the
bottom-half residual fraction at 1.0, exactly the budget-insensitivity the
conjecture predicts.

## Reproduction

The measurement runs entirely on the starter kit: the seeded generator, the
canonical rim-excluding scorer, and the kit's edge-fit arithmetic. For each
generator seed the checker builds a framed, planted-solvable 16x16 instance
with no pinned hints, then runs the same restarting exact-match DFS twice on
that board, once in row-major slot order and once in a seeded random order, and
records two localization statistics computed directly from cell occupancy of
the deepest partial reached: the frontier row (deepest fully-filled row over the
board side) and the bottom-half residual fraction (of the unfilled cells, how
many lie in rows 8 through 15). Both statistics are well defined for any fill
order, which is what lets the random control be compared to the row-major arm on
equal footing. See `compute/PLAN.md` for the claim-by-claim mapping, the
scoring-convention note, and the faithfulness caveats.

### Measured results

Run on Apple Silicon, single core; 40 boards times two arms in about three
minutes at the default 1.5M-node budget. The source is a synthesis conjecture,
not a single measured experiment, so it carries no per-seed digit table for a
kit run; the numbers below are the shape the conjecture predicts, and the
reproduction is qualitative agreement with that shape.

Localization sweep, 40 generated framed 16x16 boards, colors 22, from scratch:

| Quantity | Conjecture's shape | Measured (row-major) | Measured (random control) |
|---|---|---|---|
| Boards fully solved | none expected (search is not a solver) | 0 / 40 | 0 / 40 |
| Median frontier fraction on stalled boards | top ~10+ rows fill freely (> 0.6) | 0.75 (12 of 16 rows) | not applicable (scattered) |
| Frontier fraction range | high | 0.6875 to 0.75 | not applicable |
| Mean bottom-half residual fraction | near 1.0 (hard region is the last band) | 1.000 | 0.503 |
| Boards with all residual in the bottom half | all | 40 / 40 | 0 / 40 |

Budget-insensitivity check, 8 boards at 4M nodes (2.7x the sweep budget):

| Quantity | 1.5M-node sweep | 4M-node rerun |
|---|---|---|
| Frontier fraction, seeds 1-8 | 0.6875 to 0.75 | 0.6875 to 0.75 |
| Mean bottom-half residual fraction | 1.000 | 1.000 |

The frontier does not climb with the budget; four of eight boards move up one
row, one moves down one row, and three do not move at all, all inside a single
row of jitter. The residual stays entirely in the bottom band. This is the conjecture's "budget-insensitive
failure" reproduced directly: the search is not slow, it is stopped, and it is
stopped at the last region it tries to fill.

## Notes / caveats

This topic reproduces the localization half of the conjecture only. The two
remaining halves are stated in the vault as conjecture and are not reproduced
here, deliberately.

The "exceeds the exact-solvable window" half is machine- and solver-specific:
the concept page derives it from a 112-cell exact-completion measurement on one
particular edge-strict DFS, and notes that a stronger exact solver would move
that window. Our hard region is the bottom four to five rows, about 64 to 80
cells, which sits near that boundary, but whether a given exact solver clears it
is a separate measurement, not reproduced here.

The "too globally coupled for heuristics" half is an empirical ceiling tied to
the official 22-colour piece set's specific scarcity structure, the 92-versus-31
co-realizable scarce demand the concept cites. Our boards come from the kit's
framed generator, which is planted-solvable and plausibly admits many solutions,
where the real puzzle is believed to admit essentially one. The localization
argument holds for any solvable board under a sequential fill, so the generated
family is an honest test of localization; it is not a test of the coupling
claim, which would need the official set. This instance-family gap is the reason
the tier is qualitative rather than exact.

One reading the control sharpens beyond the source's framing: the hard region is
not a fixed sub-board of "hard cells." The same board has all its residual in
the bottom under a row-major sweep and none of it concentrated anywhere under a
random order. The difficulty is real but it is placed by the decomposition, so
the conjecture is better read as a statement about sweep orders than about board
geography.
