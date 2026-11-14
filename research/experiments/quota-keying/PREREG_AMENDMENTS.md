# quota-keying: amendments and preregistered part 2

## Amendment 1 (2026-11-13, about 01:40 local): part 2, after the memo's referee report

What: a separately preregistered second part, added at the lead's request after
the memo under test was refereed
(`research/program/night/2026-11-12/memos/where-a-record-is-paid.REVIEW.md`,
lead branch). Part 1 (PREREG.md) is unchanged: its criteria, arms and stop rule
stand and it is finished as preregistered.

Why: the referee found that the memo's "450 to 4,600 times" compared the port
with five clues at 1e10 nodes against no-clue runs at 1e8; that the published
engine pins the centre piece (139 in Blackwood's numbering, row 8 column 7 of
this repo's piece file, the clue at x=7, y=8 of `data/official.csv`); that it
builds legal boards only, breaks only on interior colours, and has no isolation
rule. Part 1 already holds clue setting and budget fixed per comparison, charges
every node of every run (runs that never leave the opening stay in every pooled
sum), contains the control "default keying, no clue" (arm m1-D) and uses
Blackwood's triple mapped as (5, 20, 7) for the primary corrected arm and for M3
(established independently in PREREG.md from his source; the referee's
`v1_colour_map.out` gives the same bijection and the same triple, 122 sides).

When, and what had run: PREREG.md was committed at 49a3ec2 (01:25). Gates ran
and passed on dell (01:28). Part-1 test runs had started before the lead's
message arrived (main stage on dell and mac-m1 from about 01:29, M3 scan on this
Mac from 01:31 and admitted seeds 301 to 312, M3 runs on this Mac from about
01:34). The scan was stopped once 12 seeds were admitted (seeds 313 and 314 were
running and their rows discarded; their guard start lines have no end line).

Engine change for part 2: one more switch, `--noisolation` (off by default):
the break path skips the port's non-adjacency rule (a break may not touch a
cell that already carries a break). The local binary was rebuilt with it at
01:36 while part-1 M3 runs were queued on this Mac; queued runs therefore use
the rebuilt binary with every part-2 switch off. Gate G2-port below checks that
this path reproduces the Track 10 histogram; the remote part-1 runs use the
binary built from commit c279e7b. Part 2 runs from a separate remote directory
(`~/e2-night/t13p2`) so part 1 is not touched.

Fidelity, from Blackwood's source (`Program.cs` lines 175 to 305, `Util.cs`
lines 22 to 210): in his frame (bottom-left start, rows upward, left and bottom
sides known) corners, the bottom row and the left column never break; middle
cells, the right column and the top row may break after the first break index;
a mismatch is admitted only when the candidate's own side there is not a ring
colour. In the port's frame (top-left start, rows downward, top and left sides
known) his bottom row is the port's top row (never reached after depth 201)
and his left column the port's left column, whose known side (the top) is a
ring colour, so `--legalbreak --ringbreakoff` already refuses breaks in the left
column and at corners. `--legalbreak --ringbreakoff --noisolation` is the
fidelity variant (labelled BWF); with isolation kept it is BWI. Remaining
differences (stated, not fixed): scan corner and direction, border cells
interspersed after depth 180, his restart and timing scheme, engineering.

### Part 2: arms

Seeds 101..110, record unlock list, cap 1e9 nodes per run, as part 1. Centre
clue only = `data/official-centre.csv` (official file, every hint but the
centre one zeroed; byte-identical to the memo's `data/official-centre.csv`).

| arm | clues | privileged | switches |
|---|---|---|---|
| c1-D | centre | 22, 21, 20 | none |
| c1-P | centre | 5, 20, 7 | none |
| c1-M4 | centre | 4, 16, 17 | none |
| c1-M3 | centre | 3, 12, 13 | none |
| c1-L | centre | 13, 16, 10 | none |
| c1-P-lb | centre | 5, 20, 7 | legalbreak |
| c1-P-bwi | centre | 5, 20, 7 | legalbreak, ringbreakoff |
| c1-P-bwf | centre | 5, 20, 7 | legalbreak, ringbreakoff, noisolation |
| nc-P-bwf | none | 5, 20, 7 | legalbreak, ringbreakoff, noisolation |
| c5-P-bwf | five | 5, 20, 7 | legalbreak, ringbreakoff, noisolation |

(No-clue BWI is part 1's arm m2-P-bw, same command.)

### Part 2: gates (before any part-2 run, stage `gate2`)

- G2-port: the rebuilt `qk` with every switch off against `rpp`, seeds 1, 2, 3,
  cap 1e8, five clues: `hist`, `nodes`, `deepest` identical.
- G2-nobreak: unlock list `999`, (5, 20, 7), no clue, seeds 1, 2, 3, cap 1e8:
  plain, legalbreak, legalbreak+ringbreakoff, and the same plus noisolation give
  identical `hist` and `hist_legal`.

### Part 2: statistics and readings

Same per-run quantities and pooled statistics as part 1 (R_tot, R_mid, S_all,
S_legal, Y, N/NL; runs with no arrival kept in every sum), and in addition for
every arm of both parts the per-seed values of N/A, S_all and S_legal with
their minimum, median and maximum (the referee saw late shares of 25 to 88%
by seed, so no single number is reported without its spread).

1. Keying gain, like for like, every node charged: G = R_tot(D) / R_tot(P) at
   each clue setting (no clue: part 1; centre: c1-D, c1-P; five: c5-D, c5-P),
   and also for M4, M3, L at the centre. The centre comparison is the main one
   (the record engine pins the centre). Paired bootstrap over seeds as part 1.
   Reading as part 1: supported if the lower end >= 100, contradicted if the
   upper end < 45, otherwise not decided.
2. Fidelity of the break path (reported as a fidelity fix, not a mechanism):
   Y(c1-P-bwf) / Y(c1-P) and Y(nc-P-bwf) / Y(m1-P), with bootstrap; and the
   isolation rule: Y(c1-P-bwf) / Y(c1-P-bwi) and Y(nc-P-bwf) / Y(m2-P-bw).
   No threshold; reported with intervals.
3. Five clues: whether any c5 run leaves the opening (deepest > 60) and
   arrives at 201, stated plainly.

Stop boundary as part 1: at 04:30 anything still running is killed and
listed. Compute: within the 40 thread-hour cap of PREREG.md (part 2 estimate
about 8 thread-hours).

## Amendment 2 (2026-11-13, 01:48 local): scheduling only

What: jobs moved between machines, no command, arm, seed, cap or criterion
changed. This Mac finished its part-1 M3 runs (openings 301, 302) early, so it
took the last 14 lines of `jobs/part2-dell.tsv` (`jobs/part2-local.tsv`) and
the M3 runs of openings 309 and 310 (`jobs/m3-local2.tsv`); dell keeps
`jobs/part2-dell-b.tsv` and `jobs/m3-dell-b.tsv`. Why: idle cores. While
relaunching dell's part-2 waiter, a `pkill -f after_rows.sh` sent over ssh also
matched its own remote shell (the pitfall named in the brief); the only effect
was that the ssh command ended early; the waiter was relaunched at once.
At 01:59 this Mac also took openings 307 and 308 (`jobs/m3-local3.tsv`); dell
keeps openings 303 to 306 (`jobs/m3-dell-c.tsv`) and its part-2 waiter now
waits for 12 M3 rows. Scheduling only.
At 02:05 dell's M3 runner was started directly with 9 slots (3 part-1 main runs
were still going) instead of waiting for the main runner to exit; the waiting
`after.sh` was stopped first so no job runs twice. Scheduling only.
At 02:06 this Mac took the last 13 lines of dell's part-2 list (`jobs/part2-local2.tsv`); dell keeps `jobs/part2-dell-c.tsv`. Scheduling only.
At 02:13 dell's part-2 runner was started directly with 9 slots (its waiter stopped first, no job runs twice); this Mac took the last 10 lines of that list (`jobs/part2-local3.tsv`), dell keeps `jobs/part2-dell-d.tsv`. Scheduling only.
