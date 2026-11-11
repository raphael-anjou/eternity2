# JUDGE: near-codeword spectrum (round 2)

Branch `devin/r2-near-codeword-spectrum`, final commit cc5da99, prereg commit bf2fecb. Judge session devin-bfda2890a20a4243b03a811c2dfb89b9. The branch was not modified.

## Verdict: invalid

The run cannot support keep or kill. No pre-registered criterion fires, and the code and data are sound (no bugs found, every number reproduced). But:

1. **K1c is undetermined.** K1c is the only part of K1 above 6x6. Stage 2 stopped after 6 of 80 solves (A5): 1 of 10 usable 7x7 instances, 0 of 10 at 8x8. The binding statement says survival needs all three criteria to pass, and the prereg's own outcome rule (PREREG.md:207) needs K1, K2 and K3 all decided. Stage 3 (hypothesis part 3, where solver boards sit) was not run. The radius (part 2) was measured only at a per-seed curve threshold, not at a solver plateau.
2. **The far floor is not an orthogonal floor.** The prereg says the corner pin "removes the 4-fold board rotation" (PREREG.md:38). It does not. A judge-built board needs no search: turn the planted board by a quarter, half or three-quarter turn, then swap the planted corner back into cell 0. On all 140 instances this board is legal, lies in the far band (distance N-2 to N-1) and has 2 or 4 breaks. It equals the CP-SAT far optimum on 30/30 instances at 5x5 C=13 and 27/30 at 6x6 C=14 (7/10 and 6/10 on the contrasts). So H_far is the cost of a corner swap on the solution's own symmetric image, not the floor of an orthogonal solution. The runner found and disclosed this (README.md:68-83).
3. **The K1b pass comes mostly from that artefact.** At the end least favourable to the hypothesis, mean B grows by 0.63 breaks from 5x5 to 6x6. H_mid accounts for +0.17 of it (4.23 to 4.40). H_far accounts for +0.47: it falls from 2.87 to 2.40 because the corner swap costs 4 breaks on 13/30 boards at 5x5 but only 6/30 at 6x6. If the same pre-registered one-sided Wilcoxon is run on H_mid alone, it gives p = 0.19 at the unfavourable end (Mann-Whitney 0.24). So "the barrier grows" is carried by how often the corner colours of the symmetric copy happen to match, not by a rising mid-band price. This decomposition is post hoc. I use it only to judge whether K1b measures what it claims, never to fire a criterion.

What stands is a descriptive result. At mu-matched 5x5 and 6x6 palettes, the cheapest board at 25% to 75% colour-tuple distance from the planted solution costs a median 4 breaks (6x6 interval [4, 4.5]). A local perturbation of the planted solution, or of its turned copy, costs 2. On the looser contrast palettes, every band median is 2 (B = 0). This is a valid small-board measurement about the planted solution and its symmetric image. It is not a test of an overlap gap between distinct solutions, which is what the hypothesis claims.

## Prereg check

- **Chronology.** PREREG.md was committed alone in bf2fecb (2026-11-11 12:59:44 UTC). Code and amendments A1 to A4 followed in 3facafa (13:17:30 UTC). Every result file first appears in cc5da99 (14:56:22 UTC). No result file exists in an earlier commit.
- **Kill criteria.** They match the binding statement and make it exact:
  - K1a: median B <= 0.
  - K1b: one-sided Wilcoxon, fires at p > 0.1. "Paired size means" is read as pairing by seed index, which is stated.
  - K1c: >= 25% of 7x7/8x8 instances with mid UB <= far UB at equal budget.
  - K2: median over the 30 6x6 instances of the K2-band minimum is <= 2.
  - K3: |median(H_mid - b_ann(mid))| > 2 at 6x6.
- **K2 band cap.** The statement gives no upper end for "a >= n/3". The prereg caps the K2 band at floor(3N/4) (PREREG.md:46, 201-202). Without the cap, K2 would fire trivially on the turned copy (2 breaks in the far band). The cap was set before any run and fits the statement's words "near branch". Accepted.
- **Amendments.**
  - A1 (explicit break boolean): the integer optimum is unchanged, and my independent model agrees.
  - A2 (curves on seeds 1..10 rather than 30) and A3 (16 stage-3 runs): touch descriptive or unrun parts only.
  - A4: a wall-time estimate.
  - A5: records the lead-ordered stop and correctly marks K1c undetermined.
  - None changes a kill criterion. A1 to A4 precede every reported-seed solve.
- **analyze.py.** It was edited in the results commit. The diff 3facafa..cc5da99 is +36/-1, and all of it adds the labelled `witness_symmetry_diagnostic` plus one import. The kill logic is unchanged.
- **prereg_respected: yes.**

## Recomputed numbers (judge scripts, from committed raw files)

- **Witness checks.** All 916 witness boards (stage 1, curves, stage 2) rechecked independently: permutation, gray legality, pin, break count = UB, distance = witness_a inside the band. 0 failures. Every OPTIMAL record has LB = UB.
- **Band medians:**

| size, palette | instances | near | mid | far | K2 band | median B | mid above near and far |
|---|---|---|---|---|---|---|---|
| 5x5 C=13 | 30 (all closed) | 2 | 4 | 2 | 5 | 1.5 | 23/30 |
| 6x6 C=14 | 30 | 2 | [4, 4.5] | 2 | [5, 7] | 2 (both ends) | 25/30 |
| 5x5 C=9 | 10 | 2 | 2 | 2 | 2 | 0 | 1/10 |
| 6x6 C=10 | 10 | 2 | 2 | 2 | 2 | 0 | 1/10 |

  - 6x6 C=14 closures: mid 23/30, far 30/30, K2 13/30.
  - Near band has 0 breaks on 1/30 (5x5) and 2/30 (6x6) instances: a second solution close to the planted one.
- **K1b.**
  - Wilcoxon p = 0.0214 (unfavourable end), 0.00068 (favourable end).
  - Mean B: 5x5 1.37; 6x6 2.00 (LB) and 2.77 (UB).
  - Secondary Mann-Whitney p = 0.0031 (UB) and 0.024 (6x6 LB against 5x5).
  - Decomposition: mean H_mid 4.23 at 5x5, 4.40/5.17 (LB/UB) at 6x6. Mean H_far 2.87 to 2.40.
  - H_mid-only Wilcoxon p = 0.19 (unfavourable end), 0.015 (favourable end).
- **K2.** Median m2 is in [5, 7].
- **K3.**
  - b_ann(mid) = 5 at both sizes. Median H_mid - b_ann is in [-1, -0.5].
  - b_ann by band at 6x6: near 2, mid 5, far 8, K2 6. At 5x5: near 2, mid 5, far 7, K2 5.
  - p_f, p_i: 0.2031, 0.125 (5x5); 0.200, 0.1125 (6x6).
- **Radius.**
  - 5x5: median 4, all 10/10 certain.
  - 6x6: median 7, only 5/10 certain. The uncertain seeds give r = 8, 9, 9, 16 and 17; two of these exceed N/3 = 12.
- **Symmetry diagnostic.** 29/30 (5x5) and 28/30 (6x6) far witnesses lie within 4 cells of a turned planted copy.
- **Stage 2 (7x7 C=15).** Near = 2 on seeds 1 to 3. Seed 1: mid [3, 15], far 2 (closed in 16 s). Seed 2: mid [3, 15].
- **mu.** Primary palettes span 0.0090 to 0.0127. Contrasts span 0.061 to 0.167.
- **Agreement.** Every README number matches.

## Reruns (subset declared before running)

The full recipe exceeds 2 h, so I ran a subset declared in advance.

- **Generator and baseline.** `ncs instances` (all 140 instances) and `ncs annealed` (4 palettes) reproduce `results/instances.jsonl` and `results/raw/annealed.jsonl` byte for byte.
- **Tests.** `cargo test --release`: 5/5 pass.
- **Independent CP-SAT model.** Written by the judge, not by the runner:
  - integer piece, rotation and side-colour variables via AddAllowedAssignments;
  - breaks reified on colour inequality;
  - AllDifferent on pieces;
  - a different random seed.

  Planned: 70 solves at 120 s x 4 workers (5x5 C=13 seeds 1 to 10, 6x6 C=14 seeds 1 to 6, 5x5 C=9 seeds 1 to 3). I stopped it after 52. All 52 intervals are consistent with the committed records. 51 are closed in both runs, and all 51 optima are equal.
- **Turned-copy construction (no search) on all 140 instances.** The far-band boards it builds cost 2 or 4 breaks at every size: 7x7 C=15 9/10 at 2, 8x8 C=16 18/20 at 2, 10x10 C=18 11/20 at 2. So at every size the far band holds a 2-break certificate that needs no search.

## Leakage, effort, censoring

- **Leakage.** The planted board defines the distance by design. Its unscrambled piece order could change CP-SAT search speed but not a closed optimum, and the independent model agrees on every closed optimum.
- **Effort.** Mid and far get equal time budgets. The test makes no method-against-baseline node or wall-clock comparison, so a node win cannot hide a wall-clock loss. The annealed baseline is exact arithmetic, not a search.
- **Censoring.** [LB, UB] intervals are carried through every criterion and evaluated at both ends, as pre-registered. Correct.
- **Baseline strength.** The annealed model ignores the board's symmetric images. It misses the far band by 6 at 6x6 (b_ann 8 against a measured 2). K3 scores the mid band only, so nothing fires, but the "random-code spectrum" comparison is weakest exactly where the confound sits.

## Findings

| file:line | severity | claim | problem | fix |
|---|---|---|---|---|
| PREREG.md:38 | blocker (validity of K1) | "This removes the 4-fold board rotation." | A turned planted copy with the pinned corner restored is a far-band board with 2 or 4 breaks on 140/140 instances. It equals H_far on 30/30 (5x5) and 27/30 (6x6). The far floor is the solution's own symmetric image, not an orthogonal floor. | Define distance as the minimum over the 4 turned planted copies, or exclude boards within r cells of any turned copy from the far band. |
| README.md:91 and scripts/analyze.py:168-175 | blocker (meaning of K1b) | The barrier grows from 5x5 to 6x6 (p = 0.021). | At the unfavourable end, about 75% of the mean growth (0.47 of 0.63) comes from H_far falling: corner swaps cost 4 breaks on 13/30 boards at 5x5 against 6/30 at 6x6. H_mid alone gives p = 0.19. | Test growth on H_mid, or on B against a far floor computed with the symmetric copies removed. |
| PREREG_AMENDMENTS.md:50 | blocker (completeness) | K1c and stage 3 | 1/10 usable instances at 7x7 and 0/10 at 8x8. Stage 3 was not run, so hypothesis part 3, and part 2 at a solver plateau, are untested. The outcome rule (PREREG.md:207) needs all criteria decided. | Finish K1c at 7x7. The turned copy is a free far certificate, so only mid-band solves are needed. |
| PREREG.md:20-22 | should-fix | "the barrier ... is at least 2 breaks and grows" | Median B at 5x5 is 1.5, so the hypothesis's own threshold fails at 5x5. K1a fires only at <= 0, so this does not kill. | State that the >= 2 part holds at 6x6 only. |
| PREREG.md:201-202 | nitpick | K2 band capped at floor(3N/4) | The statement gives no upper end. The cap is the only reading under which the turned copy does not fire K2 trivially. It was pre-registered, so it stands. | State the reason for the cap. |
| README.md:55-57 | should-fix | annealed baseline as the random-code spectrum | b_ann(far) = 8 at 6x6 against a measured 2: the first-moment count leaves out the 3 symmetric images. | Add the turned copies to the annealed count, or report b_ann only for the near and mid bands. |
| README.md:61-63 | should-fix | median near radius 7 at 6x6 | Only 5/10 seeds are certain. Two uncertain seeds give r = 16 and 17, above N/3 = 12. The radius is taken at a per-seed curve threshold, not at the solvers' plateau. | Report the certain radii separately, and state that part 2 at a solver plateau was not measured. |
| src/lib.rs:158-169 | nitpick | R(k) counts arrangements where every interior cell changes | A piece whose edge tuple has 2-fold symmetry, placed in its own cell turned by a half turn, keeps the same tuple. R(k) counts it as changed; the CP-SAT distance does not. Pre-registered (PREREG.md:132), and the effect is tiny. | Merge tuple-identical rotations in R(k). |
| README.md:68-83 | positive | symmetry diagnostic | Labelled as outside the prereg. It changes no verdict and exposes the main confound. | none |

Prose: README, PREREG and AMENDMENTS contain no em dashes and no banned phrases. Claims stay within the data, and the self-assessed outcome (inconclusive) matches it.

## Next-round test (about 1 h wall, 8 cores)

**Overlap gap with the four rotations quotiented out, at 5x5 to 7x7.**

- **Distance.** d*(board) = minimum over k = 0..3 of the colour-tuple distance to the planted board turned k times, with no pin. In CP-SAT: four distance expressions, each constrained >= lo, plus a Boolean disjunction saying at least one is <= hi.
- **Instances.** The committed 5x5 C=13 and 6x6 C=14 seeds 1 to 30, plus 7x7 C=15 seeds 1 to 10.
- **Bands on d\*.**
  - near = [1, ceil(N/4) - 1];
  - mid = [ceil(N/4), floor(3N/4)];
  - the far band is replaced by "distinct": minimum breaks subject to d* > floor(3N/4). Record whether a 0-break board exists there, i.e. a second solution that is not a symmetric copy.
- **Kill criteria (written before the run).**
  - K1' fires if the median of (H_mid* - max(H_near*, H_distinct*)) is <= 0 at 5x5 or 6x6.
  - K1' also fires if the one-sided Wilcoxon on H_mid* growth from 5x5 to 6x6 gives p > 0.1 at either interval end.
  - K1c' at 7x7: mid* solved at 300 s x 4 workers per seed (about 25 min). It fires if >= 3/10 seeds have mid* UB <= H_distinct* UB.
- **Setup.** Same OR-Tools version as this run, same instance generator.
