# PREREG: prefix-statistics (arc 2026-11-13, Track D4)

Label: **measurement** (NIGHT_BRIEF track label). No search method is compared
against a baseline, no board is produced, nothing here is Eternity II
progress. Generated boards only, plus the official pieces as a comparison
population. Branch `night/2026-11-13-d4-prefix-statistics`.

## 1. Question and hypothesis

O2 in `research/program/night/2026-11-13/THESIS_PLAN.md` (question RQ3): is
there a cheap statistic of a partial row-scan board whose value on the
planted solution's prefix is shifted, in standard deviations of its value
over the consistent prefixes of the same depth, by enough to feed a pruning
rule with rerandomised scans (extreme pruning, survey
`cryptanalysis-coding.md` candidate T1)?

Claim under test (from the lead): **no cheap statistic has |delta| >= 1.4 at
the depths where the tree is widest.** The hypothesis "a usable tilt exists"
is killed by the line in section 7.

ROUTES rows this touches (`research/program/night/2026-11-13/ROUTES.md`):
"Value order by propagation impact (A21)" (killed, section 1); "ProbCut
statistical cutoffs (A12)" (killed, section 1); "Cheap information about the
frame" (killed for local tests; the only measured shift, z = 1.40 on whole
frames, section 3); "Planted-solution recovery by spectral pair statistics
(A1)" (killed, section 6). No row exists for pruning with a deliberate low
success rate plus rerandomisation, nor for Selby-style goodness in strict
search, nor for any statistic measured on row-scan prefixes at the peak.

## 2. Inputs and code

- Sampler: Track 10's population SMC (`real-puzzle-profile`, branch
  `night/2026-11-12-t10-real-puzzle-profile`, files `engine/src/puzzle.rs` and
  `engine/src/tree.rs` at commit `dc71ad56567570aecdcb75e474f163e5bba426e2`),
  imported into `engine/src/`. Additions (marked in the source): a tile id
  per cell in `State`, a callback form `smc_with` that exposes each
  population with its parent indices and width estimate, subtree counting,
  the four scan orders. The callback cannot touch the RNG or the
  population.
- Smoke check, done before this file: on the official pieces, five clues,
  row scan, N = 1e5, seeds 201 and 202, the imported binary returns totals
  3.233816e40 and 2.590839e40 and extinction depths 191 and 190, identical to
  Track 10's committed `results/m2/five-np1e5.jsonl`; repeated after the
  callback change (seed 201, identical). At N = 1e4, five seeds: log10 of the
  mean total 40.50 (Track 10, N = 1e6: 40.457, interval 40.440 to 40.473).
- Boards: the repo engine's framed generator,
  `generate_solved_framed_bc(16, 22, seed, framed = true, border = 5)`: 5
  frame colours on the 60 along-frame joints (12 each) and 17 interior colours
  on the other 420 joints (24 or 25 each), the official census structure (the
  official set has 24 sides of each frame colour and 48 or 50 of each
  interior colour). Tile ids and rotations are scrambled with a seeded
  permutation (`planted()` in `main.rs`); the planted placement is kept for
  the true prefix only.
- Clues: the five official clue cells (row-major positions 135, 210, 34, 221,
  45), holding the planted tile in its planted rotation. Clues are fixed
  cells, not nodes (Track 10's convention).
- Official pieces: `research/experiments/single-core-benchmark/engine/data/puzzles/size_16_official_eternity.csv`
  with its five clues, for the comparison population only.

## 3. Scan, depth, populations

- Scan directions: dir 0 = rows from row 15 up to row 0, columns left to right
  (Track 10's order); dirs 1, 2, 3 = its images under quarter turns of the
  board. Clue cells are removed from the order.
- Depth k = number of free cells placed. Target depths: 32, 64 (early); 96,
  128 (rise); **144, 160, 176 (peak region; the widest level of the official
  tree is free depth 158)**; 192, 208, 224 (late).
- True prefix at depth k: the planted tiles on the first k free cells of the
  scan. The code replays it through the tree and stops with an error if any
  step is not a valid child (pipeline check).
- Sampled prefixes: one SMC repetition per (board, direction), population cap
  N = 1e5, SMC seed 1 + 1000 x board + dir. In this sampler every particle of
  a level carries the same weight (width estimate / population size), so the
  weighted mean over the population is its plain mean. At each target depth
  m = 1,000 particles are drawn uniformly without replacement (the whole
  population if smaller), plus one decoy drawn from the rest (section 8).
- Work weight of a sampled prefix x: the summed SMC weight of x's descendants
  at every depth from k to extinction (x itself included). It estimates, up
  to a factor common to the depth, the number of nodes in x's subtree: the
  work a search spends below x. Its effective sample size (sum w)^2 / sum w^2
  is recorded per row.
- The pilot (section 11) found the population extinct before depth 192 in 20
  of 24 (board, direction) runs and in all runs before 208. Rows at an
  extinct depth are recorded as extinct; no delta is computed there. **Depths
  208 and 224 are expected to be unmeasurable with the validated sampler**;
  this is a stated departure from the lead's design (section 8, positive
  control).

## 4. Statistics (fixed before any test board is generated)

Notation: E = empty cells; known cells = placed and clue cells; for an empty
cell p, the unary requirement is grey on rim sides, non-grey on other sides,
and the facing colour on sides facing a known cell; a candidate is an unused
(tile, rotation) meeting it. Frontier F = empty cells with at least one known
non-rim neighbour. Counts enter as log2(1 + count).

| # | Name | Definition | Lookahead | Class |
|---|---|---|---|---|
| S1 | next fits | log2(1 + candidates at the next scan cell) (the tree's child count) | 1 cell | search-equivalent |
| S2 | frontier fits | sum over F of log2(1 + candidates) | 1 cell, each frontier cell alone | search-equivalent |
| S3 | Hall surplus | min over nonempty S subset of F of N(S) - S, N(S) the unused tiles with a candidate rotation at some cell of S (tile level); computed per v in F as max flow with v uncapacitated minus F; truncated at 16 | 1 cell, joint over F | search-equivalent |
| S4 | colour balance | for every colour c, x_c = (r_c - f_c)/2 remaining internal joints of colour c (r_c sides of colour c on unused tiles, f_c open sides facing a known cell showing c); chi-square of x_c against J_band x pi_c (J_band = remaining empty-empty joints in c's band, frame or interior; pi_c = c's share of its band on the whole tile set). Parity: x_c is an integer on every consistent prefix by conservation, so parity carries no information; negativity is folded into the chi-square | none | census |
| S5 | goodness of used tiles | mean over placed non-clue tiles of z(t); g(t) = ln(1 + number of exact fills of a 2x2 window of t's type (interior window at the board centre, edge window on the top edge, corner window) by the whole tile set that contain t); z = g standardised within type over this board's own tiles. No solution information; no fitted parameter (so "fitted on calibration boards only" holds trivially) | none | census |
| S6 | first moment | configuration-model log expected completions: sum over colours of ln(r_c! / (r_c - f_c)!) + ln((2 x_c - 1)!!); every term common to all prefixes of a depth dropped; -1e6 if some x_c < 0 | none | census |
| S7 | AC domains | arc consistency over all empty cells (values = candidates; support = a value of the empty neighbour with the matching facing colour; tile uniqueness ignored) to a fixed point; sum over E of log2(1 + domain size). A wiped domain empties the whole connected region, so S7 is near-binary (consistent or not) | propagation to a fixed point, polynomial, no branching | search-equivalent |
| S8 | block fills | log2(1 + number of exact fills of the next 4 scan cells) | 4 cells | search-equivalent |
| S9 | colour entropy | Shannon entropy (bits) of the non-grey side colours of unused tiles | none | census |
| S10 | row lookahead (own choice) | log2(1 + number of descendants 16 scan cells ahead), depth-first, stopping at 1e5 visited nodes (the count reached so far is used) | 16 cells, node cap | search-equivalent |
| H0 | hash (negative control) | splitmix hash of the placed (cell, tile) set, uniform in [0, 1) | none | control |

Why S10: the true prefix has a completion, so it has a descendant n cells
ahead; most sampled prefixes have none. It is the bounded-lookahead
statistic with the best chance of separating among those affordable at
every depth, and the one the positive control needs. Its separation is expected to be paid for by its own search
(section 7, reading 2).

Cost: nanoseconds per evaluation are measured per statistic and depth and
reported next to every delta (pilot means at depth 144: S6 30 ns, S1 320,
S5 520, S8 1.4 us, S10 1.8 us (0.3 us at 176), S9 2.4 us, S4 2.9 us, S3 16
us, S2 56 us, S7 0.77 ms; one core of this Mac, `nice`, another track
running).

## 5. Boards, seeds, sizes

- Test boards: seeds 1113001 to 1113096 (B = 96), all four directions each.
- Calibration boards: 7001 to 7006 (pilot, already seen, section 11) and 7002,
  7003 for the replicate gate; 6x6 boards 9001 to 9010 for the exact gate.
  Disjoint from the test seeds.
- Power arithmetic. Decision family: 10 statistics x 3 peak depths = 30
  tests, Bonferroni, two-sided alpha 0.05, so per test 0.05/30 = 0.00167 and
  z = 3.144. Power 0.9 against a true mean delta of 1.4 when the interval
  must exclude 0.7: z_0.9 = 1.282. B = ((3.144 + 1.282) x sigma / (1.4 - 0.7))^2
  = 39.97 sigma^2. sigma = the board-to-board sd of the per-board delta (mean
  over directions). Pilot, 6 boards, statistics not saturated at the clip: 0.3
  to 1.54 (S9 at 176). With sigma = 1.55: B = 96. A bootstrap interval is a
  little wider than the normal one at this level; B = 96 is kept as the
  preregistered size.

## 6. Estimands

For board b, direction r, depth k, statistic s:

- **Node delta** d(b,r,k,s) = (s(true) - mean over the m sampled) / sd over the
  m sampled (sd with divisor m). If sd = 0: d = 0 when s(true) equals the
  common value, else +-10 with the sign of the difference. Every d is clipped
  to [-10, 10].
- **Work delta** dW: the same with the work-weighted mean and sd of the m
  sampled; computed only for rows with work ESS >= 10.
- Per-board value: the mean over the directions available at that depth.
  Reported: the mean over boards and its percentile bootstrap interval
  (100,000 resamples of boards, seed 20261113, the same resampled board sets
  for every cell; numpy through `uv run --with numpy`). Decision cells (peak depths)
  use the level 1 - 0.05/30 = 0.99833; descriptive cells at other depths use
  1 - 0.05/100.
- Also reported per row: the true prefix's percentile in the sampled
  population (fraction below plus half the ties), node-uniform and
  work-weighted.
- **Empirical single-presentation gain** (descriptive, distribution-free):
  for a statistic and depth, orient it by the sign of its mean delta; keep the
  fraction f of the work-weighted reference in that tail; p = share of
  (board, direction) rows whose true prefix falls in the kept tail (work
  percentile <= f, or >= 1 - f). Gain p / f for f in {0.5, 0.2, 0.1, 0.05,
  0.01}, with a Wilson interval on p.
- Direction dependence: Pearson correlation of d between the scan directions
  of the same board (all 6 pairs pooled), and of the indicator "true prefix
  in the kept tail at f = 0.1".

## 7. Readings

- **Reading 1 (the lead's line, decision).** At the peak depths 144, 160,
  176, statistic s counts as a **signal** at depth k if |mean node delta| >=
  1.4 and the 0.99833 bootstrap interval excludes 0.7 on the same side
  (lower end > 0.7 for a positive mean, upper end < -0.7 for a negative one).
- **Kill line (fixed by the lead).** The hypothesis "a usable tilt exists" is
  killed if no statistic is a signal at any peak depth. If one is, the report
  gives its delta, its cost, and its correlation across the four scan
  directions.
- **Reading 2 (usability, added here, decision for the word "usable").** The
  gain formula assumes the kept nodes cost as much below them as the dropped
  ones. A statistic correlated with subtree size (every lookahead statistic,
  and possibly the census ones) shows a node delta that its own search pays
  for. Reading 2 applies the reading-1 line to the work delta dW. A statistic
  that passes reading 1 and fails reading 2 is reported as "separation paid
  for by the search", not a usable tilt.
- **Class.** A search-equivalent statistic (S1, S2, S3, S7, S8, S10) that
  passes both readings is a measured propagation or lookahead gain (it reads
  constraints, it does not tilt a statistical model), to be priced as such
  (its cost per evaluation against the work it removes); it does not
  falsify O2 as stated in THESIS_PLAN.md. A census statistic (S4, S5, S6, S9)
  that passes both readings is the candidate falsifier of O2.
- Derived, not measured: for any statistic passing both readings, the
  survey's formula G = log2((1/K) / Q(t)), Q(t - dW) = 1/K, at K = 4 with the
  measured direction correlation stated beside it.
- Descriptive only: every depth outside the peak, every reading at 192 and
  later, all S/H0 tables at the 1 - 0.05/100 level.

## 8. Controls

- **Positive control (required by the lead, depth moved).** S10 and S8 node
  delta at depth 176 (the deepest depth with a live population in at least
  90% of pilot rows): mean >= 1.4 with the 0.99833 interval above 0.7. If
  neither passes, the pipeline is declared broken and no reading is made. The
  lead asked for 208 and 224; the validated sampler is extinct there (section
  3), so those depths are reported as unmeasurable, not as a control.
- **Negative control 1, hash.** H0 at each peak depth: the 0.99833 interval
  of its mean node delta contains 0. A failure suspends the readings.
- **Negative control 2, decoy.** At each depth a decoy prefix is drawn from
  the population (not from the m sampled), converted to a placement list and
  replayed through exactly the code path of the true prefix; its delta is
  computed like the true prefix's. For each of the 10 statistics at each peak
  depth the decoy's 0.99833 interval must contain 0. A failure suspends the
  readings. This tests the replay path and the reference computation.
- **The lead's "true prefix of a different planted board with the same tiles
  relabelled".** As written it is not constructible: the planted prefix of
  another board is not a node of this board's tree (different tiles), and a
  board relabelled from this one has the same solution. The decoy above is
  the implemented substitute. **Flagged for the design reviewer.**
- **Null in reverse.** The official pieces, five clues, four directions, N =
  1e5, m = 1,000: population mean of every statistic per depth, compared with
  the spread of population means over the 96 test boards (z per statistic
  and depth). Reported; a statistic with |z| > 3 at a peak depth is flagged as
  distributed differently on the real pieces, which limits any transfer of
  its reading to the real puzzle.

## 9. Gates (calibration only, before the first test run)

- **G-exact (small boards).** 6x6 framed boards 9001 to 9010, 9 colours (2
  frame), no clue, row scan, exact enumeration of the whole tree computing
  every statistic on every node at depths 5, 9, 14, 18, 23, 27, node-uniform
  and work-weighted; SMC with N = 500, 20 repetitions, m = 500. For each
  (board, depth, statistic) with exact sd > 0 and a live population in at
  least 15 repetitions: bias = (mean over repetitions of the SMC mean - exact
  mean) / exact sd; sd ratio = mean SMC sd / exact sd. Pass: |bias| <= 0.2 in
  at least 90% of cells and sd ratio in [0.8, 1.25] in at least 90% of cells
  (node-uniform). Work-weighted bias reported, not gated. The exact delta of
  the planted prefix on these boards is reported as an SMC-free check of the
  sign of any 16x16 effect (descriptive).
- **G-rep (16x16 replicates).** Boards 7002 and 7003, dir 0, depths 96 to
  176: SMC at N = 1e5 (3 seeds) and N = 1e6 (1 seed); N = 1e4 (3 seeds)
  descriptive. Pass: at the peak depths, the node delta at N = 1e6 is within
  0.5 of the mean of the three N = 1e5 deltas, in at least 90% of the
  (board, depth, statistic) cells where both sds are > 0. On board 7001
  (pilot) this held; e.g. S9 at 176: mean -1.21 (range -1.42 to -0.79) over
  six seeds at N = 1e5, -1.26 and -1.11 at N = 1e6. At N = 1e4 the population
  at 176 is 150 to 570 particles and the S9 delta moves to -1.55 to -2.02,
  which is why N = 1e4 is not gated and the test uses 1e5.
- If a gate fails, the failure is reported, the design is amended in
  PREREG_AMENDMENTS.md before any test run, or the experiment stops.

## 10. Caps, budget, resumability

- At most 2 processes at a time, `nice -n 10`, single-threaded each.
- One output file per test board (`results/test/board-<seed>.jsonl`, one line
  per (direction, depth) plus a final `done` line). A file without its `done`
  line is rerun from scratch; finished boards are skipped. Rows are flushed
  as written.
- Budget: pilot cost about 2 core-minutes per board (four directions), so
  about 3.2 core-hours for the test set, 0.3 for gates and the official
  pieces; aim 4 core-hours. Hard stop: if after 48 test boards the projected
  total exceeds 8 core-hours, stop at the boards finished, report the
  reduced power and the sigma it implies. Never above 10 core-hours.
- Every test-stage and gate command runs through
  `research/experiments/common/prereg_guard.py` (from `run.sh`).

## 11. Pilot (disclosed)

Before this file was written, the pipeline was run on calibration boards
7001 to 7006 (four directions, m = 1,000) and on board 7001 dir 0 at N = 1e4,
1e5, 1e6 (11 runs), committed under `results/pilot/` and `results/pilot-rep/`.
They set the cost figures, sigma, the extinction finding and the work
reading. On those 6 boards S4, S6, S7, S8, S9, S10 (all three peak depths)
and S3 (176) already pass reading 1; S4, S6, S9 (all three) and S7, S10 (160,
176) also pass reading 2. This is why reading 2 and the class split exist,
and why the test set is large: the pilot is not evidence, the test set is.

## 12. What this cannot show

- Generated boards with the official colour census, not the official puzzle;
  the comparison population on the official pieces only checks that the
  statistics are distributed alike.
- Row scans only (four directions); five official clues; strict matching.
- The reference population is the validated SMC's, not exact; G-exact checks
  it on 6x6, G-rep checks its stability on 16x16; neither proves it at 16x16.
- Late depths (192 and beyond) are not measured.
- A pass is a statement about one-presentation tilt and its correlation
  across four directions; the bits a pruned search would gain are derived
  (survey formula, unrefereed), not measured.
