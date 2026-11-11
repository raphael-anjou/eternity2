# Backdoor-guided branching (round 2)

Pre-registration: [`PREREG.md`](PREREG.md) (commit `fd42c55`, pushed before any
run). No amendment was needed (`PREREG_AMENDMENTS.md` absent). A separate judge
decides the outcome; the verdict at the end is the self-assessment.

Question: A3 (`../sat-backdoors`) found that fixing about N^0.59 planted cells
lets arc consistency plus piece all-different complete a generated board. Can
those backdoor cells be recognised at the root without the solution (Stage 1),
and does branching on the predicted cells first shrink exact search on the
hard ladder (Stage 2)?

## What was run

- Stage 1: root features (`ln_dom`, `corner`, `edge`, `frame_adj`, `degree`,
  `rarity`, `impact`, `fail_frac`, definitions in PREREG.md) for every cell of
  A3's 300 boards, rebuilt with A3's generator call
  (`compute/src/bin/bb_features.rs`), labelled with A3's committed
  `backdoor_cells`. Logistic regression on seeds 1..15, within-instance AUC on
  seeds 16..30 (`analysis/stage1.py`).
- Stage 2: four DFS solvers sharing A3's propagation (`sat_backdoors::Model`,
  path dependency, unchanged), the forced-move rule and the value order; they
  differ only in the branching-cell rule (`compute/src/lib.rs`, registered
  through the hard-ladder plug-in interface in `compute/src/bin/bb_ladder.rs`):
  `bb-mrv` (min |D|), `bb-impact` (Refalo-style impact), V1 `bb-learned`
  (Stage 1 logistic score) and V2 `bb-single` (best single non-domain feature
  on training AUC, which was `impact` with sign +, i.e. largest mean one-step
  reduction of sum ln|D|). Lookahead probes never prune; they only score.
- Rungs: R6 = 6x6 c=5 and R7 = 7x7 c=6 (ladder `primary`), R8 = 8x8 c=10 (new
  ladder file `8-c10`: `hard_ladder::instance::generate(8, 10, seed)`, i.e.
  `generate_solved_framed_bc(8, 15, seed, true, 5)` plus the ladder scramble;
  `hard_ladder check-instances` passes). Cap 1,000,000 nodes (PROTOCOL.md
  node = `NodeCounter::try_assign`), plus the preregistered 240 s wall-clock
  guard on `bb-*` runs (a time-censored run is censored; a pair where the
  baseline is time-censored and the candidate solved scores 0).
- Baselines at R8 (`scb-gacolor_ac3_random`, `hp-dfs`) were run here under the
  same cap; at R6/R7 the committed hard-ladder rows were used, restricted to the
  same seeds.
- Sample-size rule: calibration (seeds 1001..1010, cap 2e6) projected 85.8 min
  of test wall clock for 30 seeds (`results/calibration/projection.json`),
  above the 50 min threshold, so every test run used seeds 1..20.

## Stage 1 results (predictability, `results/stage1/summary.json`)

150 test instances (seeds 16..30, both regimes, n = 6..10), mean
within-instance AUC, percentile bootstrap over instances (10,000, seed
20261111):

| score | test AUC | 95% CI |
|---|---|---|
| learned logistic (V1) | 0.637 | [0.617, 0.657] |
| fail-first, -\|D\| (kill comparator) | 0.373 | [0.359, 0.387] |
| Delta = learned - fail-first (K1 statistic) | **+0.264** | **[0.233, 0.295]** |
| domain size with sign chosen on training (+\|D\|) | 0.627 | [0.613, 0.642] |
| learned - signed domain size | +0.010 | [-0.005, 0.025] |
| best single non-domain feature: `impact` (+) | 0.648 | [0.631, 0.666] |
| `degree` (+) | 0.633 | [0.620, 0.647] |
| `edge` (-) | 0.606 | [0.592, 0.619] |
| `frame_adj` (+) / `rarity` (+) | 0.569 / 0.568 | |
| `corner` (-) / `fail_frac` (+) | 0.525 / 0.519 | |
| control: low cell index | 0.536 | [0.515, 0.557] |

By A3 `k_source`: exact 0.645 vs 0.362 (87 instances), greedy upper bound
0.626 vs 0.389 (63). By regime: A 0.621 vs 0.392, B 0.653 vs 0.355. Delta CI
is above 0 in every size, regime and source split.

Reading: K1 does not fire, but the margin over fail-first is mostly the sign of
the domain-size score. Backdoor cells are the cells with **large** root
domains (interior cells with many neighbours), the opposite of what fail-first
branches on. Once domain size is given the sign learned on training data, the
eight-feature model adds +0.010 AUC (CI includes 0), and the strongest single
feature (one-step impact, 0.648) is in the same range. Weights:
`ln_dom` +0.003, `corner` -0.002, `edge` -0.195, `frame_adj` -0.158,
`degree` +0.054, `rarity` -0.125, `impact` +0.400, `fail_frac` +0.091.

## Stage 2 results (search, `results/summary.json`, per-pair rows `results/pairs.csv`)

Nodes = `NodeCounter::try_assign` calls (forced and branching assignments,
probes excluded). KM quantiles with censored = +infinity ("> cap" = not
reached). Wall clock: 8 concurrent runs on the same VM. Ratio = exp(upper
median paired log-ratio), "+inf" when the candidate is censored on more than
half the pairs. Holm over the two variants per (rung, baseline), alpha 0.05.
The label vs the strongest dynamic baseline is the worst label against
`bb-mrv`, `bb-impact` and `scb-gacolor_ac3_random`.

### R6 (6x6, ladder primary, seeds 1..20)

| solver | solved | Wilson 95% | KM q1 | KM median | KM q3 | median wall ms | total wall s | propagations | time-censored |
|---|---|---|---|---|---|---|---|---|---|
| `bb-learned` | 20/20 | [0.84, 1.00] | 1,728 | 5,453 | 17,590 | 1,276 | 44.5 | 4,913,067 | 0 |
| `bb-single` | 20/20 | [0.84, 1.00] | 1,506 | 2,886 | 7,498 | 787 | 27.0 | 2,660,935 | 0 |
| `bb-mrv` | 20/20 | [0.84, 1.00] | 257 | 400 | 1,092 | 5 | 0.2 | 14,801 | 0 |
| `bb-impact` | 20/20 | [0.84, 1.00] | 637 | 965 | 2,112 | 475 | 14.1 | 994,997 | 0 |
| `scb-gacolor_ac3_random` | 20/20 | [0.84, 1.00] | 620 | 1,350 | 2,870 | 6 | 0.2 | 1,814,791 | 0 |
| `hp-dfs` | 20/20 | [0.84, 1.00] | 31,231 | 41,823 | 62,556 | 24 | 0.6 | 0 | 0 |

| variant | baseline | median ratio | 95% CI median log-ratio | p (boot) | Holm | label |
|---|---|---|---|---|---|---|
| `bb-learned` | `bb-mrv` | 17.2 | [1.512, 3.966] | 1.0000 | no | NO-GAIN |
| `bb-single` | `bb-mrv` | 8.1 | [1.171, 3.117] | 1.0000 | no | NO-GAIN |
| `bb-learned` | `bb-impact` | 5.04 | [1.372, 2.068] | 1.0000 | no | NO-GAIN |
| `bb-single` | `bb-impact` | 3.07 | [0.769, 1.389] | 1.0000 | no | NO-GAIN |
| `bb-learned` | `scb-gacolor_ac3_random` | 8.01 | [1.433, 2.460] | 1.0000 | no | NO-GAIN |
| `bb-single` | `scb-gacolor_ac3_random` | 4.36 | [0.747, 1.775] | 1.0000 | no | NO-GAIN |
| `bb-learned` | `hp-dfs` | 0.234 | [-3.409, -0.910] | 0.0006 | reject | PASS |
| `bb-single` | `hp-dfs` | 0.113 | [-3.799, -1.665] | 0.0000 | reject | PASS |

Label vs strongest dynamic baseline: `bb-learned` NO-GAIN, `bb-single` NO-GAIN

### R7 (7x7, ladder primary, seeds 1..20)

| solver | solved | Wilson 95% | KM q1 | KM median | KM q3 | median wall ms | total wall s | propagations | time-censored |
|---|---|---|---|---|---|---|---|---|---|
| `bb-learned` | 1/20 | [0.01, 0.24] | > cap | > cap | > cap | 240,002 | 4,572.5 | 360,749,454 | 15 |
| `bb-single` | 1/20 | [0.01, 0.24] | > cap | > cap | > cap | 240,002 | 4,652.8 | 350,172,897 | 19 |
| `bb-mrv` | 15/20 | [0.53, 0.89] | 253,491 | 588,305 | 882,001 | 14,601 | 258.4 | 11,897,151 | 0 |
| `bb-impact` | 5/20 | [0.11, 0.47] | 195,779 | > cap | > cap | 240,007 | 4,100.1 | 185,997,623 | 15 |
| `scb-gacolor_ac3_random` | 9/20 | [0.26, 0.66] | 272,281 | > cap | > cap | 7,721 | 118.5 | 1,196,814,257 | 0 |
| `hp-dfs` | 0/20 | [0.00, 0.16] | > cap | > cap | > cap | 681 | 13.7 | 0 | 0 |

| variant | baseline | median ratio | 95% CI median log-ratio | p (boot) | Holm | label |
|---|---|---|---|---|---|---|
| `bb-learned` | `bb-mrv` | +inf | [+inf, +inf] | 1.0000 | no | NO-GAIN |
| `bb-single` | `bb-mrv` | +inf | [+inf, +inf] | 1.0000 | no | NO-GAIN |
| `bb-learned` | `bb-impact` | 1 | [0.000, 0.000] | 1.0000 | no | NO-GAIN |
| `bb-single` | `bb-impact` | 1 | [0.000, 0.000] | 1.0000 | no | NO-GAIN |
| `bb-learned` | `scb-gacolor_ac3_random` | 1 | [0.000, +inf] | 1.0000 | no | NO-GAIN |
| `bb-single` | `scb-gacolor_ac3_random` | 1 | [0.000, +inf] | 1.0000 | no | NO-GAIN |
| `bb-learned` | `hp-dfs` | 1 | [0.000, 0.000] | 1.0000 | no | NO-GAIN |
| `bb-single` | `hp-dfs` | 1 | [0.000, 0.000] | 1.0000 | no | NO-GAIN |

Label vs strongest dynamic baseline: `bb-learned` NO-GAIN, `bb-single` NO-GAIN

### R8 (8x8, ladder 8-c10, seeds 1..20)

| solver | solved | Wilson 95% | KM q1 | KM median | KM q3 | median wall ms | total wall s | propagations | time-censored |
|---|---|---|---|---|---|---|---|---|---|
| `bb-learned` | 2/20 | [0.03, 0.30] | > cap | > cap | > cap | 240,008 | 4,625.2 | 263,351,771 | 18 |
| `bb-single` | 1/20 | [0.01, 0.24] | > cap | > cap | > cap | 240,011 | 4,738.8 | 274,052,091 | 19 |
| `bb-mrv` | 20/20 | [0.84, 1.00] | 43,219 | 95,128 | 160,698 | 5,060 | 106.1 | 2,628,980 | 0 |
| `bb-impact` | 14/20 | [0.48, 0.85] | 14,603 | 33,987 | > cap | 164,673 | 2,993.3 | 72,059,076 | 6 |
| `scb-gacolor_ac3_random` | 12/20 | [0.39, 0.78] | 363,902 | 468,270 | > cap | 24,853 | 425.6 | 3,815,410,388 | 0 |
| `hp-dfs` | 12/20 | [0.39, 0.78] | 413,415 | 585,007 | > cap | 927 | 15.0 | 0 | 0 |

| variant | baseline | median ratio | 95% CI median log-ratio | p (boot) | Holm | label |
|---|---|---|---|---|---|---|
| `bb-learned` | `bb-mrv` | +inf | [+inf, +inf] | 1.0000 | no | NO-GAIN |
| `bb-single` | `bb-mrv` | +inf | [+inf, +inf] | 1.0000 | no | NO-GAIN |
| `bb-learned` | `bb-impact` | +inf | [1.805, +inf] | 1.0000 | no | NO-GAIN |
| `bb-single` | `bb-impact` | +inf | [2.728, +inf] | 1.0000 | no | NO-GAIN |
| `bb-learned` | `scb-gacolor_ac3_random` | +inf | [0.000, +inf] | 1.0000 | no | NO-GAIN |
| `bb-single` | `scb-gacolor_ac3_random` | +inf | [0.000, +inf] | 1.0000 | no | NO-GAIN |
| `bb-learned` | `hp-dfs` | +inf | [0.000, +inf] | 1.0000 | no | NO-GAIN |
| `bb-single` | `hp-dfs` | +inf | [0.000, +inf] | 1.0000 | no | NO-GAIN |

Label vs strongest dynamic baseline: `bb-learned` NO-GAIN, `bb-single` NO-GAIN

Kill: {'K1': False, 'K2': True, 'K3': True, 'killed': True, 'rungs_complete': ['R6', 'R7', 'R8'], 'stage1_delta': 0.2637097381730833, 'stage1_delta_ci': [0.23254810200594964, 0.29525228994616]}

Time-censored runs (240 s guard) are listed by instance in `results/summary.json`
(`time_censored_ids`). Every test-seed run of both variants is in
`results/test/`.

## Outcome against the kill criterion (self-assessment)

| criterion | result |
|---|---|
| K1 (Stage 1: CI of AUC_learned - AUC_failfirst above 0) | does not fire: +0.264, CI [0.233, 0.295] |
| K2 (no variant PASS at R8 vs the strongest dynamic baseline) | **fires**: both NO-GAIN |
| K3 (both variants NO-GAIN at all three rungs) | **fires** |

Self-assessed outcome: **killed**.

What the evidence says:

- Backdoor membership is predictable at the root above chance (AUC about
  0.64), but not better than domain size with the right sign; the signal is
  "branch on large, well-connected interior cells", the reverse of fail-first.
- Branching on that signal makes search worse, not better. At R6 the variants
  need more nodes than `bb-mrv` by a median paired ratio of 8.1 (`bb-single`) and 17.2 (`bb-learned`) (KM median 2,886 and 5,453 vs 400),
  and at R7 and R8 they solve 1 to 2 of 20 against 15/20 and 20/20 for
  `bb-mrv`. Most variant runs at R7/R8 end at the 240 s guard, because every
  node costs a full one-step lookahead over all candidate cells (about 4,000
  nodes/s against about 40,000 for `bb-mrv`); the node cap alone would not
  have rescued them: R6, which has no censoring, already shows the loss in
  nodes.
- The variants beat `hp-dfs` at R6 (PASS), which reflects A3's propagation,
  not the backdoor signal: `bb-mrv` with the same propagation does far better.
- Side result: `bb-mrv` (A3 propagation + MRV) beats the committed ladder
  baseline `scb-gacolor_ac3_random` at all three rungs on these seeds (R8:
  20/20 vs 12/20). This was not a preregistered comparison and is reported as
  an observation only.

Limits: 20 test seeds per rung (sample-size rule); one feature set and one
model; a time guard of 240 s; A3's labels above 7x7 are greedy upper bounds.
The negative Stage 2 result is for "branch first on predicted backdoor cells"
as specified; it does not rule out other uses of backdoor structure (e.g. as
restart or decomposition hints).

## Reproduce

```bash
just experiments backdoor-branching          # all stages, about 120 min on 8 vCPU
# or stage by stage:
research/experiments/backdoor-branching/run.sh stage1     # ~1 min
research/experiments/backdoor-branching/run.sh calibrate  # ~45 min, writes projection.json
SEEDS=20 research/experiments/backdoor-branching/run.sh test   # ~60 min
research/experiments/backdoor-branching/run.sh analyze
```

Node counts are deterministic (no RNG in any `bb-*` solver); runs that hit the
240 s guard depend on machine speed and are flagged as time-censored.

## Hardware and wall clock

Devin VM, 8 vCPU Intel Xeon, 31 GB RAM, 2026-11-11. Stage 1 features 1 min,
Stage 1 fit 49 s, calibration about 45 min, test grid about 70 min (bb-* totals
in the tables above), analysis 1 min. Total about 2 h, over the 90 min target,
because calibration and test of the probe-based solvers ran into the 240 s guard
on most 7x7 and 8x8 instances.

## Files

- `PREREG.md`: preregistration (unchanged since `fd42c55`).
- `compute/`: Rust crate (`bb_ladder` plug-in runner, `bb_features` Stage 1 dump, tests).
- `analysis/stage1.py`, `analysis/project.py`, `analysis/stage2.py`, `analysis/report.py`: standard-library Python.
- `results/stage1/`: features, model, summary. `results/calibration/`: calibration rows and projection.
  `results/test/`: every test-seed row. `results/summary.json`, `results/pairs.csv`.
- Ladder change: `hard-ladder` gained ladder `8-c10` (`instance.rs`, `cli.rs`) and the
  files `hard-ladder/instances/8-c10-{test,calibration}.jsonl`; existing ladder files are byte-identical.
