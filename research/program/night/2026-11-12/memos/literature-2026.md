# Literature review from outside the program, 2026-11-12

Scope: what the published literature and the practitioner record contain that
`EXPLORED.md`, `AGENDA.md` (A1 to A22), the site bibliography (`PapersView.tsx`)
and the GPU page do not know, or know in a stale form. Web reading only, no
runs. Search queries used generic technical terms only.

Verification levels used below and in the table of section 7:

- **F**: full text or the full project page was read tonight.
- **A**: the abstract page (arXiv, DROPS, IJCAI, publisher) was opened tonight.
  Numbers quoted come from the abstract unless marked otherwise.
- **M**: bibliographic record confirmed tonight through the Crossref API (title,
  authors, year, venue), because the publisher page refused automated access.
  The content was not re-read tonight; any statement about content is flagged.

Relevance grades: **direct** (on edge matching or Eternity II), **close**
(a structurally close problem: jigsaw, tiling, assignment with adjacency
costs, permutation problems), **method** (method-level only).

## 1. Answer in fifteen lines

1. **The practitioner record moved this month and the site has not.** A public
   BOINC project (pilot since September 2026) shows "466/480 Project record,
   5 official clues", four 466 boards dated 7 to 12 November 2026, and 259
   boards at 465 or better in one week, with CPU and OpenCL GPU applications.
   The program's open problem "strict five-clue 465 to 466" may already be
   closed. It is a dashboard report, not a verified board. No claim above 470
   and no solution claim exists anywhere I searched.
2. **The exact-optimisation gap in the map is real and now has a named tool.**
   Nobody has published a weighted CSP, MaxSAT branch and bound or decision
   diagram run on edge matching. toulbar2 gained an AllDifferent propagator
   with linear-assignment bounds (AAAI 2026) tested on QAPLIB, which is the
   shape of break-tolerant edge matching (cells, piece-rotation values,
   pairwise costs, one permutation constraint).
3. **A rigorous null model exists and the site does not cite it.** Random
   jigsaw papers (2016 to 2019) put the uniqueness threshold at a number of
   colours linear in the side, and a planted-jigsaw analysis proves that fill
   order changes the search exponent: rectangle from a corner has expected
   width 1 + Θ(ε), frame first, centre block and corner triangle blow up.
4. **Two "closed" method families have evidence the map lacks.** GPU branch
   and bound is a published line at supercomputer scale (339e12 nodes in 13
   hours on 256 GPUs, 2022). Evolved solver code has competition-level
   evidence in SAT (2025), with a documented "cheating phenomenon" that makes
   the evaluator the hard part. Neural solvers show no transfer evidence.
5. **Tooling the program lacks:** online tree-size estimators that work during
   a run (not only Knuth probes), proof logging for "proven optimum" claims,
   and the SAT competition's benchmarking discipline. Also two citation errors
   in the site bibliography (section 2.6).

## 2. Eternity II and edge matching, 2018 to 2026

### 2.1 What exists

**Peer-reviewed work on Eternity II since Harris et al. (2018, already cited):
I found none.** The academic line stops in 2018. What exists is one physics
paper that uses the puzzle as a hardness generator, one 2026 preprint, a small
benchmark series in the XCSP3 competition, and a busy practitioner record.

- **Hamze, Jacob, Ochoa, Perera, Wang, Katzgraber, "From Near to Eternity:
  Spin-glass planting, tiling puzzles, and constraint satisfaction problems",
  Phys. Rev. E 97, 043303 (2018), arXiv:1711.04083. F, close.** Builds planted
  Ising instances on the cubic lattice from unit-cube subproblems and shows the
  construction is a three-dimensional tile-matching puzzle that generalises
  two-dimensional edge matching (the paper names Eternity II as the motivating
  example). 6,000 instances per class, 200 per subclass; hardness measured
  three ways (simulated annealing time to solution, parallel tempering with
  isoenergetic cluster moves, and the entropic family size of population
  annealing), which correlate strongly; the hardest subclasses are 2 to 3
  orders of magnitude harder than bimodal and Gaussian spin glasses.
  *Change for the program:* it is the only published statistical-physics
  treatment of planted tile matching, it gives a tunable planted ensemble, and
  it offers population-annealing family entropy as a hardness instrument that
  agrees with two other solvers. Not in the bibliography (zero matches for the
  title or first author in the repository).

- **Seifer, "A Pipeline for Solving Edge-Matching Puzzles and Their
  Implications for Protein Folding", bioRxiv, 24 May 2026,
  doi:10.64898/2026.05.23.727379; code on Zenodo, doi:10.5281/zenodo.18109266
  (31 December 2025). A, direct. Not peer reviewed.** A QUBO model on a D-Wave
  hybrid solver reaches a complete solution only up to 64 tiles; simulated
  quantum annealing fails on that benchmark while a classical heuristic of the
  author ("nucleation with deduction") succeeds; the full pipeline (about
  100,000 logical variables, needs partial initialisation) is demonstrated on
  a puzzle "comparable in complexity" with a known solution, not on the
  official board. No score on the official 16x16 is reported in the abstract
  or the repository README.
  *Change:* the map's quantum row says "zero runs". There is now one preprint
  with runs, and its reading is negative for annealers at this size.

- **XCSP3 competition "Eternity" series. F for the 2022 proceedings
  (arXiv:2209.00917, Audemard, Lecoutre, Lonca), F for one 2018 result page,
  direct.** Instances derived from Eternity II by the BeCool team at UCLouvain
  for the 2018 competition, reused in 2022 as a series of 10 CSP decision
  instances (PyCSP3 model, border colour 0). The one result page opened
  (Eternity-10-05, 2018 mini-solver track, 165 variables) shows every solver
  finding a solution, the best in 4.8 s. The 2025 proceedings
  (arXiv:2511.06918) contain no occurrence of "Eternity" in the full text.
  *Change:* small. These are easy decision instances with no optimisation
  variant. They are a ready-made, externally defined instance set if the
  program wants numbers comparable with CP solvers. Not in the repository.

### 2.2 Practitioner record, 2025 to 2026 (all are reports)

- **BOINC project "Eternity II" (boinc.eternityathome.org), page read
  2026-11-12. F, direct, report.** Public pilot since September 2026 per the
  BOINC Synergy wiki (snapshot 30 September 2026: 66 users, 215 active hosts).
  Today's page: "466/480 Project record · 5 official clues", "Record timeline
  464 → 465 → 466", 466 boards credited on 7, 9, 10 and 12 November, "259
  465+ this week", 172 computers, 25,621 tasks in progress, an OpenCL
  application opened to AMD and Intel cards on 10 November 2026. The page does
  not state the validation method beyond "every good board it finds is checked
  and credited", and does not link source code.
  *Change:* the site's open problem "the strict five-clue 465-to-466 target"
  and the five-clue fleet page (sourced to August 2026) are probably stale.
  Verify the boards before changing any number. The distributed-farms row
  ("saturated") describes 2007 to 2021; a live farm with GPU clients exists.

- **Guzman Marte, "Frames, Signatures and Joints", Zenodo
  doi:10.5281/zenodo.23132447 (3 October 2026) and its repository. F for the
  README, direct, report.** Exact C solver with pthreads, a stochastic
  "joints" solver, and a GPU solver on a GTX 1070 through CuPy. Reports 1.3 to
  1.9 billion nodes per second on the GPU, an 11x11 16-colour board going from
  206.8 s to 0.34 s on 12 threads, interior-first ordering giving 15 to 32
  times on boards with few border colours, and logged negative results: SAT 10
  to 100 times slower than its backtracker, GPU pruning 10 times slower than
  CPU, colour parity and Hall matching checks giving nothing. States that the
  real 16x16 "remains far out of reach". No independent verification of the
  outputs is described.
  *Change:* a third-party report that agrees with two of the program's
  dead-end entries (parity and Hall-type pruning, SAT slower than DFS on
  sub-boards) and contradicts none. The interior-first result on low-border
  -colour boards is a specific claim worth a null-model check.

- **gearashi/EternityIISolver (README section dated 9 October 2026). F,
  direct, report.** GPU sampler measured at 12.4 million samples per second
  (CUDA) and 11.5 million (OpenCL) on an RTX 2060; an equal-budget CPU
  comparison "ended in timeouts" for both arms; best board 466/480, stated as
  imported from a public source, not found by the software. The README relays
  a user message that a GPU DFS was about 7,000 times slower than its CPU
  implementation (not reproduced by the author).
  *Change:* none on substance; it is one more data point that a naive GPU DFS
  port can lose badly, which is the distinction the GPU page now makes.

- **E555solver/E555 (2026, MIT licence, 309 commits). F, direct, report.**
  Row-by-row wide beam over a precomputed database of 3.1 billion 5-piece row
  chains (6.4 GB), a colour heuristic with a Mahalanobis term, CP-SAT and a
  backtracker for the tail rows. No score of its own is stated.
  *Change:* overlaps the program's beam producers and exact tail endgame; the
  chain database is a concrete precomputation the map's "pattern databases,
  macro-tiles" row calls a dead end on root-only evidence.

### 2.3 Claims above 470, and solution claims

None found. The BOINC wiki and Wikipedia both repeat that no complete solution
has been published. The only score movement is the 466 with five clues above.

### 2.4 Hardness results

- **Bosboom, Demaine, Demaine, Hesterberg, Manurangsi, Yodpinyanee, "Even
  1 × n Edge-Matching and Jigsaw Puzzles are Really Hard", arXiv:1701.00146
  (2016). A, direct.** NP-hard to approximate the maximum number of placed
  tiles in a 1 × n puzzle within a factor 0.9999999851; presented as the first
  correct inapproximability proof for edge matching. *Change:* this is the
  hardness statement that bears on the partial-match regime (470 of 480), and
  it is not in the bibliography.
- **Antoniadis, Lingas, "Approximability of Edge Matching Puzzles", SOFSEM
  2010, LNCS 5901, pp. 153 to 164, doi:10.1007/978-3-642-11266-9_13. M,
  direct.** Claims APX-hardness. The Bosboom et al. paper states that this
  proof is incomplete (reported through a search summary of their text, not
  re-read tonight). *Change:* do not cite it as settled.
- **Ebbesen, Fischer, Witt, "Edge-matching Problems with Rotations",
  arXiv:1703.09421 (preliminary version FCT 2011). A, direct.** One-row
  puzzles with rotations are NP-hard; the problems are fixed-parameter
  tractable in the number of colours. *Change:* already cited, with the wrong
  authors (section 2.6).

No phase-transition study of edge matching after Ansótegui et al. (2013,
cited) was found. The 2016 to 2019 random jigsaw papers are in section 5.

### 2.5 What is new relative to the bibliography

New: Hamze et al. 2018; Seifer 2026; the XCSP3 Eternity series; Bosboom et al.
2016; the BOINC project's 466; the Guzman, gearashi and E555 repositories. Not
new: Harris et al. 2018, Salassa et al. 2017, Abel et al. 2019.

### 2.6 Two errors in the site bibliography

- `rotations2017` attributes "Edge-Matching Problems with Rotations"
  (arXiv:1703.09421) to "J. Bosboom, E. D. Demaine, et al.". The arXiv record
  lists Martin Ebbesen, Paul Fischer and Carsten Witt.
- `kovalsky2014` gives the title "Solving Jigsaw Puzzles with Linear
  Programming" for Kovalsky, Basri, Glasner. Their paper is "A Global Approach
  for Solving Edge-Matching Puzzles" (Kovalsky, Glasner, Basri, SIAM J. Imaging
  Sciences 8(2), 916 to 938, 2015, arXiv:1409.5957). The title on the site
  appears to belong to a different paper; I did not open the site's PDF link.

## 3. Exact optimisation frameworks missing from the map

The map's "Encodings" section has SAT, CSP, LP and ILP, and MIP halo proofs. It
has no row for weighted CSP, MaxSAT, decision diagrams, lazy clause generation
or pseudo-Boolean solving. The repository does use two of them without mapping
them: MOSAIC solves 4x4 blocks with core-guided RC2, and CP-SAT appears in the
planted-board cliff measurement and as a judge's independent model.

**Reported applications to edge matching, by framework:**

| Framework | Published application to edge matching | Practitioner reports |
|---|---|---|
| Weighted CSP, cost function networks | none found | none found |
| MaxSAT (any family) | none found | none found outside this repository |
| Decision diagram branch and bound | none found | none found |
| Lazy clause generation, CP-SAT | none found | E555 (tail rows), gearashi (exact arm, no result), Guzman ("SAT 10 to 100 times slower") |
| Pseudo-Boolean | none found | none found |
| Plain CP | XCSP3 Eternity series (decision only, small) | several |

So for four of five frameworks the finding is absence, and the absence is the
blind spot: the 470 regime is an optimisation problem, and the only exact
optimisation tools ever pointed at it are ILP (2009 forum runs, Salassa et al.
2017) and the project's own block-level MaxSAT.

### 3.1 Cost function networks (toulbar2)

- **Sewa, Allouche, de Givry, Katsirelos, Montalbano, Schiex, "Assignment
  Problems in Cost Function Networks", AAAI 2026, Proc. AAAI 40(17), 14322 to
  14330, doi:10.1609/aaai.v40i17.38447. M (publisher page refused access; the
  description below comes from a search summary of the authors' preprint and
  poster), close.** Adds an AllDifferent propagator to toulbar2 that bounds
  with the linear assignment problem inside soft local consistency; the QAP is
  modelled as one permutation constraint plus unary and binary cost functions;
  benchmarks include 132 QAPLIB instances, 140 weighted N-queens instances and
  a 2024 XCSP mini-COP set, against CP-SAT and other AllDifferent propagators.
  The authors report it outperforms domain-consistency CP and integer
  programming solvers on these sets. *Change:* this is the closest published
  match to break-tolerant edge matching as an exact optimisation problem. A
  16x16 instance is a 256-variable network (one variable per cell, 1,024
  piece-rotation values each) with one AllDifferent over pieces and 480 binary
  cost functions, which is large for this technology; the sub-board and halo questions (rigidity proofs, exact
  tail endgame, T4's bound) are within its published size range. It would
  replace the 2009 Newton-type LP stall as the reference lower bound.
- **Montalbano, de Givry, Katsirelos, "Virtual Arc Consistency for Linear
  Constraints in Cost Function Networks", arXiv:2509.17706 (2025). A,
  method.** Extends VAC to linear constraints and reports improved lower
  bounds on several benchmarks. *Change:* shows the line is active in 2025.
- **Trösser, de Givry, Katsirelos, "Relaxation-Aware Heuristics for Exact
  Optimization in Graphical Models", CPAIOR 2020, LNCS,
  doi:10.1007/978-3-030-58942-4_31. M, method.** The VAC-integrality
  heuristic. Content not re-read.
- **Hurley et al., "Multi-language evaluation of exact solvers in graphical
  model discrete optimization", Constraints 21, 413 to 434 (2016),
  doi:10.1007/s10601-016-9245-y. M, method.** The cross-paradigm comparison
  (CFN, ILP, MaxSAT, CP on the same instances). Content not re-read; it is
  2016, so stale by the standard of this memo, and the reason to read it is
  its method (one instance set translated to every solver language).

### 3.2 MaxSAT

- **Li, Xu, Coll, Manyà, Habet, He, "Combining Clause Learning and Branch and
  Bound for MaxSAT", CP 2021, LIPIcs 210, 38:1 to 38:18. A, method.**
  MaxCDCL: branch and bound with clause learning, top five of 15 in the 2020
  evaluation and solving instances no other solver could. A 2025 IJCAI paper's
  abstract, seen in search results only, states that MaxCDCL-family solvers
  took the first two places of the exact unweighted track in the 2024
  evaluation (unverified lead). *Change:* MOSAIC used core-guided RC2 only.
  Edge matching with few breaks is an unweighted instance with a small optimum
  (10 falsified soft clauses out of 480), the regime where branch and bound
  with learned bounds and implicit hitting set solvers differ most from RC2.
  One comparison on the existing 4x4 and halo instances would show whether the
  solver family matters.

### 3.3 Decision diagrams

- **Bergman, Cire, van Hoeve, Hooker, "Discrete Optimization with Decision
  Diagrams", INFORMS J. Comput. 28(1), 47 to 66 (2016),
  doi:10.1287/ijoc.2015.0648. M, method.** Already the source of IDEAS T4.
- **Gillard, Schaus, Coppé, "Ddo, a Generic and Efficient Framework for
  MDD-Based Optimization", IJCAI 2020 demos, 5243 to 5245,
  doi:10.24963/ijcai.2020/757. A, method.** Open-source Rust library; the user
  supplies a dynamic-programming model and a merge rule and gets a parallel
  branch-and-bound solver. *Change:* T4 can be prototyped on ddo instead of a
  new engine, since the row-by-row frontier model is already a dynamic
  program.
- **Rudich, Cappart, Rousseau, "Improved Peel-and-Bound", JAIR 77, 1489 to
  1538 (2023), arXiv:2302.05483. A, method.** Reuses a peeled sub-diagram as
  the starting relaxation of the next node; outperforms ddo on TSP with time
  windows and closes 15 open benchmark instances. *Change:* the reuse step is
  what T4's cost estimate ignores (it rebuilds a width-W diagram per node).

No application of decision diagrams to edge matching, jigsaw or Tetravex was
found (query in section 9).

### 3.4 Lazy clause generation and CP-SAT

- **Perron, Didier, Gay, "The CP-SAT-LP Solver", CP 2023 invited talk, LIPIcs
  280, 3:1 to 3:2. A, method.** Lazy clause generation derived from Chuffed,
  plus a simplex alongside the SAT engine and a portfolio of workers (complete
  search, LNS, local search, bound improvement) that share information.
  *Change:* the site's negative SAT and CSP evidence is from 2008 to 2013
  solvers (Heule 2008, Ansótegui et al. 2013). The project's own measurement
  (CP-SAT pushing the planted wall about one size past a restarting DFS) is
  the only modern data point. Practitioner reports go both ways (Guzman: SAT
  much slower than DFS; E555 keeps CP-SAT for tails).

### 3.5 Pseudo-Boolean

- **Elffers, Nordström, "Divide and Conquer: Towards Faster Pseudo-Boolean
  Solving", IJCAI 2018, 1291 to 1299, doi:10.24963/ijcai.2018/180. A,
  method.** RoundingSat: cutting-planes conflict analysis with division.
  *Change:* the map's "cut-multiset feasibility" (A17, promoted) and "border
  balance" are counting arguments, the kind resolution-based CDCL cannot
  learn compactly and cutting planes can. No application to edge matching
  found. Untested, cheap to try on the committed halo UNSAT instances.

## 4. Methods the map closes on thin evidence, current state

### 4.1 GPU and massively parallel exact search

- **Gmys, "Exactly Solving Hard Permutation Flowshop Scheduling Problems on
  Peta-Scale GPU-Accelerated Supercomputers", INFORMS J. Comput. 34(5), 2502
  to 2522 (2022), doi:10.1287/ijoc.2022.1193; preprint arXiv:2012.09511. A
  (preprint abstract), method.** Solves 11 Taillard instances open since 1993
  and improves 8 upper bounds; the largest proof explores 339 × 10^12 nodes in
  13 hours on 256 V100 GPUs with over 4 million search agents, about 64
  CPU-years. *Change:* this is the missing reference on the GPU page. It shows
  the work-stealing, one-subtree-per-thread design holds at 10^14 nodes for a
  permutation problem with a cheap bound, which is A8's design.
- **Helbecque, Krishnasamy, Carneiro, Melab, Bouvry, "Portable PGAS-Based
  GPU-Accelerated Branch-and-Bound Algorithms at Scale", Concurrency and
  Computation 37 (2025), doi:10.1002/cpe.70321. M, method.** Search snippets
  report up to 90% of linear speedup on 24 GPUs for N-queens and near-linear
  scaling to 128 nodes; not read tonight. *Change:* a 2025 portable (NVIDIA
  and AMD) code base for backtracking on GPUs exists.
- **Talbot, Pinel, Bouvry, "A Variant of Concurrent Constraint Programming on
  GPU", AAAI 2022, 3830 to 3839, arXiv:2207.12116. A, method.** Turbo, a
  constraint solver with propagation on the GPU. A search summary quotes about
  an order of magnitude more nodes per second than Gecode on RCPSP without an
  order of magnitude in time to solution; not checked in the full text.
- **Tardivo, Dovier, Formisano, Michel, Pontelli, "Constraint propagation on
  GPU: A case study for the AllDifferent constraint", J. Logic Comput. 33,
  1734 to 1752 (2023), doi:10.1093/logcom/exad033. M, method.** GPU
  AllDifferent propagation; speedups reported as size dependent.
  *Change for both:* propagation on the GPU is where published gains are
  modest. Brute subtree search is where they are large. The page should keep
  the two apart, as the practitioner reports also do (Guzman: GPU pruning 10
  times slower than CPU).

State: open, with outside measurements at scale. Dead-ends entry 88 ("throw
more compute", 2007 to 2008 grid evidence) is stale by this literature.

### 4.2 Learning-guided search

- **Böther, Kißig, Taraz, Cohen, Seidel, Friedrich, "What's Wrong with Deep
  Learning in Tree Search for Combinatorial Optimization", ICLR 2022,
  arXiv:2201.10494. A, method.** Re-implements a well-cited guided tree search
  and finds the network "can be replaced by random values"; the performance
  came from kernelisation and classical components; classical solvers are
  often faster at similar quality.
- **Angelini, Ricci-Tersenghi, "Modern graph neural networks do worse than
  classical greedy algorithms in solving combinatorial optimization problems
  like maximum independent set", Nature Machine Intelligence 5, 29 to 31
  (2022 online), arXiv:2206.13211. M plus search summary, method.** A greedy
  algorithm beats a published physics-inspired network by a factor reported as
  10^4 in time at a million variables. A reply by the criticised authors
  exists (arXiv:2302.03602, not opened).
- **Tönshoff, Kisin, Lindner, Grohe, "One Model, Any CSP", arXiv:2208.10227
  (IJCAI 2023 per search results). A, method.** A graph network trained by
  policy gradient as a global local-search heuristic; the abstract claims it
  competes with conventional heuristics on instances far larger than those
  trained on, for colouring, MaxCut and MAX-k-SAT.
- **Xu, Li, Sanner, Khalil, "Self-Supervised Transformers as Iterative
  Solution Improvers for Constraint Satisfaction", ICML 2025,
  arXiv:2502.15794. A, method.** Sudoku, graph colouring, nurse rostering,
  MaxCut. Sizes are those of standard benchmarks, far below a 256-variable
  permutation problem with 1,024 values per cell.
- **Scavuzzo, Aardal, Lodi, Yorke-Smith, "Machine learning augmented branch
  and bound for mixed integer linear programming", Math. Program. 217, 123 to
  166, arXiv:2402.05501. A, method.** Survey of learned branching, node
  selection and cuts.

*Reading:* nothing found on edge matching, Tetravex or exact cover (section
9). The published record supports the map's verdict on "heuristics learned on
small boards" better than the program's own evidence does (one architecture,
nothing committed), and adds the reason: the two audited successes were
classical components wearing a network. The one positive, size-transfer claim
(ANYCSP on MAX-k-SAT) is for local search on random instances, not for tree
search on a structured instance. The row should be "thin here, negative
outside", not "mapped".

### 4.3 LLM-driven program search

- **Romera-Paredes et al., "Mathematical discoveries from program search with
  large language models", Nature 625, 468 to 475 (2024),
  doi:10.1038/s41586-023-06924-6. M plus abstract through Europe PMC,
  method.** FunSearch: new large cap sets and better online bin-packing
  heuristics, by searching for programs under a systematic evaluator.
- **Novikov et al., "AlphaEvolve: A coding agent for scientific and
  algorithmic discovery", arXiv:2506.13131 (June 2025). A, method.** Already
  cited in IDEAS E1.
- **Georgiev, Gómez-Serrano, Tao, Wagner, "Mathematical exploration and
  discovery at scale", arXiv:2511.02864 (November 2025). F for the first third
  of the text, method.** 67 problems; best known results rediscovered in most
  and improved in several. Reports a "cheating phenomenon": the search "would
  find loopholes or exploit artifacts", for example a verifier that
  approximated positivity by discrete checks; "designing a clever verifier
  that avoids this behavior is key". Separates a search mode (a program that
  searches for one construction within a time budget) from a generalizer mode
  (a program scored across a range of n), and notes that showing only small-n
  solutions helped generalisation; it offers no systematic test of transfer to
  larger n. *Change for E1:* the kill criterion already tests transfer to 9x9.
  Add the verifier lesson explicitly: score on node counts produced by code
  the evolved hook cannot touch, and count work, since a hook can buy nodes
  with hidden propagation.
- **Yu, Liang, Ho, Ren, "Autonomous Code Evolution Meets NP-Completeness",
  arXiv:2509.07367 (September 2025). A, method.** SATLUTION: agents evolve
  whole solver repositories starting from SAT Competition 2024 code; the
  abstract states the evolved solvers outperformed the human-designed winners
  of SAT Competition 2025 and both years' champions on the 2024 benchmarks.
  PAR-2 figures and the verification method were not read. A separate IJCAI
  2026 paper (AESAT, seen in search results only) claims a competition win.
  *Change:* this is the strongest evidence that evolved code beats
  hand-written heuristics in a mature exact solver, and its evaluator is the
  hard-to-game kind (a fixed public benchmark with runtime scoring and solver
  correctness checks). It is CDCL, where heuristics are many and local; an
  edge-matching DFS has two hooks.
- **Zhang, Liu, Lin, Wang, Lu, Zhang, "Understanding the Importance of
  Evolutionary Search in Automated Heuristic Design with Large Language
  Models", PPSN 2024, arXiv:2407.10873. A, method.** Four methods, four
  problems, nine models, five runs each; concludes that earlier comparisons
  used inconsistent settings and weak baselines. *Change:* supports E1's
  control arm (random mutations without selection).

No application to edge matching, exact cover or tiling found (section 9).

### 4.4 Cube and conquer

- **Heule, Scheucher, "Happy Ending: An Empty Hexagon in Every Set of 30
  Points", arXiv:2403.00737 (2024). A, method.** A compact encoding and a
  partitioning that give linear speedups on thousands of cores; a search
  summary gives about 17,300 CPU hours (not checked in the text).
- **Subercaseaux, Heule, "The Packing Chromatic Number of the Infinite Square
  Grid is 15", TACAS 2023, LNCS, doi:10.1007/978-3-031-30823-9_20,
  arXiv:2301.09757. A, close (a grid colouring problem).** New encoding and
  symmetry breaking, about two orders of magnitude over the previous method.
- **Jha, Li, Lu, Zeng, Bright, Ganesh, "AlphaMapleSAT: An MCTS-based
  Cube-and-Conquer SAT Solver for Hard Combinatorial Problems",
  arXiv:2401.13770 (2024, revised January 2026). A, method.** Tree search over
  cubing variables with propagation rate as reward; 1.61 to 7.57 times wall
  clock over March cubing on a 128-core machine.

*Reading:* every success is an unsatisfiability proof on a problem with heavy
symmetry and a hand-built encoding, where cubing equalises subproblem
hardness. The program's question is finding a solution or a better board,
where cube and conquer reduces to a static split of a backtracking tree. The
dead-ends entry (410, one cuber, one encoding, unsourced, "does not scale past
8x8") is thin evidence, and the ledger calls it untried at scale. The place it
could pay is the program's UNSAT work: rigidity halos that timed out at radius
4, and proving that no 471 exists inside a neighbourhood.

### 4.5 Parallel tempering and population annealing variants

- **Wang, Machta, Katzgraber, "Comparing Monte Carlo methods for finding
  ground states of Ising spin glasses: population annealing, simulated
  annealing and parallel tempering", Phys. Rev. E 92, 013303 (2015),
  arXiv:1412.2104. A, method.** Population annealing matches parallel
  tempering for ground states, beats simulated annealing with better scaling,
  and is better suited to massive parallelism. Companion: "Population
  annealing: Theory and application in spin glasses", Phys. Rev. E 92, 063307
  (2015), M.
- **Zhu, Ochoa, Katzgraber, "Efficient Cluster Algorithm for Spin Glasses in
  Any Space Dimension", Phys. Rev. Lett. 115, 077201 (2015),
  arXiv:1501.05630. A, method.** Isoenergetic cluster moves between replicas:
  an order of magnitude or more in thermalisation, growing with size.
- **Syed, Bouchard-Côté, Deligiannidis, Doucet, "Non-Reversible Parallel
  Tempering", JRSS B 84, 321 to 350 (2022), arXiv:1905.02939. A, method.**
  Deterministic even-odd swaps beat reversible swaps in theory and practice,
  with an iterative method for the optimal temperature schedule.

*Reading:* these are 2015 to 2022, not new, but none appears in the repository
(zero matches for "population annealing"). The map calls tempering
"saturated" while the cited page has uncommitted ladders and one crossing that
"actually happened" when the top temperature was raised. Three named variants
are untested here: population annealing (also an estimator, section 5), a
replica-overlap cluster move (for a permutation state the analogue is
exchanging the pieces of a connected region where two replicas differ, which
needs a piece-conserving definition), and schedule optimisation for
non-reversible swaps. Hamze et al. used the first two on tile matching in
three dimensions.

### 4.6 Quantum and quantum-inspired

- Seifer 2026 (section 2.1): hybrid annealer, complete solutions up to 64
  tiles only. Direct.
- **Tasseff et al., "On the Emerging Potential of Quantum Annealing Hardware
  for Combinatorial Optimization", arXiv:2210.04291 (J. Heuristics 30, 2024,
  per search results). A, method.** Run-time benefits on a 5,000-qubit
  annealer for contrived native problem classes; the authors state it is not
  strong evidence of an irrefutable benefit.
- **Campbell, Khurana, Montanaro, "Applying quantum algorithms to constraint
  satisfaction problems", Quantum 3, 167 (2019), arXiv:1810.05582. M plus
  search summary, method.** Resource estimates for Grover and quantum
  backtracking on satisfiability and colouring against classical solvers.
  Content not re-read.
- **Abbas et al., "Challenges and Opportunities in Quantum Optimization",
  Nat. Rev. Phys. (2024), arXiv:2312.02279. A, method.** A survey that frames
  advantage as possible in principle and asks for benchmarking against
  classical methods.

*Reading:* a one-hot encoding of 256 pieces × 256 cells × 4 rotations has
262,144 binary variables before any embedding, against annealers with about
5,000 sparse qubits. The evidence says no. The row can move from "thin, zero
runs" to "one outside run, negative at 64 tiles".

## 5. Theory that predicts hardness

### 5.1 Random jigsaw thresholds (not on the site)

The model: an n × n board, each of the 2n(n−1) internal edges gets one of q
colours uniformly, the pieces are shuffled; is the assembly unique? This is
planted edge matching without border colours.

- **Nenadov, Pfister, Steger, "Unique reconstruction threshold for random
  jigsaw puzzles", arXiv:1605.03043 (2016). A, close.** Not unique with high
  probability if q = o(n); unique if q ≥ n^(1+ε).
- **Bordenave, Feige, Mossel, "Shotgun Assembly of Random Jigsaw Puzzles",
  arXiv:1605.03086 (2016). A, close.** The same upper bound, with an assembly
  algorithm running in time n^Θ(1/ε).
- **Martinsson, "A linear threshold for uniqueness of solutions to random
  jigsaw puzzles", Combin. Probab. Comput. 28, 287 to 302 (2019),
  arXiv:1701.04813. A, close.** At least two non-similar solutions for
  2 ≤ q ≤ (2/√e) n; all solutions similar for q ≥ (2+ε) n; the expected
  number of solutions behaves like (2n/(√e q))^(2n²) (the last formula from a
  search summary of the text), and a sharp threshold at (2/√e) n is
  conjectured.

*Change:* (2/√e) × 16 ≈ 19.4. The official puzzle has 17 interior and 5
border colours, so it sits near the first-moment threshold of the uniform
model, as the site's own ensemble estimate already says. The uniform model
has no border class and no fixed colour multiplicities, so the number does
not transfer; what transfers is a proved first-moment and second-moment
analysis to check the site's phase-transition page against, and the fact that
the gap between "many solutions" and "provably one" is a factor of about 1.65
in q, which bounds how sharply any ensemble estimate can be read.

### 5.2 Search time on planted puzzles

- **Edmonds, Edmonds, Mertz, "Search Time when Solving Random Jigsaw Puzzles
  with a Planted Solution", manuscript, York University and University of
  Toronto, undated, no venue found. F, close.** For planted puzzles tuned so
  that the solution is unique with probability 1 − Θ(ε), backtracking that
  grows a rectangle from a corner has expected branching width 1 + Θ(ε)
  (linear expected time). Filling the frame first gives 2^Θ(√(ε t)) partial
  solutions; a block in the middle gives n^Θ(log n); a triangle from a corner
  gives 2^Θ(ε√n). With slightly more compatible pairs, so that solutions are
  exponentially many, every order takes exponential time, decreasing as the
  number of solutions grows. The authors report that their code behaves as
  the theorems say.
  *Change:* this is a proved null model for three items the program holds on
  argument: frame-first construction ("frame is not the basin"), centre-out
  ("loses on 5x5 to 8x8", nothing committed) and the fill-order row
  ("saturated: lever on tree size, not on ceiling"). In the unique-solution
  planted regime, order changes the exponent, and row-major scanning is
  optimal for a structural reason. The official puzzle is not in that regime
  (about 1 to 10^4 solutions and far fewer colours than uniqueness needs), so
  the theorem predicts the easy side only. It also predicts that the
  program's planted ladder becomes easy for scan-order DFS once colours exceed
  the uniqueness threshold, which is a check on the ladder's calibration.

### 5.3 Planted against random, and the overlap gap

- **Krzakala, Zdeborová, "Hiding Quiet Solutions in Random Constraint
  Satisfaction Problems", Phys. Rev. Lett. 102, 238701 (2009),
  arXiv:0901.2130. A, method.** For some problem classes the planted ensemble
  has the same quantitative properties as the random one, below a threshold.
  *Change:* the program runs its ladder on planted boards and reasons about an
  instance of unknown origin. Quiet planting is the condition under which that
  is legitimate; it is not cited in the repository.
- **Gamarnik, "The overlap gap property", PNAS 118 (2021),
  doi:10.1073/pnas.2108492118, arXiv:2109.14409. A, method.** Already cited on
  the 470-wall page.
- **Gamarnik, Moore, Zdeborová, "Disordered Systems Insights on Computational
  Hardness", J. Stat. Mech. (2022) 114015, arXiv:2210.08312. A, method.** The
  review that ties overlap gaps, low-degree hardness and planted inference
  together. *Change:* the overlap gap rules out stable algorithms (local
  search, low-degree, message passing) on random ensembles. It says nothing
  about backtracking with restarts on one instance. The 470-wall page should
  say which algorithm class its conjecture would bind.

### 5.4 Counting and rare-event estimators

- **Vaisman, Kroese, "Stochastic Enumeration Method for Counting Trees",
  Methodol. Comput. Appl. Probab. 19, 31 to 73 (2017),
  doi:10.1007/s11009-015-9457-4. M plus search summary, method.** A
  multi-path version of Knuth's probe with an oracle, shown to be a fully
  polynomial randomised approximation scheme for almost all random trees.
- **Rubinstein, "The Gibbs Cloner for Combinatorial Optimization, Counting and
  Sampling", Methodol. Comput. Appl. Probab. 11, 491 to 549 (2009). M,
  method.** Splitting with Gibbs moves for counting solutions.
- **Botev, Kroese, "Efficient Monte Carlo simulation via the generalized
  splitting method", Stat. Comput. 22, 1 to 16 (2012). M, method.**
- **Cérou, Guyader, "Adaptive Multilevel Splitting for Rare Event Analysis",
  Stoch. Anal. Appl. 25, 417 to 443 (2007). M, method.**

*Change:* IDEAS ranks 6 and 10 (splitting on local-search runs, pricing 471)
re-derive this literature. The published form gives the estimator and its
variance; population annealing is the same particle scheme with a temperature
ladder, and its family entropy is the diagnostic Hamze et al. validated. A2
(score against attempts as an extreme-value curve) would be better posed as a
splitting estimate of the density of boards at each score than as a fit to
maxima.

No statistical-physics or entropy treatment of two-dimensional random edge
matching from 2019 to 2026 was found (section 9).

## 6. Tooling

### 6.1 Tree-size estimation during a run

- **Knuth 1975** (Math. Comp. 29, 122 to 136). M. Already cited on the site.
- **Chen, "Heuristic Sampling", SIAM J. Comput. 21, 295 to 315 (1992),
  doi:10.1137/0221022. M, method.** Stratified probes that reduce Knuth's
  variance.
- **Kilby, Slaney, Thiébaux, Walsh, "Estimating Search Tree Size", AAAI 2006,
  1014 to 1019. F (abstract and conclusion), method.** Two online estimators
  computed from the branches a chronological backtracker has already visited:
  a weighted backtrack estimator and a recursive one that assumes the
  unexplored right branch resembles the explored left one. Both unbiased,
  usable inside branch and bound, and accurate enough to select the better of
  two algorithms on an instance.
- **Lelis, Otten, Dechter, "Predicting the Size of Depth-First Branch and
  Bound Search Trees", IJCAI 2013, 594 to 600. Search summary only, see
  section 8.**
- **Belov, Esler, Fernando, Le Bodic, Nemhauser, "Estimating the size of
  search trees by sampling with domain knowledge", IJCAI 2017, 473 to 479,
  doi:10.24963/ijcai.2017/67. M, method.**
- **Hendel, Anderson, Le Bodic, Pfetsch, "Estimating the Size of
  Branch-and-Bound Trees", INFORMS J. Comput. 34(2), 934 to 952 (2022),
  doi:10.1287/ijoc.2021.1103. M plus search summary of the abstract,
  method.** Shows the size cannot be approximated within a factor 2 unless
  P = NP; combines progress measures in a random forest and estimates sizes
  within a factor 3 on average on unseen instances.

*Change:* the program uses Knuth probes (A7) and nothing online. A weighted
backtrack estimator costs a few counters in `bb-mrv` and the benchmark engine,
and turns "cap 1e6 nodes, unsolved" into "estimated tree 10^k", which is the
quantity the growth-with-n protocol needs for censored runs. A7 (price set_2)
should report an online estimate next to the probe estimate.

### 6.2 Checkpointing long searches

- **Shinano, Achterberg, Berthold, Heinz, Koch, Winkler, "Solving Open MIP
  Instances with ParaSCIP on Supercomputers Using up to 80,000 Cores", IPDPS
  2016, 770 to 779, doi:10.1109/IPDPS.2016.56. M plus search summary,
  method.** 12 open MIPLIB instances solved; an earlier report from the same
  group describes solving two instances across more than ten restarts from
  checkpoint files. *Change:* the supervisor design (save only the open
  subproblem roots, restart by re-solving from them) is what the five-clue
  fleet's "bounded tickets, never repeats completed work" does by hand. The
  frontier-of-open-subtrees checkpoint is the format to adopt for any run
  longer than one night.

### 6.3 Benchmarking and reproducibility practice

- **Froleyks, Heule, Iser, Järvisalo, Suda, "SAT Competition 2020", Artificial
  Intelligence 301, 103572 (2021). M, method.** The reference description of
  the competition's rules: fixed instance sets chosen before the event,
  penalised average runtime, mandatory proofs for unsatisfiable answers.
  Content not re-read.
- **Beyer, Löwe, Wendler, "Reliable benchmarking: requirements and
  solutions", STTT 21, 1 to 29 (2019), doi:10.1007/s10009-017-0469-y. M,
  method.** BenchExec: measurement with cgroups so that memory, core pinning
  and child processes are accounted. Content not re-read.
- **Gocht, McCreesh, Nordström, "An Auditable Constraint Programming Solver",
  CP 2022, LIPIcs 235, 25:1 to 25:18. A, method.** A CP solver that writes a
  proof log checkable by VeriPB, including for global constraints.
- **Bogaerts, Gocht, McCreesh, Nordström, "Certified Symmetry and Dominance
  Breaking for Combinatorial Optimisation", JAIR 77, 1539 to 1589 (2023),
  arXiv:2203.12275. A, method.** Cutting-planes certificates that cover
  symmetry breaking and optimality claims.
- **Fichte, Le Berre, Hecher, Szeider, "The Silent (R)evolution of SAT",
  Commun. ACM 66(6), 64 to 72 (2023), doi:10.1145/3560469. M, method.** A
  comparison of old and new solvers on old and new hardware. Content not
  re-read; read it before deciding how stale the 2008 to 2013 SAT negatives
  are.

*Change:* (1) The night's prereg guard is the same idea as the competition's
frozen instance set; the missing parts are a penalised score for censored
runs and pinned-core measurement (the audit found a 4-thread against
8-thread drop that "was not isolated"). (2) The program publishes "proven
MaxSAT optimum" (MOSAIC) and UNSAT halo results without proof logs. Kissat
and CaDiCaL emit DRAT, and VeriPB covers MaxSAT and CP; a claim of proof on
the site should carry a checked certificate.

## 7. Table of every source

| # | Reference | Link | Verified | Grade | One-line finding |
|---|---|---|---|---|---|
| 1 | Hamze et al., From Near to Eternity, PRE 97, 043303 (2018) | arXiv:1711.04083 | F | close | Planted 3D tile-matching ensemble, tunable, 2 to 3 orders harder than spin glasses; PAMC family entropy tracks solver time |
| 2 | Seifer, Pipeline for Solving Edge-Matching Puzzles, bioRxiv (2026) | doi:10.64898/2026.05.23.727379 | A | direct | D-Wave hybrid solves up to 64 tiles only; no official-board score |
| 3 | Seifer, edge-matching solver code (2025) | doi:10.5281/zenodo.18109266 | F (record) | direct | MATLAB code; no results on the record or README |
| 4 | Audemard, Lecoutre, Lonca, Proceedings of the 2022 XCSP3 Competition | arXiv:2209.00917 | F (section 2.1.8) | direct | Eternity series of 10 small CSP instances from UCLouvain, 2018 |
| 5 | XCSP3 2018 result page, Eternity-10-05 | cril.univ-artois.fr/XCSP18/results/bench.php?idev=95&idbench=141651 | F | direct | All mini-solvers find a solution; best 4.8 s |
| 6 | Audemard, Lecoutre, Lonca, Proceedings of the 2025 XCSP3 Competition | arXiv:2511.06918 | F (text search) | direct | No Eternity benchmark in 2025 |
| 7 | BOINC project Eternity II, project page (read 2026-11-12) | boinc.eternityathome.org/eternity/ | F | direct, report | 466/480 "5 official clues", four boards 7 to 12 Nov 2026, OpenCL app |
| 8 | BOINC Synergy wiki, Eternity II (snapshot 30 Sep 2026) | boincsynergy.ca/wiki/Eternity_II | F | direct, report | Public pilot since September 2026; 215 active hosts |
| 9 | Guzman Marte, Frames, Signatures and Joints (2026) | doi:10.5281/zenodo.23132447; github.com/Edison82do/eternity2-carlos-guzman | F (README) | direct, report | 1.3 to 1.9e9 nodes/s on a GTX 1070; logged negatives for SAT, parity, Hall, GPU pruning |
| 10 | gearashi/EternityIISolver (2026) | github.com/gearashi/EternityIISolver | F (README) | direct, report | 12.4M samples/s on RTX 2060; no search gain; best board imported |
| 11 | E555solver/E555 (2026) | github.com/E555solver/E555 | F (README) | direct, report | Beam over 3.1e9 five-piece row chains, CP-SAT tails; no score stated |
| 12 | Bosboom et al., Even 1 × n Edge-Matching and Jigsaw Puzzles are Really Hard (2016) | arXiv:1701.00146 | A | direct | Inapproximable within 0.9999999851; first correct proof |
| 13 | Antoniadis, Lingas, Approximability of Edge Matching Puzzles, SOFSEM 2010 | doi:10.1007/978-3-642-11266-9_13 | M | direct | APX-hardness claim; proof reported incomplete by source 12 |
| 14 | Ebbesen, Fischer, Witt, Edge-matching Problems with Rotations (2017) | arXiv:1703.09421 | A | direct | FPT in the number of colours; cited on the site with wrong authors |
| 15 | Kovalsky, Glasner, Basri, A Global Approach for Solving Edge-Matching Puzzles, SIAM J. Imaging Sci. 8 (2015) | arXiv:1409.5957 | A | direct | Convex relaxations; cited on the site under a wrong title |
| 16 | Sewa et al., Assignment Problems in Cost Function Networks, AAAI 2026 | doi:10.1609/aaai.v40i17.38447 | M | close | AllDifferent with assignment bounds in toulbar2; QAPLIB |
| 17 | Montalbano, de Givry, Katsirelos, VAC for Linear Constraints (2025) | arXiv:2509.17706 | A | method | Stronger soft-consistency bounds with linear constraints |
| 18 | Trösser, de Givry, Katsirelos, Relaxation-Aware Heuristics, CPAIOR 2020 | doi:10.1007/978-3-030-58942-4_31 | M | method | VAC-integrality heuristics |
| 19 | Hurley et al., Multi-language evaluation of exact solvers, Constraints 21 (2016) | doi:10.1007/s10601-016-9245-y | M | method | Same instances across CFN, ILP, MaxSAT, CP |
| 20 | Li et al., Combining Clause Learning and Branch and Bound for MaxSAT, CP 2021 | doi:10.4230/LIPIcs.CP.2021.38 | A | method | MaxCDCL makes branch and bound competitive |
| 21 | Bergman, Cire, van Hoeve, Hooker, Discrete Optimization with Decision Diagrams, IJOC 28 (2016) | doi:10.1287/ijoc.2015.0648 | M | method | Relaxed and restricted diagrams as a branch-and-bound scheme |
| 22 | Gillard, Schaus, Coppé, Ddo, IJCAI 2020 | doi:10.24963/ijcai.2020/757 | A | method | Generic Rust library for that scheme |
| 23 | Rudich, Cappart, Rousseau, Improved Peel-and-Bound, JAIR 77 (2023) | arXiv:2302.05483 | A | method | Reuses diagrams between nodes; closes 15 open TSPTW instances |
| 24 | Perron, Didier, Gay, The CP-SAT-LP Solver, CP 2023 | doi:10.4230/LIPIcs.CP.2023.3 | A | method | Lazy clause generation plus simplex plus a sharing portfolio |
| 25 | Elffers, Nordström, Divide and Conquer, IJCAI 2018 | doi:10.24963/ijcai.2018/180 | A | method | RoundingSat, cutting-planes conflict analysis |
| 26 | Gmys, Exactly Solving Hard Permutation Flowshop Problems, IJOC 34 (2022) | doi:10.1287/ijoc.2022.1193; arXiv:2012.09511 | A | method | 339e12 nodes, 256 V100 GPUs, 13 h; 11 open instances solved |
| 27 | Helbecque et al., Portable PGAS-Based GPU-Accelerated B&B at Scale (2025) | doi:10.1002/cpe.70321 | M | method | Multi-vendor GPU backtracking, near-linear scaling reported |
| 28 | Talbot, Pinel, Bouvry, A Variant of Concurrent Constraint Programming on GPU, AAAI 2022 | arXiv:2207.12116 | A | method | GPU constraint solver with propagation on device |
| 29 | Tardivo et al., Constraint propagation on GPU: AllDifferent, J. Logic Comput. 33 (2023) | doi:10.1093/logcom/exad033 | M | method | GPU AllDifferent propagator, size-dependent speedup |
| 30 | Tardivo, Experimenting with Constraint Programming on GPU, ICLP 2019 | arXiv:1909.09213 | A | method | Early GPU CP design note, no measurements in the abstract |
| 31 | Böther et al., What's Wrong with Deep Learning in Tree Search, ICLR 2022 | arXiv:2201.10494 | A | method | Network replaceable by random values; results not reproducible |
| 32 | Angelini, Ricci-Tersenghi, Modern graph neural networks do worse than classical greedy algorithms, Nat. Mach. Intell. 5 | doi:10.1038/s42256-022-00589-y; arXiv:2206.13211 | M | method | Greedy beats a published network by orders of magnitude |
| 33 | Tönshoff et al., One Model, Any CSP (2022) | arXiv:2208.10227 | A | method | Learned global local-search heuristic; size transfer claimed on random instances |
| 34 | Xu, Li, Sanner, Khalil, Self-Supervised Transformers as Iterative Solution Improvers, ICML 2025 | arXiv:2502.15794 | A | method | Sudoku-scale constraint problems |
| 35 | Scavuzzo et al., Machine learning augmented branch and bound, Math. Program. 217 | arXiv:2402.05501 | A | method | Survey of learned branching |
| 36 | Romera-Paredes et al., FunSearch, Nature 625 (2024) | doi:10.1038/s41586-023-06924-6 | M + abstract | method | Program search under an evaluator: cap sets, bin packing |
| 37 | Novikov et al., AlphaEvolve (2025) | arXiv:2506.13131 | A | method | Evolved programs; 4x4 complex matrix multiplication in 48 products |
| 38 | Georgiev, Gómez-Serrano, Tao, Wagner, Mathematical exploration and discovery at scale (2025) | arXiv:2511.02864 | F (first third) | method | 67 problems; "cheating phenomenon"; verifier design is key |
| 39 | Yu, Liang, Ho, Ren, Autonomous Code Evolution Meets NP-Completeness (2025) | arXiv:2509.07367 | A | method | Evolved SAT solvers reported to beat 2025 competition winners |
| 40 | Zhang et al., Understanding the Importance of Evolutionary Search, PPSN 2024 | arXiv:2407.10873 | A | method | Earlier comparisons used weak baselines; open benchmark |
| 41 | Heule, Scheucher, Happy Ending (2024) | arXiv:2403.00737 | A | method | Cube and conquer, linear speedup on thousands of cores, UNSAT proof |
| 42 | Subercaseaux, Heule, Packing Chromatic Number of the Infinite Square Grid is 15, TACAS 2023 | arXiv:2301.09757 | A | close | Encoding and symmetry breaking gave two orders of magnitude |
| 43 | Jha et al., AlphaMapleSAT (2024) | arXiv:2401.13770 | A | method | Tree-search cubing, 1.61 to 7.57 times over March on 128 cores |
| 44 | Wang, Machta, Katzgraber, Comparing Monte Carlo methods, PRE 92, 013303 (2015) | arXiv:1412.2104 | A | method | Population annealing matches tempering, beats annealing |
| 45 | Wang, Machta, Katzgraber, Population annealing: theory, PRE 92, 063307 (2015) | doi:10.1103/PhysRevE.92.063307 | M | method | Theory and diagnostics of population annealing |
| 46 | Zhu, Ochoa, Katzgraber, Efficient Cluster Algorithm, PRL 115, 077201 (2015) | arXiv:1501.05630 | A | method | Isoenergetic cluster moves between replicas |
| 47 | Syed et al., Non-Reversible Parallel Tempering, JRSS B 84 (2022) | arXiv:1905.02939 | A | method | Non-reversible swaps and optimal schedules |
| 48 | Tasseff et al., Emerging Potential of Quantum Annealing Hardware (2022) | arXiv:2210.04291 | A | method | Gains on contrived native classes only |
| 49 | Campbell, Khurana, Montanaro, Applying quantum algorithms to CSPs, Quantum 3, 167 (2019) | doi:10.22331/q-2019-07-18-167 | M | method | Resource estimates for quantum backtracking |
| 50 | Abbas et al., Challenges and Opportunities in Quantum Optimization, Nat. Rev. Phys. (2024) | arXiv:2312.02279 | A | method | Survey; advantage framed as possible, not shown |
| 51 | Nenadov, Pfister, Steger, Unique reconstruction threshold for random jigsaw puzzles (2016) | arXiv:1605.03043 | A | close | Not unique for q = o(n); unique for q ≥ n^(1+ε) |
| 52 | Bordenave, Feige, Mossel, Shotgun Assembly of Random Jigsaw Puzzles (2016) | arXiv:1605.03086 | A | close | Same threshold with a polynomial assembly algorithm |
| 53 | Martinsson, A linear threshold for uniqueness, CPC 28 (2019) | arXiv:1701.04813 | A | close | Multiple solutions up to q = (2/√e) n; similar above (2+ε) n |
| 54 | Edmonds, Edmonds, Mertz, Search Time when Solving Random Jigsaw Puzzles with a Planted Solution (undated manuscript) | eecs.yorku.ca/~jeff/research/tsot/puzzle/puzzle.pdf | F | close | Fill order changes the exponent; corner rectangle has width 1 + Θ(ε) |
| 55 | Krzakala, Zdeborová, Hiding Quiet Solutions, PRL 102, 238701 (2009) | arXiv:0901.2130 | A | method | When planted and random ensembles coincide |
| 56 | Gamarnik, The overlap gap property, PNAS 118 (2021) | arXiv:2109.14409 | A | method | Barrier to stable algorithms; already cited on the site |
| 57 | Gamarnik, Moore, Zdeborová, Disordered Systems Insights on Computational Hardness (2022) | arXiv:2210.08312 | A | method | Review linking glassiness and algorithmic hardness |
| 58 | Vaisman, Kroese, Stochastic Enumeration Method for Counting Trees (2017) | doi:10.1007/s11009-015-9457-4 | M | method | Multi-path Knuth estimator with guarantees on random trees |
| 59 | Rubinstein, The Gibbs Cloner (2009) | doi:10.1007/s11009-008-9101-7 | M | method | Splitting for counting and optimisation |
| 60 | Botev, Kroese, Generalized splitting (2012) | doi:10.1007/s11222-010-9201-4 | M | method | Splitting estimator with Gibbs moves |
| 61 | Cérou, Guyader, Adaptive Multilevel Splitting (2007) | doi:10.1080/07362990601139628 | M | method | Adaptive level choice for rare events |
| 62 | Knuth, Estimating the efficiency of backtrack programs (1975) | doi:10.1090/S0025-5718-1975-0373371-6 | M | method | Random-probe estimator; already cited on the site |
| 63 | Chen, Heuristic Sampling, SIAM J. Comput. 21 (1992) | doi:10.1137/0221022 | M | method | Stratified version of Knuth's probe |
| 64 | Kilby, Slaney, Thiébaux, Walsh, Estimating Search Tree Size, AAAI 2006 | users.cecs.anu.edu.au/~thiebaux/papers/aaai06.pdf | F (abstract, conclusion) | method | Two online unbiased estimators usable during a run |
| 65 | Belov et al., Estimating the size of search trees by sampling with domain knowledge, IJCAI 2017 | doi:10.24963/ijcai.2017/67 | M | method | Probe estimators with a learned model |
| 66 | Hendel et al., Estimating the Size of Branch-and-Bound Trees, IJOC 34 (2022) | doi:10.1287/ijoc.2021.1103 | M | method | Within a factor 3 on average on unseen instances |
| 67 | Shinano et al., Solving Open MIP Instances with ParaSCIP, IPDPS 2016 | doi:10.1109/IPDPS.2016.56 | M | method | 80,000 cores; checkpoint and restart of open subproblems |
| 68 | Froleyks et al., SAT Competition 2020, AIJ 301 (2021) | doi:10.1016/j.artint.2021.103572 | M | method | Competition rules and scoring |
| 69 | Beyer, Löwe, Wendler, Reliable benchmarking, STTT 21 (2019) | doi:10.1007/s10009-017-0469-y | M | method | BenchExec measurement requirements |
| 70 | Gocht, McCreesh, Nordström, An Auditable Constraint Programming Solver, CP 2022 | doi:10.4230/LIPIcs.CP.2022.25 | A | method | Proof-logging CP solver |
| 71 | Bogaerts et al., Certified Symmetry and Dominance Breaking, JAIR 77 (2023) | arXiv:2203.12275 | A | method | Certificates for optimisation with symmetry breaking |
| 72 | Fichte et al., The Silent (R)evolution of SAT, CACM 66 (2023) | doi:10.1145/3560469 | M | method | Solver progress against hardware progress |

## 8. Unverified leads

- **MaxSAT Evaluation 2024 rankings.** The results page I tried returned 404.
  The claim that MaxCDCL-family solvers won the exact unweighted track comes
  from the abstract of an IJCAI 2025 paper seen in search results only.
- **AESAT / AE-Kissat-MAB, "Bridging LLMs and SAT Solving", IJCAI 2026
  (ijcai.org/proceedings/2026/257).** Search results state it won the 2025 SAT
  Competition main track. Page not opened; check the official results.
- **AutoSAT (arXiv:2402.10705) and AutoModSAT (arXiv:2507.22876).** Seen in
  search results only.
- **Liu et al., Evolution of Heuristics (arXiv:2401.02051).** Search results
  only.
- **Lelis, Otten, Dechter, IJCAI 2013 (ijcai.org/Abstract/13/095).** Title and
  venue from a search summary and DBLP listing; page not opened.
- **Mossel, Ross, "Shotgun assembly of labeled graphs" (arXiv:1504.07682).**
  The origin of the random jigsaw model; cited by sources 51 to 53, not
  opened.
- **Schuetz, Brubaker, Katzgraber reply (arXiv:2302.03602)** to source 32.
  Not opened.
- **Codognet, Diaz, Abreu, quantum and digital annealing for the QAP (2022).**
  Seen through a repository listing only.
- **Knud-Aage/E2-Blackwood-GPU** (a CUDA port of the Blackwood solver). The
  URL returned 404 tonight; search results describe 1,024 independent GPU
  searches and no throughput figure.
- **silveremartin-dev/Eternity** (Java, distributed, TornadoVM). Search
  results only; its table lists the 16x16 as in progress.
- **Takenaga, Walsh, "Tetravex is NP-complete" (2006).** Known from search
  results; not opened.
- **Edmonds, Edmonds, Mertz (source 54)** was read in full but has no date,
  venue or DOI. Treat it as a manuscript until a published version is found.
- **Details not confirmed in full text:** the 17,300 CPU hours of source 41,
  the Turbo against Gecode node rates of source 28, the PAR-2 figures of
  source 39, the scaling numbers of source 27, the expected-count formula of
  source 53, and the statement in source 12 that the proof of source 13 is
  incomplete. Each came from a search summary of the text.
- **Whether the BOINC project's 466 boards satisfy all five official clues.**
  The page shows the label next to the record and nothing more.

## 9. Searches that found nothing

The five absences that matter most, with the queries used (all on
2026-11-12):

1. **No weighted CSP, MaxSAT or cost-function-network work on edge
   matching.** Queries: `"Eternity II" "MaxSAT" OR "weighted CSP" OR
   "toulbar2" OR "cost function network" edge matching`; `MaxSAT OR
   "pseudo-Boolean" OR "CP-SAT" solve "jigsaw" OR "tiling" OR "edge-matching"
   OR "Tetravex" puzzle encoding benchmark results lazy clause generation`;
   `toulbar2 OR "cost function network" "jigsaw" OR "puzzle" OR "quadratic
   assignment" exact solving results "AllDifferent"`. The last one found the
   QAP work of source 16 and nothing on puzzles.
2. **No decision-diagram work on edge matching, jigsaw or Tetravex.** Query:
   `"decision diagram" OR "MDD" "edge-matching" OR "Eternity II" OR "jigsaw"
   OR "Tetravex" relaxed diagram bound puzzle`.
3. **No score above 470 and no solution claim.** Query: `"Eternity II" solved
   OR solution claim 480 OR "471" OR "472" edges 2024 2025 2026`.
4. **No learning-based result on edge matching (reinforcement learning, tree
   search with networks, graph networks, diffusion, transformers) in a paper
   or thesis.** Queries: `"Eternity II" reinforcement learning OR "Monte Carlo
   tree search" OR "AlphaZero" edge-matching puzzle results 2020..2026`;
   `Tetravex OR "edge-matching" puzzle "graph neural network" OR diffusion OR
   "learning to solve" constraint satisfaction 2022..2026`; `"edge-matching"
   puzzle deep reinforcement learning OR transformer OR "neural" Eternity II
   thesis`. Nor any evolved-heuristic work: `FunSearch OR AlphaEvolve OR
   OpenEvolve OR "LLM" evolved heuristic "edge-matching" OR "Eternity II" OR
   "exact cover" OR "tiling puzzle"`.
5. **No statistical-physics, entropy or phase-transition study of
   two-dimensional edge matching after 2013, and no peer-reviewed Eternity II
   paper after 2018.** Queries: `"edge-matching" puzzle random instances
   "number of solutions" OR "expected number" OR "first moment" OR
   "statistical mechanics" OR "entropy" square tiles colours 2019..2026`;
   `"edge-matching" puzzles hardness "phase transition" OR "NP-complete" OR
   "ETH" OR "approximation" 2019..2026`; `"Eternity II" puzzle solver paper
   2023 OR 2024 OR 2025 edge-matching arXiv`; `"Eternity II" OR
   "edge-matching puzzle" 2019 OR 2020 OR 2021 OR 2022 paper OR thesis OR
   mémoire solver`.

Limits of these absences: one search engine, English and some French terms,
no access to Google Scholar citation graphs, groups.io or Discord. A scout's
"none found" is a lead.

## 10. Ten rows the explored map should add or change

| # | Row | Change | Justifying source |
|---|---|---|---|
| 1 | **Add**, Encodings: weighted CSP and cost function networks (soft arc consistency, AllDifferent with assignment bounds) | New row, status **open, never tried on edge matching anywhere**. Target: lower bounds and exact optima on halos, tails and sub-boards in the break-tolerant regime. | 16, 17, 19; absence 1 |
| 2 | **Add**, Encodings: MaxSAT beyond core-guided (branch and bound with learning, implicit hitting set) | New row, status **thin**: one family (RC2) used on 4x4 blocks in MOSAIC. | 20; absence 1 |
| 3 | **Add**, Systematic search: decision-diagram bounds for break-tolerant DFS | New row, status **open**, linked to IDEAS T4; note that a generic library and a diagram-reuse scheme exist. | 21, 22, 23; absence 2 |
| 4 | **Change**, "SAT / CSP encodings, cube and conquer" | Split into two rows. SAT and CSP on the full board: mapped, with the caveat that the evidence is 2008 to 2013 solvers plus one modern measurement here. Cube and conquer: **thin**, and scoped to UNSAT questions (halo proofs, local non-existence of 471), where all published successes are. | 41, 42, 43, 24, 72 |
| 5 | **Change**, Hardware: GPU | Keep **open**, replace "measured ports, no tuned full-board solver" by "published at 10^14 nodes for permutation branch and bound (2022); practitioner ports at 10^9 nodes/s on a 2016 card (2026); a live BOINC OpenCL client; propagation on GPU loses, subtree search wins". | 26, 27, 28, 29, 9, 7 |
| 6 | **Change**, Hardware: quantum | From "thin, zero runs" to "one outside run: hybrid annealer, complete only up to 64 tiles (2026 preprint); resource analyses negative". | 2, 48, 49 |
| 7 | **Change**, Learning: "Transformer board generation, heuristics learned on small boards: mapped, dead ends", and **add** "program search over heuristic code" | First row to **thin here, negative outside** (two audited neural tree-search claims did not reproduce; no edge-matching result exists). New row **open**, with the condition that the evaluator and node counter sit outside the evolved code. | 31, 32, 33; 38, 39, 40; absence 4 |
| 8 | **Change**, Local and stochastic search: "Simulated annealing, parallel tempering: saturated" | To **mapped**, listing three untested variants: population annealing, replica-overlap cluster moves, non-reversible swaps with schedule optimisation. | 44, 45, 46, 47, 1 |
| 9 | **Add**, Theory: random jigsaw thresholds and planted search time as the null model | New line under "Theory already on the site" as **missing**: linear uniqueness threshold, and a proof that fill order changes the exponent on planted instances (corner rectangle easy; frame first, centre block, corner triangle exponential). The rows "fill orders: saturated", "centre-out" and "frame-is-not-the-basin" should cite it as the baseline their measurements are compared with. | 51, 52, 53, 54, 55 |
| 10 | **Change**, Explicit open problems and Hardware: distributed farms | "Strict five-clue 465 to 466" to **verify against the BOINC project's reported 466 (7 to 12 November 2026)**; "distributed farms: saturated" to **live** (a public farm with CPU and GPU clients since September 2026). | 7, 8 |

Two smaller corrections outside the map: the bibliography entries
`rotations2017` (authors) and `kovalsky2014` (title), section 2.6; and a
missing "instrumentation" entry for online tree-size estimators (sources 64,
66), which the growth-with-n protocol needs for censored runs.
