# quota-keying: preregistration

Night 2026-11-12, Track 13 (added by the lead after midnight, 2026-11-13).
Label: **measurement**, on the real 16x16 Eternity II pieces (the owner allowed
real-piece measurements on 2026-11-12). Not a record attempt: this measures
search yield; nothing here is Eternity II progress, and no board is hunted.

Subject: the unrefereed memo
`research/program/night/2026-11-12/memos/where-a-record-is-paid.md` (lead
branch, commit 596d7fe). Its claims C1 (keying), C2 (wrong-class breaks), C3
(staging) are treated as claims and measured here on more openings.

## Fixed inputs

- Pieces: `data/official.csv`, a byte copy of
  `research/experiments/single-core-benchmark/engine/data/puzzles/size_16_official_eternity.csv`
  (five clues). `data/official-noclue.csv`: the same with the three hint
  columns zeroed. Both written by `scripts/make_noclue.py`; byte-identical to
  the memo's `data/official.csv` and `data/official-noclue.csv` (checked with
  `cmp` before this file was written). No-clue unless an arm says five clues.
- Engine: `engine/` (binary `qk`), a copy of the Track 10 engine
  `research/experiments/real-puzzle-profile/engine/src/{bw.rs,puzzle.rs}` at
  `dc71ad5` (port of `blackwood_bt`, stage 4, one attempt, top-left row scan,
  seeded shuffle inside privileged-count blocks, piecewise-linear quota as a
  hard prune, cumulative break budget, non-adjacency rule). Added switches, all
  off by default:
  - legal accounting (instrumentation, search unchanged): `hist_legal[d]` =
    nodes at depth d whose board is legal so far (every placed piece shows grey
    exactly on the rim sides of its cell; a node is legal only if its parent
    is). Same definition as the memo's patch 2.
  - `--priv a,b,c`: privileged colours (default 22,21,20, the port's).
  - `--legalbreak`: in the break path, refuse a candidate whose grey sides do
    not match the rim sides of the cell (wrong class or wrongly rotated). The
    no-break path is untouched (its quirk, a piece showing grey toward an
    interior neighbour that is not yet placed, is kept and counted as illegal).
  - `--ringbreakoff`: also refuse a break where the candidate's own mismatched
    side carries a ring colour (Blackwood's rule, see below).
  - `--legalcap N`: stop after N legal nodes, histogram snapshot at N/2.
  - `--stage 1` (A1): for d in 161..255 the quota is
    q(160) + floor((T - q(160)) * (min(d,176) - 160) / 16), T = all sides of
    the privileged colours (q(160) = 119). `--stage 2` (A2): A1, plus on entry
    to depth 161 the interior non-privileged colour with fewest sides not yet
    on the board is chosen (ties: lowest colour number) with its count b on
    the board; for d in 161..255 the board must carry at least
    b + floor((t - b) * (min(d,200) - 160) / 40) sides of it (t = its total),
    so it is fully on the board by depth 200. Both are hard prunes tested
    after the published quota.
- Unlock list (record): 201, 206, 211, 216, 221, 225, 229, 233, 237, 239.
  This is also the list in Blackwood's own source (`Util.cs` line 15).
- Node: one call of the search on a non-clue cell at depth d, counted before
  the quota test (as Track 10). Arrival: a node at depth 201 (no break is
  possible before 201, so every arrival is break-free). Late: depth >= 201.
  Opening: depth < 60.

## Which colour set the published record engine uses

Established before this file was written (`scripts/map_colours.py`, output
committed with the results as `results/colour-map.txt`):

- Blackwood's source, github.com/jblackwood345/EternityII_Solver, commit
  f544e42 (2021-03-30), `EternityII_Solver/Util.cs`: line 12
  `side_edges = { 1, 5, 9, 13, 17 }` (his ring colours), line 13
  `middle_edges = {...}`, line 14 `heuristic_sides = { 13, 16, 10 }`. So in
  his numbering 13 is a ring colour and 16, 10 are interior: "one border colour
  and two interior colours" (`web/content/research/lab/experiments/joshua-blackwood/solver.mdx:95-99`)
  is right.
- His piece list (`Util.cs`, `Get_Pieces()`) maps onto this repo's piece file
  under exactly one colour bijection (grey fixed, same chirality; the mirrored
  chirality has none). Under it his {13, 16, 10} is this repo's **{5, 20, 7}**:
  ring colour 5 (24 sides), interior 20 (50) and 7 (48), T = 122 sides, so the
  quota (119 from depth 160) leaves at most 3 privileged sides off the board.
- The repo's `blackwood_bt.rs:1179-1185` tried his labels literally ({13,16,10}
  in this repo's numbering: three interior colours) and kept {22,21,20}. Neither
  is his set. The memo's triples (3,12,13) and (4,16,17) are one ring and two
  interior colours (124 sides) but not his.
- His break path (`Util.cs` lines 22 to 210, `Program.cs` 175 to 305): candidate
  lists are built per piece class (corners, side pieces, middle pieces), so a
  wrong-class piece is never placed; side pieces enter with their rim side out;
  and a mismatch is refused when the piece's own side there is a ring colour
  (`side_breaks == 0`). `--legalbreak --ringbreakoff` reproduces those two rules;
  `--legalbreak` alone only the first. His engine also places the centre start
  piece and scans from the bottom-left corner, with border cells interspersed
  after depth 180; this port does neither (stated, not fixed).

The calibration (below) ran before this was known for (4,16,17) and after for
(5,20,7).

## Calibration (excluded from every test set)

Seeds 1 and 2 only: (4,16,17) seed 1 at 1e8 with and without `--legalbreak`
(the plain run reproduced the memo's `out/e4-legal-noclue-priv4-16-17-s1-1e8.jsonl`
exactly, `hist` and `hist_legal`), and (5,20,7) with `--legalbreak` at seeds 1
and 2, 1e8 nodes (both passed depth 60; 7 to 72 s per 1e8 nodes). That is
three (triple, seed) pairs on two seeds, one more pair than the lead's limit of
two openings; reported as such. Files in `calib/`. Test seeds start at 101.

## Gates (run first, through the guard, stage `gate`)

- G-port (draft rule P12): `qk bw` with every new switch off (`--stage 0
  --legalcap 1e18`, default colours) against the Track 10 binary `rpp` built
  from `real-puzzle-profile/engine` at this commit, on `data/official.csv`,
  record list, seeds 1, 2, 3, cap 1e8: `hist`, `nodes`, `deepest` identical.
  Plus `qk --priv 4,16,17` no-clue seed 1 at 1e8 against the memo's committed
  `out/e4-legal-noclue-priv4-16-17-s1-1e8.jsonl` (`hist` and `hist_legal`).
- G-nobreak (M2 gate): unlock list `999` (no break ever allowed), (5,20,7)
  no-clue, seeds 1, 2, 3, cap 1e8: default, `--legalbreak`, and
  `--legalbreak --ringbreakoff` give identical `hist` and `hist_legal`.
- Staging switches: their off position is covered by G-port (stage 0 is the
  default path). Arms A0, A1, A2 share every node until the first node above
  depth 160 that A1 or A2 prunes; this follows from the code (the extra tests
  sit after the published quota and only for d > 160) and is not gated.
If G-port or G-nobreak fails, the variant is fixed before any test run, and the
fix goes in PREREG_AMENDMENTS.md.

## M1. Keying (C1)

Openings: seeds 101..110 (10 per arm). No clue. Record list. Cap **1e9 nodes**
per run (checkpoints at 1e7 and 1e8 are in the row). Arms, same seeds:

| arm | privileged | role |
|---|---|---|
| D | 22, 21, 20 | default keying of the port (three interior, 150 sides) |
| P | 5, 20, 7 | **primary corrected keying**: Blackwood's set mapped |
| M4 | 4, 16, 17 | memo's corrected triple (secondary) |
| M3 | 3, 12, 13 | memo's corrected triple (secondary) |
| L | 13, 16, 10 | control: his labels taken literally (three interior, 148 sides, no colour can be used up); everything of a mis-keyed quota except exhaustion |

Per run: N = nodes, A = hist[201] (arrivals), N_open = nodes at d < 60,
N_mid = nodes at 60 <= d <= 200, late = nodes at d >= 201, and the same over
`hist_legal` (NL, AL, lateL). Never-left-the-opening: deepest <= 60. Runs with
A = 0 stay in every pooled sum (their nodes count; they add no arrival).

Statistics per arm (pooled over the 10 openings):
- R_tot = sum N / sum A (nodes per break-free arrival, all nodes), and
  R_mid = sum N_mid / sum A (the memo's "nodes at 60..200 per arrival" basis;
  the memo compared its corrected R_mid with the port's R_tot, so both are
  reported for every arm). Also per-opening N/A, median, with A = 0 as
  infinite.
- S_all = sum late / sum N; S_legal = sum lateL / sum NL (late share on legal
  boards only, counting only legal nodes in numerator and denominator).
- Count of runs that never leave the opening, and of runs with A = 0.
- Gain G = R_tot(D) / R_tot(P) (and for M4, M3 and L in place of P), with a
  95% percentile bootstrap interval over openings (10,000 paired resamples of
  the 10 seeds, seed 20261113; a resample in which an arm has no arrival gives
  an infinite R for that arm and is kept, so the interval may be open).

Reading for C1 (memo: gain 450 to 4,600; late share 54 to 66% all, 8 to 20%
legal): C1's size is **supported** for an arm if the lower end of the
interval of G is >= 100; **contradicted** if the upper end is < 45 (a decade
below the memo's range); otherwise not decided. Measurement, no kill.

Five-clue arm (labelled C5, separate): `data/official.csv` (five clues), arms
D and P, seeds 101..110, cap 1e9. Same statistics. Reported plainly if no run
leaves the opening (deepest <= 60) or none arrives at 201.

## M2. Legal break path (C2)

Same seeds, cap and record list as M1. Arms: P+LB = (5,20,7) `--legalbreak`;
M4+LB, M3+LB likewise; P+BW = (5,20,7) `--legalbreak --ringbreakoff`
(secondary). Each is compared with the M1 arm of the same triple.

Statistic: Y = sum over openings of legal nodes at d >= 201, divided by the sum
of all nodes spent (cap 1e9 each, so per node spent). Ratio Y(arm+LB) /
Y(arm), 95% paired bootstrap over openings (as M1). Also reported: N/NL of the
M1 arm (the memo's "nodes per legal node", 1.75 to 2.7), legal arrivals per
node, deepest legal depth, legal completions.

Reading for C2 (memo: 1.75 to 2.7 times): **supported** if the lower end of the
interval is > 1.5; **contradicted** if the upper end is < 1.2; otherwise not
decided. Gate G-nobreak must pass first.

## M3. Staging (C3)

Engine: `--legalbreak` (the memo's section 4 asks for a legal engine), triple
(5,20,7), no clue, record list. Arms A0 (`--stage 0`, published quota, 119 of
122 by depth 160), A1 (`--stage 1`, all 122 privileged sides on the board by
depth 176), A2 (`--stage 2`, A1 plus a third interior colour fully on the
board by depth 200). The memo wrote 124 because its triples have 124 sides;
the published triple has 122, and A1 is "all privileged sides".

Openings: seeds 301, 302, ... in order, each run first as A0 with a cap of 1e8
nodes; an opening is admitted if that run reaches depth 61. The first **12**
admitted seeds out of at most 40 tried are the openings. If fewer than 6 of 40
pass, (4,16,17) seeds 301.. are added by the same rule up to 12 (A1 then has
T = 124). The A0 scan run is a prefix of the full A0 run (the search is
deterministic), so admission does not depend on anything after 1e8 nodes.

Budget: **1e9 legal nodes** per arm and opening (`--legalcap 1e9`, node cap
1e12), with a snapshot at 5e8 legal nodes. Same seed in the three arms, so the
searches are identical down to depth 160.

Statistic: D230 = (sum over openings of legal nodes at depth >= 230) / (sum
over openings of legal nodes at depth >= 60). Ratios D230(A1)/D230(A0) and
D230(A2)/D230(A0), with a 95% percentile bootstrap over openings (10,000
paired resamples, seed 20261113).

KILL LINE (copied unchanged from the lead's brief):
if D230(A1)/D230(A0) and D230(A2)/D230(A0) are both below 3, staging is dead on the real pieces; it continues only if one arm reaches 10; uninformative with fewer than four openings or fewer than 100 deep nodes in A0.

("Deep nodes in A0" = the numerator of D230(A0), summed over openings.)
Point estimates decide the kill line; the bootstrap interval is reported with
it. Null for noise: D230 of the first 5e8 legal nodes of each A0 run against
the second 5e8, pooled the same way. Also reported per arm: legal arrivals per
legal node, legal nodes at d >= 201 per arrival, deepest legal depth, legal
completions, and the third colour chosen (A2, `c3_hist`).

## Order, stop boundary, compute

Order: gates; M1 and M2 and C5 (all 10 seeds, interleaved by seed); M3 scan;
M3 runs opening by opening (A0, A1, A2 of one opening launched together).
Stop boundary: at **04:30** local any run still going is killed. M1/M2/C5 arms
are reported on the seeds that finished (a killed run is listed, not dropped
silently). M3 counts only openings whose three arms all finished; the others
are listed as incomplete.

Compute: dell (up to 12 threads), mac-m1 (up to 7, 6 used: it carries other
load), this Mac 2 cores until 02:15 then 4. Each run is single-threaded,
`nice`. Estimate: M1 about 7 thread-hours (a corrected run at 1e9 nodes is
about 10 to 12 minutes, a default run under one), M2 about 4, C5 under 1, M3
about 7 (36 runs at up to 12 minutes), gates under 1: about 20 thread-hours;
cap 40. Remote code is shipped by `git archive` of the committed tree.

## What this cannot show

No board will be finished; D230 is a proxy 26 cells from the end. The port
differs from Blackwood's engine in scan corner, border interspersing, start
piece and engineering. Nothing here bears on a full solution.
