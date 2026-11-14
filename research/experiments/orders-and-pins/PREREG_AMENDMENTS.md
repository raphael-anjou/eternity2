# orders-and-pins: amendments

Each entry: what, why, when. All written after the gate stages (gate6, gt)
ran and before any test-stage run (a-official, a-null, b0 to b4). No
official-set estimate of any order beyond GT's first K levels, and no Part B
number, had been produced.

## A1. s6-beam order file listed only the free cells (2026-11-13, 09:44)

What. `scripts/orders.py`'s beam search returned the 251 free cells without
the five clue cells, so `orders/16/s6-beam.txt` was not a permutation of the
256 cells and every engine unit on it refused to run (gt stage: 11 failed
units, no rows). Fix: the beam order is returned with the clue cells
appended (every consumer skips them). The free-cell sequence of s6-beam is
unchanged; its file and SHA-256 change. Search 7 (annealing from the beam
order) now anneals over the 256-cell list instead of the 251-cell one, so its
random moves differ; its output is whatever the rerun gives (recorded in
`orders/manifest.json` and below). The gt units for s6-beam are rerun.

Why. A code bug found by the gate; the PREREG text (the search and the
selection rule) is unchanged.

Result of the rerun: searches 1 to 6 give the same outputs as before
(manifest identical for them; s6-beam's free-cell sequence unchanged).
Search 7 now returns a new order, `s7-sa-beam`, model 10^41.138 (it was a
duplicate of s6-beam before). By the selection rule (up to five non-duplicate
outputs) it joins the measured set, which becomes 19 orders: the 15 named
and s5-sa (40.896), s7-sa-beam (41.138), s6-beam (41.153), s4-sa (44.508).
`plans/` were regenerated; per-unit seeds after s5-sa in each plan shift by
the inserted units (no test-stage unit had run). Gate rows already written
for s4-sa used the seeds of the earlier gt plan (recorded in each row).

## A2. GT: equality test at fully kept levels (2026-11-13, 09:44)

What. In GT, at shallow levels where the population keeps every child
(width at most N), the SMC estimate is exact and its standard error is 0.
`analyze.py` (copied from Track 10's gate) then demanded equality to a
relative 1e-9, but `tsmc` stores widths as log10 with 6 decimals, a relative
precision of about 2e-6. Every order failed GT for this reason alone (for
rows-b-lr, depths 1 to 6, relative differences 1e-8 to 2e-7; depths 7 to 9,
where resampling acts, are within 1.6 standard errors). The tolerance at
zero standard error is set to a relative 1e-5. The rule of PREREG A3 ("more
than 3 standard errors from exact") is unchanged; G6 is not affected (its
means are stored at 7 significant digits and its fully kept widths are at
most 200, so they are exact).

Why. A precision artefact of the stored format, not an estimator error.

## A3. GT depths listed in PREREG A3 (2026-11-13, 09:44)

What. PREREG A3 states the rule (K = largest depth whose model cumulative
node count is at most 10^9) and then lists K values computed with an earlier
budget of 10^8.5. The plan used the stated rule; the actual K values, from
`plans/gt-depths.json`, are: 9 for the line scans, boustrophedons, spirals
and frame-first (8 for spiral-out), 14 antidiag, 13 bands2, 5 s5-sa, 3
s4-sa, 19 s6-beam, 5 s7-sa-beam s6-beam. The rule governs; the listed numbers were wrong.

## Gate outcomes that the amendments do not touch

G6 fails, by Track 10's rule, for spiral-out (24% of no-clue pairs outside 3
standard errors: the SMC mean falls far below exact at the last free depths
of several instances), antidiag (10.5%), bands2 (3.3%) and snake-cols-l
(2.04%). By PREREG A3 these four orders are **not measured** on the official
set, whatever their GT result. They are still run (the plan is unchanged)
and their numbers are reported as unvalidated, never used in a reading. The
clue5 variant of G6 is weak: with 5 of 36 cells fixed, every level fits in
the population of 200 and the SMC is exact (0 of all pairs outside, error 0).

## A4. The design review's must-change items M1 to M12 (2026-11-13, 10:01)

What. The lead's design review
(`research/program/night/2026-11-13/memos/d3-design-review.md`, lead
worktree) is applied as written, items M1 to M12. Written after the gate
stages gate6 and gt had run and their outcomes were seen, and before any
test stage: no official estimate of any order (beyond GT's first K levels)
and no Part B number existed. **The new Part A status rule (M1) changes a
gate's consequence after its outcome was seen, on a lead ruling, before any
official estimate of the four orders that failed G6 existed.** Also stated,
as the review asks: calibration board 100149 has the official colour counts,
so its numbers (spiral-out 10^50.2, antidiag 10^45.0, spiral-in 10^43.5)
already indicated the Part A outcome before the preregistration; Part A is
confirmatory.

M1, Part A status. G6 is split. G6-total: median over instances of
|log10(mean total / exact)| at most 0.05 and every instance enumerated, in
both variants. G6-profile: the old pair rule; it gates only statements about
widths at given depths, which are not reported for an order that fails it.
An order is **measured** iff (named orders) G6-total passes, GT passes, the
sd of the five N = 1e6 repetitions' log10 totals is at most 0.10, and the
N = 1e5 ladder mean (M2) is within 0.10 dex of the N = 1e6 mean. Otherwise
it is **bounded below**: log10 T >= log10(mean of the N = 1e6 totals) - 1.30
(Markov, 95%, valid for any unbiased non-negative estimator), with the point
estimate shown and marked unvalidated. This replaces "an order whose gate
fails is reported as not measured" in PREREG A3 and the last paragraph of
A1 to A3 here. GT is kept and labelled an implementation check (over its
first 3 to 19 levels the population holds nearly every node).

M2. a-official adds, for every order, N = 1e5, 5 repetitions, seeds 705001
onward (units `a:<order>:np1e5:r<i>`). Used only by M1 (and by M11).

M3, reading. An order **beats row scan** only if measured and the PREREG A4
rule holds. It **does not beat row scan** iff it is measured and the rule
fails, or its Markov lower bound is above rows-b-lr's 95% upper limit minus
1.0. Otherwise it is **undecided** and named. The kill line is reported as
reached only if no order beats and none is undecided; with undecided orders,
as "reached for n of 18 compared orders, undecided for the rest" (the review
wrote "n of 19"; the 19th order is the baseline itself). The model
comparison (Spearman and its two conditions) uses measured orders; the
Spearman over all 19 point estimates is reported beside it, labelled.

M4, Part B value. For every configuration with both estimators: the SMC
mean, the PPD mean, and the pooled value = arithmetic mean of the two means
(on totals). The pooled value is the reported figure; its 95% lower bound is
log10(pooled) - 1.30. The disagreement in dex is reported. This replaces
"the larger of the two log10 means" and "lower bound only" of PREREG B1.

M5, budgets. A configuration is **shown under** a budget only if validated:
no SMC repetition extinct early (below), sd of log10 totals at most 0.30 for
each estimator, the two means within 0.50 dex, its policy passed gate GB for
at least one estimator, and pooled + 0.30 dex under the budget. **Shown
over** iff its Markov lower bound is above the budget. Pins to a budget: a
bracket per policy (largest k shown over, smallest k shown under, or "not
shown within the grid"), per board for greedy and spread; for random, the
largest k at which at least half of the (board, set) pairs are shown over
and the smallest at which at least half are shown under. This replaces the
budget rule of PREREG B1 and B3.
Deviations, to keep the quantity defined: (a) "no repetition goes extinct
before the last free depth" is unattainable as written: every SMC run on an
unpinned 16x16 tree dies near depth 190 while the tree is alive (Track 10),
and PPD's planted path always reaches the end. The criterion used: an SMC
repetition is extinct early iff its estimated width at its last nonzero
depth exceeds 1e-3 of its own estimated total (the lost tail could then
matter); not applied to PPD, whose diagnostics are its sd, the agreement and
GB. (b) GB covers greedy (sets k = 12 and 16) and spread (k = 16); the greedy
policy passes GB iff both greedy sets pass for the same estimator. Random
sets have no GB set, so no random configuration can be shown under a budget;
they can be shown over.

M6, gate GB (stage gb). Calibration boards 100149 and 100244, rows-b-lr,
five clues, sets greedy k = 12, greedy k = 16, spread k = 16. Exact widths
by `tenumd` at maxd 88, 72 and 56 (cap 3e9). Deviation: all three depths are
enumerated on both boards in one stage (no conditional plan); D = the
deepest that completes on board 100149, and board 100244 is read at the same
D (fails if incomplete there). Against them: SMC N = 1e5 and PPD N = 1e4, 10
repetitions each, per-depth means. Pass per (set, estimator) on both boards:
|log10(mean W_D / exact W_D)| <= 0.10 and truncated sums within 0.10 dex. An
estimator that fails GB for a policy is reported for that policy as a lower
bound only.

M7, gate g8 (stage g8). The 30 instances of
`../hard-ladder/instances/8-c10-test.jsonl` (board files written to
`boards/b8/` from the `planted` field), pin sets none, clustered4,
clustered8, spread4, spread8 as in the review's `pins8_check.py`, rows-b-lr
on 8x8, exact by `tenum` (cap 4e8, instances past the cap skipped and
counted), SMC N = 100 and PPD N = 10, 200 repetitions each. Pass per
estimator: mean within 3 standard errors or 0.02 dex of exact on at least
95% of enumerated (instance, set) pairs, and median |log10 error| at most
0.02. It replaces the two 6x6 instances as PPD's gate. It tests the code and
unbiasedness, not the bottleneck regime.

M8, end-to-end. b4 passes at a k iff it passes on at least 4 of 5 boards
(per-board rule of PREREG B3) and no estimator's mean is more than 0.30 dex
from the exact total on any board; the sign of every miss is reported, and
per board the nodes at the first complete board over the exact total.

M9, the quantity. Every figure is the full tree: every valid partial board,
the cost of exhausting, for row scan from the bottom with pins used only
when the scan reaches them. The expected cost to the first solution with one
solution and a random value order is half of it (one bit less), about
T / (S + 1) with S solutions; a solver free to choose its order after seeing
the pins needs no more.

M10, policies. b2 is "greedy in the unplanted model: a demonstration that
the model's placement fails", never a best case, cut to k = 4 and k = 12
(additivity for greedy is then read at k = 4 only). The best case reported
is "the best policy tried": the validated policy with the smallest k under a
budget among those defined without looking at the test boards (here spread
only). Random sets are "typical". No figure is called optimal or
well-placed without naming the policy. Caveat for the report: the spread
depths 16 to 160 were chosen after exploratory runs on calibration board
100149 (not a test board).

M11, transfer. "Real pieces (argued)" in PREREG B3 is struck: no model-only
figure for the real pieces appears in a reading; the model stays as a
comparison column. Stage bl: the official set and the five Part B boards,
each of the five clues removed in turn, rows-b-lr, SMC N = 1e5, 5
repetitions (seeds 950001 onward). A clue's worth = log10 T(four clues) -
log10 T(five clues); on the boards T(five) is b0 (N = 1e6), on the official
set the M2 ladder value of rows-b-lr (N = 1e5, same N as bl). The README may
give painted-board figures as the estimate for the real pieces iff (i) the
official five-clue tree (a-official, N = 1e6) lies within [min - 0.2,
max + 0.2] of the five b0 values and (ii) each official clue's worth lies
within [min - 0.3, max + 0.3] dex of the same clue's worth on the five
boards. Otherwise the figures are painted-board figures only and the failing
clue is named.

M12, order and bookkeeping. Stages run in the order gb, g8, b0, bl, b1, bs,
b4, b3, b2, a-official (with M2), a-null (`run.sh all`). Cuts if the cores
are taken back: a-null first, then b3's second set, then b1's PPD subset.
The K list of A3, corrected: 9 for the line scans, boustrophedons,
spiral-in and frame-then-rows; 8 spiral-out; 14 antidiag; 13 bands2; 5
s5-sa; 3 s4-sa; 19 s6-beam; 5 s7-sa-beam.
Code changes for A4: `scripts/plan.py` (A_LADDER block in a-official;
GREEDY_K = [4, 12]; new plans gb, g8, bl; 8x8 board files and order file);
`scripts/analyze.py` (G6 split; `part_a` rewritten for M1 to M3;
`gate_gb`, `gate_g8`; `b_configs` with the pooled value and the Markov bound;
`validated`/`status` for M5; brackets; M8 end-to-end; M11 transfer checks);
`run.sh` (stages gb, g8, bl, all).
The optional items S1 to S3 are run only if time remains after everything
else; if run, they are added by a later amendment before they run.

## A5. Gate GB cannot test the spread policy at full size; a secondary label (2026-11-13, 10:20)

What. Written after the gb and g8 stages ran and while b0 (the first test
stage) was running; before bs, b3 and b2 had produced any row, and before
any b0 row was read.

GB outcome (calibration boards only, `results/gate/gb.jsonl`): greedy k = 12
at D = 72 fails for SMC (+0.68 dex on board 100149, every repetition extinct
before D on board 100244) and for PPD (-0.197 dex on board 100149, -0.005 on
100244); greedy k = 16 at D = 56 passes for both on both boards. So the
greedy policy fails GB (M5 needs both greedy sets to pass for one
estimator). Spread k = 16 could not be tested: no truncation depth (88, 72,
56) completed under 3e9 nodes on either board. This is structural: the
first spread pin sits at free depth about 18, and the unpinned row scan
already has about 10^14 partial boards at depth 16 (model profile), so no
spread tree can be enumerated past the first few levels (GT used depth 9 for
10^9 nodes). Under M5 as written, no spread configuration can be shown under
a budget. M5 is applied as written; nothing is shown "under".

Added, as a secondary label, never reported as "shown under": a spread
configuration is **under, checked end-to-end only** if it meets every M5
condition except GB and the end-to-end check (M8) passes at k = 40 or k = 48.
The README reports it in a separate column, says that it rests on the
easy-regime whole-tree check and on agreement between the two estimators,
not on GB, and that this label was defined after the GB outcome was seen.
Code: `scripts/analyze.py` (`status` gains the secondary label).

## A6. Two more reporting thresholds on the owner's request (2026-11-13, 10:31)

What. Every "pins to a budget" reading in Part B gets two more budget lines
beside 2^78 and 2^61.5: 10^15 nodes (about one month on 17 to 30 cores at
1.6e7 nodes per second, the rate the previous night's engine measured on the
lab machine) and 10^16 nodes (about one year). Same rules (A4 M5 and A5):
shown under only if validated, shown over by the Markov lower bound,
otherwise a bracket, per policy. Added on the owner's request, relayed by
the lead, after the test stages had started (b0 and bl had run or were
running). It changes no run and no existing reading. When writing it I had
not looked at any Part B test output (no b0, bl or later row had been read
or summarised). As for the other budgets, the figure is the full tree (the
cost of exhausting); the first solution costs about half.
Code: `scripts/analyze.py` (`BUDGETS`).

## A7. Stages a-official, a-null and b2 run on mac-m1 (2026-11-13, 11:57)

What. On the lead's instruction, the three stages last in the A4 order and
independent of the local ones run on the lab machine mac-m1 instead of this
Mac: a-official (with the stability ladder), then a-null, then b2. The tree
was shipped with `git archive` of commit `ba0b77b1` (paths `research` and
`engine`, every `results/` excluded) to `~/e2-night/d3`, built there with
`nix shell nixpkgs#cargo nixpkgs#rustc -c cargo build --release -j 4`. The
shipped PREREG.md has the same SHA-256 as here (48dc0dbd...); the guard there
runs in copied-tree mode and logs that hash. Check before use: three
identical units (tsmc on the official set, order s4-sa, N = 2000, seeds 4242
and 4243; tppd on board 100149 with two pins, N = 500, seed 7) give
identical totals on both machines (4.320931e+39, 2.209958e+40,
5.213149e+34). Plans and seeds are the committed ones, unchanged, so the rows
are the ones the PREREG names. Driver `remote_driver.sh` (detached, nohup;
PROGRESS.txt with step, start, end, exit code; a failed step does not stop
later ones; 4 processes before 12:50, 7 after). Locally,
`scripts/drive.py` skips any stage listed in `REMOTE_STAGES` (a-official,
a-null, b2), so nothing runs twice; the local guard still logs a start and an
end line for each skipped stage. The remote rows are collected after 22:00
(README, TODO). This changes no reading and no seed. Part B stages b1, bs,
b4, b3 stay on this Mac. No optional addition (S1 to S3) was started.

## A8. Status rule: a walled order is not "measured" (2026-11-13)

What. In `scripts/analyze.py` `part_a`, an order whose SMC repetitions go
extinct before free depth 150 of 251 (minimum over the five N = 1e6
repetitions) no longer has status "measured"; it takes "bounded below" and
the Markov reading of A4 (M3). Effect: s7-sa-beam changes from "measured,
beats" to "bounded below (LB 37.04), undecided"; the kill line reads reached
for 15 of 18 orders, undecided for s4-sa, s5-sa, s7-sa-beam; no order beats
row scan. The threshold, 150, is the judge's. It also demotes spiral-out (minimum
extinction depth 140): its status becomes "bounded below" (LB 49.00), and
its reading stays "does not beat" because that bound is above row scan. The
kill-line count is unchanged (15 of 18). The model-versus-measurement
Spearman is recomputed over the 12 remaining measured orders (0.934); the
judge's 0.949 (13 orders, spiral-out kept) is the judge's reanalysis.

When. Written AFTER the outcome was seen (the committed reading was
"s7-sa-beam beats row scan, kill line not reached"), on the judge's ruling
in `JUDGE.md` (F1). It is a correction of a status rule, not a preregistered
reading, and no row, seed or run changed.
