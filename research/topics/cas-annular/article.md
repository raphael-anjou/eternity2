---
id: cas-annular
title: Concentric annular solving plateaus at 430-436 and beats ALNS from a perfect frame
summary: Greedy-annular solving from perfect 60/60 frames plateaus at 429 to 437 (mean 432.8 over 20 frames, matching the source's 430 to 436 mean 432.4) and beats an ALNS-lite continuation from the same frames on every frame (mean +18.3; the source measured +34 to +49 against a full ALNS).
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
  - cd research/topics/cas-annular/compute && cargo run --release -- 20 8 1024 45 > ../results/cas_plateau.json
results:
  - label: CAS plateau + baseline comparison (20 frames / 8 frames)
    path: results/cas_plateau.json
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

The compute crate enumerates distinct perfect 60/60 frames of the official
puzzle by seeded border DFS, then runs the greedy-annular schedule (shells
solved inward with a width-1024 per-shell beam) from each frame, and an
ALNS-lite baseline (random destroy of 6 to 12 cells plus greedy
first-improvement repair, 45 s) from the same pinned frames.

| quantity | source (vols 74-79) | measured |
|---|---|---|
| CAS score band over frames | 430 to 436 | 429 to 437 (n = 20) |
| CAS mean | 432.4 | 432.8 |
| baseline from the same frame | 385 to 398 (full ALNS) | 411 to 419 (ALNS-lite proxy) |
| CAS beats the baseline | every overlapping frame, +34 to +49 | every frame, mean gap +18.3 |

The plateau band and its mean reproduce to within one point on each edge.
The baseline gap keeps its direction and its every-frame consistency; its
magnitude is smaller here because the baseline is a light proxy rather than
the original alns_only winning5 configuration, and the shell solver is a
beam rather than the original per-shell MIP. Frame boards are linked in the results file, which is the binary's own
output (per-frame rows for both arms plus summary blocks).
