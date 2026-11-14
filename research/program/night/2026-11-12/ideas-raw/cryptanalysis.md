# Cryptanalysis idea: dissection of the board into seam-guessed bands

## Stall attacked

Two quotes, one stall:

- `research/LEDGER.md` (2026-11-11, round 2 hard-instance ladder): "Learning: on this generator the counts at or just below the critical point are the hardest".
- `research/experiments/mrv-baseline/JUDGE.md` (line 5): "at R6 and R7 bb-mrv fails G1 and is slower in wall clock, and Q2 is uninformative above n=8 because every pair is censored."

So at the critical colour count, the strongest piece-side propagation stops paying for itself (R6 node ratio 0.80, R7 0.70, slower in wall clock), and above 8x8 both complete solvers are censored on every pair. This attacks the critical-colour half of that stall. Why it fits: at few colours, each edge removes only about 1/c of candidates, so local propagation has little to cut. The same weak selectivity moves the peak of the partial-fill "bulge" late in the fill. The site's own model on `build/exact/meet-in-the-middle` says a two-sided split "saves only the distance between the midpoint and the peak". Under that page's formula T(a,k), the peak sits where (pieces left) x 4 x p^2 is about 1, that is near a* = N - c^2/4. That gives about 83% of the cells at 6x6 c=5 and 82% at 7x7 c=6, against about 61% at 8x8 c=10. This is an uncommitted model estimate that ignores the frame, and the cheap test is there to check it. So critical-colour boards are where splitting the board on a seam should pay most, and where propagation pays least.

## Field and mechanism

Cryptanalysis: **dissection of bicomposite problems**. Multiple encryption with r rounds is a chain of r steps through a narrow intermediate state. Dissection does not store a full half-table. It *guesses* an intermediate state in an outer loop. For each guess, it runs independent meet-in-the-middle attacks on the sub-chains on either side, each now fixed at both ends. Memory falls to the size of the sub-chain lists, and time stays near plain meet in the middle (r = 4: time M^2, memory M, where plain MITM needs memory M^2).

On the board, cut it into 4 horizontal bands B1..B4. The intermediate state is the colour vector on a seam (n edges, at most about c^n values: about 5^6 at 6x6 c=5). The outer loop is over the middle seam s (between B2 and B3). For each s:

- (i) B1's list, keyed by its lower seam and its piece set, is built once and stored.
- (ii) B2 is enumerated with its lower boundary fixed to s, and each fill is joined against B1 on seam s1 plus a disjoint piece set.
- (iii) The mirror-image steps run for B3 and B4.
- (iv) The top-half and bottom-half candidates for s are joined on an exact complementary piece set (the whole pool is used, so the bottom key is the complement of the top key: an exact hash key, with no phantom boards).

Within a band, enumeration uses the ladder's propagation, plus the global colour-supply check against the unused pool. That check is the only far-side constraint a band can see.

## Original paper

- I. Dinur, O. Dunkelman, N. Keller, A. Shamir, "Efficient Dissection of Composite Problems, with Applications to Cryptanalysis, Knapsacks, and Combinatorial Search Problems", CRYPTO 2012, LNCS 7417, pp. 719-740, DOI 10.1007/978-3-642-32009-5_42; extended version IACR ePrint 2012/217 ("Efficient Dissection of Bicomposite Problems with Cryptanalytic Applications", revised 2019). Verified on eprint.iacr.org and the Springer record.
- Precursor (the r = 4 case): R. Schroeppel and A. Shamir, "A T = O(2^{n/2}), S = O(2^{n/4}) Algorithm for Certain NP-Complete Problems", SIAM J. Comput. 10(3):456-464, 1981, DOI 10.1137/0210033.

## Hypothesis

H1, which is needed for anything to work: on critical-colour boards, a two-sided exact plan (store one half, stream the other, join on the seam plus the complementary pool) enumerates fewer partial states than the ladder's best complete DFS needs to reach the first solution.

H2, which says the stated reason is right: the advantage is larger at the critical colour count than at critical + 2, because the bulge peaks later there.

H3, the dissection-specific part: guessing the middle seam cuts peak stored entries by at least 10x against the plain two-band plan, at no more than 4x its total enumerated states. If true, the plan still fits in RAM at the sizes (9x9 and up) where DFS is censored.

## Nearest explored cell and missing mechanism

The cell is "Meet in the middle (BANDSAW), mapped, open: about 20x memory per extra mismatch". The concept page also says four quadrants are worse: "the pairwise product balloons before the cross-constraints from the other two quadrants can prune it".

Mechanisms BANDSAW and that page do not have:

- (a) The guessed intermediate state. The quadrant critique prices a 4-cycle join that materialises pairwise products. Dissection runs on a chain of bands and never materialises an unconstrained product, because every inner enumeration (B2, B3) has both boundaries fixed by the guess. This is the published remedy for exactly the memory wall the cell records.
- (b) Use as a full-board exact solver on the critical-colour ladder, scored on G1 against complete propagating DFS. BANDSAW is an endgame-band bracket tool on 16x16 boards with mismatches, and the bulge argument on the page is model-only and uncommitted.

## Cheapest falsifying test (6x6 or 7x7)

**Stage 0, a profile only, with no join code.** It decides H1 and H2 before anything is built.

- Instances: hard-ladder R6 (6x6 c=5) and R7 (7x7 c=6) test seeds. As a control, the plus2 track at 6x6 c=7 and 7x7 c=8, seeds 1..30.
- For each instance and each split row h, count with the ladder's propagation the top-band partial boards over rows 1..h, and the bottom-band partial boards over rows h+1..n. Count each try_assign as a node, the same accounting as the ladder. Bucket the counts by seam vector.
- Two-band cost: C2 = min over h of (N_top(h) + N_bot(n-h)). Report join probes separately.
- Four-band dissection cost C4 and its peak memory both follow exactly from the same bucketed counts: the sum over s of the B2 and B3 states with s fixed, plus the B1 and B4 lists, and the maximum stored list.
- Per-band cap: 1e8 states. A censored count is a lower bound, which can only strengthen a kill.

**Stage 1, only if stage 0 passes.** Implement the dissection solver and run the ladder's G1 gate at R7, with streaming and a stop at the first join hit.

## Baseline and control

- Baselines: `scb-gacolor_ac3_random` (ladder baseline, 3 run seeds) and `bb-mrv`. Use the better of the two per rung, nodes to first solution, same try_assign counter. Also report the DFS exhaustive count where it finishes under 1e8, so the comparison is not only against a lucky first hit.
- Control for H2: the same ratio on the c + 2 track at the same sizes.
- Control for H3: the plain two-band plan, which is dissection with no guess.

## Kill criterion

- **K1 (kills the whole line).** The median over instances of C2 / (better DFS nodes to first solution) is at least 1 at R7, or at least 1 at R6. Dissection can only spend more time than C2, so this also kills stage 1.
- **K2 (kills the mechanism story even if K1 does not fire).** The median log10 ratio at critical colour is not lower than at c + 2, with a paired bootstrap 95% CI over instances that includes 0 or lies above it, at both sizes.
- **K3 (kills dissection in favour of plain MITM).** Median peak stored entries for C4 not at least 10x below C2's, or median C4 / C2 above 4, at 7x7.
- **Stage 1 kill.** G1 NO-GAIN at R7 against the better DFS, or a median paired wall-clock ratio above 1.

## Budget

- Stage 0: one session, 2 to 4 core-hours on the VM (enumeration with a 1e8 cap per band, 60 critical and 60 control instances), CPU only.
- Stage 1, only if stage 0 passes: one session plus 2 to 4 core-hours.
- No AWS.

## Closest prior work

- The project's BANDSAW and the `build/exact/meet-in-the-middle` concept page (Horowitz and Sahni 1974, two-band split, complementary-pool join, the bulge model, the four-quadrant critique).
- Schroeppel-Shamir 1981, the 4-list special case of dissection for knapsack.
- On the solver side, cube-and-conquer, which also splits on guessed variables (`build/exact/sat-csp-encodings`).

## Why this is not a variant of it

- **BANDSAW and the MITM page.** They store one side of a single seam and price four quadrants as a cycle join. Neither guesses an interface state as an outer loop to split the problem into independently solved sub-chains with both ends fixed, the step that changes the memory exponent. BANDSAW also never ran as an exact full-board solver against complete DFS on the critical-colour ladder, and its bulge argument was never measured. This test measures it first and lets it kill the idea.
- **Schroeppel-Shamir.** It works on additive knapsack with independent halves. Here the bands are coupled through one piece pool, so the join key carries the exact complementary piece set, and there is no knapsack analogue of in-band propagation.
- **Cube-and-conquer.** It splits on cell assignments chosen by lookahead and solves each cube with one-sided search. Dissection splits on an interface colour vector, whose value space (c^n) is far smaller than the cell assignments that produce it, and it solves each guess two-sidedly with a join.

## Expected failure mode, stated up front

K1 is the likely outcome. Complete propagating DFS needs about 1e3 nodes at R6 and about 1e6 at R7, while an unconstrained half-board can hold far more fills than that. If K1 fires, the result still settles the site's uncommitted bulge model with a committed measurement at small size, which is worth a ledger line.
