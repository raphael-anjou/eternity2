---
id: cas-annular
title: Concentric annular solving plateaus at 430-436 and beats ALNS from a perfect frame
summary: Greedy shell-by-shell solving of the official 16x16 puzzle from a perfect 60/60 frame plateaus at 430-436 matched edges regardless of frame choice, yet beats a 60-second ALNS continuation from the same frames by 34 to 49 points.
status: draft
created: 2026-07-22
updated: 2026-07-22
contributors:
  - name: Raphael Anjou
    role: author
credits:
  - Internal research vault, volumes 74-79 (2026-05-15), CAS cluster.
tags:
  - local-search
  - constructive
  - negative-result
sources:
  - label: Vault concept, CAS umbrella (refuted as greedy)
    url: null
  - label: Vault concept, CAS from 20 frames final results (vol-76)
    url: null
  - label: Vault concept, CAS beats ALNS when starting from a perfect frame (vol-77)
    url: null
reproduce:
  - cd research/topics/cas-annular/compute && cargo run --release
results: []
site:
  render: false
  dataFile: null
---

# Concentric annular solving plateaus at 430-436 and beats ALNS from a perfect frame

> **Status:** draft · **Created:** 2026-07-22 · **Updated:** 2026-07-22
> **Authors:** Raphael Anjou

## Summary

Concentric Annular Solving (CAS) fills the official 16x16 board from the
outside in: place the border ring as a perfect 60/60 frame, then solve each
inner ring in turn as one assignment problem over the remaining pieces,
maximising matched edges against the ring outside it. Run on 20 enumerated
perfect frames, CAS lands in a narrow band of 430 to 436 matched edges out of
480 (mean 432.4, standard deviation about 2). The frame does not matter: every
60/60 frame leads to the same plateau, because by rings 5 to 7 the remaining
piece pool no longer contains pieces whose colour profiles fit the inward-facing
constraints. The bottleneck is piece availability, not edge coverage, and it is
structural rather than a tuning issue. CAS is therefore refuted as a record
path: the same hardware and time reach 459 and above through other pipelines.

The finding has a second, positive half. When an ALNS local-search continuation
is forced to start from the same state (a perfect frame plus an empty interior),
it plateaus at 385 to 398 in 60 seconds per frame, so CAS wins every head-to-head
comparison by 34 to 49 points. ALNS destroy-and-repair operators expect a full
board to mutate; handed an empty interior they cannot construct one quickly,
while CAS is exactly the constructive tool for that state. The durable lesson is
that algorithm choice depends on the starting state, and pipeline composition
matters as much as the components.

## Reproduction

The reproduction runs CAS on the official instance through the starter kit. The
solver builds a perfect 60/60 frame by depth-first search over the 60 border
pieces (every rim edge grey, every within-ring adjacency matched), then fills
rings 1 to 7 shell by shell. The kit re-scores the final board through the
canonical rim-excluding scorer, which is the same matched-edges convention the
vault numbers use, so the claim's numbers are directly comparable.

Two parts of the original method still need porting for a faithful measurement:
the per-shell exact assignment (the vault solved each shell as a MIP; the
skeleton uses greedy best-fit per cell, which is expected to land below the
430-436 band) and frame enumeration (the vault measured a distribution across 20
distinct frames; the skeleton takes the first frame its search finds). The ALNS
comparison arm additionally requires an ALNS implementation, which the kit does
not provide. See `compute/PLAN.md` for the full design and the expected numbers.
