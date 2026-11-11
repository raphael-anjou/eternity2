# JUDGE: near-codeword spectrum (round 2, fix run)

Branch `devin/r2-near-codeword-spectrum`, final commit ffc029e, prereg commit bf2fecb. Judge session devin-23bea9004bf046028e5c03239f3c94a0. The branch was not modified.

## Verdict: kill

The pre-registered K1a fires, decided, on fully closed data. At 5x5 C=13 (30 instances, every mid* and far* solve OPTIMAL) the median barrier B* = H*_mid - H*_far is -3: the cheapest board far from every turned copy of the planted solution (median 7 breaks) costs more than the cheapest mid-overlap board (median 4). At 6x6 C=14 the median B* is in [-7, -1], <= 0 at both ends. An independent CP-SAT model written by the judge reproduces all 60 5x5 mid*/far* optima exactly and proves the 6x6 far* lower bound (no far* board with <= 4 breaks) on 5 of 6 rerun seeds. The hypothesis "a mid-band barrier in front of a distinct low-break far floor" is false at the sizes the prereg makes its exact claims at.

Conditions on the verdict:

1. The kill rests on amendment A6 (rotation-quotiented distance d*). On the pre-registered distance d_0 nothing fires. I accept A6 as a correction, not a post-hoc rescue: the binding statement defines H_far as the "far (orthogonal) floor", round 1 proved that d_0's far band holds the planted solution's own turned image (a 2 or 4 break board on 140/140 instances), A6 keeps every threshold, test and outcome rule, it was committed (da4d6e6, 15:13 UTC) before any fix-run solve started (15:15 UTC), and it is the quantity the round-1 judge prescribed. The amendment also moves the result against the hypothesis's interest, so it is not a fishing step toward a positive effect.
2. Scope: no primary instance has a 0-break far* board (proved at 5x5), so at these sizes the planted puzzle has a single solution class and far* simply follows the first-moment curve (far* 7 against b_ann(far) 7 at 5x5). The kill says there is no barrier on mu-matched 5x5 and 6x6 boards; it does not speak to a landscape with many distinct solutions, which is where the 470-wall conjecture lives.
3. K1c* (7x7 on d*) "fires" by the letter but carries no evidence; see findings.

## Prereg check

- **Chronology.** PREREG.md was committed alone in bf2fecb (12:59:44 UTC) and never edited afterwards (`git log -- PREREG.md` shows only bf2fecb). Code and A1-A4: 3facafa (13:17). First results: cc5da99 (14:56). Round-1 report, A6-A10 and `cpsat_quotient.py`: da4d6e6 (15:13), no result files. Fix-run results, `analyze_fix.py`, `check_fix.py`, A11: ffc029e (17:18). The da4d6e6..ffc029e diff of PREREG_AMENDMENTS.md only appends A11 (a witness recheck, read by no criterion); `run_fix.sh` only gains the `check_fix.py` call.
- **Kill criteria vs the binding statement.** K1a (median B <= 0 at 5x5 or 6x6), K1b (one-sided Wilcoxon, fires at p > 0.1), K1c (>= 25% of 7x7/8x8 instances with mid <= far at equal budget, certificates only), K2 (median m2 <= 2 at 6x6 on a >= n/3 band), K3 (|median H_mid - b_ann| > 2 at 6x6) match the statement. The K2 cap at floor(3N/4) was accepted in round 1.
- **Amendments.**
  - A1-A5: judged in round 1, no criterion change.
  - A6: distance corrected to d*; thresholds unchanged. Far band upper end moved from N-1 to N, which d* requires (d_k can reach N for k != 0). Accepted, see verdict condition 1.
  - A7: K1c on d_0 with the free turned-copy far certificate and a capped mid solve. Using the certificate in place of a 300 s far solve can only make K1c easier to fire; UNKNOWN counted as "no mid board found" matches the statement's "best mid-band board found". Accepted.
  - A8: stage 3 dropped. Hypothesis part 3 untested; it cannot change a kill.
  - A9: K1c* on d* at 7x7 with 150 s per band (prereg 300 s). A new criterion added after the round-1 report; I give it no weight either way.
  - A10, A11: reporting and an extra check.
- **prereg_respected: yes.**

## Recomputed numbers (judge script `recompute.py`, from committed raw files, no runner code imported)

- Band ranges of all 360 fix-run records match the stated bands. Witnesses: 340 rechecked with my own grid, quarter-turn and break code: permutation, gray legality, pin, breaks = UB, band on d* (d_0 for k1c), cap, and all four d_k equal to the recorded `witness_dk`; 0 failures. Every OPTIMAL record has LB = UB. The runner's `check_fix.py` also reports 340 / 0.
- d* bands, medians [unfavourable, favourable], closed counts:

| palette | near* | mid* | far* | K2* | median B* | B* <= 0 certified | mid* above near* and far* |
|---|---|---|---|---|---|---|---|
| 5x5 C=13 (30) | 2 | 4 (30/30) | 7 (30/30) | 4.5 (30/30) | -3 | 30/30 | 0/30 |
| 6x6 C=14 (30) | 2 | 4 (26/30) | [5, 12] (0/30) | 5 (17/30) | [-7, -1] | 26/30 | 0/30 |
| 5x5 C=9 (10) | 2 | 2 | 2.5 | 2 | -0.5 | 10/10 | 0/10 |
| 6x6 C=10 (10) | 2 | 2 | 2 | 2 | 0 | 9/10 | 0/10 |

  - 5x5 C=13 far* optima: 6 (4 instances), 7 (15), 8 (11). 6x6 far* LB 4 to 6, UB 8 to 14; far* witnesses at d* 28 to 32. far* = 0 on 0 primary instances; near* = 0 on 1/30 (5x5) and 2/30 (6x6).
- K1a*: fires at 5x5 (-3, closed) and at 6x6 (both ends <= 0).
- K1b*: Wilcoxon p = 0.99999931 (unfavourable), 1.64e-6 (favourable); mean B* -3.2 (5x5), [-7.30, -0.23] (6x6). Undecided. Secondary H*_mid growth p = 0.125 / 0.042.
- K2*: median m2 = 5 (both ends). Does not fire.
- K3*: b_ann(mid) = 5 at 6x6; median H*_mid - b_ann = -1 (both ends). Does not fire. b_ann near/mid/far: 5x5 C=13 2/5/7, 6x6 C=14 2/5/8, 5x5 C=9 1/2/3, 6x6 C=10 1/3/3.
- K1c (d_0): 7x7 cap 2 on 9 seeds, all INFEASIBLE; cap 4 on 1 seed, UNKNOWN. 8x8 cap 2 on 9, cap 4 on 1, all 10 UNKNOWN. 0/20 mid <= far. Does not fire.
- K1c* (7x7, d*): mid*/far* per seed [LB, UB]: s1 [3,17]/[3,15], s2 [3,15]/[3,16], s3 [3,15]/[3,15], s4 [3,15]/[3,15], s5 [3,9]/[4,17], s6 [3,15]/[3,16], s7 [3,12]/[3,19], s8 [3,13]/[3,18], s9 [3,16]/[3,18], s10 [3,6]/[3,15]. Mid UB <= far UB on 9/10 (2 exact ties, 7 strict). No solve closed.
- mid* against the first run's d_0 mid: per-instance intervals equal on 24/30 (5x5) and 20/30 (6x6). At 5x5 mid* is lower by 1 on the other 6 instances (both closed). Medians are equal (4).
- All README and summary_fix.json numbers above match.

## Reruns (subset declared before running)

Full `run_fix.sh` is about 2.1 h wall, so I ran a declared subset with my own CP-SAT model (placement-index IntVar per cell with AddElement for piece, side colours and the four planted-tuple matches; AllDifferent on pieces; breaks reified on colour inequality; random_seed 777 + seed; 120 s x 4 workers; OR-Tools 9.15.6755):

- S1: 5x5 C=13 seeds 1-30, mid* and far*. 60/60 OPTIMAL, 60/60 equal to the committed optima.
- S2: 6x6 C=14 seeds 1-6, far* with breaks <= 4. INFEASIBLE on seeds 1, 2, 3, 4, 6 (15 to 49 s); UNKNOWN on seed 5 (committed LB 4). Confirms far* >= 5 where the committed LB says 5.
- S3: 6x6 C=14 seeds 1-6, mid*. 4 OPTIMAL at 4 (equal to committed), seeds 4 and 5 [4,6] and [4,5], overlapping the committed [4,10] and [4,12].

72/72 consistent. The K1a* kill at 5x5 is therefore independent of the runner's model.

## Leakage, effort, censoring

- **Leakage.** The planted board defines the distance by design. Pieces are unscrambled (piece i at cell i), which could change search speed but not a closed optimum; my model uses a different encoding and agrees on every closed value.
- **Effort.** Mid* and far* get equal budgets (120 s x 4 in star1, 150 s x 4 in star7). No method-vs-baseline node or wall-clock comparison is made, so a node win cannot hide a wall-clock loss. The annealed baseline is exact arithmetic.
- **Censoring.** Intervals are carried through every criterion at both ends. 6x6 far* never closed; the 6x6 K1a* verdict uses its LB (favourable end, median -1), which my capped solves confirm. The 5x5 verdict needs no bound at all.
- **Baseline.** b_ann ignores the three turned images, which is why A10 reports it only for near and mid. With the images quotiented out, measured far* (7) equals b_ann(far) (7) at 5x5: the far side behaves like a random code with no second codeword, which is the reading the kill rests on.

## Findings

| file:line | severity | claim | problem | fix |
|---|---|---|---|---|
| README.md:12-19 | should-fix | "K1a fires at 5x5 and at 6x6; K1c* at 7x7 also fires" | K1c* compares upper bounds of 20 unconverged solves (LB 3 or 4 on all, UB 6 to 19). It is a criterion added after round 1 (A9) at half the prereg budget, and its count of 9/10 says nothing about the landscape. | Headline the kill on K1a* only; report K1c* as "fires by the letter, uninformative". |
| README.md:115 | nitpick | "7 of its 9 counts are ties or near ties" | Of the 9 seeds with mid UB <= far UB: 2 exact ties, 3 within 1 to 2 breaks, 4 differ by 5 to 9. | "2 ties and 3 within 2 breaks". |
| README.md:124-125 | should-fix | "Mid* is identical to the first run's mid band at these sizes (median 4)" | Medians are equal, instances are not: 6/30 5x5 instances have mid* one break below d_0 mid (both closed), 10/30 differ at 6x6. The pinned d_0 mid band excludes boards at mid distance from a turned copy, so mid* <= mid. | State "same median; mid* <= mid, lower by 1 on 6/30 at 5x5". |
| README.md:113-120 | should-fix (scope) | "the overlap-gap picture ... is false at small size" | Correct for these instances, but none has a distinct second solution (far* = 0 on 0/60), so B* <= 0 is close to forced: with one solution class the far side is the expensive end of a monotone spectrum. The 470-wall conjecture is about landscapes with many distinct solutions. | Add: "at these sizes the planted puzzle has one solution class; the test does not reach the many-solution regime". |
| PREREG_AMENDMENTS.md:72-103 (A6) | should-fix (disclosure) | kill decided on d* | On the pre-registered d_0 nothing fires; the kill is conditional on A6. Accepted (binding statement's "orthogonal floor", thresholds unchanged, committed before the fix run), but a reader of the headline cannot see it. | One line in the README headline: "on the corrected distance d* (A6); on d_0 no criterion fires". |
| README.md:107 | nitpick | K1c 8x8 "decided, does not fire" | All 10 capped 8x8 solves are UNKNOWN, none INFEASIBLE; the verdict follows the "best found" rule but is censored, unlike 7x7 (9 proofs). | Say "no board found in budget (no proof)". |
| PREREG_AMENDMENTS.md:132-146 (A9) | nitpick | K1c* added as a K1 part | A criterion added after a judge report can only be informative if it is decided on closed bounds; at 150 s nothing closed. | Next round: give K1c* a proof requirement or drop it. |
| README.md:83-86 | positive | "no 0-break board in far*, proved absent at 5x5" | Verified: 30/30 far* OPTIMAL at 6 to 8; independent model agrees. | none |
| scripts/check_fix.py | positive | independent witness check | Agrees with the judge's own checker, 340/340. | none |

Prose: README.md, PREREG.md and PREREG_AMENDMENTS.md contain no em dashes and none of the banned phrases. Stage 3 and part 2 at a solver plateau are listed as untested.

## Next-round test (about 1 h wall, 8 cores)

**Overlap gap where distinct solutions exist.** At mu-matched 5x5 and 6x6 the planted puzzle has one solution class, so the gap cannot be tested there. Pick palettes where it can.

- Instances: `generate_solved_framed` at 6x6 and 7x7, palettes chosen so the first-moment expected number of rotation classes of solutions is about 5 to 50 (compute from the instance census as in the annealed DP), seeds 1..40.
- Step 1: CP-SAT far* with breaks = 0 (the judge's element model proves 6x6 far* infeasibility in under 50 s). Keep only instances with a distinct far* solution S2; record the pairwise d* between planted S1 and S2.
- Step 2: H_between = min breaks over boards whose d* to S1 and d* to S2 both lie in [0.25 d(S1,S2), 0.75 d(S1,S2)]; H_near = min breaks within 0.25 d of either.
- Kill (written before the run): median over kept instances of H_between - H_near <= 0, or fewer than 10 instances kept at either size (then the regime is not reachable at small size and the conjecture stays untested). Survives if the median is >= 2 at both sizes.
