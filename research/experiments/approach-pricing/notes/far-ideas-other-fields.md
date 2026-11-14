# Far ideas: what fields that look unrelated would do with Eternity II

Draft of 2026-11-13, not independently checked. Lens: fields that look unrelated to combinatorial search. Grades on every number: **measured** (toy run in `far-ideas-other-fields/`), **derived** (arithmetic shown or in a script), **cited** (source opened today, link given), **unverified** (source not opened), **guessed**. Toy results are on generated 5x5 to 7x7 boards and are never Eternity II progress.

Bottom line first. Of 32 ideas from 20 fields, 29 map onto a route already priced out (named per row) or have a crisp reason to be blind here. The three I would keep are not good bets either: a physical side channel in how the pieces were printed (highest payoff, odds about 0.2 percent, settled by one photograph), coarse-to-fine colour merging (renormalisation and homotopy continuation; dead on cost in the toy probe and on distance by count), and nonabelian tiling-group invariants (topology; theory and probe both say blind). The pattern behind the kills is the one in THESIS_PLAN 9.3, seen from twenty directions: every tool that "sees structure" either needs a measurement this puzzle does not provide (a received word, a diffraction pattern, a family of homologues, a continuous fit score) or reduces to a test that only bites once few tiles remain.

## 1. The list

Each entry: field and tool; what it might see; the first reason it probably fails here. "Row" means a row of `ROUTES.md`.

1. **DNA tile self-assembly, proofreading tile sets (Winfree and Bekbolatov 2003, unverified).** Cooperative binding at temperature 2 and k x k proofreading blocks make every growth error need two coincident mismatches. Fails: proofreading is redundancy the designer builds into the tile set; this set is given and sits at E[Z] about 1 with none to spare. Kinetic growth itself already lost to DFS (A15: 23 of 30 against 30 of 30 at 6x6, row "Blocker-eviction").

2. **DNA computing as massive parallelism (Adleman style).** A test tube assembles about 2^70 to 2^79 structures at once (a mole is 2^79). Fails twice: each-tile-once cannot be enforced in bulk, since every species is present in many copies, so the tube solves the reuse (Wang) problem whose solutions are irrelevant; and 2^79 is 55 bits short even if it could. Mismatch rates of 1e-3 to 1e-4 per attachment add a third failure at 256 perfect joints.

3. **Crystallography: direct methods and phase retrieval (Sayre, Hauptman and Karle).** Recover a structure from measured Fourier magnitudes plus atomicity. Fails: there is no measurement. The tile multiset is the composition, i.e. only the zero-frequency term; with no magnitudes there is no phase to retrieve. Iterative projections beyond the plain difference map (RAAR, RRR) are the same family Track 11 measured at 0 of 30 near critical (row "Difference map").

4. **Protein structure: contact prediction from co-evolution.** Contacts are read from correlated mutations across thousands of homologous sequences. Fails: one instance, no family. The single-instance analog, pair statistics over the piece set, ranked true neighbours below colour rarity (A1: z = -7.94 at 8x8, row "Planted-solution recovery").

5. **Protein structure: fragment assembly (Rosetta).** Monte Carlo replacement of local fragments from a library. Fails: it is large-neighbourhood local search, killed as a route to a solution because boards at 470 sit 246 or more cells from any solution (row "A 470 board as a damaged solution").

6. **Genome assembly: de Bruijn graphs, tiles as edges.** In a row a tile is an edge from its west colour to its east colour; a row is a trail in the colour multigraph, and 1D assembly became easy when reads became edges (Eulerian path) instead of nodes (Hamiltonian path). Fails: the columns are a second trail structure over the same edges, and coupling two Eulerian decompositions is the 2D sequencing-by-hybridisation problem, hard again. Also (guessed, derived in outline) a valid row's tile set nearly fixes its order, so row sets carry the same entropy as row sequences and "sets first" gains nothing.

7. **Genome assembly: repeat resolution and unitigs.** Unique regions assemble trivially; only repeats are hard. Fails: at this density every patch is a repeat. The first-moment count of interior k x k blocks grows with k (about 1.8 bits per cell of positive area entropy, derived: 10 bits of placement freedom against 2 x 4.09 bits of joints), so no interior patch is ever unique. This is the Bordenave, Feige, Mossel window condition the tilings survey already quotes (k about 9,100).

8. **Computer-vision jigsaws: loop constraints (Son, Hays, Cooper 2014, unverified).** Closed 2 x 2 loops are far more reliable than pairs. Fails: compatibility is binary. About 2^23.7 tile quadruples close an interior 2 x 2 loop (derived: 256^4 x 4^4 x 17^-4, ignoring distinctness) against 225 true loops, and the ratio worsens with loop size by the area law above.

9. **Archaeology and fresco reconstruction: seriation (Robinson matrices, Fiedler vector).** Order fragments so that similar ones sit together. Fails: spectral seriation is the spectral pair statistic of A1, measured below colour rarity.

10. **Print and manufacturing forensics (physical fit, registration, ink zones, back-number orientation).** A physical object can leak what the abstract puzzle hides. Value is total if the printed sheet holds the pieces in solution orientation or position (section 2.1). Fails, probably: the digests say the pieces were sorted before printing and the community took orientation from the punch-out panels, which is the canonical form; the five clue rotations (0, 1, 1, 2, 1 in that form) show the canonical form is not the solution's. Developed as idea B.

11. **Discrete tomography (Ryser; multicolour reconstruction is NP-hard from three colours, Gardner, Gritzmann, Prangenberg 1999, unverified).** Search the cheap projections first: the colour multiset crossing each of the 15 horizontal cuts. Fails: forgetting positions along a cut throws away 240 x 4.09 = 981 bits of vertical joints and keeps only about 15 x 25 = 375 bits of multiset equalities (derived, rough: log2 of 17^16 over the number of orderings of a typical multiset), so the projected problem has about 2^600 times more solutions; projections in both directions together are the original problem.

12. **Mechanics and tomography: moment (virial) identities.** For each colour, the signed first moment of its half-edge positions balances on a solved board, giving linear equations in tile positions. Fails: these are linear integer laws, the family gap 5 computed (flux, checkerboard, row census) and found to bite only between depths 144 and 192 (THESIS_PLAN 9.3).

13. **Coding theory: iterative decoding of 2D product, turbo and LDPC codes; peeling.** Decode rows and columns alternately with soft information. Fails: decoding needs a received word close to the codeword. With no channel observation, "decoding" is counting codewords, and peeling is forced moves, of which there are none (topic `no-forced-moves`). BP itself is priced out by theory at this density.

14. **Harmonic analysis on the symmetric group (Kondor; Huang and Guestrin, unverified).** Keep low-order Fourier coefficients of a distribution over the tile-to-cell permutation. Fails: first-order coefficients are the Birkhoff polytope and second-order are QAP-style relaxations, i.e. the LP family, blind until late (row "LP / ILP").

15. **Holographic algorithms and dimer counting (Kasteleyn, FKT, Valiant).** Planar perfect matchings are counted in polynomial time; a planar grid model with local signatures sometimes reduces to them. Fails: signatures over 17 colours are generic and fall on the hard side of the planar #CSP dichotomies (Cai and coauthors, unverified), and each-tile-once is a global constraint that no local signature expresses.

16. **Linguistics: 2D picture languages and tiling systems (Giammarresi and Restivo).** Recognisable picture languages are exactly projections of local tiling systems. Fails: the theory gives closure and decidability results, not algorithms for a finite instance; a 2D CYK over sub-rectangles is a region table, which O1 prices at the strict table size.

17. **Sequence alignment and music (Needleman-Wunsch, dynamic time warping).** Exact dynamic programming in 1D. Fails: 2D alignment is NP-hard, and DP over rows with the used set is the transfer matrix with used set, priced at the size of the tree (row "Tensor networks...").

18. **Economics: stable matching and auctions.** Tiles bid for cells; prices clear the market. Fails: a tile's utility depends on its neighbours (matching with externalities, where stable outcomes need not exist), and auction prices are the Lagrangian per-piece prices of path 1, which isolated a solution on 2 of 19 toys (row "Tensor networks...").

19. **Neuroscience: Hopfield-Tank networks, softassign, modern Hopfield memories; with them Gromov-Wasserstein matching from shape analysis.** Relax the assignment to a soft doubly stochastic matrix and anneal. Fails: deterministic annealing is mean field, the same family as BP at this density; Hopfield-Tank is known to break down well below 256 assignment variables, and a modern Hopfield memory retrieves only what was stored.

20. **Game theory: the puzzle as a potential game.** Each tile is a player maximising its matched edges. Fails: Nash equilibria are local optima of the score; that is local search, saturated near 460 and 246 cells from any solution.

21. **Game theory on the maker: adversarial selection.** If the maker rejected paintings his own solver finished within T nodes, the solution lies outside the first T nodes of that order. Fails: that excludes a fraction T / 2^134 of the tree, about 0 bits for any T a 2007 machine could run (derived).

22. **Chemistry: hierarchical (two-handed) assembly and retrosynthetic disconnection.** Build sub-assemblies separately and join them; cut the target where the synthon is rare. Fails: joins are region tables with a used-set test, priced at 2^131.6 (O1); and no colour is rare (48 to 50 sides each), so there is no cheap disconnection.

23. **Physics of nucleation (Wulff shape, heterogeneous nucleation at walls).** The barrier is the widest level of the tree; its height depends on the nucleus shape, and nucleation starts at walls. Fails: the optimal growth shape is the optimal fill order, already priced (row scan within 0.24 bit of the best static order, gap 6b); the clues and corners are already the walls.

24. **Rigidity percolation (Maxwell counting, the Jacobs and Thorpe pebble game).** Find rigid clusters where constraints outnumber freedoms. Fails: the Maxwell count is the first-moment count, and interior blocks of every size have positive entropy, so no interior cluster is rigid; the pebble game also needs generic linear constraints, which matching is not.

25. **Topology: colour loops, Truchet arcs, height functions.** On a solved board each colour's matched edges form a system of arcs; turning numbers or heights might give an invariant computable from the tiles. Fails: most tiles carry a colour once, so each colour's "curve" is mostly single edges with endpoints and only parity survives; the height-function and invariant-group route is already done (gap 5: invariant group Z^2 + Z/2; topics `no-height-function`, `sigma-cycles`, `flux-invariants`). Pak's tile-counting invariants (cited, opened: [Pak, Tile invariants: new horizons](https://www.math.ucla.edu/~pak/papers/tilesurvey.pdf)) are relations between how often each tile is used; here every tile is used exactly once, so they are vacuous by construction.

26. **Combinatorial group theory: Conway-Lagarias tiling groups with nonabelian finite quotients.** What gap 5's abelian laws miss might live in a nonabelian quotient. Developed as idea C; theory and probe say blind.

27. **Sheaf cohomology of contextuality and cohomological k-consistency (Abramsky and coauthors; O Conghaile 2022, unverified).** An integer-affine relaxation on top of k-local consistency can see global obstructions that local consistency misses. Fails, probably: at k = 1 it is gap 5's linear laws; affine relaxations refute nonlinear random CSPs badly, and every refuter measured here starts biting after depth 144. Untested at k = 2; a 7x7 test is a day of coding.

28. **Renormalisation group, multigrid, homotopy continuation.** Solve a coarse version (colours merged) and refine. Developed as idea A, with a toy probe.

29. **Kinetic proofreading (Hopfield 1974).** Repeat a weak discrimination step to multiply its selectivity. Fails: what it amplifies here is the D4 tilt, about 1 bit, and repeats are correlated across scan directions; the rerandomisation arithmetic in O2 already prices this.

30. **Number theory: the generator.** The one account says colours came from "gaussian residues of powers of appropriately chosen primes"; the five clues are 20 known-plaintext edge colours, about 80 bits of filter on any candidate generator. Fails: the same account says about 200 keyboard inputs were mixed in as entropy, far beyond enumeration; this is the seed-recovery row ("Looks untried", not recommended).

31. **Probabilistic method: Lovasz Local Lemma (Moser-Tardos) and coupling from the past.** Resample violated constraints; or sample exactly from a mixing chain. Fails: the LLL needs slack far below the threshold and here E[Z] is about 1 by design; exact sampling of a solution at E[Z] about 1 is solving.

32. **Astronomy plate solving, Hi-C scaffolding, cryo-EM alignment.** Geometric hashing against a catalogue; ordering by a contact signal that decays with distance; aligning many noisy copies of one object. Fails: no catalogue, no distance-decaying signal (A1), and one copy of many objects instead of many copies of one.

## 2. The three developed

### A. Coarse-to-fine colour merging (renormalisation, multilevel refinement, homotopy continuation)

**Construction.** Choose a merge map m on the 17 interior colours (here, one pair merged). The coarse puzzle replaces each tile by its image under m. The true solution is a coarse solution. Two ways to use the coarse level, both implementable: (i) sample M coarse solutions, compute for each cell the vote (the most frequent fine tile and rotation there), and use vote order as the value order of the fine search; (ii) repair: from one coarse solution, search fine solutions within Hamming radius r cells of it. Repeat over a merge hierarchy, as multigrid does.

**What would have to be true for 10 bits.** Coarse solutions must concentrate near the truth. Price of the coarse level: merging two of 17 equally frequent colours raises the match probability from 17/289 to 19/289 on each of the 420 joints that carry interior colours, so the coarse puzzle has about (19/17)^420 = 2^67.4 times more solutions in the first moment (derived). If those 2^67 solutions were spread like random planted-ensemble solutions, quiet planting says the truth is a typical member and the vote carries nothing (cited, opened: [Krzakala and Zdeborova 2009, arXiv:0901.2130](https://arxiv.org/abs/0901.2130); the abstract states the planted ensemble reproduces the random one's properties for a class of problems). For 10 bits the vote would have to put the true tile first at the cells before depth 158 often enough to supply 10 bits, about 0.07 bit per cell, i.e. a measurable tilt on every board.

**Closest published work.** [Walshaw 2004, Multilevel refinement for combinatorial optimisation problems, Annals of OR 131](https://ideas.repec.org/a/spr/annopr/v131y2004i1p325-37210.1023-banor.0000039525.80601.15.html) (abstract opened; full text 403): the paradigm helps on graph partitioning, colouring and TSP, "not a panacea". Its explanation (coarsening restricts and smooths the space for a local refiner) presumes an objective with gradual structure; edge matching at the target is a needle.

**Smallest experiment and kill line.** Generated 6x6 boards (3 frame colours, 8 interior, centre clue), one interior pair merged; enumerate every fine and coarse solution; measure (a) the truth's centrality rank among coarse solutions (mean overlap with the ensemble, ranked against every coarse solution's own; 0.5 is no signal) and (b) the vote's accuracy on the truth as a percentile of its accuracy on each coarse solution. Kill: mean rank and mean percentile both under 0.65 over 20 or more boards. Cost: under 15 core-minutes. (Written before the run. In hindsight it lacked a cost line and a regime check; section 3.1 says why it did not fire and why that is not a signal.) **Result: section 3.1.**

**Odds of 10 bits:** under 0.1 percent (guessed), lower after the probe.

### B. The printed object as a side channel (print forensics, physical fit)

**Construction.** Examine unpunched sheets or punched pieces, not the abstract set. Three measurable channels: (i) the rotation of the front pattern relative to the upright number on the back, per piece; (ii) sheet position from print registration (offset between colour plates drifts smoothly across a sheet), ink-zone density (offset presses meter ink in column zones) and die-cut nicks that continue along a shared cut line; (iii) the sheet layout itself. Any of these is information about the solution only if the artwork was laid out from the solved board.

**What would have to be true.** Rotation alone is enough to end the problem. With every rotation known, a row-scan interior cell offers at most about 196 candidates (7.6 bits) against two joints (8.17 bits), so the tree is subcritical from the first interior row (derived; toy value in section 3.2). Knowing only the axis of each tile (1 bit) leaves about 8.6 bits against 8.17 at the start, still a small tree. Sheet position to within two cells would do the same. So the condition is not on the method; it is that the sheet reflects the solution in rotation or position.

**Closest published work and evidence.** The rules and Wikipedia ([opened](https://en.wikipedia.org/wiki/Eternity_II_puzzle)): "The other side of each piece is blank apart from an identifying number". The repo's own evidence, digest grade: the pieces were "sorted before printing" (digest 0010, a 2010 thread), the numbering is exactly a sort of canonical forms (verified on the file), and the community took orientation "from the punch-out panels" (msg 977 via digest), which fits a canonical print (`how-the-puzzle-was-made.md` sections 2 and 8, which already asks for a photograph of unpunched sheets). The clue rotations in canonical storage are 0, 1, 1, 2, 1, so canonical is not the solution's orientation. What is new here is only the price: if the photograph showed any non-canonical rotation pattern, it is worth everything, not a few bits.

**Smallest experiment and kill line.** One photograph of one unpunched sheet, front and back. Kill: pieces appear in number order and each in its canonical rotation (lowest side on the fixed side). Secondary check from a punched set: the 60 frame pieces with backs upright; if the grey side points the same way on all of them, the print is canonical and the channel is dead. Toy calibration in section 3.2.

**Odds:** about 0.2 percent that the channel exists (guessed; the digest evidence and 18 years of a million-pound prize argue against it). Payoff if it exists: a full solution. It is still the best ratio of payoff to cost on this list, because the cost is minutes.

### C. Nonabelian tiling-group invariants (Conway-Lagarias, Thurston; combinatorial group theory)

**Construction.** Map each colour to an involution of a finite group G (involutions because joints are unoriented and both tiles read a shared joint clockwise; the letter must equal its own inverse). A tile's word is the product of its four side letters clockwise; rotation conjugates it, so its conjugacy class is a tile invariant. For a row-scan prefix, the unfilled region R is simply connected; by van Kampen, if R can be tiled by the remaining tiles, the boundary word of R equals a product of conjugates of their words, each used once. In a finite group a product of conjugacy classes is a union of classes and does not depend on order, so the test is: the class of the boundary word lies in the class product of the remaining tiles. Computable with the class multiplication table; implemented for G = S_6 in `far_probes.py classprod`.

**What would have to be true for 10 bits.** The test must refute wrong prefixes before depth 158 beyond what the abelian quotient (parity and the linear laws of gap 5) already refutes. Theory says it cannot: in a nonabelian finite simple group a bounded number of non-central classes already cover the group (Liebeck and Shalev's covering-number bound, unverified: I saw it only quoted in a search result, not in an opened paper), and products of classes become near uniform ([Shalev, arXiv:1601.00795](https://arxiv.org/abs/1601.00795v2), abstract opened). With about 100 tiles remaining at the peak, the product of their classes is the whole coset of the commutator subgroup, so only the abelianisation survives, which is gap 5's object. An infinite quotient escapes this but brings the word problem with each relator used once, which is the puzzle again.

**Closest published work.** Conway and Lagarias 1990 (JCTA 53, unverified) and Pak's survey ([opened](https://www.math.ucla.edu/~pak/papers/tilesurvey.pdf)), whose tile-counting group is vacuous here because every tile is used once.

**Smallest experiment and kill line.** Generated 6x6 boards, colours to random involutions of S_6; random valid row-scan prefixes; at each depth, the share refuted by the class test and by its sign (abelian) part alone. Soundness check: the true prefix must never be refuted. Kill: the class test refutes no more than the sign test at any depth before the last row. **Result: section 3.3.**

**Odds of 10 bits:** under 0.01 percent (derived from the covering argument, confirmed on the toy).

## 3. Probes (toys)

All runs: one core, `nice -n 10`, `timeout 900`, stdlib Python, deterministic per seed. Generated boards: rim joints from F frame colours, all other inner joints from C interior colours, uniform; a centre clue. These boards are not Eternity II and a toy result is not progress.

### 3.1 Colour merging (idea A)

Run: `far_probes.py consensus 6 3 8 30 1` (6x6, 3 frame colours, 8 interior, interior colours 4 and 5 merged), output `consensus_n6_F3_C8.txt` (all 30 seeds, inside the 900-second limit). Six boards had fewer than three coarse solutions and were skipped; 24 used. Measured:

- **The coarse tree contains the fine tree**, so enumerating it always costs more: 1.6 to 99.7 times the fine enumeration's nodes, median about 7 (24 boards). This is also derived: every prefix valid with fine colours is valid with merged ones. A coarse level can only pay if its first solution comes much sooner and repair from it is cheap.
- **On toys the coarse solutions are a cluster around the fine ones.** On the 16 boards where merging added solutions (coarse count 2 to 8 times the fine count), the truth's mean overlap with the coarse ensemble is 0.895 against 0.885 for the average coarse solution, and its centrality rank is 0.96. My kill line on rank (written before the run, in section 2.A) therefore does not fire. It does not mean what it would have to mean: at 6x6 every solution set is a tight cluster of local swaps (fine solutions alone: 1 to 96 per board, overlap about 0.9 with each other), so being central costs nothing and identifies nothing. The vote does not single the truth out: its accuracy on the truth sits at the 0.28 percentile, on average, of its accuracy on the coarse solutions (16 boards), below the 0.5 of no signal, because sibling fine solutions tie with it.
- **Why the toy does not transfer, derived.** On the 16x16 one merge adds about 2^67 coarse solutions per fine one. A coarse solution that is not a fine one carries tens of a/b mismatched joints (about 420 x (4/289) / (19/289), roughly 90 joints carry a merged colour, half of them mismatched if a and b were random). Path 4's count (argued, not independently checked) puts boards with that many mismatches 246 or more cells from any solution, and only about 10^9 near-solutions per solution within a few cells, so the share of coarse solutions near the truth is about 2^30 / 2^67 = 2^-37. The toy sits on the small-board side of the sign flip ROUTES records for A10 (between 8x8 and 12x12), where everything is near everything.

Verdict for idea A: dead on cost (measured, toy) and on distance (derived from an not independently checked count). A coarse level is a relaxation, and relaxations here add solutions faster than they remove work, as the tilings survey's item 3 already says for clue removal.

### 3.2 Value of known rotations (idea B)

Run: `far_probes.py rotation 6 3 5 10` (6x6, 3 frame colours, 5 interior, centre clue; full enumeration with a 5e7-node cap), output `rotation_n6_F3_C5.txt`. Measured: with every tile's true rotation known, full enumeration takes 54 to 328 nodes and finds 1 or 2 solutions; with rotations unknown it takes 1.8e7 and 3.8e7 nodes on the two boards that finished and more than 5e7 on the other eight. The ratio is at least 17.2 to 19.8 bits on 36 cells (lower bounds where capped). On the 16x16 the derivation in section 2.B gives a subcritical tree, i.e. the whole problem, not 10 bits. This calibrates the photograph: any rotation pattern on the sheet that is not the canonical form is worth testing at once.

### 3.3 Class-product refuter (idea C)

Run: `far_probes.py classprod 6 3 5 10 6 300` (6x6, colours to random involutions of S_6, 10 boards, 300 random row-scan dives each), output `classprod_n6_F3_C5_S6.txt`; control `classprod_cover.py`, output `classprod_cover.txt`. Measured:

- Soundness: the true prefix was refuted 0 times at every depth on every board.
- Refutation: 0 of about 35,000 wrong prefixes refuted at any depth from 1 to 28 (at most 8 tiles remaining; random dives rarely go deeper), by the class test or by its sign part. The sign part is automatically satisfied by any edge-consistent prefix, since it reduces to colour counts mod 2.
- Control: the class product of k random tile classes covers the whole parity coset of S_6 in 40 percent of draws at k = 2, 93 percent at k = 3, 99 percent at k = 4 and 100 percent from k = 5 on (300 draws each). So the nonabelian test can only bite with three or fewer tiles left, which is the covering-number argument seen on the toy.

Verdict for idea C: dead. Nonabelian finite quotients of the tiling group carry no information before the last handful of cells; only the abelianisation (gap 5) survives, and an infinite quotient is the puzzle itself.

## 4. What I would bet on, and what I did not check

Ranked by payoff over cost, not by odds: B (one photograph, about 0.2 percent, total payoff), A (dead on cost on the toy and on distance by the path-4 count; under 0.01 percent), C (dead by theory and toy). None of them reaches the program's 1 percent prior. The useful output of this memo is the list: twenty fields, each with the reason its tool is blind here, most of them the same reason (a missing measurement, or a test that waits for the last rows).

Not checked: the sources marked unverified; whether the 2010 "sorted before printing" remark refers to the printed sheets or to the numbering only (the raw archive is not on this machine); cohomological k-consistency at k = 2 (idea 27), the one row here with no measurement or theorem behind its kill; colour merging at sizes above 6x6 or with a merge hierarchy deeper than one pair.
