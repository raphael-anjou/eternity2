# Proposal: revocable branching cells via proof-number search

**Summary.** The proposal brings proof-number search from game-tree search into the ladder's propagating DFS. The aim is to let the search change its mind about which cell to branch on in each subtree it has to refute. I checked the paper on Crossref. A web search found no earlier use of proof-number search to choose branching cells in a CSP or SAT solver. That search was quick, so a scout should repeat it before any code is written. I wrote no file, ran no experiment and pushed nothing.

## Stall attacked (with the quote and its file)

The stall is the critical-colour rungs, where extra propagation saves little. `research/program/AGENDA.md`, A22 result:

> "At the critical-colour rungs R6 and R7 G1 fails (0.80, 0.70) and `bb-mrv` is slower in wall clock (1.64x, 1.85x), so it is not adopted there."

`research/experiments/hard-ladder/README.md`:

> "counts at or just below the one-expected-solution point are the hardest, not the most informative."

The stronger propagation that wins at 8x8 c=10 stops paying off at the critical point. Every ladder solver also shares one fixed choice. Each search node branches on one cell, picked by a cheap rule (domain size, border-first or row order), and that choice is never revisited inside the subtree. On a planted board almost all nodes sit in subtrees that must be refuted. A subtree's cost depends heavily on which cell the refutation splits on.

At the critical point, domain sizes after propagation are close together. They therefore predict poorly which split gives the smallest refutation. A16 points the same way: its predicted backdoor cells added only +0.010 AUC over domain size and branched worse than MRV. The mechanism targets this fixed, uninformed cell choice.

## Field and mechanism

The field is game-tree search, and the mechanism is proof-number search run as df-pn with a transposition table.

The CSP is recast as an AND/OR graph whose OR choice for disproof is the branching cell:

- **State S.** S is the set of placements so far, after propagation.
- **Refuting S.** It is enough to show that every value fails on one cell X. The refuter picks X.
- **Proving S.** It is enough to find one working value on any cell.

Two kinds of node follow from that:

- **Split node (S, X).** Its children are the values of X.
  - pn = min over children
  - dn = sum over children
- **State node S.** It keeps up to k candidate splits open: the k smallest-domain cells, ties broken by index.
  - dn(S) = min over its open splits
  - pn(S) = min over its open splits

Best-first selection always descends into the split with the lowest current disproof number. If that split's dn grows past a sibling's, effort moves to the sibling. The cell choice is therefore revocable and is decided by subtrees that have actually been explored, not by a prediction.

The transposition table is keyed on the placement set, which does not depend on order. Splitting on X then Y and on Y then X then share one subtree, so an extra open split only costs the nodes that are new. df-pn's threshold scheme keeps the search depth-first with bounded memory. Initial values are fixed before any run, with no sweep: a leaf's dn is the smallest candidate-cell domain size after propagation, and its pn is 1. Propagation is bb-mrv's, unchanged.

## Original paper

- L. V. Allis, M. van der Meulen, H. J. van den Herik, "Proof-number search", *Artificial Intelligence* 66(1):91-124, 1994, DOI 10.1016/0004-3702(94)90004-3 (checked on Crossref).
- The depth-first version (df-pn) is A. Nagai's PhD thesis, University of Tokyo, 2002. I did not check this one.
- Survey: A. Kishimoto, M. H. M. Winands, M. Müller, J.-T. Saito, "Game-Tree Search Using Proof Numbers: The First Twenty Years", *ICGA Journal* 35(3), 2012, DOI 10.3233/ICG-2012-35302 (checked on Crossref).

## Hypothesis

At the critical colour count, the cell that MRV picks at a refuted node often has a much larger refutation tree than another cell among the k smallest domains. Proof-number search over revocable splits, with the same propagation and node counting as bb-mrv, finds the cheaper refutations as it goes. The cost of the abandoned splits it expands is smaller than that saving.

The result would be fewer nodes than both bb-mrv and scb at R7 (7x7 c=6), where neither of those beats the other.

## Nearest explored cell and missing mechanism

**Nearest cells.**
- "Fixed / dynamic fill orders, comb search" (saturated: "lever on tree size, not on ceiling").
- "No-good learning, shared transposition tables" (mapped).

On the ladder, exact search is the ceiling, so a lever on tree size is the right target there.

**Missing mechanism.** Every ordering in EXPLORED.md and in A16, A21 and A22 picks a single cell per node and keeps it. The transposition tables and no-goods in the mapped cell remember refuted states, but they never move effort between different ways of refuting the same state. Here, effort shifts between competing refutations of one subproblem, driven by disproof numbers that update as the search runs. That is what proof numbers add, and no explored cell has it.

It is also not A12 (ProbCut). ProbCut throws subtrees away on a statistical guess and lost planted solutions. This method cuts nothing: it is complete, and it only reorders where effort goes.

## Cheapest falsifying test (6x6 or 7x7)

**Stage 0: is there headroom?** This step uses no new solver and costs under 2 core-hours.

1. Sample at least 300 refuted nodes from bb-mrv's trees on the ladder test seeds: R6 (6x6 c=5) and R7 at depths 8 to 20.
2. At each node, take the 3 smallest-domain cells. Count the exact refutation tree size under each, using bb-mrv's propagation and MRV below the split, with a cap of 1e6 nodes (capped runs are reported as censored).
3. Compute r = size(MRV's cell) / size(best of the 3).

If r is near 1, no cell choice can help, and stage 1 is not built.

**Stage 1: run the solver.**
- Write df-pn with k=3 on the ladder's `NodeCounter` and `run_all` interface.
- Run R6 and R7 test seeds with the ladder protocol (G1, 1e6-node cap), plus R7 at the 1e7 cap.
- Count every `try_assign` in every split, including splits that were later abandoned. A transposition-table hit counts as zero nodes, and the peak table size is reported.

## Baseline and control

**Baselines.** The better of `bb-mrv` and `scb-gacolor_ac3_random` at each rung. At R7 these are roughly 15/20 (bb-mrv, A16) and 10/30 within 1e6 nodes (scb, A15). Both are rerun on the same seeds.

**Matched control.** The same df-pn code, transposition table, propagation and starting values, with k=1, so the split is always MRV's cell. The only difference left is the revocable cell choice. Without this control, a gain could come from best-first value selection or from transposition sharing instead.

## Kill criterion

- **K0 (stage 0).** Median r ≤ 1.25, and r ≥ 2 at fewer than 20% of sampled nodes, at both R6 and R7. If this fires, stage 1 is not built.
- **K1 (method).** At R7, the median paired node ratio against the better baseline is not ≤ 0.7, or its 95% CI includes 1. The method also dies if it solves fewer R7 instances than the better baseline at the 1e7 cap.
- **K2 (mechanism).** The k=3 arm is not significantly below the k=1 control on paired nodes at R7 (one-sided Wilcoxon p ≥ 0.01). If K1 passes but K2 fires, the gain comes from df-pn itself, not from revocable splits, and is recorded that way.
- **K3 (cost).** A node gain with a median wall-clock ratio above 1 against the better baseline at R7 is recorded as NO-GAIN.

## Budget

- Stage 0: under 2 core-hours.
- Stage 1: about 4 core-hours across R6, R7 and three arms plus control. That includes R7 at 1e7 nodes, where transposition-table memory may reach about 1 GB.
- CPU only. No AWS. No 16x16 until it beats R7.

## Closest prior work

- **Strong branching in MIP** (Applegate, Bixby, Chvátal, Cook, 1995) and **lookahead SAT solvers** (Satz, Li and Anbulagan, 1997). Both probe several candidate variables one level deep, then commit.
- **Impact-based search** (Refalo, CP 2004; already A21 in the agenda) and **dom/wdeg** (Boussemart et al., ECAI 2004). These choose variables from measured impacts or failure counts, then commit at each node.
- **AND/OR search spaces for graphical models** (Dechter and Mateescu, AIJ 2007). Their AND nodes are subproblems that do not interact, not alternative splits.
- **In this repo:** A16 (branching on predicted backdoor cells), A22 bb-mrv (MRV), and the mapped no-good and transposition-table cell.

## Why this is not a variant of it

Each of these chooses the branching cell once per node, before it has seen that cell's subtree, and then builds a single refutation under it. Strong branching and lookahead look ahead one level; impact and dom/wdeg rely on statistics from other parts of the search; MRV uses domain size. Dechter and Mateescu's AND nodes split a problem into independent parts, which the E2 grid almost never has.

Proof-number search keeps several refutations of the same state open and grows whichever is currently cheapest. Effort moves to another split when the explored subtrees show the first is getting expensive. It has no fixed probe depth, unlike ProbCut, whose A12 probes were too short to see anything. Nothing is pruned, and the shared transposition table keeps redundant splits from costing twice. The k=1 control isolates this one change.

The idea has two known risks. Proof-number search is known to use a lot of memory, which K3 and the reported table size watch. And the cell choice may simply not matter at the critical point; K0 tests that before any solver is written.

Sources:
- [DBLP record for Allis et al. 1994](https://dblp.org/rec/journals/ai/AllisMH94)
- [Proof-Number Search, Chessprogramming wiki](https://www.chessprogramming.org/Proof-Number_Search)
- [Kishimoto et al., ICGA Journal 2012](https://content.iospress.com/articles/icga-journal/icg35302)
- [Proof-number search, Wikipedia](https://en.wikipedia.com/wiki/Proof-number_search)
- [Pesant, Quimper, Zanarini, Counting-Based Search (arXiv 1401.4601)](https://arxiv.org/pdf/1401.4601)
- [Adaptive Branching for Constraint Satisfaction Problems (arXiv 1008.0660)](https://arxiv.org/pdf/1008.0660)

Files read:
- `<repo>/research/program/AGENDA.md`
- `<repo>/research/program/EXPLORED.md`
- `<repo>/research/LEDGER.md` (from line 2382)
- `<repo>/research/experiments/hard-ladder/README.md`
- `<repo>/research/experiments/mrv-baseline/README.md`
- `<repo>/web/content/research/build/dead-ends.mdx` (the MCTS section)
