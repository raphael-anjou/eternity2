---
id: flux-invariants
title: "Flux and character invariants: a rotation-parity law the official set obeys exactly"
summary: Weighting each tile edge and reading a signed flux vector turns the official 256-piece set into a linear algebra fact — the flux law has full complex rank 22, forty independent real constraints and twenty-one F2 parity constraints, every one of them invisible to colour counting, and its mod-2 shadow gives a sound endgame certificate that flags a single rotation error with a detection rate rising toward 100% as the board fills.
status: draft
created: 2026-07-23
updated: 2026-07-23
contributors:
  - name: Raphael Anjou
    role: author
credits:
  - Adapted from a vol-226 blind-attack derivation of Conway-Lagarias boundary invariants for edge matching.
tags:
  - structure
  - invariants
  - algebra
sources:
  - label: Eternity II piece set (official instance)
    url: https://e2.bucas.name/
repro:
  tier: exact
  cmd: cd research/topics/flux-invariants/compute && cargo run --release > ../results/flux_invariants.json
  scope: Instance facts and the three flux-law ranks are exact on the official 256-piece set; the endgame certificate runs on a pool of framed solved 8x8 boards (planted rotations) and reproduces the paper's mechanism, soundness, and monotone catch curve, not its exact 13/50/100 percentages.
reproduce:
  - cd research/topics/flux-invariants/compute && cargo run --release > ../results/flux_invariants.json
results:
  - label: Results JSON (instance facts, flux-law ranks, endgame certificate curve; seed 1, byte-stable)
    path: results/flux_invariants.json
site:
  render: false
  dataFile: null
---

# Flux and character invariants: a rotation-parity law the official set obeys exactly

> **Status:** draft · **Created:** 2026-07-23 · **Updated:** 2026-07-23
> **Authors:** Raphael Anjou

## Summary

Give every edge colour a weight and read each tile as a signed vector, its East
weight minus its West on one axis, its South minus its North on the other. Sum
that vector over the whole board and every internal seam cancels against its
twin, leaving only the perimeter, which is all gray. So the total is zero. That
is a conservation law, a discrete divergence theorem, and because a quarter
turn of a tile rotates its vector by ninety degrees, the law is really a
statement in the Gaussian integers about which rotations a solution may use.

This note reproduces the linear-algebra content of that law on the official
256-piece set and its use as an endgame check. The instance facts come out
exactly as the source states them: 196 interior, 56 edge, 4 corner pieces; the
gray border colour on 64 half-edges; five frame colours that never touch an
interior piece, 24 half-edges each; the remaining interior colours split five
at 48 and twelve at 50; every single colour count even, which a perfect edge
matching requires. The five official clues are all interior pieces. The flux
law itself has full complex rank 22, forty independent real constraints, and
twenty-one independent F2 parity constraints, and its rotation-blind shadow is
identically zero for all 22 colours, which means every constraint it imposes is
information colour counting cannot see. Reduced mod 2, the law becomes a small
linear system over the tiles' rotation parities, and that system is a sound
endgame certificate: on planted boards it never rejects a valid partial and
catches a single injected rotation error with a probability that climbs toward
100 percent as the board approaches full.

## Reproduction

The claims split into two groups. The instance facts and the three ranks are
exact linear-algebra properties of the canonical 256-piece list, recomputed in
a single deterministic pass with no sampling and no solver: any deviation
refutes the claim, and the checker exits nonzero on the first mismatch. The
endgame certificate is a general law tested, as in the source, on planted
boards; its soundness (never rejecting a valid partial) is exact, and its catch
rate is a measured curve whose reproduced content is the monotone rise toward
full fill, not the exact percentages.

One convention has to line up for any of this to mean anything. The source
reads a tile clockwise from the top as (N, E, S, W) and rotates by sending
(N,E,S,W) to (W,N,E,S). The starter kit stores edges as URDL, up-right-down-left,
which is exactly (N, E, S, W), and its clockwise quarter turn sends the same
quadruple to (W,N,E,S). So the kit's rotation index is the source's number of
clockwise turns with nothing reindexed, and the flux vector rotates by a power
of i under the kit's own `rotated`. The checker records this in its output and
derives the frame colours from the data (colours that never appear on an
interior piece) rather than trusting a label, so a renumbering between the
kit's `official.json` and the source's list cannot fool it. See `compute/PLAN.md`
for the claim-by-claim map, the convention proof, and the scale-faithfulness
argument (why the ranks run on the real 16x16 instance and the certificate on
planted 8x8 boards).

### Measured results

Committed as `results/flux_invariants.json`, byte-stable on rerun (Apple
Silicon, single core, runtime under one second). Every deterministic field is
checked against the source; the certificate reports its own catch curve.

Instance facts and flux-law ranks (all exact):

| Quantity | Source | Measured |
|---|---|---|
| Piece-type histogram | 196 / 56 / 4 | 196 / 56 / 4 |
| Gray (colour 0) half-edges | 64 | 64 |
| Frame colours | 5, border-only, 24 each | 5 (colours 1..5), 24 each, supply 120 |
| Non-frame multiplicities | 5 at 48, 12 at 50 | 5 at 48, 12 at 50 |
| Every colour count even | yes | yes |
| Five clues all interior | yes | yes (piece ids 138,180,207,248,254) |
| Complex rank of the flux matrix | 22 of 22 | 22 |
| Independent real constraints | 40 | 40 |
| Independent F2 parity constraints | 21 (augmented 21, consistent) | 21 (augmented 21, consistent) |
| Census orthogonality (i to 1 shadow) | identically zero, all 22 colours | zero, all 22 colours |
| Base-orientation nontriviality (j=1) | 20 of 22 colours nonzero | 20 of 22 |

Endgame certificate (mod-2 feasibility, framed solved 8x8 board pool of 30,
planted rotations, 900 orderings per fill, one injected rotation error):

| Fill fraction | Source catch rate | Measured catch rate | Valid partials rejected |
|---|---|---|---|
| 0.50 | 13% | 0.6% | 0 of 900 |
| 0.75 | 50% | 14.1% | 0 of 900 |
| 0.90 | 100% | 88.3% | 0 of 900 |
| 0.95 | (not in source) | 94.6% | 0 of 900 |

Every instance fact and every rank matches the source exactly, including the
two that are the point of the paper: the flux law is full rank and its
information is entirely orthogonal to colour counting (the rotation-blind
shadow vanishes on all 22 colours, so nothing the law says can be recovered
from a census). The certificate reproduces its defining properties exactly: it
is sound, rejecting none of the 3600 valid partials across the sweep, and its
detection probability rises monotonically with fill, from under one percent at
half full to the mid-nineties near full. The absolute percentages sit below the
source's 13 / 50 / 100 because the planted setup differs (this run is an 8x8
board at 13 colours with a uniformly chosen injected tile, not the source's
exact planted family), which the plan flags up front as setup-bound; the
reproduced result is the mechanism and the direction, and both hold.

## Notes / caveats

The certificate is an obstruction, a necessary condition only: it can certify a
partial dead, never completable, which is the correct and intended role for an
invariant inside a branch-and-prune search. Its pruning is orthogonal to and
additive on top of edge-matching and colour-count pruning, and it is an endgame
tool by design, sharpening exactly as the remainder shrinks and the search
tree's tail grows most expensive. The exact catch percentages are
planted-setup-bound; the reproduced claim is that the mod-2 shadow of the flux
law is a sound, monotone, fill-strengthening rotation-error detector, and that
the flux law on the real instance is full rank and census-orthogonal. Both
reproduce exactly. The source's frame-localized certificate (colours 1..5 give
ten real constraints on the 60 frame tiles) and its open meet-in-the-middle and
per-band conjectures are not exercised here.
