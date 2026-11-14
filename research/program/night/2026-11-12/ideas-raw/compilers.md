# Cut-keyed trace partitioning of the propagation state (field: compilers)

## Stall attacked

Critical-colour boards, where stronger local propagation stops paying.

- `research/program/AGENDA.md`, A22 result: "At the critical-colour rungs R6 and R7 G1 fails (0.80, 0.70) and `bb-mrv` is slower in wall clock (1.64x, 1.85x), so it is not adopted there."
- `research/experiments/hard-ladder/README.md`: "counts at or just below the one-expected-solution point are the hardest, not the most informative."
- `research/program/EXPLORED.md`, arc consistency row: "pruning fades about two cells out".

Structural reading: at the critical count, the strongest propagator available (`bb-mrv`: AC plus piece all-different, hidden singles, MRV) buys almost nothing over `scb`. Both propagators keep one domain per cell, so they are non-relational. Whenever two of a cell's supports lead to different consequences elsewhere, the domain takes their union. At the critical count there are few colours, so each undecided edge has only 5 or 6 possible colours. But each colour choice changes which pieces the all-different leaves free for distant cells. A per-cell union throws that correlation away at every hop, and this loss is the mechanism behind "fades two cells out". The proposal attacks that loss of precision at the join, and leaves branching and ordering alone.

## Field and mechanism

Compilers: static program analysis by abstract interpretation. The mechanism is trace partitioning. The analyzer does not join all abstract states at a control point. It keeps a disjunction of abstract states, indexed by a partitioning key: the value of a chosen expression, or the branch taken at an earlier point. It pushes every partition through all subsequent transfer functions on its own, and joins only at a merge point chosen by a directive. This recovers disjunctive (relational) precision with a non-relational base domain, at a cost linear in the number of partitions rather than exponential.

Translation to the solver:

- **Base domain.** The `bb-mrv` propagation state (cell domains plus piece all-different), unchanged.
- **Partitioning key.** A tuple of k = 2 undecided interior edge colours lying on a cut of the empty region. This is an auxiliary quantity, not a decision variable. Preregistered rule: on the horizontal cut just below the lowest row that holds a placed cell, take the two undecided edges with the smallest colour domains above size 1. That gives at most c^2 partitions (at most 25 at R6 and 36 at R7).
- **Transfer.** Every placement the DFS makes is applied in every live partition, followed by full `bb-mrv` propagation in that partition. A partition that wipes out is dropped for the rest of that subtree; it is never recomputed.
- **Join.** The node's visible domains are the union over live partitions. A value is pruned at the node when it is dead in every partition. The node fails when every partition is dead.
- **Merge directive.** When a placement fixes a key edge, the partitions collapse and a new key is taken by the same rule.

The DFS (MRV order, value order, node counting) is exactly `bb-mrv`'s. Only the propagation state differs.

## Original paper

- X. Rival and L. Mauborgne, "The trace partitioning abstract domain", ACM Transactions on Programming Languages and Systems 29(5), article 26, 2007, DOI 10.1145/1275497.1275501 (https://www.di.ens.fr/~rival/papers/toplas07.pdf).
- First version: L. Mauborgne and X. Rival, "Trace Partitioning in Abstract Interpretation Based Static Analyzers", ESOP 2005, LNCS 3444, DOI 10.1007/978-3-540-31987-0_2.

## Hypothesis

On critical-colour ladder boards, `bb-mrv` loses because it forgets how edge colour interacts with piece availability. If so, conditioning its propagation on a few cut-edge colours, kept alive down the descent, closes a large share of `bb-mrv`'s failing subtrees at their roots, while doing at most c^2 times the propagation work per node. The gain should come from partitioning on the cut, not from more propagation in general. A random choice of key edges should therefore gain clearly less.

## Nearest explored cell and missing mechanism

Nearest cell: arc consistency and Régin all-different (mapped: "pruning fades about two cells out"). The `bb-mrv` and `scb` propagators on the ladder belong to it. A17 (Hall matching) is still a single, non-relational domain per cell.

Missing mechanism: a relational propagation state. Domains are conditioned on the colour of chosen cut edges, the partitions persist and are refined through the descent, and they are joined only at a merge point. No page or ladder experiment has measured singleton-style or partition-style consistency of any kind. A grep of `web/content/research` and the experiment READMEs for singleton, shaving, probing and constructive disjunction finds nothing.

## Cheapest falsifying test (6x6 or 7x7)

**Stage 0: replay, no new search.** On ladder R6 (6x6 c=5) and R7 (7x7 c=6), test seeds 1..30, take `bb-mrv`'s deterministic search tree up to the 1e6 node cap. At every node, run the partitioned propagation (cut key, k = 2). Count the nodes that would never be generated because the partitioned state of an ancestor is already dead. This gives the replay node ratio: surviving nodes divided by `bb-mrv` nodes. It is an upper bound on the node gain with the same branching. Two controls are replayed on the same trees:

- **C1, random key.** The same number of partitions, but the two key edges are drawn at random among undecided interior edges that are not on the cut. This control decides whether the cut is the mechanism.
- **C2, POAC on the cell.** Partition-One Arc-Consistency on the cell MRV branches on next, with domains partitioned by that cell's values (the known CP technique). It is run with a budget of propagation calls matched to the cut-key arm.

**Stage 1 (only if stage 0 passes).** The real search, `bb-mrv` with the partitioned state, on the same rungs at a 1e6 node cap, with CPU time recorded per instance.

## Baseline and control

- **Baseline:** `bb-mrv` (A22, unchanged code) and `scb-gacolor_ac3_random` (the ladder baseline), using the committed rows where the seeds match.
- **Matched controls:** C1 (random key, same partition count) and C2 (POAC at a matched propagation budget). C1 separates the idea that relational precision on the cut matters from the idea that any extra propagation helps. C2 separates trace partitioning on an auxiliary cut quantity from the existing CP technique.

## Kill criterion

**Stage 0.** The idea is killed if any of these holds:

- K1: the median replay node ratio at R7 is above 0.5.
- K2: C1's median replay ratio is within 0.1 of the cut-key arm at both R6 and R7. The mechanism is then plain extra propagation, and the cut plays no part.
- K3: C2 reaches a replay ratio at or below the cut-key arm's at a matched propagation budget. The idea is then a variant of POAC with nothing new.

**Stage 1.** The idea is killed if any of these holds:

- G1 against `bb-mrv` is NO-GAIN at R7 (paired node ratio CI reaches 1).
- The median paired wall-clock ratio at R7 is above 1 against the better of `bb-mrv` and `scb`.
- It solves at most baseline + 2 of 30 at R7 within 1e6 nodes.

## Budget

- Stage 0: under 1 core-hour. Each node replay costs at most 36 `bb-mrv` propagations, and the R7 trees are at most 1e6 nodes on about half the seeds.
- Stage 1: 2 to 3 core-hours.
- CPU only, no AWS.
- 8x8 and above only after both stages pass.

## Closest prior work

- H. Bennaceur and M.-S. Affane, "Partition-k-AC: An Efficient Filtering Technique Combining Domain Partition and Arc Consistency", CP 2001, LNCS 2239, pp. 560-564, DOI 10.1007/3-540-45578-7_39. This is the source of Partition-One AC (POAC).
- Singleton arc consistency: R. Debruyne and C. Bessière, IJCAI 1997.
- The adaptive POAC and SAC line of work: A. Balafrej, C. Bessière and A. Paparrizou, AAAI 2014.

## Why this is not a variant of it

POAC and SAC partition on the values of a decision variable (a cell's piece and rotation). They run as a filtering pass that is recomputed from scratch at each node and joined at once.

This proposal imports the two parts of trace partitioning that those methods lack:

- **Key.** The key is an auxiliary expression, the colours of edges on a cut of the empty region. It has few values exactly at the critical colour count, and it is never branched on.
- **Lifetime.** The partitions are the search state. They persist through successive placements, a dead partition stays dead for the whole subtree, and the merge happens at a directive point (the key edge gets fixed).

Partitioning on a cell refines partitioning on one of its edges. Per key, cell POAC is therefore at least as strong as an edge key, so the claim is not stronger pruning per key. The claim is about where the correlation lives: in the interaction between cut colours and piece availability. Control C2 is in the test to kill the idea if POAC already captures that correlation.
