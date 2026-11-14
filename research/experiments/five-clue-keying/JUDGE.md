# Judge report: five-clue keying (night/2026-11-12-t14-five-clue-keying @ 9269011)

## Verdict: kill

The experiment is valid and its numbers reproduce exactly. The hypothesis "with five clues, a screened one-ring keying (one ring colour plus two interior colours, quota scaled by (T - 3) / 119) is a lever on legal search past depth 201" is dead at this scope: none of the 680 keyings placed a single legal node at depth 201 or more in 2,040 screening runs, and the five screened keyings, the published keying and the five random keys are all at exactly 0 on 20 held-out openings, against D's 9,758 legal late nodes. The preregistered reading ("not a lever") is literally correct, but it comes from a floor effect (G_R = 0/0 = 1 by convention), so it means "this family under this quota rule does not work on this port at 1e9 nodes", not "keying does not matter". Two claims in the README go past the evidence and must be reworded (F1, F2). Scope, in every statement: 1e9 nodes per run, one attempt per opening, this port, top-left row scan without border interleaving.

## Reasons

1. **Preregistration chronology holds.** `PREREG.md` was committed alone in 6bcad1c at 02:41:38 local and never changed after (its SHA-256 57bf0568... matches the committed blob and every guard start line). Amendment 1 (331f63e, 02:46:20) precedes the first screening run (02:46:57 local) and changes only the confirmation deal, adds per-second statistics and the flag; Amendment 2 (5e83939) is a runner fix. The confirmation arms were committed in f21d8f5 (03:48:07) before the first confirmation run (03:48:30).
2. **Guard log complete.** I read all five `results/run-log*.jsonl`: 2,270 unique run ids, every start paired with an end, all exits 0, first line 02:43:05 local (gate) after the PREREG commit, no start line before it, every start line carries the committed PREREG hash (end lines carry no hash by design). Every one of the 2,270 raw rows (7 gate, 2,043 screen, 220 confirm) has a matching log line. Two local confirmation lines show `dirty_paths: 1` at 03:48:45, 38 s before amendment 2 was committed (the amendment prose being written; the engine and runners were the committed f21d8f5 tree). Remote lines say "copied tree" (git check skipped); the overwritten remote logs are disclosed and restored from f21d8f5. The deterministic rerun (item 4) covers rows from dell, mac-m1 and this Mac, so the remote code was the committed code.
3. **Track label.** Measurement; no arm claims to be an optimization, so 1b does not apply. The port gate holds: `scripts/check_gates.py` passes all seven gate rows against Track 13's committed `c5-P-bwf-s101`, `c1-P-bwf-s102`, `nc-P-bwf-s101` (hist, hist_legal, nodes, nodes_legal, deepest), including the factor-1 scaled row and the three per-host reruns. The engine diff against Track 13's `qk` is the `--quotascaled` switch and the `qscale` output field only. `--priv` is a set in the engine, so the P arm's `5,7,20` equals the gate's `5,20,7`.
4. **Rerun: 20 of 20 rows identical.** I rebuilt `fck` from the committed source and reran 20 rows on 4 cores (about 6 core-minutes): conf-D on seeds 401, 402 (flagged), 405, 414, 416; conf 5-7-20 s402 (deepest 196), 1-7-9 s420 (195), 3-6-8 s413, 2-9-11 s401 (flagged), 1-20-21 s419; scr 1-7-9 s1..s3, scr D s1..s3, scr 2-6-8 s3 (flagged), 2-16-17 s1, 5-7-20 s2, 3-14-15 s1. hist, hist_legal, nodes, nodes_legal, deepest, qscale identical on all 20.
5. **Accounting is equal.** Every run hit the 1e9 cap (no exhausted tree, no censoring question), every node charged, no seed dropped, no subsampling: 681 x 3 screening rows and 11 x 20 confirmation rows, all present. All arms of a confirmation seed ran on one host. A random-keying control exists (five draws matching `random.Random(20261113)` as preregistered). Wall clock: no node win to hide; five-clue rows cost 11 to 90 s (the 191 s row is the no-clue gate).
6. **The reading is a floor effect.** With every one-ring arm at 0, G_R = 1 [1, 1] is 0/0, and the screen ranked on the tie-break (deepest legal depth), a proxy of how far a dead search got. The screened top 5 then confirmed nothing. The README says this under "What the literal reading hides"; the headline reading should carry it (F3).

## Findings

- **F1 (claim past the evidence, must fix).** README: "the default keying (outside that family) does beat every member of it, so keying family matters, in the direction opposite to the published engine's choice." D differs from the family in two things at once: its keying, and its quota slack. D runs the unscaled curve (119 privileged sides by cell 160 out of T = 150, about 79%), every family member runs T - 3 of T (about 97.5%). Keying and slack are not separated, so "keying family matters" is not established. Fix: "D, with its unscaled quota, beats every one-ring keying under the scaled quota; keying and quota slack are confounded here." Also add that P at factor 1.0 is Blackwood's own quota, so P's death is a port-versus-engine discrepancy (scan order, no border interleaving) at least as much as a keying effect.
- **F2 (imprecise description).** "Their deepest legal depth: median 47, maximum 136 (over 680 keyings and 3 openings)": 47 is the median over keyings of the per-keying maximum; the median over all 2,040 runs is 39. Say which.
- **F3 (headline wording).** State the reading as "not a lever within the one-ring family under the scaled quota (floor: every arm 0)", not bare "not a lever".
- **F4 (argued explanation does not match the rows).** Flagged runs (503 of 2,040, confirmed) lose legality at depth 1 to 8 (the top border row: deepest legal 5 in 174 runs, 1 in 148), then wander to depth 18 overall. The README's mechanism ("edge pieces carrying it come first at interior cells", "at the first interior cells of row 1") places the illegal step in row 1; the data place it in row 0. Reword as "the run leaves legality in the top row and stops at the first interior cell".
- **F5 (cosmetic).** PREREG.md says "written about 02:50" and Amendment 1 says "about 02:50" and "passed on at 02:47", but they were committed at 02:41:38 and 02:46:20. The commits are what count and they are in the right order; the prose times are wrong.

## Recomputed numbers

My own script (not committed) from `results/raw/`:

| quantity | mine | README |
|---|---|---|
| one-ring screening rows with any node (legal or not) at depth >= 201 | 0 of 2,040 | 0 |
| one-ring deepest legal, max | 136 (1-7-9 s1) | 136 |
| D screening: deepest legal per seed / legal late / arrivals / Ys | 209, 204, 209 / 3,347 / 64 / 1.116e-6 | same |
| screening top 5 | 1-7-9 (136), 3-7-20 (103), 1-8-9 (96), 5-7-20 (96), 3-6-8 (94) | same |
| P rank; random ranks | 4; 410, 381, 159, 433, 100 | 4; 100 to 433 |
| flagged one-ring screening runs; D | 503; 0 | 503; 0 |
| deepest by T: 120 / 122 / 124 (max, runs past 60) | 136, 12/150 / 103, 23/900 / 75, 13/990 | same |
| D confirmation: Y per node / per second / legal late / arrivals / deepest | 4.88e-7 / 12.9 / 9,758 / 195 / 209 | same |
| D per-seed y min / median / max | 0 / 3.44e-7 / 1.47e-6 | 0 / 3.45e-7 / 1.47e-6 (upper median) |
| every other confirmation arm: legal late, arrivals | 0, 0 | 0, 0 |
| confirmation deepest legal max: 1-7-9, 3-7-20, 1-8-9, P, 3-6-8 | 195, 69, 102, 196, 194 | same |
| legal nodes at depth >= 230, any arm | 0 | 0 |
| Track 13 (reported, checked on its committed rows): c1-P-bwf Y, deepest legal; c5-P-bwf deepest legal; c5-D arrivals | 0.0940, 238..246; 52..191; 135 | same |

The bootstrap intervals are degenerate (every non-D arm is 0 on every seed), so I did not rerun them; 0 [0, 0] and 1 [1, 1] follow directly.

## Explored / dead-end pages

The verdict does not lean on any site page. Track 13's figures used here are graded measured: I recomputed them from Track 13's committed rows on this branch.

## Unplanned observations

- The fidelity engine costs D about 9x: Track 13's plain-engine `c5-D` gives Y 4.39e-6 (seeds 101..110), the fidelity-engine D here 4.88e-7 (seeds 401..420). Different seeds, so a rough figure, but it says Track 13's 135 arrivals were largely the plain engine's break path.
- The test that would separate keying from slack is cheap: D's keying with the scaled quota, and a one-ring keying with an unscaled or shifted quota, on the same 20 openings (about 1 thread-hour).
