---
id: n13-encoding-cliff
title: The N=13 cliff is in the search, not the instances
summary: Every chronological heuristic tested dies between 12x12 and 13x13 on planted boards, yet a CP-SAT encoding with AllDifferent and Element channelling solves the same instances in half a second. The encoding is the algorithm.
status: draft
created: 2026-07-22
updated: 2026-07-22
contributors:
  - name: Raphael Anjou
    role: author
tags:
  - complexity
  - exact-methods
  - methodology
sources: []
reproduce:
  - cd research/topics/n13-encoding-cliff/compute && cargo run --release -- sweep --ns 10,11,12,13,14 --gen-seeds 1,2,3,4,5 --budget-secs 20 > ../results/heuristic.jsonl
  - cd research/topics/n13-encoding-cliff/compute && cargo run --release -- export --ns 10,11,12,13,14,15,16 --gen-seeds 1,2,3,4,5 --out instances
  - cd research/topics/n13-encoding-cliff/compute && uv run cpsat_cliff.py instances/*.json --time-limit 300 --workers 8 > ../results/cpsat.jsonl
results: []
site:
  render: false
  dataFile: null
---

# The N=13 cliff is in the search, not the instances

> **Status:** draft · **Created:** 2026-07-22 · **Updated:** 2026-07-22
> **Authors:** Raphael Anjou

## Summary

On planted, guaranteed solvable Eternity II style boards with five pinned
clues, restarting depth-first search full-solves every 12x12 instance in well
under a second and then fails catastrophically at 13x13: it plateaus around
12 percent of the target and never improves, no matter the budget. Ten
heuristic families were tested, from a naive row-major filler to AC-3
propagation and Hall-deficit pruning. Six of them full-solve 12x12; not one
solves 13x13. The boundary is sharp, seed-independent, and not a budget
effect.

That looked like a property of the instances. It is not. A CP-SAT model of
the same instances, built from an AllDifferent constraint over cell-to-piece
assignment plus Element channelling of edge colours, posed as a decision
question, solves 25 of 25 instances across sizes 10 through 14 with a smooth
time curve: the 13x13 boards that no heuristic touches fall in 0.50 seconds
median, and planted 16x16 boards fall in about 15 seconds. The cliff belongs
to the search paradigm, not to the boards. All ten heuristic families share
one bias, chronological greedy commitment with only local information, so
their agreement was one witness reported ten times.

The formulation gap alone is worth stating. A generic MIP over placement
binaries with sum-to-one rows (HiGHS branch and bound) scores 11 of 264 on a
12x12 instance in 300 seconds; CP-SAT on the same instance returns the full
264 in 0.42 seconds. Three orders of magnitude, same problem, different
encoding. Two cautions on scope: the planted boards are not canonical
Eternity II (they plausibly admit very many solutions, where the real puzzle
is believed to admit essentially one), and the original measurements used 26
interior colours where the real puzzle has 22, which makes those boards more
constrained and easier than the real thing.

## Reproduction

Two arms, both driven from `compute/`, both scored through the kit's
canonical rim-excluding scorer so a full solve at size N means exactly
2N(N-1) matched edges (312 at N=13, 480 at N=16).

The heuristic arm is a Rust binary on the starter kit: it generates framed,
colour-balanced planted instances, pins five solution cells as clues, and
runs a restarting randomized depth-first search that places only exact
matches. Expected shape: full solves in milliseconds up to N=12, hard
failure at the cliff, with scores pinned rather than creeping as the budget
grows. The same binary exports each instance as JSON for the second arm.

The exact arm is a Python script (OR-Tools CP-SAT) that reads the exported
instances and builds the structured encoding: one integer variable per cell
over piece ids under AllDifferent, a piece-and-rotation index channelled
through Element tables into per-side colour variables, hard adjacency and
rim constraints, and the clues fixed. It asks the decision question only,
then independently re-verifies any returned board (piece distinctness, clue
conformance, recomputed edge count) rather than trusting the solver's
status. The claim reproduces if the exact arm solves every instance with a
smooth time curve through the size at which every heuristic in the first arm
has collapsed. See `compute/PLAN.md` for the full design, the expected
numbers, and the fidelity caveats (colour count and generator differences
against the original measurement).
