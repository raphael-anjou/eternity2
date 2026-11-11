# Hard-ladder comparison protocol

Mandatory for every candidate exact-search method compared on this ladder.
A candidate experiment may add rules; it may not weaken, drop or reinterpret
any rule below. The ladder's own usability verdict is in `README.md`; a
candidate may only use a ladder configuration (colour set, cap) that the
README or a recorded amendment declares usable.

## Fixed inputs

- Instances: `instances/<ladder>-test.jsonl` (30 per size, n = 6..10). The
  planted board in each row is for verification only; a solver receives only
  the `puzzle` object. Calibration instances are for tuning; they never enter
  a comparison.
- Cap: `TEST_CAP` (1,000,000 nodes, amendment A1) per instance, identical for candidate and
  baseline. The runner sets it from the role; it is not a command-line knob.
- Baseline: the committed rows `results/test/<ladder>/<baseline>.jsonl` for
  the calibrated best baseline (see `CALIBRATION.md`) and for `hp-dfs`.
  Candidates compare against the best baseline; `hp-dfs` is also reported.
- Node metric: one node per (cell, piece, rotation) assignment made by the
  search that agrees with the rim and every assigned neighbour, counted
  whether or not propagation later refutes it (`src/counter.rs`). A candidate
  calls `NodeCounter::try_assign` once per assignment, before making it, and
  stops when it returns false. Propagation work, fit tests and backtracks are
  secondary columns. A candidate that assigns without calling `try_assign`,
  or batches several assignments behind one call, is invalid.
- A run counts as solved only if the harness verifies the returned board.
  Any `error` row (panic, unverifiable board, exhaustion of a planted
  instance) invalidates the candidate at that rung until fixed.

## G1: per-rung paired effort

For each instance i at rung n, with C the cap:

| candidate | baseline | log-ratio r_i |
|---|---|---|
| solved, c_i | solved, b_i | ln(c_i / b_i) |
| solved, c_i | censored | ln(c_i / C) (an upper bound on the true ratio) |
| censored | solved | +infinity (candidate loses) |
| censored | censored | 0 (tie) |

Statistic: the upper median of the 30 r_i (the 16th order statistic, which is
the conservative choice for an even sample). 95% CI: percentile bootstrap,
10,000 paired resamples of instances, seed 20261111.

G1 passes at rung n iff exp(median r) <= 0.5 AND the upper end of the 95% CI
of the median r is < 0.

## Rung order and dropping

G1 is evaluated at n = 6, then 8, then 10. A method is dropped at the first of
these rungs where G1 fails; results at larger rungs may be reported but cannot
rescue it. Rungs 7 and 9 enter G2 and the tables.

## G2: growth

Per method, m_n is the Kaplan-Meier median nodes at rung n (the 15th order
statistic of 30 with censored = +infinity). Slope s = OLS slope of log10(m_n)
on cell count n^2 over n = 6..10. Censoring is resolved against the
candidate: a censored candidate median makes its slope +infinity (G2 fails);
a censored baseline median is set to C (a lower bound, which lowers the
baseline slope). G2 statistic: d = s_candidate - s_baseline; 95% CI by
bootstrap (10,000 resamples of instances within each rung, paired across
methods, seed 20261111). G2 passes iff d < 0 and the upper end of its CI is
< 0.

## Mandatory reporting

Every candidate report gives, per rung and for candidate and baseline: solved
count out of 30 with Wilson 95% interval, KM median and quartiles of nodes,
median and total wall clock (ms, same VM, same thread count, stated), and the
G1 table above, plus G2. A node win that is a wall-clock loss must be visible
in the same table. No claim extends beyond n = 10 or to the 16x16 board.

## Commands

```
cd research/experiments/hard-ladder
<your binary> run --solver NAME --role test --ladder primary   # writes results/test/primary/NAME.jsonl
python3 analysis/analyze.py compare results/test/primary/NAME.jsonl \
    ../hard-ladder/results/test/primary/<baseline>.jsonl --out results/compare-NAME.json
```

`analysis/analyze.py compare` implements G1, the dropping rule and G2 exactly
as written here; if code and text disagree, the text governs and the code is
fixed.
