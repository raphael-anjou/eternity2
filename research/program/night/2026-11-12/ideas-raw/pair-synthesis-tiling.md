# Boundary-word lemmas: conflict-learned, translation-invariant pocket refutations

Pair: program synthesis x tiling theory / combinatorics on words. Proposal only; nothing run.

## Stall attacked (with the quote and its file)

The critical-colour rungs, where propagation saves little and the two strongest complete DFS solvers tie.

- `research/program/AGENDA.md`, A22 result: "At the critical-colour rungs R6 and R7 G1 fails (0.80, 0.70) and `bb-mrv` is slower in wall clock (1.64x, 1.85x), so it is not adopted there."
- `research/experiments/hard-ladder/README.md`: "counts at or just below the one-expected-solution point are the hardest, not the most informative."
- `web/content/research/build/dead-ends.mdx` (perfect rows): "the locally best early rows consume exactly the pieces whose edge colors the later rows need", and (pattern databases): "Patch databases indexed by frontier colors and remaining pieces are untested here."
- `research/program/EXPLORED.md`: "No-good learning, shared transposition tables | mapped | no-goods outrun memory at 16x16".

Structural reading. The propagation that wins above the critical point (AC-3, piece all-different, Hall matching, MRV) sees one cell at a time plus global piece counts. It cannot see that a small empty pocket (for example a 2 x 3 hole, or the 2-row strip under a stretch of frontier) bounded by particular colours cannot be filled from what is left in the pool. That is the perfect-rows starvation failure at local scale. At the critical colour count, where domains stay wide and nothing is forced, DFS refutes such a pocket only after an expensive subtree. The hypothesis is that it then refutes the same pocket again under other upstream histories, at other positions and rotations. A placement-level no-good cannot carry that refutation across, because it names cells and pieces, and that is why no-goods outran memory.

## Fields and mechanism (what each field contributes)

**Mechanism.** When a DFS subtree of at least T nodes is refuted, try to explain the failure as a *boundary lemma* ℓ = (F, u, Q):

- F is a polyomino of empty interior cells, keyed up to translation and the four rotations.
- u is the boundary word of F, read clockwise. Each letter is a colour or a hole ◇ (an unconstrained edge).
- Q is a downward-closed pool predicate: for each piece type s, count_s(pool) ≤ m_s, with m_s = ∞ for types the conflict does not need.

ℓ asserts that no placement of distinct pieces fills F so that it matches every non-hole letter of u, for any pool that satisfies Q. A lemma is accepted only after an exact solve of F against the largest pool Q allows (m_s copies of each kept type and unlimited copies of each dropped type) finds no filling. Fewer pieces can only remove fillings, so one exact check proves ℓ for every pool satisfying Q. Q stays true as the pool shrinks along a branch.

**Generalisation (from program synthesis).** Starting from the failed node's own region and pool, minimise by deletion with the exact check as oracle:
1. turn boundary letters into holes;
2. raise piece-type counts to ∞;
3. shrink F.

**Use.** At every later node, match the stored lemmas against every empty interior translate or rotation of F. Matching is partial-word matching of u against the current boundary colours, plus a count test of the pool against Q. A match prunes the node.

**Program synthesis contributes conflict-driven lemma learning with "equivalence modulo conflict"** (Neo). When a partial program fails, Neo extracts a minimal unsatisfiable core over the specifications of its components. It then learns a lemma that prunes every partial program whose components are equivalent with respect to that core, not only the one that failed. Translated here, a refutation becomes a lemma over the colours and piece types the conflict actually used, minimised by deletion against an exact oracle. Without this, a learned no-good is a set of (cell, piece, rotation) literals. That is the mapped no-good cell, and it cannot transfer between branches that used different pieces upstream.

**Tiling theory and combinatorics on words contribute the lemma's language and why it transfers.** In the Conway–Lagarias view, whether a region can be tiled depends only on its boundary word. Wang-style local matching rules are invariant under translation and rotation. So a refutation stated as (region shape, boundary word) holds wherever that shape and word recur. Partial words (words with holes, Berstel–Boasson) are the right object for a boundary in which some edges are open, and give the matching problem used at lookup. Without this, Neo's lemmas stay attached to fixed holes of one sketch, which here means fixed board positions: one lemma per position, the memory blow-up again.

**Why neither alone gives it.** Boundary-word invariants alone (Conway–Lagarias group invariants, the site's flux invariants) are fixed and hold for all tilings. They ignore the depleted pool, and on this site they gave no measurable gain. Conflict-driven learning alone yields position-bound, piece-literal lemmas. Only the combination gives a lemma that is learned from a real conflict, proved exactly, keyed by a boundary partial word, invariant under translation and rotation, and still valid as the pool depletes.

## Original papers

- Program synthesis: Yu Feng, Ruben Martins, Osbert Bastani, Işıl Dillig, "Program synthesis using conflict-driven learning", PLDI 2018, pp. 420-435, DOI 10.1145/3192366.3192382 (arXiv 1711.08029).
- Tiling theory: J. H. Conway and J. C. Lagarias, "Tiling with polyominoes and combinatorial group theory", Journal of Combinatorial Theory, Series A 53(2), 1990, pp. 183-208, DOI 10.1016/0097-3165(90)90057-4.
- Supporting, combinatorics on words (holes): J. Berstel and L. Boasson, "Partial words and a theorem of Fine and Wilf", Theoretical Computer Science 218(1), 1999, pp. 135-141, DOI 10.1016/S0304-3975(98)00255-2.

## Hypothesis

On critical-colour planted boards (ladder R6 6x6 c=5, R7 7x7 c=6), a large share of the expensive refutations in propagating DFS have a local core: a pocket of at most 2n interior cells whose generalised boundary lemma is exactly infeasible. These lemmas recur at other positions, rotations and branches of the same run. Pruning with them cuts the search tree of the best ladder DFS at R7 well below its parent, where piece-side propagation (A22) and Hall matching (A17) gave no gain.

## Nearest explored cell and missing mechanism

- **No-good learning / shared transposition tables** (mapped; "no-goods outrun memory at 16x16"). Missing mechanism: the lemma key is a partial boundary word of a region shape, invariant under translation and rotation, not a set of (cell, piece) literals. Its pool condition is a minimised downward-closed count predicate, so one lemma covers many positions and upstream histories.
- **Pattern databases, macro-tiles, local cluster enumeration** (mapped, dead ends: root bound trivial, cluster counts blow up). Missing mechanism: no enumeration. Lemmas are induced only from refutations the search actually paid for, then generalised and proved exactly. The table holds what the search needed, not every pattern.
- **Forbidden patterns** (static 2x2 density over the full piece set). Missing mechanism: these lemmas depend on the depleted pool, and they are learned per run.
- **Parked "abstraction refinement on piece distinctness"**: not this. Distinctness is never relaxed in the search. Unlimited copies appear only inside the soundness check of a dropped type, which proves a stronger statement.

## Cheapest falsifying test (6x6 or 7x7)

**Stage 1, diagnostic (no pruning; the solver only logs).** Ladder R6 and R7 primary test seeds 1..30, cap 1e6 nodes, parents `bb-mrv` and `scb-gacolor_ac3_random` (unchanged node rule from `hard-ladder/PROTOCOL.md`). At each refuted subtree of at least T = 200 nodes, run the extraction:
- F is grown, connected and interior, from the cells wiped out most often in that subtree, up to 2n cells.
- The exact check is a small DLX/DFS over F.

Record:
- (a) local-core rate: share of expensive refutations that yield a verified lemma;
- (b) reuse: later nodes in the same run where some lemma would have matched at a different position, rotation or branch than its origin, and the subtree size the parent then spent under them (counterfactual node savings);
- (c) soundness: no lemma may ever match a node on the planted-solution path. One such match is a bug, and the run stops.

**Stage 2 (only if stage 1 passes).** Lemma pruning switched on in both parents on R7 seeds 1..30 and R6 seeds 1..30. Extraction and matching are counted in wall clock, and every lemma check runs under its own node budget.

## Baseline and control

- **Baseline.** The parent solvers unchanged: `scb-gacolor_ac3_random` (ladder baseline, 16/30 at R7) and `bb-mrv` (A22). G1 against the better of the two per rung.
- **Control C1 (separates the mechanism from plain no-good learning).** The same arm with generalisation off. A lemma keeps its origin position, rotation 0, no holes, and Q = "pool ⊆ origin pool". This is a placement-level pocket no-good.
- **Control C2 (separates learning from a stronger per-node check).** No learning. At every node, the exact infeasibility check runs on the same region shapes around the MRV cell, at equal CPU.

## Kill criterion

- **Stage 1 kill:** median local-core rate below 10% at R7, or median projected node saving below 30% per instance at R7, or any soundness violation.
- **Stage 2 kill:** G1 NO-GAIN against the better parent at R7, or the median wall ratio against that parent above 1 at R7.
- **Mechanism kill:** C1 comes within a paired node ratio CI that overlaps the full arm's, or C2 matches the full arm in solves and nodes at equal CPU. In either case the gain is not from shift-invariant, pool-generalised lemmas.

## Budget

- Stage 1: about 4 core-hours (120 parent runs at R6/R7 of seconds each, extraction overhead about 10x).
- Stage 2: about 4 core-hours (2 parents x 3 arms x 60 instances, 1e6 nodes).
- CPU only, no AWS. One session per stage, judged by a separate session.

## Closest prior work

1. Neo (Feng et al., PLDI 2018): conflict-driven lemmas generalised by equivalence modulo conflict, inside one program sketch with fixed holes.
2. No-good learning and lazy clause generation in CP and SAT, including the mapped no-good page and CDCL on edge-colour SAT encodings (`build/exact/sat-csp-encodings`, `build/reduce/nogood-learning`). Learned clauses there are tied to specific variables, which means specific cells.
3. Conway–Lagarias boundary-word tiling invariants and their analogue on this site (flux invariants): fixed algebraic certificates over boundary words.
4. Pattern databases (dead-ends page): enumerated patch bounds, measured only at the root.

## Why this is not a variant of it

- Against no-good learning and CDCL: their lemmas name variables, so a refutation at column 3 says nothing at column 5 or after a 90° turn. Here the key is (shape, boundary partial word) and each lemma applies at every translate and rotation. The pool condition is a minimised count predicate, not the set of pieces placed upstream. Control C1 isolates exactly this difference.
- Against Neo: Neo's equivalence is over interchangeable components at a fixed sketch position. Transfer across positions and rotations comes from the tiling side (boundary-word invariance of local matching rules), which Neo does not have.
- Against Conway–Lagarias and flux invariants: those are necessary conditions true for every tiling and blind to the pool. These lemmas are learned per run, depend on the depleted pool, and are proved by exact search.
- Against pattern databases and cluster enumeration: nothing is enumerated in advance. The table is grown only from refutations the search paid for, so it cannot blow up the way 3x3 cluster lists did. Control C2 checks that the gain is reuse of learned refutations, not a stronger per-node check bought with CPU.
