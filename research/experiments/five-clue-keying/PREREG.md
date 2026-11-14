# five-clue-keying: preregistration

Night 2026-11-12, Track 14 (written 2026-11-13, about 02:50 local).
Label: **measurement**, on the real 16x16 Eternity II pieces (the owner allowed
real-piece measurements on 2026-11-12). Not a record attempt: this measures
search yield; no board is banked as a result, and nothing here is Eternity II
progress. The optimization rule (identical node sequence) does not apply: no
arm is an optimization.

Question: with the five official clues on the board, is the colour keying
(the privileged colours whose sides the quota counts) a lever on how much
break-free and break-admitting legal search reaches depth 201 and beyond, and
if so which keying?

Origin: Track 13 (`../quota-keying/`, branch at 55bd52e, being judged; its
numbers are the runner's claims) reported that with five clues the published
keying (5, 20, 7) made no arrival at depth 201 in 10 runs of 1e9 nodes with
the fidelity engine (`c5-P-bwf`, deepest 191), while the port's default
keying (22, 21, 20) made 135 arrivals with the plain engine (`c5-D`).

## Fixed inputs

- Pieces: `data/official.csv`, a byte copy of `../quota-keying/data/official.csv`
  (the official piece file with the five clues). Colours in this repo's
  numbering: ring (frame) colours 1..5 (24 sides each), interior colours 6..10
  (48 sides each) and 11..22 (50 sides each).
- Engine: `engine/`, a copy of `../quota-keying/engine/` (binary renamed
  `fck`) with one added switch, off by default: `--quotascaled`, which
  multiplies the whole published quota curve by (T - 3) / 119, where T is the
  total number of sides of the three privileged colours. The published curve
  reaches 119 at cell 160 and Blackwood's triple has T = 122, so his engine
  leaves at most 3 privileged sides off the board after cell 160; the rule
  keeps "T minus 3 by cell 160" for every triple and the same shape before.
  This scaling rule is **a choice** (others are possible: a shift, or no
  scaling). For T = 122 the factor is exactly 1.0, so the published triple's
  tree is unchanged. For T = 120 and 124 the quota at cell 160 is 117 and 121.
- Every arm uses the **fidelity engine** of Track 13: `--legalbreak
  --ringbreakoff --noisolation` (class-correct break path, no mismatch on a
  ring-coloured side, no isolation rule), record unlock list 201, 206, 211,
  216, 221, 225, 229, 233, 237, 239, at most 10 breaks, top-left row scan,
  one attempt.
- Node, depth, legality: as Track 13 (`hist[d]`, `hist_legal[d]`; a node is
  legal when every placed piece shows grey exactly on the rim sides of its
  cell and its parent is legal).

## Keyings

- The 680 candidate keyings: every triple (r, a, b) of one ring colour r in
  1..5 and two interior colours a < b in 6..22, each with `--quotascaled`.
  The published triple (5, 20, 7) is one of them (T = 122, factor 1.0).
- Default D: (22, 21, 20), three interior colours (T = 150), run with the
  port's own unscaled quota (119 by cell 160), as the port and Track 13 run
  it. It is not one of the 680 and is not scaled (scaling it would ask for
  147 of 150 sides by cell 160, a different keying from the one Track 13
  measured).
- Random control R1..R5, drawn now from the 680 by
  `random.Random(20261113).sample(range(680), 5)` over the list in the order
  r, then a, then b ascending: **(2, 9, 11), (1, 7, 10), (1, 20, 21),
  (3, 14, 15), (5, 15, 16)**. If one of them is also selected by the
  screening or is the published triple, it is run once and counted in each
  role.

## Port gate (draft rule P12), stage `gate`, before any other run

The `fck` binary (every new switch off) must reproduce Track 13's committed
fidelity-engine rows exactly (`hist`, `hist_legal`, `nodes`, `deepest`) on
three (keying, seed, clue) rows: `c5-P-bwf-s101` (five clues), `c1-P-bwf-s102`
(centre clue), `nc-P-bwf-s101` (no clue), same command lines. Also
`c5-P-bwf-s101` with `--quotascaled` added must give the same row (factor
1.0). Each remote machine reruns `c5-P-bwf-s101` before its first job.
If any gate fails, nothing else runs and the failure is reported.

## Stage 1: screening (stage `screen`)

Openings: seeds **1, 2, 3** (calibration seeds; Track 13 used 1 and 2 only to
calibrate, never as test seeds). Budget: **cap 1e9 nodes per run** (Track 13's
committed five-clue rows took 12 to 62 s per 1e9 nodes; at 1e8 only 3 of 10
default runs had any node past depth 200, so 1e8 would rank mostly zeros).
Runs: 680 + D = 681 keyings x 3 openings = 2,043 runs. Order: opening-major
(every keying on seed 1, then seed 2, then seed 3), keyings within an opening
in a fixed shuffled order (`random.Random(1)`), split across machines by
`scripts/jobs.py`. Every node is charged; runs that never leave the opening
(deepest <= 60) and runs that exhaust their tree before the cap are kept with
the nodes they spent.

Screening statistic per keying k: Ys(k) = (sum over screening openings of
legal nodes at depth >= 201) / (sum over the same openings of all nodes
spent). Secondary: deepest legal depth (max over openings). Ranking over the
680: Ys descending, ties broken by deepest legal depth descending, then by
list index ascending. The **top 5** of the 680 are the screened arms; the
**best screened keying** is rank 1. D is ranked in the same table for report
only.

Reduction, fixed now: the screening stops at **04:30 local** whatever is
running. Ys is then computed over the openings that are complete for all 681
keyings (seed 1 alone, or seeds 1 and 2, or all three); runs of an incomplete
opening are listed and not used.

Winner's curse: choosing 5 of 680 by a noisy statistic inflates their
screening scores. The screening numbers are reported, but **only the
confirmation numbers count** for the reading below.

## Stage 2: confirmation (stage `confirm`)

Held-out openings: seeds **401..420** (never used in screening, never used by
Track 13). Arms: top 5 screened, D, P = (5, 20, 7), R1..R5 (12 or fewer
distinct keyings after overlaps; at most 13). Cap **1e9 nodes per run**, every
node charged, runs paired by seed across arms. Order: opening-major (all arms
on 401, then 402, ...).

Statistic per arm: Y = (sum over openings of legal nodes at depth >= 201) /
(sum over openings of all nodes spent). Per-seed values y_o = lateL_o / N_o
are reported with min, median and max. Ratios:
- Y(arm) / Y(D) for every arm.
- Y(best screened) / max over R1..R5 of Y(R_i) ("best random key"; the max is
  taken inside each bootstrap resample, so the control is also selected on
  the data it is judged on, which favours the control).
95% intervals: percentile bootstrap over openings, 10,000 paired resamples,
seed 20261113. In a resample, numerator and denominator both 0 gives ratio 1;
numerator > 0 and denominator 0 gives +infinity (kept).

Reduction, fixed now: the confirmation stops at **05:40 local** whatever is
running; it is then analysed on the openings complete for every arm, taken in
seed order 401, 402, ... (an opening counts only if all arms finished it).

## Reading (written before any run, not changed after)

Let G_D = Y(best screened) / Y(D) and G_R = Y(best screened) / best random,
with their 95% intervals.
- **Keying is a lever for five clues** if G_D >= 3 and G_R >= 3, and the
  lower end of both intervals is above 1.5.
- **Keying is not a lever** if the interval of G_R includes 1.
- Anything else: **undecided at this budget**.
Uninformative (stated as such, reading still given literally) if fewer than
10 confirmation openings are complete, or if no arm has any legal node at
depth >= 201.

## Descriptive only (no threshold)

Per arm: deepest legal depth, legal nodes at depth >= 230, arrivals
(`hist[201]`), runs that never leave the opening, runs with no node past
depth 200. Whether any five-clue arm gets further than Track 13's centre-clue
fidelity runs (`c1-P-bwf`, same 1e9 budget: deepest legal 238 to 246, Y
0.094, Track 13's numbers). Secondary comparison, no reading: best of the top 5
in confirmation against the best random key (both selected on confirmation
data). Screening rank of D and of P.

## Compute

dell (12 threads), mac-m1 (7), this Mac (4), each run single-threaded under
`nice`. Estimate: screening 2,043 runs at 15 to 60 s, 9 to 34 thread-hours;
confirmation at most 260 runs at up to 100 s, at most 7 thread-hours; gates
under 0.1. Cap 60 thread-hours. Remote code shipped by `git archive` of the
committed tree. Every run goes through `../common/prereg_guard.py`.

## Baselines and controls

Baseline: D (the port's default keying, what Track 13's 135 five-clue
arrivals came from) and P (the published keying). Control for "any keying
would do": R1..R5. The ladder rules (bb-mrv, scb) do not apply: this is a
measurement on the real board, not a ladder comparison.

## What this cannot show

No board is finished; depth 201 is 55 cells from the end. The port differs
from Blackwood's engine in scan corner, border interspersing, restarts and
engineering. One budget (1e9 nodes) per run. Nothing here bears on a full
solution; at most it bears on how to configure a record-style engine for the
five-clue board.
