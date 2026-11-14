# Judge report: dead-node refutation census (Track 6, night 2026-11-12)

Branch `night/2026-11-12-t6-dead-node-census`, judged at commit 70df1dd.
Specification read from `night/2026-11-12-lead`: `IDEAS.md` ("Shared stage
0"), `ideas-raw/TOURNAMENT.md` sections 3(a) and 4, and the proposals
`compilers.md`, `pair-glasses-compilers.md`, `tiling-words.md`,
`pair-synthesis-tiling.md`.

## Verdict: keep (as a measurement), with one verdict downgraded

The census is valid, its numbers reproduce exactly, and its soundness holds.
Of the four stage-0 kills it issues, three stand (T2 on K0a, T5, ST for the
`bb-mrv` parent). The T3 kill on K2 does not stand as a mechanical kill: the
statistic and the control differ from the proposal's K2. T2's K0c is a
comparison of two infinite medians and is not a kill (T2 is killed by K0a
alone). OC is not decided, as preregistered.

## Per-proposal rulings

| proposal | census verdict | ruling | deciding number (recomputed) |
|---|---|---|---|
| T2 cavity summaries (GC) | killed (K0a, K0c) | stage-0 kill stands (on K0a only) | R7 B-pool share r <= 3: 0.0083 weighted, 0.057 unweighted; every alternative below 0.13 against 0.30 |
| T5 band consistency (TW) | killed (census kill) | stage-0 kill stands | k=2 share R6 0.0023, R7 0.0018 weighted (0.0064, 0.0125 unweighted) against 0.05 |
| T3 cut-keyed partitioning (CO) | killed (K2) | stage-0 kill does not stand | one-shot share cut 0.245 / rand 0.251 (R6), 0.103 / 0.096 (R7); not the proposal's replay ratio, control not partition-matched |
| ST boundary-word lemmas | killed (local-core rate) | stage-0 kill stands (bb-mrv parent) | R7: 0 of 1,447 sampled dead nodes with s >= 200 proven, median rate 0.000 over 30 trees |
| OC piece-priced frontier DP | no kill | not decided by this census | E floor at R7 0.017 weighted |

## 1. Preregistration order

- `PREREG.md` committed in 41c069f at 20:15:28; `git log --follow` shows no
  later edit. Every row commit (stage B 20:47:39 onward, gate 20:51:22) comes
  after it.
- No `results/run-log.jsonl`: `research/experiments/common/prereg_guard.py` is
  not on this branch (it entered the lead branch at 20:41 and 20:44, commits
  e254e3f and 21f2154, neither an ancestor here). Finding, not invalidating:
  the order rests on commit order and file evidence only.
- File evidence (local file times): `results/` created 20:36:13; the stage
  queue log in the session scratch area was born at 20:34:28; a smoke run at
  20:23 used calibration seeds 1001 and 1002 only. Instance rows carry no
  timestamps, so the start of the first test-seed run is bounded below by
  20:34:28, 19 minutes after the PREREG commit. Nothing indicates a test-seed
  row before 20:15:28.
- Amendment hygiene: amendment 1 is dated 20:30 but was committed at 20:26:52;
  amendment 2 was rewritten in place in 597a392 (the first version said the
  runs would be on `dell`/`mac-m1`). Neither touches a threshold. The machine
  change (all rows on `mac-m4`) does not affect counts, which are
  machine-independent and which I reproduced on another Mac.
- Stage B ran before the soundness gate rows existed (the gate crashed and was
  fixed in 66e9408, a gate-only code path). PREREG requires the gate before any
  share is reported, which holds.

## 2. Track label and identity

Label: measurement, correctly applied (no solver claim). The diff from the
merge base touches only `research/experiments/dead-node-census/` and
`research/experiments/justfile`: the A22 crate, `hard-ladder`,
`sat-backdoors`, `NodeCounter` and PROTOCOL.md are untouched. My recompute of
the identity gate: 450 instance rows (90 trees x 5 stages) against the
committed `bb-mrv` rows, 0 mismatches in status, nodes, propagation,
backtracks; every tree has a single node-sequence hash across all passes.

## 3. Rerun (subset fixed before running)

Fresh build into a separate target dir, all five oracles in one pass,
4 threads: R6 seeds 1..30, N7 seeds 1..30, R7 seeds 9..18 (70 trees, about
5.5 minutes), plus the soundness gate on R6 seeds 1..30. Compared every field
except wall times and machine: 70 instance rows (nodes, propagation,
backtracks, hashes, dead counts, strata) and 13,247 node rows (251,693
fields), **0 mismatches**. Gate rerun: 2,100 rows, identical count, 0
violations.

## 4. Soundness

- Committed gate rows: 7,524 live states (3,930 planted-path, 3,594
  solution-path) on 90 trees; I recomputed "dead" from each oracle field
  (B-pool, B-dom, A1..A3, E, four D arms, C): 0 fire on a live state.
- Independent check (my own code, `judgefuzz` in the judge scratch area): a
  naive completion search written from the raw model (row-major fill, unused
  pieces, rim and neighbour matching, no propagation) on 4x4 c=3, 4x4 c=4,
  5x5 c=4, 5x5 c=5, seeds 501..530, random and planted-biased walks: 39,574
  live and 299 dead states, all decided. **0 oracle fired on a live state.**
  The check is not vacuous: on the dead states B-dom fired 284 times, D-cut
  215, D-rand 207, B-pool 179, E 48, A3 23, A2 4. My naive search agreed with
  the census's own `tree::completes` on all 39,873 states.
- Limit: the gate runs C with zero wipe counts (pocket grown from the lowest
  index), so C's wipe-driven growth is exercised for soundness only by the
  fuzz walks with random wipes. C never fired on my 299 dead states.

## 5. Headline numbers recomputed

Own script from the committed rows (stdlib, same bootstrap seed): every share
in the README table matches to the printed precision. Population: R6 13,966
dead nodes, 5,335 sampled; R7 11,191,605 dead, 6,000 sampled (7 of 30 trees
censored); N7 78,220 dead, 5,912 sampled; on every tree the design weights sum
exactly to the dead-node count (no dropped strata or seeds). Budget
exhaustions: B-pool hit its budget at 186 sampled R7 nodes; E and C never
overflowed.

## 6. Per-proposal rulings in detail

### T2 boundary-keyed cavity summaries (GC): kill stands, on K0a only

1. Transcription. The proposal's K0a counts sampled dead nodes (unweighted);
   IDEAS/TOURNAMENT corrected it to weighted. The PREREG copies the corrected
   version (R7, r <= 3, B-pool with the current pool and window-internal
   distinctness, threshold 0.30). Faithful. K0b uses raw keys among refuted
   nodes; the proposal counts repeats over cavity queries. Close enough, and it
   does not fire. K0c faithful in wording but see 3.
2. Oracle. B-pool is the proposal's own cavity (pool candidates, distinctness,
   window touching the placed region); windows may also hold assigned cells,
   which is at least as strong. B-dom (propagated domains) is a stronger
   reading.
3. Conventions. K0a under every alternative: weighted 0.0083, unweighted
   0.057, pooled ratio 0.0086 / 0.059, B-dom 0.040 / 0.240, and every node
   with a budget-exhausted cavity counted as proven 0.037 / 0.123. All far
   under 0.30: the kill does not depend on the convention. **K0c is not a
   kill**: both weighted medians are infinite (R7 0.939 of weight at r = inf,
   N7 0.912), and the "inf ordered last, R7 not above N7" rule turns a tie of
   two infinities into a kill. By the share at r = inf the weighted reading
   puts R7 above N7 (K0c would not fire); the unweighted reading (0.642 against
   0.695) goes the other way. K0c decides nothing; the T2 kill rests on K0a.

### T5 band consistency (TW): kill stands

1. Transcription. Proposal: k=2 proves dead under 5% of the dead nodes at both
   rungs (unweighted, `scb` parent). IDEAS/TOURNAMENT: weighted, census on
   `bb-mrv`. The PREREG copies the corrected version. Faithful.
2. Oracle. A is exactly the proposal's stage-1 diagnostic (one forward and
   backward pass per band on current domains, distinctness relaxed, wipe-out
   means dead), and it scans column bands too, so it is stronger in coverage.
   It is weaker than the stage-2 mechanism (band filter iterated with
   propagation down the search), but the proposal set its own stage-1 kill on
   this one-shot diagnostic.
3. Conventions. k=2 weighted 0.0023 (R6) / 0.0018 (R7), unweighted 0.0064 /
   0.0125, per-tree median 0.0004 / 0.0013; even k=3 is 0.0113 / 0.0077. No
   convention approaches 0.05. The mechanism criterion (k=1 inside the k=2 CI)
   does not fire and was a stage-2 criterion in the proposal anyway.
4. Limit: the `scb` tree, whose domains are weaker, was not measured; the
   TOURNAMENT fixed the census parent to `bb-mrv`.

### T3 cut-keyed trace partitioning (CO): kill does not stand

1. Transcription. The proposal's K2 is on the **median replay node ratio**
   with partitions persistent down the descent, and its control C1 has **the
   same number of partitions** with edges **not on the cut**. IDEAS keeps both
   (the T3 card still lists "the full replay on bb-mrv trees with persistent
   partitions" and "C1 random key"). The PREREG replaces the replay ratio by
   the one-shot weighted share at sampled nodes, keeps the 0.10 margin from the
   other scale, and draws D-rand among all undecided edges except the two cut
   edges with no partition matching. Measured consequence: D-rand used a median
   of 16 partitions against D-cut's 6 at R7 (10 against 6 at R6), and had more
   partitions than D-cut at 5,178 of 6,000 sampled R7 nodes.
2. Oracle. Per node, a one-shot partition with the same key equals the
   persistent partition state if A3 propagation is a monotone closure
   (argued, not measured); the census re-chooses the key at every node instead
   of inheriting it, and it does not aggregate into a replay ratio. So the
   statistic is a proxy on a different scale, not the proposal's K2.
3. Conventions. As preregistered it fires: |rand - cut| = 0.006 (R6) and
   0.007 (R7) weighted, 0.009 / 0.025 unweighted. On the nodes where D-rand had
   no more partitions than D-cut (a selected subset, so indicative only):
   R6 cut 0.393 / rand 0.379, R7 0.179 / 0.197 weighted. By depth band the
   arms agree within 0.02 at both rungs. The evidence leans towards K2 firing
   under the proposal's definition, but the mechanical kill was issued on a
   different statistic with an unmatched control, so it is not a stage-0 kill
   of the proposal.
4. What decides it: the replay the proposal and IDEAS specify (under 1
   core-hour by the proposal's estimate) on the same 60 R6/R7 trees,
   persistent partitions keyed on the cut rule, C1 drawn among undecided edges
   off the cut line with the partition count matched, K2 on the median replay
   ratio, K1 (replay ratio above 0.5 at R7) and K3 read from the same run. K3
   on the census proxy does not fire (POAC 0.063 against cut 0.103 at R7).

### ST boundary-word lemmas: kill stands for the `bb-mrv` parent

1. Transcription. Proposal: median local-core rate below 10% at R7, rate =
   share of refutations with subtree >= 200 that yield a verified lemma. PREREG:
   same threshold, unweighted design-weighted share among sampled dead nodes
   with s >= 200. Faithful.
2. Oracle. C grows one pocket of up to 2n cells from the most-wiped cells and
   checks it exactly with pool candidates, as the proposal's extraction does
   before generalisation. Generalisation (holes, larger pools, smaller F) can
   only make a pocket easier to fill, so if the grown pocket is feasible no
   lemma exists along the proposal's own path. Same strength as the mechanism.
3. Conventions. 0 of 1,447 sampled R7 nodes with s >= 200 are proven (30 of 30
   trees have such nodes, 19 to 70 each); 0 of 88 at R6, 0 of 456 at N7. Mean,
   median, weighted or pooled: 0.000. No convention changes it.
4. Limit: the proposal also names `scb` as a parent; not measured here.

### OC piece-priced frontier DP: not decided by this census

As preregistered. E (colour-only feasibility, the lambda = 0 floor) proves
0.017 of weighted R7 dead nodes (0.110 unweighted), 0.208 at N7.

## 7. Other checks

- No ordering-method claim, no node gain claim, no wall-clock claim: the
  README states every share is an upper bound and that work is not subtracted.
- No planted-solution leakage: oracles see only the node's state; the planted
  path is used only by the gate.
- Censored R7 trees: dead nodes there are only fully exhausted subtrees; nodes
  open at the cap are excluded, as preregistered. No censored run counts as a
  win (there is no win).
- README wording "The preregistered stage-0 kill fires for T2, T5, T3 and ST"
  is mechanically true but should carry the two rulings above (K0c is a tie of
  infinities; T3's K2 is a proxy).


## 8. Reproduce this judgement

- Recompute: `python3 -I recompute.py` and `python3 -I compare.py` (judge
  scratch scripts reading `results/`), numbers as quoted above.
- Rerun: `census run --oracles B,A,E,D,C --threads 4` on primary n=6 seeds
  1-30, plus2 n=7 seeds 1-30, primary n=7 seeds 9-18; `census gate` on primary
  n=6 seeds 1-30.
- Independent soundness: a stand-alone crate path-depending on the census lib
  for the oracles only, with its own naive completion search.

