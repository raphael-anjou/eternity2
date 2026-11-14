# Referee report: "The partial-match regime: counting, bounds, and what to build"

Night 2026-11-12. Memo under review: `research/program/night/2026-11-12/memos/partial-match-regime.md`
(last saved 21:22:40). Referee scripts and outputs:
`research/program/night/2026-11-12/memos/partial-match-regime-review/` (`r1` to `r6`, stdlib
Python, `python3 -I`, about 5.5 minutes of one core in total, reruns of the memo's
three scripts included). No solver was run on the 16x16. The memo's scripts were
rerun from a scratch copy; nothing in the memo's folder was touched.

Grades used below: **recomputed** (my own code or my own derivation), **rerun**
(the memo's code, same output), **read** (a file of the repo, path given),
**reported** (somebody else's number, quoted by the site).

## 1. Verdict table

| claim | verdict | P(true as stated) | reason in one line |
|---|---|---|---|
| A. Exact first moment over the colour-shuffle ensemble, four community numbers | `holds` | 0.97 | Recomputed by a different method from the committed piece set: 14,702.27, 4.097e-8, 9.2751e42, 10^35.314. Exact equality with full enumeration on 2x2, agreement within noise on 3x3 and 4x4. |
| B. Boards per score (10^34.4 at 470 ... 10^7.1 at 480, zero at 479, 10^2.7 per point) | `holds with a correction` | 0.85 | Every number recomputed to three decimals. "Zero at 479" is true only for placements with a legal grey frame; with the frame free, the site's own page gives a 479 from any 480 (14 border pieces, recomputed). |
| C. One break fewer costs 10^1.9 to 10^2.4, so 471 = 80 to 235 x 470, 472 about 10^4 x | `not established` | 0.3 | The re-optimised figure is stable in the model (10^1.7 to 10^2.2 in every regime I tried). The truncation figure is not (10^0.03 to 10^2.4). The only data, three score ladders, give 17 to 30 per point in mid-ladder and 30, 64, 312 at the top step, on 11, 4 and 2 boards. |
| D. The schedule is worth 22 decades | `holds with a correction` | 0.9 as a model statement | Arithmetic rerun (45.44 - 23.34). 20.6 of the 22 decades are earned by any single late gate (best gate, depth 217: 10^24.83). The list's shape is worth 1.5 to 2.3 decades, not 6.6. No real engine was measured. |
| E1. Looseness of a distinctness-free bound: about 0.2 break per remaining cell, bites 40 to 56 cells from the end | `holds with a correction` | 0.6 | Derived, not fitted. The constant is specific to the 16x16 tail (0.39 decade per cell over 2.3 decades per break). Track 8 measured 0.08 to 0.24 per cell with a tighter bound. |
| E2. Ideal such bound worth at most 5.5 over the best list; a perfect bound from depth 201 does not beat the best list; the line of work is closed | `not established` | 0.3 | True only if the nodes before the first unlock dominate. In the memo's own quota regime (the one that matches the reported 10^15) 92 to 98% of nodes are late, the reuse bound is worth 10 to 28 on Blackwood's list, and a perfect bound from 201 beats the best list by 5 to 9. |
| F. No published soft-constraint, MaxSAT, diagram or dual-decomposition result on edge matching in maximisation form; exact quadratic assignment stops near 30 | `holds with a correction` | 0.75 for the absence, 0 for "near 30" as worded | Twelve searches found nothing. Exact quadratic assignment has reached 35, 40 and 42 on dense instances and 64 to 128 on symmetric ones. All checked citations exist. |
| G. Recommended design and first measurement F0 | `not established` (design), `holds with a correction` (F0) | 0.6 that F0 is the right first measurement, no number for the design | No script evaluates the gate, so its "5 to 10" is an argument. Three of the four parts exist in the repo under other names. F0 is the measurement on which C and E2 turn; it needs two additions. |

## 2. Claim by claim

### A. The exact first moment

**Reruns.** `model.py` (6.5 s), `brute_check.py` (19 s) and `gap_check.py` (59 s)
reproduce their committed outputs line for line (wall times aside).

**Independent recomputation** (`r1_counts.py`, `r6_strict_tree.py`). I did not use
inclusion and exclusion. `r1` counts matchings with exactly k two-coloured pairs by
a transfer over colours (state: half-edges of earlier colours still unpaired, pairs
closed so far), in exact integers, with the census read from
`engine/data/official_eternity2.csv`. Census found: 4 corners, 56 border, 196
interior pieces; five ring colours, each 12 clockwise and 12 anticlockwise; 17
interior colours, five on 48 half-edges and twelve on 50; no ring colour on an
interior side. Results:

| quantity | memo | recomputed | community (site) |
|---|---|---|---|
| E[N(0)], starter | "10^4.17 = 14,790" | 14,702.27 | 14,702 |
| E[N(0)], five clues | 4.1e-8 | 4.097e-8 | about 4e-8 |
| E[N(0)], no clue | 1.15e7 | 1.15266e7 | 1.1527e7 |
| strict row scan, nodes per board | 9.3e42 | 9.2751e42 (peak depth 161, 10^48.76, total 10^50.029) | 9.2766e42 against 9.2751e42 |
| N(467)/N(480) | 10^35.31 | 10^35.314 | 2.05e35 = 10^35.312 |

The theory referee (`theory-referee.md`, items 9 and U5) reached 14,702.27,
4.1e-8, 1.1527e7 and 10^7.0617 by a third route (factorial moments). Three
routes agree.

**Enumeration** (`r2_enumerate.py`, own generator, scorer and depth-first
enumerator).

- 2x2, every instance of the ensemble and every placement, four colour censuses (16 to 576 instances): the mean N(k) equals the formula as exact rationals in all four cases.
- 3x3, 500 sampled instances per census, all 2,304 placements each, full law of the number of breaks: largest |z| 0.97 over 7 values of k (two colours) and 1.94 over 12 values (three colours).
- 4x4, 160 instances, ring joints 6+3+3, interior joints 5+4+3, depth-first counts with at most 2 breaks anywhere, compared with my own implementation of the thinning formula T(d, j): largest |z| 1.31 over 24 comparisons, and N(1) = 0 in every instance.

So "exact over the ensemble" is true, for complete boards and for partial boards
under the "anywhere" rule. The restricted-pattern formula T_S follows from the
same exchangeability and I checked it only through the memo's own code.

**What the four reproductions do and do not show.** All four community numbers
are first moments of the same pairing model (the site's formula at k = 0, Owen's
table for k = 13). Agreement to four digits shows the memo's algebra equals the
community's algebra. It is not a test of the model against the instance.

**What the ensemble average hides for one designed instance** (shown, not argued):

- The mean is carried by rare instances. On the 4x4 ensemble, 92% of instances have no perfect board while the mean is 1.5 (sd 7.2); 41% have no board at 2 breaks while the mean is 210 (sd 333). Nodes per board found, instance by instance: 74 to 12,317, median 970, against 777 for the ratio of sums (`r2`, part 4).
- A planted instance is not a draw. Same census, planted: 29 times the formula at 0 breaks and 7.9 times at 2 breaks, and the tree is wider too (depth 12: 20,068 nodes against 9,497). The size of this effect at k = 10 on the 16x16 is not known; the memo's "within 2 decades with probability 0.8" is a judgment with no computation behind it.
- The designed set differs from a draw in ways the ensemble does not impose: 256 distinct pieces up to rotation and no self-symmetric piece (`r1`). On one statistic it looks typical: 64 interior pieces carry a repeated colour, against 61.0 expected.
- Two instance-level calibration points exist tonight, both from the theory referee and not recomputed by me: the frame count (first moment 10^37.71 against two near-exact counts at 10^37.59 and 10^37.61) and the 36-piece Clue 3 puzzle (first moment 8.2e8 against 2,195,647,488 true, a factor 0.37). The memo cites neither.
- One prediction of the count that the memo does not state, and that committed data supports: 99.5% of the ensemble's 470 boards have no ring break (`r1`). `research/topics/border-mismatch-share` finds 0 border-to-border breaks among 99 on nine 469 boards, and 13.1% on the seam, against 56/420 = 13.3% under exchangeability. Weak support, since engines choose where breaks go.

### B. Boards per score

Recomputed (`r1_counts.out`), log10 E[N(k)], no clue: 7.062 (480), zero (479),
12.286 (478), 14.976, 17.813, 20.609, 23.395, 26.164, 28.915 (472), 31.648 (471),
34.361 (470), 37.053, 39.725, 42.376 (467), 47.612 (465). The step is 2.69 to
2.84 between 477 and 470 and falls to 2.50 at 460. The memo's table and its
"10^2.7 per point" stand. These are ensemble expectations; see A for what that
leaves open about the instance.

**Parity.** The memo's argument (every colour count even, so one break is
impossible) is the one on `web/content/research/why/parity-defect-floor.mdx`
(lines 38, 75 to 91), and the theory referee calls that proof correct (items at
lines 14, 154, 163). It assumes "a legal frame (every rim-facing edge gray)".
The memo's N(k) is defined over class-respecting placements, so inside its own
definition "zero at 479" is correct, and my enumeration finds N(1) = 0 on every
instance at 2x2, 3x3 and 4x4. But `web/content/research/build/analysis/parity-arguments.mdx`
(lines 62 to 71, msg 6317) states the opposite for the matched-edges score
without the frame condition: a border piece whose two ring sides share a colour,
turned end for end, breaks exactly one scored joint, and "14 border pieces
qualify". I recomputed 14 of 56 (`r4_extras.out`). The theory referee does not
mention that page. The memo's headline should carry the condition. For engines
that only place legal frames, nothing changes.

### C. The price of a point

**What the model says, rerun and extended** (`r3_sensitivity.out`).

- Re-optimised lists, 17 colours, uniform sampling: 22.58 (K = 10) against 24.48 (K = 9): 10^1.90, confirmed. My own descent (finer steps) gives 10^1.89.
- Truncation: 23.34 to 25.71 when the first unlock is dropped (10^2.38, the memo's 235), but 25.14 when the last one is dropped (10^1.81, a factor 64). The cheaper way to truncate is not the one the memo priced.
- The 469 comparison rests on an assumed eleventh unlock at depth 196 (memo line 234). The repo has the longer list: `[201, ..., 239, 241, 256]` (`research/experiments/single-core-benchmark/engine/crates/solver-engine/src/schedule_builders.rs:209`). With 241 as the eleventh entry, 469 to 470 is 10^1.73, not 10^2.33.

**Is a model that is 10^6 off in level usable for ratios?** For one of the two
ratios, yes inside the model; for the other, no. I varied the level two ways:
(i) the tail below a perfect depth-160 prefix with the pool spread over fewer
colours (the memo's own `tail_yield` setting) plus a prefix cost c0 from 1 to
10^12 nodes; (ii) the 17-colour tree with all nodes before depth 201 discounted
by 10^0 to 10^-8.

| regime | late share, Blackwood list | 471 by truncation | 471 with re-optimised lists |
|---|---|---|---|
| memo (17 colours, no discount) | 0.09% | 10^2.38 | 10^1.89 |
| 17 colours, strict part discounted 10^-4 | 90% | 10^1.41 | 10^1.79 |
| 17 colours, strict part discounted 10^-6 | 99.9% | 10^0.13 | 10^1.73 |
| 15 + 4 colours from depth 160, c0 = 1 | 98% | 10^0.76 | 10^1.72 |
| 15 + 4 colours, c0 = 10^3 | 92% | 10^1.30 | 10^1.75 |
| 15 + 4 colours, c0 = 10^6 | 1.4% | 10^2.35 | 10^1.83 |
| 17 colours from depth 160, c0 = 10^12 | about 0 | 10^2.39 | 10^2.20 |

The re-optimised ratio stays between 10^1.7 and 10^2.2 over twelve decades of
level. The truncation ratio goes from 1 to about 240. So the memo's range "80 to 235"
mixes a robust number (about 50 to 160) with one that depends entirely on where
the engine spends its nodes.

**The absolute level and the quota.** The memo's arithmetic is right: the tail
yields differ by 10^5.89 (depth 200) to 10^10.78 (depth 155) between 17 + 5 and
15 + 4 colours (rerun). I priced the consequence the memo did not
(`r5_gate_and_469.out`): with 15 + 4 colours below depth 160 and the repo's
eleven-entry list, a 469 costs 10^15.5 nodes when a quota-satisfying prefix costs
up to 10^3 nodes, and 10^17.4 when it costs 10^6. The reported figure is about
10^15. So the quota explanation is numerically consistent in the model, but only
in the regime where 92 to 98% of the nodes are after the first unlock. The memo
uses the opposite regime (0.09% late) for every conclusion of sections 2.4, 2.5
and 4. Both cannot hold at once. This is the most serious problem in the memo.

**Independent check against data.** The memo sets JSA's ladder aside as "a
different quantity". The model can compute that quantity: the score histogram
of one engine with one fixed list (`r3`, part 1). Under Blackwood's list it
predicts a factor 64 from 470 to 471 as a by-product, rising to 106 four points
lower; under a 13-entry list 70 to 118; with fewer colours in the tail 49 to 98
(`r4`). The model's by-product factor (64) is also its cheapest truncation
price, so in the model the two quantities are the same size.

Observed, all reported, all from depth-first engines with depth-gated break
schedules:

| source | counts | factor per point |
|---|---|---|
| JSA's run of Verhaard's eii, 82 days, one PC (`louis-verhaard/eii.mdx:219-224`, msg 6687) | 4,017,182 at 463; 227,245 at 464; 13,637 at 465; 625 at 466; 2 at 467 | 17.7, 16.7, 21.8, then 312 (95% interval 87 to 2,600 on 2 boards) |
| Five-clue fleet, modified Blackwood engine (`community/five-clue-fleet.mdx:97-99`) | 7,645 at 463; 327 at 464; 11 at 465 | 23.4, then 29.7 (17 to 60 on 11 boards) |
| Volunteer project dashboard (`OBSERVATIONS.md:173-177`) | 259 at 465 or better in a week, four at 466 | about 64 (25 to 230 on 4 boards), if the two counts cover the same window |

Two readings. Mid-ladder, the observed factor is 17 to 30, three to five times
below the model's 60 to 120. At the top step of each ladder it is 30, 64 and
312, which brackets the memo's range with very few boards. The ladder's shape is
also opposite: the model's factor falls toward the cap (106 to 64), the observed
one rises. The memo's conjecture that JSA's 20 "is close to rho = 16" is not
what its own model gives.

The record timeline cannot settle it: 468 "every 4 days" on unstated hardware
(msg 10033), the first 469 after a few days on about 200 cores (one event), the
first 470 after about a month on one 32-core machine, called "a stroke of luck"
by its author (`web/src/data/records-timeline.ts:41`), a second 470 in 2024. In
core-days the 469 and the 470 cost about the same, each on a single event.

**Ruling.** The statement the evidence carries: one more point costs between one
and two and a half decades for this family of engines; the model's value for a
re-tuned list is 10^1.7 to 10^2.2; no measurement at 469 to 471 exists. "472
about 10^4 times" inherits the same uncertainty squared.

### D. The schedule is worth 22 decades

Arithmetic: 45.44 (ten breaks anywhere) against 23.34 (Blackwood's list), rerun.
The reasoning (a break allowed before the tree's peak multiplies everything
below it) is sound and holds for any depth-first search.

Two corrections (`r5`, part a). First, the split. A single gate with all ten
breaks free from depth g costs 10^29.93 at g = 201 (the memo's row), 10^24.83 at
g = 217 (the best), 10^27.08 at g = 229. So 20.6 of the 22.1 decades come from
"late" alone. The list's shape adds 1.5 decades (Blackwood) to 2.3 decades
(model-optimal list) over the best gate. The memo's "the cumulative list is
worth 6.6 decades over a single gate" compares with a gate placed at 201, which
is a poor gate. Second, the status. The 22 decades are the distance to a rule
that no engine uses, in a model whose tree is the 10^50 strict tree sampled
uniformly. For a real engine the ordering will hold; the number has not been
measured and the memo does not claim it has.

### E. The distinctness-free bound

**Derived or fitted?** Derived. The memo's estimate is (reuse over distinct, in
decades) divided by (decades per extra break). On the 16x16 with 64 cells left:
10^24.96 (0.39 decade per cell) over 10^2.30 per break, 10.9 breaks, 0.17 per
cell (`r4`, part b); the model table gives 11 at 56 cells. The small-board
agreement is not the same constant twice: on 6x6 and 7x7 the ratio is 0.26 to
0.31 decade per cell and a break is worth 1.25 to 1.52 decades. The same formula
gives 2.5, 3.6 and 2.9 breaks against 2.75, 2.75 and 2.2 measured by the memo on
4 to 8 prefixes. So the formula survives a change of both terms, which is better
evidence than a matched constant. Both "true" and "reuse" minima in the 16x16
table are first-moment thresholds, each a lower bound on its own quantity; their
difference is an estimate of the gap, not a bound on it.

**Track 8.** Results are committed on `night/2026-11-12-t8-score-bound-gap`
(6x6 rows at 20:54:17, 7x7 at 21:06:44; `results/summary.json` in that worktree,
written 21:27:36, read by me). Median gap, pooled, 240 prefixes per cell:

| rows left | 6x6 W=4096: predicted, measured | 6x6 W=64 | 7x7 W=4096 | 7x7 W=64 |
|---|---|---|---|---|
| 1 | 0, **0** | 0 to 1, **0** | 0 to 1, **0** | 1 to 2, **0** (miss) |
| 2 | 2 to 3, **1** (miss) | 3 to 5, **3** | 2 to 4, **2** | 4 to 6, **4** |
| 3 | 3 to 4, **3** | 4 to 6, **4** | 4 to 5, **4** | 5 to 7, **5** |

Ten of twelve cells fall in the predicted range; both misses are on the side of
a tighter bound than predicted. Side predictions: "W = 512 and 4096 within one
edge on 6x6" holds (2 against 1, 4 against 3); "does not close with W" fails at
two rows on 6x6 (3, 2, 1 for W = 64, 512, 4096), because Track 8's diagram keeps
used-piece sets and is not a pure distinctness-free bound; "frontier relaxation
above 70% at W = 64" fails (38 to 45% of absorbed nodes); the one-row gap is 0
for W of 64 and more, as predicted; the stage-0 kill did not fire (median 4 at
W = 4096 on 7x7, the threshold itself). Gap per remaining cell at W = 4096: 0.08
to 0.19 on the critical rungs, 0.14 and 0.24 on 7x7 with 8 colours (12 to 16
prefixes).

**Was it blind?** File times cannot show it. Track 8's 6x6 rows were on disk and
committed 28 minutes before the memo's last save, the 7x7 rows 16 minutes
before, and both before the memo's own `gap_check.py` (21:12:36) existed. I
found no trace of a read, and the two misses go the way a copied table would
not. The prediction was not blind to the quantity in any case: the memo measured
the same gap on the same two board sizes at 21:16 and predicted afterwards.

**Does the conclusion survive a different late share?** No (`r3`, parts 3 and 4).
log10 nodes per 470 board:

| regime | Blackwood list, plain | plus ideal reuse bound | best list, plain | best list plus bound | perfect bound from 201 |
|---|---|---|---|---|---|
| memo (0.09% late) | 23.34 | 23.34 | 22.57 | 21.83 (gain 5.5) | 22.88 |
| 17 colours, strict part x 10^-4 | 20.32 | 19.40 | 19.73 | 18.63 (gain 13) | 18.88 |
| 17 colours, strict part x 10^-6 | 20.27 | 18.53 | 18.39 | 17.48 (gain 8) | 16.88 |
| 15 + 4 colours, c0 = 1 | 17.23 | 15.78 | 16.06 | 15.50 (gain 3.6) | 15.10 |
| 15 + 4 colours, c0 = 10^3 | 17.26 | 16.25 | 16.46 | 15.88 (gain 3.8) | 15.73 |
| 15 + 4 colours, c0 = 10^6 | 19.07 | 19.06 | 18.20 | 17.64 (gain 3.6) | 18.61 |

What survives: over a list re-optimised for the regime, the ideal reuse bound is
worth 1.5 to 14 in every regime tried, so "less than about one decade over the
best list" is robust inside the model. What does not: on the list actually in
use the same bound is worth 10 to 57 once the engine is late-dominated; and a
perfect bound from depth 201 beats the best list by 5 to 9 in the quota regime
and by 30 or more when the strict part is discounted by 10^-6. The sentence "a
perfect bound of any kind from depth 201 cannot beat the best list at all" is a
property of one regime. The "best list" in the late-dominated regimes is itself
an untested model output (it moves the unlocks later, for example 199, 210, 216,
...). The memo names this weak point and gives 0.45 to "more than half the nodes
are late"; its own quota calculation implies well above 0.9 unless the prefix is
expensive. The line of work is not closed by this memo. It is closed or opened
by F0.

### F. Literature

Opened and checked (by a separate search session, pages read, no project name in
any query):

- Sewa, Allouche, de Givry, Katsirelos, Montalbano, Schiex, AAAI 2026, 40(17), 14322-14330, DOI 10.1609/aaai.v40i17.38447: exists. The paper says "tb2+LAPJV solved 33 [...] CPLEX solved only 22 instances, and CP-SAT 25" on "the 132 smallest of the 136 instances" of QAPLIB within 1,200 s, and "scr20 in 337 seconds and 0.9 million search nodes". The memo's account is accurate.
- Anstreicher, Brixius, Goux, Linderoth, Math. Program. 91, 563-588, 2002: exists. "Averaged approximately 650" workers is from the authors' announcement; the journal text could not be opened.
- The three citations flagged as from memory: Heule's paper is from the Second International Workshop on Logic and Search (LaSh 2008), per his CV; Gilmore is J. SIAM 10(2), 305-313, 1962, DOI 10.1137/0110022; Lee and Leung is JAIR 43, 257-292, 2012, title "Consistency Techniques for Flow-Based Projection-Safe Global Cost Functions in Weighted Constraint Satisfaction".
- Schaus and Deville's 458 is confirmed only through Salassa et al. ("the best score over 20 runs equals 458").
- XCSP3 2022: "Eternity" is in the CSP section, modelled with `satisfy`, 10 instances. It is a decision benchmark, which answers the memo's open point.

The negative claim was not falsified in twelve varied searches. The nearest
published work in maximisation form is Salassa et al. (integer programming,
already cited).

"Exact methods stop near 30 assignment variables" is wrong as worded. Proven
optima exist for tai35b and tai40b (Gonçalves et al., arXiv:1510.02065), sko42
and tai30a (Fujii et al., arXiv:2101.09629), ste36a/b/c, and, by exploiting
symmetry, esc64a, tai64c and esc128 (Fischetti, Monaci, Salvagnin, Operations
Research 2012). The corrected statement: dense unstructured instances stop
between 30 and 42 with very large machines; symmetric or sparse ones have
reached 64 to 128; tai256c is open. The memo's inference (256 cells are out of
reach for a full-board bound) still follows. Its reason (a size wall at 30) is
overstated, and the structured cases are the ones closest to a grid flow matrix.

### G. Design and first measurement

**Soundness of the parts.** The gate is declared unsound and is a heuristic
schedule; that is consistent. Its expected gain ("between 5 and 100, central
estimate 5 to 10") is not computed by any script: `model.py` has no gate. The
probabilities 0.5 and 0.25 in section 5 have no calculation under them.

**What is already in the repo** (read):

- Colour staging is Blackwood's quota, present as data in three engine forks: `schedule_builders.rs:188-189`, depths `[16, 26, 56, 76, 102, 160]`, counts `[0, 28, 71, 89, 106, 119]`, and `build_quota` in `research/experiments/single-core-benchmark/engine/crates/bench-audit/src/bin/blackwood_bt.rs:117`. The memo proposes to compute it; nothing computes it today.
- "Parity as the only sound per-node bound" exists as the LEDGER prune: `research/topics/ledger-prune` (`compute/src/main.rs:119-120`, `deficit <= r && odd <= 2 * r`). It carries the colour-deficit condition as well, and its article reports node ratios of 1.5 to 5 on generated boards in suffix races with budgets 1 to 5, and unreproduced 140 to 4,330 on a 464 board's tail. The memo's table (line 485) says the supply test is "untested with breaks" and its model gives parity 4% of late nodes. The memo does not cite the topic, and `EXPLORED.md` has no row for it. The two results are in different settings (suffix exhaustion against a scheduled search) and nobody has reconciled them.
- The exact last-row solver exists as `research/topics/exact-tail-endgame` (CP-SAT, draft), which the memo cites.
- The first-moment gate is in no engine. The theory referee proposed the same statistic independently tonight (`theory-referee.md:355`, item C3, "census-exact completion count as a node score") with a different first test: rank correlation between the estimate and exact completion counts on 8x8. That test is cheaper than F2 and should come before it.

**Is F0 the right first measurement?** Yes. Section C and E above show that the
truncation price, the value of any late bound and the consistency of the quota
explanation all turn on one number, the share of nodes after the first unlock.
Two additions cost nothing: (1) the complete-board score histogram of the same
run, which `blackwood_bt.rs` already prints (lines 263 and 1475, no committed
output), set against the model's 64 per point; (2) the nodes spent per
quota-satisfying depth-160 prefix (the c0 of my tables). F0's stated threshold
(90% late) should be replaced by a reading of the table in E: above about 50%
late, late bounds have one to two decades of room over the current list.

Track 7 (`partial-match-ladder`) measures the best score at fixed work, and the
work to reach the optimum minus k for k = 1 to 6, with the repo's general
engines on 8x8 to 12x12. That is close to F1's slope but not F1: no
cumulative-list engine and no model prediction beside it. Its README still
held a results placeholder when I read it.

## 3. Errors found

1. `partial-match-regime.md`, lines 18, 208-209, 258-278 against lines 20 and 464-469. The memo explains the 6-decade level error by the quota and prices the quota's benefit with `tail_yield`, then draws its conclusions on bounds from a tree in which 0.09% of nodes are late. With the quota put in the tail (`r3`, part 3; `r5`), the level matches the reported 10^15 only when 92 to 98% of nodes are late, and in that regime the conclusions of lines 20 and 464-469 reverse. Fix: state every conclusion of section 4.2 as conditional on the late share, and add the table of section E above.
2. Line 20 and lines 464-467: "A perfect bound of any kind from depth 201 cannot beat the best list at all." False outside the pre-unlock-dominated regime (15.10 against 16.06 in the quota regime). Fix: "does not beat the best list if more than about 99% of nodes precede the first unlock".
3. Line 17 and lines 230-233: "80 to 235". The 235 is the truncation that drops the first unlock; dropping the last costs 64 (`r3`, part 2), and the truncation price falls to 1 to 20 when the engine is late-dominated. Fix: quote 10^1.7 to 10^2.2 for re-tuned lists and say the truncation price is unknown until F0.
4. Lines 233-234 and `model.py:454`: the eleventh unlock is assumed at 196. The repo's list continues with 241 and 256 (`schedule_builders.rs:209`); `records-timeline.ts:41` says the 470 run went from 11 entries to 10. Fix: use `blackwood + [241]`; 469 to 470 becomes 10^1.73.
5. Lines 235-239: JSA's "about 20 per edge" is called a different quantity and conjectured equal to rho = 16. The model's value for that quantity is 64 to 118. Fix: report the model's histogram next to the three observed ladders and drop the conjecture (or keep it at a probability well under 0.4).
6. Line 206: "The cumulative list is worth 6.6 decades over a single gate." The best single gate (depth 217) costs 10^24.83. Fix: "1.5 decades over the best single gate; 6.6 over a gate at 201".
7. Line 16: "The schedule is worth 22 decades; it is the algorithm." 20.6 of them belong to any late gate. Fix: give the split.
8. Lines 15 and 149: "zero at 479 (parity)". True for legal-frame placements only; `parity-arguments.mdx:62-71` gives a 479 with a flipped border piece. Fix: add "for boards with a legal grey frame".
9. Line 146: "10^4.17 = 14,790 against the canonical 14,702". The script's value is 14,702.27; 14,790 is the rounded logarithm raised back. Fix: "14,702".
10. Line 21 and line 626: "exact methods stop near 30 assignment variables". See F. Fix: "30 to 42 on dense instances, 64 to 128 on symmetric ones".
11. Line 485: the colour supply test is called "untested with breaks". `research/topics/ledger-prune` tests it with breaks. Fix: cite it and state the difference of setting.
12. Line 10 and line 568: "made blind". Not verifiable, and the prediction followed the memo's own measurement of the same gap. Fix: "made without reading Track 8's files, after my own measurement on the same sizes".
13. `model.py:525-534` (`bounded_cost`): the survival factor is applied to each node independently of its ancestors and to a typical 17-colour pool. The memo lists this as a weak point; the table at lines 450-457 should carry the caveat in its caption.

No arithmetic error was found in any script.

## 4. What the program may rely on tomorrow

1. The first-moment count with breaks is exact over the colour-shuffle ensemble and reproduces 14,702.27, 4.1e-8, 9.2751e42 and 10^35.31; `model.py` may serve as the program's null model for counts.
2. Expected legal-frame boards per score: 10^34.4 at 470, 10^31.65 at 471, 10^28.9 at 472, none at 479, a step of 10^2.7 per point, as ensemble expectations.
3. Breaks must be late: any late rule beats "breaks anywhere" by about 20 decades in the model, and the reason holds for any depth-first search.
4. A bound that drops piece distinctness is loose by reuse over distinct (0.26 to 0.39 decade per cell) divided by the decades one break buys; on 6x6 and 7x7 that is 1 to 4 edges at two and three rows, as Track 8 measured.
5. For lists re-tuned to the target, the model prices a point at 10^1.7 to 10^2.2, whatever the absolute level.

## 5. What it must not rely on yet, and the cheapest measurement for each

- **"471 costs 80 to 235 times 470."** Not until a ladder exists near the top. Cheapest: print the complete-board score histogram of the Blackwood-style engine over a run long enough for 100 boards at its second-best score (already in `blackwood_bt.rs`), and compare with the model's 64 per point.
- **"A late bound is worth at most 5.5" and "a perfect bound from 201 cannot beat the best list."** Not until F0. Cheapest: nodes by depth for that engine, 60 s on one core. Above about 50% late, the reuse bound has one to two decades of room over the current list.
- **"The 6-decade gap is the quota."** Consistent only if a quota-satisfying depth-160 prefix costs under about 10^3 nodes. Cheapest: the same F0 run, counting nodes at depth 160 or less per prefix that reaches depth 160.
- **N(470) of the real instance "within 2 decades".** No computation supports the interval. Cheapest: the memo's count on 6x6 to 8x8 planted boards at k = 4 to 6 against an exact or sampled count, which gives the planted excess at the k that matters.
- **The gate and computed colour staging gain anything.** No script evaluates them. Cheapest: the theory referee's C3 test (rank correlation of the estimate with exact completion counts on 8x8), before any engine work.
- **Parity removes only 4% of late nodes.** In conflict in spirit with `ledger-prune`'s 1.5 to 5. Cheapest: run the LEDGER conditions inside a scheduled search (not a suffix race) on one generated 16x16 and count fires by depth.
- **"Zero at 479" for any board.** Only with a legal frame; the site's two parity pages need one sentence each to say which case they cover.
