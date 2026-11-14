# five-clue-quota

Night 2026-11-12, Track 15. Label: measurement. Real 16x16 Eternity II pieces
with the five official clues. Not a record attempt: no board is banked and
nothing here is Eternity II progress. It bears on how to configure a
record-style engine for the five-clue board; it does not bear on a full
solution.

Preregistration: `PREREG.md` (7bb763a, committed alone at 04:00:53 local,
before any run). Amendments in `PREREG_AMENDMENTS.md`: 1, a diagnostic gate
row; 2 (bdcf9b5, 04:05:13), the deconfounding arms and reading R5 relayed by
the lead from the Track 14 judge, committed before the first test run
(03:05:42 UTC = 04:05:42 local). Every command went through
`../common/prereg_guard.py`: `results/run-log.jsonl` (this Mac),
`results/run-log-x86.jsonl`, `results/run-log-m1.jsonl` (remote copies of
the committed tree, so their lines say "copied tree"); 584 start lines, 584
end lines, one PREREG hash.

Grades: **measured** (a row in `results/raw/` behind it), **reported** (another
track's committed number), **argued**.

Scope of every reading: at 1e9 nodes per run, one attempt per opening, on this
port, with a row scan from the top-left corner without border interleaving.

## Setup

Engine `engine/` (binary `fcq`): a copy of Track 14's `fck` with three
switches, all off by default. `--orientfix` refuses, in the no-break path, any
piece that shows grey toward a cell not on the rim or a colour toward the rim
(`wrong_class`, the test the fidelity break path already applies).
`--quotamode 2|3` gives the delayed (Q2: q[i] = q0[i - 48], binds from cell
65) and early-halved (Q3: halved below cell 96) curves; Q1 is `--noquota`.
`--quotaslack` (amendment 2) scales the published curve by T / 150. All arms:
fidelity engine (`--legalbreak --ringbreakoff --noisolation`), record unlock
list, cap 1e9 nodes per run. Seeds 501 to 516, all arms of a seed on one host
(x86 501 to 508, m1 509 to 513, this Mac 514 to 516). No reduction was
needed: 32 arms x 16 openings = 512 test rows, all present.

Quota used by Track 14 (measured from its rows): `conf-D-s401` ran with no
`--quotascaled` (`qscale` 1, 119 of 150 privileged sides by cell 160, 79.3%);
`conf-1-7-9-s401` and `conf-5-7-20-s401` ran scaled (`qscale` 0.983 and 1.0,
T - 3 of T, about 97.5%). The reference arm D-off-Q0 is that working arm.

## Gates (measured; `scripts/check_gates.py` on `results/raw/gate*-*.jsonl`)

- **G-off**: with the fix off, `fcq` reproduced Track 14's `conf-D-s401`,
  `conf-1-7-9-s401`, `conf-5-7-20-s401` exactly (hist, hist_legal, nodes,
  nodes_legal, deepest), again with the amendment-2 binary (`gate2-*`), and on
  each remote host before its first job (`gate1-*`).
- **G-on, unflagged rows**: the fix changes all three rows (123, 12 and 63
  depths with a different count), because each fix-off run had spent 0.99%,
  3.92% and 2.08% of its nodes on illegal boards. The change is fully
  explained: with the fix on and the cap set to the committed row's
  `nodes_legal`, `hist` equals the committed `hist_legal` exactly on all three
  (the fix-on search is the legal-node subsequence of the fix-off search). At
  cap 1e9 the deepest depth, deepest legal depth and legal late nodes are
  unchanged (209 / 209 / 1,335 for D; 20 / 20 / 0 for 1-7-9; 93 / 93 / 0 for
  5-7-20).
- **G-on, flagged rows**: `conf-D-s402` (4 legal nodes) reaches depth 205 with
  1,061 legal late nodes with the fix on; `conf-1-8-9-s403` (4) reaches 63.
  `conf-2-9-11-s401` (2) becomes fully legal but stays at depth 18. Explanation
  (amendment 1 row `gate-on-2-9-11-noquota-1e8-s401`, measured: still 18
  without any quota): not the quota. Cell 18 sits directly above the clue at
  cell 34, and 683M of the run's 1e9 nodes are at depth 18 with no child, so
  under this opening no remaining piece fits cell 18 between its row-0
  neighbour and the clue, and chronological backtracking re-enumerates cells
  8 to 17 for the rest of the budget (argued from the histogram).

Location of the defect (measured): every stranded fix-off test run loses
legality in the top row (cells 0 to 15): first
illegal placement at depth 1 (3 runs), 3 (1), 5 (6), 7 (3), 9 (3), 10 (4),
11 (3) (`results/analysis.txt`, last section). This matches the Track 14 judge
(F4: depth 1 to 8, top row). Mechanism (argued from `bw.rs`): row-0
candidates are keyed on (grey top, left colour); the right side is checked
only at the last column and the bottom only when the cell below is filled, so
a corner piece (grey top and right) can be placed mid-row, and the next cells
match its grey like a colour.

## Results (measured; `results/analysis.txt`, from `scripts/analyze.py`)

Y = legal nodes at depth >= 201; ratios to D-off-Q0, 95% paired bootstrap
over the 16 openings (10,000 resamples). "pos" = openings with any legal node
at depth >= 201.

| arm | stranded | pos | Y per node | Y per s | ratio per s [95%] | deepest legal median (range) |
|---|---|---|---|---|---|---|
| D-off-Q0 (reference) | 1/16 | 12 | 5.68e-7 | 16.3 | 1 | 205 (5..220) |
| D-on-Q0 | 0 | 13 | 5.80e-7 | 16.0 | 0.98 [0.95, 1.04] | 205 (33..220) |
| D-off-Q1 / on | 4 / 0 | 4 / 6 | 3.4e-8 / 8.0e-8 | 0.9 / 1.9 | 0.055 / 0.118 | 199 / 199.5 |
| D-off-Q2 / on | 4 / 0 | 4 / 5 | 8.0e-8 / 8.2e-8 | 2.2 / 2.0 | 0.133 / 0.125 | 198 / 199 |
| D-off-Q3 / on | 4 / 0 | 0 / 0 | 0 | 0 | 0 | 96 / 96 |
| P-off-Q0 / on | 0 / 0 | 2 / 2 | 5.3e-9 | 0.19 / 0.16 | 0.012 / 0.010 | 75.5 / 81.5 (52..205) |
| P-off-Q1 / on | 0 / 0 | 9 / 9 | 8.1e-8 | 2.1 / 2.0 | 0.127 / 0.121 | 203.5 (199..209) |
| P-off-Q2 / on | 0 / 0 | 0 / 0 | 0 | 0 | 0 | 186.5 (179..194) |
| P-off-Q3 / on | 0 / 0 | 0 / 0 | 0 | 0 | 0 | 96 |
| K1-off-Q0 / on | 0 / 0 | 1 / 1 | 2.4e-8 | 0.68 / 0.66 | 0.042 / 0.040 | 69.5 (20..205) |
| K1-off-Q1 / on | 1 / 0 | 8 / 9 | 1.7e-7 / 1.8e-7 | 4.3 / 4.3 | 0.26 [0.06, 0.86] both | 202 / 204 |
| K1-off-Q2 / on | 1 / 0 | 0 / 0 | 0 | 0 | 0 | 181.5 |
| K1-off-Q3 / on | 1 / 0 | 0 / 0 | 0 | 0 | 0 | 96 |
| R1 (2-9-11) off / on, Q0 | 5 / 0 | 0 / 0 | 0 | 0 | 0 | 26.5 / 38 |
| D-off-QS / on (arm a) | 1 / 0 | 0 / 0 | 0 | 0 | 0 | 33 (5..35) / 33 (24..35) |
| P-off-QM / on (arm b) | 0 / 0 | 14 / 14 | 4.08e-7 | 10.0 | 0.61 [0.30, 1.53] / 0.62 [0.28, 1.59] | 205 (199..209) |
| K1-off-QM / on (arm b) | 1 / 0 | 12 / 13 | 4.8e-7 / 4.9e-7 | 12.0 / 12.3 | 0.74 [0.29, 2.18] / 0.75 [0.30, 2.33] | 205 (10..214) / 205 (20..214) |

Full table with per-node intervals, never-past-60 counts and per-seed values:
`results/analysis.txt`. No arm placed a legal node at depth 230 or deeper; the
deepest legal depth of any run is 220 (D, seed 509).

## Readings (literal, preregistered)

- **R1**: one-ring stranded share (the 8 original one-ring arms, P and K1 under
  Q0 to Q3) fix off 3/128 = 2.3%, fix on 0/128: "no stranding to explain in these arms". Supplementary
  **R1'** (2, 9, 11): fix off 5/16 = 31%, fix on 0/16: **the defect explains
  the stranding** (R1'). The fix does not make (2, 9, 11) work: 0 legal nodes
  past depth 46 either way.
- **R2**: not supported. With the fix on, Q1 (no quota) reaches depth >= 201
  on 9 of 16 openings for both P and K1, but their Q0 twins are not zero (2 and
  1 positive openings), so the rule's second condition fails; Q2 and Q3 reach
  0 of 16.
- **R3**: no one-ring configuration meets the rule (best: K1-on-QM, ratio per
  second 0.75 [0.30, 2.33]); the default remains the best configuration
  measured.
- **R4**: does not apply: P-on-Q0, P-on-Q1, K1-on-Q0 and K1-on-Q1 placed
  legal nodes at depth >= 201 (P-on-QM and K1-on-QM did too, but QM is outside
  the original design). On these openings the published
  keying with its own quota is not dead (2 of 16), only rare; Track 14 saw 0 of
  20 on other seeds (reported).
- **R5** (amendment 2): **slack explains the result**, with the fix off and
  with the fix on. Arm (a), the default keying asked for 147 of 150 sides by
  cell 160, has 0 positive openings of 16 (its Q0 twin 12 and 13) and never
  passes depth 35. Arm (b), the one-ring keyings asked for 79% of their sides,
  has 14 (P) and 12 to 13 (K1) positive openings against 2 and 1 for their Q0
  twins.

## What this means (argued)

Track 14's "default beats every one-ring keying" was a quota-slack effect:
the default keying asked for 147 of 150 sides and the one-ring keyings for
79%, and the ones with the looser curve did far better. At matched slack the
default is still ahead on point estimate (the one-ring keyings sit at 0.61 to
0.75 of it per second), with intervals including 1, so on 16 openings keying is
not shown to matter at matched slack. The orientation defect is real and strands runs in the top row,
but it is not why the one-ring family looked dead: P and K1 are rarely
stranded and the fix changes their deep yield only through the few stranded
runs it rescues. The early part of
the curve (cause a) is not the lever either: delaying it (Q2) or halving it
(Q3) gives zero, while removing the whole curve (Q1) or loosening all of it
(QM) helps, so the lever is the overall slack of the curve as a share of the keying's
sides; this experiment does not show which segment of the curve binds (runs
under a tight curve die early and mid-board too, not only late).

## Unplanned observations (measured unless marked)

- Q3 is a design artifact: halving the curve below cell 96 makes a step from
  about 49 to 100 privileged sides at cell 96; no Q3 run of any keying passes
  depth 96, and every Q3 run that reaches 96 stops there. Q3 measures that wall, not
  "a looser early curve".
- The default keying with the scaled quota never passes depth 35, the clue
  at cell 34 and its neighbour, earlier than any one-ring arm under its own scaled quota
  (median 70 to 82).
- Q2 also kills the default (4 to 5 positive openings against 12 to 13):
  shifting the curve 48 cells later makes it tighter late (q2(200) = q0(152)),
  consistent with late slack as the lever.
- With the fix on, the deep yield of every unflagged run is unchanged: in all
  233 test pairs whose fix-off run was not stranded, the fix-off and fix-on
  legal late counts are identical (checked over `results/raw/test-*`); the fix only rescues stranded
  runs (D s508: depth 52 to 197; D s514: 0 to 188 late nodes; K1 s514 under
  Q1 and QM: 172 and 200).
- Stranding is not only the defect: (2, 9, 11) with the fix on is legal and
  still stuck at depth 18 on 6 of 16 openings (cell 18 above the clue at cell
  34, see the gates). A row scan with no look-ahead to the clues can spend
  1e9 nodes under a dead prefix (argued).
- The fix costs no measurable speed per node: D-on-Q0 against D-off-Q0 is
  0.98 [0.95, 1.04] per second, 1.02 per node.

## What was not done

- No other keyings, no curve shape other than the four preregistered and the
  two deconfounding ones, no budget above 1e9, no restarts, no other scan
  order or border interleaving. QM was run with Q0 shape only (no QM x Q1..Q3).
- Track 13's JUDGE.md was not in this worktree; the defect was located from
  the code and the rows.

Compute (measured from the rows' `seconds`): 5.06 thread-hours of test runs,
0.52 calibration, 0.09 gates, 5.67 in all. Wall clock 04:00 to 04:22 local.

## Reproduce

```
just experiments five-clue-quota build
just experiments five-clue-quota gate 4
just experiments five-clue-quota test 12 x86     # seeds 501..508; m1 509..513; local 514..516
python3 -I research/experiments/five-clue-quota/scripts/analyze.py
```
Deterministic per row (same binary, same seed, same counts; seconds vary).
