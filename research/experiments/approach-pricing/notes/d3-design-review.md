# D3 orders-and-pins: design review before the test run

> **Status (2026-11-13):** this file is itself a design review (before the test run), not a memo; no second analysis report on it.

2026-11-13. Reviewed: `PREREG.md`, `PREREG_AMENDMENTS.md` (A1 to A3),
`scripts/plan.py`, `scripts/analyze.py` (Part B), `engine/src/oap.rs`,
`engine/src/tree.rs`, `results/gate/gate6.jsonl`, `results/calib`,
`results/run-log.jsonl`, on branch `night/2026-11-13-d3-orders-and-pins`
(head 84b4295d). Nothing was edited there. No test stage was run.

What I ran (one core, `nice -n 15`, the experiment's built binary):

- A recomputation of the 6x6 gate from `results/gate/gate6.jsonl`, split by
  quantity (inline, table in ruling 1).
- `d3-design-review/pins8_check.py`: exact enumeration against plain
  SMC and the decomposition on five 8x8 hard-ladder CALIBRATION instances
  with clustered and spread pins. This is 8x8, not the 5x5 or 6x6 the plan
  allowed; the enumerations took under a second in total.

Not verified: anything at 16x16 beyond what the study's files state. The
proposed gate GB (M5) is untested; its expected outcome is my reasoning.

## Overall view

The design has one error of principle, in both parts: it treats an unbiased
estimator that fails low as if it were symmetric. Unbiased and non-negative
gives a one-sided guarantee for free (Markov: the truth is above estimate / 20
with 95% confidence, whatever the variance). So "this tree is at least X" needs
no gate, and "this tree is under X" needs validation in the same regime. Part A
only needs lower bounds for the gate-failed orders, so "not measured" throws
away a sound reading. Part B's budget statements are upper claims, the unsafe
direction, and "the larger of two estimators that both fail low" does not
protect them.

## Changes that MUST be made before the test run

Each is written as an amendment the study can apply (A4 onward), all dated
after the gate stages and after this review, before any test stage.

**M1. Part A status rule (replaces "G6 fail = not measured").** G6 is split in
two: G6-total (median over instances of |log10(mean total / exact)| at most
0.05, every instance enumerated) and G6-profile (the share of (instance,
depth) pairs outside 3 standard errors at most 2%). A4 reads totals only, so
G6-total gates the total; G6-profile gates only statements about widths at
given depths (peak depth, profile), which are not reported for an order that
fails it. Each order then gets one status from its own official runs:
*measured* iff G6-total and GT pass, the sd of the five repetitions' log10
totals at N = 1e6 is at most 0.10, and the N = 1e5 mean (M2) is within 0.10
dex of the N = 1e6 mean; otherwise *bounded below*, reported as log10 T >=
log10(mean of the totals) - 1.30 (Markov, 95%), with the point estimate shown
and marked unvalidated. State in the amendment that this rule was written
after the G6 outcomes were seen and before any official estimate of these
orders existed.

**M2. Add a stability ladder to a-official.** Every order, official set, five
clues, N = 1e5, 5 repetitions, new seed block (705001 onward). About 6 core
minutes. Used only by M1.

**M3. Kill-line reading under M1.** An order *beats row scan* only with status
measured and the PREREG A4 rule. An order *does not beat row scan* iff it is
measured and fails the A4 rule, or its Markov lower bound (M1) is above
`rows-b-lr`'s 95% upper limit minus 1.0. Anything else is *undecided* and is
named. The kill line "no order beats row scan" is reported as reached only if
no order beats and none is undecided; with undecided orders it is reported as
"reached for n of 19, undecided for the rest".

**M4. Part B value: strike "the larger of the two".** For every configuration
with both estimators report three numbers: the SMC mean, the PPD mean, and
the pooled value = the arithmetic mean of the two means (on totals, not on
logs). The pooled value is the reported figure: it is unbiased, non-negative,
and never more than 0.30 dex under the larger one. Its 95% lower bound is
log10(pooled) - 1.30 (Markov). The disagreement in dex stays reported.

**M5. Part B budget rule: strike "both 95% upper bounds under it".** A
configuration is *shown under a budget* only if it is *validated*: no
repetition of either estimator goes extinct before the last free depth, the
sd of log10 totals is at most 0.30 for each estimator, the two means differ by
at most 0.50 dex, its pin policy passed gate GB (M6) for at least one
estimator, and the pooled value plus 0.30 dex is under the budget. A
configuration is *shown over a budget* iff its Markov lower bound is above
it. "Pins to a budget" is reported per policy as a bracket: the largest k
shown over, and the smallest k shown under; if no k is shown under, "not
shown within the grid".

**M6. Add gate GB: exact truncated trees under pins at full size.** On the
two calibration boards (100149, 100244), rows-b-lr, five clues: greedy k = 12
and k = 16, spread k = 16. Exact widths by `tenumd` (it accepts `--board` and
`--fixed`) at `--maxd` 88, then 72, then 56, keeping the first that completes
under `--cap 3e9` (search the depth on board 100149, reuse it on 100244).
Against them: SMC N = 1e5, 10 repetitions, and PPD N = 1e4, 10 repetitions,
per-depth means (both commands already print log10 widths). Pass per (policy,
estimator), on both boards: |log10(mean W_D / exact W_D)| at most 0.10 at the
truncation depth D, and the truncated sums within 0.10 dex. An estimator that
fails GB for a policy is reported for that policy as a lower bound only. This
is the clustered case of the end-to-end check; full enumeration of clustered
sets is not feasible (past 1e10 at k = 12 and 16).

**M7. Add an 8x8 implementation gate for both estimators with pins.** The 30
instances of `../hard-ladder/instances/8-c10-test.jsonl`, pin sets clustered4,
clustered8, spread4, spread8 and none as in `pins8_check.py` (it writes the
board files from the instances' `planted` field), exact by `tenum`, SMC N =
100 and PPD N = 10, 200 repetitions each. Pass per estimator: the mean total
within 3 standard errors or 0.02 dex of exact on at least 95% of (instance,
set) pairs, and median |log10 error| at most 0.02. It replaces the two 6x6
instances as PPD's gate. State that it tests the code and unbiasedness, not
the bottleneck regime (ruling 3).

**M8. End-to-end (b4) pass rule.** Passes at a k iff it passes on at least 4
of 5 boards and no estimator's mean is more than 0.30 dex from the exact total
on any board; the sign of every miss is reported. The report also gives, per
enumerated board, nodes at the first complete board divided by the exact
total.

**M9. State the quantity.** PREREG, README and every budget line say: the
figure is the full tree (every valid partial board, the cost of exhausting),
for row scan from the bottom with the pins used only when the scan reaches
them; the expected cost to the first solution with one solution and a random
value order is half of it (one bit less), about T / (S + 1) with S solutions;
a solver free to choose its order after seeing the pins needs no more.

**M10. Relabel the policies.** b2 is "greedy in the unplanted model: a
demonstration that the model's placement fails", never "best case", and is
cut to k = 4 and k = 12 on the five boards. The best case reported is "the
best policy tried": the validated policy with the smallest k under the budget
among spread and any policy added under S1 or S2 below, each defined without
looking at the test boards. Random sets are "typical". No figure is called
optimal or well-placed without naming the policy.

**M11. Transfer to the real pieces: strike "Real pieces (argued)" in B3.** No
model-only figure for the real pieces appears in a reading; the model stays
as a comparison column (measured minus model). Add stage bl (leave one clue
out): official set and the five Part B boards, each of the five clues removed
in turn, rows-b-lr, SMC N = 1e5, 5 repetitions (about 10 core minutes). The
README may give the painted-board figures as the estimate for the real pieces
iff (i) the official five-clue tree lies within [min - 0.2, max + 0.2] of the
five boards' b0 values and (ii) each official clue's measured worth lies
within [min - 0.3, max + 0.3] dex of the same clue's worth on the five
boards. If either fails, the figures are reported as painted-board figures
only and the failing clue is named.

**M12. Order of stages and bookkeeping.** New order: GB, 8x8 gate, b0, bl, b1,
bs, b4, b3, b2, a-official (with M2), a-null. Cuts if the cores are taken
back: a-null first, then b3's second set, then b1's PPD subset. Fix the
garbled K list at the end of A3 ("5 s7-sa-beam s6-beam"). Every change of
`analyze.py` and `plan.py` that M1 to M11 require is listed in the amendment;
all amendments are committed before any test stage and the guard's log must
show it.

Recommended, not required (cut first):

- **S1.** Planted first-moment model: W_(d+1) = g_d W_d + 1, with g_d the
  current model's width ratio at step d. It cannot fall below the planted
  path and regrows after a squeeze, which is exactly what the current model
  misses. Check it against the calibration board (greedy k = 12 and 16 must
  come out above 1e10) before use; if it does, add "bg: greedy in the planted
  model", k = 2, 4, 8, 12, 16, as a best-case candidate. Labelled as a model
  revised after calibration.
- **S2.** Measured greedy for k = 2 and 3 with a selection and evaluation
  split: choose each next cell from S1 by SMC on boards 1 and 2 (N = 1e5, 2
  repetitions), report the chosen set's value on boards 3 to 5 only. About 25
  core minutes per step.
- **S3.** Spread k = 32 exact (about 4e10 nodes, 30 to 40 core minutes) on
  one board, cap 8e10: the largest tree on which the estimators can be
  checked whole.

## Rulings

### 1. Part A, the failed gates: CHANGE (M1 to M3)

The estimator is unbiased for every width and for the total. `tree.rs::smc`
keeps a uniform subset without replacement of all children and multiplies by
e / m; by induction est_k times the population mean of any function is
unbiased for that function's sum over the level, extinct runs counting as
zero. So the gate does not detect bias. It detects skew: when a few nodes of
a level carry most of the descendants (a level that falls and rises again, or
a late step that few earlier fills can satisfy, such as the frame closing
last in the outward spiral), most repetitions lose them and return too
little, rare ones return far too much, and the mean of 200 repetitions sits
below exact with a standard error that is itself too small. The gate file
shows this directly:

| order (no clue) | pairs outside 3 se | of which low | outside, in the levels holding 99% of nodes | max abs log10 error of the total |
|---|---|---|---|---|
| spiral-out | 24.44% | 264 of 264 | 0 of 457 | 0.012 |
| antidiag | 10.46% | 112 of 113 | 4 of 483 | 0.037 |
| bands2 | 3.33% | 36 of 36 | 0 of 570 | 0.015 |
| snake-cols-l | 2.04% | 22 of 22 | 0 of 573 | 0.012 |
| rows-b-lr | 0.93% | 8 of 10 | 2 of 573 | 0.023 |

The failures are in tail levels that carry under 1% of the nodes, they are
almost all low, and the totals are right to 0.04 dex for all 15 orders. The
passing orders show the same one-sidedness (rows-b-rl: 15 of 15 low). The
pair criterion tests a quantity A4 never reads.

"Not measured" for exactly these four would make the kill line vacuous for
the orders least like row scan, and it is not needed. To say an order does
not beat row scan, a lower bound on its tree is enough, and an unbiased
non-negative estimate gives one with no gate. On the calibration board the
outward spiral is at 10^50.2 and the anti-diagonal order at 10^45.0; the
Markov bounds 10^48.9 and 10^43.7 are far above 10^39.5. The two near orders
(bands2, snake-cols-l) need the point estimate, which is what the stability
criterion in M1 decides.

G6 is also not sufficient. spiral-in passes it and is the noisiest order at
full size (sd of log10 0.50 at N = 1e5, 0.32 at 1e6, and 2.6 dex under the
model). M1's full-size criterion catches that; G6 cannot.

Alternatives considered: more particles (does not change the direction of the
failure; the ladder in M2 is the cheap diagnostic); planted-path (no planted
solution on the real set); the model with an error bound (the model is 2.6
dex off on spiral-in on the calibration board, so no bound worth stating
exists). The lower bound is the cheapest sound option.

The truncated-tree gate GT: keep, as an implementation check. It caught the
s6-beam file and it exercises the full-size child function, including steps
with no known neighbour. Over the first 3 to 19 levels the population holds
nearly every node, so it says nothing about the estimator where it samples.
Label it so in the report.

### 2. Part A, the reading: KEEP the 19 orders, run Part A last (M12)

The premise is off: the PREREG budgets 1.25 core-hours for Part A (a-official
1.0, a-null 0.25), not 3; by the calibration timings it is nearer 1.5. The
wrong-side cost among the eight line scans is 0.13 to 0.16 dex in the model,
which needs N = 1e6. Dropping the five far orders to N = 1e5 would save about
18 minutes and cost an amendment; not worth it. What should change is the
order: the outcome of Part A is close to known (the calibration board already
shows 45 and 50 for the far orders, and the prior for the kill line failing
was 3%), so it goes last and a-null is the first cut.

### 3. Part B, the decomposition estimator: KEEP the estimator, CHANGE what is reported and how it is gated (M4 to M7, M9)

Read `oap.rs::ppd` and `smc_from`. It is unbiased, for every width and for the
total. The partition is exact (every node is on the planted path or leaves it
first at one step j); each departure term is an independent unbiased SMC; the
truncation of an oversized initial set keeps the full count as the starting
weight; the planted child is unique because the painter rejects duplicate and
symmetric pieces; the per-departure seeds are distinct across j and
repetitions. It is not a bound in either direction. In practice it fails low
in the same way as plain SMC, for prefixes other than the planted one that
survive a squeeze, so it removes one failure (losing the planted path) and
not the other.

Same tree: yes. `Tree::new` takes the pieces, the order and the fixed cells
only; the planted solution is used only to choose where the sum is split. The
priced tree is the one the enumerator walks.

Which quantity: the PREREG prices the full tree (all valid partial boards,
the sum of the widths), the cost of exhausting. It never says so against the
first-solution cost; M9 fixes that. On the 8x8 check, over 20 pinned trees
with one solution, nodes at the first solution over the total averaged 0.49
(range 0.01 to 0.88) in the enumerator's fixed value order.

"The larger of the two" is not an estimator: its mean is above the truth when
both work and it inherits both failures when they fail. The equal-weight mean
(M4) keeps unbiasedness and the lower-bound guarantee and gives up at most
0.30 dex. I do not recommend the decomposition as primary: on unpinned 8x8
trees PPD at a tenth of the population had 1.6 to 3.8 times the standard error of
SMC and a median 0.55 to 0.70 of exact (SMC 0.92 to 1.00), and in calibration
PPD at k = 0 has sd 0.198 against 0.033. It is better only under a squeeze.

The gate: two 6x6 instances is not enough. The 8x8 check (5 instances, 5 pin
sets, 200 repetitions) found both estimators within 3 standard errors of
exact on all 25 (largest departures: SMC 1.117 with se 0.042, PPD 0.870 with
se 0.066, both on unpinned trees). But 8x8 with 8 pins is a tree of a few
hundred to a few thousand nodes: both estimators are nearly exact there, and
the squeeze-then-regrow regime does not occur. So 8x8 with pins is an
implementation gate (M7), and the regime gate has to be at full size on
truncated trees (M6). I expect plain SMC to fail GB for greedy k = 12 (all
repetitions were extinct by depth 53 while the exact tree is alive at depth
56 to 64); whether PPD passes is the open question GB answers.

### 4. Part B, pin policies: CHANGE the labels, KEEP spread, greedy cut (M10, S1, S2)

Greedy in the unplanted model is not a best case; it is the model's own
failure mode chosen 16 times. Spread is a fair, fixed, solution-blind policy,
and it is a candidate best case, not the best case: nothing shows it is near
optimal. A reader will accept "well-placed" only as "placed by a named rule
fixed before the boards were measured, and the best such rule we tried"; the
true optimum then needs no more pins than reported, which is the direction a
budget statement wants. Measured greedy is affordable for k = 2 and 3 only
with a selection and evaluation split (S2), otherwise the maximum over 55
noisy cells is read on the boards that chose it. The cheaper improvement is
S1: the planted recursion makes the model's greedy stop clustering by itself.
One caveat for the README: the spread depths (16 to 160) were chosen after
exploratory runs on the calibration board, which is not a test board.

### 5. What transfers to the real puzzle: the working inclination, with two measured conditions (M11)

Agreed: the painted-board measurements are the estimate for the real pieces,
and model-only figures are dropped from the readings. The argument is
stronger than "ordinary among painted boards": the real puzzle is itself a
planted instance with these colour counts, so planted census boards are its
ensemble. But the independently assessed ordinariness result covers the row-scan tree with
five clues, not pin values. Two cheap measurements extend it: the official
five-clue tree against the five b0 values, and the worth of each official
clue (leave one out) against the same clue on the five boards. These are the
only pin values that can ever be measured on the real pieces. Without them
the README says "on boards painted with the official colour counts" and
nothing about the real set.

### 6. Generator switch: KEEP

The census painter is the one Track 10's independently assessed baseline used for its
painted boards, so it is the required choice, not a deviation; the framed
generator would break the comparison (colour counts vary). The 6x6 and 8x8
gate instances come from the framed generator, which is fine for a gate. No
effect on the comparison with 10^40.457, which is on the official set.

### 7. End-to-end check: not sufficient, CHANGE (M6, M8, S3)

Spread k = 40 and 48 are trees of 10^8 to 10^10, the regime where both
estimators already agreed in calibration. They validate nothing at 10^18 to
10^23 and nothing clustered. "3 of 5" tolerates a 40% failure rate in the
easiest regime. M8 tightens the rule; M6 adds the clustered case in the only
form that can be enumerated; S3 adds the largest whole tree affordable. Even
so, budget statements at 10^18 and above rest on diagnostics, not on an exact
check, and the README must say that.

### 8. Amendments and order of events: nothing compromised, two points to state

Order from the log and the commits: PREREG committed 09:30:23; gate6 09:31 to
09:34; first gt run 09:34 to 09:43 at head 73ccfbfc; amendments committed
09:48:40; gt rerun 09:48 at head a7d8f22a; no test stage in the run log. A1
(bug, a 19th order enters by the written selection rule, seeds shift before
any test unit) and A3 (text corrected to the rule the plan used) are clean.
A2 changes a gate's analysis after the gate ran; the reason (6-decimal log
storage against a 1e-9 equality test) is an artefact and the 3-standard-error
rule is untouched, so it stands, but the report must list it as a post-gate
change. Points to state openly: (a) M1 changes a gate's consequence after
its outcome was seen, on a ruling, before any official estimate of the
four orders existed; (b) the calibration board has the official colour
counts, so its numbers (spiral-out 50.2, antidiag 45.0, spiral-in 43.5)
already predicted the Part A outcome before the preregistration was written.
Part A is confirmatory and should be described that way.

### 9. Budget

About 6.3 core-hours with the additions, inside "6 to 7, never above 12":

| block | core-hours |
|---|---|
| GB (M6) and the 8x8 gate (M7) | 0.5 |
| b0 0.4, bl 0.15, b1 1.3, bs 0.6, b4 0.5, b3 0.9, b2 cut to two k 0.15 | 4.0 |
| a-official with the N = 1e5 ladder 1.4, a-null 0.3 | 1.7 |
| S1 to S3, only if the above is done | up to 1.5 |

Part B first, Part A last.
