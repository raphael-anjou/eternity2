# Review: learning against propagation on toy boards

2026-11-14. Independent re-check. Generated toys only; not Eternity II progress. Script: `scripts/recheck/recompute.py` (own parser, stdlib).

## Ruling

**Kill line: undecidable on this data.** It joins two clauses with "and". The depth clause (104 to 88, real pieces) has no data. The size clause leans fired but is not established: for CaDiCaL over row scan with arc consistency the rise of the median lead from 6x6 to 7x7 is at most +2.09 bits on the committed counts, with no lower bound, against a threshold of 2.

## (a) Soundness

Sound as far as it goes, with one handling fault.

Checked and found correct:

- All 30 boards regenerate byte for byte from `scripts/mkboards.py` in all three formats; the clasp arms on both machines report the same variable count per board.
- Solution counts of every finished run equal the tree's; no learning-off run has a lemma.
- Reruns here: tree, arc consistency and weighted degree on all 5x5 and 6x6 boards, tree on 7x7 board 7, CaDiCaL on ten 5x5 and three 6x6 boards. 74 of 74 identical in every count. No clasp run was repeated (not installed here).
- My parser reproduces the finished and cut table and the paired medians: U1 minus U0 +1.31, +2.03; K minus AC +2.32, +2.19, +3.89 (3 pairs); C minus AC +3.32, +3.20; U0 minus AC +1.68, +1.28, +1.22; seconds K against AC -3.15, -2.98, U1 against U0 -3.81.

Units. The arms count different things: nodes, branches tried, conflicts. Arc consistency's branches tried exceed its failed branches by 1.13, 0.94 and 1.10 bits at the three sizes. Failure against failure, CaDiCaL's lead is +0.93, +1.27 and (3 boards) +2.77: one bit lower in level, the rise unaffected.

The fault: 7x7 board 5 is dropped from every pair against arc consistency because the arc consistency run was killed with no output. CaDiCaL finished that board in 430 s with 1.97 million conflicts, clasp default in 435 s. Arc consistency tries about 215,000 branches a second at 7x7, so it had tried at least about 640 million when killed: a lead of at least about 8 bits (inferred from the rate, not counted). The dropped board is the one where learning wins most, in counts and in seconds.

Cause of the two lost runs: `dyn.c` tests its limit on processor time; `cdcl_pysat.py` tests it only every 200,000 conflicts (about 77 s).

## (b) The kill line

Pair taken: CaDiCaL (K) over row scan with arc consistency (AC), because the pre-stated test names exactly that learner and that control. The cleaner measure of learning alone, U1 over U0, was not named and has no finished 7x7 run.

K over AC, median over boards:

- 6x6: +2.19, 10 boards, exact.
- 7x7: 3 exact (+3.64, +3.89, +5.33); 5 upper bounds (+3.14, +3.70, +3.96, +4.61, +4.88; K was cut); board 4 unknown (K killed, no counts); board 5 at least about +8.
- Median over all 10 boards, taking only committed counts: at most +4.28. Rise at most +2.09. No lower bound exists: a cut learner may need any number of further conflicts.
- With board 4 at the bound its neighbours suggest (about +3.2, inferred): median at most +3.93, rise at most +1.74.
- The measurement note's +1.73 (8 boards) and +1.70 (3 boards) are reproduced but do not bound the 10 board figure.

So the median rise is probably under 2 bits (my odds 80%), by under half a bit, with a bootstrap interval (+0.35 to +2.73, computed on upper bounds) that contains 2. On the mean it is open: board 5 is bounded only from below.

Other pairs. Clasp default over AC: rise at most +0.61. U1 over U0: at most +4.12, no information. L1 over L0: both arms cut on all 10 boards; the measurement note's +3.15 is a ratio of two stopping points.

Unstated confound: inner colours go 4, 5, 7 with size and median solution counts 60, 296, 36. At 6x6 the largest lead (+7.66) is on the board with most solutions (1728); the 7x7 outlier has 1536. "6x6 to 7x7" changes tightness as well as size.

## (c) Worth for 16x16

In counts: a lead of 2 to 4 bits over arc consistency, of which 1.2 to 1.7 bits is U0 over AC (encoding and units, no learning). Learning proper is +1.3 and +2.0 bits at 5x5 and 6x6.

In seconds: a loss of about 3 bits at 6x6 and on the 3 finished 7x7 pairs. U1's conflict rate falls from 1,454 to 256 a second between 6x6 and 7x7, 2.5 bits in one step, more than the largest count rise the data allow for the named pair.

My figures: expected gain in time at 16x16 from clause learning as tested, 0 to 2 bits. Odds of 10 bits in time: 2%. Odds that the count lead keeps rising with size: 30%, on weak evidence. Against 56 to 73 bits, a footnote. Grade: measured for toy counts, argued for 16x16.

Missing parts:

- Real-piece prefixes, depths 112 to 88: yes, about 10 core-hours. It is the only part on the real pieces, its subtrees (2^13 to 2^25 nodes) are of 5x5 to 6x6 size where every arm finishes in about a minute, and the kill line cannot be ruled without it. Arms: tree, AC, K, clasp default, U0; seconds beside counts; wall-clock limits; counts written on every exit.
- 7x7 with a longer limit: no. About 30 core-hours for a number near the threshold. Only if the real-piece part shows a rise.
- 8x8: no. Hundreds of core-hours for one more point on a confounded axis.

## (d) Statements stronger than the data

1. "+3.92 using 8 boards of which 5 cut (an upper bound in the sense above)". It bounds those 8 boards only. With board 5 restored the bound is +4.28 and the rise bound +2.09, which no longer sits clear of 2.
2. "Runs killed with no counts ... are dropped", stated as neutral. Board 5 is the strongest case for learning in the whole run.
3. "each of the 4 finished 7x7 learning runs whose board AC also finished took longer than AC". True, and conditioned to exclude the one 7x7 board where learning won in seconds, by at least 3 bits.
4. "In seconds every learning arm was slower than its control at 6x6". True of medians; single boards reach +2.12 and +3.48 for learning.
5. The main table prints +11.50 (U1), +10.53 (L1), +7.46 (L0) as medians. They are bounds from runs stopped early, as are the bootstrap intervals on cut data.
6. "U0 and U1 differ only by learning". Also by backjumping, restarts and lemma deletion.
7. "identical counts on 12 test runs" across clasp versions: no raw file supports it. Reported, not measured.
8. U0 seconds at 7x7 come from another machine than AC's.

## Unplanned observations

- The tail of the lead sits on the boards with most solutions. The 16x16 subtrees have about none, so the low end is the relevant one.
- The cost per conflict grows faster than the conflict count falls.
