# quota-keying: judge's report

Branch `night/2026-11-12-t13-quota-keying`, judged at HEAD 55bd52e by a
session that did not run the experiment. Label: measurement (no optimization
track, so the node-sequence hash check of step 1b does not apply).

## Verdict: keep

The measurement is valid and reproduces exactly. Its three preregistered
readings stand: C1 (keying gain of 450 to 4,600) **contradicted** at every
clue setting tested; C2 (legal break path) **supported**; C3 (staging) **dead
by the literal kill line**, but only for the memo's schedule on this port (see
required wording fix 1). Three wording fixes are required before any of this
is cited on the site; none changes a number or a reading.

## 1. Preregistration order (passes)

- `PREREG.md` committed alone at 49a3ec2, 01:25:38 +01:00, never modified
  afterwards (`git log --follow`: one commit). SHA-256 44978b67... matches the
  committed file and every one of the 294 guard start lines in the five logs.
- First test-stage guard line: 00:28:18Z (01:28:18 local, dell, `test-main`),
  2 min 40 s after the PREREG commit and after the engine commit c279e7b
  (01:26:22). Calibration rows (01:19 to 01:23) used seeds 1 and 2 only and are
  excluded from every test set.
- Part 2 (`PREREG_AMENDMENTS.md` amendment 1) committed at 00:37:32Z; its gates
  ran 00:39:09 to 00:40:01Z; first `test-part2` line 00:44:11Z. Amendment 2 is
  scheduling only (checked: no arm, seed, cap or command changed in the job
  files it adds).
- All 292 raw rows have a guard start line with matching label and seed and an
  end line with exit 0; no label/seed has two successful runs. The only
  unended lines are m3scan seeds 313 and 314, disclosed and discarded (the scan
  rule stops at 12 admitted seeds).
- 12 local guard lines carry `dirty_paths = 1`. The following commits show the
  modified tracked file was `scripts/analyze.py` / `sync.sh` /
  `PREREG_AMENDMENTS.md`, never the engine (engine source unchanged after
  1b02303). Local M3 run 302-A0 started 15 s before the 01:36:44 rebuild that
  added `--noisolation`; my rerun from HEAD reproduces it exactly, so the binary
  question is moot.
- Remote logs say "copied tree" (no git on the hosts), as disclosed; their
  PREREG hashes match.

## 2. Gates (pass)

`scripts/check_gates.py` on the committed rows: all 13 PASS (G-port x3,
G-nobreak x3, memo comparison, G2-port x3, G2-nobreak x3). I rebuilt `qk` and
`rpp` from a `git archive` of HEAD and reran `gate-port-rpp-s1`: identical.
Note that G-nobreak/G2-nobreak only show the switches leave the no-break path
untouched; they do not test the break path itself (nothing could, short of an
independent reimplementation). I read the break-path code
(`engine/src/bw.rs` 618 to 690): `--legalbreak` filters by `wrong_class`
before evaluation, `--ringbreakoff` refuses a mismatch whose own side has a
colour with at most 24 sides, `--noisolation` skips only the adjacency test.
Matches PREREG.

## 3. Recomputed numbers (own script, from `results/raw/`)

Every pooled statistic in `results/analysis.txt` and the README matches my
recomputation to the printed precision; my bootstrap uses a different RNG and
lands within 0.1 of each interval end.

| quantity | branch | mine |
|---|---|---|
| G_tot no clue, P / M4 / M3 / L | 1.31, 0.83, 1.20, 0.88 | 1.31 [0.68, 2.43], 0.83 [0.31, 1.60], 1.20 [0.44, 2.57], 0.88 [0.37, 1.77] |
| G_mid no clue, P | 6.25 [4.73, 8.52] | 6.25 [4.65, 8.56] |
| G_tot centre, P / M4 / M3 / L | 0.74, 0.029, 0.024, 0.85 | 0.743 [0.42, 1.26], 0.0287, 0.0243, 0.852 |
| C5: P arrivals; D arrivals | 0; 135 | 0 (deepest 52..191); 135 |
| Y ratio P+LB / P, M4, M3, P+BW | 7.63, 4.60, 3.16, 7.02 | 7.63 [4.87, 14.3], 4.60 [1.93, 8.12], 3.16 [1.96, 6.01], 7.02 [4.47, 13.1] |
| Y c1-BWF/c1-P; nc-BWF/m1-P; isolation centre / no clue | 4.47, 10.8, 1.86, 1.53 | 4.47, 10.8, 1.86, 1.53 |
| N/NL plain P (no clue) | 5.09 | 5.09 |
| D230 A0 / A1 / A2 | 0.0224, 0.00115, 0 | 0.02241, 0.001146, 0 |
| deep legal nodes A0 | 2.65e8 | 265,296,654 |
| D230 ratio A1/A0 | 0.051 [0.031, 0.081] | 0.0512 [0.0309, 0.0814] |
| noise null first / second half | 0.0207 / 0.0241 | 0.0207 / 0.0241 (ratio 1.16) |
| thread-hours | 16.1 | 16.13 |

`sum(hist) == nodes` holds for every row. Every arm has all 10 seeds (M3: all
12 openings, three arms each); clue file per arm verified from the guard
commands (rows do not record the CSV).

## 4. Rerun (exact)

19 runs from HEAD, built from `git archive`, 4 cores, about 16 core-minutes:
gate-port-rpp-s1; m1-D-s105, m1-M4-s101, m1-L-s101, m2-M4-lb-s105,
m2-P-bw-s109, c5-D-s103, c5-P-s102, c5-P-bwf-s101, c1-D-s110, c1-L-s101,
c1-M3-s101, c1-P-lb-s110, c1-P-bwf-s102, nc-P-bwf-s109; M3 A0-s302, A1-s302,
A1-s307, A2-s309. `nodes`, `nodes_legal`, `deepest`, `hist`, `hist_legal`,
checkpoints, legal snapshots and `c3_hist` identical on all 19 (15.7 core-minutes measured).
This covers all three binary lineages (dell/mac-m1 part 1 from c279e7b, part 2
from the later commits, and the local rebuild).

## 5. Adversarial checks

- Order control: every arm is 10 shuffled-order seeds, paired across arms
  (M3: 12 openings, same seed in A0/A1/A2). Not a single fixed-order baseline.
- Leakage: none possible (no planted solution; real pieces, no board hunted).
- Node accounting: identical across arms (one count per non-clue search call,
  before the quota test); cap 1e9 nodes per run in M1/M2/part 2, 1e9 legal
  nodes in M3. Runs with no arrival stay in every pooled sum.
- Censoring: every M1/M2/part-2 run stopped at its cap; no run is counted as a
  win; no completions anywhere.
- Dropped seeds / subsampling: none.
- Baseline: the default keying D is the port's own (`blackwood_bt.rs`), and L
  is a sensible "mis-keyed but not exhaustible" control. Adequate for the
  question asked (does the memo's corrected keying give a 450x gain).
- Wall clock: see required fix 2. Node-normalised gains hide a large per-node
  cost difference.
- Claims past evidence: see required fixes 1 and 3.

## Required wording fixes (README and any ledger or site text)

1. **M3 scope.** "staging is dead on the real pieces" must read: dead for the
   memo's schedule (A1: all 122 privileged sides by depth 176; A2: plus a third
   colour by 200) on the top-left row-scan port without border interspersing.
   A2 never placed a legal node past depth 197 in 12 openings, so D230(A2) = 0
   is a structural infeasibility of that schedule under row scan (the README's
   own argued edge-cell count), not a yield measurement. The kill line was
   applied as written and stands for this schedule and engine.
2. **Wall clock.** Add next to the M1 table: on the same host (dell), a plain
   P run costs about 1,500 s per 1e9 nodes against about 64 s for D (about 23x
   per node), because its late nodes go through the port's break path, which
   scans every unused piece per node. Pooled over all hosts, m1-P makes 1.25
   arrivals per second against 19.2 for m1-D. The R_mid "gains" of 6 to 10 are
   per node only and are a wall-clock loss; host mix differs between arms, so
   this is a rough figure and also partly the port's engineering, not
   Blackwood's.
3. **C1 scope.** "contradicted" holds at 1e9 nodes per run, one attempt, on this
   port. Say so where the reading is stated; the memo's own runs were at 1e10
   with five clues, which the referee already flagged.

## Grading of what the verdict leans on

- Blackwood's colour set {5, 20, 7}: measured here (`scripts/map_colours.py`,
  committed output) and agrees with the referee's independent map.
- The memo's claimed ranges (450 to 4,600; 1.75 to 2.7): reported (memo, lead
  branch), used only as the thresholds the preregistration fixed.
- No site "explored" or "dead end" page is relied on.

## Unplanned observations (judge)

- Two arms lose whole runs to an illegal subtree at the very start:
  m1-D-s105 / c1-D-s105 / c5-D-s105 (deepest 22, 22, 49; 5 legal nodes out of
  1e9) and m1-M3-s108 / c1-M3-s108 / m2-M3-lb-s108 (2 legal nodes). This is the
  kept no-break-path quirk (a piece showing grey toward an interior cell) and
  `--legalbreak` cannot fix it, since it only touches the break path. Most
  "never left the opening" runs in D are this, not a keying effect. Dropping
  s105 from D would move G_tot by about 10%, not the reading.
- c5-P and c5-P-bwf rows are identical seed for seed (no run reaches depth 201,
  so the break switches never act); likewise m1-L and c1-L at seed 101 (the
  centre cell is never reached).
