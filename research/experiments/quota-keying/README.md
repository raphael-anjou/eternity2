# quota-keying

Night 2026-11-12, Track 13. Label: measurement. Real 16x16 Eternity II pieces.
Not a record attempt; nothing here is Eternity II progress. Preregistration:
`PREREG.md` (commit 49a3ec2, committed alone before any test run). Every test
command went through `../common/prereg_guard.py`; the logs are
`results/run-log.jsonl` (this Mac), `results/run-log-x86.jsonl` and
`results/run-log-m1.jsonl` (remote copies of the committed tree, shipped by
`scripts/ship.sh`, so their guard lines say "copied tree").

Grades: **measured** (a row in `results/raw/` behind it), **reported** (a
source opened, not run), **argued**.

## Corrections made after the judgment

The judge (`JUDGE.md`, at 17c5688) kept the branch as a measurement and required
three rewordings. No number and no preregistered reading changed; the wording
below replaces the earlier wording wherever it appeared, and the inline passages
are edited to match.

1. **Staging scope.** The kill is for the memo's schedule on this port, which
   scans rows from the top-left corner without mixing in border cells (A1: all
   122 privileged sides by depth 176; A2: plus a third colour by 200). A2 never
   placed a legal piece past depth 197 in any of 12 openings, so D230(A2) = 0
   means the schedule is nearly impossible with that scan order. It is not a
   yield measurement. The kill line was applied as written and stands for this
   schedule and this engine only.
2. **Wall clock.** The per-node gains of the published keying are a wall-clock
   loss. On the same host (x86) a plain P run costs about 1,500 s per 1e9 nodes
   against about 64 s for D (about 23x per node), because its late nodes go
   through the port's break path, which scans every unused piece per node.
   Pooled over all hosts, m1-P makes 1.25 arrivals per second against 19.2 for
   m1-D. The arms ran on different machines, so the figure is rough, and it is
   partly the port's engineering, not Blackwood's.
3. **Keying scope.** "Contradicted" holds at 1e9 nodes per run, one attempt, on
   this port. The memo's own runs were at 1e10 nodes with five clues.

Two unplanned observations from the judge are added to the list at the end of
Results: the port's no-break defect behind most "never left the opening" runs of
D, and the limit of the breaks-off gates.

## Which colour set the published record engine uses (measured from source)

Blackwood's source (github.com/jblackwood345/EternityII_Solver, commit
f544e42, `EternityII_Solver/Util.cs` lines 12 to 14) names his ring colours
`{1, 5, 9, 13, 17}` and `heuristic_sides = {13, 16, 10}`: one ring colour and
two interior colours in his labels. His `Get_Pieces()` list maps onto this
repo's piece file under exactly one colour bijection (same chirality; none
mirrored), `results/colour-map.txt`, from `scripts/map_colours.py`. Under it his
set is this repo's **{5, 20, 7}**: ring 5 (24 sides), interior 20 (50) and 7
(48), 122 sides, so the quota (119 from depth 160) leaves at most 3 privileged
sides off the board. The referee's `v1_colour_map.out` (lead branch,
`memos/where-a-record-is-paid-review/`) gives the same bijection.
`blackwood_bt.rs:1179-1185` tried {13, 16, 10} literally in this repo's numbering
(three interior colours, 148 sides) and kept {22, 21, 20} (150 sides); the memo's
(3, 12, 13) and (4, 16, 17) are one ring and two interior colours but not his.
His break path (`Util.cs` 22 to 210, `Program.cs` 175 to 305) uses class-specific
candidate lists (no wrong-class piece is ever placed), admits a mismatch only
when the piece's own side there is not a ring colour, never breaks at corners,
in his bottom row or his left column, and has no isolation rule.

## Results

All numbers below are in `results/analysis.txt` (written by
`scripts/analyze.py` from `results/raw/`); intervals are 95% percentile
bootstraps over openings (10,000 paired resamples, seed 20261113). Every run
has a cap of 1e9 nodes (M1, M2, part 2) or 1e9 legal nodes (M3); runs that
never arrive at depth 201 stay in every pooled sum. "D" = port default keying
(22, 21, 20), "P" = Blackwood's published triple (5, 20, 7). No run finished a
board, legal or not.

### M1, keying (C1), no clue, 10 openings per arm (measured)

| arm | arrivals (runs with none) | R_tot nodes per arrival | R_mid (depth 60..200) per arrival | S_all pooled (per-seed min..max) | S_legal pooled (min..max) |
|---|---|---|---|---|---|
| D (22, 21, 20) | 12,607 (1) | 7.9e5 | 5.7e5 | 0.012 (0..0.036) | 0.0039 (0..0.009) |
| P (5, 20, 7) | 16,527 (0) | 6.0e5 | 9.1e4 | 0.84 (0.55..0.94) | 0.18 (0.04..0.46) |
| M4 (4, 16, 17) | 10,484 (4) | 9.5e5 | 5.7e4 | 0.22 (0..0.72) | 0.016 (0..0.08) |
| M3 (3, 12, 13) | 15,183 (2) | 6.6e5 | 6.8e4 | 0.30 (0..0.74) | 0.039 (0..0.13) |
| L (13, 16, 10), control | 11,115 (3) | 9.0e5 | 4.2e5 | 0.077 (0..0.43) | 0.0020 (0..0.005) |

Gain G = R_tot(D) / R_tot(arm), every node charged: P 1.31 [0.69, 2.43],
M4 0.83 [0.31, 1.61], M3 1.20 [0.45, 2.54], L 0.88 [0.36, 1.73]. Preregistered
reading: **contradicted** for every arm (upper end below 45; the memo claimed
450 to 4,600), at 1e9 nodes per run, one attempt, on this port (the memo's runs
were at 1e10 with five clues). On the R_mid basis (opening excluded, the memo's corrected-arm
basis): P 6.25 [4.73, 8.52], M4 10.1 [6.9, 14.1], M3 8.4 [5.8, 15.5], L 1.37
[1.03, 1.80]. Runs that never left the opening (deepest <= 60): D 1, P 0, M4 0,
M3 1, L 2 of 10. Runs with no arrival in 1e9 nodes: D 1, P 0, M4 4, M3 2, L 3.

Wall clock (judge's correction; these gains are per node only). On the same host
(x86), a plain P run costs about 1,500 s per 1e9 nodes against about 64 s for D
(about 23x per node), because its late nodes go through the port's break path,
which scans every unused piece per node. Pooled over all hosts, m1-P makes 1.25
arrivals per second against 19.2 for m1-D. The R_mid "gains" of 6 to 10 are
therefore a wall-clock loss. The host mix differs between arms, so this is a
rough figure, and it is partly the port's engineering, not Blackwood's.

### Five clues (C5, measured)

D: 135 arrivals over 10 runs (R_tot 7.4e7; late share 6.6e-6). **P: no run
reached depth 201** (deepest 52 to 191; 8 of 10 left the opening, none passed
depth 191), and neither did P with Blackwood's break path (`c5-P-bwf`, same
search before 201). With the five clues on the board the published keying
never produced an arrival in 10 x 1e9 nodes.

### M2, legal break path (C2), no clue, 10 openings (measured)

Y = legal nodes at depth >= 201 per node spent. Y(arm with `--legalbreak`) /
Y(arm): P 7.63 [4.82, 13.9], M4 4.60 [1.94, 8.11], M3 3.16 [2.00, 6.03];
P with Blackwood's ring rule as well 7.02 [4.42, 12.9]. Preregistered reading:
**supported** (lower end above 1.5) for all three; the effect is larger than
the memo's 1.75 to 2.7. Nodes per legal node in the plain P arm: 5.09 (memo:
1.75 to 2.7). Gate G-nobreak passed (identical histograms with breaks off).
Per the referee, this is a fidelity fix (Blackwood's engine never places a
wrong-class piece), not a mechanism.

### Part 2 (PREREG_AMENDMENTS.md, measured)

Centre clue only, 10 openings per arm. Gain G = R_tot(c1-D) / R_tot(arm): P 0.74
[0.42, 1.27], M4 0.029 [0.001, 0.078], M3 0.024 [0.001, 0.072], L 0.85 [0.37,
1.76]: **contradicted** for every arm. R_mid basis: P 1.65 [1.03, 2.64]. With
the centre pinned, P's late share is 0.62 of all nodes (per seed 0.07..0.96)
and 0.053 legal (0.007..0.12); D's is 0.0067 (0..0.021) and 0.0022.

Break-path fidelity (Y ratios): centre, BWF / plain P 4.47 [3.06, 6.41], legal
only 2.66 [1.90, 3.71], legal + ring rule 2.40 [1.72, 3.35]; no clue, BWF /
plain P 10.8 [7.0, 19.4]. Isolation rule (BWF against BWI, the only
difference): dropping it raises Y by 1.86 [1.68, 2.03] at the centre and 1.53
[1.40, 1.71] with no clue. Late share of the fidelity engine (BWF, all nodes
legal): no clue 0.37 (per seed 0.25..0.55), centre 0.094 (0.013..0.22); five
clues 0 (no arrival).

### M3, staging (C3), 12 openings (measured)

Admission: seeds 301 to 312 all passed depth 60 within 1e8 nodes (12 of 12
tried). Legal engine (`--legalbreak`), P, no clue, 1e9 legal nodes per arm and
opening.

| | A0 (published, 119 of 122 by 160) | A1 (all 122 by 176) | A2 (A1 + third colour by 200) |
|---|---|---|---|
| D230 (pooled) | 0.0224 | 0.00115 | 0 |
| legal nodes at >= 230 (sum) | 2.65e8 | 1.35e7 | 0 |
| legal arrivals (sum) | 133,831 | 5,964 | 0 |
| deepest legal depth (range) | 238..246 | 236..245 | 181..197 |

D230(A1)/D230(A0) = 0.051 [0.031, 0.081]; D230(A2)/D230(A0) = 0 [0, 0].
Kill line read literally: both ratios are below 3, 12 openings (>= 4) and 2.65e8
deep nodes in A0 (>= 100): **staging is dead for the memo's schedule on this
port**, which scans rows from the top-left corner without mixing in border
cells. A2 never placed a legal piece past depth 197 in any of 12 openings, so
its zero means the schedule is nearly impossible with that scan order and is not
a yield measurement. Noise null:
A0's first 5e8 legal nodes give D230 0.0207, the second 5e8 0.0241 (ratio 1.16).

### Unplanned observations (measured unless marked)

- The five-clue published keying never reaches depth 201 (above).
- A2 never placed a legal node past depth 197 in any of 12 openings. Argued
  reason: every interior colour also sits on the inner side of 1 to 6 edge
  pieces (`data/official.csv` census), and by depth 200 the row scan has filled
  37 of 56 edge cells, so fully placing a third colour by 200 forces all of its
  edge pieces into the first 37 edge cells, on top of A1's demand that all 21 edge and corner
  pieces carrying ring colour 5 are placed by depth 176 (count from the referee's `v1_colour_map.out`). The memo's staging asks for
  something the row scan can rarely supply.
- The plain port spends most of its late nodes on illegal boards with the
  published keying (5.09 nodes per legal node with no clue), more than with the
  memo's triples (1.27 and 1.61).
- `c1-P-lb` and `c1-P-bwi` have identical arrival counts on all 10 seeds while
  their late work differs by 8 to 13%: the budget saved late is spent after the
  last arrival in shallow opening branches that never reach 201 again within
  the cap. Arrival counts at a fixed cap are therefore sensitive to where the
  cap falls.
- Late shares vary by more than a decade between seeds in every arm (spread
  table in `results/analysis.txt`), as the referee warned.
- Added by the judge. Some seeds are stuck at depth 22 to 66 because the port's
  no-break path placed a wrongly oriented piece early (a piece showing grey
  toward an interior cell, a quirk kept from the original). This is a defect of
  the port, not a keying effect, and `--legalbreak` cannot fix it since it only
  touches the break path. Most "never left the opening" runs of the default
  keying are this: m1-D-s105 / c1-D-s105 / c5-D-s105 (deepest 22, 22, 49; 5 legal
  nodes out of 1e9) and m1-M3-s108 / c1-M3-s108 / m2-M3-lb-s108 (2 legal nodes).
  Dropping s105 from D would move G_tot by about 10%, not the reading.
- Added by the judge. The breaks-off gates (G-nobreak, G2-nobreak) only show that
  the switches leave the no-break path untouched. They do not test the break
  path itself; nothing short of an independent reimplementation could.
- Added by the judge. c5-P and c5-P-bwf rows are identical seed for seed (no run
  reaches depth 201, so the break switches never act); likewise m1-L and c1-L at
  seed 101 (the centre cell is never reached).

### What was not done

- No variant of the scan corner, border interspersing or restarts of
  Blackwood's engine; budgets above 1e9 per run; the memo's (4, 16, 17) M3
  fallback (not needed: 12 of 12 seeds passed).
- The calibration used three (triple, seed) pairs on two seeds, one pair more
  than the lead's limit (PREREG.md, Calibration).

Compute: 16.1 thread-hours of runs from the guard logs
(`scripts/core_hours.py`), wall clock 01:14 to about 02:40 local.


## Reproduce

```
just experiments quota-keying build
just experiments quota-keying gate 12
python3 -I quota-keying/scripts/check_gates.py
just experiments quota-keying main 12
just experiments quota-keying m3scan 4
just experiments quota-keying m3 12 <admitted seeds, comma list>
python3 -I quota-keying/scripts/analyze.py      # writes results/summary.json, results/analysis.txt
```

The colour map needs Blackwood's source, which is not vendored here (GPL-3.0):

```
git clone https://github.com/jblackwood345/EternityII_Solver /tmp/bw && git -C /tmp/bw checkout f544e42
python3 -I quota-keying/scripts/map_colours.py /tmp/bw/EternityII_Solver/Util.cs
```

## Files

| File | What |
|---|---|
| `engine/` | copy of the Track 10 port with the switches of PREREG.md (`qk bw`) |
| `scripts/jobs.py` | the preregistered job lists; `jobs/*.tsv` are its output |
| `scripts/one.sh`, `scripts/run_jobs.sh`, `run.sh` | runners (every run through the guard) |
| `scripts/ship.sh`, `scripts/sync.sh` | ship the committed tree to a remote, fetch rows and guard logs |
| `scripts/check_gates.py` | the gates |
| `scripts/analyze.py` | the preregistered statistics; output `results/analysis.txt`, `results/summary.json` |
| `scripts/map_colours.py`, `results/colour-map.txt` | Blackwood's colour labels to this repo's numbers |
| `results/raw/*.jsonl` | one row per run: per-depth histogram of all nodes and of legal nodes |
| `calib/` | the calibration rows (seeds 1 and 2, excluded from every test set) |
