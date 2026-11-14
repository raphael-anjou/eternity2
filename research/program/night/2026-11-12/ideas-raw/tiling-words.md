# A23. Band consistency by composing tile transducers · proposed

Field: tiling theory and combinatorics on words.

## Stall attacked (with the quote and its file)

- `research/program/EXPLORED.md`, Arc consistency row: "pruning fades about two cells out". `web/content/research/build/dead-ends.mdx` cites the community source: "constraint influence fades out about two cells away" (groups.io message 6208).
- `research/experiments/cut-multiset-pruning/README.md`: "Above the critical colour count the baseline's propagation buys no node reduction on this rung."
- AGENDA A22: at the critical-colour rungs R6 (6x6 c=5) and R7 (7x7 c=6), the extra piece-side propagation in `bb-mrv` fails G1 (0.80, 0.70) and is slower in wall clock.

Why the mechanism hits this stall. Every pruning method on the ladder reasons over binary edge constraints: AC-3, GAColor, the per-cell all-different filter. Arc consistency is exact only on constraint graphs without cycles. The board graph is made of 4-cycles (every 2x2 square), so arc consistency can never tell that a value fails every way around a square. That is a structural reason for the two-cell horizon, not a tuning issue. The mechanism below is exact on every cycle inside a band of k full rows, plus the side borders, at once.

## Field and mechanism

- Field: tiling theory (Wang tiles) and finite automata on words.
- The tiling-theory step: a Wang tile set is a letter-to-letter transducer. A tile with colours (N, E, S, W) is a transition from state W to state E that reads N and writes S. A row of tiles turns the colour word on its top edge into the word on its bottom edge. A band of k rows is the composition of that transducer with itself k times: its states are the k-tuples of vertical-edge colours, and its transitions are vertical stacks of k tiles that match each other. Jeandel and Rao build these composed transducers, simplify them, and test them for emptiness to prove tile sets cannot tile.
- What changes for Eternity II: the board is finite and every piece is used once. Restrict the transducer's alphabet at each cell to that cell's current domain (piece and rotation). Fix the side columns to their border domains. Feed in the band's known top row of edge colours and the bottom-border language where they apply.
- Turning it into a pruner: a forward pass and a backward pass over the band's columns mark which (cell, piece, rotation) values lie on some accepted path. Every other value is removed. That is generalised arc consistency on the whole band, which acyclic reasoning cannot reach.
- Cost per column: at most (c+1)^k states, the k-tuples of vertical-edge colours (c interior colours plus the border colour). That is 36 at k=2 and 216 at k=3 for c=5.

What the band check does not enforce. Piece uniqueness is not enforced inside the band: a piece may label two band cells on the same path. The existing all-different filter still runs alongside. This relaxation is the expected way the idea fails, and the test measures it directly.

## Original paper

- E. Jeandel and M. Rao, "An aperiodic set of 11 Wang tiles", Advances in Combinatorics 2021:1, arXiv:1506.06492. The arXiv listing confirms title, authors and journal. The search did not surface the transducer material itself, so the runner should check the paper's transducer section before writing the PREREG.
- The transducer reading of Wang tiles also appears in J. Kari, "A small aperiodic set of Wang tiles", Discrete Mathematics 160 (1996) 259-264, DOI 10.1016/0012-365X(95)00120-L (from memory, not checked).
- Filtering by forward and backward passes over the layered automaton is the method of G. Pesant, "A regular language membership constraint for finite sequences of variables", CP 2004, LNCS 3258, DOI 10.1007/978-3-540-30201-8_36 (from memory, not checked).
- The tiling paper supplies the 2D-to-word reduction, meaning the composed band transducer. The CP paper supplies the filtering pass. Pesant's method alone only covers one-dimensional sequences.

## Hypothesis

Adding band-consistency filtering with k=2 or k=3 to the ladder's propagating DFS cuts nodes at the critical-colour rungs R6 and R7, where arc consistency saves little. The gain comes from the band's cycles, so k=1 should not reproduce it.

## Nearest explored cell and missing mechanism

- Nearest cells:
  - "Arc consistency, Régin all-different" (mapped, "pruning fades about two cells out").
  - "Pattern databases, macro-tiles, local cluster enumeration" (mapped, dead ends).
- Missing mechanism: exact consistency over a constraint subgraph that contains cycles, computed from the current domains during search.
  - Macro-tiles are precomputed blocks and, measured by Zajc (msg 5883), "a speedup but no domain reduction".
  - The pattern-database entry in `dead-ends.mdx` was measured at the root only, and it says itself: "Patch databases indexed by frontier colors and remaining pieces are untested here." This proposal fills that gap as a domain filter, not as a score bound.

## Cheapest falsifying test (6x6 or 7x7)

- Boards: hard-ladder R6 (6x6 c=5) and R7 (7x7 c=6), test seeds 1..30, cap 1e6 nodes, hard-ladder node rule (one counted `try_assign`).
- Stage 1 (diagnostic, about 30 minutes):
  - Run the baseline DFS and log every node where it later backtracks with no solution below.
  - At each such node, run the k=2 band filter on every 2-row window that still has unassigned cells.
  - Report the share of those dead nodes that the filter proves dead (a domain wipe-out) and the mean share of domain values it removes, for k = 1, 2, 3.
- Stage 2: the baseline with the band filter added as an extra propagator, applied to every k-row window touched by the last placement. Report nodes and CPU time per instance, 3 run seeds where the value order is random.

## Baseline and control

- Baseline: `scb-gacolor_ac3_random`, the ladder DFS at R6 and R7. That is where A22 left `bb-mrv` unadopted. Also report `bb-mrv` with and without the filter.
- Mechanism control: k=1. A one-row band is a path. AC-3 already settles a path's horizontal constraints exactly, so k=1 should be close to inert. Any gain at k>=2 over k=1 then comes from consistency over cycles, not from extra filtering in general.
- Relaxation control: the same k=2 filter built from the full piece set instead of the cells' current domains. This separates the remaining-pieces information from band structure alone.

## Kill criterion

- Stage 1: k=2 proves dead fewer than 5% of the dead nodes the baseline keeps, at both rungs.
- Stage 2: G1 shows no gain against the baseline at both R6 and R7 for the better of k=2 and k=3; or the gain holds in nodes but the median paired CPU ratio is above 2 at both rungs.
- Mechanism kill, even if stage 2 passes: k=1 comes within the k=2 node ratio's CI at both rungs.

## Budget

About 1 session to implement (filter plus a fuzz test against brute-force band enumeration on 2x4 windows). About 3 core-hours to run on CPU, no AWS. Nothing touches 8x8 or 16x16 unless stage 2 passes at both rungs.

## Closest prior work

1. Pesant's regular constraint: the same forward-backward filtering, on one-dimensional sequences.
2. Inside this project:
   - The transfer matrices on `why/entropy-area-law` (strip counting for entropy) and in A5 (exact frame count): both count offline and never run during search.
   - The 2x2 macro-tiles in `dead-ends.mdx`.
3. Dynamic-programming consistency over a tree decomposition (bucket elimination), of which a k-row band is a width-k case.

## Why this is not a variant of it

- Pesant's constraint needs an automaton over a 1D sequence; on its own it has nothing to say about a 2D board. The tiling-theory step is what makes k grid rows into one automaton whose letters are column stacks.
- The transfer matrices and the frame count are static counts over the full piece set. Here the automaton is rebuilt from the current domains at search nodes, and its output is domain reductions.
- Macro-tiles enumerate blocks ahead of time and reduce no domain (Zajc). This filter removes values that arc consistency keeps.
- Generic tree-decomposition consistency is the theory behind the filter, not a measured method on edge-matching. The k=1 control is what makes the cycle claim falsifiable on its own.

## Sources

- [An aperiodic set of 11 Wang tiles (arXiv:1506.06492)](https://arxiv.org/abs/1506.06492v4)
- [PDF](https://arxiv.org/pdf/1506.06492)
