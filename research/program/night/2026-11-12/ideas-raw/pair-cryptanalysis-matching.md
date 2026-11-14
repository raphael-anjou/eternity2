# Score-band key counting on an auction-priced transfer graph

Pair: cryptanalysis x auction and matching theory. Proposal only, nothing run.

## Stall attacked

Critical-colour boards where local propagation saves little, and the censoring
above 8x8. From `research/experiments/mrv-baseline/JUDGE.md`: "Adoption is
supported at the R8 rung only; at R6 and R7 bb-mrv fails G1 and is slower in
wall clock, and Q2 is uninformative above n=8 because every pair is censored."
From AGENDA A17 (Hall matching, the matching-based pruning already tried): "6x6
c=5: GAIN, paired node ratio 0.900 ... 7x7 c=6 and 8x8 c=10: NO-GAIN." The
dead-ends page gives the structural reason: constraint influence "seems to fade
out about two cells away" (msg 6208).

The mechanism attacks the locality of that inference. Every pruning rule on the
ladder (AC-3, GAColor, piece all-different, Hall) reasons over a neighbourhood
or over the cell-piece bipartite graph with edges relaxed. None of them sees,
at a node, whether the edge constraints of the whole remaining board can be met
with the pieces actually left. The proposal keeps every edge constraint exact
over the whole board, relaxes only use-once, and handles use-once with prices
and an exact count of how many relaxed boards outrank the true one.

## Fields and mechanism

**Object.** A broken-profile transfer graph G over cells in row-major order.
State = the frontier colours (n south edges of the filled band plus the west
colour of the current cell); arc = a (piece, rotation) that fits the state. A
source-to-sink path is an edge-valid board that may reuse pieces ("with
replacement"). Size: about 5^6 x 5 states per layer at 6x6 c=5, about 6^7 x 6
at 7x7 c=6, both fit in memory.

**Auction and matching theory contributes the prices.** Persons = cells,
objects = pieces. Cell i values piece j at minus its priced max-marginal
M_ij(p), the cost of the cheapest path through G that puts j at i (forward plus
backward min-sum on G). Bertsekas bidding: a cell whose best piece is also
another cell's best raises that price by (second-best minus best) + eps, with
eps-scaling; stop when best responses form a perfect matching or eps bottoms
out. Uses the scrambled puzzle only.

Why prices need this and not a dual-bound method. With prices p, every
piece-distinct board costs exactly P = sum_j p_j, whatever the board. The
Lagrangian dual of a feasible exact-use problem is degenerate: p = 0 already
attains the optimum L = 0, so maximising the dual bound (subgradient, the
notebook's tensor-network patch) gives no reason to move prices at all. What
matters is the lower tail: how many with-replacement boards cost at most P. The
auction acts on max-marginals, i.e. on the cheapest paths, exactly where
reusing a popular piece makes a relaxed board outrank the true one.

**Cryptanalysis contributes exact rank and band enumeration.** Martin,
O'Connell, Oswald and Stam turn key enumeration after a side-channel attack
into path counting on a layered graph whose nodes carry the partial score: an
exact DP gives the number of keys in any score band (the rank of the true key)
before any enumeration, and keys in a band are enumerated by descending only
into branches whose counted band completions are nonzero. Here the layers are
cells and the node is (frontier state, integer partial cost), which generalises
their independent subkeys to dependent ones through the transfer state.
Outputs: N_< and N_= (with-replacement boards of cost below and equal to P),
which bracket the rank of every true solution, and a complete enumerator of the
band cost <= P that never enters a subtree without a relaxed completion in the
band, with an early abort on piece reuse as the verification oracle. Complete:
every distinct solution has cost P, so none is ever cut.

**Why neither alone.** Band counting without prices (p = 0): every
with-replacement board ties at P, about 1e13 at 6x6 c=5 by a first-moment count
against about one distinct solution, so the rank is astronomically worse than
DFS. Prices without the band machinery: the relaxed argmin is not a distinct
board and decoding it is the notebook's Lagrangian route, where message-passing
marginals did not converge at 4x4 and exact contraction solved 6x6 in five
minutes without a search-size guarantee. The price vector only has value as a
score that ranks the true key high; the band counter only has value when the
score is informative, and with no leakage (A1 found no pair-statistic signal
beyond colour rarity) the score has to be built from the constraint structure,
which is what the auction does.

**Scaling prediction (derivation, not a measurement).** The reduction factor in
the band from prices behaves like exp(-n^2 K / 2), where K is the mean squared
deviation of piece usage in relaxed boards, against an excess of relaxed over
distinct boards of order e^(n^2). The mechanism wins iff K is large enough,
and the gain then grows with board size. Stage 0 measures K.

## Original papers

- Cryptanalysis: D. P. Martin, J. F. O'Connell, E. Oswald, M. Stam, "Counting
  Keys in Parallel After a Side Channel Attack", ASIACRYPT 2015, LNCS 9453,
  pp. 313-337, DOI 10.1007/978-3-662-48800-3_13 (IACR ePrint 2015/689).
- Auction and matching: D. P. Bertsekas, "The auction algorithm: A distributed
  relaxation method for the assignment problem", Annals of Operations Research
  14 (1988), pp. 105-123, DOI 10.1007/BF02186476.

## Hypothesis

On ladder rung R7 (7x7 c=6), auction prices cut the priced rank of the planted
solution, log10(N_< + N_=), by at least two orders of magnitude against zero
prices and by more than mean-usage or subgradient prices, and the complete band
enumerator reaches the first solution in fewer nodes than the better of
`scb-gacolor_ac3_random` and `bb-mrv`.

## Nearest explored cell and missing mechanism

- Tensor-network counting (dead end) with its per-piece Lagrangian amendment,
  and LP/ILP relaxations (mapped, 420 to 440, ILP wall at 8x8). Missing: an
  exact, pre-search count of the relaxed boards that outrank the true board,
  plus a complete enumerator of that band. Prices are chosen to thin the lower
  tail, not to maximise a dual bound that is degenerate at p = 0.
- A17 Hall pruning and A9 Murty k-best: both act on the cell-piece bipartite
  graph with edge constraints relaxed. Here the edge constraints are exact over
  the whole board and only use-once is relaxed.
- Meet in the middle (BANDSAW): no lists, no boundary hash join, no second
  half. One graph, memory = the transfer table and a cost window per state.

## Cheapest falsifying test (6x6 or 7x7)

Ladder instances, test seeds 1..30, R6 (6x6 c=5) and R7 (7x7 c=6); 5x5 at
Owen's count as a third point for K only.

- Stage 0 (no search). For each instance and four price arms (zero, mean-usage
  p_j = mu_j - 1 from sum-product counts on G, subgradient Lagrangian as in the
  notebook, auction), compute N_< and N_= exactly by a cost-window DP (prices
  scaled to integers; a state is dropped when its forward min plus backward
  min exceeds P; window cap 256 units, states over the cap reported as
  censored lower bounds). Sanity check: the planted board's priced cost equals
  P. Report K per size.
- Stage 1 (only if stage 0 passes). Run the band enumerator under auction
  prices with the ladder `NodeCounter` (one node per arc stepped into), 1e6
  node cap, against `scb-gacolor_ac3_random` and `bb-mrv` on the same seeds.
  Wall clock includes the DP and the auction.

## Baseline and control

- Search baseline: the ladder DFS `scb-gacolor_ac3_random` and `bb-mrv`
  (the better per rung), PROTOCOL.md G1 on nodes, plus wall clock.
- Controls isolating each field: zero prices (cryptanalysis alone); mean-usage
  prices and subgradient prices (prices without the auction's lower-tail
  bidding); the band enumerator with p = 0 is the relaxation-only control.

## Kill criterion

- K1 (mechanism, stage 0): at R7 the median paired difference
  log10 N_<=(auction) - log10 N_<=(zero) is above -2.
- K2 (the pairing): auction prices do not beat both mean-usage and subgradient
  prices by at least 0.5 in median paired log10 N_<= at R7 (Wilcoxon p < 0.01).
  If K2 fires the idea is recorded as not needing the auction and closed as a
  two-field proposal.
- K3 (method, stage 1): G1 fails at R7 against the better baseline, or solves
  at 1e6 nodes are not above that baseline's, or the median paired wall ratio
  is above 1.
- Report regardless: K at n = 5, 6, 7 and whether the measured reduction tracks
  exp(-n^2 K / 2).

## Budget

Stage 0: about 2 core-hours (6x6 seconds per instance, 7x7 minutes per
instance per arm, 4 arms, 30 seeds). Stage 1: about 2 core-hours at 1e6 nodes
per run. RAM: the 7x7 window DP needs a few GiB per layer pair; check before
launch. CPU only, no AWS.

## Closest prior work

The project's notebook entry in `build/dead-ends` (Tensor-network counting,
"one amendment"): a per-piece Lagrangian penalty on top of exact contraction
solved generated 4x4 in one second and 6x6 in five minutes; with
message-passing marginals instead, the same penalty did not converge at 4x4.
Outside the project, Martin et al. 2015 count and enumerate keys over
independent subkeys with scores from measured leakage.

## Why this is not a variant of it

The notebook decodes a board from priced marginals and has no statement of how
much search the decoding needs; its prices maximise a penalised fit, which for
a feasible instance is already optimal at zero. This proposal never decodes
marginals: it computes, before search, the exact number of relaxed boards that
rank at or ahead of every true solution, enumerates that band completely, and
chooses prices to shrink that number. Martin et al. need independent subkeys
and an external leakage score; here subkeys are coupled through the transfer
state and the score is manufactured by an auction from the puzzle alone. The
subgradient and mean-usage arms are the notebook-style and statistics-style
price controls, so a variant result would show up as K2.
