# Referee review of `growth-law.md` (night 2026-11-12)

Independent recomputation. Scripts: `research/program/night/2026-11-12/memos/growth-law-review/` (stdlib Python, `python3 -I <script>`, plus one C file, `enum.c`, compiled by `rvlib.py` into the temp directory or `RV_BIN_DIR`). Compute used: about 7 minutes on one core. No file other than this one and the scripts was written.

The review's row-major DFS (`enum.c`) reproduces 159 of 159 committed solved `hp-dfs` rows node for node (6x6 c=5, 6, 7; 7x7 c=7, 8; 8x8 c=10), so its counts on new rungs are `hp-dfs` counts.

## 1. Verdict table

| claim | verdict | probability true as stated | one-line reason |
|---|---|---|---|
| A. Parameter-free model predicts median row-major DFS within about 2 on 14 above-critical rungs | holds with a correction | 0.9 | Reproduced independently (0.86 to 1.96). The error is one-sided (model high on every rung with a bulge) and reaches 2.0 to 2.1 on two rungs the memo could not test (7x7 c=6, 9x9 c=11), which I ran. |
| B. Whole-board critical colours are 3.9, 5.3, 6.6, 7.9, 9.3; every "critical" rung is above critical; the 16x16 is at critical | wrong | 0.03 | The first moment uses independent uniform colours. Generated boards have exact colour counts and a two-sided frame ring. Enumeration: 6x6 c=4 has 357 solutions per board (memo formula: 0.39). Corrected critical counts: 4.65, 5.99, 7.32, 8.63, 9.94. |
| C1. Propagation and cell order change node counts by a surface factor only | not established | 0.65 | Provable for a static order with bounded lookahead, with exp(O(n ln n)) not exp(O(n)). No argument covers a dynamic order. At n <= 16 the surface allowance is as large as the bulk, so the statement forbids nothing measurable. |
| C2. Reasoning that relaxes piece distinctness cannot refute a node before the peak | wrong as stated | 0.02 (0.8 for "cannot change the bulk rate") | Measured: a distinctness-free check of the next two rows refutes 37% (6x6 c=5) and 57% (7x7 c=8) of the nodes that lie more than one row before the peak. |
| D1. DFS / `bb-mrv` node ratio does not grow with n | holds | 0.85 | Paired medians 9.9, 15.7, 5.4 at n = 6, 7, 8 (+2 track); about 2 at n = 9 with measured DFS (4 informative pairs, interval 0.75 to 8.4). |
| D2. A22's "gain grows with n" is an artefact of scb's border-first order | not established | 0.3 | The trend is scb losing its lead over plain DFS (5.2, 4.6, 0.89, at most 0.83). Border-first explains a factor 1.8 of the 6.0 at n = 8; the missing piece all-different explains 4.4. The memo's frame counts are wrong by about 600. |
| E. Only merging and two-sided search change the bulk rate; two-sided gives 1/e to (ln 2)/2 at critical, nothing for x >= 1/2 | holds with a correction | 0.85 for the algebra, 0.5 for "only two families" | Algebra reproduced numerically. "Only two" is a statement about the model, not a result. On enumerated 6x6 boards two-sided listing never beats first-solution DFS. |
| F. The failed `bb-mrv` prediction at 9x9 | the failure is in the assumed gain, not in the DFS model | n/a | Measured DFS median at 9x9 c=11 is 1.56e8 (model 3.3e8). The gain over DFS is about 2 or less, not 7 to 14. |

## 2. Claim by claim

### A. First-moment model against plain row-major DFS

What I did. Reran `model.py`, `medians.py`, `table_hp.py`, `compare_full.py`, `regime.py`: all tables in sections 3 to 5 of the memo reproduce (`mc_small.py` not rerun, 81 s). Then wrote an independent model (`a_dfs_model.py`: own profile, own Monte Carlo over value orders, no code shared with `model.py`) and read the raw jsonl rows with an own loader.

Numbers (model median / measured KM median, with the memo's depletion correction (ii) and without it):

| rungs | with (ii) | without (ii) |
|---|---|---|
| all 14 solved rungs with naive ln S < 0 (18 comparisons across sources) | 0.86 to 1.96, geometric mean 1.32 | 0.98 to 2.23, geometric mean 1.57 |
| the 8 rungs with an interior bulge (x < 1) | 1.08 to 1.96, geometric mean 1.52 | 1.43 to 2.23, geometric mean 1.83 |
| the 6 rungs with x >= 1 (8x8 c = 12 to 17) | 0.86 to 1.42 | 0.98 to 1.70 |

How the 14 were chosen: every (n, c) with a finite KM median and c above the memo's critical count. No solved above-critical rung was left out. Six of the 14 have x >= 1, where there is no bulge and the cost is the first frame row: they test the frame-row arithmetic, not the growth law. The below-critical failures (8x8 c = 1 to 5, off by many decades) are stated in the memo.

Rungs left out because censored, which I ran (`a_dfs_model.py run`, each run a few seconds):

| rung | cap | solved | measured KM median | model median (with (ii), without) | ratio |
|---|---|---|---|---|---|
| 7x7 c=6, primary seeds 1..30 | 2e8 | 30/30 | 40,171,900 | 8.1e7, 1.04e8 | 2.0, 2.6 |
| 8x8 c=8, primary seeds 1..30 | 1.5e8 | 2/30 | above cap | 1.2e9; model P(below cap) 0.08, expected 2.4 solved | consistent |
| 9x9 c=11, plus2 seeds 1..30 | 2e8 | 18/30 | 155,672,958 | 3.3e8, 4.0e8; model P(below cap) 0.32, expected 9.6 solved | 2.1, 2.5 |
| 9x9 c=11, pilot seeds 101..108 | 2e8 | 5/8 | 161,184,835 | 3.3e8 | 2.0 |

Ruling. The agreement is real and does not come from tuning: correction (ii) moves the ratio by a factor 1.1 to 1.3, and without it the model is still within 2.6 everywhere. The corrected statement: the model is high on every rung that has a bulge, by 1.1 far above critical rising to 2.0 to 2.1 near critical and at n = 9. It is a bias, not scatter, and it grows in the direction of the rungs the program cares about. The memo's 0.7 on "within 2.5 at 9x9" comes out true (2.1). The model's frame terms are wrong at full depth (see B) but the first-solution cost is set near the peak, where the error has not yet accumulated.

### B. Critical colour counts

First principles. A solution is an arrangement: 4! placements of the corners, (4w)! of the edge pieces, P! 4^P of the interior pieces (w = n - 2, P = w^2), all four turns of the board counted. There are 4(n-1) ring seams and 2w(w+1) other seams. The memo multiplies by 1/5 per ring seam and 1/c per other seam (`model.py`, `critical_c`). That is exact only for pieces with independent uniform colours. I checked that bookkeeping on random (not planted) 4x4 piece sets with independent colours: measured mean 354.39 solutions (se 62) against the formula's 354.375 at cf = 2, c = 2.

The generator does not draw independent colours. `paint_framed` cycles the colours over each band, so every colour has an exact count, and every ring seam joins a clockwise-facing side to an anticlockwise-facing side. The matching probabilities for that ensemble are:

- ring: prod f_k! / E_F! with f_k the seams of frame colour k and E_F = 4(n-1);
- interior band: prod (2 e_k - 1)!! / (2 E_I - 1)!! with e_k the seams of colour k.

This is the formula on the site's solution-counting page. With 17 + 5 colours at n = 16 my implementation returns 1.153e7 with no clue and 14,702.3 with the starter pinned, the canonical figure. The memo's value for the same board, S = 16.4 (ln S = 2.8), is the page's "naive product" (0.02 with the starter, times 784), which the page itself says is 7e5 times too small.

Exact enumeration on generated boards (`b_solutions.py`; own copy of the painter, boards with two pieces equal up to rotation redrawn; committed instances where they exist):

| quantity | measured | memo's formula | balanced formula |
|---|---|---|---|
| valid frames, 6x6 (200 generated; 30 committed) | 2,570; 2,596 | 5.3 | 1,643 |
| valid frames, 7x7 (12 generated; 10 committed) | 6.2e5; 7.9e5 | 980 | 4.7e5 |
| valid frames, 16x16 (Owen's census, site) | 4.05e37 | 2.0e34 | 5.17e37 |
| solutions beyond the planted four, 6x6 c=4 (16 boards) | 353 | 0.39 | 340 |
| same, 5x5 c=3 (300 boards) | 31.6 | 0.025 | 11.4 |
| same, 6x6 c=5 (30 committed) | 1.2 | 5e-5 | 0.063 |
| same, 6x6 c=6 and c=7 (30 committed each) | 0.27 | below 1e-7 | below 1e-4 |

At 6x6 c=4, the count the memo places at critical (3.91), every one of 16 boards has more than 4 solutions and the mean is 357. The balanced formula is within 4% there. Far above critical the few extra solutions (0.27 per board) are local variants of the planted board, which no annealed formula counts.

Corrected table (first moment equal to 1, all four turns counted):

| n | 6 | 7 | 8 | 9 | 10 | 12 | 16 |
|---|---|---|---|---|---|---|---|
| Owen | 5.22 | 6.38 | 7.56 | 8.74 | 9.93 | 12.33 | 17.14 |
| memo | 3.91 | 5.25 | 6.60 | 7.94 | 9.27 | 11.91 | 17.11 |
| balanced (this review) | 4.65 | 5.99 | 7.32 | 8.63 | 9.94 | 12.54 | 17.68 |

Consequences.

- The primary rungs are at +0.35 (6x6 c=5), +0.01 (7x7 c=6) and +0.68 (8x8 c=8) colours from critical, with ln S = -2.8, -0.1, -7.2 (memo: +1.1, +0.8, +1.4 and -9.9, -8.0, -16.1). The secondary 8x8 c=7 is 0.32 below. The +2 track is 2.0 to 2.7 above.
- The ladder README derives its colours from Owen's interior count rounded to an integer. That choice lands within 0.7 colours of the balanced whole-board count at every size, and Owen and balanced coincide at n = 10.
- The 16x16 with 17 + 5 colours and no clue is 0.68 colours below critical (1.15e7 expected solutions). With the starter it has 14,702; with five clues 3.9e-8 besides the planted one. In colour units (one colour is 24.7 nats there) the five-clue puzzle is 0.69 colours above critical, which is where the primary 8x8 rung sits. The memo's sentence "with five clues the real puzzle is again at S of order 1" is wrong: 14,702 / 784^4 = 3.9e-8.
- The memo's explanation of 2.0e34 against Owen's 4.05e37 (planted cycle terms) is wrong. The balanced annealed count gives 5.17e37 with no planted term at all.

One thing the memo gets right in direction. The first-solution cost of row-major DFS at 6x6 peaks at c = 4, not at the primary c = 5 (`b_peak.py`, 24 generated boards per colour, cap 1e8: medians 59,299 at c=3 with 20/24 solved, 167,241 at c=4, 39,310 at c=5, 2,884 at c=6). So the primary 6x6 rung is about one colour to the easy side of the DFS hardness peak. The reason is not that c = 4 has one expected solution (it has about 350): the peak of first-solution cost sits on the many-solutions side of the one-solution point. At 5x5 the cost falls monotonically from c = 3 up.

Literature. Martinsson's heuristic is the same count without a frame and with independent colours; it does not bear on the frame correction. Edmonds, Edmonds and Mertz note the extra edge-to-interior seams that the memo also counts. Neither treats exact colour counts.

### C. Surface against bulk

C1. What can be proved. For a static order and a propagator whose refutations are all implied by "the next region R has no consistent filling", every surviving partial board on S corresponds to at least one consistent board on S plus R, and one board on S has at most (4P)^|R| fillings of R. So the propagating count at depth d is at least Z_dfs(d + |R|) / (4P)^|R|, deterministically. With |R| of order n this is a factor exp(O(n ln n)), the same class as the memo's frame entropy and perimeter terms. The memo's "exp(O(n))" in item 6 should read exp(O(n ln n)), as its own item 2 says.

Where the argument stops.

1. Dynamic order. With MRV the filled set depends on the branch. The count at depth d is then a sum over sets chosen by looking at the colours, and E[Z(S)] = A(S) for a fixed S says nothing about it. The memo's bound e(S) <= 2|S| + 2w limits what a set can cost, not which boards reach it. Random k-SAT is a standing counter-example to "branching rules only move sub-exponential factors" in a setting without geometry. Here the lattice confines information to the frontier, which is a reason to expect the claim to hold, not a proof.
2. Propagation through the unused pool (hidden singles, piece all-different) is not frontier-local by construction. It is weak before the peak because the pool is large, which again is an expectation.
3. Size. At n = 16 one row of seams is worth 2w ln c = 79 nats and the frame 87 nats (balanced count), against a bulk term P/e = 72. At n = 8: 25 and 19 against 13. "Surface" is not small at any size the program can run, so the claim forbids no ratio that has been or can be measured below n = 16.

The memo's quantitative model of propagating solvers failed by 10 to 200, as it says. The measured ratios (D) are flat or falling with n, which is compatible with C1 and does not test it.

C2. Measured (`c_refute.py`, whole trees of the committed instances; "relaxed" = no filling of the next L cells even if a piece may be reused inside the lookahead; pieces already on the board stay excluded):

| instances | region | nodes | no child (L = 1) | relaxed, one row | relaxed, two rows | strict, two rows |
|---|---|---|---|---|---|---|
| 6x6 c=5, 30 | more than one row before the peak | 327,089 | 8.3% | 35% | 37% | 50% |
| 6x6 c=5, 30 | last row before the peak | 5,637,115 | 35% | 63% | 70% | 93% |
| 7x7 c=8, 30 | more than one row before the peak | 974,185 | 23% | 52% | 57% | 77% |
| 7x7 c=8, 30 | last row before the peak | 5,951,755 | 34% | 76% | 83% | 97% |

The memo's reasoning ("relaxed completions number about 4m/c^2 per cell, above 1 before the peak, so such nodes cannot be refuted") confuses a mean above 1 with existence. A branching process with mean b > 1 still dies with probability q(b) > 0, and q tends to 1 at the peak. What survives of C2: a distinctness-free check cannot refute a fraction tending to 1 of the nodes well before the peak, so it cannot change the bulk rate. The prediction written for the census ("no refutation before it") is false and should be replaced by "a share below the strict-lookahead share, rising to most nodes at the peak".

What would settle C1: the DFS / `bb-mrv` ratio at fixed x (not at fixed +2 colours) for n = 6 to 10 with measured DFS, plus the per-depth node profile of `bb-mrv` set against Z_d of DFS on the same instances. C1 predicts ln(ratio) grows no faster than n ln n; a ln(ratio) that grows like n^2 refutes it. Plain DFS now costs seconds per instance up to 9x9 c=11 (see F), so only the `bb-mrv` side is expensive.

### D. Node ratios from the raw rows

`d_ratios.py`, paired by instance, censored runs entered at their cap, pairs censored on both sides dropped, bootstrap 95% on the median paired ratio.

+2 track:

| n, c, x | DFS / `bb-mrv` | DFS / scb | scb / `bb-mrv` |
|---|---|---|---|
| 6, 7, 0.77 | 9.9 [8.4, 12.4] (medians 700 / 57 = 12.3) | 5.2 [4.5, 8.7] | 1.8 [1.7, 3.1] |
| 7, 8, 0.64 | 15.7 [7.5, 20.1] (44,926 / 3,218 = 14.0) | 4.6 [3.1, 9.8] | 2.5 [1.7, 4.4] |
| 8, 10, 0.69 | 5.4 [4.3, 8.0], 50 pairs, 14 are lower bounds (30-seed medians 483,034 / 69,036 = 7.0) | 0.89 [0.71, 1.31] | 6.0 [3.2, 12.0] |
| 9, 11, 0.62 | 2.0 [0.75, 8.4], 4 informative pairs, DFS measured | 0.83 or less (5 upper bounds) | not known |

Primary rungs (x near 0.37): DFS / `bb-mrv` = 68 [55, 119] at 6x6 c=5 and 82 [54, 165] at 7x7 c=6 with measured DFS. The memo's table has "about 140 (model DFS)" and "about 90" for scb at 7x7 c=6; measured, they are 82 and 51.

D1 holds: 12, 14, 7 are the ratios of medians, the paired values are 9.9, 15.7, 5.4, and nothing grows. The n = 9 value is about 2 on thin data.

D2. The trend A22 saw (scb / `bb-mrv` 1.8, 2.5, 6.0) is scb falling to the level of plain DFS at n = 8 and below it at n = 9. That part of the memo's reading is supported. The attribution to border-first order is not: at 8x8 c=10 the committed ablations give `mrv-bf` / `bb-mrv` = 1.83 [1.65, 2.08] and scb / `scb-ad` = 4.4 [3.7, 5.0], so border-first carries the smaller part. The memo's numerical support (scb's 5.5e5 nodes "of the order of F_8 = 4e5") rests on frame counts that are too small by a factor of about 600: generated 8x8 boards have about 2.6e8 valid frames, and scb at 7x7 c=8 uses 7,088 nodes on boards with about 7e5 valid frames. scb does not enumerate frames.

### E. Two-sided search

`e_twosided.py`. Interior-only exact product against the continuum:

| x | P | peak rate (g(x)) | half-depth rate ((ln 2 - 1 - ln x)/2) | gain, nats |
|---|---|---|---|---|
| 1/e | 196 | 0.3704 (0.3679) | 0.3483 (0.3466) | 4.33 |
| 1/e | 262,144 | 0.3679 (0.3679) | 0.3466 (0.3466) | 5,585 |
| 0.30 | 196 | 0.5070 (0.5040) | 0.4503 (0.4486) | 11.1 |
| 0.45 | 196 | 0.2505 (0.2485) | 0.2476 (0.2458) | 0.58 |
| 0.50, 0.55, 0.70 | any | equal | equal | 0 |

The algebra is right: 1/e against (ln 2)/2, 6% lower, e^4.3 at P = 196, no gain from x = 1/2. With the frame included (row-major top half against the row-major peak, memo's profile) the gain is 5.8 nats at 16x16 c=17, 1.3 at 8x8 c=7, 0 at 8x8 c=8 and 6x6 c=5.

Corrections.

- Measured on enumerated 6x6 boards: at c=4, listing all consistent top halves costs 5.1e6 nodes, so two sides cost 1.0e7 against a whole tree of 4.9e7 (4.8 times less), but the first-solution DFS median there is 1.7e5. At c=5 two-sided listing costs 4.5e5 against a whole tree of 5.0e5 and a first-solution median of 4.2e4. Two-sided search must list both halves in full; DFS stops at the first solution. On every board I could enumerate, DFS wins.
- "Only two families" is what the first-moment model can express, not a theorem about methods. The memo's own table gives 0.15 to no-good learning, which the model cannot represent at all.
- Neither literature source discusses two-sided search.

### F. The failed prediction

Pilot rows now on the branch (commit `1b5e26d`, 21:00): `bb-mrv` 2 solved of 6 (9,970,108 and 78,349,212), 4 censored at 1e8, seeds 104 and 105 still without a row; scb 0 of 4. Unchanged from what the memo read.

New: plain row-major DFS on the same eight pilot instances, cap 2e8: 74,537,617; 163,609,759; above cap; 161,184,835; 18,902,593; 83,444,100; above cap; above cap (seeds 101 to 108). Five of eight solved, KM median 1.61e8. On A22's seeds 1..30: 18 of 30, KM median 1.56e8.

Reading.

- The DFS column of the memo at 9x9 (3.2e8) is high by 2.1, the same bias as at 7x7 c=6. Claim A survives at n = 9.
- The prediction failed in the other factor. The memo divided by a gain of 7 to 14. With measured DFS the gain is about 2 (median of 4 informative pairs), and at most 2.1 by medians (1.61e8 against a `bb-mrv` median of at least 7.8e7). Under a common cap of 1e8, DFS solves 3 of 8 and `bb-mrv` 2 of 6.
- So the ratio runs 10, 16, 5, about 2 for n = 6 to 9. D1 is confirmed and sharpened: the gain of this propagating solver over plain row-major DFS is not constant, it is shrinking. That is compatible with C1 (a bounded factor) and against any reading of A22 as a growth-law change.
- B, C2 and E are untouched by the failure. The rule "extrapolate a propagating solver by a constant gain over DFS" should be dropped.
- Wall clock: the DFS runs took 2 to 6 s each (33 to 60 million nodes per second). The pilot's `bb-mrv` run on seed 103 took 6,772 s for 7.8e7 nodes. At 9x9 c=11 plain DFS is the stronger baseline by more than three orders of magnitude in time.

### Literature check

Martinsson, arXiv:1701.04813 (abstract and pages 1 to 4 read). Represented correctly: n x n pieces, q jig types, boundary edges coloured too, no frame palette; at least two non-similar solutions for q <= (2/sqrt e) n, all solutions similar for q >= (2 + eps) n. With side w these are x = 1/e and x = 1, as the memo says. The paper also gives the first-moment count 4^(n^2) (n^2)! / q^(2n(n-1)), conjectures the sharp threshold at (2/sqrt e) n, and notes that k x k sub-solutions unrelated to the planted one stop dominating only at q = 2n. That last remark is the memo's bulge phase, so the paper supports the picture behind A and C. It says nothing on frames (B) or on search (C, E).

Edmonds, Edmonds and Mertz (manuscript, pages 1 to 7 read). Represented correctly in the main: q = sqrt(4n/eps) with n the number of pieces, so eps = 1/x; Theorem 5 (rectangle from a corner, expected width 1 + Theta(eps)); Theorems 2 to 4 (edge-first e^Theta(sqrt(eps) m), block in the middle n^Theta(log n), triangle 2^Theta(eps sqrt n)); Lemma 1 has the "1 + planted partner" terms; section 6 is the same first moment as the memo's A(d). One imprecision: Theorem 6 is the case 1/q' = (2/eps)(1/q), that is x' = eps/4, the many-solutions side. It does not cover the phase 1/e < x < 1 where the rungs sit, and no theorem of the manuscript does. The manuscript supports "order decides between linear and exponential in the side" (C, for static orders). It has no dynamic ordering, no propagation, no two-sided search.

## 3. Errors found

1. `growth-law/model.py`, `critical_c` (lines 344 to 350) and `ln_solutions` (339 to 342), used by `regime.py` and `table_hp.py`; `growth-law.md` line 12 (item 4), lines 186 to 193 (section 5 table and the paragraph after it), line 229. The first moment assumes independent uniform colours. Fix: ring factor prod f_k! / (4(n-1))!, interior factor prod (2 e_k - 1)!! / (2 E_I - 1)!!. Critical counts become 4.65, 5.99, 7.32, 8.63, 9.94, 12.54, 17.68; rung offsets +0.35, +0.01, +0.68 (primary) and 2.0 to 2.7 (+2 track).
2. `growth-law/model.py`, `urn_log` (lines 114 to 148): the "urn" variant pairs ring half-edges freely. Ring seams pair a clockwise side with an anticlockwise side. It returns ln S = -7.2 at 6x6 c=5 where the balanced count gives -2.8. Fix as in 1. The variant is not used for any published number.
3. `growth-law.md` line 77 (and lines 16, 157 to 166, 175, 195): F_n = 5.3, 980, 4.0e5, 3.2e8, 4.3e11. Measured 2.6e3 and 6e5 to 8e5 at n = 6, 7. Balanced: 1.6e3, 4.7e5, 2.6e8, 2.6e11, 4.4e14, and 5.17e37 at n = 16.
4. `growth-law.md` line 61: the gap between 2.0e34 and Owen's 4.05e37 is attributed to planted cycle terms. It is the balanced ring count (5.17e37). Remove the attribution.
5. `growth-law.md` line 199: "ln S = 2.8", "with five clues the real puzzle is again at S of order 1". Fix: 1.15e7 with no clue, 14,702 with the starter, 3.9e-8 with five clues (balanced ensemble, equal to the site's figures).
6. `growth-law.md` line 14 (item 6, second sentence), line 213 (table row, "cannot be refuted, whatever the region size", "no refutation before it"), line 233. Fix: "refutes a share of the nodes before the peak (37% to 57% measured two rows ahead), and cannot change the bulk rate".
7. `growth-law.md` line 14: "exp(O(n))" should be exp(O(n ln n)), consistent with line 10.
8. `growth-law.md` line 160: "about 90" and "about 140" (model DFS at 7x7 c=6). Measured DFS median 40,171,900 gives 44 to 51 and 68 to 82.
9. `growth-law.md` lines 16, 166, 180: "at most about 3 at n = 9 (model DFS)". Measured DFS gives about 2, interval 0.75 to 8.4.
10. `growth-law.md` line 195: scb "pays the valid frames". Not supported: see D2.
11. `growth-law.md` line 244 (recommendation 2): the frame-colour schedule (3 colours at n = 6 to 8) is derived from the wrong frame count. Withdraw it. With 5 frame colours the primary rungs are already within 0.7 colours of critical.
12. `growth-law.md` line 82: Theorem 6 of Edmonds et al. is x' = eps/4, not "the regime x < 1".

## 4. What the program may rely on tomorrow

1. Row-major DFS model (`table_hp.py`): median nodes within a factor 2.1, always on the high side, on every above-critical rung from 6x6 to 9x9, including two rungs tested out of sample. Not below critical.
2. Solution counts of generated boards follow the balanced first moment. Critical colours 4.65, 5.99, 7.32, 8.63, 9.94 for n = 6 to 10; primary rungs at +0.35, +0.01, +0.68; the 16x16 at 1.15e7, 14,702 and 3.9e-8 expected solutions with 0, 1 and 5 clues.
3. Plain row-major DFS at 9x9 c=11: KM median 1.56e8 nodes, 18 of 30 under 2e8, seconds per instance. It is the cheapest reference at that size.
4. DFS / `bb-mrv` in nodes: about 70 to 80 at x near 0.37 (n = 6, 7); 10, 16, 5 on the +2 track at n = 6, 7, 8; about 2 at n = 9. No growth with n.
5. Two-sided search: rate (ln 2 - 1 - ln x)/2 against g(x), gain only for x < 1/2, 4.3 nats at P = 196, x = 1/e, and no advantage over first-solution DFS on any enumerated board.

## 5. What it must not rely on yet

| statement | why not | cheapest measurement |
|---|---|---|
| The memo's critical table, "no rung is at critical", recommendation 2 | Wrong ensemble (section B). | Already settled at n = 6. To confirm at n = 7: whole-tree solution counts on the 30 committed 7x7 c=6 instances (about 3e8 nodes each, 5 minutes with `enum.c`); the balanced count predicts about 0.9 unrelated solutions per board plus local variants. |
| "Propagation and ordering are surface factors" (C1) as a filter on candidate methods | Argument covers static orders only, and the allowance is as large as the bulk at n <= 16. | DFS / `bb-mrv` at fixed x near 0.37 for n = 6 to 9 with measured DFS (DFS side: minutes), and the per-depth node profile of `bb-mrv` against Z_d of DFS on the same 7x7 and 8x8 instances. |
| "No refutation before the peak" as a census prediction (C2) | False (37% to 57%). | Rerun the census split at the peak, weighting each refuted node by the size of the subtree it removes. `c_refute.py` gives the unweighted shares in 4 s. |
| "scb loses because it is border-first" (D2) | Border-first carries 1.8 of 6.0 at n = 8. | The memo's own test (scb and `mrv-bf` against `bb-mrv` at 8x8, c = 12 to 17), plus `scb-ad` on the same rungs. |
| The gain of `bb-mrv` at n = 9 | 4 informative pairs. | Finish seeds 104 and 105 of the pilot; run plain DFS to completion on the 8 pilot seeds (cap 2e9, about a minute each). |
| The DFS model at 10x10 c=12 (5.3e11) | Bias has grown from 1.1 to 2.1 between n = 6 and 9. | 10x10 c=14 (model median 4.2e8, seconds per instance) and c=13 (model 8.3e9, about two minutes per instance) on 30 seeds with `enum.c`, under an hour in total. |
| Where the DFS hardness peak sits for n >= 7 | Known only at 6x6 (c = 4, about 350 solutions per board). | First-solution DFS at 7x7 c=5 and 8x8 c=7 with a cap of 1e10 (minutes per instance). |
| The first-solution heuristic at S of order 1 and any node count for the 16x16 | The memo says so itself; nothing here changes that. | None cheap. |

## Scripts

- `enum.c`: row-major DFS (first solution, whole tree, census with strict and relaxed lookahead, frame count).
- `rvlib.py`: generator copy, naive and balanced first moments, loaders.
- `a_dfs_model.py [run]`: claims A and F. `b_solutions.py`, `b_peak.py`: claim B. `c_refute.py`: claim C2. `d_ratios.py [run]`: claims D and F. `e_twosided.py`: claim E.
