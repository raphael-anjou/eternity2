# Where a record is paid

Night 2026-11-12. Scripts, the instrumented engine copy and every output are in
`research/program/night/2026-11-12/memos/where-a-record-is-paid/`. `run_all.sh`
there lists every command. Compute used: about 17 core-minutes of engine runs
(two at a time, none above 3e8 nodes), about 3 core-minutes of Python, four
builds of the copy. No existing file was edited and no git command that
changes anything was run.
Nothing here is a record attempt or Eternity II progress.

Grades: **measured** (I ran it, the command or output file is named),
**reported** (somebody else's number, source opened), **argued** (reasoning or
model output with no measurement behind it). "Model" always means the
first-moment count of `partial-match-regime/model.py`, used through the
review's `partial-match-regime-review/r3_sensitivity.py`.

## 0. Answer

1. The Track 10 measurement and the review's inference are about two
   different engines. The port that Track 10 measured privileges three
   interior colours (22, 21, 20: 150 sides) against a quota that stops at 119,
   so it never uses a colour up. The review's 92 to 98% is a model of an engine
   that has removed one ring colour and two interior colours from the last 96
   cells, which is what the record engine is reported to do (measured for the
   port, reported for the record engine).
2. Same binary, same pieces, quota keyed on one ring colour and two interior
   colours (124 sides), no clue: an arrival at depth 201 costs 2.5e4 to 7.8e4
   nodes instead of 3.5e7 to 1.2e8, and 54 to 66% of all nodes fall at depth
   201 or later (measured, two openings, 1e8 and 3e8 nodes).
3. But 79 to 95% of those late nodes sit on boards where a piece of the wrong
   class, or a border piece turned the wrong way, has been bought with one
   break. Counting legal boards only, the late share is 8 to 20% of all nodes,
   10 to 46% once the opening is left out (measured). So neither 4.5e-7 nor 92
   to 98% describes a record-style engine. The cost is spread: between a half
   and nine tenths of it before depth 201, the rest after.
4. With the cost spread like that, nothing that touches one stage can buy
   orders of magnitude. Anything that acts only after depth 201 (bounds, exact
   tail solvers, list tuning, filters on arrivals) is capped at 1.1 to 1.9
   times in this engine (measured split) and at 9 times in the model. Tail
   first, two-sided meets, pockets and frame-last orders are dead in the model
   by 4 to 20 decades.
5. The one lever with decades in it is the one the record engine already
   uses: restrict the search to prefixes whose leftover pool has fewer colours.
   On the real pieces, arrivals with one more interior colour fully used reach
   legal depth 230 three to five times more often (measured). The model prices
   one more colour, used up between depth 160 and 200, at 25 to 50 times
   cheaper per 471 board if the head can be steered at no cost, and at 4 to 8
   decades dearer if it can only be filtered. The truth is between and is
   unmeasured.
6. This lever works only while boards at the target score are plentiful. It
   spends boards to buy yield. At 480 there is nothing to spend. Nothing in
   this memo bears on a full solution.

## 1. Terms

- **Depth** d: number of cells already filled when the search is called (0 to
  255), row scan, as in Track 10. **Node**: one call of the search on a free
  cell, counted before the quota test.
- **Quota**: at each depth the board must already carry at least q(d) sides of
  the three **privileged colours**, or the branch is cut. q reaches 119 at
  depth 160 and stays there.
- **Opening**: depths below 60. **Stretch**: depths 161 to 200, after the
  quota stops rising and before any break is allowed. **Late**: depth 201 or
  more. **Arrival**: a node at depth 201. It carries no break.
- **Legal** board: every placed piece shows grey exactly on the rim sides of
  its cell. A node is legal only if its parent is, so the legal nodes are the
  tree a legal-only engine would walk.
- **Yield**: finished boards per arrival.
- **"15 + 4 colours"**: a model row. The half-edges left for the last 96 cells
  are spread evenly over 15 interior and 4 ring colours. The full set is
  17 + 5.

## 2. Job 1: the reconciliation

### 2.1 What each number counts

Both count the same thing: calls of the search, and "late" means depth 201 or
more. The definitions do not differ. The engines do.

| | Track 10 M1 | Review, section C and E |
|---|---|---|
| Object | the port `bw.rs`, run | a model row, 15 + 4 colours below depth 160 |
| Privileged colours | 22, 21, 20 | one ring and two interior, gone by depth 160 |
| Late share | 4.5e-7 at 1e10 nodes | 0.92 to 0.98 |
| Grade | measured, judged keep | model output |

### 2.2 The port does not use any colour up (measured)

`python3 -I` census of the CSV the port loads
(`single-core-benchmark/engine/data/puzzles/size_16_official_eternity.csv`):
colours 1 to 5 are ring colours with 24 sides each, colours 6 to 10 are
interior with 48 sides, colours 11 to 22 are interior with 50 sides. So 22, 21
and 20 are three interior colours with 150 sides. The quota asks for 119 by
depth 160. Up to 31 privileged sides can still be off the board there. At the
13 arrivals of seed 1 (1e9 nodes) the pool still held 14 to 19 of them, median
18 (`out/a6_extra.out`, part 2).

The site says the record engine privileges "one border colour and two interior
colours" (`joshua-blackwood/solver.mdx`, reported), which is 120 to 124 sides,
and the repo's own schedule struct says the pool is "about 120"
(`solver-engine/src/config.rs:102`, read). With 119 asked of 124, at most 5
sides stay out. `blackwood_bt.rs:1180-1185` explains the port's choice: his
labels 13, 16, 10 were tried literally in the repo's numbering, scored worse at
30 million nodes, and 22, 21, 20 was kept as "empirically near-optimal". In the
repo's numbering 13, 16 and 10 are also three interior colours. The A/B never
tried one ring colour with two interior ones.

Check that my copy is the port: seed 1 at 1e9 nodes gives a histogram
identical to the 1e9 checkpoint of the committed `results/m1/record-s1.jsonl`
(measured, `run_all.sh` step 1).

### 2.3 Same binary, one ring colour and two interior colours (measured)

`engine-copy/` is the Track 10 engine with a `--priv` option and counters
added (`patch_engine.py` to `patch_engine4.py`; the search is not changed, and
each patched build reproduces the previous histograms exactly). No clue on the
board, because the 470 is a no-clue record. Record unlock list, 1e8 nodes.

| Privileged (ring, interior, interior) | seed | deepest | arrivals | nodes at 60..200 per arrival | late nodes per arrival | late share |
|---|---|---|---|---|---|---|
| 3, 12, 13 | 1 | 243 | 187 | 7.1e4 | 2.9e5 | 0.54 |
| 4, 16, 17 | 1 | 245 | 252 | 2.5e4 | 2.6e5 | 0.66 |
| 4, 16, 17, run to 3e8 | 1 | 245 | 1,123 | 7.8e4 | 1.6e5 | 0.62 |
| 1, 20, 21 / 1, 6, 7 / 2, 14, 15 / 5, 18, 19 | 1 | 52 to 69 | 0 | | | 0 |
| 3, 12, 13 | 2, 3, 4 | 71 to 96 | 0 | | | 0 |
| port as committed (22, 21, 20), five clues, 1e10 | 1 to 6 | 207 to 220 | 87 to 285 | 3.5e7 to 1.1e8 | 35 to 71 | 4e-7 to 2e-6 |

Source: `out/a1_hist.out`. Seven of nine runs were still in the opening after
1e8 nodes, so their late share is 0 so far. The two that got through spent
2.8e7 and 3.3e7 nodes below depth 60 first. With five clues on the board the
same keying did not leave the opening in three tries (deepest 18, 46, 60). At
the arrivals of the two open runs, 0 to 4 privileged sides were still off the
board, median 3 to 4 (`out/a6_extra.out`).

So the keying alone moves the cost of an arrival by a factor 450 to 4,600 and
the late share from 1e-6 to more than a half.

### 2.4 Legal boards only (measured)

The break path of this engine tests every unused piece in every rotation and
counts a non-grey side on the rim as a break (`bw.rs`, `eval_candidate`). An
interior piece on a border cell is therefore one break, and so is a border
piece turned inward. The repo's own rule for banking a board asks for a
physical board (`blackwood_bt.rs:999-1003`), and the model counts legal boards
only. `hist_legal` (patch 2) counts the legal nodes.

| Run | late nodes that are legal | nodes per legal node | late share, legal engine | same, opening left out | legal late nodes per arrival | deepest legal depth |
|---|---|---|---|---|---|---|
| 3, 12, 13, 1e8 | 20.7% | 1.75 | 0.195 | 0.456 | 6.0e4 | 236 |
| 4, 16, 17, 1e8 | 5.1% | 2.69 | 0.090 | 0.348 | 1.3e4 | 236 |
| 4, 16, 17, 3e8 | 5.3% | 2.41 | 0.079 | 0.101 | 8.7e3 | 237 |

Source: `out/a7_legal_hist.out`. Before depth 201, 99.2 to 99.8% of nodes are
legal. No run completed a board, legal or not. No legal node went past depth
237, that is past the thirteenth cell of the next to last row. Extra unlocks
at depth 240 and later change nothing before 240 (two runs, same file).

I did not check whether Blackwood's own code admits wrong-class pieces in its
break path. The nine 469 boards of `border-mismatch-share` have no ring break
(reported in the review), which suggests record boards are legal.

### 2.5 The model against these measurements

Per node at depth 161, log10 (`out/a2_tail_model.out`):

| | arrivals at 201 | late nodes per arrival |
|---|---|---|
| Model 17 + 5 | -4.98 | 1.92 |
| Model 16 + 4 | -2.63 | 2.84 |
| Model 15 + 4 (the review's regime) | -0.47 | 3.84 |
| Model 14 + 4 | +1.79 | 5.04 |
| Port, five clues, 1e11, two seeds | -6.37, -6.74 | 1.78, 1.73 |
| Port, up to depth 194 only (before the cell above a clue) | -2.97, -3.20 against model -4.34 (17 + 5), -2.73 (16 + 5) | |
| Corrected keying, three runs | -2.81, -2.53, -3.15 | legal 4.77, 4.12, 3.94 |

Three readings, all measured against a model output.

- The port sits between the 17 + 5 and 16 + 5 rows up to depth 194, then loses
  a factor 48 at depth 194 and 40 at depth 205, the cells above the two late
  clues. It is a quota-free engine with clues in its tail.
- The corrected engine's stretch is 2 to 2.7 decades less fertile than the
  15 + 4 row and sits on the 16 + 4 row. Its legal tail per arrival is at or
  above the 15 + 4 row. No single row fits both. The pools at the arrivals have
  an effective interior colour count (one over the sum of squared shares) of
  13.9 and 14.5 in the median (`out/a5_probe_legal.out`).
- Past depth 231 the legal sample falls under the model: at depth 234 the
  legal count per arrival is 10^0.3 to 10^1.05 against 10^2.1 in the 15 + 4
  row, and at 237 it is 10^-2.75 or nothing against 10^0.49. This is on 187
  and 1,123 arrivals whose subtrees are very unequal (2.6 below), so a heavy
  tail and a model error are both possible. Not resolved.

### 2.6 Which is wrong

- **The Track 10 number is right and its scope is narrower than its headline.**
  It measures a quota that enriches three interior colours and removes none.
  "Not worth pursuing for this engine" holds for that keying. It says nothing
  about a record-style engine. JUDGE finding 4 already limits it to "this
  port"; the reason is now known.
- **The review's 92 to 98% is not confirmed either.** Its prefix cost
  assumption holds (measured: 70 to 142 nodes at depths 60..160 per node at
  depth 160, under its 10^3). Its stretch does not: the 15 + 4 row puts 0.34
  arrivals under each depth-161 node, the engine has 0.0007 to 0.003. And most
  of what makes the late share large in this binary is the wrong-class
  placements. Legal late share: 8 to 20%.
- **The level is still not explained.** Rows that bracket the measurement give
  10^19.6 (16 + 4), 10^17.2 (15 + 4) and 10^16.2 (14 + 4) nodes per 470
  (model, prefix cost 10^2, `out/a6_extra.out`). The reported 470 took about a
  month on 32 cores, 10^15.2 to 10^15.9 nodes at 1.9e7 to 1e8 nodes per second
  (reported time, argued rate). Only the most favourable row reaches it.
- **Budget dependence.** Under the corrected keying the share is 0 until the
  opening is passed (about 3e7 nodes here), then jumps. For the one opening
  run to 3e8 the all-node share went 0.66 to 0.62 and the legal share with the
  opening left out went 0.35 to 0.10. One opening, half a decade. A capped
  restart of 5e10 nodes is not reached by this.

### 2.7 Where a record run spends its nodes

For a record-style engine, legal boards, no clue. Measured on two openings at
1e8 to 3e8 nodes; everything about finished boards is extrapolated.

| Stage | Depths | Measured | Grade |
|---|---|---|---|
| Opening | below 60 | about 3e7 nodes, once per restart; 7 of 9 tries not through at 1e8 | measured |
| Head | 60 to 160 | 70 to 142 nodes per node at depth 160; never backtracks above depth 14 to 16 | measured |
| Stretch | 161 to 200 | 27 to 34 nodes per node at depth 160; 0.0012 to 0.0049 arrivals | measured |
| All of 60..200 | | 2.5e4 to 7.8e4 nodes per arrival | measured |
| Tail, legal | 201 to 237 | 8.7e3 to 6.0e4 nodes per arrival; two thirds in row 13, one third in row 14 | measured |
| Tail, wrong-class | 201 to 245 | 1.5e5 to 2.5e5 more nodes per arrival; about 77 to 93% of the wall clock (argued from node counts: a break-path node costs about 1.3 microseconds, a strict one 25 nanoseconds) | measured, this binary |
| Last 19 cells | 237 to 255 | no legal node seen | not measured |
| Boards per arrival | | 10^-13.4 (15 + 4), 10^-11.1 (14 + 4), 10^-15.5 (16 + 4) | model |

A record run is a stream of cheap prefixes, one arrival for every 25,000 to
80,000 nodes, each followed by a legal tail of 9,000 to 60,000 nodes, and a
finished board once in 10^11 to 10^15 arrivals.

## 3. Job 2: what would make a 471 cheaper by orders of magnitude

### 3.1 The yield chain

- **Nodes per arrival** (measured, legal): 3.8e4 to 1.3e5 with the tail, of
  which 54 to 90% before depth 201.
- **Arrivals per board** (model): 10^13.4 at 470 and 10^15.7 at 471 in the
  15 + 4 row. One point costs a factor 10^2.36 in boards per arrival in every
  row (`out/a2_tail_model.out`, part 3; `out/a6_extra.out`).
- **Where the arrivals are lost** (model, 15 + 4, per arrival): gain 10^3.1 up
  to depth 229 as breaks unlock, lose 10^4.8 from there to depth 240, lose
  10^11.7 in the last row.
- **Cost of the next point.** Boards per arrival fall by 10^2.36. Nodes per
  arrival fall by at most the tail's share, a factor 1.1 to 1.9. So a 471
  costs 120 to 230 times a 470 for this engine (argued from a measured split
  and a model ratio). In the model alone the factor runs from 1 (14 + 4) to
  220 (16 + 4), because it depends on where the nodes are. Anchored on the
  reported 470, a 471 is 10^17.3 to 10^18.3 nodes.
- **The dominant factor** is arrivals per board. It is 13 to 15 decades. Nodes
  per arrival is 5 decades and is already balanced between head and tail.

The record engine's model price for a 471, used as the reference below:
10^18.0 to 10^18.1 (15 + 4, Blackwood's list without its first entry, prefix
cost 10 to 100 nodes, `out/a4_price.out` P1).

### 3.2 Are arrivals the right currency: can a prefix be ranked

Probe at depth 201 (`--probe 201`, patches 1 and 4): for each arrival, the
colour census of the unused pieces and of the open frontier, the legal nodes
below it and the deepest legal depth below it. 187 arrivals for one opening,
1,122 for the other (`out/a5_probe_legal.out`).

**Arrivals are very unequal (measured).** The top 10% of arrivals hold 85% and
94% of the legal late nodes. The log10 of the legal subtree has a standard
deviation of 1.36 and 1.04 decades. Median 1,242 and 63 nodes, mean 59,473 and
8,343.

**The pool predicts some of it (measured).** Predictor G10: log10 of the
probability that a uniform matching of the tail's half-edges, with the
arrival's own colour counts, has at most 10 two-coloured pairs (exact,
`model.interior_dist`; the ring part uses an even split by facing). Its spread
over arrivals is 0.88 and 0.64 decades. Spearman correlation with the log of
the legal subtree: 0.40 and 0.16; with the deepest legal depth: 0.38 and 0.21.
The top half by G10 holds 75% and 88% of the arrivals that reach legal depth
230; the top fifth holds 42% and 42%.

**A natural experiment on colour exhaustion (measured).** Arrivals grouped by
the number of interior colours with no half-edge left:

| Opening | colours gone | arrivals | mean legal subtree | reach legal depth 230 |
|---|---|---|---|---|
| 4, 16, 17 | 0 | 706 | 3,357 | 1.7% |
| 4, 16, 17 | 1 | 393 | 16,445 | 8.1% |
| 4, 16, 17 | 2 | 23 | 22,932 | 26.1% |
| 3, 12, 13 | 1 | 162 | 32,651 | 14.8% |
| 3, 12, 13 | 2 | 25 | 233,278 | 48.0% |

Each colour that is fully gone multiplies the chance of reaching depth 230 by
3 to 5 and the legal subtree by 1.4 to 7. The model's figure for one colour is
a factor 10^1.2 in tail nodes per arrival. The colours in question are mostly
the two privileged interior ones, of which the quota lets 2 sides stay out.
Depth 230 is a proxy. No board was finished.

**What ranking buys.** A filter keeps the arrivals it likes and skips the tail
of the others. It still has to make every arrival. Its gain is capped at
1 + (tail cost per arrival) / (head cost per arrival). Measured legal ratio:
0.11, 0.53 and 0.84, so the cap is 1.1 to 1.8. With the measured correlations
the table of `out/a4_price.out` P2 gives 1.0 to 1.2. A perfect predictor at
depth 201 buys less than a factor 2. The same cap applies at depth 160: head
70 to 142 nodes, everything after 40 to 180 legal nodes per node at depth 160.

So arrivals are not a currency to hoard. The information in the pool is worth
something only if it is used to steer the head toward better pools, before
the nodes are spent. That is what a quota is.

### 3.3 Why steering can buy decades, and when it cannot (argued)

Restricting the search to prefixes of some kind s keeps a share q(s) of the
boards and a share f(s) of the prefixes. Yield per prefix moves by q/f. The
record quota keeps roughly 10^-11 of the boards (hand estimate: the 50 joints
of two colours and the 12 joints of a ring colour must all fall in the first
160 cells) and, by the model rows, raises boards per prefix by 10^10.2
(17 + 5 against 15 + 4, `out/a6_extra.out`). It can afford to throw boards
away because the list reaches 10^26.7 boards at 470 and 10^24.3 at 471 (model,
17 + 5). After the quota about 10^13 boards at 471 are left to spend.

Two consequences.

- More restriction of the same kind has room at 471. Each further decade of
  yield costs a decade of boards, and there are about 13 to spend. The limit
  is not the count. It is whether the search can walk the restricted prefixes
  without wandering outside them. The measured opening (3e7 nodes below depth
  60, most tries not through at 1e8) is the price of the first restriction.
- At 480 the expected count is about one board with five clues and about
  14,702 with the starter clue only. A restriction keeps a given board with
  probability q and makes the search cheaper by q/f. For the record quota the
  hand estimate of q is 10^-11 and the model's gain is 10^10.2: what is won in
  cost is lost in the chance that the board is there. Colour steering does not
  bear on a full solution.

### 3.4 The candidates

Model prices are log10 nodes per 471 board in the 15 + 4 row, against the
record engine's 18.0 to 18.1. "Cap in this engine" is what the measured split
allows. Odds are mine.

| # | Candidate | Model price | Factor against 18.0 | Cap in this engine | Assumptions | P(10 times cheaper 471 on the real pieces) | P(passes 470 or five-clue 466) | P(bears on a full solution) |
|---|---|---|---|---|---|---|---|---|
| 1 | Finish and stage the colour exhaustion: all 124 privileged sides by depth 176, one more interior colour by depth 200 | 16.3 to 16.6 with a perfect ramp; 22.6 to 26.3 if arrivals are only filtered | 25 to 50 cheaper, or 10^4.6 to 10^8 dearer | none known | even split of the pool; the ramp costs nothing in the head; list unchanged | 0.12 | 0.03 | below 0.01 |
| 2 | Re-tuned unlock list | 17.8 to 17.9 | 1.7 | 1.1 to 1.9 | model's late-heavy regime | 0.02 | 0.01 | 0 |
| 3 | Any device that acts only after depth 201: score bound, exact tail solver, splitting inside the tail | 17.05 to 17.15 at best (perfect knowledge) | 9 at best | 1.1 to 1.9 | breaks released at 201 under the oracle; cost of the device zero | 0.03 | 0.005 | 0 |
| 4 | Filter or rank arrivals by the pool statistic | 17.9 to 18.0 | 1.0 to 1.2 | 1.1 to 1.8 | measured correlation 0.16 to 0.40 | 0.01 | 0 | 0 |
| 5 | Perfect knowledge from depth 181, or 161 | 15.4 to 15.6, or 13.0 to 14.0 | 300, or 10^4 to 10^5 | not capped | a predictor of the tail with 75 to 95 cells left; none exists; Track 8 reports its bound loosens with size | 0.03 | 0.01 | 0.01 |
| 6 | Better generator of arrivals: multilevel splitting, sequential Monte Carlo | not priced | at best 1 / (late share) | 2.2 to 10 | arrivals for free | 0.02 | 0.005 | 0 |
| 7 | Tail first: build the last 56 cells from the full pool, then the head exactly | at least 36.9 | 10^19 dearer | | 10^27.9 tails per board, each a 200-cell exact search of at least 10^9 nodes | below 0.005 | 0 | 0 |
| 8 | Two-sided meet on an interior row | about 37 | 10^19 dearer | | the meeting row closes three joints per cell: 10^19.1 fewer perfect fills than a rim row | below 0.005 | 0 | 0 |
| 9 | Breaks in a 7 x 8 interior pocket | about 21.7 | 10^3.7 dearer | | 127 joints for 56 cells against 109 | below 0.005 | 0 | 0 |
| 10 | Frame placed last | above 40 | | | each head is a complete 14 x 14 interior: 10^26.3 nodes each, 10^-34.6 perfect frames each | below 0.005 | 0 | 0 |

Sources: `out/a4_price.out` (P1 rows 1, 2, 3, 5; P2 row 4; P3 row 1; P4 row
7; P5 rows 9, 10), `out/a6_extra.out` parts 4 and 5 (rows 8, 10).

Notes on the rows.

- **Row 1** is the only one with decades in it and the only one with support
  on the real pieces (the natural experiment of 3.2). Against it:
  Blackwood reports "eliminating four colours early instead of three (no
  gain)" (`solver.mdx`, reported, msg 10056). The model says why early is
  hard, and the piece set agrees (measured, `out/a8_piece_counts.out`): the
  tail from depth 160 needs 70 interior pieces, and the pieces free of three
  given interior colours number 80 to 102 (median 90), of four 53 to 82
  (median 67). So three interior colours by depth 160 leave little choice and
  four fit only for a few quadruples. At depth 201 the tail needs 34, and 34
  to 64 pieces are free of five given colours. The model also says that the share of
  plain arrivals that happen to have used a third colour up is 10^-5.9 at
  depth 200 and 10^-11.6 at depth 176, so everything rests on the ramp: a
  quota that prunes early. I have no price for a real ramp. A second ring
  colour is tight: 19 to 23 border pieces are free of two given ring colours
  (measured) and the tail at depth 201 holds 19, so about 34 of the 37 border
  cells of the head would have to take pieces that carry one of the two
  colours (argued).
- **Rows 2 to 4** are constants. In the model's late-heavy regime they looked
  larger (list 15 times at 470, bound 5 to 9 times). The measured engine is not
  in that regime once illegal nodes are removed.
- **Row 3 and Track 8.** A perfect late bound removes at most the legal late
  nodes: 8 to 46% of the run. Track 10's reading of the score-bound line
  survives, for a different reason than the one it gives.
- **Row 6.** Track 10's M1b found that uniform resampling dies at depth 36 to
  48 under the quota. A generator would have to solve the opening first.
- **Rows 7 to 10** need no measurement. The first-moment count of the tail or
  of the meeting row is exact for the ensemble and the margins are 4 to 20
  decades.
- **Where the breaks go.** The unlock list is already a choice of where breaks
  go along the scan, and re-optimising it is row 2. Breaks before depth 196
  never enter the model's best lists in this regime. A run with five extra
  unlocks from depth 176 (measured, `out/a1_hist.out` D) spent 99.6% of its
  post-opening nodes late and never left one depth-176 prefix.

### 3.5 Two things that are not candidates but are worth more than the constants

For the repo's own Blackwood-style engines, not against the record:

- **Key the quota on one ring colour and two interior colours.** Measured: a
  factor 450 to 4,600 in nodes per arrival, deepest depth 243 to 245 instead of
  209 to 220 at 1e8 nodes. Which triple is best is open (two of six triples
  got through the opening at seed 1).
- **Refuse wrong-class pieces in the break path.** Measured: 1.75 to 2.7 times
  fewer nodes at 1e8 to 3e8. In seconds the saving is larger, about 77 to 93%
  of the run (argued from node counts), since that path scans every piece at
  each node.

Both are prerequisites of any measurement on a record-style engine.

## 4. The experiment I would run first

**Paired staging test, legal boards, corrected keying.** It also redoes M1
for a record-style engine on more than two openings, as its control arm.

- Engine: `engine-copy/` plus two changes: wrong-class and wrongly turned
  pieces refused in the break path; a second quota array.
- Openings: six (privileged triple, seed) pairs that pass depth 60 within 1e8
  nodes. Tonight 2 of 9 passed, so scan about 30 pairs, a few seconds each.
- Arms, same seed, so the three searches are identical down to depth 160:
  A0 the published quota (119 of 124 by depth 160). A1 all 124 privileged
  sides on the board by depth 176. A2 as A1, plus every side of a third
  interior colour (the one with fewest sides left at depth 160) on the board
  by depth 200, linear ramp.
- Budget: 1e9 legal nodes per arm and opening, 18 runs. From tonight's
  timings 1e9 legal nodes take 2 to 5 minutes (argued), so the whole test fits
  in 2 core-hours.
- Statistic: D230 = legal nodes at depth 230 or more, per legal node spent at
  depth 60 or more, summed over the six openings. Report also legal arrivals
  per node, legal late nodes per arrival, and the legal profile by depth
  against the 15 + 4 and 14 + 4 rows.
- Prediction: the natural experiment gives a factor 3 to 5 per colour finished
  in the chance of reaching depth 230; the model gives 10^1.3 to 10^1.4 more
  nodes per arrival at depths 228 to 231 for one colour fewer.
- **Kill line, written now:** if D230(A1) / D230(A0) is below 3 and
  D230(A2) / D230(A0) is below 3, staging is dead on the real pieces and
  Blackwood's "no gain" extends to late staging. It goes on only if one arm
  reaches 10.
- Uninformative if fewer than four openings finish all three arms, or if A0's
  D230 rests on fewer than 100 nodes in total. Null for the noise: D230 of the
  first half of each A0 run against its second half (tonight the two halves
  of the 3e8 run had the same mean legal depth, 210.4 and 210.4).
- What a pass would not show: a board. D230 is a proxy 26 cells from the end.
  No legal node passed depth 237 tonight and extra late unlocks did not help,
  so a ladder of finished boards is out of reach of 5 core-hours.

## 5. What I could not check

- Blackwood's code. Which colours 13, 16 and 10 are in the repo's numbering,
  whether his break path admits wrong-class pieces, his interleaving of border
  cells after depth 180 (the port has none). "One border colour and two
  interior" is the site's statement.
- Five clues. The corrected keying never left the opening with the clues on
  the board (three tries, 1e8 nodes). Everything measured here is no-clue.
  Nothing here is evidence about a five-clue 467.
- More than two openings, and budgets above 3e8 nodes. Every share in 2.3 and
  2.4 rests on two openings, one privileged triple each, one seed.
- The model's boards per arrival. No board was finished, no legal node passed
  depth 237, and the legal sample is 2 to 3 decades under the model from depth
  234. Every price per board in section 3 is a model figure.
- The absolute level. This engine with the model's yield prices a 470 at
  10^16 to 10^20; the reported cost is 10^15.2 to 10^15.9.
- The share of boards the quota keeps (10^-11) is a hand estimate, not a
  script output. The count of boards left to spend at 471 inherits it.
- A real ramp. Row 1 has a best case and a worst case 6 to 10 decades apart.
- Whether tonight's other tracks ran `blackwood_bt` with its default colours.
  `bw_portfolio.log` at the repo root shows a stage 4 run at deepest depth 205
  after 3e8 nodes, which is what the default keying does (read, not traced).
- Restart racing across openings. The two openings differ by a factor 4 in
  the chance of reaching depth 230 (19% and 4.5% of arrivals), on different
  colour triples. Not priced. The explored map lists restart portfolios as
  saturated.

## 6. For the lead: findings that touch existing files

I edited none of these.

1. `real-puzzle-profile/RESULTS.md`, headline and "Why the share is small":
   the reading "the score-bound line is not worth pursuing for this engine"
   and "whether Blackwood's engine behaves the same is not measured" should
   say that the port privileges three interior colours (150 sides) against a
   quota of 119, so no colour is used up. The numbers stand.
2. `partial-match-regime.REVIEW.md`, error 1 and section 5, second bullet:
   "92 to 98% late" is a property of the 15 + 4 row. Measured for a legal
   engine with the corrected keying: 8 to 20%.
3. `blackwood_bt.rs:1180-1185`: the default privileged colours were chosen on
   depth at 30 million nodes and are three interior colours. A fix or an
   issue is the owner's call.
4. `bw.rs` and `blackwood_bt.rs`, break path: wrong-class placements are
   admitted at the price of one break. If banked boards must be physical,
   these nodes are waste.
5. The explored map has no row for the keying of the quota or for staging
   further colours.

## 7. Files

| File | What |
|---|---|
| `run_all.sh` | every command, in order |
| `patch_engine.py` to `patch_engine4.py` | the four instrumentations of the engine copy; search unchanged |
| `engine-copy/` | the Track 10 engine, copied at `dc71ad5`, then patched |
| `data/official.csv` | the port's piece file, copied |
| `data/official-noclue.csv`, `data/official-centre.csv` | the same with the hint columns zeroed (all five, or all but the centre); the second is unused |
| `a1_hist.py` | where the nodes are, per run |
| `a2_tail_model.py` | the review's tail model against the histograms |
| `a3_probe.py`, `a5_probe_legal.py` | arrivals at depth 201: predictor, natural experiment |
| `a4_price.py` | model prices of the candidates |
| `a6_extra.py` | model rows at the measured prefix cost, privileged sides left, reported compute, two closed forms |
| `a7_legal_hist.py` | all nodes against legal nodes |
| `a8_piece_counts.py` | pieces free of given colours on the official set |
| `out/` | raw engine rows (`e*.jsonl`), probe rows (`probe*.jsonl`), script outputs (`a*.out`) |
