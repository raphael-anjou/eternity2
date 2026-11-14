# five-clue-keying

Night 2026-11-12, Track 14. Label: measurement. Real 16x16 Eternity II pieces
with the five official clues. Not a record attempt: no board is banked, and
nothing here is Eternity II progress. Preregistration: `PREREG.md` (6bcad1c,
committed alone at 02:41 local, before any run); amendments in
`PREREG_AMENDMENTS.md` (1: the Track 13 judge's findings, committed at 02:46
before any screening run; 2: a runner fix before the first confirmation run).

Question: with five clues, is the colour keying (the privileged colours whose
sides the quota counts) a lever, and if so which keying?

Grades: **measured** (a row in `results/raw/` behind it), **reported**
(Track 13's committed numbers, not rerun here unless said), **argued**.

## Setup

Engine `engine/` (binary `fck`): a copy of Track 13's `qk` with one switch,
`--quotascaled`, which multiplies the published quota curve by (T - 3) / 119
(T = sides of the three privileged colours; factor exactly 1.0 for the
published triple). Every arm: fidelity engine (`--legalbreak --ringbreakoff
--noisolation`), record unlock list, cap 1e9 nodes per run, one attempt,
top-left row scan without border interleaving. Candidates: the 680 triples of
one ring colour (1..5) and two interior colours (6..22), scaled quota; default
D = (22, 21, 20) with the port's unscaled quota; published P = (5, 20, 7) is
one of the 680.

Port gate (draft rule P12, measured): `fck` reproduced Track 13's committed
rows `c5-P-bwf-s101`, `c1-P-bwf-s102`, `nc-P-bwf-s101` exactly (`hist`,
`hist_legal`, `nodes`, `nodes_legal`, `deepest`), and `c5-P-bwf-s101` with
`--quotascaled` gave the same row; x86 and m1 each reproduced
`c5-P-bwf-s101` before their first job (`scripts/check_gates.py`,
`results/raw/gate-*.jsonl`).

## Screening (measured; `results/screen.txt`, from `scripts/analyze_screen.py`)

Seeds 1, 2, 3; 681 keyings x 3 = 2,043 runs, all finished at 03:47 (no
reduction applied).

- **None of the 680 one-ring keyings placed a single legal node at depth 201
  or more** on any of the three openings. Their deepest legal depth: median 47
  (the median over keyings of each keying's maximum over its 3 openings; the
  median over all 2,040 runs is 39), maximum 136 (1-7-9, seed 1). D reached depth 209 on seeds
  1 and 3 and 204 on seed 2 (3,347 legal late nodes, 64 arrivals at 201; Ys =
  1.12e-6 per node).
- So the screening statistic Ys is 0 for all 680, and the ranking fell to the
  preregistered tie-break (deepest legal depth, then list index). Top 10:

| rank | keying | deepest legal | flagged runs (< 100 legal nodes) |
|---|---|---|---|
| 1 | 1-7-9 | 136 | 0 |
| 2 | 3-7-20 | 103 | 2 |
| 3 | 1-8-9 | 96 | 1 |
| 4 | 5-7-20 (published) | 96 | 0 |
| 5 | 3-6-8 | 94 | 2 |
| 6 | 3-8-22 | 93 | 0 |
| 7 | 1-9-15 | 84 | 1 |
| 8 | 5-7-22 | 84 | 0 |
| 9 | 3-7-17 | 80 | 2 |
| 10 | 1-7-20 | 79 | 1 |

  D would rank 1 (ahead of every candidate); P ranks 4. The random control
  keys rank 100 to 433. With every Ys equal to 0, the top 5 is a selection on
  how far a dead search got, not on yield; the winner's curse warning of
  PREREG.md applies in full.

## Confirmation (measured; `results/analysis.txt`, from `scripts/analyze.py`)

Seeds 401..420, all 20 openings complete for all 11 arms (P is both screening
rank 4 and the published arm). All arms of a seed ran on one host
(amendment 1).

| arm | role | Y per node | Y per second | legal late nodes | arrivals | deepest legal (max) | runs never past depth 60 | flagged runs |
|---|---|---|---|---|---|---|---|---|
| 1-7-9 | screen #1 | 0 | 0 | 0 | 0 | 195 | 5 | 0 |
| 3-7-20 | screen #2 | 0 | 0 | 0 | 0 | 69 | 17 | 1 |
| 1-8-9 | screen #3 | 0 | 0 | 0 | 0 | 102 | 13 | 4 |
| 5-7-20 | screen #4, published | 0 | 0 | 0 | 0 | 196 | 2 | 0 |
| 3-6-8 | screen #5 | 0 | 0 | 0 | 0 | 194 | 5 | 0 |
| D | default | 4.88e-7 | 12.9 | 9,758 | 195 | 209 | 2 | 1 |
| 2-9-11, 1-7-10, 1-20-21, 3-14-15, 5-15-16 | random R1..R5 | 0 | 0 | 0 | 0 | 46 to 57 | 20 each | 8, 0, 0, 0, 0 |

Per-seed y for D: min 0, median 3.45e-7, max 1.47e-6 (seeds 402 and 414 have
none: 402 is a flagged run with 4 legal nodes, 414 stays at depth 33). Every
other arm is 0 on every seed. Nobody reached depth 230 (legal nodes at
depth >= 230: 0 in every arm).

Ratios (95% paired bootstrap over openings, 10,000 resamples, seed 20261113):
every arm against D is 0 [0, 0], per node and per second, with and without
flagged runs. Best screened (1-7-9) against the best random key: G_R = 1
[1, 1] (both 0; 0/0 is 1 by the PREREG convention), per node and per second,
with and without flagged runs.

### Reading (literal, preregistered rule)

G_D = 0 [0, 0], G_R = 1 [1, 1]. The interval of G_R includes 1, so by the rule
the reading is **"not a lever within the one-ring family under the scaled
quota, every arm at 0"**. This is a floor effect: the best-screened against
best-random ratio is 0/0, set to 1 by the preregistered convention, so it
does not say that keying does not matter. Scope: at 1e9 nodes per run, one attempt
per opening, on this port, with a row scan from the top-left corner without
border interleaving. Not uninformative by the rule (20 openings; D has legal
late nodes).

What the literal reading hides (argued): the rule was written for keyings that
differ in yield. Here every one-ring keying, screened, published or random, is
at zero, and the only arm with any yield is the default three-interior keying.
"Not a lever among one-ring-plus-two-interior keyings with this quota" is
what the data support. The default D, with its unscaled quota (119 privileged
sides by cell 160 out of T = 150, about 79% required), beats every one-ring
keying under the scaled quota (T - 3 of T, about 97.5% required). Keying and
quota slack are confounded here, so "keying family matters" is not
established. The published keying runs at factor 1.0, which is Blackwood's own
quota, so its failure points at how this port differs from his engine (scan
order, no border interleaving) at least as much as at the keying.

### Against the centre clue (descriptive)

Track 13's centre-clue fidelity runs at the same budget (`c1-P-bwf`, reported,
its committed rows): Y 0.094, deepest legal 238 to 246, 9.5e7 legal nodes at
depth >= 230. No five-clue arm here gets further: the deepest legal depth over
all 220 confirmation runs is 209 (D).

## Unplanned observations

- **The whole one-ring family is dead with five clues on this port** (measured,
  2,040 screening runs and 200 confirmation runs, none past legal depth 201;
  screening maximum 136, confirmation maximum 196). Track 13 saw it for P
  alone; it holds for all 680 triples.
- The deepest depth reached falls with T (measured, screening): T = 120 (two
  48-side interior colours, quota 117 at cell 160): max 136, 12 of 150 runs
  past depth 60; T = 122: max 103, 23 of 900; T = 124: max 75, 13 of 990. Apart
  from the flagged runs (next item), most runs stop at depth 33 to 56, most
  often at 46 and 47, the end of row 2 just after the clue at (13, 2). Argued: the early quota (59
  privileged sides by cell 48) is the binding prune at the row-2 clue, and a
  family whose privileged sides sit partly on edge pieces cannot supply it
  there; a lower or later quota for five clues is the obvious next test (not
  run).
- **503 of 2,040 one-ring screening runs (25%) are flagged** (fewer than 100
  legal nodes; measured), which lose legality at depth 1 to 8 (deepest legal 5 in 174 runs, 1 in 148),
  i.e. in the top border row, and then go on to depth 18 overall, stopping at
  the first interior cell. This is the port quirk the Track 13 judge named
  (the no-break path places a piece showing grey toward an interior cell and
  the run spends its budget below it). D had 0 flagged screening runs and 1 of 20
  in confirmation. Flagged runs do not change any reading (the statistics
  without them are also 0).
- The published keying in confirmation stops at depth 52 to 196, as in Track
  13's `c5-P-bwf` (52 to 191, reported).
- Wall clock: five-clue runs cost 12 to 90 s per 1e9 nodes (measured, row
  `seconds`), far below Track 13's no-clue published runs (about 1,500 s on
  x86, reported): the searches never reach the expensive break path.

## Corrections made after the judgment

Independent judge (`JUDGE.md`, ee5391c8): verdict kill, measurement valid,
20 of 20 rerun rows identical. Five findings, applied above.

1. **Confound (was: "keying family matters, opposite to the published
   choice").** D has the unscaled quota (119 of 150 privileged sides by cell
   160, about 79% required); every one-ring keying has the scaled quota
   (T - 3 of T, about 97.5%). Keying and slack are not separated. P at
   factor 1.0 is Blackwood's own quota, so its death is a port-versus-engine
   discrepancy at least as much as a keying effect.
2. **Headline.** "Not a lever within the one-ring family under the scaled
   quota, every arm at 0." A floor effect: G_R is 0/0, set to 1 by the
   preregistered convention.
3. **Median 47** is the median over keyings of each keying's maximum; the
   median over all 2,040 runs is 39.
4. **Flagged runs** lose legality at depth 1 to 8, in the top border row,
   then go on to depth 18; the earlier account placed the illegal step in
   row 1 and offered an argued mechanism the rows do not support (removed).
5. **Prose times.** PREREG.md ("written about 02:50") and Amendment 1
   ("about 02:50", "passed on at 02:47") give times that do not match the
   commits: PREREG at 02:41:38, Amendment 1 at 02:46:20. The commit order is
   right and the commits are what count; the PREREG files are left untouched.

Unplanned observation from the judge (rough, different seeds): the fidelity
engine cuts D's yield about 9 times against Track 13's plain-engine `c5-D`
(Y 4.39e-6 on seeds 101..110 against 4.88e-7 on seeds 401..420), so Track 13's
135 arrivals were largely the plain engine's break path. Track 15 is
separating keying from slack (D's keying under the scaled quota, a one-ring
keying under an unscaled or shifted quota, same openings).

## What was not done

- No other quota rule (shift, no scaling, a lower early curve), no keying
  outside the one-ring family except D, no budget above 1e9, no restarts, no
  fix of the port quirk.
- Track 13's `c5-D` (plain engine) was not rerun with the fidelity engine on
  its seeds; D here is the fidelity engine on new seeds.

## Guard logs

`results/run-log.jsonl` (this Mac), `results/run-log-x86.jsonl` and
`results/run-log-m1.jsonl` (remote screening and gate, "copied tree").
Defect, disclosed: the second `scripts/ship.sh` at 03:48 shipped the committed
`results/` directory and overwrote each remote guard log with this Mac's log.
The remote screening logs had been fetched and committed first (f21d8f5), so
nothing is lost: `run-log-x86.jsonl` and `run-log-m1.jsonl` are restored
from that commit, and `run-log-x86-confirm.jsonl` / `run-log-m1-confirm.jsonl`
are the remote logs as found after the confirmation (their first 722 lines are
the copy of this Mac's log, then each host's own gate-free confirmation
lines). `scripts/core_hours.py` deduplicates by run id. `ship.sh` now excludes
`results/`. First guard line: 02:43:05 local (gate), after the PREREG commit
at 02:41:38; one PREREG hash in every line.

Compute (measured, `scripts/core_hours.py`): gate 0.09, screening 18.6,
confirmation 1.9; **20.6 thread-hours**. Wall clock 02:41 (PREREG) to about
03:55 (last confirmation row).

## Scope

This bears on how to configure a record-style engine for the five-clue board
(on this port, the published family of keyings does not even reach depth
201); it does not bear on a full solution.

## Reproduce

```
just experiments five-clue-keying build
just experiments five-clue-keying gate
python3 -I five-clue-keying/scripts/check_gates.py
just experiments five-clue-keying screen 12
python3 -I five-clue-keying/scripts/analyze_screen.py    # results/screen.txt, screen-arms.json
just experiments five-clue-keying confirm 12
python3 -I five-clue-keying/scripts/analyze.py           # results/analysis.txt, summary.json
```

Runs are deterministic (same `hist` for the same command), so the rows
reproduce exactly; `seconds` does not.

## Files

| File | What |
|---|---|
| `engine/` | Track 13's engine plus `--quotascaled` |
| `scripts/jobs.py`, `scripts/jobs_keys.py` | preregistered job lists; keying list and random control |
| `scripts/one.sh`, `run_jobs.sh`, `host_run.sh`, `launch.sh`, `ship.sh`, `sync.sh`, `run.sh` | runners (every run through the guard) |
| `scripts/check_gates.py`, `gate1_check.py` | port gate |
| `scripts/analyze_screen.py`, `scripts/analyze.py` | statistics; outputs in `results/` |
| `results/raw/*.jsonl` | one row per run (per-depth histograms of all and legal nodes, seconds) |
