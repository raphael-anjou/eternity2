# Gap 3: the shape of the tree, not its mean

> **Status (2026-11-13):** independently re-checked, see `notes/gap-3-shape-of-the-tree-recheck.md`.
> **Re-check's rulings:**
> - Verdict "empty at 10 bits": stands with a narrower scope, and for a different reason than the memo offers. Defensible scope: restart and portfolio schemes over value orders and the row and column scans, blind to the solution, on boards distributed like the painted ones. Not covered: other static orders, dynamic orders with propagation, restarts that keep learned information. What carries it: at any fixed cutoff the mean over cases of 2^gain is about 1/2, so a blind mixture gains g bits with probability at most 2^-(g+1) on the ensemble mean, about 2^-11 for 10 bits.
> - Counting bound P(best gain >= g) <= 2^-(g+1) "under exchangeable siblings": unproven for the real tree; the model behind it is rejected by the memo's own data (median 22 against mean 3.0e5 at depth 96). Replaced by the size-bias bound above.
> - "The share of cases reaching g bits falls like 2^-g, as a counting argument says it must": wrong as worded. The argument gives 2^-(g+1); the counts sit above it; nothing says "must".
> - F2: stands, but "the largest restart gain is G" needs a narrower reading, the two forms bracket the gain 2.3 bits apart in the median.
> - "A portfolio never beats its best member, and running blind costs up to log2 K bits": stands, but the cost is an upper bound that is not paid in full.
> - M-a (depth 96, mean 3.0e5, median 22, top 1% holding 61%): stands with error bars the memo did not give; the median reproduces, the mean and top share do not to two digits. "Under 10^16 nodes" is a wrong number, the level profile gives 10^18.4.
> - M-c table (best gain median 0.0, maximum 4.5): corrected to exact median 0.5, maximum 4.8.
> - "Restarting loses a median of 7.7 bits at 10^15 nodes": 7.7 is a bound, the exact median is 6.7 to 7.1, and it is not a cost of restarting.
> - Transfer, "two of four descriptors fall just outside" the painted range: wrong, sampling noise; on two fresh seeds all four fall inside.
> - Odds: 0.1% for 10 bits stands inside the scope above; "under one in a million" for 56 bits is a guess with no support. "The 4.5 bit maximum is more likely too high than too low": not confirmed, stable within 0.3 bits across seeds.
> **Read with:** `notes/gap-3-shape-of-the-tree-recheck.md`.

Desk study of 2026-11-13. Draft, not independently checked. Grades: **measured** (script in `gap-3/`, today, not independently assessed), **derived** (algebra here), **cited** (opened today unless marked), **guessed**. Painted 16x16 boards and the 5x5 and 6x6 boards are toys; nothing on them is Eternity II progress.

**Verdict: the gap is empty at the 10 bit level.** The tree is far from balanced, but its imbalance is of the kind restarts cannot use. On 24 painted full-size boards and scan directions the best restart cutoff gains a median of 0.0 bits and at most 4.5; the share of cases reaching g bits falls like 2^-g, as a counting argument says it must. At the run length a 10^15-node run affords (10^15 nodes) restarting loses a median of 7.7 bits. No mechanism is proposed.

## 1. The gap, and what the evidence covered

The question is whether the time to the first solution, over randomised value orders, scan directions, board symmetries and relabellings, has a lower tail fat enough for restarts or a portfolio to beat the mean by many bits.

What existed, and its reach:

- **Survey, monte-carlo B1** ("the tree is balanced, restarts lose 1 bit"). Argued from level means. The derivations second analysis ruled it not established (W9): a mean says nothing about the dispersion of sibling subtrees along the planted path.
- **54 toy boards** (second analysis): best cutoff gains a median of 0.00 bits. Exact, but 5x5 and 6x6, one direction, trees of 10^3 to 10^6.
- **Track 10 at `dc71ad56`**. The per-level record holds the mean, variance and zero share of the number of children of population members, nothing about subtree sizes. It shows that branching is close to Poisson from the second row on (variance 1.94 against mean 2.06 at depth 64; zero share 13% rising to 48% at depth 176). Measured, independently assessed.
- **D3**: first solution after 0.09 to 0.94 of the tree. Read in its README: five boards (ten rows, pairs of pin counts on the same board), one fixed value order each. Compatible with a uniform position; too few for more.
- **D4**: the correlation between scan directions (0.88 opposite, about 0.1 adjacent) concerns a census statistic of the planted prefix, not search time. It bears on tilt pruning, not on restarts.

Not covered before today: subtree sizes inside a level, where the planted prefix sits among them, and the restart gain at full size.

## 2. Literature

| Source | What it gives | Grade |
|---|---|---|
| Chen, Gomes, Selman, CP 2001, https://www.cs.cornell.edu/gomes/pdf/2001_chen_cp_formal.pdf (text extracted and read) | Balanced tree: run time uniform, restarts useless. The proof rests on: "we assume that the leaf is located somewhere uniformly at random on the fringe of the tree". Imbalanced tree: heavy tail when a wrong branch leads to a subtree of size b^i with probability p^i and p > 1/b^2. Heavy tails need "a reasonable probability mass for finding a solution early" | cited, theorem |
| Gomes, Selman, Crato, Kautz, J. Autom. Reasoning 24 (2000), https://www.cs.cornell.edu/gomes/pdf/2000_gomes_jar_phenomena.pdf (text extracted and read) | Heavy tails on solvable instances, including one critically constrained quasigroup instance, under randomised variable and value choice with propagation. Long runs are "an exponentially large subtree containing no solutions" entered by "unfortunate choices early on". None found on unsolvable instances | cited, experiment |
| Chatterjee, Diaconis, arXiv:1511.01437 (abstract opened) | A sample of about exp(D(nu‖mu)) is necessary and sufficient for importance sampling | cited, theorem |
| Krzakala, Zdeborová, arXiv:0901.2130 (abstract opened) | Planted and random ensembles share "many of the properties" for a class of problems | cited; edge matching not shown to be in the class |
| Luby, Sinclair, Zuckerman 1993 | Optimal fixed cutoff l(t) = E[min(T,t)]/q(t) | not reopened; confirmed in `surveys/CITATIONS-recheck.md`. Kraft and Ville inequalities: textbook, not opened |

The two Cornell papers settle what to measure: restarts pay when the solution is found early with more probability than its share of the work, and do not pay, whatever the imbalance, when it is located in proportion to work.

## 3. The result: an identity, a counting bound, and three measurements

Fix a cell order. Let T be its tree, D = 251 the free depth, and for each depth j let the planted prefix have siblings with subtree sizes s(j,1), s(j,2), and so on.

**F1 (derived, exact for every shape).** Under a random value order each sibling precedes the planted child with probability one half, so E[time to solution] = D + (T - D)/2. Shape cannot move the mean.

**F2 (derived).** A run cut at t nodes succeeds only if no sibling larger than t precedes the planted child at any depth. With n_j(t) the number of siblings larger than t at depth j:

q(t) <= prod_j 1/(1 + n_j(t)), and gain(t) <= log2(T/(2t)) - sum_j log2(1 + n_j(t)) - log2(1 - q).

The gain is achieved, without the last term, when t is replaced by D plus the sum of all siblings of size at most t. So **the largest restart gain on an instance is G = max over t of [log2(T/2t) - sum_j log2(1 + n_j(t))]**. The product is the planted path's inverse probability under a sequential sampler that counts only large children, the quantity the plan asked for.

**F3 (derived).** For a scheme that draws a level-d prefix from a solution-free law pi, searches below it and restarts when the subtree is exhausted or S* nodes are spent (S* the planted prefix's subtree): gain = log2(N_d pi(x*)) + log2(E_U[S] / E_pi[min(S, S*)]) - 1. With pi uniform the first term is zero and the gain is at most log2(1/Q_d) - 1, where **Q_d is the share of the level's work that sits in subtrees no larger than the planted one**.

**Counting bound (derived).** The weights 1/prod(1 + n_j) form a probability over the large nodes of a level, so at most w of them have weight below w (Kraft). If the planted path is exchangeable with its siblings, then on the planted path the number of large children is one plus a Poisson number (the size bias of a planted path), and 2^gain is a non-negative supermartingale in the cutoff depth with one-step factor 1 - exp(-lambda) below 1. By Ville's inequality **P(best gain >= g bits) <= 2^-(g+1)**. So the typical gain is negative (the planted path has more children than an average node), and luck of g bits has odds 2^-g.

**A portfolio over K orders (derived).** Its success rate per node is a weighted mean of the members' q_o(t_o)/t_o, so it never beats its best member, and running blind costs up to log2 K bits. Rotations and reflections are scan orders; relabelling is a value order.

### Measurements

**M-a. The tree is very imbalanced (measured, real pieces, no solution needed).** `g3tree.c` reimplements Track 10's tree and reproduces its total: 10^40.438 against the independently assessed 10^40.457. For population members (uniform over a level up to sampler noise) the subtree below was counted exactly. At depth 96: mean 3.0e5 nodes, median 22, 20% of nodes have nothing below, and the largest 1% of subtrees hold 61% of the work. Depth 112: mean 5,600, median 8. Depth 128: mean 523, median 5. If the solution sat under a median node, "draw, search below, restart" would gain 13.4 bits at depth 96. Also: depth-first search with random value order needs more than 10^6 nodes to reach depth 24 in 59 of 300 runs (a cell above a clue is refuted 16 placements after the choice that dooms it). These are heavy tails in the sense of Gomes et al.; they sit in the first two rows, are bounded by the width of those rows (under 10^16 nodes, derived from the level profile), and are already counted in T.

**M-b. The solution does not sit under a median node (measured, 11 painted boards, scan of Track 10).** The planted prefix's subtree is 27 times the level mean at depth 96 (range 1.8 to 135) and ranks above 94.5% to 99.95% of the level's nodes. Its work rank Q_96 over the 11 boards: 0.02, 0.05, 0.15, 0.24, 0.32, 0.44, 0.51, 0.66, 0.80, 0.88, 0.89. That is what uniform draws on (0, 1) look like: at depths 96 and 112 the solution is located in proportion to work, the hypothesis of Chen, Gomes and Selman. At depths 128 and 144 Q is skewed toward 1 (medians 0.85 and 0.95). The ideal uniform-draw scheme gains a median of -0.44 bits at depth 96 (range -0.95 to +2.12), -0.41 at 112, -0.90 at 128, -0.98 at 144. An oracle knowing every subtree's size and S* would gain a median of 0.7 bits at depth 96. On the 54 exact toys: minimum over depths of Q has medians 0.20 to 0.52, and the best depth gains at most 1.1 bits.

**M-c. The restart gain itself, at full size (measured, 6 painted boards times 4 scan directions).** Every sibling of the planted path (412 to 451 per case, 148 to 196 of them with fewer than 1,000 nodes below) had its subtree estimated by an SMC of 10^4 members started from it (exact when no level exceeds the population). Their sum reproduces the tree within 0.4 bits of the level profile on the six boards where both exist. From the sizes, F2 gives:

| Quantity, bits | median | min | max |
|---|---|---|---|
| time to solution over tree, quantiles 1%, 10%, 50% (sampled orders) | 0.011, 0.097, 0.50 | | |
| best achieved gain, any cutoff | 0.04 | -1.57 | 4.54 |
| upper bound on the gain, any cutoff | 2.37 | 1.44 | 6.56 |
| best achieved gain, cutoff at most tree/2^40 | -1.29 | -5.93 | 4.54 |
| best achieved gain, cutoff at most tree/2^85 (a run of about 10^15 nodes) | -7.71 | -19.98 | 2.13 |

Cases with an achieved gain of at least 1, 2, 3, 4, 5 bits: 7, 5, 2, 2, 0 of 24, against 2^-g = 12, 6, 3, 1.5, 0.75. The two cases at 4.5 bits use a cutoff near tree/2^56 with success probability 2^-50 per run.

**M-d. Uniform-child probes (measured, real pieces).** A probe that picks a uniform valid child survives to depth 32 with probability 2^-9.8 and to depth 48 with 2^-17.5 (2 million probes). Its law, given survival, diverges from uniform on the level by 0.2, 2.3, 3.4 and about 6 bits at depths 16, 32, 40, 48 (at least 10 at depth 96, population estimate). On painted boards the planted prefix costs 43 bits more than a uniform draw at depth 96 (range 35 to 58), a uniform node 44: the planted prefix is no easier to hit than any other node.

### Answer to the question

The largest gain is G of F2, a function of the tree total and of the sizes of the planted path's siblings. Its solution-free parts (T, the per-level branching law, the subtree-size distribution of a level, probe survival) are measurable on the real puzzle now, and M-a and M-d did so. The n_j(t) are functions of the solution and cannot be known without it; their law can, from painted boards, and gives G a median of 0.0 bits with a 2^-g tail. A scheme run blind (ten cutoffs times four directions) pays about 5 bits for not knowing which one is lucky, so a net gain of 10 bits needs luck of about 15 bits.

## 4. The one mechanism that exists, its price and its falsifier

**M1, restart lottery.** Run row-scan depth-first search with a random value order and a random one of the four scan directions, cut at t nodes, restart; t on a geometric grid.

- Price on the real 16x16 (derived from M-c, guessed in transfer): 134.4 - 1 - G bits with G a median of 0.0, at least 5 with probability about 3%, at least 10 with probability about 0.1%, before the 5 bits of the blind grid. With runs of 10^15 nodes: a median of 7.7 bits worse than plain search, that is about 2^91 runs instead of the 2^84 that the budget arithmetic gives.
- The single assumption: the real instance's planted path is distributed like a painted board's. Support: Track 10 found the official set ordinary among 20 painted boards; today its level-96 shape sits at the edge of 11 painted boards (median subtree 22 against 20 to 25; share of work in the top 1%: 0.61 against 0.52 to 0.73; zero share 0.203 against 0.165 to 0.199; median-node gain 13.4 bits against 13.5 to 14.0). Two of four descriptors fall just outside.

**Falsifier E1 (kills "empty").** 100 painted 16x16 boards, four directions, sibling mode of `g3tree.c` (33 s each). Gate first: SMC sibling sizes against exact counts for every sibling below 3e9 nodes, and populations of 10^4 against 10^5 on 20 cases. Measure the best achieved gain per case. Baseline: the 2^-g tail. Kill line for emptiness: more than 12 of 400 cases above 8 bits, or any case above 12. Cost: 3.7 core-hours plus about 1 for the gate.

**E2 (not covered here).** Randomised variable order with propagation, the setting of Gomes et al.: the monte-carlo survey's C1 (bb-mrv, random tie-breaking, critical ladder), kill line as written there. A few core-hours.

**E3, transfer.** The shape of M-a on the real pieces in the other three directions and on 50 painted boards. Ten core-minutes.

## 5. Odds

A restart or portfolio scheme over value orders, scan directions, symmetries and relabellings yields 10 bits net on the real puzzle: 0.1% (guessed from the measured tail and the blind-grid cost). 56 bits: under one in a million, and that figure is the chance that the planted-board reading is wrong for the real instance, not the chance of luck, which is 2^-56. Dynamic orders with propagation are outside these odds.

## 6. Not checked

- M-c rests on SMC estimates of sibling subtrees with 10^4 members. Only their sum was checked. Noise in the sizes inflates a maximum over cutoffs, so the 4.5 bit maximum is more likely too high than too low.
- 24 cases are 6 boards; directions of one board are not independent trials. The 2^-g tail is compatible with the counts, not established by them.
- Population members share ancestors, so the samples of M-a and M-b are not independent. Sample means agree with the level profile within 18% (median 3%).
- The counting bound assumes exchangeable siblings, which M-a contradicts; it is the reason offered, M-c is the evidence.
- Transfer from painted boards to the real puzzle is the weakest link (D4's painter control failed on 6x6 boards).
- Only row scans (four directions) were measured.
- F1 and F2 are written for one solution; the starter-clue case (about 14,702 expected) was not examined.

## Appendix A. Derivations

**F1.** The children of a node are visited in a uniform random order, so each sibling of the planted child precedes it with probability 1/2, whatever the others do. Time = D + sum over siblings of [precedes] times (1 + size). Take expectations; the sum of (1 + size) over all siblings is T - D.

**F2.** If a sibling of size above t precedes the planted child, the run spends more than t nodes before the solution. Per depth the planted child is first among itself and the n_j(t) large siblings with probability 1/(1 + n_j(t)); depths are independent. Luby et al.: cost l(t) = E[min(T, t)]/q(t) >= t(1 - q)/q. Conversely, with t' = D + (sum of siblings of size at most t), a run cut at t' succeeds whenever no large sibling precedes, so l(t') <= t'/q.

**F3.** Attempts are independent, succeed with probability pi(x*), and cost at most E_pi[min(S, S*)] plus the cost of drawing. Plain search costs T/2 >= N_d E_U[S]/2. For pi uniform, E_U[min(S, S*)] >= E_U[S; S <= S*] = Q_d E_U[S].

**Supermartingale.** In a tree where the subtrees below the nodes of one level are exchangeable, take t equal to the common size at level k. Then T/t is the number N_k of level-k nodes, a product of mean branchings lambda_j, and gain = -1 + sum over j < k of [log2 lambda_j - log2(1 + n_j)]. On the planted path n_j is Poisson(lambda_j) (the planted child plus the offspring of an ordinary node), and E[lambda/(1 + n)] = 1 - exp(-lambda) < 1. Hence 2^(gain + 1) is a non-negative supermartingale started at 1, and Ville gives P(sup >= 2^(g+1)) <= 2^-(g+1). The mean step is log2 lambda - E log2(1 + Poisson(lambda)), about -0.41 bits at lambda = 2: short cutoffs lose more than long ones, as on the re-check's toys (tree/100 loses 1.6 to 6.9 bits) and in the last row of the table of M-c.

**Why imbalance does not help.** F3's bound is a likelihood ratio: the planted law's mass on "small subtree" over the work's mass on it. The planted prefix is, in the ensemble, a prefix drawn in proportion to its number of completions, and both completions and subtree size rise with the same census of remaining tiles (D4's statistic S6). M-b measures the ratio directly: about 1 at depths 96 and 112, below 1 deeper.

## Appendix B. Scripts and outputs

All in `notes/gap-3/`. Total compute about 45 core-minutes on one core (the plan's 30 was raised by the coordinator to one core with runs of at most 15 minutes), including two runs lost to my own errors (an uncapped search, an uncapped planted subtree).

| File | Role |
|---|---|
| `g3_prep.py` | board files: real pieces in four scan directions; painted boards with the official colour census (60 frame joints, 420 inner joints), planted solution, five clue cells pinned |
| `g3tree.c` | Track 10's exact row-scan tree: planted path, exact subtree counts, population SMC, uniform-child probes, random-order search to a depth, sibling mode. Build: `cc -O2 -o g3tree g3tree.c -lm` |
| `g3_analyze.py` | level shape, planted rank Q, gains of F3, probe divergence |
| `g3_cutoff.py` | F2 from sibling sizes: sampled time distribution, achieved gain and upper bound per cutoff |
| `g3_toy_rank.py` | exact Q and gains on the re-check's 54 toy boards (same seed; reproduces his best-cutoff gains 0.61, 1.57, 0.21 maxima) |
| `out/real_shape.txt`, `out/real-BU.levels.txt`, `out/real-BU.first-reach.txt` | M-a, M-d |
| `out/painted_shape.txt`, `out/paint-*-BU.levels.txt` | M-b (member rows of 2.9 MB per board not kept; regenerate with seeds 1 to 11) |
| `out/sib/`, `out/cutoff.txt` | M-c |
| `out/toy_rank.txt` | toys |

Commands (W = a work folder holding the binary and `boards/`):

```
python3 -I g3_prep.py real  <official csv> W/boards
python3 -I g3_prep.py paint <official csv> W/boards 1 24
./g3tree boards/real-BU.txt 200000 7 200 4000 3000000000 96,112,128,144 2000000 -1 0 0      # M-a, M-d
./g3tree boards/paint-$s-BU.txt 200000 $s 200 4000 3000000000 96,112,128,144 0 -1 0 0       # M-b, s = 1..11
./g3tree boards/paint-$s-$d.txt 0 $s 200 0 0 96 0 -1 0 0 10000                              # M-c, s = 1..6, d in BU TD LR RL
./g3tree boards/real-BU.txt 0 1 180 0 1000000 16,24,32,40,48,64,80,96 0 -1 300 200000       # first-reach cost
python3 -I g3_analyze.py out/*.out ; python3 -I g3_cutoff.py out/sib-*.out ; python3 -I g3_toy_rank.py
```

Boards 1 to 4 of M-b were run with a cap of 2e9 and a looser cap on the planted subtree; no count reached a cap in any of the 11 boards.
