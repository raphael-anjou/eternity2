# cas-annular reproduction plan

> **Repro tier: seeded-statistical (plateau band 430-436 and the CAS-vs-baseline gap direction).**
> exact = re-run reproduces committed results byte-for-byte. seeded-statistical =
> committed results are the record of our run (hardware noted in the article);
> reproduction = fresh run agreeing within the stated tolerance. qualitative =
> the effect (sign/ordering/band) reappears; exact digits are lineage-bound.

## (a) The exact claim, with expected numbers

Source pages (research repo vault):

- `research/vault/concepts/_umbrella-cas.md` (CAS umbrella, status refuted-as-greedy, vols 74-79)
- `research/vault/concepts/cas-frame-final.md` (vol-76, 2026-05-15)
- `research/vault/concepts/cas-beats-alns-from-frame.md` (vol-77, 2026-05-15)

Claim 1 (plateau). CAS run on each of 20 enumerated perfect 60/60 frames of the
official 16x16 puzzle scores, per frame (cas-frame-final):

| CAS score | count |
|---|---|
| 430 | 5 |
| 431 | 2 |
| 432 | 4 |
| 433 | 3 |
| 434 | 2 |
| 435 | 2 |
| 436 | 2 |

Range 430-436, mean 432.4, best 436 (frame_solution_0), std about 2. Conclusion
quoted from the source: "Greedy-annular CAS cannot beat 436 on canonical E2 from
any 60/60 frame." Mechanism: by shells 5-7 the remaining 4-12 cells have very
few piece-rotation candidates matching shell-6's inward colours; the bottleneck
is piece availability, not the frame choice and not edge coverage (the umbrella
page confirms all 480 edges are in the CAS objective).

Claim 2 (CAS beats ALNS from a frame). Pinning each of 10 enumerated 60/60
frames and running a continuation (cas-beats-alns-from-frame): CAS (shells 1-7
sequentially via MIP, 25-45 s/frame) scores 430-436; ALNS (standard alns_only
winning5, 60 s/frame, seed=1) scores 385-398. CAS wins every one of the 8
overlapping comparisons by +34 to +49 points (frames 1, 10, 11, 12, 13, 14, 15,
16; deltas +41, +39, +49, +43, +34, +38, +42, +41).

Claim 3 (context, from the umbrella). CAS-hybrid + ALNS scores 418 (worse than
CAS alone); CAS + ALNS refine reaches 437-439. CAS is not a record path (other
pipelines on the same class of hardware reach 459+); it is the right tool for
the specific state "perfect frame + empty interior".

## (b) Scoring-convention mapping

The vault numbers are matched interior edges out of 480 on the 16x16 board
(2wh - w - h = 480), rim edges excluded. The kit's canonical `score_cells` /
`Instance::finish` computes exactly this: a right or down adjacency scores 1 iff
the facing edges are equal and non-border, max 480. So vault CAS scores map 1:1
to kit scores; no conversion is needed.

One convention gap to note honestly: the concept pages do not record whether the
vol-74-79 runs pinned the five official clue pieces (strict-5/5). All five clues
sit in the interior, so the frame side is unaffected. The reproduction pins the
clues (`official_instance(true)`); if the originals were unhinted this makes the
reproduction slightly conservative, which is the safe direction for a plateau
claim.

"60/60 frame" means: all 60 border cells placed with border pieces, every rim
edge grey, and all 60 within-ring adjacencies matched.

## (c) Scale-faithfulness

This claim only makes sense on the canonical 16x16 instance. Two reasons, both
cited from the sources:

1. The mechanism lives in deep shells. cas-frame-final locates the bottleneck
   specifically in shells 5-7 (of 8). A 16x16 board has 8 concentric rings; an
   8x8 board has 4 and a 10x10 has 5, so a small-N test never reaches the regime
   where the plateau arises and would falsely refute (or vacuously confirm) the
   claim. Ring sizes at 16x16 are 60, 52, 44, 36, 28, 20, 12, 4 cells.
2. The numbers are instance-specific. The 430-436 band and the +34..+49 deltas
   are properties of the official piece set's colour distribution interacting
   with greedy outer-shell locking. A generated instance would test the
   qualitative shape (plateau below records, CAS > ALNS from a frame) but not
   the cited numbers.

Small-N or generated-seed runs are acceptable only as smoke tests of the code
path, never as evidence for or against the claim.

## (d) Exact reproduction steps

Step 1, skeleton (this directory, runnable now):

    cd research/topics/cas-annular/compute
    cargo run --release            # official instance, 30 s budget
    cargo run --release -- 60      # custom budget in seconds

The binary builds one perfect 60/60 frame by DFS over the border pieces, fills
rings 1-7 greedily (best-fit per cell, ring order), and prints the canonical
score, the outcome kind, the node count, and the board URL. Expected today:
frame found within seconds, full board, score BELOW 430 (greedy per-cell is a
stand-in for the per-shell MIP; see step 2). Do not read the skeleton's number
as a refutation: the vault's per-shell solver is stronger by construction.

Step 2, faithful shell solver (port needed): replace the greedy ring fill with
one exact assignment per shell, maximising matched edges (ring-internal plus
edges into the already-placed outer ring) over the remaining unused pieces, as
the vault did via MIP (25-45 s/frame total). Options: a MIP through a solver
crate (e.g. HiGHS bindings), or branch-and-bound per shell (rings 5-7 are small:
20, 12, 4 cells). Acceptance: scores enter the 430-436 band.

Step 3, frame distribution: enumerate 20 distinct perfect frames (continue the
frame DFS past the first solution, taking every k-th solution or randomising
piece order by seed) and run step 2 on each. Acceptance against claim 1: range
430-436, mean within about 1 of 432.4.

Step 4 (optional, claim 2): port an ALNS continuation (destroy-and-repair from
the pinned frame, 60 s/frame, 8 seeds minimum with min/median/max reported) and
compare per frame. Acceptance: CAS wins every frame by tens of points. The kit
has no ALNS; this arm is the largest port.

Results to write (later, into `../results/`): per-frame table
(frame id, CAS score, breaks, board URL) as JSON, plus the ALNS arm if built.
Runtime estimate for steps 2-3: about 20 frames x under 1 min/frame, within a
30 min compute envelope once the shell solver exists.
