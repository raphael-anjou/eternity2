# Near-codeword spectrum: fewest breaks at each overlap with a planted solution

Pre-registered round-2 test. Binding statement: [PREREG.md](PREREG.md)
(commit bf2fecb, pushed before any run), with amendments
[PREREG_AMENDMENTS.md](PREREG_AMENDMENTS.md). The first run (A1 to A5) was
judged invalid ([JUDGE_ROUND1.md](JUDGE_ROUND1.md), committed verbatim): the
corner pin does not remove the 4-fold board rotation, so its far band held a
turned copy of the planted solution, and K1c was undetermined. This README
reports the fix run (A6 to A11), which decides the outcome, and keeps the
first run's numbers as a labelled secondary record.

**Self-assessed outcome: killed (K1a fires at 5x5 and at 6x6; K1c* at
7x7 also fires).** Once
the four turned copies of the planted solution are quotiented out, the
cheapest board far from every copy costs more breaks than the cheapest
mid-overlap board. The mid band is a cheap saddle between the planted
solution and its neighbourhood, not a barrier in front of a distinct
low-break floor. Stage 3 (where solver boards sit) was not run (A8).

## What was measured

On `generate_solved_framed(n, C, seed, true)` boards, with the planted corner
piece pinned at cell 0, CP-SAT minimises internal breaks over full gray-legal
boards whose distance from the planted board lies in a band. Distance is
colour-tuple distance (duplicate frame pieces swapped in place do not count).

- **d\* (decides the outcome, A6):** d* = min over k = 0..3 of d_k, where d_k
  is the distance to the planted board turned k quarter turns. Bands (N =
  n^2): near* [1, ceil(N/4)-1], mid* [ceil(N/4), floor(3N/4)], far*
  [floor(3N/4)+1, N] (far from every turned copy, the judge's "distinct"
  band), K2* [ceil(N/3), floor(3N/4)]. B* = H*_mid - H*_far.
- **d_0 (first run, secondary):** the same bands on d_0 alone, far ending at
  N-1. Its far band contains the turned planted copy with the corner swapped
  back, a 2- or 4-break board that needs no search.

Values are [LB, UB] intervals when a 120 s x 4 worker solve did not close;
every criterion is evaluated at both interval ends as pre-registered. Every
witness was rechecked twice: in the solver, and by `scripts/check_fix.py`
(numpy, shares no code with the solver): permutation, gray legality, pin,
breaks = UB, all four d_k, band membership, breaks cap. Fix run: 340
witnesses, 0 failures. First run: 916 witnesses, 0 failures (judge).

## Commands

```
research/experiments/near-codeword-spectrum/scripts/run_fix.sh   # fix run (A6 to A11), resumable
just experiments near-codeword-spectrum                          # first run (scripts/run_all.sh)
```

`run_fix.sh` creates `.venv` with `requirements.txt` (OR-Tools 9.15.6755,
SciPy 1.18.1, NumPy 2.5.4) via uv if missing, runs
`scripts/cpsat_quotient.py --stage {star1,k1c,star7}`, then
`scripts/analyze_fix.py` (writes `results/summary_fix.json`) and
`scripts/check_fix.py`. It reads the committed `results/instances.jsonl`
and `results/raw/annealed.jsonl` (made by the Rust crate, `cargo run
--release --bin ncs`; the judge reproduced both byte for byte). Re-evaluate
committed raw data only: `.venv/bin/python scripts/analyze_fix.py`. Rust
checks: `cargo test --release` (5 tests).

Hardware and wall clock: VM, 8 vCPU Intel Xeon, 31 GB RAM, no AWS, 2 jobs x
4 CP-SAT workers at a time. Fix run 2026-11-11 from 15:15 UTC: star1 4166 s,
k1c 1861 s, star7 1502 s (about 2.1 h wall, about 17 core-hours). First
run: about 1.6 h wall, 13 core-hours. Witnesses and bounds of time-limited
solves are not bit-for-bit reproducible; closed optima and INFEASIBLE proofs
are.

## Results on d* (fix run, A6)

Median [LB, UB] over instances (one number when both ends agree); "closed"
counts solves at OPTIMAL.

| palette | near* | mid* | far* | K2* | median B* | closed mid* / far* / K2* |
|---|---|---|---|---|---|---|
| 5x5 C=13 (primary, 30) | 2 | 4 | 7 | 4.5 | -3 | 30 / 30 / 30 |
| 6x6 C=14 (primary, 30) | 2 | 4 | [5, 12] | 5 | [-7, -1] | 26 / 0 / 17 |
| 5x5 C=9 (contrast, 10) | 2 | 2 | 2.5 | 2 | -0.5 | 10 / 10 / 10 |
| 6x6 C=10 (contrast, 10) | 2 | 2 | 2 | 2 | 0 | 9 / 10 / 10 |

- No instance has mid* strictly above both near* and far* (0/30 at both
  primary sizes). Mid* UB <= far* LB, i.e. B* <= 0 certified, on 30/30 (5x5)
  and 26/30 (6x6) primary instances.
- far* at 5x5 C=13: optima 6 (4 instances), 7 (15), 8 (11). No primary
  instance has a 0-break board in far*: no second solution distinct from the
  planted one and its turned copies was found (proved absent at 5x5; at 6x6
  the far* LB is 4 to 6 on every instance). The 6x6 far* witnesses sit at
  d* = 28 to 32 of 36.
- Near* = 0 on 1/30 (5x5) and 2/30 (6x6) primary instances: a second
  solution close to the planted one.
- Contrast palettes: every band is about 2 breaks, B* about 0. The no-gap
  control behaves as expected.
- 7x7 C=15 (A9, 150 s x 4 workers per band): mid* median [3, 15], far* median [3, 16]; no solve closed (LB 3
  or 4 on every seed). Mid* UB <= far* UB on 9/10 seeds (seed 1 is the
  exception, 17 against 15); the best mid* boards found cost 6 and 9 breaks
  (seeds 10 and 5), the best far* board 15.

## Kill criteria (self-assessment; a separate judge decides)

Thresholds, tests, both-ends rule and outcome rule are the prereg's (A6
changes only the distance they are computed on).

| criterion | numbers | verdict |
|---|---|---|
| K1a*, 5x5 | median B* = -3 (all closed) | **fires** |
| K1a*, 6x6 | median B* in [-7, -1] | **fires** (both ends <= 0) |
| K1b* | one-sided Wilcoxon on B*: p = 1.0 (unfavourable end), 1.6e-6 (favourable end); mean B* -3.2 (5x5), [-7.3, -0.23] (6x6) | undecided (ends disagree; far* open at 6x6) |
| K1c, 7x7 (d_0, A7) | far = free certificate (2 breaks on 9/10, 4 on 1/10); mid capped at it: INFEASIBLE 9/10, no board found 1/10; 0/10 mid <= far | decided, does not fire |
| K1c, 8x8 (d_0, A7) | certificate 2 on 9/10, 4 on 1/10; no capped mid board found in 300 s x 4 on 10/10; 0/10 | decided, does not fire |
| K1c*, 7x7 (d*, A9) | mid* UB <= far* UB on 9/10 seeds at equal budget (150 s x 4 each); medians [3, 15] against [3, 16]; no solve closed | **fires** (>= 3/10; certificates only, both bands open) |
| K2* | median over 30 6x6 instances of the K2* minimum: [5, 5] > 2 | decided, does not fire |
| K3* | b_ann(mid) = 5; median (H*_mid - b_ann) = -1 | decided, does not fire |

K1 fires through K1a at both exact sizes, so the outcome is killed
regardless of K1b*. K1c* fires as well, but it carries less weight: at
150 s both 7x7 bands stay open at about [3, 15], so it compares weak
upper bounds (as K1c allows), and 7 of its 9 counts are ties or near ties. The kill reads: on mu-matched 5x5 and 6x6
planted puzzles, the overlap-gap picture of a mid-band barrier in front of
a distinct low-break floor is false at small size. Boards at 25% to 75%
distance from every turned copy cost a median 4 breaks; boards far from
every copy cost more (median 7 at 5x5, at least 5 at 6x6). The far side is
not a floor of orthogonal near-solutions; it is the most expensive band.

Secondary, reported only (A6):
- H*_mid growth 5x5 to 6x6 (the judge's alternative K1b fix): one-sided
  Wilcoxon p = 0.13 (unfavourable end), 0.042 (favourable end). Mid* is
  identical to the first run's mid band at these sizes (median 4).
- The judge's K1' quantity H*_mid - max(H*_near, H*_far) has the same medians
  as B* (-3 at 5x5, [-7, -1] at 6x6), because far* >= near* everywhere.

## Secondary: the first run on d_0 (A1 to A5, symmetry-confounded)

These measure the planted solution against its own turned image. They do
not decide the outcome.

| palette | near | mid | far | K2 band | median B | closed mid / K2 |
|---|---|---|---|---|---|---|
| 5x5 C=13 | 2 | 4 | 2 | 5 | 1.5 | 30/30, 30/30 |
| 6x6 C=14 | 2 | [4, 4.5] | 2 | [5, 7] | 2 | 23/30, 13/30 |
| 5x5 C=9 | 2 | 2 | 2 | 2 | 0 | 10/10, 10/10 |
| 6x6 C=10 | 2 | 2 | 2 | 2 | 0 | 10/10, 10/10 |

- On d_0, K1a (median B 1.5 and 2), K1b (Wilcoxon p = 0.021 / 0.00068), K2
  (median m2 in [5, 7]) and K3 (median H_mid - b_ann in [-1, -0.5]) do not
  fire. The judge showed about 75% of the K1b growth is H_far falling (the
  corner-swap cost of the turned copy), and H_mid alone gives p = 0.19; A6
  replaces this reading.
- The hypothesis's ">= 2 breaks" barrier held on d_0 at 6x6 only (median B
  1.5 at 5x5). It was never a kill threshold (K1a fires at <= 0); on d* it
  fails at both sizes.
- Symmetry diagnostic (first run, not in the prereg): 29/30 (5x5) and 28/30
  (6x6) far witnesses lie within 4 cells of a turned planted copy.

## Other reported quantities

- **K2 band cap.** The K2 band ends at floor(3N/4) (pre-registered). The
  reason: the statement's "a >= n/3" has no upper end, and without the cap
  the 2-break turned copy in the far band fires K2 trivially. On d* the cap
  no longer matters for that reason, and K2* passes (median 5).
- **Annealed baseline (exact first-moment DP), near and mid bands only
  (A10).** 5x5 C=13: near 2, mid 5; 6x6 C=14: near 2, mid 5; 5x5 C=9: 1, 2;
  6x6 C=10: 1, 3. The count leaves out the three turned images, so its far
  value (7 at 5x5, 8 at 6x6) is not a floor prediction and is not reported
  as one. On d*, measured far* (7 at 5x5) is close to the first-moment far
  value, which is what a band free of symmetric copies should give.
- **Near radius r(b)** (first run curves, seeds 1..10, b = H_mid - 1 per
  seed). 5x5: median 4, all 10 seeds certain (2 to 6). 6x6: certain seeds
  (5/10) r = 2, 4, 4, 5, 6 (median 4); uncertain seeds r = 8, 9, 9, 16, 17
  (open curve points; two exceed N/3 = 12); all 10 together median 7. The
  radius at the solvers' typical plateau (hypothesis part 2 as stated) was
  not measured: stage 3 was not run (A8).
- **R(k) rotation caveat (A10).** The annealed DP counts a 2-fold symmetric
  interior piece turned by a half turn in its own cell as changed, CP-SAT
  does not. There is 1 such piece over the 30 5x5 C=13 instances and 5 over
  the 30 6x6 C=14 instances; not corrected.
- Zero rotation-duplicate pieces over all 140 generated instances.

## What is not tested

- Hypothesis part 3 (solver boards with few breaks sit on the far branch):
  stage 3 not run (A8).
- Hypothesis part 2 at a solver plateau: not measured (A8); the per-seed
  curve radius above is the only radius reported.
- K1c* at 8x8 (A9): not run.
- 6x6 far*: no solve closed at 120 s x 4 workers; the kill does not depend
  on it (K1a fires at the favourable end, -1).

## Files

- `src/lib.rs`, `src/bin/ncs.rs`: instances, annealed DP, stage-3 samplers.
- `scripts/cpsat_spectrum.py`: first-run CP-SAT model and schedule (d_0).
- `scripts/cpsat_quotient.py`: d* model, free certificates, fix-run schedule.
- `scripts/analyze.py`, `scripts/analyze_fix.py`: kill criteria (first run,
  fix run); `scripts/check_fix.py`: independent witness recheck.
- `scripts/run_all.sh`, `scripts/run_fix.sh`: recipes.
- `results/instances.jsonl`: every reported instance (pieces, p_f, p_i, mu).
- `results/raw/{stage1,curve,stage2,annealed}.jsonl`: first run.
- `results/raw/{star1,k1c,star7}.jsonl`: fix run (status, UB, LB, wall
  time, d_k of the witness, witness board as piece*4+rotation).
- `results/raw/wall.txt`, `results/raw/wall_fix.txt`,
  `results/summary.json`, `results/summary_fix.json`.
