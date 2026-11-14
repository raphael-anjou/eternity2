# five-clue-quota: preregistration

Night 2026-11-12, Track 15 (written 2026-11-13, about 04:15 local).
Label: **measurement**, on the real 16x16 Eternity II pieces (the owner allowed
real-piece measurements on 2026-11-12). Not a record attempt: this measures
search yield; no board is banked, and nothing here is Eternity II progress.
The optimization rule (identical node sequence) does not apply: no arm is an
optimization. The random-order control of the protocol (rule 3) is not run:
no arm claims a search mechanism against a fixed-order baseline; the readings
compare configurations of one engine on paired openings.

## Question

Track 14 (`../five-clue-keying/`, being judged; its numbers are the runner's
claims) reported that with the five official clues none of 680 keyings made
of one ring colour and two interior colours placed a legal node at depth 201
or deeper (1e9 nodes per run), while the port's default keying (22, 21, 20)
did. Its runner argued two causes, neither measured:

- (a) the early part of the quota curve prunes the search just past the clue
  at cell 45 (x 13, y 2), near depth 46 to 47;
- (b) the port's orientation defect (in the no-break path a piece may show
  grey toward an interior cell whose piece is not yet placed, and grey is then
  matched like a colour; header of `engine/src/bw.rs`) strands one-ring runs
  at depth 18 to 22.

This experiment separates the two.

## Fixed inputs

- Pieces: `data/official.csv`, a byte copy of `../five-clue-keying/data/official.csv`
  (official pieces with the five clues at cells 34, 45, 135, 210, 221).
- Engine: `engine/`, a copy of `../five-clue-keying/engine/` (binary renamed
  `fcq`) with two switches, both off by default:
  - `--orientfix`: in the no-break path, a candidate is refused when
    `wrong_class(pos, piece)` holds, that is when it shows grey on a side that
    does not face the rim or a non-grey colour on a side that faces the rim.
    This is the exact legality test the fidelity break path already applies
    (`--legalbreak`), now applied in the no-break path where the defect lives.
  - `--quotamode 0|2|3`, applied after `--quotascaled` (see quota curves).
- Every arm: fidelity engine of Tracks 13 and 14 (`--legalbreak
  --ringbreakoff --noisolation`), record unlock list 201, 206, 211, 216, 221,
  225, 229, 233, 237, 239, at most 10 breaks, top-left row scan without border
  interleaving, one attempt, **cap 1e9 nodes per run**.
- Node, depth, legality: as Track 13 (`hist[d]`, `hist_legal[d]`; a node is
  legal when every placed piece is class-correct and its parent is legal).

## Factors (3 x 2 x 4 = 24 arms, plus 2 supplementary arms)

- Keying: **D** = (22, 21, 20), three interior colours, 150 sides;
  **P** = (5, 20, 7), the published triple, 122 sides; **K1** = (1, 7, 9),
  Track 14's screening rank 1, 120 sides.
- Defect fix: **off** / **on** (`--orientfix`).
- Quota curve, q0 = the curve Track 14 used for that keying: for D the port's
  unscaled published curve (119 by cell 160); for P and K1 the published curve
  scaled by (T - 3) / 119 (`--quotascaled`; factor 1.0 for P, 0.983 for K1).
  - **Q0**: q0 as is.
  - **Q1**: no quota at all (`--noquota`).
  - **Q2**: q0 delayed by 48 cells: q2[i] = q0[i - 48] for i >= 48, 0 below.
    q0 is 0 up to cell 16, so q2 is 0 up to cell 64 and binds from cell 65,
    after rows 0 to 3, which hold the two clues nearest the scan start (cells
    34 and 45). The plateau value is reached at cell 208 instead of 160
    (`--quotamode 2`).
  - **Q3**: q0 with every threshold at a cell below 96 halved (integer floor),
    unchanged from cell 96 on (`--quotamode 3`).
- Supplementary (R1' only, not part of the 24): keying **R1** = (2, 9, 11),
  scaled, Q0, fix off and fix on. Reason: Track 14's confirmation rows show
  no stranded run for P or K1 (0 of 20 each) but 8 of 20 for (2, 9, 11), so
  R1 on the 24 arms may have nothing to explain.

Arm names: `<keying>-<off|on>-<Q0..Q3>`, e.g. `D-off-Q0`, plus `R1-off-Q0`,
`R1-on-Q0`. The reference arm is **D-off-Q0** (Track 14's only working
configuration).

## Openings and hosts

Seeds **501 to 516** (16 openings), never used by Tracks 13 and 14. Every arm
runs every opening (paired by seed). All arms of one seed run on the same
host: dell (12 threads) seeds 501 to 508, mac-m1 (7 threads) 509 to 513, this
Mac (4 threads) 514 to 516. Single-threaded runs, `nice -n 10`.

## Gates (draft rule P12), stage `gate`, before any test run

Gate rows use Track 14 confirmation seeds (not test seeds).

- **G-off**: fix off, Q0: `fcq` must reproduce Track 14's committed rows
  `conf-D-s401`, `conf-1-7-9-s401`, `conf-5-7-20-s401` exactly (`hist`,
  `hist_legal`, `nodes`, `nodes_legal`, `deepest`), same command lines. Each
  remote host reruns `conf-5-7-20-s401` before its first job. A failure stops
  everything.
- **G-on-unflagged**: fix on, Q0, the same three rows (none is flagged: legal
  nodes >= 100). Prediction, argued from the code: an illegal node only arises
  in the no-break path (the fidelity break path already refuses wrong-class
  pieces), illegality is inherited by every descendant, and no counter that
  steers the search (quota, breaks, used pieces) differs between a legal node
  and its illegal siblings' subtrees. So with the fix on, the node sequence
  is exactly the legal-node subsequence of the fix-off run, and a run differs
  from its fix-off twin whenever the fix-off run spent any node on an illegal
  board (all three rows did: 1 to 4% of nodes). Test of that prediction:
  rerun each row with the fix on and `--cap` set to the committed row's
  `nodes_legal`; its `hist` must equal the committed `hist_legal` exactly.
  Also run the fix-on rows at cap 1e9 and report how `hist`, deepest depth and
  legal late nodes differ from the committed rows. If the capped test fails,
  the fix changes legal search, which needs an explanation before any test run.
- **G-on-flagged**: fix on, Q0, three flagged Track 14 rows: `conf-D-s402`
  (4 legal nodes), `conf-1-8-9-s403` (4), `conf-2-9-11-s401` (2). Each must
  leave the opening (deepest > 60) or be explained (for example: the legal
  tree under that opening is genuinely empty past depth 60 within the cap).

## Calibration, stage `calib`

Before the test runs: the 26 arms on seed 1 on dell and on seed 2 on mac-m1
(non-test seeds). From the calibration seconds, projected wall time per host =
(sum of the host's calibration seconds over 26 arms) x (seeds assigned) /
(threads). If the largest projection puts the end of the test stage after
05:25 local, reductions apply in this order, each only if the previous one is
not enough: (1) drop keying K1 (8 arms); (2) reduce to 10 openings, seeds 501
to 510 (dell 501 to 506, mac-m1 507 to 510, this Mac none). The applied
reduction is written in PREREG_AMENDMENTS.md before the test stage starts.
Calibration rows are reported but never enter any statistic.

## Statistics (stage `test` rows only)

Per arm, over openings:

- Y_node = (sum of legal nodes at depth >= 201) / (sum of nodes spent);
  Y_sec = (same numerator) / (sum of seconds). Every node of every run is
  charged, including runs that never leave the opening.
- Stranded share = share of runs with `nodes_legal` < 100 (Track 14's flag).
- Deepest legal depth (largest d with `hist_legal[d]` > 0): median and range.
- Openings with any legal node at depth >= 201 ("positive openings").
- Per-seed values of all of the above in the analysis file.
- Ratios of each arm to D-off-Q0, per node and per second: pooled ratio of
  the arm's Y over the reference's Y, with a 95% percentile bootstrap over
  openings (10,000 paired resamples, `random.Random(20261113)`). A resample
  where both are 0 counts as ratio 1; reference 0 and arm positive counts as
  infinity.

## Readings (fixed now, not changed after any test run)

Scope sentence for every reading: "at 1e9 nodes per run, one attempt per
opening, on this port, with a row scan from the top-left corner without border
interleaving".

- **R1** (defect and stranding): stranded share pooled over the one-ring arms
  of the 24 (P and K1, all four quotas), fix off against fix on. "The defect
  explains the stranding" if the fix-off share is above 15% and the fix-on
  share is below 5%. If the fix-off share is 15% or less, R1 reads "no
  stranding to explain in these arms", and the supplementary R1': the same
  rule on R1-off-Q0 against R1-on-Q0 (16 runs each).
- **R2** (early quota): "the early quota explains the dead one-ring family" if,
  with the fix on, some one-ring arm under Q1, Q2 or Q3 has a legal node at
  depth >= 201 on at least 8 of 16 openings (half) while its Q0 twin (same
  keying, fix on, Q0) has none on any opening. Otherwise not supported.
- **R3** (does a one-ring configuration work): a one-ring configuration
  "works for five clues" only if its Y_sec ratio to D-off-Q0 is at least 1
  with the bootstrap interval's lower end above 1. Otherwise "the default
  remains the best configuration measured" (unless another D arm meets the
  same rule, which is then reported by name).
- **R4**: if no one-ring arm with the fix on places a legal node at depth
  >= 201 under any quota, the reading is: "the one-ring family is dead on this
  port for five clues for a reason not identified here".

Each statement in the README is graded measured, reported or argued.

## Budget

26 arms x 16 openings = 416 runs. Track 14's five-clue rows took 12 to 90 s
per 1e9 nodes when the break path is rarely reached; runs that live in the
break path (Q1 may) can cost more, which the calibration measures. Ceiling:
40 thread-hours (dell 12, mac-m1 7, this Mac 4 threads). Report due 05:50.

## Procedure

Every real-piece command (gate, calib, test) goes through
`python3 ../common/prereg_guard.py . <stage> -- <command>`, launched from
`scripts/one.sh`. Remote hosts get the committed tree by `git archive`
excluding `results/`; their guard logs are fetched to
`results/run-log-<host>.jsonl`, raw rows to `results/raw/`. The analysis is
`scripts/analyze.py` writing `results/analysis.txt`.
