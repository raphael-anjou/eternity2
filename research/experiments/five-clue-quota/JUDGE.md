# five-clue-quota: judgment

Judge: independent session, 2026-11-13, branch `night/2026-11-12-t15-five-clue-quota` at 7fb856e9.
Read the branch only (not the runner's conversation).

**Verdict: keep**, as a measurement, with three README wording fixes (W1 to W3
below) before anything from it reaches the site. The preregistered readings
R1 to R5 are applied correctly and the raw rows support them. The one
headline that changes how later work is configured, R5 ("slack explains the
result"), is clean: 0 of 16 against 14 of 16 positive openings, on paired
seeds, with the arms committed before the first test run.

## 1. Preregistration order (measured)

- `PREREG.md` committed alone in 7bb763a5 at 04:00:53 +01:00 (03:00:53 UTC);
  `git log --follow` shows no later change; the working file hashes to the
  committed blob.
- Guard logs: 584 start lines, 584 end lines, all paired, all exit 0, every
  line carries PREREG sha256 `a3510a90...`. First gate line 03:01:50 UTC,
  first calib line 03:02:19 UTC, first test line 03:05:42 UTC (this Mac),
  last test line 03:21:06 UTC. All after the PREREG commit.
- Amendment 2 (deconfounding arms, R5, `--quotaslack`) committed in bdcf9b55
  at 03:05:13 UTC, 29 s before the first test line. Every local test line has
  `head` bdcf9b55. The remote hosts run copied trees (no git check); their
  amendment-2 gate rerun (`gate1-*`, 03:05:41 and 03:05:42 UTC) reproduces
  Track 14 exactly, and their QS/QM rows carry `qscale` values only the
  amendment-2 binary can produce.
- Every committed raw row (test, calib, gate, `old-gate1`) has a matching
  guard start line (label, seed, stage). No test row without a log line.
- 512 test rows = 32 arms x 16 seeds, every row at exactly 1e9 nodes; no seed
  dropped, no reduction applied.

Findings (minor, not invalidating):
- F-a: 92 of the 96 local test lines record `dirty_paths: 1`. The engine,
  data, jobs.py and one.sh are byte-identical between bdcf9b55 and 7fb856e9
  (`git diff`): only README, analyze.py, job TSVs and the tracked
  `gate-local.log` (deleted in 7fb856e9) changed, so the code that ran is the
  committed code. The dirty file was most likely `gate-local.log` or
  analyze.py being edited; the guard does not say which.
- F-b: PREREG.md says "written ... about 04:15 local" but was committed at
  04:00:53. A typo in the header; the commit time is what counts.
- F-c: the R5 code in analyze.py was written after the test rows; I checked
  it against the amendment text and it implements the rule as written.

## 1b. Track label

Measurement, not optimization: no node-sequence identity is claimed, so no
hash check applies. No arm claims an ordering mechanism, so the missing
random-order control (protocol rule 3) is not a defect here.

## 2-3. Recomputed numbers (own script, `results/raw/test-*.jsonl`)

Every row: `sum(hist) == nodes`, `sum(hist_legal) == nodes_legal`,
`hist >= hist_legal` elementwise. My numbers match the README table to its
printed precision. Selected (pos = openings with a legal node at depth >= 201,
ratio = pooled Y per second against D-off-Q0, my bootstrap with a different
RNG seed):

| arm | pos /16 | legal late nodes | Y per s | ratio per s [95%] |
|---|---|---|---|---|
| D-off-Q0 | 12 | 9,095 | 16.3 | 1 |
| D-on-Q0 | 13 | 9,283 | 16.0 | 0.983 [0.95, 1.04] |
| D-off-QS / D-on-QS | 0 / 0 | 0 | 0 | 0 |
| P-off-QM / P-on-QM | 14 / 14 | 6,527 | 10.0 | 0.614 [0.29, 1.54] / 0.615 [0.27, 1.58] |
| K1-off-QM / K1-on-QM | 12 / 13 | 7,643 / 7,843 | 12.0 / 12.3 | 0.738 [0.29, 2.15] / 0.753 [0.29, 2.24] |
| P-off-Q0 | 2 | 84 | 0.19 | 0.012 |
| K1-off-Q0 | 1 | 388 | 0.68 | 0.042 |
| P-on-Q1 / K1-on-Q1 | 9 / 9 | 1,294 / 2,867 | 2.0 / 4.3 | 0.121 / 0.261 |
| Q2 one-ring, all Q3 | 0 | 0 | 0 | 0 |

Also confirmed: deepest legal depth of any run 220; 233 of 233 non-stranded
fix-off/fix-on pairs have identical legal late counts; stranded fix-off runs
first go illegal at depth 1 (3), 3 (1), 5 (6), 7 (3), 9 (3), 10 (4), 11 (3);
test compute 5.06 thread-hours.

Gates, recomputed against Track 14's committed rows
(`../five-clue-keying/results/raw/conf-*-s401.jsonl`): `gate-off-*` and
`gate2-off-*` match on hist, hist_legal, nodes, nodes_legal, deepest for D,
1-7-9, 5-7-20; both remote `gate1-*` rows match; all three `gate-oncap-*`
rows have `hist` equal to the Track 14 row's `hist_legal` with nodes equal to
its `nodes_legal`. The capped test of the "legal-node subsequence" prediction
holds.

Quota curves (`--printquota`, fresh build from 7fb856e9): D unscaled reaches
119 by cell 160; D with `--quotascaled` 147; P with `--quotascaled` 119
(identical, cell by cell, to D unscaled); P with `--quotaslack` 96; K1 95. Q3
steps from 50 at cell 95 to 102 at cell 96 for D, which confirms the
README's "Q3 is a wall" observation.

Rerun: see the section "Rerun" at the end.

## 4. Adversarial checks

- Readings applied literally: R1, R2, R3, R4, R5 all correct as stated, with
  one pooling slip:
  - F-d: R1 is preregistered over "the one-ring arms of the 24 (P and K1, all
    four quotas)", i.e. 8 arms, 128 runs. analyze.py pools all P and K1 arms
    present, which after amendment 2 includes QM: 4/160 = 2.5% instead of
    3/128 = 2.3%. The reading ("no stranding to explain", threshold 15%) is
    unchanged. Fix: restrict the R1 pool to Q0..Q3 in analyze.py and the
    README.
- Equal node accounting: every run is charged 1e9 nodes; stranded runs count
  in the denominator. Wall-clock: all arms of a seed ran on the same host, so
  pooled per-second ratios are paired; QM arms are about 15% slower per node
  than D-off-Q0 and the per-second ratio already charges that.
- Censoring: no run completes; all are capped; nothing counted as a win by
  censoring.
- Planted solution: not applicable (real pieces, no planted board).
- Baseline: D-off-Q0 is exactly Track 14's working arm (reproduced by G-off).

### Claims past the evidence (wording fixes required)

- W1: "the binding part is the tight late requirement (97.5% of T by cell
  160), not the stretch near cell 46". Not supported as a localisation. P-Q0
  runs die at a median deepest legal depth of 75.5 (52 to 205) and D-QS at 33
  (5 to 35), both far before cell 160, so a tight scaled curve binds early
  and mid-board too. What the data shows: preregistered cause (a) as stated
  (pruning at depth 46 to 47) is not what stops P (every P-Q0 run passes 51),
  the two early-segment manipulations (Q2, Q3) do not rescue the family (Q3
  is a design artifact, Q2 also tightens the late curve), and loosening the
  whole curve (Q1, QM) does. Rewrite as "the lever is the overall slack of
  the curve as a share of T; this experiment does not localise which segment
  binds".
- W2: "swap the slack and the ranking swaps". True only for the cross pairs
  (D at 98% loses to P/K1 at 79%). At matched slack the default is still
  ahead on point estimate (0.61 to 0.75 per second, intervals including 1).
  Say that, and keep "keying is not shown to matter at matched slack, on 16
  openings" as the conclusion.
- W3: R4 lists P-on-QM and K1-on-QM; R4 is preregistered over "any quota"
  of the original design. Harmless (Q0 and Q1 arms alone already make R4
  inapplicable), but name only the Q0..Q3 arms for the literal reading and
  mention QM separately.

## 4b. Site pages

The verdict does not lean on the site's explored or dead-end pages. Track 14
numbers quoted in the README are graded "reported" there, correctly.

## Rerun

Fresh `git archive` of 7fb856e9 (results excluded), `cargo build --release`,
8 committed test rows rerun on this Mac, 4 parallel, command lines from
`scripts/jobs.py`. All 8 match exactly on hist, hist_legal, nodes,
nodes_legal, deepest and qscale, including rows first run on dell and mac-m1:

| row | host of committed run | legal late nodes (rerun = committed) |
|---|---|---|
| D-off-Q0-s509 | mac-m1 | 1,361 |
| D-off-Q0-s514 | this Mac | 0 (the stranded reference run) |
| D-on-Q0-s514 | this Mac | 188 (the fix rescue) |
| D-off-QS-s515 | this Mac | 0 |
| P-off-QM-s501 | dell | 801 |
| P-off-QM-s516 | this Mac | 44 |
| K1-off-QM-s502 | dell | 2,450 |
| K1-on-QM-s514 | this Mac | 200 |

About 0.04 core-hours. Seconds differ (host and load), node counts do not.

## Not done

- No full-recipe rerun (5 thread-hours on three hosts; outside the 1
  core-hour budget on the real pieces).
- adversarial-review was applied by this judge directly (design, scope,
  prose, code lenses) rather than by fanned-out subagents.
- I did not check the remote hosts' binaries beyond the gate reruns and the
  `qscale` fields; their guard lines say "copied tree".

## Unplanned observations

- P's Q0 curve is the same absolute curve as D's Q0 (scale factor 1.0, 119
  by cell 160). So the experiment already contains both comparisons: same
  absolute quota, P dies (2 of 16 openings); same share of T (QM), P lives
  (14 of 16). The relevant variable is the share of privileged sides the
  curve demands, not the count.
- The orientation fix is free and only rescues stranded runs; D-on-Q0 could
  replace D-off-Q0 as the reference in later work without changing any
  non-stranded run's deep yield.
