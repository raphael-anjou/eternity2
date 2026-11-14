# Night shift report, 2026-11-12 to 2026-11-13

Lead branch: `night/2026-11-12-lead`. **Nothing was pushed** (owner's rule for
this night), so there are no pull requests: every branch below is local.

Status of this file: final, updated at 05:49. Tracks 0 to 15 are all judged
(eight keep, eight kill). No session is running on any machine.

## The night in five lines

1. **The price of a full solve is now measured on the real pieces:** 10^40.46
   search nodes by row scan with the five clues (Track 10, judged). The model
   that predicted 10^40.49 holds. The gap to any affordable compute is 17 to 20
   orders of magnitude.
2. **No new way to solve the puzzle was found.** Six independent attempts and
   one on the frame all came back negative on their own lens, each with a
   number.
3. **The site's strict five-clue record of 465 is stale.** A public 466 board
   verifies with independent scripts.
4. **The score-bound idea for passing 470 is not supported.** The bound loosens
   with board size (Track 8, judged). How much of a record engine's search it
   could act on is not settled: Track 10 measured 4.5e-7 on a port that a later
   memo shows was keyed differently from the published engines; the memo's own
   figure of 8 to 20% did not survive its referee, who saw 0.4% to 53% across
   seeds and clue settings. Track 13 (judged) confirms the spread.
5. **Of the brief's five method and measurement tracks, none produced a method
   gain.** Four kills, and three useful tools: a 5x faster reference solver,
   resumable long runs, a preregistration guard.

## One line per track

Verdicts are the independent judge's, from the JUDGE.md on each branch.

| Track | Label | Verdict | Headline number | Branch (local) |
|---|---|---|---|---|
| 0. bb-mrv per-node cost | optimization | **KEEP** | Node-identical on 140 gate instances; 5.3x, 6.1x, 5.3x faster at 6x6, 7x7, 8x8 | `night/2026-11-12-t0-node-cost` |
| 1. 9x9 high-cap pilot | measurement | **KILL** (underpowered) | 6 of 8 pairs hit the 1e8 cap on both solvers; no evidence the node ratio keeps shrinking; 27.4 core-hours | `night/2026-11-12-t1-9x9-pilot` |
| 2. A20 blocker eviction | method | **KILL** (this implementation) | 4.7x to 9.2x more nodes than the baselines at 6x6; 5 of 30 solved at 7x7 against 16 and 23 | `night/2026-11-12-t2-blocker-backjump` |
| 3. A21 value order by impact | method | **KILL** | Node ratio 1.075, 0.829, 1.096 at 6x6, 7x7, 8x8; every interval spans 1 | `night/2026-11-12-t3-value-impact` |
| 4. A17 Hall matching on bb-mrv | method | **KILL** | Removes under 1% of nodes (0.992 to 1.000) and is slower wherever it prunes | `night/2026-11-12-t4-hall-on-mrv` |
| 5. Baseline tournament | measurement | **KEEP** | 47 entrants at three rungs; anchors reproduce committed rows, 0 mismatches on 1,310; top two places within run-seed noise | `night/2026-11-12-t5-baseline-tournament` |
| 6. Dead-node census (added) | measurement | **KEEP** | Three idea-slot proposals killed at stage 0 (0.8%, 0.2% and 0 of dead-node weight); one not decided | `night/2026-11-12-t6-dead-node-census` |
| 7. Partial-match ladder (added) | measurement | **KILL** (this pool and budget) | 2 of 5 entrants stall in the 2% to 4% band, 3 needed; no 10x10 or 12x12 setting qualified | `night/2026-11-12-t7-partial-match-ladder` |
| 8. Score-bound gap (added) | measurement | **KEEP** (on the threshold) | Median gap exactly 4 edges at 7x7 with 3 rows left; grows 1, 2, 4 edges with size at 2 rows left | `night/2026-11-12-t8-score-bound-gap` |
| 9. Resumable long runs (added) | optimization | **KEEP** (after one fix round) | Resume is node-identical in every interruption case; overhead 1.008; 2,202 of 2,202 tampered checkpoints refused | `night/2026-11-12-t9-resumable-runs` |
| 10. Real 16x16 profile (added) | measurement | **KEEP** | Real row-scan tree 10^40.457 nodes with five clues (interval 40.440 to 40.473), model 10^40.49 | `night/2026-11-12-t10-real-puzzle-profile` |
| 11. Difference-map bands (added) | method | **KILL** (at this cap) | 0 of 30 solved near critical at 7x7 and 8x8 at every band width (bb-mrv 23 of 30 at 7x7); no band effect at 6x6; no exponent measured | `night/2026-11-12-t11-difference-map-bands` |
| 13. Record-engine keying on the real pieces (added 01:15) | measurement | **KEEP** | Colour keying changes the node cost of reaching depth 201 by 1.31 [0.68, 2.43] with no clue and 0.74 with the centre clue, and is slower per second; a legal break path gives 7.63 [4.87, 14.3] times more legal deep nodes per node | `night/2026-11-12-t13-quota-keying` |
| 14. Five-clue keying on the real pieces (added 02:38) | measurement | **KILL** (this family, this quota) | 0 of 2,040 runs over 680 published-style keyings reach legal depth 201 with five clues; the default keying does, but with a looser quota (confounded) | `night/2026-11-12-t14-five-clue-keying` |
| 15. Five-clue quota curve and port defect (added 03:58) | measurement | **KEEP** | With five clues the share of privileged sides the quota demands decides whether the search reaches depth 201, not the colours: default keying under a tight quota 0 of 16 openings, published-style keyings under a matched quota 12 to 14 of 16 (0.61 [0.29, 1.54] of the default's yield per second). The orientation fix is free (0.983 [0.95, 1.04]) | `night/2026-11-12-t15-five-clue-quota` |
| 12. Cut-keyed replay (added) | method | **KILL** | A matched random key beats the cut key on 30 of 30 instances at 7x7 (0.419 against 0.486 of bb-mrv's nodes); work ratio 1.21 | `night/2026-11-12-t12-cut-keyed-replay` |

Idea slot: `IDEAS.md` exists, with 13 ranked proposals. Tonight's census and
Track 8 already decided four of the top five at stage 0 (see "What to do next").

Tracks 6 to 12 were not in the brief. The owner widened the night in steps:
more machines, autonomy until 08:00, an instruction to aim at solving the
puzzle, and permission for a 16x16 measurement on the real pieces.

## Observations, ranked by bearing on a solution or on passing 470

Grades: **measured** (committed data, judged), **reported** (a source says so,
opened), **argued** (derived, with or without a referee). Full detail and
sources are in `OBSERVATIONS.md`.

1. **The price: 10^40.46 nodes, measured on the real pieces.** Row scan, five
   clues, sampled with a validated estimator. One clue: 10^47.14. No clue:
   10^50.04. The first-moment model matches all three to within 0.03 digits.
   The official piece set is ordinary among 20 boards painted from its colour
   counts. (measured, Track 10)
2. **Where the difficulty sits: the frame.** With the true frame given, the
   interior costs about 10^8.3 nodes by inward spiral. At the row-scan peak 30
   of 39 digits are partial-frame multiplicity. But searching frames first
   costs 10^45.9, because a wrong frame costs as much to refute as the right
   one to confirm. (argued, refereed model; not measured on the real pieces)
3. **No cheap information about the frame was found.** About 20 cheap bits, all
   at the four corners (17.45 exact on the official pieces). 91 more would be
   needed for 1,000 core-years. On the real pieces, propagation sees nothing
   past the corner windows. Open: all-different reasoning and clause learning
   on partial frames. (measured, refereed; small boards say nothing here: on
   6x6 propagation alone refutes 94% to 100% of wrong frames)
4. **A 470 board is not a damaged solution.** Almost all are 246 or more cells
   from any solution. The record hunt and the solve are different problems.
   (argued, memo; bears on agenda entries A6, A9, A14)
5. **Where a record's cost is paid is not settled.** Track 10 measured a
   histogram peaking at depth 161 with 4.5e-7 of nodes late, on a port that
   privileges three interior colours. A memo's claim that the published keying is
   450 to 4,600 times cheaper was refuted by its referee (unlike comparison; 3
   to 20 like for like). The late share varies from 0.4% to 53% of legal nodes
   across seeds and clue settings. Established from public source: the
   published engine keys on this repo's colours 5, 20, 7, builds legal boards
   only and pins the centre piece. (Track 10 measured; referee's runs small;
   Track 13, judged, confirms the spread and measures the keying gain at 0.7
   to 1.3)
6. **Expected boards per score:** 10^34.4 at 470, 10^31.65 at 471, a factor
   10^2.7 per point, as ensemble expectations. The claim that 471 costs 80 to
   235 times 470 is not established. (argued, refereed)
7. **With five clues, quota slack is the lever on this port, not colour
   keying.** The same absolute quota (119 sides by cell 160) lets the default
   keying live (12 of 16 openings reach depth 201) and kills the published
   keying (2 of 16); the same share of sides lets both live (12 to 14 of 16).
   A tight quota kills the default too (0 of 16, never past depth 35). No
   arm beats the default: it is not a gain, it is an explanation of Track 14
   and a parameter nobody has swept. Which part of the curve binds is not
   shown. (measured, Track 15, on this port only)
8. **A five-clue 466 is public and verifies.** The site says 465. (measured,
   scripts in `checks/verify-466/`)
9. **Three families are priced out, not merely untried:** decompositions and
   joins (at most 1 to 2.4 digits), frame-first enumeration (5.5 digits worse),
   a difference-map search (follows the tree at band width up to 1 on easy
   boards, and fails outright near critical at widths 0 to 3: Track 11,
   judged). (argued and refereed, except Track 11 which is measured)
10. **The node metric hides per-node cost.** Plain row-major DFS solves 9x9
   boards in 2 to 6 s where bb-mrv took 6,772 s for one solve; at 8x8 it runs at
   1% to 23% of bb-mrv's time. Seen by a referee, by Track 1's judge and by
   Track 5's judge. (measured in parts, reported in parts)
11. **bb-mrv's single fixed value order sits inside the spread of random
    orders.** Its tables carry run-seed-sized noise that they do not show.
    (measured, Track 5 judge; weaker than the lead first wrote)
12. **The site's own evidence is thin.** Dead-ends page: 1 of 36 entries backed
    by committed data. Explored map: 8 of 40 rows. Theory pages: 3 of 30 wrong,
    3 overstated, all six confirmed by a second referee, who also found 3 of 5
    sampled "sound" pages one grade too kind. Two bibliography errors
    confirmed. (measured by audit, refereed)
13. **Small boards at these budgets are not in the regime of 470 against 480.**
    At 10x10 and 12x12 every engine stalls 5% to 12% below the optimum.
    (measured, Track 7)

Stated odds of a full solution by any path examined: about 1 in a million on
three desktops in a year (argued, by the memos' authors).

## What the referees changed

Every memo that got a second reader lost at least one headline claim. The
pattern is in draft protocol rule P9.

| Memo | Claim that fell |
|---|---|
| Growth law | "No ladder rung is critical" (refuted by enumeration) |
| Partial-match regime | "471 costs 80 to 235 times 470"; "a score bound has little to remove" (self-contradiction) |
| Solve path 2 | Price 10^45.3 (script error; 10^40.66 corrected) |
| Solve path 6 | Price 10^47.7 (same script error); "instances near critical" |
| Literature | "The record moved this month"; "row-major scanning is optimal" |
| Where a record is paid | "Keying alone is worth 450 to 4,600 times" (unlike comparison); "anything after depth 201 is capped at 1.1 to 1.9" |
| Frame information | "The corner bits are already inside the price" (two of four corners); the 6x6 refuter result (one weak refuter) |
| Solve path 5 | "Ranking is worth 25 to 150"; "no error beyond 0.3 orders" (not established; Track 10 later measured 0.03 on the real set) |

The one memo that kept every headline: the theory referee (six rulings, all
confirmed). The frame-information memo kept its conclusion and lost two
supporting statements. Every memo of the night has had a second reader.

Reconciled, pending review: the partial-match referee inferred that 92 to 98%
of a record run's nodes are late; Track 10 measured 4.5e-7. A memo
(`memos/where-a-record-is-paid.md`) finds that neither describes a record
engine: the first is one row of a model, the second a port with a different
colour keying. Its referee agreed with that and refuted the memo's own
headline. Track 13 (judged) measured the keying gain at 0.7 to 1.3.

## The lead's own errors

- Forwarded an unreviewed memo claim to Track 7 as a hypothesis; retracted 45
  minutes later.
- Read path 6's 10^47.7 as a one-clue figure in another convention. It was a
  script error. Corrected.
- Called the random-value-order result a "third sighting" before the judge
  ruled; the judge weakened it.
- Copied "within one edge of the trivial bound" into the status log without its
  restriction to critical colour counts. Corrected.
- Gave a referee the critical colour counts for the wrong board sizes; the
  referee caught it.
- Pointed Track 11 at an over-coloured rung; corrected before any test seed.
- Briefed Track 15 with Track 14's unjudged account of where the port's defect
  acts and with an ambiguous reference arm; corrected by message before its
  runs, after Track 14's judge ruled.
- Read Track 10's late-node share as a fact about record engines and told the
  owner the score bound was "not worth building for the record-style engine".
  The share describes the port that was run. Corrected an hour later. Then
  relayed the next memo's figures before its referee refuted the main one.
- Said twice that no more tracks would start, then started Track 15.
- Set the fallback timer on Track 15's judge at 80 minutes, so a judge held by
  a prompt at 04:26 was found only at 05:45.
- Status time stamps written from memory ran up to 15 minutes ahead until 21:15.

## Operational failures

- **Track 1's runner session was lost** across the lead's context compaction
  and went unnoticed for 50 minutes; a wrap-up runner finished the analysis.
- **Track 11 lost an hour** to a command held by a permission prompt with
  nobody awake.
- **Track 15's judge lost 80 minutes** the same way (a shell loop with a
  nested substitution, held by a prompt). Two sessions lost to this in one
  night: draft rule P13 covers it; an allow-list for the agents' shell would
  remove it.
- **Track 5's wait loop missed the end of its runs** for an hour.
- **No run log on Tracks 1, 2, 3, 4, 5, 6, 7, 8:** the preregistration guard was
  written at 20:41, after they branched. Order rests on commit order and file
  times, which each judge checked. Tracks 9 to 15 have the log.
- **Budget overruns:** Track 5 used 19.1 core-hours against 10; Track 8 used 31
  thread-hours against 10 to 15.
- **Thread oversubscription on machine-c** early in the evening, from a "take what
  is left" rule; replaced by fixed quotas.
- Remote slips, all contained: a file written to /tmp on machine-b (moved and
  deleted), a `pkill -f` that matched its own ssh command, rsync paths broken
  by a zsh modifier, one queue launched with the wrong cap and killed before
  any measurement.
- Track 9's first judgment found a real defect (an altered checkpoint resumed
  silently); one fix round and one rejudge closed it.
- The owner's Touch ID was requested on every ssh call until a shared
  connection was set up.

## What to do next (lead's recommendation, for the owner to decide)

Ranked by bearing on a full solution or on passing 470.

1. **Make the repo's Blackwood-style port faithful before any more record
   runs with it.** Track 13 (judged) and its referee established six
   differences from the published engine: colour keying; non-physical pieces
   admitted in the break path (4 to 11 times fewer legal deep nodes per node);
   an isolation rule the original lacks (1.5 to 1.9 times); scan order and
   border interleaving; the last 80 cells filled as a band; and a defect that
   places a wrongly oriented piece early and strands the run. Keying alone is
   worth about nothing on nodes and loses on wall time. No order-of-magnitude
   route to 471 was found. Track 14 (judged): no published-style keying reaches
   depth 201 with five clues on this port, confounded with quota slack. Track 15 (judged) separates the causes: it is
   the slack. Three concrete steps follow: make the orientation fix the
   default (free, and the fixed search is exactly the legal search); state
   every quota as a share of the keying's sides, never as a count; sweep that
   share with five clues before any record run (one evening on two machines,
   not run tonight). The default keying stays the best arm measured.
2. **Update `/research/records`**: the five-clue 466 verifies. Add a command
   that verifies a board given as pieces and rotations.
3. **Stop ranking methods by node counts on 6x6 to 8x8 boards.** Adopt draft
   rules P7 (work or time), P10 (censored pairs are not ties) and P2 (a growth
   stage). Use plain DFS or the fast solver as the reference above 8x8.
4. **The decision-diagram score bound: undecided.** It loosens with size
   (Track 8). Whether a record engine spends enough of its search in the tail
   for it to matter is not known; two attempts to close the question tonight
   were each overturned.
5. **Correct the site** where the referees found errors: three theory pages,
   two bibliography entries, and an evidence grade on the dead-ends page and
   the explored map (draft rule P3). Sentence-level corrections with line
   numbers are in section 6 of `memos/theory-and-frame.REVIEW.md`, split into
   those its author stands behind and those not yet.
6. **Merge the tools:** Track 0 (fast solver), Track 9 (resumable runs), the
   preregistration guard, the tracker.
7. Ideas not pursued tonight that their authors rated highest: recovering the
   generator's seed; a partial-match ladder built from engines that keep
   searching after a first complete board.

Idea slot after tonight: cavity summaries, band consistency and boundary-word
lemmas are killed at stage 0 (Track 6); the decision-diagram score bound passed
stage 0 on the threshold and is not recommended (Track 8 with Track 10);
cut-keyed partitioning is killed on its own matched control (Track 12); proof-number search is untested.

## Drafts, not applied

- `DRAFTS.md`: AGENDA lines and LEDGER entries for Tracks 0 to 15, and a draft issue on the
  Blackwood-style port, written by
  the plumbers from each JUDGE.md.
- `PROTOCOL_AMENDMENT_DRAFTS.md`: fourteen draft rules, P1 to P14.
- `EVIDENCE_AUDIT.md`: the grade of every dead-end and explored entry.
- No edit was made to EXPLORED.md, PROTOCOL.md, AGENDA.md, LEDGER.md or any
  site page.

## Open questions for the owner

1. Is the BOINC project (466 with five clues, as its page reports) yours or
   known to you? Its boards cannot be downloaded, so they are not verified.
2. May the site pages, PROTOCOL.md and EXPLORED.md be edited from these drafts?
3. Which branches should become pull requests? Nothing is pushed.

## Compute used

This Mac (10 cores), machine-b (12 threads) and machine-c (7 threads) over Tailscale,
from 18:30. No paid service. Per-track figures are in each README.
