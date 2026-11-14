# Referee report: "Where a record is paid"

Night 2026-11-12. Memo under review:
`research/program/night/2026-11-12/memos/where-a-record-is-paid.md`. Referee
scripts, engine copy and outputs:
`research/program/night/2026-11-12/memos/where-a-record-is-paid-review/`
(`v1` to `v4`, `run.sh`, `out/`, `rerun/`).

Scope. Track 13 is re-measuring the memo's numbers on more openings. This
report covers what a re-measurement cannot settle: definitions, reasoning, the
model, and what the published record engine does. I ran a few engine runs
only where a control was missing from the memo's design (wrong clue setting,
wrong colour triple), never to redo its numbers.

Compute: about 9 core-minutes. Engine runs 7.8 core-minutes (14 runs, none
above 1e8 nodes, two at a time, from my own copy in `engine/`; one extra 1e8
run whose output I lost to a shell mistake is counted), Python about one
minute, three builds. No existing file was edited and no git command was run.
Web: the public repository `jblackwood345/EternityII_Solver` (file tree, five
historic versions of two files, commit list), read into a scratch folder and
not vendored. No project or owner name was sent in any request.

Grades: **recomputed** (my code), **rerun** (the memo's code, same output),
**read** (a file, path given), **run** (my engine run, file in `out/`).

## 1. Verdict table

| # | Claim | Verdict | P(true as stated) | Reason in one line |
|---|---|---|---|---|
| 1 | The Track 10 port privileges 22, 21, 20 (three interior colours, 150 sides) against a quota that stops at 119, so no colour is used up; the record engine removes one ring colour and two interior ones | holds with a correction | 0.9 for the port, 0.3 for "124 sides" | Port and census verified from source. The record triple is 122 sides, not 124: Blackwood's {13, 16, 10} is this repo's {5, 20, 7}. The memo never ran it |
| 2 | "The keying alone moves the cost of an arrival by a factor 450 to 4,600" | wrong | 0.02 | The baseline had five clues and 1e10 nodes, the test had no clue and 1e8. Port's own keying with no clue: 3.3e5 to 4.2e5 nodes per arrival, not 3.5e7 to 1.1e8. Like for like the factor is 3 to 20, and 0.8 to 2.5 once every node of the run is charged |
| 3 | 54 to 66% of all nodes are late in a record-style engine | not established | 0.15 | True of two runs. Published triple, no clue, three seeds: 25%, 53%, 88%. Published triple with the centre piece pinned, as the record engine has it: 3% and 5% |
| 4 | Legal late share is 8 to 20% (10 to 46% without the opening); the cost is spread, half to nine tenths before depth 201 | not established | 0.2 | Published triple, three seeds: 3.6%, 8.2%, 53%. With the centre piece pinned: 0.4% and 1.4% on 24 and 25 arrivals. The measured range is 0.4% to 53% |
| 5 | "Neither 4.5e-7 nor 92 to 98% describes a record-style engine" | holds | 0.8 | Follows from row 4 whatever the exact share. But the two shares the memo sets against them are not comparable with either (section 3) |
| 6 | Counting legal boards only is the right accounting; the port's break path admits wrong-class pieces | holds | 0.95 | Settled from source: the published engine cannot place a wrong-class or wrongly turned piece, with or without a break |
| 7 | "No clue on the board, because the 470 is a no-clue record" | wrong | 0.02 | The published engine pins piece 139 in the centre (`Program.cs`, line 177 and the `start` lists). The memo built the centre-only piece file and did not use it |
| 8 | Anything that acts only after depth 201 (bounds, exact tail solvers, list tuning, filters) is capped at 1.1 to 1.9 times | wrong | 0.15 | The formula holds only if the device leaves the set of reachable boards unchanged. The memo's own model row with 12% late nodes gives 8.7 times for perfect knowledge from 201, of which 7.6 is more boards |
| 9 | A better generator of arrivals is capped at 2.2 to 10 times | holds with a correction | 0.35 | The formula is right for arrivals of equal quality. With the legal share anywhere from 0.4% to 53% the cap runs from 1.9 to about 250 |
| 10 | "A 471 costs 120 to 230 times a 470 for this engine" | not established | 0.15 | It prices the dearer of two ways to cut an unlock. Cutting the last one costs 59 to 66 in every model row at any split. The earlier ruling (one to two and a half decades) stands |
| 11 | The model prices of the candidate table reproduce | holds | 0.97 | All eight scripts rerun byte for byte |
| 12 | The tail model is the exact first moment (no joint-spreading error) | holds | 0.95 | Its joint law equals the price review's `rv_lib` to four decimals at every point tried. The spreading error of the two earlier memos is not here |
| 13 | Rows 7 to 10 (tail first, two-sided meet, pocket, frame last) are dead by 4 to 20 decades, "exact for the ensemble" | holds with a correction | 0.9 that all four are dead | Those rows use independent joints at 1 over the colour count, not the exact law. Their figures are off by 3 to 5 decades. The direction survives and the pocket gets worse (5.8 decades, not 3.7) |
| 14 | One more colour used up between depth 160 and 200 is worth 25 to 50 times with a free ramp; it is "the one lever with decades in it" | not established | 0.15 that any real ramp buys 10 times | A best case with zero head cost, priced on the part of the model the sample contradicts, against a natural experiment 4 to 8 times weaker than the model and against Blackwood's own "four colours: no gain" |
| 15 | The absolute cost of a 470 is unexplained (model 10^16 to 10^20, reported 10^15.2 to 10^15.9) | holds | 0.9 | The memo's own statement. Two more reasons not to expect a match: the reported run had the centre piece pinned, and its node counter counts backtracks |
| 16 | Opening: about 3e7 nodes once per restart, 7 of 9 tries stuck at 1e8 | wrong | 0.1 as a property of the record keying | The published triple passed depth 60 on 3 of 3 seeds, against 2 of 9 for the memo's triples, five of whose six break Blackwood's rule (ring colour on a corner piece). Small numbers, cause not established |
| 17 | Recommendation: key the quota on one ring colour and two interior colours | holds with a correction | 0.7 that it is the right fidelity fix, 0.02 for its stated gain | The triple to use is 5, 20, 7, and `blackwood_bt` already takes `--priv`. What it buys is more fertile arrivals, not cheaper ones |
| 18 | Recommendation: refuse wrong-class pieces in the break path | holds | 0.9 | It is what the published engine does. Saving in nodes with the published triple: 1.3 to 3.8 times (three seeds), port-specific |
| 19 | Colour steering does not bear on a full solution | holds | 0.8 | Sound first-moment argument on a hand estimate; nothing here contradicts it |

## 2. The most serious error

**`where-a-record-is-paid.md`, lines 126 and 135 to 136** (also lines 26 to 29
of the answer and lines 398 to 401 of the recommendations).

The table of section 2.3 sets "port as committed (22, 21, 20), five clues,
1e10" (3.5e7 to 1.1e8 nodes at depths 60 to 200 per arrival) against two
no-clue runs at 1e8 (2.5e4 to 7.8e4) and concludes "the keying alone moves the
cost of an arrival by a factor 450 to 4,600". Three things changed between the
two rows: the keying, the clues (five against none) and the budget (1e10
against 1e8). The memo has no run of the port's own keying without clues.

I ran it (`out/A-*.jsonl`, same binary, same no-clue piece file, same unlock
list, 1e8 nodes, seeds 1 to 3):

| Keying | Clues | Seeds | Nodes at 60..200 per arrival | All nodes of the run per arrival | Arrivals in 1e8 | Deepest |
|---|---|---|---|---|---|---|
| 22, 21, 20 (port default) | five, 1e10 (Track 10) | 1 to 6 | 3.5e7 to 1.1e8 | | 87 to 285 | 207 to 220 |
| 22, 21, 20 (port default) | none | 1, 2, 3 | 3.9e5, 4.2e5, 3.3e5 | 3.9e5, 4.2e5, 3.4e5 | 256, 237, 291 | 231, 238, 243 |
| 3, 12, 13 and 4, 16, 17 (memo) | none | 1 | 7.1e4, 2.5e4 | 5.3e5, 4.0e5 | 187, 252 | 243, 245 |
| 5, 20, 7 (published triple) | none | 1, 2, 3 | 2.1e4, 1.1e5, 1.2e5 | 1.7e5, 2.0e5, 4.4e5 | 596, 510, 230 | 245, 243, 242 |
| 5, 20, 7 (published triple) | centre | 1, 2 | 2.1e6, 8.6e5 | 4.2e6, 4.0e6 | 24, 25 | 241, 242 |

Reading:

- Removing the clues is worth a factor 90 to 330 in nodes per arrival for
  the port's own keying (with the budget going from 1e10 to 1e8 at the same
  time). That is most of the memo's 450 to 4,600.
- The keying itself is worth 3 to 20 on the nodes at depths 60 to 200, and
  0.8 to 2.5 when every node of the run is charged (the corrected keying
  spends far more nodes after depth 201).
- Charged for its seven stuck tries, the memo's keying made 439 arrivals in
  9e8 nodes, one per 2.1e6. The port's default made one per 3.8e5. At this
  budget the memo's "corrected" triples are five times dearer per arrival
  than the keying they correct.
- "Deepest depth 243 to 245 instead of 209 to 220 at 1e8 nodes" (line 400)
  compares against five-clue runs. With no clue the default keying reaches
  231 to 243.

What the keying does change is the tail under each arrival: 5e3 to 2.3e4
legal late nodes per arrival with the published triple against 1.3e3 to 2.6e3
with the default (run). That is the model's point (a tail with fewer colours
is more fertile), and it is a statement about yield per arrival, which nobody
has measured. The memo's headline says cost.

Fix: delete the factor 450 to 4,600 and the "deepest depth" comparison;
replace row 6 of the table by a no-clue control; state that the measured
effect of the keying on nodes per arrival is 3 to 20 on three seeds and that
its effect on boards per arrival is a model figure.

## 3. Claim by claim

### 3.1 The reconciliation, from source

**The port** (read: `wt-t10/.../real-puzzle-profile/engine/src/main.rs:322`
and `bw.rs`, `run.sh`, `PREREG.md:31` and `49`). Privileged colours
`[22, 21, 20]`, quota `build_quota(1.0)` reaching 119 at depth 160 and held
there, five clues from the piece file. Track 10's own PREREG lists "Blackwood's
own piece numbering and privileged colours" among what the port lacks, and
JUDGE finding 4 says "no Blackwood colours, quota, scan or restarts". So the
qualitative point was already on file; the memo adds the side counts.

**The repo's `blackwood_bt`** (read:
`research/experiments/single-core-benchmark/engine/crates/bench-audit/src/bin/blackwood_bt.rs:1178-1185`).
As cited: the default is `vec![22, 21, 20]`, chosen in an A/B at 30 million
nodes against {13, 16, 10} read as repo labels and against {12, 14, 15}. All
three are triples of interior colours. The comment's own diagnosis is that the
literal labels "transfer badly".

**The census** (recomputed, `v1_colour_map.py`, from the piece file the port
loads). Ring colours 1 to 5 with 24 sides each; interior colours 6 to 10 with
48 and 11 to 22 with 50; no ring colour on an interior piece. 22 + 21 + 20 is
150 sides on 113 pieces. The memo's census is right.

**The published engine's triple** (recomputed). Blackwood's piece table and
the repo's piece file list the same 256 pieces in the same order; one colour
bijection makes all 256 agree up to rotation:

`1->1, 2->6, 3->11, 4->16, 5->4, 6->9, 7->14, 8->19, 9->2, 10->7, 11->12, 12->17, 13->5, 14->10, 15->15, 16->20, 17->3, 18->8, 19->13, 20->18, 21->21, 22->22`

His `side_edges` {1, 5, 9, 13, 17} map onto the repo's ring colours 1 to 5.
So:

| Version of `Util.cs` | `heuristic_sides` (his labels) | Repo labels | Sides |
|---|---|---|---|
| 2020-09-04, first public | 6, 18, 1 | 9, 8, 1 | 48 + 48 + 24 = 120 |
| 2020-09-06, the 469 era | 17, 2, 18 | 3, 6, 8 | 24 + 48 + 48 = 120 |
| 2020-12-06 to the final commit of 2021-03-30 ("the exact code used to find a 470") | 13, 16, 10 | **5, 20, 7** | 24 + 50 + 48 = **122** |

Every version has one ring colour and two interior colours. The quota reaches
119 at index 160 and is not tested after (`max_heuristic_index = 160`), so at
most 3 privileged sides are off the board there, not 5. The memo's "124
sides" is the size of the triples it happened to try. The repo already holds
the right number: `solver-engine/src/schedule_builders.rs:173` has
`let bw_pool = 122u32;`.

The local digest of the mailing list
(`research/community/digests/0014-2019-01--2021-12.md`, line 136, msg 10076)
gives his selection rule: colours that overlap on pieces and are "absent from
corners and the start piece". In the repo's file colour 5 is the only ring
colour on no corner piece, and none of 5, 20, 7 is on piece 139 (recomputed).
The full archive `messages.jsonl` that `AGENTS.md` points to is not on this
machine (the path resolves to `dev-projects/research/community-exports/`,
which does not exist), so I could not read msgs 10051, 10056 or 10076
themselves, only the digest.

**What else the published engine does** (read: `Program.cs` and `Util.cs` at
the final commit `f544e42`):

1. **No wrong-class piece, ever.** Each cell draws from a list built for its
   class: `corners`, `left_sides`, `top_sides`, `right_sides_with_breaks` or
   `right_sides_without_breaks`, `middles_with_break` or `middles_no_break`
   (`Program.cs`, the `master_piece_lookup` loop). Border lists are filtered
   by a fixed rotation (0 bottom, 1 left, 2 top, 3 right), so a border piece
   cannot be turned inward either. This settles the memo's open question
   (line 158): the answer is no.
2. **What a break is.** `Get_Rotated_Pieces(piece, allowbreaks)` admits a
   piece whose left or bottom side mismatches, one of the two and not both,
   and only if the piece's own mismatching side is not a ring colour
   (`side_breaks == 0`, `Util.cs:80`). So a break is always on an interior
   colour, never on a joint between two border pieces. The bottom row, the
   left column and the corners are never built with breaks.
3. **No isolation rule.** The code enforces one break per placed piece on its
   own two closing joints. It has no test that two breaks do not share a
   piece. The repo's `blackwood_bt.rs` (lines 520 to 532) and the port forbid
   any cell from touching two broken joints, which is stricter.
4. **The centre piece is pinned.** Piece 139 is removed from the middle
   pieces and placed at row 7, column 7 in rotation 2, with dedicated lists
   for the cells below and to the left of it.
5. **The last 80 cells are not a row scan.** `Get_Board_Order`: rows 0 to 10
   in row scan (176 cells), then the five remaining rows are filled as a
   5 by 5 block (176 to 200), then seven columns of five cells each, bottom to
   top (201 to 235), then a 5 by 4 corner block. The unlocks 201, 206, 211,
   216, 221 are the first cells of five successive columns. The site page's
   "row scan all the way to depth 180, then border pieces interspersed" is a
   loose description of this.
6. **Its node counter counts loop iterations**, forward and backward
   (`node_count++` at the top of the loop), with a cap of 5e10 per attempt.

So the port, and `blackwood_bt` with it, differs from the published engine in
six ways: the triple, wrong-class placements, the isolation rule, the order
of the last 80 cells, the clue setting the memo chose, and the quota test
(the port asks for `quota[d]` with d pieces placed, the original with d + 1).

**Verhaard's `eii`.** The site page
(`web/content/research/lab/experiments/louis-verhaard/eii.mdx`, lines 155 to
196 and 262 to 266) describes a per-depth slip array and comb-search fill
orders; its "piece quota" tables survive only inside the binary. Nothing on
the page or in the digests says it keys on three colours. No code of his is
public, so I establish nothing about its keying.

### 3.2 The definitions

**Node.** Same as Track 10: one call of the search on a free cell, counted
before the quota test (same binary, verified in `bw.rs`). Comparable.

**Late, arrival.** Depth 201 or more; a node at depth 201. Comparable with
Track 10. An arrival is counted in the port's row scan, where depth 201 is
the ninth cell of row 12. In the published engine cell 201 is the first cell
of a column in a five-row band. The two "arrivals" are different boards.

**Legal.** Grey exactly on the rim. Since the published engine can only build
such boards, the legal tree is the right object. One residual difference:
the port's legal tree still allows breaks on joints between border pieces and
applies the isolation rule.

**Opening.** "Depths below 60" is a bin of the histogram, so it counts every
node below depth 60 in the run, not the nodes spent before depth 60 is first
passed. In the memo's 3e8 run the bin did not grow after 1e8, so there it is
a one-off. In my centre-clue runs it holds 46% and 76% of the run while
arrivals are being produced. The memo's "once per restart" is a reading of
one run.

**Are the shares comparable?**

| Share | What it is | Comparable with 54 to 66% | Comparable with 8 to 20% |
|---|---|---|---|
| Track 10's 4.5e-7 | all nodes, five clues, 1e10 nodes, default triple, six seeds | No: clues and budget differ. The default triple with no clue at 1e8 gives 0.4%, 1.2%, 2.6% (run). Four decades of the gap are clues and budget | No |
| The review's 92 to 98% | model, legal boards, per quota-satisfying depth-160 prefix, no opening, prefix cost 1 to 1,000 nodes | No: the 54 to 66% counts illegal nodes and the opening | Only the "opening left out" figure (10 to 46%) is the same quantity |

The memo sets the with-opening figure (8 to 20%) against 92 to 98% in its
answer and in its note for the lead (lines 32 to 35, 481 to 483). The
like-for-like figure is the other one, and with the published triple it is
4.6%, 13% and 53% on three seeds (`v3_runs.out`).

**Is the 450 to 4,600 like for like?** No on clue setting, no on budget, yes
on accounting. See section 2.

**What the selection does.** The two openings were the 2 of 9 that passed
depth 60 within 1e8 nodes. Nodes per arrival and every share are therefore
conditional on a good opening and leave out what the seven failures cost. For
an engine that restarts, the unconditional price per arrival at this budget
is 2.1e6 nodes, 30 to 80 times the quoted 2.5e4 to 7.8e4. Second, the
triples were not the record's kind. Five of the memo's six carry their ring
colour on a corner piece (colours 1, 2, 3 and 4 are all on corners;
recomputed), and 3, 12, 13 also has colour 12 on the centre piece, both
against Blackwood's rule. That does not explain the failures by itself: both
triples that passed break the rule, and the one compliant triple the memo
tried (5, 18, 19) was stuck at seed 1. What is on file is 2 passes in 9 tries
for the memo's triples against 3 in 3 for the published one (6e4, 1.7e7 and
1.8e7 nodes in the bin below depth 60). With numbers this small the cause is
open, but "7 of 9 stuck" is not a measured property of the record keying.
Third, both passing runs used seed 1, so they share one shuffle of the
candidate lists.

### 3.3 The model

**Reruns.** `a1` to `a8` reproduce their committed outputs byte for byte
(`rerun/summary.txt`; `a4_price.py` takes 33 seconds).

**The joint law.** `a2`, `a6` part 1 and `a4` P1 and P3 all go through
`r3_sensitivity.tail_tree`, which uses `model.py`'s exact law for the event
"these e joints match, these j break" under a uniform arrangement of the
tail's colour multiset. I compared it with `rv_lib.p_int_table` and
`p_frame_table`, the independent code that tonight's price review validated,
on the tail from depth 160 at 17 + 5, 15 + 4 and 14 + 4 colours, at 5 to 159
joints: largest difference 0.0000 decades (`v2_model.out`, part 5). The error
of the two earlier memos, the full-board match probability spread evenly
over joints, is not in the tail model.

**Where the opposite shortcut is used.** `a4_price.py` P4 and P5 and
`a6_extra.py` parts 4 and 5 price complete fills with independent joints at
1/17, 1/15, 1/5 or 1/4 each. For a region that closes every joint of its
pool this is too pessimistic, because the last joints of each colour are
nearly forced. Recomputed with the exact law (`v2_model.out`, part 4):

| Quantity, 15 + 4 colours | Memo (independent) | Exact |
|---|---|---|
| Perfect completions of the bottom strip from depth 200 | 10^-37.1 | 10^-32.9 |
| Perfect completions of the rim row from depth 240 | 10^-14.3 | 10^-11.4 |
| 7 by 8 pocket against the strip | 3.7 decades worse | 5.8 decades worse |

Rows 7 to 10 stay dead, and the sentence "the first-moment count of the tail
or of the meeting row is exact for the ensemble" (line 385) is not true of
the figures printed.

**The published order.** The memo and the model use a row scan for the last
80 cells. I put Blackwood's order into the same model (`v2_model.out`, part
3). Boards per depth-160 prefix rise by 0.53 decades, the price of a 470
falls by 0.2 to 0.5 decades, the 471 price at 15 + 4 colours moves from
10^18.09 to 10^18.27. The split of the loss changes (from depth 229 to 240
the row scan loses 4.8 decades and his order 0.9; the last 16 cells lose 11.7
and 15.3) but the totals do not. The row-scan simplification is not what is
wrong with the model.

**Does a conclusion rest on the part that is off?** Three parts are off by
the memo's own account: the stretch (the 15 + 4 row puts 0.34 arrivals under
a depth-161 node, the engine has 0.0007 to 0.003), the legal tail past depth
231, and the level. What rests on them:

- Every "arrivals per board" (10^13.4 at 470, 10^15.7 at 471), every model
  price in the candidate table and the 25 to 50 of row 1 are sums over the
  depths 234 to 256, where no legal node was seen and the sample is 2 to 3
  decades under the model at the last depths it reaches. In my runs with the
  published triple the legal count per arrival falls by 3.9 to 4.4 decades
  from depth 231 to 237 in the runs that reach 237 (`v4_profile.out`); the
  15 + 4 row falls by 2.4.
- I tested one explanation: the port's isolation rule, which neither the
  model nor the published engine has. Same opening, rule off
  (`out/E-noiso-*.jsonl`): deepest legal depth 238 against 236, legal count
  per arrival at 236 10^-0.17 against 10^-0.51. The rule is not the cause.
  The shortfall stays unexplained; a mean carried by arrivals too rare for a
  sample of a few hundred is still the first candidate.
- The candidate table takes the 15 + 4 row as its baseline. At K = 10 that
  row has 97% of its nodes late; the memo's own section 2.6 says the engine
  is not in that regime. So the table's reference price is set in a regime
  the memo has just rejected.
- What does not rest on them: the ordering of rows 7 to 10, the direction of
  every colour effect, and the ratio between scores (2.34 to 2.39 decades per
  break in every row).

### 3.4 "471 costs 120 to 230 times 470"

The derivation: boards per arrival fall by 10^2.36, nodes per arrival fall by
at most the late share, so 229 / (1.1 to 1.9). The 10^2.36 is the truncation
that drops the **first** unlock (`BW[1:]` in `a2`, `a4`, `a6`). The other
truncation drops the last one. Recomputed in four model rows
(`v2_model.out`, part 1):

| Row | Boards, first unlock dropped | Boards, last unlock dropped | Cost ratio at late share 8%, 20%, 46%: first dropped | last dropped |
|---|---|---|---|---|
| 17 + 5 | 10^-2.39 | 10^-1.82 | 224, 195, 132 | 66 |
| 16 + 4 | 10^-2.37 | 10^-1.80 | 217, 189, 128 | 64 |
| 15 + 4 | 10^-2.36 | 10^-1.79 | 209, 182, 123 | 61 |
| 14 + 4 | 10^-2.34 | 10^-1.77 | 201, 175, 118 | 59 |

Dropping the last unlock leaves the late nodes as they are and costs 59 to 66
whatever the split. In a head-heavy engine, which is what the memo says it
measured, that is the cheaper cut by a factor 2 to 3.5. This is error 3 of
`partial-match-regime.REVIEW.md` ("the 235 is the truncation that drops the
first unlock; dropping the last costs 64"), made again. The memo's own
candidate table uses a third figure: its reference prices are 10^17.23 for a
470 and 10^18.0 to 10^18.1 for a 471, a ratio of 6 to 7.

The model's range for one point is therefore 60 to 230 by truncation and 50
to 160 with re-tuned lists; the three observed ladders give 17 to 30 in
mid-ladder and 30, 64, 312 at the top step. The earlier ruling stands: one to
two and a half decades, no measurement at 469 to 471. "Anchored on the
reported 470, a 471 is 10^17.3 to 10^18.3 nodes" inherits the same range and
should read 10^17.0 to 10^18.3.

### 3.5 The caps

**"After depth 201: 1.1 to 1.9 times."** The cap is 1 / (1 - late share). It
is the gain from deleting every late node while finding the same boards. It
applies to a sound pruning bound and to a filter. It does not apply to an
exact tail solver or to a change of the unlock list, because those change
which boards are reached. The memo's own outputs show it (`a4_price.out`, P1;
recomputed in `v2_model.out`, part 2):

| Model row | Late share | Cap by the memo's formula | Perfect knowledge from 201 buys | Of which more boards |
|---|---|---|---|---|
| 15 + 4, prefix cost 10^2, 471 | 0.125 | 1.14 | 8.7 | 7.6 |
| 15 + 4, prefix cost 10^3, 471 | 0.043 | 1.05 | 8.0 | 7.6 |
| 16 + 4, prefix cost 10^2, 470 | 0.053 | 1.06 | 3.0 | 2.9 |

Row 3 of the candidate table prints "9 at best" and "1.1 to 1.9" side by side
without noticing that the first is computed in a row whose late share would
cap it at 1.14. The correct statement: a pruning device after depth 201 is
capped at 1 / (1 - legal late share); a device that may place the breaks
anywhere after 201 is worth up to about 3 times at 470 and 8 times at 471
more in boards, on top of that, in the model. That is still under one
decade, so the memo's qualitative reading (no orders of magnitude after 201)
survives in the model. It rests on the model's boards per arrival, which is
the unverified part.

**"A better generator: 2.2 to 10 times."** 1 / (late share) is the right
formula if arrivals come free and are as good as the ones the search makes.
The memo uses the opening-left-out shares here and the with-opening shares
for the other cap; a generator replaces the opening too, so the with-opening
share (8 to 20%, cap 5 to 13) was the consistent choice. The formula also
ignores quality: the memo's own section 3.2 finds the top tenth of arrivals
holds 85 to 94% of the legal late nodes.

**Do they survive the uncertainty of the split?** No. Legal late shares now
on file, no clue unless stated:

| Triple | Seed | Legal late share | Same, opening left out |
|---|---|---|---|
| 3, 12, 13 (memo) | 1 | 0.195 | 0.456 |
| 4, 16, 17 (memo), 1e8 then 3e8 | 1 | 0.090, 0.079 | 0.348, 0.101 |
| 5, 20, 7 | 1 | 0.526 | 0.527 |
| 5, 20, 7 | 2 | 0.036 | 0.046 |
| 5, 20, 7 | 3 | 0.082 | 0.129 |
| 5, 20, 7, centre piece pinned | 1, 2 | 0.004, 0.014 | 0.009, 0.059 |
| 22, 21, 20 | 1, 2, 3 | 0.003, 0.006, 0.006 | same |

The pruning cap runs from 1.0 to 2.1 and the generator cap from 1.9 to 250.
One opening moved from 0.35 to 0.10 between 1e8 and 3e8 nodes. A record run
has 5e10 nodes per attempt and the centre piece pinned; neither condition has
a measurement with more than 25 arrivals. Neither cap is a number the program
can plan on. What can be said: in no run so far is the legal late share above
53%, so the review's 92 to 98% regime has not been seen in this port.

### 3.6 Row 1 of the candidate table

"25 to 50 times cheaper" is 10^18.0 against 10^16.3 to 10^16.6, the
"perfect ramp" column of `a4` P3. The ramp is assumed to cost nothing in the
head. Evidence on that assumption, all against it: the published triple with
the centre piece pinned needs 4e6 nodes per arrival against 2e5 without the
clue (run); a quota scaled to use up three interior colours never left depth
25 in 1e8 nodes (`out/D-*.jsonl`); Blackwood reports no gain from four
colours. The support the memo offers, the natural experiment of section 3.2,
measures arrivals that reach legal depth 230 (3 to 5 times more per colour
gone) and legal subtree size (1.4 to 7 times). Both are node counts, that is
cost. The model's prediction for the same quantity is 10^1.3 to 10^1.4, 20 to
25 times. The real pieces show a quarter to an eighth of the model's effect,
on a proxy 26 cells from the end. The answer's "the one lever with decades in
it" should read "the one lever the model prices above one decade, in its best
case".

The proposed experiment (section 4 of the memo) is sound in shape. It needs
four changes before it is run: the triple 5, 20, 7; the centre piece pinned
in a second set of arms; openings not selected on passing depth 60 (or the
failures charged); and a statistic that is not a node count, since more nodes
at depth 230 is what a dearer tail looks like as well.

### 3.7 The two recommendations, and what the repo already knows

**Re-key the quota.**

- Already in the repo as knowledge: the site page says "one border colour and
  two interior colours" (`joshua-blackwood/solver.mdx:95-96`);
  `schedule_builders.rs:173` has the pool at 122; the digest has the
  selection rule.
- Already in the repo as a switch: `blackwood_bt` takes `--priv c1,c2,c3`
  (`blackwood_bt.rs:1165-1167`). No file, script or `just` recipe passes
  5, 20, 7. `bw_portfolio.log` at the repo root does not record the triple.
- A different correction already in use: `solver-engine`
  (`schedule_builders.rs:29-122` and `165-175`, the same code in the
  `mrv-baseline` and `cut-multiset-pruning` forks) picks the three most
  frequent colours that are on no corner and not on the start piece, which on
  this file gives three 50-side interior colours, and scales the quota by
  pool / 122, so 146 of 150 sides by the end of the ramp, on a depth axis
  remapped past the border ring. That engine does use its colours up. It has
  no ring colour. Its docstring records the same label confusion ("Blackwood's
  literal triple [17, 2, 18] does NOT apply to our color labels"): the triple
  is the 469-era one and maps to 3, 6, 8.
- Nowhere: no row in `research/program/EXPLORED.md` (its only relevant row,
  line 20, lists the whole method as "saturated"), nothing in
  `research/LEDGER.md` beyond a records-page correction (line 503), no topic
  or experiment on the keying. The memo's finding 6.5 is correct.
- Not tried by anyone, including the memo: the published triple in this
  repo's numbering. My five runs with it are the first.

**Refuse wrong-class pieces.** Not documented anywhere in the repo.
`blackwood_bt.rs` defines a rim break as a break on purpose (header, lines 17
to 19) and its comment on banking (lines 996 to 1003) shows the author knew
the greedy fallback could cost physicality, but not that the native break
path can. The published engine never admits such a piece. The recommendation
is right. Its stated saving (1.75 to 2.7 times in nodes, 77 to 93% of the
wall clock) is for the memo's two openings; with the published triple the
node ratio is 3.8, 1.3 and 2.0.

Two more fidelity gaps of the same kind that the memo does not list: the
isolation rule (not in the published code; one paired run shows a small
effect) and the order of the last 80 cells (0.2 to 0.5 decades in the model;
`blackwood_bt` has a `--border-intersperse` flag that I did not compare with
his table).

## 4. Errors found

1. **Lines 126 and 135 to 136, 26 to 29, 398 to 401.** Keying effect measured
   across a change of clue setting and budget. See section 2.
2. **Lines 116 to 117.** "No clue on the board, because the 470 is a no-clue
   record." The 470 engine pins the centre piece; the digest (line 160)
   says the same of the board. Fix: rerun on `data/official-centre.csv`,
   which the memo built and left unused. With it, the memo's two openings
   make 0 and 4 arrivals in 1e8 nodes (`out/C-*.jsonl`).
3. **Lines 22 to 25, 98 to 101, 113, 398.** "124 sides", "120 to 124". The
   record triple is 5, 20, 7 with 122 sides, and the memo's six triples do
   not include it. Fix: use it everywhere.
4. **Lines 36 to 39, 299 to 305, table rows 2 and 3, lines 380 to 382.** The
   cap 1 / (1 - late share) applied to devices that change the reachable
   boards. Fix: restrict it to pruning bounds and filters; for tail solvers
   and list changes quote the model's 3 to 9.
5. **Lines 248 to 253.** 120 to 230 is the dearer truncation. Fix: 60 to 230
   in the model, and the earlier ruling.
6. **Lines 30 to 35, 199 to 204, 481 to 483.** 8 to 20% set against 92 to
   98%. The like-for-like figure is the one with the opening left out, and
   the range on five no-clue openings is 5 to 53%.
7. **Lines 128 to 131, 223, 327 to 328, 417.** The opening and the "2 of 9"
   read as a property of one-ring-two-interior keying. Five of the six
   triples break Blackwood's corner rule and none is the published one,
   which passed on 3 of 3 seeds.
8. **Lines 385 to 387; `a4_price.py` P4, P5; `a6_extra.py` parts 4, 5.**
   Independent joints called exact. Fix: the exact law (already imported in
   the same script), figures in section 3.3.
9. **Lines 338 to 340 against lines 199 to 204.** The candidate table's
   baseline is the 15 + 4 row, which the memo's own measurement rejects.
   Fix: say so in the caption, and give the table at 16 + 4 as well.
10. **Lines 293 to 297 and 360 to 361.** The natural experiment is offered as
    support for a yield gain. It measures node counts and comes out 4 to 8
    times under the model. Fix: report the ratio to the model.
11. **Line 42 to 43.** "The one lever with decades in it" against a table
    entry of 25 to 50 (1.4 to 1.7 decades, best case).
12. **Lines 62 to 64.** "Opening: depths below 60" is a histogram bin, used
    as "once per restart" at line 223.

No arithmetic error was found in any script.

## 5. What the program may rely on tomorrow

1. The published record triple is {5, 20, 7} in this repo's colour numbering:
   ring colour 5 (24 sides), interior 20 (50) and 7 (48), 122 sides, quota 119
   by cell 160. Earlier versions used {9, 8, 1} and {3, 6, 8}, 120 sides. The
   full label map is in `v1_colour_map.out`.
2. The published engine builds legal boards only, takes at most one break per
   placed piece on an interior-coloured side, never breaks the bottom row,
   the left column, a corner or a joint between border pieces, has no rule
   that breaks must not touch, pins piece 139, and fills its last 80 cells as
   a five-row band in columns. Its nodes count backtracks.
3. Track 10's M1 stands as a measurement of the port with five clues. It says
   nothing about the record engine, for at least six reasons now listed.
4. The tail model is the exact first moment and reproduces; the order of the
   last 80 cells moves its prices by under half a decade.
5. In the model, one unlock fewer costs 59 to 66 if the last is cut and 120
   to 230 if the first is cut, in every row.
6. With no clue the port's default keying costs 3.3e5 to 4.2e5 nodes per
   arrival at 1e8 nodes (three seeds). Any claim about what a keying buys in
   cost must be set against that, not against the five-clue figure.
7. A break path that refuses wrong-class pieces is a correct fidelity fix for
   `blackwood_bt` and the port.

## 6. What it must not rely on yet

| Not yet | Why | Cheapest way to settle it |
|---|---|---|
| The keying is worth 450 to 4,600 in cost per arrival | Confounded with clues and budget; like for like it is 3 to 20 | Done here on three seeds; Track 13 can widen it if it adds the default keying with no clue as an arm |
| A legal late share of 8 to 20%, or any single range | 0.4% to 53% across the eight runs keyed on one ring colour and two interior ones; budget-dependent in the one run that was extended | Track 13, provided it uses 5, 20, 7 and a centre-clue arm |
| "After 201: at most 1.1 to 1.9" and "generator: 2.2 to 10" | Wrong formula for solvers and lists; split not pinned | Needs boards per arrival, which needs finished legal boards. None was seen in any run |
| "471 is 120 to 230 times 470" | Dearer of two truncations; no data at 469 to 471 | The score histogram of a long run, as the earlier review asked |
| "One more colour: 25 to 50 times" | Best case of a model whose deep tail the sample contradicts by 2 decades | The memo's staging test with the four changes of section 3.6 |
| Any statement about the record engine's opening | Measured on triples that break its selection rule | Runs with 5, 20, 7 and the centre piece pinned, at more than 1e8 nodes |
| That the port's tail resembles the published tail | Different order, different break rule, wrong-class pieces | Port his `Get_Board_Order` and his break lists; a day of work, then every number above can be redone on the right engine |
