# Second analysis report: gap 3, the shape of the tree

Reviewed 2026-11-13: `gap-3-shape-of-the-tree.md` and `gap-3/`. Scripts and outputs of this review: `gap-3-review/`. Grades: **measured** (my script, today), **derived** (algebra here), **read** (the memo's own output files). Painted 16x16 boards are toys; nothing here is Eternity II progress. The two Cornell papers were not reopened; rulings that touch them rest on the memo's quotations.

## Summary

The verdict "empty at 10 bits" **stands with a narrower scope**, and for a different reason than the one the memo offers. The memo's proof is a proof about a tree it has itself shown not to be the real one, and the observed tail sits above the bound that proof gives. What carries the verdict is a simpler fact that the memo's data support and the memo does not state: at any fixed cutoff, the mean over cases of 2^gain is about 1/2. That is a statement about a mean, so it applies to any blind mixture of orders and cutoffs whatever the dependence between orders, and it settles the "best of K" worry without a tail bound.

Scope of the verdict as it can be defended: restart and portfolio schemes over value orders and the row and column scans, blind to the solution, on boards distributed like the painted ones. Not covered: other static orders, dynamic orders with propagation, restarts that keep learned information.

## Rulings

| # | Claim | Ruling |
|---|---|---|
| 1 | F1, E[T] = D + (T - D)/2 for every shape | **Stands**, for a tree with one solution and a search whose subtree below a node depends only on the node |
| 2 | F2, the bound on q(t) and the achieved form | **Stands**. "The largest restart gain is G" needs a narrower reading: the two forms bracket the gain, 2.3 bits apart in the median |
| 3 | F3, the draw-and-search scheme | **Stands as an idealisation** (a uniform draw from a level is assumed free; the memo's own M-d prices it at 10 bits at depth 96) |
| 4 | Counting bound, P(best gain >= g) <= 2^-(g+1) "under exchangeable siblings" | **Unproven** for the real tree; the model behind it is rejected by the memo's data. Replaced below |
| 5 | "The share of cases reaching g bits falls like 2^-g, as a counting argument says it must" | **Wrong as worded**. The argument gives 2^-(g+1); the counts sit above it; nothing says "must" |
| 6 | "A portfolio never beats its best member, and running blind costs up to log2 K bits" | **Stands**, but the cost is an upper bound that is not paid in full. Corrected below |
| 7 | M-a, shape of the real tree at depth 96: mean 3.0e5, median 22, top 1% holding 61% | **Stands with error bars the memo did not give**: median reproduces, mean and top share do not to two digits |
| 8 | M-a, "bounded by the width of those rows (under 10^16 nodes)" | **Wrong number**: the level profile gives 10^18.4 |
| 9 | M-b, the solution is located in proportion to work at depths 96 and 112 | **Stands**, and it is the size bias. Extended below to the whole path |
| 10 | M-c table: best gain median 0.0, maximum 4.5 | **Stands with corrected values**: exact median 0.5, maximum 4.8 |
| 11 | "Restarting loses a median of 7.7 bits at 10^15 nodes" | **Stands with a narrower meaning**; 7.7 is a bound, the exact median is 6.7 to 7.1, and it is not a cost of restarting |
| 12 | Transfer: "two of four descriptors fall just outside" the painted range | **Wrong**: sampling noise. On two fresh seeds all four fall inside |
| 13 | Odds: 0.1% for 10 bits | **Stands** inside the scope above. "Under one in a million" for 56 bits is a guess about a modelling assumption and has no support |
| 14 | "The 4.5 bit maximum is more likely too high than too low" (section 6) | **Not confirmed**: stable within 0.3 bits across seeds and a tenfold population |

## 1. The counting argument

**The proof.** The Kraft step is correct. The supermartingale step (Appendix A) takes "t equal to the common size at level k": every subtree of a level has the same size. In that model the algebra is right (E[lambda/(1 + Poisson(lambda))] = 1 - exp(-lambda), Ville applies). But M-a measures a median of 22 against a mean of 3.0e5 at depth 96, so the hypothesis fails by four orders of magnitude, as the memo's section 6 concedes.

**The model is also rejected by its own prediction (derived and measured, `r3_bestofk.py`).** A product of independent factors with mean below 1 has a tail a^-theta with E[X^theta] = 1. At lambda = 2 theta is 2.01, so the model predicts P(G >= g) of about 2^-2g, far below Ville: 5.1%, 1.2%, 0.27% at 1, 2, 3 bits (simulated, 400,000 paths). The exact gains of the 24 cases give 8, 5, 3 at those levels where the model expects 1.2, 0.3, 0.06 (binomial p of 1e-5; five boards of six have a direction above 1 bit, p of 0.001 by board). The real tree is more generous to restarts than the model, so the model cannot be the reason they fail.

**Is exchangeability the right assumption?** No. In a tree with unequal subtrees, a planted child exchangeable with its live siblings would sit under small subtrees more often than their share of work, and restarts would gain. The right hypothesis is the opposite of naive exchangeability: **the planted child is chosen among its siblings in proportion to subtree size**. That is what "size bias" means here, and yes, the factor of 27 is size bias. Measured on the real pieces at depth 96, the subtree size at which half of the level's work is reached is 31 to 48 times the mean (my two seeds; 42 in the memo's own `real_shape.txt`). A prefix placed in proportion to work therefore sits near 40 times the mean in the median, and the memo's 27 is that law at Q = 0.44. Deeper it is more than size bias: the same ratio is 12 and 7.5 at depths 128 and 144, against planted medians of 48 and 106.

Tested along the whole path (measured, `r3_tail.py` section E, 3,546 choices on the 24 cases, law P(child) proportional to size^alpha): alpha = 0 (exchangeable) loses 0.86 bits of likelihood per choice to alpha = 1. Best fit 1.10 overall; by depth 0.70 (0 to 31), 0.90 (32 to 95), 1.45 (96 to 159), 3.0 or more (160 on, partly an artefact: the planted subtree contains the solution path). The two shallow values are biased toward zero by estimator noise (0.3 to 0.55 bits per sibling there, `r3_noise.py`), so alpha below 1 in the first six rows is not established; it is the one place where the data lean the way restarts need, and it is worth at most 1 to 2 bits.

**Does the bound survive size bias?** A different bound does, and size bias is what makes it true.

Corrected statement:

- (derived, exact, any shape) A uniformly chosen node of a tree is among the first t nodes of a depth-first search with random value order with probability t/T, because the search visits t distinct nodes. So if the solution is placed in proportion to work, E[q(t)] = t/T, and since 2^gain(t) <= (T/2t) q/(1 - q), **the mean of 2^gain at any fixed cutoff is 1/2** up to the factor 1/(1 - q), which matters only within a few bits of the full tree. Shape does not enter.
- (derived) By linearity the same holds for any scheme that mixes orders and cutoffs without knowing the solution, **independent or not**. Markov then gives P(a blind scheme gains g bits on a drawn instance) <= 2^-(g+1) on the ensemble mean, about 2^-11 for 10 bits.
- (measured, section C) Mean over the 24 cases of 2^gain at cutoffs tree/2^10, 2^20, 2^40, 2^60, 2^85: 0.52, 0.56, 0.42, 1.19, 0.13, against 0.5 expected. The last two are carried by one case each (53% and 68% of the sum), so the check is good at long cutoffs and weak at short ones.
- (derived) For the best cutoff chosen knowing the solution there is no theorem. Ville does not apply (the set of large siblings changes with t). A union over a factor-two grid gives about 127 x 2^-g, and a tree in which each scale has its own lucky solutions saturates it. The 2^-g tail is an observation up to 5 bits on 6 boards, nothing more.

**Best of K orders.** Two different quantities, both computed (`r3_bestofk.py`, tail c x 2^-g with c = 1/2 for the memo's bound and c = 1 for the observed counts):

- Best member net of its K-fold cost: E[max G] - log2 K is a constant, -0.1 bits (c = 1/2) or +0.9 (c = 1), for every K from 4 to 2^16. An exponential tail in bits buys log2 K and pays log2 K.
- The portfolio's real net gain is log2 of the mean of 2^G over members, because rates add. With a tail of index 1 that mean grows like c ln K: 1.7 bits at K = 4, 2.6 at 64, 3.1 at 1,024, 3.7 at 65,536 (medians, c = 1), with a ceiling of 6.5 bits if the tail held all the way to the largest possible gain of 126 bits. Measured on the six boards with four directions each: -0.3, 0.5, 0.7, 1.7, 2.6, 3.1 bits, against 0.5 for one direction. So a portfolio does beat a single direction by 1 to 3 bits, which the memo's "pays log2 K" hides.
- Both figures assume each member runs at its own best cutoff, which is a function of the solution. With cutoffs blind as well, the mean of 2^gain is 1/2 and the scheme loses 1 bit.

No reading reaches 10 bits. Tile relabelling adds nothing (it is the value order, already averaged in q). Rotations and reflections make 8 row and column scans, of which 4 were measured.

## 2. The observed tail

Exact best gains (conditional sampling, checked against plain sampling where q is large): median 0.5, maximum 4.83, and 8, 5, 3, 2, 0 of 24 at 1 to 5 bits. The memo's 7, 5, 2, 2, 0 come from its achieved lower bound and compare against 2^-g (12, 6, 3, 1.5, 0.75). Its own bound is 2^-(g+1): 6, 3, 1.5, 0.75, 0.38. **The counts are above the stated bound at every level from 1 to 4 bits**, not significantly at any one (binomial p 0.17 to 0.23, cases treated as independent). They fit 2^-g. For a portfolio: see above, 1 to 3 bits with oracle cutoffs. I see no dependence between opposite directions in the gains (six pairs each, too few to say).

The two top cases are not noise: 4-LR gives 4.75, 4.54 (new seed), 4.26 (population 10^5); 6-TD gives 4.83, 5.13, 4.86. The tree total itself moves by 0.2 to 0.4 bits between runs.

## 3. "Restarting loses a median of 7.7 bits at 10^15 nodes"

Three things about what is compared.

- The baseline is plain search run to the end, 2^134 nodes, which nobody can run. The row constrains the cutoff to tree/2^85, about 10^15 nodes. A single run of 10^15 nodes is itself such a truncated run. So the figure is not a penalty for restarting against an affordable alternative. It says that a run of 10^15 nodes with a random value order succeeds with a median probability of 2^-91, not the 2^-85 of the budget share. The lower tail of the run time is thin, not fat.
- 7.7 is the memo's achieved bound. Exact: median -6.7 for the best cutoff at most tree/2^85 (range -19.3 to +2.6), -7.1 at exactly tree/2^85.
- It is a median over single (board, direction) cases, and the distribution is very skewed. The mean of 2^gain at that cutoff is 0.13 measured, 0.5 under work-proportional placement. The chance of success is the mean, and a scheme that spreads the budget over directions moves from the median toward the mean: averaged over the four directions of one board the six boards give -8.6, -6.9, -5.3, -4.6, -2.6, -0.9 bits, a median near -5, not -7.

Corrected statement: at a cutoff of 10^15 nodes a typical single direction is 6.7 to 7.1 bits below the budget share and the average direction about 1 to 3 bits below; in no case is it above by more than 2.6 bits.

## 4. The real pieces, reproduced

`r3_shape.rs` uses Track 10's child function unchanged (`tree.rs`, `puzzle.rs` at `dc71ad56`) and shares no code with the memo's `g3tree.c`: own sampler, own member draw, own exact count. Population 200,000, two seeds, 2.6e9 nodes counted, no count reached its cap, 87 s and 48 s.

| Depth 96 | memo | seed 101 | seed 202 |
|---|---|---|---|
| tree total, log10 (independently assessed 40.457) | 40.438 | 40.463 | 40.484 |
| mean subtree (level profile) | 3.0e5 (3.1e5) | 4.0e5 (3.3e5) | 2.0e5 (3.2e5) |
| median | 22 | 21 | 23 |
| empty share | 0.203 | 0.194 | 0.184 |
| share of work in the top 1% | 0.61 | 0.65 | 0.53 |
| median-node gain, bits | 13.4 | 13.9 | 12.7 |

Reproduced. The median and the empty share are stable; the sample mean of 4,000 members moves by a factor of two and the top share by 0.12, because single subtrees hold 4% to 5% of the sample's work. Depths 112, 128, 144 agree with the memo to the same precision (means 5,400 to 5,900, 577 to 585, 25 to 26; medians 8, 4 to 5, 2).

Consequence for the transfer argument: the memo reads the real set as sitting "at the edge" of the painted boards with two descriptors outside. With this run-to-run spread, all four descriptors of the real pieces are inside the painted ranges (empty share 0.184 and 0.194 against 0.165 to 0.199; median-node gain 12.7 and 13.9 against 13.5 to 14.0, a spread wider than the painted range itself). The solution-free shape gives no sign that the real set differs.

The "under 10^16" of M-a is wrong: the profile is 10^16.8 at depth 24, 10^18.4 at depth 28, 10^18.0 at depth 32. It does not touch the verdict (10^18.6 against 10^40), but it matters at the budget: the early thrashing is not bounded below 10^15, so a single uncut run can spend the whole budget in the first two rows. A cutoff is rational there for that reason alone, and it buys nothing toward the solution.

## 5. What the memo did not cover

- **Other static orders.** F1 and F2 hold for any fixed cell order. The placement law and the fixed-cutoff mean were measured for four scans only.
- **Dynamic orders with propagation**, the setting of Gomes, Selman and Kautz. The memo says so (E2) but its headline does not. The factor of two gives a sharper question than the memo asks. With about one solution, F1 makes the first-solution cost half the exhaustion cost in the mean for every search whose subtree depends only on the node. Two consequences. First, no run exceeds twice the mean, so the heavy upper tails that restarts cut in the literature cannot exist inside one tree; cutting long runs is worth at most 1 bit, and anything more must come from the lower tail, which is the placement law. Second, under work-proportional placement in each order's tree, a blind restart scheme over a randomised family with exhaustion trees T_o gains log2(T_ref x mean of 1/T_o) - 1 against a reference order: it is a way of sampling the family's small trees, and gains nothing against the family's best order. **Both T_ref and the T_o are solution-free and can be estimated on the real pieces with Track 10's estimator.** The absence of heavy tails on unsatisfiable instances, quoted by the memo, is the literature's statement that this dispersion was small where it was looked for.
- **Restarts that carry information** (learned nogoods): outside this memo; gap 1.
- **More than one solution.** A painted board has its planted solution plus about one more in expectation, so the time to the first solution is below the time to the planted one by a fraction of a bit, on both sides of every comparison. The starter-clue case was not examined (the memo says so).

## Left open, worth a study

1. **Dispersion of the exhaustion tree over randomised orders, on the real pieces** (solution-free). SMC estimate of log2 T_o for about 100 orders from each family: random static orders, minimum-remaining-values with random tie-breaking, with and without arc consistency. Statistic: log2(T_row x mean of 1/T_o), nodes and cost kept apart. Kill line: below 11 bits. This replaces the memo's E2 and decides whether the Gomes setting can matter here.
2. **E1 of the memo, with two changes**: the exact gain instead of the achieved bound (`r3_tail.py`), and as primary statistic the mean of 2^gain at fixed cutoffs tree/2^60 and tree/2^85, where 24 cases are carried by one. 100 boards, four directions, 3.7 core-hours. The kill line of "more than 12 of 400 above 8 bits" is lenient: 2^-g expects 1.6.
3. **Placement exponent in the first six rows** with populations of 10^5, where alpha reads 0.7 to 0.9 and estimator noise is half a bit. About 6 core-minutes per case.
4. Not worth a study: tightening the tail bound for the best cutoff. It needs the solution to be used.

## Scripts

All in `notes/gap-3-review/`. About 25 core-minutes on one core at `nice -n 10`, longest run 6.2 minutes, every search capped (3e9 per subtree, 1.2e10 per run). No web request was made.

| File | Role |
|---|---|
| `build.sh`, `r3_shape.rs`, `r3_shape.py` | item 4: real pieces, Track 10's child function, own sampler and counts; `out/real_shape.txt`, `out/real-*.levels.txt` |
| `r3_tail.py` | items 1 to 3 from the memo's `out/sib/` (read only): exact gains with a self-check, tail counts, fixed-cutoff means, four-direction portfolio, placement exponent; `out/tail.txt` |
| `r3_bestofk.py` | the memo's model (tail exponent, simulated tail) and the best of K; `out/bestofk.txt` (the K = 2^20 rows use 20 repetitions and are rough) |
| `r3_noise.py` | sibling sizes across seeds and populations; `out/noise.txt`, `out/sib-reruns/`, `out/tail-reruns.txt` |

The memo's `sib-6-TD.out` was regenerated bit for bit from its sources before the reruns.

One error of mine, fixed before the numbers above: the first version of the conditional sampler drew the planted child's position from the wrong law and overstated the exact gains by about 0.2 bits (median 0.71 instead of 0.51).
