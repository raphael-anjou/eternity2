---
id: the-470-wall
title: The 470 wall as a phase boundary
summary: A random-CSP analysis of the official piece set places the entropic ceiling for planted-solution-uncorrelated boards at roughly 465 to 470, matching the community wall, with 480 an isolated planted needle beyond an empty overlap gap.
status: draft
created: 2026-07-22
updated: 2026-07-22
contributors:
  - name: Raphael Anjou
    role: author
tags:
  - structure
  - complexity
  - statistics
sources:
  - label: "Achlioptas, Coja-Oghlan. Algorithmic barriers from phase transitions. FOCS 2008."
    url: https://arxiv.org/abs/0803.2122
  - label: "Gamarnik. The overlap gap property: a topological barrier to optimizing over random structures. PNAS 2021."
    url: https://www.pnas.org/doi/10.1073/pnas.2108492118
reproduce:
  - cd research/topics/the-470-wall/compute && cargo run --release --bin the_470_wall > ../results/landscape.json
results:
  - label: Exact instance statistics and the annealed score landscape (JSON, computed here)
    path: results/landscape.json
site:
  render: false
  dataFile: null
---

# The 470 wall as a phase boundary

> **Status:** draft · **Created:** 2026-07-22 · **Updated:** 2026-07-22
> **Authors:** Raphael Anjou

## Summary

For years the best public Eternity II boards have clustered in the 460s, with the
record at 470 of 480. This topic asks whether that wall is an engineering limit
or a property of the instance itself. Treating the official piece set as a
planted random constraint-satisfaction ensemble gives a quantitative answer: the
number of board configurations uncorrelated with the planted solution is
astronomically large up to a score of roughly 465 to 470 and collapses beyond
it. Heuristic search lives on that entropic band, so the high 460s come almost
for free and everything past them does not.

The analysis rests on one exact parameter of the real set: the expected number
of tiles that fit a fully constrained interior cell is about 0.0094, far below
one, which places the instance deep in the rigid regime of such ensembles. In
that regime, exhaustive enumeration of small planted instances tuned to the same
parameter shows the solution set collapsing to the planted board plus a few
globally orthogonal boards, with no partial-overlap solutions at all. An
annealed first-moment count prices the real instance at roughly 10 to 20
mutually near-orthogonal perfect solutions. The reading of the 470 wall is then
a phase boundary: the ten missing points are the width of an empty overlap gap
between the entropic band and the planted needle, not a deficit of solver
engineering.

Two caveats are load-bearing. The score-landscape and constraint-density numbers
are exact computations on the official set, reproduced here from the shared
engine. The overlap-gap structure is exact only for small instances (up to 7x7
exhaustive enumeration) and is a conjectural extrapolation to 16x16 along a
monotone trend in the constraint-density parameter. This draft states both
layers and keeps them separate.

## Reproduction

The compute crate in `compute/` recomputes every instance-side number from the
official piece set as shipped in the shared engine, with no external data. It
verifies the color economy (960 non-gray half-edges, a frame ring saturating
colors 1 to 5 at 60 adjacencies, and an interior subsystem of 420 adjacencies on
colors 6 to 22), derives the per-adjacency collision probabilities for the two
subsystems, computes the constraint-density parameter, evaluates the annealed
first-moment count of perfect uncorrelated placements, and runs an exact
log-space convolution of the 480 Bernoulli adjacency indicators to produce the
full annealed score landscape, including the mean random score near 36.7 and the
entropy collapse above 470. It also checks the one statistically significant
anomaly of the bag: the real set has zero rotation-duplicate tiles.

The small-instance overlap-gap enumeration and the matched-null distinguishability
battery are specified in `compute/PLAN.md` and are not yet ported to this crate.
See that file for the exact claim list, the scoring-convention mapping, and the
step-by-step reproduction design.
