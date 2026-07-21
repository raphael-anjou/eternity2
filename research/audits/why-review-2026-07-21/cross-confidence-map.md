# The confidence map: what the walls rest on

A wall-by-wall reading of the evidence under every structural claim in
`/research/why`. For each named wall it states the strongest form the evidence
actually supports, the tier of that evidence, whether any independent party has
reproduced it (counted only when a page cites the reproduction), and the one
caveat a reader should carry.

## How to read the tiers

- **Proven, external.** A theorem or an exact computation that an outside party
  has stated or replicated, and the page cites that party.
- **Proven, internal.** An exact, deterministic computation done in this
  project, with the code and result committed under `research/topics/`.
- **Measured, single instance.** A real measurement, but taken on the one
  Eternity II board (or one E2-like board), with no replication. True, and not
  a defect on its own, but it is one data point.
- **Conjecture.** The page itself frames the claim as unproven in general.

A page is not marked down for being measured on a single instance. It is marked
down only where it presents a lower tier as a higher one.

## The map

| Wall / claim cluster | Strongest form the evidence supports | Tier | Independent replication cited on the page | The one caveat |
|---|---|---|---|---|
| **Hardness peak** (17 interior colours sit at the phase transition; ~1 expected solution) | The 17.14 colour count follows from the one-expected-solution criterion, and 17 is near the search-hardness peak for framed edge matching | Proven, external | Yes. Brendan Owen's 1947 derivation; Ansotegui, Bejar, Fernandez and Mateu (Constraints 2013, CCIA 2008) confirm the peak. Owen's 2-by-L rectangle count (msg 4909) is a direct measurement | "About one expected solution" is a first-moment average that assumes edge colours are independent; it locates the peak, it does not count solutions |
| **Complex theory / the funnel** (tree width per depth; ~14,702 one-clue solutions, ~1 with five clues; plateau holds ~99% of the work) | A calibrated first-moment estimator of tree width and solution count, right within about a factor of two everywhere it could be checked | Conjecture (page badge), resting on external replication | Yes. Owen's model; McGavin's C port and 10x10 solve (~180 core-years, inside prediction). The estimator is ported line for line and validated against the one-clue table | It is an average, blind to whether counted partial boards are genuinely distinct; use it to rank scan orders and read tree shape, never as a true count or a bound. The plateau depth figures come from one community backtracker on one board |
| **No forced moves** (every interior piece keeps 73 to 137 right-hand partners; zero pieces pinned to one option) | An exact, exhaustive count over the official interior set: no interior piece is ever forced to a single right-hand partner | Proven, internal | No external replication cited; the count is this project's own, committed under `research/topics/no-forced-moves` and reproducible | This counts pairwise right-hand compatibility on the full piece set, not candidate counts inside a live partial board; it is a property of the pieces, not a proof that search never narrows |
| **Forbidden patterns** (99.72% of 2x2 placements can never match; 20 of 289 corner colour-pairs are voids) | Exact exhaustive counts: 38.96% of pairs, 83.26% of L-trominoes, 99.72% of 2x2 squares are infeasible over distinct interior pieces | Proven, internal | The 2x2 counts are this project's committed exact result. The 20 voids are a cited 2008 community observation (msg 5027) | The "forbidden-patch count tracks distance to a solution" reading is a useful heuristic signal, not a proven monotone distance; two boards with equal edge-scores need not order the same way in general |
| **Entropy area law** (matching grammar stays rich to ~80 cells, then distinctness collapses as exp(-alpha n^2), alpha about 0.085) | The per-width entropy density has a well-defined limit (Fekete subadditivity), measured near 0.67; the distinctness fraction of colour-valid patches decays in the area | Measured, single instance, on a proven scaffold | Partial. Fekete's lemma and Shannon entropy are the external theorems that make the limit well defined; the alpha about 0.085 decay and the ~80-cell figure are this project's own fit, not externally replicated | The theorem guarantees a limit exists and is positive; it does not fix its value. The alpha and the 80-cell threshold are a single-instance measurement and an extrapolation past the exactly counted widths |
| **Rigidity** (record boards are locally frozen; no local refill improves them) | On the public record boards, no rearrangement of a board's own pieces inside a one- or two-cell halo closes any mismatch (SAT: UNSAT) | Proven, internal for the SAT halo; the deeper MIP claims are proven for the regions closed but not committed on the site | Yes, and it is the strongest-replicated wall. Millilaw's freeze and SAT-residual tests, reproduced here on the public boards with a positive control; benj39100's GPU-annealing hard core (msg 11902) approaches it from a third method | The committed site evidence is the SAT halo out to radius 2 on five boards. The MIP proof table (halo-1 to halo-4, the dual bound implying board-wide <= 476) is sourced to an external project paper, not to a committed topic result; the general "all boards" statement is explicitly a conjecture |
| **Sigma-cycles / basin-hopping** (moving between record boards is one indivisible cycle of 80 to 154 cells; every partial application scores worse) | On three measured record pairs, the permutation between boards is one or a few large cycles, and no proper sub-cycle improves the score | Measured, single instance (three pairs) | No external replication cited; computed here against McGavin's 469 and two project boards | The page states plainly that three pairs is a small sample and the "every sub-cycle is worse" property is not proven in general. Read it as a strong observation on the boards to hand, not a theorem |
| **Mismatch geometry** (a near-perfect board packs all its errors into one five-row band, set by scan direction) | On the measured record and project boards, residual mismatches cluster in one row-band that flips with build direction; a hole-count objective lands leftovers in the same band | Measured, single instance | Partial. The scan-direction flip is this project's own reading; Verhaard's seven-hole board and Zamofing's "residual always lands in the top band" (msg 11901) are cited as independent same-direction observations | It is a pattern across a handful of boards with a mechanistic story, not a proof. One of its supporting citations (msg 11901) sits beyond the local archive's coverage and could not be checked here |
| **Piece theft** (fill in scan order and each cell demands a scarce north-west pair; 47 demands have a single supplier) | An exact count of (north, west) demands over the interior set: most have one to three suppliers, and a specific number have exactly one | Proven, internal for the counts; measured for the "where solvers die" mechanism | No external replication; Regin 1994 is cited as the all-different theory that names the mechanism, not as a replication of the E2 numbers | The scarce-supplier counts are exact and solid; the claim that this is *where* real solvers die is a mechanism illustrated on the instance, not a measured failure-rate across solvers |
| **Hint geometry** (18 scattered hints beat 80-plus contiguous ones; 99% of search sits past depth 132) | On one 16x16 E2-like puzzle, scattered hints solved it in minutes where piled contiguous rows did not, because the work lives in the back half of the fill | Measured, single instance (a community result on an E2-like board, not the official puzzle) | Yes, and the page attributes it cleanly. McGavin's 18-hint solve and 41-billion-node tree (msg 11746); Joe's depth statistics (msg 11725) | This is explicitly not the official puzzle, whose five fixed clues differ. The depth figures are one backtracker's 1-billion-iteration sample. The "80 or more" contiguous-hint figure is a comparison point, not a tight threshold |
| **Border balance / NS-1** (border and interior show the same colour multiset across the seam; deficit > 0 proves a board is dead) | An exact necessary condition: the four known full solutions satisfy it; a positive deficit certifies infeasibility | Proven, external (the condition), measured for the pruning yield | Yes for the condition. Cited to 2007-2022 community statements (angwin_uk 2073, mjqxxxx 2098, Hopfer 10754/10757) | The condition is necessary, never sufficient: deficit zero proves nothing, piece swaps slip past it, and on 469-class boards over 85% of remaining errors are interior-to-interior where it never looks. The 10 to 28% prune yield is a single-instance measurement |
| **Rare-colour geography** (five colours live only on the frame, 24 edges each, zero in the interior) | An exact count over the official set: the five border colours never appear on an interior edge | Proven, internal | No external replication of the count; the 17+5 design intent is cited to Owen (msg 1947) | The page is careful, and correct, to say the split is structural (grey rim, separate colour pools), not a scarcity trick; the "rare" label is an artefact of fewer border edges, and the colours could be relabelled freely |
| **Design recipe** (compact board, no symmetric or duplicate pieces, split palette, flat frequencies, one expected solution) | The community reconstructed a coherent hardest-puzzle recipe, ingredient by ingredient, and each ingredient is sourced to a launch-year post | Conjecture (page badge), each ingredient externally sourced | Yes, densely. Owen's derivations and measurements (1947, 1667, 2076, 2164), the design-space census (8014 to 8034), the provenance chain to Selby and Riordan | It is a reconstruction of design intent, not a statement the designers published; the page badges it conjectured and attributes every ingredient. The generation account (msg 4177) is second-hand and flagged as such |
| **Complexity framing** (edge matching is NP-complete as a family; one fixed board is a constant, not a problem) | Edge matching is NP-complete; a single fixed instance has no complexity class; E2's real difficulty is empirical over a ~10^557 space | Proven, external | Yes. Demaine and Demaine 2007 for NP-completeness; Ansotegui et al. for the empirical case | The category distinction is exactly right and load-bearing: the NP-completeness result caps what general solvers can promise, it says nothing about this board. The 10^557 figure is an arrangement count, not a hardness proof |
| **Prune versus speed** (a per-level prune compounds; a speedup is a constant divisor; E2 leaves almost nothing local to prune) | The compounding argument is exact arithmetic; on E2, legal pruning often costs more than the subtree it removes | Proven (the arithmetic), measured (the "pruning does not pay" verdict) | Partial. The principle is exact and self-contained; the community verdict is cited to McGavin (msg 11848) and 95A31 (msg 11856) | The demo tree numbers are illustrative, not a measurement of any real solver, and the page says so. Two of its supporting citations (msg 11848, 11856) sit beyond the local archive's coverage and could not be checked here |

## Cross-cutting notes for the reader

**The four "load-bearing" walls are not equally hard evidence.** The section's
own framing (no forced moves, hardness peak, area law, rigidity) mixes tiers.
No forced moves and forbidden patterns are exact internal counts. The hardness
peak is externally corroborated. The area law rests on an external existence
theorem but a single-instance measured value. Rigidity is the best-replicated
wall by method diversity (SAT, MIP, annealing), yet only its SAT-halo layer is
committed on the site.

**Replication is real but concentrated on rigidity.** Only rigidity carries
three independent methods reaching the same verdict. The remaining walls are
either exact single-project computations (forbidden patterns, no forced moves,
rare colours, piece theft) or single-instance measurements (sigma-cycles,
mismatch geometry, entropy area law). That is a legitimate evidence base, but a
reader should not read "corroborated by the literature" as meaning every wall
has been independently reproduced. It means the phase transition and the
missing gradient have.

**Four community citations sit beyond the archive snapshot.** The messages at
11848, 11856, 11901 and 11902 are past the last message this project's local
community archive contains (its highest message is 11823). They may be genuine
later posts, but they cannot be checked against the archive, so any wall that
leans on them (rigidity's annealing corroboration, mismatch geometry's
same-band citation, prune-versus-speed's community verdict) rests on a source a
reader cannot yet verify locally.

**Single-instance is the section's honest default.** Most of these results are
measured on the one Eternity II board, which is the only board that matters, so
this is appropriate. The map flags it not as a fault but so a reader knows which
claims would survive a different instance (the exact piece-set counts) and which
are statements about this board's particular record set (the cycle sizes, the
mismatch bands, the frozen cores).
