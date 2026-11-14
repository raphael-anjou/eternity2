# Explored territory

**Grades.** *measured and judged*: committed data, ruled on by an independent judge; *measured*: a committed result or reproducible command in this repo, not judged by the program; *argued and refereed*: a derivation checked claim by claim by an independent referee; *argued, not refereed*; *reported*: someone else's number, cited, not rerun here; *claim without committed data*: an own run or argument with nothing committed.
**A "killed", "saturated" or "mapped" whose grade is not measured is a lead, not a verdict:** a new idea inside that cell opens the cited page, states what the earlier attempt lacked, and may proceed.

What has already been searched heavily on Eternity II, compressed from
`web/content/research/build/approaches-map.mdx`, `build/dead-ends.mdx`,
`why/walls-and-methods.mdx`, `open-problems.mdx`, the lab experiments and
`research/LEDGER.md`. A new idea that lands inside one of these cells is a
**variant**, not a new direction. Variants are allowed only when they name the
specific mechanism the earlier attempt lacked and the page that recorded where
it stopped.

Status: **saturated** (many strong attempts, a measured stop), **mapped**
(tried here or in the community, stop understood), **thin** (tried once or
only argued about), **open** (listed on `/research/open-problems`). Statuses
predate the 2026-11-12 evidence audit and are kept as written unless a row says
otherwise; the grade column is the audit's
(`night/2026-11-12/EVIDENCE_AUDIT.md`, part 3), checked on 2026-11-13 by
opening the evidence of every row graded measured. By the audit, 8 of the 40
original rows have a measured stop.

The **scope / what would reopen it** column carries what the cited page or a
later result says the stop does not cover. **Disputed** marks a row where two
sources disagree and the disagreement could not be settled on 2026-11-13; both
statements are written, with pointers. Night results are on local branches
named `night/2026-11-12-t<N>-<name>` (not merged, not pushed; see the ledger
entries of 2026-11-12). The fuller index of routes, with every source, is
`night/2026-11-13/ROUTES.md` (a draft).

## Systematic search

| Family | Status | Where it stopped | Grade | Scope / what would reopen it | Page |
|---|---|---|---|---|---|
| Fixed / dynamic fill orders, comb search | saturated | lever on tree size, not on ceiling | reported (community msgs 2714, 5263); only the mean-field law is measured (`just research-fill-order-mean-field`); "not on ceiling" is not on the page | Own comb producer beat row-major at the top in notebook runs only (457 against 455). Track 5 (judged): at 8x8 c=10 row-major DFS solves 24/30, column-major 20, five other orders 0, so small-board order effects are large and order-specific. A refereed literature check found that a planted-jigsaw manuscript recommends growing a rectangle with both sides at least two, not a single row. Reopen: a measured order comparison above 9x9 with a growth stage, or a planted-jigsaw style order | build/backtracking/fill-order |
| Break-tolerant DFS with scheduled late mismatches | saturated | 467 (Verhaard), 470 (Blackwood) | reported (community results, not rerun) | The 470 came from many cores. This repo's Blackwood-style port differs from the published engine in keying, non-physical pieces in the break path, an isolation rule, the centre piece, scan order and an orientation defect (Track 13, measured and judged; two of these disputed by the 2026-11-13 port-fidelity report, not judged). Reopen: a faithful port measured (AGENDA A32) | build/solvers, records |
| Restart portfolios, heavy tails | saturated | changes which boards, not the ceiling | reported (msg 2822: 500 random 7x7, one run per policy); the stop phrase is not on the page | "Nobody has measured that distribution for the full 16x16" (page). Reopen: a measured runtime distribution on the 16x16 | build/backtracking/restarts |
| Arc consistency, Régin all-different | mapped | pruning fades about two cells out | measured (checked: `research/experiments/dfs-study/results/report.md`, 60 s mean score on corner-pinned 16x16 variants: MRV-FC 322.2, MRV-AC3 320.6, plus per-colour Régin 320.6); the "two cells out" phrase is a 2008 community remark (msg 6208), reported | The measurement is score at a fixed budget, not pruning distance. Piece-level Régin was never built; AC-2001 never built; hard AC is unsound under break budgets. On the real pieces arc consistency over all 196 interior cells kills 0.4313 of shuffled border words against 0.4293 for the first ring alone (frame referee's measurement). Reopen: piece-level all-different or clause learning on partial frames (track D1, running 2026-11-13) | build/reduce/* |
| No-good learning, shared transposition tables | mapped; **disputed** | no-goods outrun memory at 16x16 | reported (msgs 6145, 9618, about 1% each) | Statement 1 (this map): mapped, no-goods outrun memory. Statement 2 (the cited page, audit): the page has a failure lookup table, a deletion-minimised 28-piece core and a 4.4x cut on the 6x6 ring, and says no-goods pay on small boards, enumeration and endgames. Measured since (Track 6, judged): on `bb-mrv` trees at 7x7 c=6, refuting boundary keys of small cavities recur at 46% but a stored refutation is soundly reusable at only 13.5% of refuted nodes, and those nodes carry 0.8% of the dead-node weight; boundary-word lemmas proved 0 of 1,447 large dead subtrees. A 2026-11-13 memo (argued, not refereed) finds no proof that a nogood-driven frame search cannot beat row scan. Pointers: `build/reduce/nogood-learning`, `night/2026-11-12-t6-dead-node-census`, `night/2026-11-13/memos/nogood-frame-search-price.md`. Settles it: track D1 (cores of wrong partial frames on the real pieces) | build/reduce/nogood-learning, dead-ends |
| Exact cover / DLX | mapped | small boards and counting only | claim without committed data (the project's own XCC "spent days broken" past small boards; audit: status too strong) | Soft edges as minimum-cost exact cover never tried. Reopen: a working XCC at 8x8 or above | build/exact/exact-cover-dlx |
| Meet in the middle (BANDSAW) | mapped, open | about 20x memory per extra mismatch | claim without committed data for the 20x (the bandsaw page says its instances, budgets and table sizes are not committed); verdict argued and refereed | Resolved 2026-11-13 (flag 4 of ROUTES): two sources, same verdict, different numbers. The refereed price review (`night/2026-11-12/memos/price-and-ledger.REVIEW.md`) bounds the best two-list seam join at a gain of 0.8 to 1.0 digit, at most 2.4 for any band join storing both lists (strict model, first moment); the 20x per mismatch is uncommitted and should not be quoted. A soft interface was designed and never run; a column seam never tried. Reopen: information about piece zones, or a first BANDSAW soft-interface run | build/exact/meet-in-the-middle |
| Exact tail endgame | mapped | plateaued | measured (checked: `research/topics/exact-tail-endgame/results/tailforge.json`, `just research-exact-tail-endgame`) | Exact beat the producer on 24 of 24 seeds by 1 to 9 edges; "plateaued" is only the page's frontmatter; no non-exact control was run; record-level producer not reproduced. Whether a record engine spends its cost in the tail is undecided (see "Where a record's cost is paid" below). Reopen: a search whose cost sits in the tail | build/exact/exact-tail-endgame |
| Solver engineering, JIT codegen | saturated | speed only; more attempts for record hunting | reported (msgs 10056, 11657); JIT speed measured (dfs-study codegen) | "Speed does not change quality" is one uncommitted 5-minute budget point; the page asks for a 1x, 10x, 100x budget sweep, not run. The night's Track 0 (judged) made the reference solver 5.3x to 6.1x faster, node-identical | build/faster/*, lab/.../jit-backtracker |

## Encodings

| Family | Status | Where it stopped | Grade | Scope / what would reopen it | Page |
|---|---|---|---|---|---|
| SAT / CSP encodings, cube and conquer | mapped | full board stalls; sub-board proofs | reported; sub-board UNSAT measured (`research/topics/rigidity-sat-halo/results/halo-sat.json`) | Resolved as scoped 2026-11-13 (flag 7 of ROUTES), by opening both pages: `build/dead-ends` reports one community attempt at cube-and-conquer on the 16x16 (May 2026, one lookahead cuber with kissat, one encoding, one budget) that worked up to about 8x8 and did not scale; `build/exact/sat-csp-encodings` says it "has not been attempted at scale". Both hold if "at scale" means Pythagorean-triples-scale compute; the SAT page should cite the reported attempt. Other cubers, encodings and budgets are untested; the CDCL-stall mechanism is a hypothesis. Reopen: clause learning on partial frames of the real pieces (track D1) | build/exact/sat-csp-encodings, dead-ends |
| LP / ILP relaxations, relaxation bounds | mapped | 420 to 440, ILP wall at 8x8 | reported (msgs 6905, 5602) | The 420 to 440 is one 2009 solver's stall, "a property of that solver, not of integrality" (page); a 10x10 6-colour ILP was solved in about 19 min; conditional 478 bounds are notebook only. Reopen: an assignment-bound weighted CSP on the maximisation form (see Reformulations below) | build/exact/lp-relaxations, dead-ends |
| MIP halo proofs of rigidity | mapped; **disputed** (meaning, not data) | record basins locally optimal | measured (checked: `halo-sat.json`, Chebyshev halos around the five public record boards, kissat; some radius-4 solves time out and are left open); the MIP table is not committed | Statement 1 (rigidity page and this map): record basins are locally optimal, a property of the records. Statement 2 (`night/2026-11-12/memos/solve-path-4-last-ten-edges.md`, argued, not refereed): local optimality is the null expectation (rigidity predicted at odds 10^44 to 10^54), so the halo result agrees with the data and says nothing specific about records. Also flagged in ROUTES: the rigidity page's timing claim (218 freed cells refuted in 0.31 s) against committed data, not rechecked here. A disconnected better arrangement is not ruled out. Settles it: a referee for path 4, or an exact neighbourhood that moves a record | why/rigidity-wall |

## Local and stochastic search

| Family | Status | Where it stopped | Grade | Scope / what would reopen it | Page |
|---|---|---|---|---|---|
| Local search, ALNS, hyper-heuristics | saturated | 461 academic, 460 class here | reported (461 is Wauters, msg 9023; neither number is on the cited page); the stall is measured in `research/experiments/repair-study/results` | Budgets far beyond a minute not tested; the temperature-insensitivity result is uncommitted. Track 7 (judged): on generated 8x8 boards ALNS from a greedy start was the weakest of five entrants | build/local-search/local-search-alns |
| Simulated annealing, parallel tempering | saturated | below the backtrackers; PT the only new-basin source | claim without committed data (ladder sizes, temperatures and budgets not committed; "only new-basin source" is not on the cited page) | Measured and judged elsewhere: A4, PT finds more distinct basins than MAP-Elites (22.33 against 5.63 at 8x8). The page reports that with the top temperature near 30 the crossing happened. Reopen: adaptive multilevel splitting (AGENDA A29), coupled replicas (A11) | build/local-search/parallel-tempering |
| Beam search, beam producers | saturated | breadth stalls in the interior | claim without committed data (16x16 figures are notebook runs; only planted 10x10 and 12x12 committed) | The page puts the wall in the last rows, not the interior; comb producer 457 raw; beam beat DFS and LDS at equal nodes. Reopen: committed 16x16 beam runs | build/construct/beam-search, lab/.../beam-width |
| Genetic algorithms, crossover, recombining two good boards | mapped | crossover fights the structure | reported (msgs 565, 1311, 2360, 5589, 5950) | Problem-aware recombination not tested by this project; the 2008 GA plus backtracker reached 462 to 463. Reopen: partition crossover run here | build/local-search/evolutionary, dead-ends |
| Iterated maps / divide-and-concur | thin until 2026-11-12; now killed at its cap near critical | tested once (old); 0 of 30 solved at 7x7 and 8x8 near critical (Track 11) | measured and judged (Track 11); the earlier "tested once" was a graph-colouring test, not E2 | Resolved 2026-11-13 (flag 8 of ROUTES): path 6 (`solve-path-6-free.md`, reviewed, small and weak) solved over-coloured planted boards cheaply; Track 11 found 0 of 30 near the critical colour count at every band width k = 0..3, no band effect at 6x6, and 1 of 270 runs at 8x8 c=10. Different regimes, both hold. Kill covers beta 0.25, cap 20,000 iterations; the band-width exponent was not measured. Reopen (judge): a different mechanism, a band projection that lowers iterations at 6x6 against k = 0 before any scaling claim, not a larger cap | build/exact/iterated-maps |
| Ant colony, MCTS with cheap prior, gradient descent on a soft board | mapped | dead ends | claim without committed data (ant: one configuration, one seed; MCTS: only the faithful port; gradient: four seeds of one Sinkhorn schedule) | "This is one schedule of it, not the whole family" (page) | dead-ends |
| Fake 480 and repair, repair seeded by exact prefix | mapped | dead ends | claim without committed data for the stop (reported 463; one seed, one schedule, one preset) | The committed repair study found the DFS-seeded start the best variant of the study (mean 446). Reopen: a new repair neighbourhood family (AGENDA A6, A14) | dead-ends, lab/.../repair-study |

## Learning

| Family | Status | Where it stopped | Grade | Scope / what would reopen it | Page |
|---|---|---|---|---|---|
| Corpus priors, learned move ordering, anti-pattern mining | saturated | fast to the plateau, not past it | claim without committed data (private corpus, few seeds, no equal-compute baseline) | The page says "getting past the plateau is open, not ruled out"; no experiment re-mined its own outputs; annealed or adaptive weights untried. Reopen: AGENDA A31 (evolved ordering code) or annealed weights at equal compute | build/learning/* |
| PRIOR, PALIMPSEST, LODESTONE, KEYRING, REPLAY | mapped | 451 to 463 | measured, artifact re-verification only (checked: `research/topics/record-boards/results/verified-scores.json` re-verifies stored boards 463, 460, 458, 460; it does not rerun the searches) | 463 has no random-break control; KEYRING's basin not ablated; REPLAY's two-break engine ties one-break at equal wall clock | lab/.../learning/* |
| Transformer board generation, heuristics learned on small boards | mapped | dead ends | claim without committed data (one model architecture) | Other architectures untested | dead-ends |

## Propagation and inference

| Family | Status | Where it stopped | Grade | Scope / what would reopen it | Page |
|---|---|---|---|---|---|
| Survey propagation, belief-propagation ordering | mapped | signal fades two cells out | reported (msg 6208, 2008); own SP test one 4x4 and three settings, uncommitted | On the real pieces arc consistency sees nothing past the corner windows (frame referee's measurement), which supports the stop for propagation; BP itself was not run on partial frames. Reopen: BP on partial frames of the real pieces | dead-ends |
| Tensor-network counting, lattice reduction | mapped | dead ends | claim without committed data (one full-scale attempt died at 3 GB) | Night path 1 (`solve-path-1-counting-algebra.md`, argued, not refereed): colour-only contraction needs about 10^19.85 frontier states; the dead-ends page's Lagrangian patch solved 2 of 19 toy boards when rerun. Reopen: a counting object smaller than the tree | dead-ends |
| Pattern databases, macro-tiles, local cluster enumeration | mapped | dead ends | claim without committed data (macro-tiles: Zajc 2008, reported) | Interior-node pattern databases "untested here" (page); macro-tiles are a real constant-factor win. Track 6 (judged) bounds small-region refutation on `bb-mrv` trees: cavities up to 3x3 prove 0.8% of dead-node weight at 7x7 c=6 | dead-ends |
| Forbidden-pattern / border-supply / Eulerian closure pruning | mapped | no measurable gain in search | claim without committed data (one 10 s run; the 99.72% density is committed, the search test is not) | The null result is for strict, matching-only search; "on a mismatch-tolerant search the check is untested here", and record engines are mismatch-tolerant. Reopen: a test inside a break-tolerant engine | dead-ends, why/forbidden-patterns |

## Construction orders and geometry

Perfect rows, centre-out, concentric rings, clue-to-clue corridors, colours
first then pieces, forced diverse borders, fluid frame, frame-is-not-the-basin:
all **mapped** (dead ends or plateaued; see dead-ends and the lab pipelines
CAS, CLOISTER, GAUNTLET, LADDER, MIDDEN, MOSAIC, STAGED). Per row:

| Family | Status | Grade | Scope / what would reopen it | Page |
|---|---|---|---|---|
| Perfect rows | mapped | claim without committed data (single-run proof of concept) | Beams of 256 or 1,024 not tried | dead-ends |
| Centre-out | mapped | claim without committed data (generated 5x5 to 8x8, one solver's scan orders) | Audit: status too strong. Tested only on 5x5 to 8x8 | dead-ends |
| Concentric rings | mapped | claim without committed data (one schedule, one ordering, one run per budget); related committed material exists in `research/topics/cas-annular/`, not linked from the entry | "Not that layered orders are refuted in general" (page). Audit: status too strong | dead-ends |
| Clue-to-clue corridors | mapped | claim without committed data for the A/B verdict; corridor counting committed (`research/topics/clue-corridors/results/corridor_counts.json`) | One greedy filler, 20 s runs, 12 seeds | why/clue-corridors |
| Colours first, pieces later | mapped | reported (msg 11182) | | dead-ends |
| Forced diverse borders | mapped | reported (Millilaw); own 100-border test single seed, uncommitted | "A cheap-filter screen, not an exhaustive refutation" (page) | dead-ends |
| Fluid frame | was "dead end"; corrected 2026-11-13 to: mechanism measured, no board produced | measured (checked: `research/topics/frame-manifold/results/manifold.json`, `just research-frame-manifold`) | Resolved 2026-11-13 (flag 6 of ROUTES) by opening the page and the data: 180 of 500 perfect frames (36%) have an interior cell no piece can fill, and free rim exchanges revived 60 of 60 sampled dead frames. That is a working mechanism, not a dead end; the page re-measures the mechanism only and produces no completed board | lab/.../pipelines/fluid-frame |
| Frame-is-not-the-basin | mapped | measured (checked: `research/topics/frame-is-not-the-basin/results/frame_first.json`) | One producer with a ceiling near 440: "whether a near-record producer would see a border effect is not tested here". Reopen: a near-record producer tested for a border effect | why/frame-is-not-the-basin |
| Lab pipelines CAS, CLOISTER, GAUNTLET, LADDER, MIDDEN, MOSAIC, STAGED | mapped | measured for CAS (checked: `research/topics/cas-annular/results/cas_plateau.json`) and for the GAUNTLET and STAGED boards (record-boards artifacts); CLOISTER, LADDER, MIDDEN, MOSAIC have no reproduction command | GAUNTLET (458) and LADDER (451) are tagged `outcome: new-basin` on their pages, not plateaued | lab/.../pipelines/* |

## Hardware

GPU (measured ports, no tuned full-board solver: **open**), FPGA (designed,
never driven: **thin**), distributed farms (**saturated**), quantum (**thin**,
zero runs). Per row:

| Family | Status | Grade | Scope / what would reopen it | Page |
|---|---|---|---|---|
| GPU | open | reported (hobby ports, msgs 9811, 11598, 11835; none rerun here) | Speeds are a floor; about 100x one core in 2026. Reopen: AGENDA A8, a measured GPU backtracker | build/hardware/gpu-solving |
| FPGA | thin | reported (msgs 9236, 9237, 10649) | "Designed, never driven" is too weak: a prototype ran on real silicon and walked the benchmark tree; no full-puzzle run | build/hardware/fpga-solving |
| Distributed farms | saturated | reported | Fleets solved finite targets (10x10 in about 180 core-years); a live five-clue fleet is under test, so "saturated" is stronger than the page | build/faster/distributed-solving |
| Quantum | thin | argued, not refereed (cost figures are arithmetic; no member ran any hardware or simulator) | "One unfinished embedding attempt, zero runs" | build/hardware/quantum |

The night's price (below) sizes every hardware route: the gap from row scan
to 1,000 core-years is 17 to 20 orders, and every credible hardware
multiplier together is worth about 2.5 orders, range 1.5 to 4 (argued in
`solve-path-5-price.md`; its referee did not establish the ranking factor
inside that figure, and puts the gap at 19 to 20 orders without it).

## Theory already on the site

Phase transition and hardness peak, entropy area law, rigidity, sigma cycles,
no height function, permutation-code wall, flux invariants, ring purity, border
balance, parity defect floor, piece theft, constraint immediacy, rare-colour
geography, the 470 annealed landscape. Several of these still lack a matched
null model (see the deferred findings in `research/LEDGER.md`).

Grade: measured for the results committed under `research/topics/*` (audit
part 3); argued and refereed for the night's rulings. The theory referee of
2026-11-12, confirmed by a second referee (`night/2026-11-12/memos/theory-referee.md`,
`theory-and-frame.REVIEW.md`), found three pages wrong (entropy-area-law: the
limit is close to 0.435, not 0.67, and distinctness falls below 1e-3 near 46
cells, not 155; parity-defect-floor: at least about 154, not "at most 76";
how-hard-is-this-instance: no crossing at N = 12 to 13) and three overstated
(sigma-cycles, design-recipe, why-e2-is-hard); 3 of 5 sampled "sound" pages
were one grade too kind (irreducible-hard-region, complex-theory,
rigidity-wall). Sentence-level corrections are in section 6 of
`theory-and-frame.REVIEW.md`; none was applied to this file.

## Judged program results (rounds 1 to 3 and the night of 2026-11-12)

Routes this repo's research program tested with a preregistration and an
independent judge, or priced with an independent referee. Rows marked
"night" were examined during 2026-11-12 and 2026-11-13; AGENDA entries carry
the full result lines.

| Family | Status | Where it stopped | Grade | Scope / what would reopen it | Where |
|---|---|---|---|---|---|
| Row-scan DFS as the price of a full solve (night) | priced out | 10^40.457 nodes with five clues on the real pieces (interval 40.440 to 40.473), 10^47.14 one clue, 10^50.04 none; model 10^40.49 | measured and judged (Track 10); model argued and refereed | Row scan, strict matching, plain DFS; node = valid partial board. Earlier 10^45.3 and 10^47.7 were a script error. Gap to 1,000 core-years 17 to 20 orders. Reopen: about 56 bits of information about the solution, or an order that changes the exponent | t10-real-puzzle-profile; `memos/price-and-ledger.REVIEW.md` |
| Frame-first enumeration (night) | priced out; **disputed** | 10^37.6 frames x about 10^8.3 nodes per refutation = 10^45.9, 5.5 digits above row scan | argued and refereed (model; all frames enumerated on five 6x6 boards) | Statement 1 (refereed): refuting a wrong frame costs about what confirming the right one does. Statement 2 (`night/2026-11-13/memos/nogood-frame-search-price.md`, argued, not refereed): the 10^8.3 is a model mean that may hold only for the true frame, and a uniformly drawn frame dies on a corner block with a core of 1 to 4 cells. Neither is measured on the real pieces. Settles it: track D1 stage 0 (cost to confirm a right frame, and to refute frames that pass the four corner blocks) | `memos/price-and-ledger.REVIEW.md`; branch `night/2026-11-13-d1-frame-cores` |
| Cheap information about the frame: corner blocks, first ring, arc consistency (night) | killed for local and corner tests | about 20 cheap bits, all at the corners (17.45 exact on the official pieces); 91 more needed for 1,000 core-years | argued and refereed, with a referee measurement on the real pieces | Struck by the referee: "the corner bits are already inside the price" (two corners only) and the 6x6 refuter result (arc consistency plus all-different refutes 94.4% to 100% of wrong 6x6 frames; 6x6 says nothing about 16x16). Open: all-different reasoning and clause learning on partial frames (track D1). Candidate: AGENDA A34 | `memos/frame-information.md`; `memos/theory-and-frame.REVIEW.md` |
| Traces of the maker's program: numbering, stored orientation, piece statistics (night) | killed for now | numbering is exactly a sort (0 inversions in 20,656 pairs); stored orientation is the canonical form; about 25 statistics, none past the multiple-comparison line | argued and refereed (2026-11-12 path 2; 2026-11-13 construction memo, refereed) | At most 42.7 bits untested (order within colour-count tiers), with no mechanism. The coarse stratified-painter loophole is invisible in the piece set and testable only by search under a guessed painter (seed-recovery odds struck as a guess) | `memos/solve-path-2-designed-instance.md`; `night/2026-11-13/memos/how-the-puzzle-was-made.REVIEW.md` |
| Planted-solution recovery by spectral pair statistics (A1) | killed | spectral ranking below colour rarity (z -7.94 at 8x8, -5.91 at 10x10) | measured and judged | Generated boards. Path 2's theorem rules out design traces, not solution marginals. Reopen: a statistic using solution marginals (A18) | AGENDA A1 |
| Backdoors: growth (A3), backdoor-guided branching (A16) | A3 promoted (measurement); A16 killed | A3 exponent b = 0.589 [0.549, 0.626]; A16 adds +0.010 AUC over domain size, NO-GAIN in nodes | measured and judged | Backdoors found with the planted solution in hand; at 8x8 and up k is a greedy upper bound | AGENDA A3, A16 |
| Quality-diversity over basin descriptors (A4) | killed | PT 22.33 against MAP-Elites 5.63 distinct basins at 8x8 | measured and judged | Generated 8x8 and 10x10. Reopen: a better descriptor set | AGENDA A4 |
| Overlap gap / near-codeword spectrum (A10) | killed on small boards | 5x5 C=13 mid band 4 breaks against far band 7 | measured and judged (second judge) | Resolved as scope 2026-11-13 (flag 10 of ROUTES): the kill holds for single-solution-class 5x5 and 6x6 boards; path 4 (argued, not refereed) argues it does not transfer to 16x16. The barrier between two distinct solutions is untested (A10's named next step) | AGENDA A10 |
| ProbCut statistical cutoffs (A12) | killed | V1 13/30 against a kill line of 15 at 8x8 c=10; cuts removed planted branches | measured and judged | 6x6 and 8x8 c=10. Track 5 found A12's `fc-luby` seeds 2k and 2k+1 identical (a code defect, unfixed). Reopen: a cutoff with a sound error bound | AGENDA A12 |
| Non-chronological unassignment (A15) and blocker eviction inside propagation (A20, night) | killed (these implementations) | A15 loses to propagating DFS by two orders of magnitude; A20 4.7x to 9.2x more nodes at 6x6, 5 of 30 solved at 7x7 against 16 and 23 | measured and judged | What ran is a repair search (24 to 31% latest-decision fallback, partial restarts). Not covered: conflict-directed backjumping proper, conflict sets from propagation explanations, stored no-goods | AGENDA A15, A20 |
| Hall matching / cut-multiset pruning (A17, night Track 4) | killed on `bb-mrv` | under 1% fewer nodes (0.992 to 1.000), slower wherever it prunes | measured and judged (both rounds) | Resolved 2026-11-13 (flag 2 of ROUTES): round 2's GAIN (0.900 at 6x6) was against `scb`, which lacks piece all-different; Track 4's kill is against `bb-mrv`. Both hold; the current verdict is the one on the stronger baseline | AGENDA A17 |
| Propagating DFS lead and its growth with n (A22, night Tracks 0, 1, 5) | undecided; resolved as "no evidence of growth" | A22: `bb-mrv` 30/30 against 20/30 at 8x8 c=10 (G1 0.147); Track 1: 6 of 8 9x9 pairs capped at 1e8 | measured and judged (A22, Tracks 0, 1, 5); growth-law referee's runs argued and refereed | Resolved 2026-11-13 (flag 1 of ROUTES): the sources do not conflict. A22 itself found no evidence of a growing gain against `scb`; the growth-law referee found `bb-mrv`'s node lead over plain DFS does not grow (paired medians 9.9, 15.7, 5.4 at n = 6, 7, 8); Track 1 ran out of cap. In wall time plain row-major DFS wins at 8x8 (about 1% of `bb-mrv`'s time, Track 5) and solves 9x9 c=11 in 2 to 6 s (referee). Reopen: a 9x9 or 10x10 run scored on work or time with plain DFS or `bb-mrv-fast` as reference | AGENDA A22; t0, t1, t5 |
| Value order by propagation impact (A21, night Track 3) | killed | node ratio 1.075, 0.829, 1.096 at 6x6, 7x7, 8x8 | measured and judged | Count of option bits, least first; not a log-product impact. `bb-mrv`'s fixed value order lies inside the spread of random orders | AGENDA A21 |
| Small-region reasoning: cavities, bands, pockets, boundary-word lemmas (night Track 6) | killed at stage 0 (three proposals) | at 7x7 c=6, of dead-node weight: cavities to 3x3 0.8%, 2-row bands 0.2%, pockets 3.5%, partitioned propagation about 10% | measured and judged | `bb-mrv` parent, 6x6 and 7x7 critical rungs; shares are upper bounds on savings with unchanged branching. Every oracle's share rises steeply with depth. Not measured: the `scb` parent. Reopen: a check strong before the peak | AGENDA A24, A27, A28 |
| Cut-keyed trace partitioning (night Track 12) | killed | a partition-matched random key beats the cut key on 30 of 30 instances at 7x7 (0.419 against 0.486 of `bb-mrv`'s nodes); work ratio 1.21 | measured and judged | 6x6 and 7x7, MRV branching (where the cut line is close to arbitrary). Untested: a cut key under a row-scan solver | AGENDA A25 |
| Relaxed decision-diagram score bound (night Track 8) | open (stage 0 passed on the threshold); **undecided** | median gap exactly 4 edges at 7x7 with 3 rows left; 1, 2, 4 edges at 2 rows left for n = 6, 7, 8 | measured and judged (stage 0 only) | Resolved 2026-11-13 (flag 12 of ROUTES) by opening Track 8's JUDGE.md: the judge kept stage 0 and did not recommend against a build; the "not worth a build for the record-style engine" reading rested on Track 10's late share, which describes a differently keyed port, and the lead withdrew it (REPORT.md). Status: undecided. Gap measured only on deeply dead prefixes. Reopen: stage 1 near the optimum with a wall-clock arm, and the late share of a faithful engine | AGENDA A26 |
| Where a record engine's cost is paid (night) | undecided | four numbers, four different engines (below) | measured and judged (Tracks 10, 13); argued and refereed (memo reviews) | Resolved as scoped 2026-11-13 (flag 11 of ROUTES), not settled: Track 10, 4.5e-7 of nodes late at 1e10 on a port privileging colours 22, 21, 20 (budget-dependent); partial-match referee, 92% to 98% late from one model row; record-cost referee, 0.4% to 53% of legal nodes late across seeds and clue settings with the published keying; Track 13 confirms the spread. None describes the published engine faithfully ported. Reopen: that port, then a histogram by depth | t10, t13; `memos/where-a-record-is-paid.REVIEW.md` |
| Record-style port fidelity and colour keying on the real pieces (night Tracks 13, 14, 15) | kept as measurements; Track 14's family killed | keying alone 0.7 to 1.3 on nodes per arrival and slower per second; legal break path 7.63x more legal deep nodes per node; with five clues the quota share, not the colours, decides whether depth 201 is reached | measured and judged | This port, 1e9 nodes per run, one attempt per opening, top-left row scan. Port-fidelity fixes running 2026-11-13 (track D2, reported, not judged). Reopen: the quota share sweep (AGENDA A32) | t13, t14, t15 |
| Difference map with exact band projections (night Track 11) | killed at its cap | see the iterated-maps row above | measured and judged | | t11-difference-map-bands |
| Small-board ladders as a proxy for 470 against 480 (night Track 7) | killed as preregistered (this pool and budget) | 2 of 5 entrants in the 2% to 4% gap band, 3 needed; at 10x10 and 12x12 every entrant stalls 5% to 12% below the optimum | measured and judged | Reopen: entrants that keep searching after a first complete board (AGENDA A33) | t7-partial-match-ladder |
| Decompositions, joins, counting and algebra (night paths 1 and 3) | priced out | best join gain 1.0 digit, at most 2.4; transfer matrix with used set as large as the tree (10^43 to 10^50) | argued and refereed for joins (price review); argued, not refereed for path 1 | Strict model, first moment. Reopen: a counting object smaller than the tree, or information about piece zones | `memos/solve-path-3-exponent.md`, `solve-path-1-counting-algebra.md`, `price-and-ledger.REVIEW.md` |
| A 470 board as a damaged solution; repair toward 480 (night path 4) | priced out (argued) | of about 10^33 boards at 470, only about 10^9 per solution lie within a few cells of one; the rest sit 246 to 254 cells away | argued, not refereed | An expectation over the model, not a proof; bears on AGENDA A6, A9, A14 (none run). Reopen: a referee for path 4, or an exact neighbourhood that moves a record | `memos/solve-path-4-last-ten-edges.md` |
| First-moment counts and expected boards per score (night) | kept | about 14,702 solutions with the starter clue, about 4.1e-8 with five (ensemble estimates; never "one solution"); 10^34.4 boards at 470, 10^31.65 at 471 | argued and refereed | Struck: "471 costs 80 to 235 times 470" and "a score bound has little to remove" | `memos/partial-match-regime.REVIEW.md`, `growth-law.REVIEW.md` |
| Growth law of search on planted small boards (night) | kept with corrections | a parameter-free model predicts plain row-major DFS within a factor 2.1 above critical; corrected critical colour counts 4.65, 5.99, 7.32, 8.63, 9.94 for n = 6..10 | argued and refereed | Struck: "no ladder rung is critical", "relaxing distinctness cannot refute before the peak"; "propagation is only a surface factor" not established. The model fails for propagating solvers | `memos/growth-law.md`, `growth-law.REVIEW.md` |
| Typical-set colour-ledger prune with an exact miss rate (night path 4) | open | at most about 1.7 orders by the author's estimate | argued, not refereed | Never built | `memos/solve-path-4-last-ten-edges.md` |
| Weighted CSP, MaxSAT cores, assignment-bound all-different on the maximisation form | open | none run here | reported (literature memo; toulbar2 AllDifferent with assignment bounds, AAAI 2026, refereed as holding but not shown to reach these sizes) | Exact QAP methods stop far below 256 assignment variables. Reopen: a first run on 8x8 break-tolerant instances | `memos/literature-2026.md`, `literature-and-path-6.REVIEW.md` |

## Routes of the arc of 2026-11-13 to 2026-11-14

One line per family; evidence in `../experiments/approach-pricing/` (index: [routes.md](../experiments/approach-pricing/routes.md); notes and their re-checks under `notes/`), `../experiments/relaxation-small-boards/` and `../experiments/learning-vs-propagation/`. Gains are bits of nodes on the row-scan tree that survived an independent re-check, unless stated. No family is priced at 10 bits or more; the arc stopped on 2026-11-14 (ledger entries of that date; reopening conditions A40 to A55 in AGENDA.md). Generated boards are toys, never Eternity II progress.

| Family | Status | Where it stopped | Grade | Where |
|---|---|---|---|---|
| Late-biting checks (ceiling of a perfect free cut) | priced out | a perfect cut from free depth 144 saves at most 2.4 bits (57.1 at depth 52, 17.1 at 96, 6.8 at 128); arc consistency refutes 57 to 90 percent of wrong prefixes only at depths 96 to 128; largest gain priced 5.5 bits at about 13 bits per node | measured and derived; ceiling table re-derived by a second analysis | [approach-pricing README](../experiments/approach-pricing/README.md); `scripts/route-audit/strata_gain.py` |
| Frame cores: cheap refutation of wrong frames of the real pieces | killed (far stratum undecided) | lower medians 0.89, 0.12, 1.00 against a line of 100; the stage 2 pass is a corner artefact; nested arc cost 2.2 to 2.5 bits above break-even | measured and judged | [frame-cores](../experiments/frame-cores/) |
| Static scan orders and pinned tiles | killed for orders; pins open as a price | none of 18 orders shown to beat row scan; best extra pin 10.05 bits, median 5.79; a 10^15 run needs probably 16 to 24 spread pins | measured and judged | [orders-and-pins](../experiments/orders-and-pins/) |
| Known-tile worth by position (pin map) | kept as a calibration | worth set by place in scan order; a touching pair loses about half of the second tile's worth; adaptive placement rules differ by 6.8 bits at k = 3 | measured and judged (toys) | [pin-map](../experiments/pin-map/) |
| Prefix statistics and arc consistency on sampled wrong prefixes | kept as a measurement | a census statistic tilts about 2 sd, worth 1.02 bits in one scan direction, -1.88 with four; no transfer claim | measured and judged; the arc-consistency follow-up not judged | [prefix-statistics](../experiments/prefix-statistics/) |
| Port fidelity of the record-style engine | kept | seven differences fixed, 23 of 23 node sequences identical; with five clues the faithful defaults reach less depth than the old ones | measured and judged twice | [single-core-benchmark](../experiments/single-core-benchmark/) (port-fidelity audit) |
| Search that learns (gap 1; clause learning against propagation) | closed for the narrow family; general verdict unproven | about 2 bits (ceiling 6.6); 2^5 to 2^6.5 fewer conflicts than nodes at 5x5 and 6x6, flat; learning is +1.31 and +2.03 bits at 5x5 and 6x6; stop line undecidable | argued and re-checked; toy measurement independently re-checked | `notes/gap-1-search-that-learns.md`; [learning-vs-propagation](../experiments/learning-vs-propagation/) |
| Lossy representations (gap 2) | empty, narrower scope | two-list join 3.0 bits; a sound colour-blind sketch at most 2.0 (needs a second reader) | argued and re-checked | `notes/gap-2-lossy-representations.md` |
| Shape of the tree: restarts, portfolios (gap 3) | empty at 10 bits, narrower scope | 1 to 3 bits with oracle cutoffs, a loss of about 1 bit blind | argued and re-checked, toy and painted boards | `notes/gap-3-shape-of-the-tree.md` |
| Adaptive orders (gap 4) | empty, narrower scope | row-shaped rules within 3 bits; arc consistency 5.5; free smallest-domain 13 to 49 bits worse; finishing rows one or two at a time at most 5.3 or 6.7 | argued and re-checked | `notes/gap-4-adaptive-orders.md` |
| Structure of the tile set (gap 5) | empty, narrower scope | no colour automorphism; invariant group Z^2 + Z/2; 0 bits before depth 192 | argued and re-checked | `notes/gap-5-tile-set-structure.md` |
| Algebra and counting, linearisation (gap 6a) | none found, not a barrier | fifteen reformulations, none below 2^134.4; degree 2 fixes 0 of 10 boards at 4x4 and 0 of 5 at 5x5; degree 3 at 5x5 did not fit in memory | argued and re-checked; linearisation measured, not independently checked | `notes/gap-6a-reformulation-algebra.md`; `notes/gap-6a-linearisation-degree.md` |
| Search-space reformulation (gap 6b) | empty, narrower scope | nothing within 90 bits of row scan; row scan within 0.24 bit of the best static order (no certificate) | argued and re-checked | `notes/gap-6b-reformulation-search-space.md` |
| Far ideas (71 listed) and geometry isolation | none priced at 10 bits with a mechanism | four statements ruled wrong; the 0 of 1,000 annealing result at 6x6 was a budget artefact | argued and re-checked | `notes/far-ideas-and-isolation-recheck.md`; `notes/geometry-isolation.md` |
| Relaxations: convex lift, low-rank form, nonnegative factor | killed on toy boards; lift exactness at 6x6 open | lift not exact on one of five 5x5 boards; low-rank form loses to annealing (8 to 35 times at 5x5, 3.6 to 63 at 6x6), exponent 4.5 in P on two sizes; nonnegative factor 0 of 50 | measured and re-checked, one solver family for the lift | [relaxation-small-boards](../experiments/relaxation-small-boards/) |
| Eight field surveys (cryptanalysis, SAT and proof complexity, statistical physics, tilings, learning-guided search, exact algorithms, Monte Carlo, quantum) | none finds a solver | each names each-tile-once, or the absence of any signal, as the reason; 121 citations checked, 103 confirmed, none invented | reported; citations and derivations re-checked | `../experiments/approach-pricing/surveys/` |
| Route audit and anomaly hunt | negative | 78 lines examined (47 closures, 10 weak and bearing on a solve); 51 anomaly candidates, none survives a painted-board null | not independently checked | `notes/route-audit-weak-kills.md`; `notes/anomaly-hunt.md` |

## Explicit open problems

470 to 480; strict five-clue 466 to 467 (a public five-clue 466 verified on
2026-11-13 with `night/2026-11-12/checks/verify-466/`; the site still said
465 then, and a BOINC project's reported 466 boards cannot be downloaded);
Owen's hint-free 10x10 set_2; crossing a rigidity basin; the meet-in-the-middle
memory wall; a diversity mechanism beyond parallel tempering; whether local
rigidity follows from area-law distinctness; whether the hardness peak
produces the no-forced-moves profile; whether the sigma-cycle geometry caps
constructive search; whether the far stratum of wrong frames of the real pieces can be
refuted below break-even (AGENDA A44; cheap refutation of near frames is
killed, A35); exactness of the convex lift at 6x6 (A41).
