# Evidence audit of the dead-ends page and the explored map

Night 2026-11-12. Three read-only scouts graded every "this does not work"
claim. Grades: `measured` (a committed result or reproducible command in this
repo backs it), `reported` (someone else's number, not rerun here), `argued`
(reasoning, or a one-off attempt with nothing committed). A scout's "none
found" means a search of the repo returned nothing; it is a lead to check, not
a proof of absence. Nothing on the site or in EXPLORED.md was edited.

## Part 1. `web/content/research/build/dead-ends.mdx`, lines 1 to 456

| line | entry | claim | label on page | grade | backing | scale tested | flag |
|---|---|---|---|---|---|---|---|
| 47 | Symmetry breaking | no rotational, duplicate or colour-relabeling symmetry to break | Proven | measured | `research/topics/adversarial-piece-set/results/invariants.json`; the colour-relabeling enumeration has no committed file | official 16x16 piece set, exact | ok |
| 68 | Check invariants on placed cells | pruners over the placed region never fire | Measured | argued | none found; only a one-line summary in `web/src/data/experiments.json:1810` | not stated (3 variants) | label overstates |
| 88 | Throw more compute at it | bigger machines hit the same wall | Measured | reported | groups.io msg 3511 (2007-08 grid); Blackwood via /research/records | 2007-08 grid; GPU claim undated | stale, and label overstates |
| 109 | Survey propagation | messages flatten; Lagrangian rescue does not converge | Measured (one 4x4, 3 settings) | argued | none found; `experiments.json:1853` is a summary; msg 6208 is a forum remark | one generated 4x4; forum 2008 | label overstates |
| 143 | Belief-propagation move ordering | hints near-uniform; random order does as well | Measured | argued | none found; the linked no-forced-moves result measures partner counts, not BP | not stated | label overstates |
| 153 | Sort candidates by future flexibility | no gain, 5% slower | Measured | argued | none found | 16x16, 1 thread, 10 s x 3 runs; hardware not stated | label overstates |
| 175 | Tensor-network counting | cannot enforce piece uniqueness | Proven | argued | structural argument; the Lagrangian 4x4 / 6x6 / out-of-memory amendment is uncommitted | 4x4 (1 s), 6x6 (5 min), 16x16 died at about 3 GB | open inside: "the patched variant is unvalidated at full scale rather than proven impossible" |
| 199 | Solve colours first, place pieces later | colour arrangements too many to validate | Measured | reported | groups.io msg 11182 | 2023 to 2025 forum | label overstates |
| 221 | Relaxation bounds | LP ceiling about 478; full-scale MIP finds nothing | Measured | argued | `build/exact/lp-relaxations.mdx:316` says the conditional-LP numbers are notebook measurements, not yet committed | 16x16 with 60 border cells fixed, HiGHS 4 threads, 30 min | label overstates |
| 253 | Pattern databases | root bound is trivially 480 | Measured (root only) | argued | none found; trivial by construction | root of the empty 16x16 | open inside: "Patch databases indexed by frontier colors and remaining pieces are untested here." |
| 279 | Lattice reduction | easy half polynomial, hard half bilinear | Proven (structural) plus measured probe | argued | none found for the recovery counts or timings | 2x2, 3x3, 4x4; one pure-Python LLL | label overstates |
| 311 | Learn a heuristic on small boards | skill does not transfer to 16x16 | Measured, one architecture | argued | none found; the linked page has `repro: kind: prose` | small boards to 16x16; one architecture | label overstates |
| 322 | Gradient descent on a soft board | soft optimum rounds to 336/480 | Measured, four seeds | argued | none found | 16x16, 4 seeds | label overstates |
| 351 | Enumerate local clusters first | cluster counts explode before pruning helps | Measured | reported | groups.io msg 11428, 11429; the committed subgrid table covers small blocks only | 3x3 / 4x4 regions; forum late 2024 | label overstates |
| 367 | Build one perfect row at a time | best rows starve later rows at row 10 | Measured | argued | none found | 16x16, single deterministic run, beam 32; 256 and 1,024 "were not tried" | label overstates |
| 388 | Build from both ends, meet in the middle | zero valid joins at beam 32 | Measured | argued | none found; `experiments.json:194` has status "idea" only | 16x16, two variants, beam 32 | label overstates |
| 410 | Cube and conquer | cubing does not scale past about 8x8 | Reported | reported | "William Millilaw, May 2026", no link or message number; LEDGER.md (around line 2091) separately calls it "untried at scale" | 16x16, one cuber plus kissat, one encoding and budget | open inside: "other cubers, encodings and budgets are untested" |
| 428 | Generate boards with a transformer | model memorises elite boards | Reported | reported | "William Millilaw, May 2026", no link | 51M-parameter model | ok |
| 448 | Ant-colony construction | ceiling 296/480, learning hurts | Measured, three failures deep | argued | none found; LEDGER.md (around lines 2308, 2317) says inconsistent ant-colony numbers were dropped | 16x16, one seed for the best config | label overstates |

**Counts, part 1 (19 entries).** Grades: 1 measured, 5 reported, 13 argued.
Flags: 13 label overstates, 3 open inside, 1 stale, 2 ok. The page's "Measured"
tag is not backed by committed data for most entries that carry it; the
`experiments.json` summaries are not data.

**Most likely to hide a live direction (scout's view):**

1. Pattern databases (253): only the root was computed; interior patch databases are named untested.
2. Tensor-network counting (175): the Lagrangian patch solves 4x4 and 6x6; one 16x16 attempt, which ran out of memory.
3. Build from both ends (388): exact joins only; softened or diagonal joins never run.
4. Throw more compute (88): the evidence is a 2007-08 grid, and the GPU framing was already wrong once.
5. Cube and conquer (410): unsourced, and the ledger separately calls it untried at scale.

## Part 2. `web/content/research/build/dead-ends.mdx`, lines 457 to 913

Coverage caveat from this scout: whole-repo searches timed out, so it searched
LEDGER.md, `research/topics/*/article.md`, `research/experiments/*/README.md`,
`research/program/*.md`, the community folder, and result files by distinctive
numbers. "None found" means not found there. It did not check that groups.io
messages 5883 and 6060 exist in the local archive.

| line | entry | claim | label on page | grade | backing | scale tested | flag |
|---|---|---|---|---|---|---|---|
| 473 | MCTS with cheap prior (TSP transplant) | k-opt relocation inert; bandit trails beam 2048 at equal wall clock | Measured | argued | none found; only the EXPLORED.md row | official puzzle and generated ladder, hardware not stated | label overstates; open inside: "only the faithful port was tested" |
| 504 | Recombine two good boards | partition crossover collapses to "pick the better parent" on over 99% of pairs | Reported | reported | "William Millilaw implemented it", no citation; own 442-to-271 graft has no committed data | pairs of 455+ boards; one partial and one donor | ok |
| 537 | Widen the mismatch vocabulary | double-break vocabulary ties the classic engine at equal wall clock | Measured | argued | none found; LEDGER.md line 1804 mentions the A/B, no file holds it | 600 s x 4 threads per round, official puzzle | label overstates; open inside: "unsupported conjecture, not refuted" |
| 568 | Decode mismatches like a codeword | syndrome decoding buys nothing | Measured | argued | none found; the permutation-code-wall topic is a different analysis | one 459/480 board, 200 s solves, 8 seeds | label overstates |
| 594 | Mismatches as physical defects | defect model fails | Measured | argued | none found | the same 459 board; sweep "stopped after a single seed, at 441" | label overstates |
| 622 | Force a diverse border, then fill | border-first diversity gives at most 436/480 | Reported plus "We measured" | reported | Millilaw, no citation; own numbers not found; related draft topic `frame-is-not-the-basin` exists (28 frames, best 436) | 100 sampled borders, 60 s fill plus 60 s polish, single seed | label overstates |
| 664 | Build a fake 480 and repair it | duplicate-piece repair never converges, tops out near 463 | Reported | reported | "William Millilaw ran it", not found in the repo | not stated | ok |
| 681 | Seed repair with exact solver's deep prefix | pinned 297/480 prefix repairs to 424 | Measured | argued | none found | one seed, one schedule, one preset | label overstates; open inside: "not as a refutation of all exact-prefix seeding" |
| 706 | Start from centre, grow outward | centre-out loses on 5x5 to 8x8 | Measured | argued | none found for those sizes; `dfs-study/results/report.md` covers 16x16 with a different metric | generated 5x5 to 8x8 | label overstates |
| 727 | Encode the human strategies | human fill orders reach about 392 | Measured | argued | none found; dfs-study gives 376.6 for row-major, which does not match | 6 seeds per order | label overstates |
| 752 | Precompute 2x2 macro-tiles | macro-tiles shrink no domain | Measured | reported | Markus Zajc 2008, groups.io message 5883; no rerun found | 2008 forum | label overstates |
| 771 | Drive backtracker in concentric rings | ring order stalls at depth 80, 382/480 | Measured | argued | none found | official puzzle, 1 h and 2 h, one run per budget | label overstates; open inside: "not that layered orders are refuted in general" |
| 790 | Share a transposition table across workers | workers' deepest boards agree on 0.8% | Measured | argued | none found | 8 workers x 15 s, official puzzle, one run | label overstates |
| 816 | Border-supply ("lollypop") pruning | supply check cuts 0.0014% of tests, costs 8% runtime | Measured | reported | Markus Zajc, groups.io message 6060; own rerun not found | Zajc on 8x8, 2008; own on 16x16, single thread, 10 s | label overstates; open inside: "On a mismatch-tolerant search the check is untested here" |
| 858 | Eulerian border-closure check | closure check never prunes beyond propagation | Measured | argued | none found for the pruning test; `ring-purity` backs the structure, not this test | official puzzle, 1M partial borders | label overstates |
| 877 | Forbidden-pattern pruning during search | can never fire in edge-strict search | Proven | argued | by-construction argument; `forbidden-patterns/results/feasibility.json` backs the theorem; the empirical count has no committed file | 24 boards x 225 patches | ok |
| 898 | What every dead end has in common | faster hardware and local fixes do not touch the gap | none | argued | synthesis prose only | not stated | stale: EXPLORED.md and AGENDA A8 mark GPU as open |

**Counts, part 2 (17 entries).** Grades: 0 measured, 5 reported, 12 argued.
Flags: 13 label overstates, 1 stale, 3 ok; "open inside" also applies to 5
rows.

**Most likely to hide a live direction (scout's view):**

1. Widen the mismatch vocabulary (537): no data committed, and the page itself calls the hypothesis "not refuted".
2. Border-supply pruning (816): explicitly untested on mismatch-tolerant search.
3. Exact-prefix seeding (681): one seed, one configuration; the page says it is not a refutation.
4. Concentric rings (771): one schedule with one ordering, no data committed.
5. The closing synthesis (898): presents faster hardware as not helping, while the map marks GPU as open.

## Dead-ends page, both parts together

36 entries: **1 measured, 10 reported, 25 argued.** 26 carry a label stronger
than their evidence. 8 name, in their own text, a variant that was never run.

### Calibration of this audit (lead's spot check, 20:15)

The counts above are a **lower bound on what is measured**, not a verdict on
the page. Six "none found" verdicts were spot-checked with `git grep` over
`research/topics` and `research/experiments` on `origin/main`:

- Entry 537 (mismatch vocabulary): `research/experiments/dfs-study` has a
  double-break strategy (`engine/crates/dfs-engine/src/strategy/breaks.rs`) and
  `results/matrix.json`. Related committed material exists; the scout did not
  connect it. Whether it backs the page's A/B numbers is unchecked.
- Entry 771 (concentric rings): `research/topics/cas-annular/` exists. Same
  remark.
- Entries 322 (Sinkhorn), 448 (ant colony), 752 (macro-tiles), 790 (shared
  transposition table): nothing relevant found, consistent with the scout.

So about one in three spot-checked "argued" grades may be "measured elsewhere,
not linked from the entry". That is still a defect of the page (its legend
says entries from the project's own runs link a page with the exact
reproduction command), but a different one: missing links, not missing data.

The page's own legend also defines "Measured" more loosely than this audit:
"we ran it (or the community did) and watched it fail". Under that definition
a forum report counts as Measured. The audit's stricter grade asks whether
someone can rerun it from this repo today.

**Recommended follow-up, in order:** (1) for each "argued" row, a second pass
that searches by the entry's distinctive numbers and links the result if it
exists; (2) for what remains, either commit the run or relabel the entry;
(3) split the page's "Measured" into "measured here, reproducible" and
"measured by the community".


## Part 3. `research/program/EXPLORED.md`, every row against the page it cites

Line numbers refer to the cited page under `web/content/research/`. Own runs
that are not committed are graded `argued`. Flag precedence: stop not on page,
then over-compressed, then status too strong.

| row | status in map | stop in map | grade | backing | what the map leaves out | flag |
|---|---|---|---|---|---|---|
| Fill orders, comb | saturated | lever on tree size, not on ceiling | reported | build/backtracking/fill-order.mdx:189-218 and 543-549 (community msgs 2714, 5263; record engines treat order as designed). Only the mean-field law is measured (`just research-fill-order-mean-field`, research/experiments/fill-order-mean-field/README.md exists, page l.55-64). "Not on ceiling" is not stated. | Own comb producer beat row-major at the top, notebook only: "457 ... against 455 for the best of 72 row-major runs" (l.404-405). | over-compressed |
| Break-tolerant DFS | saturated | 467 (Verhaard), 470 (Blackwood) | reported | build/solvers/index.mdx:40 ("community result ... not rerun here"); build/faster/distributed-solving.mdx:302-306 | The 470 came from many cores. This repo's build "stalls once the five clues are pinned" (solvers/index.mdx:106-107). | ok |
| Restarts, heavy tails | saturated | changes which boards, not the ceiling | reported | build/backtracking/restarts.mdx:56-68 (msg 2822, 500 random 7x7 puzzles, one run per policy per puzzle). Frontmatter `repro: prose` (l.48-49). Stop phrase not found. | "Nobody has measured that distribution for the full 16×16" (l.87-88). Blackwood's flat cutoff does not establish a heavy tail (l.172-177). | stop not on page |
| Arc consistency, Régin | mapped | pruning fades about two cells out | measured | build/reduce/arc-consistency.mdx:228-237 and research/experiments/dfs-study/results/report.md:32-34 (FC 322.2, AC-3 320.6, +Régin per-colour 320.6 at 60 s on corner-pinned variants; `just experiments dfs-study`). The "two cells out" phrase is not on this page; it is community msg 6208 in dead-ends.mdx:121-124. | Piece-level Régin "has not built" and its payoff is unmeasured (build/reduce/alldiff-regin.mdx:204-208). AC-2001 was never built (arc-consistency l.245-249). Hard AC is unsound under break budgets (l.253-259). | stop not on page |
| No-good learning, shared tables | mapped | no-goods outrun memory at 16x16 | reported | build/reduce/nogood-learning.mdx:307-311 (msg 6145 about 1%, msg 9618 about 1%); `repro: prose` (l.67-68) | Failure Lookup Table (l.181-186). Deletion-minimised 28-piece core (l.242-246). 68M invalid pairs mined (l.250-255). 4.4x cut on the 6x6 ring (l.316-325). Unverified claim "discards about 75-80% on average" (l.141-142). Pays on small boards, enumeration and endgames (l.313-342). | over-compressed |
| Exact cover / DLX | mapped | small boards and counting only | argued | build/exact/exact-cover-dlx.mdx:140-148 and 185-192; `repro: prose` (l.29-30). Own XCC "spent days broken on anything larger" (l.150-155). "No published measurement compares a DLX engine's node rate" (l.170-172). | Soft edges can be expressed as minimum-cost exact cover (l.176-180). The project's own XCC never ran past small boards. | status too strong |
| Meet in the middle (BANDSAW) | mapped, open | about 20x memory per extra mismatch | argued | meet-in-the-middle.mdx:155-157. lab/.../meet-in-the-middle/bandsaw.mdx:108-112 says the instances, budgets and table sizes behind 20x "are not yet committed"; l.79-81 call the full-size claim an extrapolation. research/experiments/bandsaw/results/bracket.json exists; no growth table found in it. | "A *soft* interface ... was designed but never run", and a column seam was never tried (meet-in-the-middle.mdx:344-348). A one-sided branch-and-bound beat the join (l.268-275). | status too strong |
| Exact tail endgame | mapped | plateaued | measured | build/exact/exact-tail-endgame.mdx:29-31; research/topics/exact-tail-endgame/results/tailforge.json exists; `just research-exact-tail-endgame`. "Plateaued" is only the frontmatter `outcome` (l.16). | Exact beat the producer on all 24 seeds by 1 to 9 edges (l.143-147). No non-exact control: "they have not been run" (l.190-192). Record-level producer not reproduced (l.194-199). | over-compressed |
| Solver engineering, JIT | saturated | speed only; more attempts for record hunting | reported | build/faster/solver-engineering.mdx:522-535 (Blackwood msg 10056, Razvan msg 11657). JIT speed is measured: lab/.../jit-backtracker.mdx:28-30 and 87-90 (dfs-study/engine/crates/dfs-codegen exists). The "speed does not change quality" claim is one uncommitted 5-minute budget point (l.501-511). | "needs a budget sweep (1×, 10×, 100×), which has not been run" (l.510-511). The 4-thread vs 8-thread drop was not isolated (l.512-520). | over-compressed |
| SAT / CSP, cube and conquer | mapped | full board stalls; sub-board proofs | reported | build/exact/sat-csp-encodings.mdx:68-73 ("no complete solver has been reported to decide one", budgets "mostly unstated") and 150-154. Sub-board UNSAT is committed: research/topics/rigidity-sat-halo/results/halo-sat.json (why/rigidity-wall.mdx:513-527). | This page says cube-and-conquer "has not been attempted at scale" (l.73-77); dead-ends.mdx:416-426 reports it tried (8x8 limit). The CDCL-stall mechanism is a hypothesis: "that measurement has not been made" (l.204-206). Planted 26-colour 16x16 solved in about 15 s (l.174-177). | over-compressed |
| LP / ILP relaxations | mapped | 420 to 440, ILP wall at 8x8 | reported | build/exact/lp-relaxations.mdx:74-78 and 204-211 (msgs 6905, 5602). Conditional 478 bounds are notebook only (l.314-319). `repro: prose` (l.58-59). | The 420-440 is one 2009 Newton-type stall, "a property of that solver, not of integrality" (l.225-227). Region-restricted integer models work (l.564-571). A 10x10 6-colour ILP was solved in about 19 min (l.217). | over-compressed |
| MIP halo proofs of rigidity | mapped | record basins locally optimal | measured | research/topics/rigidity-sat-halo/results/halo-sat.json (radius-4 Chebyshev, five public boards; why/rigidity-wall.mdx:513-527). The MIP table itself is not committed (l.173-179). Statement at l.65 and 189-195. | A disconnected better arrangement may exist and no bound rules it out (l.387-390). Some radius-4 SAT solves timed out and are left open (l.524-526). | ok |
| Local search, ALNS | saturated | 461 academic, 460 class here | reported | build/local-search/local-search-alns.mdx has no 461 or 460 (l.183-190 say "record-class"). 461 is Wauters msg 9023 (lp-relaxations.mdx:214). Stall is measured in research/experiments/repair-study/results (ALNS l.196-203). | "budgets far beyond a minute were not tested" (l.202-203). The temperature-insensitivity result is uncommitted (l.164-166). | stop not on page |
| SA, parallel tempering | saturated | below the backtrackers; PT the only new-basin source | argued | build/local-search/parallel-tempering.mdx:184-188 (ladder sizes, temperatures and budgets "not in the committed results"); `repro: prose` (l.35-36). "Only new-basin source" is not on this page (it is open-problems.mdx:160-162). | "With the ladder's top temperature raised to roughly 30, the crossing actually happened" (l.150-153). It produced the then-best cold-start board. | stop not on page |
| Beam search, producers | saturated | breadth stalls in the interior | argued | build/construct/beam-search.mdx:325-326 (16x16 figures "notebook runs, not committed"). Committed only for planted 10x10/12x12: research/topics/beam-width-smc/results/beam-width-smc.json (l.328-335). Page puts the wall in "the last rows", not the interior (l.339-341). | Comb-order producer reached 457 raw (l.357-361). Beam beat DFS and LDS at equal node budget (l.207-212). Nothing above 455 in the width grid (l.158-160). | status too strong |
| Genetic algorithms, crossover | mapped | crossover fights the structure | reported | build/local-search/evolutionary.mdx:181-243 (msgs 565, 1311, 2360, 5589, 5950); `repro: prose` (l.61-62). Partition crossover tested by a community member (dead-ends.mdx:511-519, Reported). | Problem-aware recombination "has not been tested on Eternity II by this project" (l.176-177). The 2008 GA+backtracker hybrid reached 462-463 (l.201-213). | over-compressed |
| Iterated maps / divide-and-concur | thin | tested once | argued | build/exact/iterated-maps.mdx:229-237 (the one test was a hill-climber vs the paper on graph 3-colouring, not E2). L.265-270: nobody ran it on the full puzzle or the 10x10 ladder. | "whether it solves edge matching at scale is unmeasured" (l.280-281). Elser's code solved small model versions (l.242-245). | stop not on page |
| Ant colony, MCTS, gradient descent | mapped | dead ends | argued | build/dead-ends.mdx:456-471 (ant: "best configuration at one seed"), 482-502 (MCTS: "only the faithful port was tested"), 329-349 (gradient: "four seeds of one Sinkhorn schedule"). No code for any of them found under research/. | "this is one schedule of it, not the whole family" (l.335-336). | status too strong |
| Fake 480 and repair; repair seeded by prefix | mapped | dead ends | argued | build/dead-ends.mdx:664-679 (Reported, Millilaw, about 463) and 681-704 ("a single seed, one schedule and one repair preset"). Weakest part decides the grade. | Committed repair study: the DFS-seeded start was the "highest-scoring variant in the whole study, at a mean of 446" (lab/.../repair-study/findings.mdx:55-56; results in research/experiments/repair-study/results). | over-compressed |
| Corpus priors, move ordering, anti-pattern mining | saturated | fast to the plateau, not past it | argued | build/learning/when-learning-collapses.mdx:33-35 ("private corpus ... few seeds each and no baseline at equal compute"). L.94 "Getting past the plateau is open, not ruled out." All three pages `repro: prose`. | "None of the experiments here re-mined its own outputs" (l.80-81). Annealed or adaptive weights untried (l.52-53). | status too strong |
| PRIOR, PALIMPSEST, LODESTONE, KEYRING, REPLAY | mapped | 451 to 463 | measured | research/topics/record-boards/results/verified-scores.json exists; pages cite `just research-record-boards` (kind stochastic, `produces` defaults to artifact: re-verifies stored boards, not the search). REPLAY has `kind: seeded` with no cmd. | 463 has no random-break control (build/learning/anti-pattern-mining.mdx:78-80). KEYRING's new basin is not ablated (when-learning-collapses.mdx:88-90). REPLAY: two-break engine ties one-break at equal wall-clock (replay.mdx:89-91). | over-compressed |
| Transformer boards, small-board heuristics | mapped | dead ends | argued | build/dead-ends.mdx:311-320 ("one model architecture") and 428-446 (Reported, Millilaw). Weakest part decides the grade. | "other architectures are untested" (l.319-320). | status too strong |
| Survey propagation, BP ordering | mapped | signal fades two cells out | reported | build/dead-ends.mdx:121-124 (msg 6208, 2008: influence "seems to fade out"). Own SP test: one 4x4, three settings (l.114-141). BP ordering raced uncommitted (l.146-151). | "loopy message passing does work on some grid problems" (l.117-118). "one toy instance and three settings" (l.135-136). | over-compressed |
| Tensor-network counting, lattice reduction | mapped | dead ends | argued | build/dead-ends.mdx:175-197 (plain contraction is an argument; patched run is a single attempt) and 279-309 (structural arguments plus one pure-Python timing). Nothing committed. | The Lagrangian patch "fully solve[s] generated 4×4 (one second) and 6×6 (five minutes)" (l.189-190). Full scale tried once (3 GB, died). | over-compressed |
| Pattern DBs, macro-tiles, cluster enumeration | mapped | dead ends | argued | build/dead-ends.mdx:253-277 (root only), 351-365, 752-769 (macro-tiles: Zajc msg 5883, Reported) | Interior-node pattern databases: "untested here" (l.272-273). Macro-tiles are a real constant-factor win (l.764-766). | over-compressed |
| Forbidden-pattern, border-supply, Eulerian pruning | mapped | no measurable gain in search | argued | build/dead-ends.mdx:821-826 (Zajc msg 6060, Reported), 833-844 (own: one 10 s run), 865-875, 883-887 (none committed). research/topics/forbidden-patterns/results/feasibility.json supports the 99.72% density, not the search test. | Null result is for strict, matching-only search: "On a mismatch-tolerant search the check is untested here" (l.853-854). Record engines are mismatch-tolerant. | over-compressed |
| Perfect rows (construction) | mapped | dead ends or plateaued | argued | build/dead-ends.mdx:367-386 ("Single-run proof of concept"; nothing committed) | "Much larger beams, 256 or 1,024, were not tried" (l.380-381). | over-compressed |
| Centre-out | mapped | dead ends or plateaued | argued | build/dead-ends.mdx:706-725 ("generated small puzzles with one solver's scan orders", l.718-719; nothing committed) | Tested only on 5x5 to 8x8. | status too strong |
| Concentric rings | mapped | dead ends or plateaued | argued | build/dead-ends.mdx:771-788 ("One schedule paired with one ordering, one run per budget", l.783-785; nothing committed) | "not that layered orders are refuted in general" (l.784-785). | status too strong |
| Clue-to-clue corridors | mapped | dead ends or plateaued | argued | why/clue-corridors.mdx:25-35: counting is committed (research/topics/clue-corridors/results/corridor_counts.json exists); the A/B "numbers are the study's original measurements, not committed". Verdict rests on the A/B. | One greedy filler, 20 s runs, 12 seeds (l.6-8). The counting shows only that a corridor is no more constraining than any chain. | status too strong |
| Colours first, pieces later | mapped | dead ends or plateaued | reported | build/dead-ends.mdx:206-219 (counts from msg 11182: 6.6e11 and 2.97e13 arrangements) | Nothing material. | ok |
| Forced diverse borders | mapped | dead ends or plateaued | reported | build/dead-ends.mdx:628-636 (Millilaw, Reported). Own 120 s catalogue is "explicitly not exhaustive" and the 5,000-border test was 100 sampled, "single seed" (l.638-662); uncommitted. | "a cheap-filter screen, not an exhaustive refutation" (l.657-659). | over-compressed |
| Fluid frame | mapped | dead ends or plateaued | measured | lab/.../pipelines/fluid-frame.mdx:24-27; `just research-frame-manifold`; research/topics/frame-manifold/results/manifold.json exists | Not a dead end: 36% of perfect frames cannot start the interior, and free rim exchanges revived 60 of 60 dead ones (l.83-89). It "re-measures the mechanism only" (l.26-27). | over-compressed |
| Frame-is-not-the-basin | mapped | dead ends or plateaued | measured | why/frame-is-not-the-basin.mdx:26-37; `just research-frame-is-not-the-basin`; research/topics/frame-is-not-the-basin/results/frame_first.json exists | One producer with a ceiling near 440: "whether a near-record producer would see a border effect is not tested here" (l.55-57). | over-compressed |
| Lab pipelines (CAS, CLOISTER, GAUNTLET, LADDER, MIDDEN, MOSAIC, STAGED) | mapped | dead ends or plateaued | measured | CAS: research/topics/cas-annular/results/cas_plateau.json, `just research-cas-annular` (cas.mdx:31-33). GAUNTLET and STAGED: record-boards artifacts. CLOISTER, LADDER, MIDDEN, MOSAIC: repro blocks have no cmd or topic (not found). | GAUNTLET (458) and LADDER (451) are tagged `outcome: new-basin`, not plateaued (gauntlet.mdx:23, ladder.mdx:21). | over-compressed |
| GPU | open | measured ports, no tuned full-board solver | reported | build/hardware/gpu-solving.mdx:178-231 and 295-298 (msgs 9811, 11598, 11835); `repro: prose` (l.63-64). All hobby ports, none rerun here. | Speeds are "a floor, not a ceiling" (l.175-176); about 100x over one core in 2026 (l.222-224). | ok |
| FPGA | thin | designed, never driven | reported | build/hardware/fpga-solving.mdx:62-68 and sources l.33-36 (msgs 9236, 9237, 10649) | A prototype ran on silicon: "ran a prototype on real silicon" (l.7). The hardware walked the full benchmark tree (source label, l.35-36). No full-puzzle run. | over-compressed |
| Distributed farms | saturated | (none given) | reported | build/faster/distributed-solving.mdx:255-273 and 308-314; `repro: prose` (l.40) | Fleets solved finite targets (10x9 in 93 days, 10x10 in about 180 core-years, l.288-291). "Whether one core would have reached 469 ... was never tested" (l.305-306). A live five-clue fleet is under test (l.316-320). | over-compressed |
| Quantum | thin | zero runs | argued | build/hardware/quantum.mdx:64-73 (no member ran any hardware or simulator); the cost figures are arithmetic; `repro: prose` (l.60-61) | "one unfinished embedding attempt, zero runs" (l.8). | ok |
| Theory on the site (14 items, no stop asserted) | n/a | n/a | measured | Committed results exist for: entropy-area-law, phase-transition (color-split.json), no-height-function, permutation-code-wall, flux-invariants, ring-purity, piece-theft, rare-color-geography, parity-defect-floor, sigma-cycles, the-470-wall (landscape.json), no-forced-moves. | Pages tagged `rigor: conjectured`: the-470-wall.mdx:12, complex-theory. Phase-transition's full-board solution counts rest on external sources (why/phase-transition.mdx:40). | ok |

**Counts, part 3 (40 rows).** Grades: 8 measured, 15 reported, 17 argued.
Flags: 20 over-compressed, 9 status too strong, 5 stop not on page, 6 ok.

**Rows most misleading for someone filtering new ideas (scout's view):**

1. Forbidden-pattern, border-supply and Eulerian pruning: the null result holds only for strict, matching-only search, while the record engines tolerate mismatches.
2. Corpus priors, "saturated": the page calls going past the plateau "open, not ruled out", and nothing re-mined its own outputs.
3. Parallel tempering: "the only new-basin source" is not on the cited page, which cites only uncommitted notebook runs.
4. Iterated maps, "tested once": the one test was a graph-colouring benchmark; nobody ran it on Eternity II.
5. LP / ILP, "420 to 440": one 2009 solver's stall, which the page says is not a verdict on integrality.

## What this means for the program

- EXPLORED.md is a useful index and a poor filter. Half its rows hide a
  partial success, an untested variant or a scale limit that the cited page
  states. Scouts and inventors should open the page; the scout definition now
  says so.
- A status of saturated or mapped should require a `measured` stop. By this
  audit 9 rows have a status stronger than their evidence.
- Several stops apply to strict, perfect-match search only, while the open
  problem (470 against 480) lives in mismatch-tolerant search. That
  distinction is absent from the map and should be a column.
- The same calibration caveat as for the dead-ends page applies: a scout's
  "not found" is a lead, and the lead's spot check found related committed
  material for 2 of 6 such verdicts.
