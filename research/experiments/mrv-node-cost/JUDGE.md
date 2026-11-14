# Judge: Track 0, bb-mrv node cost (optimization)

**Verdict: keep.** The track label holds (the node sequence is identical to the
unmodified A22 `bb-mrv` on all 140 gate instances, recomputed independently),
and the preregistered wall-clock target is met at every rung with wide margin.
`bb-mrv-fast` may replace `bb-mrv` wherever only node counts, statuses and
boards are consumed, at n = 6..8 as gated, and at n = 9, 10 with the caveat
below.

Judged on 2026-11-12 from branch `night/2026-11-12-t0-node-cost` at `b916767`,
local only. Two cores, shared machine.

## 1. Preregistration order

`PREREG.md` is commit `47aa9b5` (18:26:16), the first commit on the branch
after the merge base `6247af0` (origin/main). Code freeze `512b10d` follows it;
every results commit (`20599ca` 18:40, `662d0b1` 19:24, `b916767` 20:10) comes
after. `PREREG.md` was not modified after `47aa9b5`. The solver code
(`compute/`) is unchanged between `512b10d` and `HEAD`. Order: valid.

## 1b. Track label: hash gate recomputed against the original source

How the reference was tied to the original, without trusting the branch's
`RefSolver`:

1. `git diff origin/main -- research/experiments/{mrv-baseline,backdoor-branching,hard-ladder,sat-backdoors} engine`
   is empty: the original crates, NodeCounter and the ladder instances are
   untouched on the branch.
2. A22's binding `bb-mrv` arm (`mrv-baseline/compute/src/main.rs`) is
   `BbSolver { rule: Rule::Mrv, guard }`, the same construction the branch
   times and gates as `bb-mrv`.
3. I copied `backdoor-branching/compute/src/lib.rs` from the branch (identical
   to origin/main) into a scratch crate and added exactly one line in
   `Search::dfs`, right after the `try_assign` check: a call hashing
   (cell, `opt_piece[o]`, `opt_rot[o]`) with the same FNV-1a byte layout.
   `diff` against the original shows only that line plus the hash helper.
   This hashes the real `BbSolver` path (including the `Rule` dispatch and
   `Static::new`), not the branch's copy.
4. My driver runs that instrumented original and the branch's
   `FastSolver<HashHook>` on each instance and requires equal hash, equal
   hashed count, hashed count = nodes, equal nodes, backtracks, propagate
   calls, status and board.

Result, all 140 binding instances (I ran the full gate, not a half):

| set | instances | orig vs fast mismatches | my orig hash vs branch `ref.hash` |
|---|---|---|---|
| R6 primary n=6 cal 1001..1010, test 1..30 | 40 | 0 | 40 equal |
| R7 primary n=7 cal 1001..1010, test 1..30 | 40 | 0 | 40 equal |
| 8-c10 cal 1001..1010, test 1..30 | 40 | 0 | 40 equal |
| 8-c10-fresh test 31..50 | 20 | 0 | 20 equal |

130 solved, 10 censored at 1e6 (all at R7), 0 exhausted, identical in both
arms. A fallback random half of the 8x8 set was preregistered in my scratch
notes (`random.Random(20261112)`, 30 of 60) before running, but was not needed
since all 60 ran.

Further cross-check: the branch's `orig` node counts equal A22's committed
`mrv-baseline/results/raw/bb-mrv` rows on the 50 instances both cover, 0
disagreements.

I also recomputed every gate flag in the committed rows field by field from
the raw JSON (not from `ref_eq_orig` / `fast_eq_ref`): 140 rows, the expected
140 instance set exactly, 0 missing, 0 extra, 0 mismatches.

Beyond the gate (not preregistered by the branch; my probe):

| instance | cap | status | orig vs fast | fast / orig, one run |
|---|---|---|---|---|
| plus2 test n9-c11-s1..s4 | 300,000 | censored | identical hash, all 4 | 0.161 to 0.174 |
| plus2 test n10-c12-s1 | 100,000 | censored | identical hash | 0.143 |

The branch's own supplementary check (plus2 calibration 9x9 1001..1003 and
10x10 1001 at 1e5) is also 4 of 4 identical.

## 2. Timing, recomputed from committed rows

Own script over `results/raw/time-*.jsonl`: every instance has exactly 10
runs in slot order A B B A A B B A A B, every run of both arms has identical
(status, nodes, backtracks, propagate). Per-instance min of 5, ratio
fast / orig, upper median, percentile bootstrap 10,000, seed 20261111:

| rung | instances | median ratio | 95% CI | secondary (median of 5) | range | ratio of summed time |
|---|---|---|---|---|---|---|
| R6 | 30 | 0.188 | [0.178, 0.203] | 0.191 [0.181, 0.198] | 0.153..0.242 | 0.193 |
| R7 | 30 (7 censored, tied) | 0.163 | [0.161, 0.166] | 0.164 [0.161, 0.166] | 0.153..0.171 | 0.163 |
| 8x8 c=10 | 50 | 0.190 | [0.188, 0.191] | 0.190 [0.189, 0.192] | 0.166..0.205 | 0.192 |

These equal README and `summary.json`. Every CI upper end is far below 1. No
instance has ratio above 1, so there is no hidden wall-clock loss. Censored
R7 runs consumed exactly 1e6 nodes in both arms, so they are a per-node cost
comparison, not a counted win; excluding them leaves the R7 median at 0.162.

Rerun of an alternated subset with the branch's own `time` mode (rebuilt from
the frozen source), 8-c10 test seeds 1..5, under concurrent load: ratios
0.186, 0.188, 0.188, 0.192, 0.203 versus the branch's 0.187, 0.185, 0.193,
0.184, 0.198; node counts identical. Same order, as expected. My single-run
orig-then-fast timings from the gate rerun give a median ratio 0.181 over 92
instances with orig above 200 ms, consistent.

## 3. Adversarial review (adapted to an optimization track)

- Planted-solution leakage: `FastSolver` receives only `&Puzzle` and builds
  the same `sat_backdoors::Model`; `planted` is never read. Clean.
- Unequal node accounting: both arms call `NodeCounter::try_assign` once per
  option tried and increment `backtracks` identically; the hash count equals
  the counter in every row. Equal.
- Baseline: the unmodified A22 binding arm, not a straw man. The orig arm
  still pays `Static::new` (unused by `Rule::Mrv`); that is the real cost of
  the A22 binary and is a per-run constant, negligible at R7 and 8x8.
- Censored runs as wins, dropped seeds, subsampling: none (above).
- Guard: both arms check the 600 s guard per node; no timed run hit it (all
  10 runs per instance identical).
- Claims: README claims stay within the measured rungs and this machine. The
  line "Supports n <= 11" is argued from an `assert!(cells <= 128)`, not
  measured at n = 11; at n >= 12 the solver panics rather than silently
  misbehaving.

Minor deviations, no effect on the verdict:

- PREREG names `results/gate.jsonl`, `results/gate-supp.jsonl`,
  `results/timing.jsonl`; the branch writes split files under
  `results/raw/`. Same content, different layout, and README says "No
  amendments". Fix if wanted: one line in a `PREREG_AMENDMENTS.md`.
- The candidate was developed on calibration seeds before the freeze. This is
  disclosed and does not matter for an identity gate, which every calibration
  seed passes with the frozen code.

## 4. Use by other tracks at 9x9

Safe at 9x9 c=11 with this caveat: identity there is measured only on
censored prefixes (7 instances, at most 300,000 nodes each, no solved 9x9
board compared). The equivalence argument (same monotone rules, same
fixpoint) plus 145 of 145 identical sequences make a divergence unlikely, but
a track that reports 9x9 node counts from `bb-mrv-fast` should run
`mrv_node_cost gate` on a few of its own instances at its own cap and record
the result; the gate mode exists for that. Speed-up at 9x9 and 10x10 in my
single runs: about 5.8x to 7x.
