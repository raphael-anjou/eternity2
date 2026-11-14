# Ideas, night 2026-11-12

Output of the idea slot. Nothing here ran tonight. Ten inventor proposals and
three from the lead (13 after one merge) were novelty-checked by scouts and
ranked by a separate judge in a full round robin (78 pairs). Sources:
`ideas-raw/*.md` (full proposals), `ideas-raw/NOVELTY.md`,
`ideas-raw/TOURNAMENT.md` (cards, pair table, evidence grades).

## Ranking

| rank | entry | wins of 12 | regime |
|---|---|---|---|
| 1 | Proof-number search, revocable branching cells | 12 | first-solution nodes |
| 2 | Boundary-keyed cavity summaries | 11 | first-solution nodes |
| 3 | Cut-keyed trace partitioning | 10 | first-solution nodes |
| 4 | Relaxed decision diagrams as a score bound (lead L1) | 9 | partial match (470 regime) |
| 5 | Band consistency by tile transducers | 8 | first-solution nodes |
| 6 | Adaptive multilevel splitting on local-search runs | 6 | partial match |
| 7 | Piece-priced frontier bound with safe banding | 6 | first-solution nodes |
| 8 | Partial-match ladder (lead L4, an instrument) | 5 | partial match |
| 9 | Information-set decoding of an incumbent | 5 | partly |
| 10 | Price 471 by rare-event splitting (lead L3) | 3 | partial match |
| 11 | Dissection into seam-guessed bands (with lead L2) | 2 | partial match |
| 12 | Boundary-word lemmas learned from refutations | 1 | first-solution nodes |
| 13 | Auction prices plus score-band counting | 0 | first-solution nodes |

## How to read this ranking

- **The criterion favours cheap node tests.** The brief ranks by "attacks a
  recorded stall with a cheaper falsifying test". Stage 0s on 6x6 and 7x7 node
  counts are the cheapest, so four of the top five only bear on nodes to a
  first solution on small generated boards. By PROTOCOL_TARGETS_PROPOSAL.md
  only growth with n bears on the 16x16, and no inventor preregistered a growth
  stage; the judge added one to every top-five entry.
- **One measurement decides ranks 2, 3 and 5.** The dead-node refutation census
  below costs about 3 core-hours and replaces three separate stage 0s. The
  judge recommends giving the freed slot to rank 6, the cheapest entry on the
  470 regime.
- **Rank 8 is a precondition, not a loser.** Without a partial-match instrument,
  ranks 4, 6, 9 and 10 cannot be judged on the target the 16x16 has. The judge
  recommends building it as a measurement track, starting from the
  scaling-ladder lab page.
- **6x6 c=5 is too easy.** It solves in about 951 nodes, so a factor of 2 there
  is noise in wall clock. Decisions should rest on 7x7 c=6 and the growth track.
- **Evidence grades.** Only stops graded `measured` decided a pairing. Of 21
  explored or dead-end claims the ranking leaned on, 7 are measured, 8 reported
  and 6 argued (TOURNAMENT.md section 5).

## Shared stage 0: dead-node refutation census

Run `bb-mrv` with a fixed seed on ladder R6 and R7 test seeds 1..30, plus 7x7
c=8 as the above-critical null, cap 1e6 nodes, full tree logged with subtree
sizes. Sample up to 200 dead nodes per instance, stratified by depth, every
statistic weighted by subtree size. At each sampled node run five checks and
log each one's work:

- A. band filter over 1, 2, 3 rows, piece uniqueness relaxed (rank 5);
- B. exact k x k cavities, k = 2, 3, 4, with the current pool, giving the
  refutation length (rank 2);
- C. exact pockets of up to 2n cells grown from the most-wiped cells (rank 12);
- D. propagation partitioned on 2 cut edges, on 2 random edges, and POAC on the
  MRV cell at matched budget (rank 3);
- E. colour-only feasibility of the whole empty region (floor for rank 7).

For B and C also record how often a refuting boundary key recurs across
prefixes, raw and under translation and rotation. Output per check and rung:
weighted share of dead nodes proven dead, median work, recurrence. Every share
is an upper bound on node savings with unchanged branching.

## Top five, AGENDA format

### T1. Revocable branching cells by proof-number search · proposed

- **Hypothesis.** At R7, MRV's cell often has a much larger refutation tree
  than another of the 3 smallest-domain cells; df-pn over revocable splits,
  same propagation as `bb-mrv`, finds the cheaper refutations at net gain.
- **Borrowed from.** Allis et al., AIJ 66, 1994; df-pn (Nagai 2002).
- **Nearest cell, missing mechanism.** Dynamic fill orders (saturated: one
  committed cell per node). Missing: effort moved between refutations.
- **Cheap test.** Stage 0 headroom oracle (r = MRV cell size over best of 3,
  300+ refuted nodes, R6/R7). Stage 1 df-pn k=3, R6/R7. Correction: add a
  growth stage on the +2 track n = 6..8, and count work (nodes plus
  propagations) beside nodes.
- **Baseline.** Better of `bb-mrv` and scb per rung (R7: scb 16/30 within
  1e6). Controls: df-pn k=1; added: dom/wdeg on `bb-mrv` (untested here),
  so a gain is not plain failure weighting.
- **Kill.** K0: median r at most 1.25 and r at least 2 at under 20% of nodes,
  both rungs. K1: R7 paired node ratio not at most 0.7 or CI includes 1. K2:
  k=3 not below k=1 (Wilcoxon p at least 0.01). K3: wall ratio above 1. Added:
  growth slope CI not below 0.
- **Budget.** Stage 0 under 2, stage 1 4, growth 3 core-hours.

### T2. Boundary-keyed cavity summaries (single field: compilers) · proposed

- **Hypothesis.** At R7 most of `bb-mrv`'s dead nodes are refuted by a k x k
  cavity, k at most 3, given their boundary and pool, and those boundary keys
  recur, so tabulated refutations (sound by pool monotonicity) cut work.
- **Borrowed from.** Reps, Horwitz, Sagiv, POPL 1995 (summary edges). The glass
  framing (Montanari and Semerjian 2006) is kept as the stage-0 readout only.
- **Nearest cell, missing mechanism.** No-good learning (mapped). Missing: keys
  bounded to a window boundary. Prior: Mocsi's lookup table (`reported`,
  never worked); Chu et al. 2010.
- **Cheap test.** Stage 0 is the shared census (3a), oracle B plus raw and
  canonical key recurrence (ST's readout folded in). Stage 1: four arms as
  proposed on R6/R7 fresh seeds, then the +2 growth track.
- **Baseline.** Better of `bb-mrv` and scb; arm 2 (lookahead, no table) and
  arm 4 (random-shape keys).
- **Kill.** K0a: under 30% of weighted dead nodes with r at most 3 at R7.
  K0b: key recurrence under 10%. K0c: r at R7 not above r at 7x7 c=8. Stage 1:
  G1 NO-GAIN on work at R7, wall ratio above 1, arm 3 not below arm 2.
- **Budget.** Census share about 1, stage 1 1 to 3, growth about 3 core-hours.

### T3. Cut-keyed trace partitioning · proposed

- **Hypothesis.** `bb-mrv` loses at the critical rungs by joining cut-colour
  cases at each cell; keeping partitions keyed on 2 cut-edge colours,
  persistent through the descent, closes failing subtrees at their roots.
- **Borrowed from.** Rival and Mauborgne, TOPLAS 29(5), 2007.
- **Nearest cell, missing mechanism.** Arc consistency (mapped). Missing: a
  relational, persistent state keyed on an auxiliary cut quantity; POAC
  (Bennaceur and Affane 2001) is the control.
- **Cheap test.** Stage 0 is the shared census, oracle D (cut key, random key,
  POAC at matched budget), plus the full replay on bb-mrv trees with
  persistent partitions. Correction: report replay work (surviving nodes times
  live partitions times propagation cost), not nodes alone.
- **Baseline.** `bb-mrv`, scb; C1 random key, C2 POAC.
- **Kill.** K1 replaced: replay work ratio above 1 at R7, or replay node ratio
  above 0.5. K2: C1 within 0.1 of the cut key at both rungs. K3: POAC at or
  below the cut key. Stage 1: G1 NO-GAIN at R7, wall ratio above 1, at most
  baseline + 2 solves. Added: growth slope on the +2 track.
- **Budget.** Census share about 0.5, stage 1 2 to 3, growth about 3
  core-hours.

### T4. Relaxed decision diagrams as a score bound in break-tolerant DFS · proposed

- **Hypothesis.** A width-W relaxed DD that keeps frontier colours exact and
  relaxes piece distinctness bounds the best score reachable from a prefix
  tightly enough to prune break-tolerant DFS before the endgame.
- **Borrowed from.** Bergman, Cire, van Hoeve, Hooker, IJOC 28(1), 2016; Cire
  and van Hoeve, Oper. Res. 61(6), 2013.
- **Nearest cell, missing mechanism.** Break-tolerant DFS (saturated); no
  bound in `break_solver.rs` lines 15 to 24. Missing: a per-node score bound.
  Record engines' code was not inspected; their absence of a bound is
  `reported`.
- **Cheap test.** Stage 0 bound gap by rows remaining and W in {64, 512,
  4096}; report the share of merges that relax frontier colours. Correction:
  stage 1 "prove no board at optimum + 1" is vacuous on planted boards;
  replace with work to reach optimum minus k (k = 1..4) on 8x8 c=8 boards
  where `BreakSolver` stalls, with and without the bound (L4's metric).
- **Baseline.** `BreakSolver` with its schedule; control W = 1.
- **Kill.** Stage 0: median gap with 3 rows left above 4 edges at every W.
  Stage 1: work ratio above 0.5 or wall ratio above 1 at any k.
- **Budget.** 4 to 8 core-hours.

### T5. Band consistency by composed tile transducers · proposed

- **Hypothesis.** GAC over a k-row band (k = 2, 3), built by composing the row
  transducer of the cells' current domains, removes values arc consistency
  keeps because it is exact on the band's cycles.
- **Borrowed from.** Jeandel and Rao, Adv. Comb. 2021 (transducer section to
  be checked); Pesant, CP 2004 (forward-backward filtering).
- **Nearest cell, missing mechanism.** Arc consistency (mapped). Missing:
  cycle-exact consistency computed from current domains during search.
- **Cheap test.** Stage 0 is the shared census, oracle A, with oracle B on the
  same bands as an exact-distinctness ceiling, so the cost of the relaxation
  is measured, not argued. Stage 2 as proposed, then the +2 growth track.
- **Baseline.** Better of scb and `bb-mrv` (correction: the proposal names scb
  only as baseline); k=1 control; full-piece-set relaxation control.
- **Kill.** Census: k=2 proves dead under 5% of weighted dead nodes at both
  rungs. Stage 2: G1 no gain at R6 and R7, or CPU ratio above 2 at both.
  Mechanism: k=1 within the k=2 CI. Added: growth slope CI not below 0.
- **Budget.** Census share about 0.5, stage 2 about 3, growth about 3
  core-hours.

## Evolved-code proposal (brief, idea slot step 4)

### E1. Evolve bb-mrv's cell and value ordering code · proposed

- **Hypothesis.** A search over programs for the two ordering hooks of `bb-mrv`
  (which cell next, which value first), scored automatically, finds an ordering
  that beats MRV with index value order on work at the critical rungs and on
  growth with n.
- **Borrowed from.** AlphaEvolve (Novikov et al., arXiv:2506.13131): language
  models mutate code, an automatic evaluator scores every program, an
  evolutionary archive keeps the survivors. Open implementation: OpenEvolve.
- **Nearest cell and missing mechanism.** "Corpus priors, learned move ordering"
  (saturated) and dynamic fill orders (saturated): those learn weights for a
  fixed form of heuristic, or pick among hand-written orders. Missing: a search
  over the ordering program itself, against a scorer outside its reach.
- **Cheap test.** By PROTOCOL_TARGETS_PROPOSAL.md section 4. Evolved code plugs
  into the ordering hooks only; the harness, `NodeCounter` and verifier are out
  of reach. Evolution sees calibration seeds only (R6, R7, 8x8 c=10) and scores
  on solves within a fixed work budget. Selection of the final program uses G1
  on work and the growth slope on test seeds, plus 9x9, a size never seen in
  evolution.
- **Baseline and controls.** `bb-mrv`; scb at R6 and R7. Controls: the best of
  an equal number of random mutations without selection (is it search or
  luck?), and tonight's A21 result as the hand-written value-order reference.
- **Kill.** The selected program fails G1 on work against `bb-mrv` at 7x7 c=6
  test seeds, or its growth slope CI is not below 0, or its gain on test seeds
  is under half its gain on calibration seeds (overfitting to the seeds it
  saw).
- **Budget.** About 500 candidate programs at about 2 core-minutes each: 15 to
  20 core-hours, plus model usage for the mutations, to be estimated and
  approved before launch. CPU only.
- **Dependency.** Tracks 2 and 3 of tonight measure two hand-written points in
  this space (backjump rule, impact value order). Read their verdicts before
  sizing this.

## Lead's recommendation for the next round

Not a result, an ordering to discuss:

1. The census (3 core-hours): it closes or opens three top-five entries at once.
2. The partial-match ladder as a measurement track: it is what makes any
   470-regime idea judgeable.
3. Rank 1 stage 0 (under 2 core-hours) and rank 4 stage 0 (bound gap), each
   cheap and independent.
4. An evidence audit of `dead-ends.mdx` and EXPLORED.md: the judge found
   entries labelled "Measured" with no committed result, and three "where it
   stopped" figures that are uncommitted (see OBSERVATIONS.md).
