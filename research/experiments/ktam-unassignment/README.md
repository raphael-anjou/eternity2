# A15: non-chronological, bond-weighted unassignment (kTAM detachment, IMS eviction)

Round-3 test of agenda item A15 on the hard-instance ladder
(`../hard-ladder`). Preregistration: [`PREREG.md`](PREREG.md) (first commit on
the branch, pushed before any run). Calibration: [`CALIBRATION.md`](CALIBRATION.md)
(committed before any test-seed run). Deviations: [`PREREG_AMENDMENTS.md`](PREREG_AMENDMENTS.md)
(one ordering note, no change to arms, analysis or kill criterion).

## What was run

All method arms share one placement machinery (`compute/src/search.rs`): a
consistent partial board (no mismatched seam, grey exactly on the rim), MRV
cell choice with ties by attachment bonds, a uniformly random consistent
candidate, and at a dead end one of three unassignment rules:

- **B3** random eviction: remove K uniformly random placed pieces within
  Manhattan radius R of the dead cell (selected `b3-rinf-k2`: R = inf, K = 2).
- **V1** kTAM-style detachment: remove K pieces within radius R with
  probability proportional to exp(-beta * bonds) (selected `v1-beta2-rinf-k1`).
- **V2** IMS-style eviction: place a minimum-cost rim-consistent piece at the
  dead cell, evicting the mismatching neighbours and the piece's other copy,
  with tabu T and noise (selected `v2-t0-n0.1`: no tabu, noise 0.1).

Baselines through the ladder registry, unchanged: **B1** `scb-gacolor_ac3_random`,
**B2** `hp-dfs`. Their R6/R7 rows equal the committed ladder rows node for
node (checked by the analysis: 0 mismatching rows).

Effort: one event = one placement (one ladder node, `NodeCounter::try_assign`)
or one removal. Method arms: events = placements + removals, counted exactly.
DFS arms: events = 2 * nodes - n^2 for a solve (every placement except the
final n^2 is undone once). Budget E = 1e6 events per instance; every arm also
ran to the ladder's 1e6 node cap for the secondary node table. Every claimed
solve is verified with `hard_ladder::verify_board`; the planted board is read
by the driver only after the solver returns.

Rungs: R6 (6x6 c=5) and R7 (7x7 c=6), ladder test seeds 1..30. R8 (8x8 c=9,
instances generated and committed in `instances/`) was **not run**: the
preregistered trigger (V1 or V2 GAIN or PASS against B1 at R6 or R7) did not
fire; both variants are NO-GAIN against B1 on both rungs.

## Results (test seeds, from `results/summary.json`)
### R6

| arm | solved within E /30 | Wilson 95% | KM q25 / median / q75 events | solved within 1e6 nodes | KM median nodes | removals (total) | removed = planted | placed = planted | wall median ms | wall total s |
|---|---|---|---|---|---|---|---|---|---|---|
| B1 | 30 | 0.89 to 1.00 | 1,204 / 1,866 / 3,844 | 30 | 951 | n/a | n/a | n/a | 6 | 0.2 |
| B2 | 30 | 0.89 to 1.00 | 55,772 / 83,610 / 125,076 | 30 | 41,823 | n/a | n/a | n/a | 19 | 0.6 |
| B3 | 19 | 0.46 to 0.78 | 202,290 / 565,948 / censored | 24 | 282,992 | 13,557,588 | 0.128 | 0.128 | 297 | 12.3 |
| V1 | 23 | 0.59 to 0.88 | 154,016 / 481,968 / 814,676 | 27 | 241,002 | 9,618,001 | 0.085 | 0.085 | 267 | 10.1 |
| V2 | 27 | 0.74 to 0.97 | 51,300 / 221,284 / 620,100 | 30 | 110,660 | 5,939,624 | 0.068 | 0.068 | 88 | 4.2 |

| comparison (events, G1) | variant | median ratio | 95% CI of median ln ratio | p one-sided | Holm label | unadjusted label |
|---|---|---|---|---|---|---|
| vs B1 (B1) | V1 | 186.345 | 4.554 to 6.536 | 1.0000 | NO-GAIN | NO-GAIN |
| vs B1 (B1) | V2 | 100.350 | 3.295 to 5.493 | 1.0000 | NO-GAIN | NO-GAIN |
| vs B1orB3 (B1) | V1 | 186.345 | 4.554 to 6.536 | 1.0000 | NO-GAIN | NO-GAIN |
| vs B1orB3 (B1) | V2 | 100.350 | 3.295 to 5.493 | 1.0000 | NO-GAIN | NO-GAIN |
| vs B3 (B3) | V1 | 0.880 | -0.730 to 0.000 | 0.2911 | NO-GAIN | NO-GAIN |
| vs B3 (B3) | V2 | 0.430 | -1.398 to -0.332 | 0.0042 | PASS | PASS |
| vs B2 (B2) | V1 | 7.644 | 0.976 to 2.860 | 1.0000 | NO-GAIN | NO-GAIN |
| vs B2 (B2) | V2 | 2.887 | 0.388 to 1.535 | 0.9986 | NO-GAIN | NO-GAIN |

Best variant: V2; vs B3 median ratio 0.430, p = 0.0042.
Kill terms: `{"both_no_gain_vs_B1": true, "best_solved": 27, "max_baseline_solved": 30, "best_le_max_plus_3": true, "b3_within_2": false, "best_not_sig_vs_B3": false}`
Baseline reproduction: `{"B1": 0, "B2": 0}` mismatching rows.

### R7

| arm | solved within E /30 | Wilson 95% | KM q25 / median / q75 events | solved within 1e6 nodes | KM median nodes | removals (total) | removed = planted | placed = planted | wall median ms | wall total s |
|---|---|---|---|---|---|---|---|---|---|---|
| B1 | 10 | 0.19 to 0.51 | 544,513 / censored / censored | 16 | 921,773 | n/a | n/a | n/a | 6672 | 160.3 |
| B2 | 0 | 0.00 to 0.11 | censored / censored / censored | 0 | censored | n/a | n/a | n/a | 476 | 14.9 |
| B3 | 0 | 0.00 to 0.11 | censored / censored / censored | 0 | censored | 29,999,574 | 0.099 | 0.099 | 1007 | 32.2 |
| V1 | 0 | 0.00 to 0.11 | censored / censored / censored | 0 | censored | 29,999,390 | 0.060 | 0.060 | 1305 | 40.5 |
| V2 | 0 | 0.00 to 0.11 | censored / censored / censored | 0 | censored | 29,999,064 | 0.046 | 0.046 | 1012 | 32.6 |

| comparison (events, G1) | variant | median ratio | 95% CI of median ln ratio | p one-sided | Holm label | unadjusted label |
|---|---|---|---|---|---|---|
| vs B1 (B1) | V1 | 1.000 | 0.000 to +inf | 1.0000 | NO-GAIN | NO-GAIN |
| vs B1 (B1) | V2 | 1.000 | 0.000 to +inf | 1.0000 | NO-GAIN | NO-GAIN |
| vs B1orB3 (B1) | V1 | 1.000 | 0.000 to +inf | 1.0000 | NO-GAIN | NO-GAIN |
| vs B1orB3 (B1) | V2 | 1.000 | 0.000 to +inf | 1.0000 | NO-GAIN | NO-GAIN |
| vs B3 (B3) | V1 | 1.000 | 0.000 to 0.000 | 1.0000 | NO-GAIN | NO-GAIN |
| vs B3 (B3) | V2 | 1.000 | 0.000 to 0.000 | 1.0000 | NO-GAIN | NO-GAIN |
| vs B2 (B2) | V1 | 1.000 | 0.000 to 0.000 | 1.0000 | NO-GAIN | NO-GAIN |
| vs B2 (B2) | V2 | 1.000 | 0.000 to 0.000 | 1.0000 | NO-GAIN | NO-GAIN |

Best variant: V1; vs B3 median ratio 1.000, p = 1.0000.
Kill terms: `{"both_no_gain_vs_B1": true, "best_solved": 0, "max_baseline_solved": 10, "best_le_max_plus_3": true, "b3_within_2": true, "best_not_sig_vs_B3": true}`
Baseline reproduction: `{"B1": 0, "B2": 0}` mismatching rows.

R8 triggered: False. Method killed: True. Mechanism killed: False. Self-assessed outcome: **killed**.


Reading the tables:

- B1 solves every R6 board within a few thousand events (KM median 1,866
  events, 951 nodes). The best method arm, V2, solves 27/30 within E with KM
  median 221,284 events: about 100 times more effort per paired instance
  (G1 median ratio 100.4, 95% CI of median ln ratio 3.30 to 5.49). V1 is
  worse (23/30, ratio 186).
- At R7 no method arm and no B3 run solves any board within 1e6 events (nor
  within 1e6 nodes); B1 solves 10/30 within E (16/30 within 1e6 nodes, the
  round-2 figure).
- Mechanism versus generic unassignment: at R6, V2 beats the tuned random
  eviction arm B3 (27 vs 19 solved, G1 median ratio 0.43, p = 0.004, PASS
  under Holm against B3); V1 is not separable from B3 (ratio 0.88, p = 0.29).
  At R7 all three are 0/30, so no separation there.
- Removed pieces matching the planted placement: 6.8% (V2), 8.5% (V1), 12.8%
  (B3) at R6, equal to each arm's own placement rate to three decimals. This
  is close to forced by the design: almost every placed piece is eventually
  removed in a censored or long run, so removals and placements are nearly
  the same multiset. The measure therefore shows no selectivity of the bond or
  blocking rule toward wrong pieces, and V1/V2 place planted pieces less often
  than B3. Descriptive only; boards with other solutions make it a lower bound.

## Exploratory: equal CPU time at R7 (non-binding, PREREG_AMENDMENTS.md E1)

Added after the verdict because B1 runs AC-3 and GAColor propagation inside
every node, so nodes and events do not charge its work per node. This check
does not change the verdict above. `./run.sh explore` measures per-thread CPU
time (`CLOCK_THREAD_CPUTIME_ID`) for every arm on the 30 R7 test instances at
1e6 nodes (8 parallel runs on 8 vCPU), then reruns the method arms at the node
cap that matches B1's CPU time per instance. Node caps are fixed numbers, so
the reruns stay deterministic in nodes; only the CPU times vary by machine.
Hardware instruction counters are not available on this VM, so there are no
instruction counts. The native propagation counters measure different things
in each solver (B1: engine propagation steps; B3/V1/V2: candidate-list scans),
so the propagation-per-node column is descriptive only and not comparable
across solver families.

| arm | nodes per CPU s | native propagation per node | median CPU ms per 1e6-node run | CPU/wall |
|---|---|---|---|---|
| B1 | 89,404 | 83.1 | 8,405 | 0.996 |
| B2 | 1,590,883 | 0.0 | 568 | 0.998 |
| B3 | 698,360 | 153.6 | 1,448 | 0.994 |
| V1 | 488,668 | 183.1 | 2,016 | 0.997 |
| V2 | 691,397 | 101.5 | 1,483 | 0.997 |

Equal CPU budget C = 11.8 s per instance (median CPU time of a censored B1 run).

| arm | node cap | solved within C /30 | KM median CPU s |
|---|---|---|---|
| B1 | 1,000,000 | 16 | 10.2 |
| B3 | 8,300,000 | 0 | not reached |
| V1 | 5,800,000 | 1 | not reached |
| V2 | 8,200,000 | 1 | not reached |

- Per node, B1 costs about 8 times the CPU of V2 (89 thousand vs 691 thousand
  nodes per CPU second).
- Given the same CPU time per instance (11.8 s), B1 solves 16/30, V1 and V2
  1/30 each and B3 0/30. Charging B1's propagation therefore does not change
  the R7 picture.
- CPU pressure: CPU time is within 1% of wall time in every arm, so the 8
  parallel runs were not oversubscribed. Running them in parallel still slows
  each run through shared cache and memory. Single-thread nodes per CPU second
  are 105,890 for B1 (8 parallel runs: 16% slower) and 967,251 for V2 (29%
  slower), so the per-node cost ratio between them depends on load (7 to 9).
- Raw rows: `results/exploratory/cpu/R7/*.jsonl`; summary:
  `results/exploratory/cpu/summary.json`.

## Outcome against the kill criterion (self-assessment; a separate judge decides)

- **Method clause fires**: on every rung run (R6, R7) both V1 and V2 are
  NO-GAIN (Holm) against B1 on events, and the best variant solves no more
  than max(B1, B2, B3) + 3 (R6: 27 <= 33; R7: 0 <= 13).
- **Mechanism clause does not fire**: at R6, B3 is not within 2 solves of the
  best variant (19 vs 27) and V2 is significantly better than B3.
- Self-assessed outcome: **killed** (method clause). Within this family of
  incomplete unassignment searches, IMS-style eviction is measurably better
  than random eviction on 6x6 c=5, but every variant is about two orders of
  magnitude behind the complete DFS baseline at R6 and solves nothing at R7.
  This says nothing about other unassignment designs (for example combined
  with propagation or with a chronological core), which were not tested.

## Reproduce

From a clean checkout (needs Rust and Python 3, stdlib only):

```
just experiments ktam-unassignment          # or: research/experiments/ktam-unassignment/run.sh
just experiments ktam-unassignment check    # build, tests, clippy, instance check
```

Stages: `check`, `calibrate`, `test`, `analyze`, `all` (default). Events and
nodes are deterministic given the seeds (run seed 1, instance seeds); wall
clock is not. Raw rows: `results/calibration/<arm>/<config>.jsonl`,
`results/test/<rung>/<arm>.jsonl`; summary: `results/summary.json` and
`results/summary.md`.

Commands run for the committed results (in this directory):

```
compute/target/release/ktam gen                       # R8 instances (once)
./run.sh check
./run.sh calibrate                                     # about 90 s
./run.sh test                                          # about 48 s, R8 skipped by the trigger
python3 analysis/analyze.py summary
```

Clean-checkout check: a fresh `git clone --depth 1` of the branch ran
`just experiments ktam-unassignment` end to end in 2 min 19 s (build, 5 tests,
clippy, instance check, calibration, test, analysis) and reproduced all 1,380
raw rows (calibration and test) and `summary.json` exactly, apart from wall
clock fields.

Context for the effort metric: B1 runs AC-3 and GAColor propagation after
every placement and only placements count as nodes or events, so its
propagation work is free under E. The native propagation counters are in
`results/summary.json` but measure different things per arm. Per run wall
clock (tables above) is the cross-arm cost check: at R6 B1 is still far
faster (median 6 ms vs 88 ms for V2).

Hardware: Devin VM, 8 vCPU Intel Xeon, 31 GB RAM, Rust 1.99.1, Python 3.12,
8 runs in parallel. Wall clock: calibration about 90 s for 54 configs x 20
runs; test stage 48 s for 5 arms x 60 runs; total session compute well under
the 90 minute budget. Wall times in the tables are per run with 8 runs in
parallel.
