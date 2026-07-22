---
id: ring-purity
title: "Ring purity: the five frame colors saturate the border with zero slack"
summary: The five frame-only colors of the official instance appear exclusively on the 60 border pieces, and every valid solution consumes all 120 frame half-edges on the border ring, reducing the border sub-problem to an Eulerian circuit on a 5-vertex multigraph.
status: draft
created: 2026-07-22
updated: 2026-07-22
contributors:
  - name: Raphael Anjou
    role: author
credits:
  - Derived during a flow/cut study of exact conservation laws on the official instance.
tags:
  - structure
  - border
  - graph-theory
sources:
  - label: Eternity II piece set (official instance)
    url: https://e2.bucas.name/
reproduce:
  - cd research/topics/ring-purity/compute && cargo run --release > ../results/ring_purity.json
results:
  - label: Results JSON (seed 1, deterministic clauses + SIS)
    path: results/ring_purity.json
site:
  render: false
  dataFile: null
---

# Ring purity: the five frame colors saturate the border with zero slack

> **Status:** draft · **Created:** 2026-07-22 · **Updated:** 2026-07-22
> **Authors:** Raphael Anjou

## Summary

The official Eternity II piece set contains five colors that never appear on any of the 196 interior pieces. Call them the frame colors. Each frame color occurs on exactly 24 half-edges, all of them on the 60 border pieces. This note establishes the Ring Purity Theorem: the piece data forces every valid solution to spend the entire frame-color supply, all 120 half-edges, on the 120 ring-facing edge slots of the border ring, with zero slack.

The proof rests on three exhaustively checkable facts about the piece list. First, frame colors occur only on border pieces. Second, every one of the 56 edge pieces carries exactly two frame-colored slots and one non-frame slot among its three non-zero slots, and the non-frame slot sits opposite the gray slot. Third, both non-zero slots of every corner piece are frame-colored. In any valid solution an edge piece's inward-facing slot cannot be frame-colored, because the matching interior piece would need a frame color it does not have. The two frame slots are therefore forced onto the two ring-facing positions, and the demand of 56 x 2 + 4 x 2 = 120 ring slots exactly equals the supply of 5 x 24 = 120 frame half-edges.

A direct consequence: a valid border-ring arrangement is exactly an Eulerian circuit of the 5-vertex, 60-edge multigraph whose vertices are the frame colors and whose edges are the border pieces, each joining its two ring-facing colors. On the real instance this multigraph is connected with all five degrees equal to 24, so an Eulerian circuit exists, as it must, since a full solution exists. Two quantitative strength measures accompany the theorem: ring-color matching alone cuts first-step branching from 59 candidates to about 21.2 on average (a 2.78x reduction), and a sequential importance sampling estimate over 1000 greedy ring builds shows the color-matching law concentrates probability mass by roughly 53 orders of magnitude relative to a uniform ordering of the border pieces, while still leaving a residual rarity near 10^-27.

## Reproduction

All claims split into two groups. The structural facts (frame-color identification, per-piece slot structure, the 120 = 120 saturation count, multigraph degrees, connectivity, Eulerian existence, and the exact first-step branching counts) are deterministic properties of the official 256-piece list and are recomputed by exhaustive enumeration in a single pass, no sampling and no solver involved. The two strength measures are Monte Carlo: the branching factor is an exact count, and the greedy-completion statistics use seeded sequential importance sampling with 1000 trials, reporting completion rate and mean log10 path probability with its spread.

The checker in `compute/` loads the official instance through the starter kit's `official_instance`, derives the frame-color set from the data rather than assuming color labels, verifies every clause of the theorem, builds the ring multigraph, and emits one JSON object with every measured number. See `compute/PLAN.md` for the claim-by-claim mapping, the scoring-convention note, and why this topic must be checked on the canonical 16x16 instance rather than a generated small board.

Measured results (committed as `results/ring_purity.json`, byte-stable on rerun; Apple Silicon, single core, runtime under one second):

| Quantity | Expected | Measured |
|---|---|---|
| Piece-type histogram | 196 / 56 / 4 | 196 / 56 / 4 |
| Frame colors | 5 | 5 (colors 1..5) |
| Half-edges per frame color | 24 each, supply 120 | 24 each, supply 120 |
| Non-frame color multiplicities | 5 at 48, 12 at 50 | verified (hard check passed) |
| Edge-piece slot structure | 2 frame ring slots, non-frame opposite gray | 56/56 |
| Corner-piece slot structure | both slots frame, gray slots adjacent | 4/4 |
| Ring demand vs supply | 120 = 120, zero slack | 120 = 120 |
| Ring multigraph | 60 edges, all degrees 24, connected, Eulerian | 60 edges, degrees all 24, connected, Eulerian |
| Self-loops / distinct color-pairs | 14 / 15 | 14 / 15 |
| First-step branching multiset | {20, 21, 21, 22, 22}, mean 21.2, 2.78x | {20, 21, 21, 22, 22}, mean 21.2, 2.783x |
| SIS completion rate (1000 trials) | about 9.4% | 8.7% (seed 1); 9.3% (seed 2) |
| Mean log10 path probability | -27.3, sd about 0.8 | -27.30, sd 0.64 (seed 1); -27.39, sd 0.62 (seed 2) |
| Uniform baseline | log10(1/59!) = -80.1 | -80.14 |

Every deterministic clause matches exactly. The two Monte Carlo measures land inside sampling error of the source values across two independent seeds, and every one of the 87 completed seed-1 builds closed into a circuit (open ends matching), consistent with the Eulerian structure. One incidental observation beyond the plan: the seed-1 sample standard deviation (0.64) is a little tighter than the source's reported 0.8, which is within what 87-versus-94 sample fluctuations allow.

## Notes / caveats

The exact count of undirected Eulerian circuits of the real ring multigraph is open. A single-orientation BEST-theorem shortcut was attempted during the original investigation, caught as an undercount by a K5 sanity check, and withdrawn; no circuit count is claimed here. The greedy completion measure ignores the geometric corner/edge alternation constraint of the physical ring, so it measures the color-matching law in isolation.
