# Gap 2: lossy representations of the used set

> **Status (2026-11-13):** independently re-checked, see `notes/gap-2-lossy-representations-recheck.md`.
> **Re-check's rulings:**
> - Verdict "the gap is empty": the conclusion survives, but three of the reasons given for it do not and the headline number is wrong. "Empty" stands for the family the memo names (lossy states that are functions of tile ids and frontier colours chosen without looking at the tile colours, one table, region lists). The largest gain found anywhere in the family is 2 bits on the strict join in level terms, and 4.1 bits if a disjoint-set join that nobody has is assumed.
> - "The exact join gains 5.9 bits on the tree": wrong. Like with like, node sum 2^131.6 (cut at depth 139) against 2^134.6 is a gain of 3.0 bits; widest level 2^128.7 against 2^130.2 is 1.5 bits. The memo compares a widest level with a node sum.
> - "Complementary used sets make the test an equality": true only for the last join of a plan. Every join below it is a disjointness test, which has no hash key.
> - M1.1 and A1 (no b-bit sketch beats a b-bit hash): stands with a narrower scope, the proof needs a repair. A1 as written proves 2^-(b1+b2); the pass rate is at least 2^-min(b1, b2).
> - A2 (whole-board bound): stands, the proof needs a repair (count ordered prefixes per state). The price does not rest on it.
> - "Tracked subsets are the best sound sketch" and the price "at least 2^128.7": unproven, replaced. Any single sound sketch table whose sketch is chosen without looking at tile colours costs at least 2^126.7 in level terms, so the family saves at most 2.0 bits on the strict join. The bound is the re-check's, model level, weakly checked on toys, no second reader.
> - Neither 2^126.6 nor 2^128.7 is a floor; review section 3 says what each is the price of.
> - M3 and A4 (random tile split 2^275 to 2^277): stands with a narrower scope, correct for restrictions chosen without looking at the tiles, and it uses the relabelling symmetry of the planted ensemble, an assumption about the real instance.
> - Appendix B "seam of 35 to 39 bits" and appendix D "190 against 217 bits": wrong reading of a correct number. 35 is not a collision rate. Independent halves collide at 61.9 bits; halves with complementary tile sets at 42.9 bits. The hash width for no false join is 190 bits; 217 is wrong.
> - The five unopened papers: two attributions wrong, three correct at abstract level only (review section 5).
> - Proposed extension of O1's scope: stands with a narrower scope, text in review section 7.
> **Read with:** `notes/gap-2-lossy-representations-recheck.md`; `surveys/DERIVATIONS-recheck.md` (W5, W6) for the "floor 126.6" baseline the memo starts from.

Draft, 2026-11-13, not independently checked. One Desk study. Scripts and outputs in `notes/gap-2/`. Every statement is tagged measured (toy boards, this study), derived (first-moment model on the real colour census, scripted), cited, or guessed. Toy boards are generated boards and say nothing about Eternity II beyond checking the algebra.

**Verdict: the gap is empty.** No lossy summary of "which tiles are used" makes two halves joinable, or a frontier tabulable, below the cost of the exact two-list join. The reason is one counting identity, stated in section 3. The best any member of the family does on the real board is the exact join itself, 2^128.7 in the model against a model tree of 2^134.6: a gain of 5.9 bits, a footnote under the plan's own rule. Two by-products need the program's attention: the "2^126.6 floor" quoted in O1 is a list size, not a cost, and does not reproduce as a floor (section 1); and the fixed-census model gives a seam of 35 bits where uniform colours give 61.9 (appendix B).

## 1. The gap, restated, and what the evidence covers

O1 says: exact tables keyed by (boundary colours, used tiles) are as large as the tree, and neither a sieve over tile subsets (B1) nor oblivious linear compression (B2) shrinks them. Its scope note excludes "lossy hashed states". The question is whether a state that is allowed to be wrong about the used set can be much smaller and still useful.

"Lossy" covers four different things, and the existing evidence treats them unevenly.

| Kind | Example | Existing evidence | Covered? |
|---|---|---|---|
| L1. Lossy key in a join of two strictly generated lists | hash or Bloom filter of the used set, collisions verified later | cryptanalysis survey D1, D3 (join priced at 131.9, "floor" 126.6); second analysis W5, W6 | Time: yes. The trade-off between key width and false joins was never written down |
| L2. Sound sketch inside a table (a relaxation: never rejects a legal filling, may accept illegal ones) | track a subset of tiles, count per class of a random partition, random linear projection of the indicator | exact-algorithms 2b "colour coding" (worst case only); ROUTES section 4 path 1 (colour-only counting vacuous) | No. B1 covers tracked subsets only when completed by a sieve; B2 covers rank, not relaxation |
| L3. Unsound sketch (may reject the solution), rerun with fresh randomness | colour coding proper, divide and colour, bitstate hashing of the frontier | cryptanalysis D5 point 1 (blind restriction loses); second analysis item 7 (beam of population P costs 2^135.7 to 2^137.2) | Mostly. The random split of tiles between halves was priced only as "information about piece zones" |
| L4. Approximate counts used to steer | number of relaxed completions as a value heuristic | none direct; O2 and Track D4 measure statistics of partial boards | No |

One correction to the baseline first (derived, `g2_real_tradeoff.out` section 1; detail in appendix B). The 10^38.16 = 2^126.8 quoted as the join floor is the larger of the two list sizes at a cut after row 7 or 9. It is neither a floor on list size (2^124.6 after row 5) nor the cost of producing the lists, since each list is the last level of a tree that is wider in mid-row. With each side generated by its own row scan, the cheapest of the 255 cuts costs **2^128.7, at depth 137**. The exact join therefore gains 5.9 bits on the model tree (2^134.6), not 7.8. This disagrees with the independently re-checked figure and needs a ruling; it changes no verdict.

## 2. Literature

Opened this study:

- Dillinger and Manolios, "Bloom Filters in Probabilistic Verification" (FMCAD 2004), https://www.ccs.neu.edu/~pete/pub/bloom-filters-verification.pdf (abstract and introduction read). Explicit-state model checking is the field that has used lossy visited-state tables for 35 years: "the bitstate method, which stores states in a Bloom filter" and hash compaction. Both save memory per state; the search still visits every reachable state it reports, and "supertrace ... starts omitting states" when the table is too small. Cited. Bearing: L1 and L3. The field uses lossy state storage to save memory or to search partially, never to visit fewer states for the same coverage.
- Baldacci, Mingozzi, Roberti, "New Route Relaxation and Pricing Strategies for the Vehicle Routing Problem", Operations Research 59 (2011), https://orbit.dtu.dk/en/publications/new-route-relaxation-and-pricing-strategies-for-the-vehicle-routi/ (abstract page). "We introduce a new route relaxation called ng-route". Cited for existence only. Bearing: vehicle routing uses L2 in production: the visited set of a path is replaced by a small memory, and the relaxation's value is a bound inside branch and price.

Not opened, known from search summaries only, so unverified: Righini and Salani, Networks 51 (2008), decremental state-space relaxation (start with no tracked node, add the nodes that repeat in the relaxed optimum, iterate; exact dynamic programming and plain state-space relaxation are its two extremes); Christofides, Mingozzi and Toth (1981), the original state-space relaxation; Hoda, van Hoeve and Hooker (CP 2010), relaxed decision diagrams of bounded width for all-different; Håstad and Wigderson, Theory of Computing 3 (2007), disjointness of two k-sets in Θ(k) bits (the journal site refused the connection twice); Chen et al., SIAM J. Comput. 2009, divide and colour.

I found no use of a relaxed used set as a search device for a feasibility problem with about one solution. In routing the relaxation pays because it bounds an optimisation and because routes are short (tens of customers out of hundreds), the small-region regime where section 3 also finds a gain. Guessed, from abstracts and summaries.

## 3. Mechanisms, each priced on the real 16x16

Notation at a cut after depth d of the baseline row scan (model order "rows bottom-up", five clues). N_A: strict fillings of the first d cells. N_B: strict fillings of the other 256 - d cells. F: number of seam colourings (2^61.9 at a row boundary, 2^66.0 mid-row). C: number of admissible used sets (2^241.3 at d = 137). All from `g2_real_tradeoff.py`, which imports the re-check's `m12_model.py`.

### M1. Hashed or Bloom-filtered join (L1)

Algorithm. Generate the bottom list by row scan from the far rim. Insert (seam, h(used set)) into a table, h a b-bit hash. Run the top row scan to depth d; at each node look up (seam, h(complement of its used set)); verify hits against the stored fillings or by continuing the search.

The trade-off, exactly.

1. *No b-bit sketch beats a b-bit hash.* Because the two used sets have sizes adding to 256, "disjoint" is the same as "the second equals the complement of the first". The test is equality, not disjointness. For any pair of maps σ, τ into 2^b values and any acceptance rule that accepts every complementary pair, the number of accepted pairs among C x C is at least the sum of squared class sizes, at least C^2 / 2^b by Cauchy-Schwarz, with equality for a balanced hash compared for equality. So the pass rate of a wrong pair is at least 2^-b, and structured sketches (tracked subsets, class counts, Bloom filters of the set itself) can only be worse per bit. Derived (appendix A1). On the toy boards, structured sketches applied to the strict list behave like a hash of about the same width; I did not test the inequality sketch by sketch.
2. *Pass rate of a wrong top node* = 1 - exp(-N_B / (F 2^b)), a one-hash Bloom filter with F 2^b cells and N_B insertions. Derived. The toy tables show the shape (the pass rate collapses once the cells outnumber the list); the formula itself was not fitted, since the toy seams are far from uniform.
3. *Candidate joins* = N_A N_B / (F 2^b).

At the best cut (d = 137: N_A = 2^127.1, N_B = 2^128.7, F = 2^66.0):

| b | cells F 2^b | pass rate of a wrong top node | candidate joins | occupied cells |
|---|---|---|---|---|
| 0 (colours only) | 2^66 | 1 | 2^189.9 | 2^66 |
| 58 | 2^124 | 1 | 2^131.9 | 2^124 |
| 64 | 2^130 | 0.35 | 2^125.9 | 2^128.4 |
| 72 | 2^138 | 0.0017 | 2^117.9 | 2^128.7 |
| 190 | 2^256 | 0 | below 1 | 2^128.7 |

The table is informative only once b reaches log2(N_B / F) = 62.8, and at that point it holds N_B entries. Below that every cell is occupied and the filter passes everything. So 190 bits of key remove all false joins where the exact key has 256, and a Bloom filter at false-positive rate 10^-6 needs 29 bits per entry instead of about 190. That is a saving of 2.7 bits of memory on a table of 2^127 entries, and nothing in time.

Price: 2^128.7, identical to the exact join. Gain over the exact join: 0 bits of time. Assumption: none beyond the first-moment model.

### M2. Sound sketch DP for the bottom table (L2)

This is the only place where a lossy state could cut time: build the bottom table by a dynamic programme over (frontier, sketch) without enumerating strict fillings, so that the work is the number of table states and not the bottom tree's widest level.

Algorithm. Choose a sketch s that can be updated tile by tile and that never forbids a legal placement: s = used set restricted to t tracked tiles (the other 256 - t may repeat); or the count of used tiles in each class of a random partition; or M x (indicator) mod q for a random matrix M. Run the frontier DP over the bottom region with state (frontier colours, s). At the seam the occupied cells form the filter of M1.

Why it fails. A sound sketch accepts every legal filling and, in addition, every "pseudo-filling" it cannot tell from one. Two exact statements:

- *Whole-board cut* (derived, proof in appendix A). If the sketch takes W values at depth j of a fill of the whole board, the accepted tile sequences number at least 256! x C_j / W. So a sketch small enough to merge anything at the peak (W below the 2^61 to 2^64 used sets per seam that the tree has there) admits at least 2^(241 - 64) = 2^177 pseudo-solutions per solution. Named sketches are priced in appendix B.
- *Region* (derived, `g2_region_sketch_price.py`). For a bottom region of r cells the inflation is smaller, because a region uses only r of the 256 tiles and few of them collide. For the tracked-subset family the table at the seam holds N_B x I entries with I = [sum_k C(r,k) t!/(t-k)! (256-t)^(r-k)] / [256!/(256-r)!], and the work is max over layers of min(N_i I_i, F_i C(t, k_i)).

| cut d | region cells | strict widest level | t = 0 | t = 64 | t = 128 | t = 192 | t = 240 | best t |
|---|---|---|---|---|---|---|---|---|
| 96 | 160 | 2^135.3 | 222.1 | 208.1 | 190.7 | 168.4 | 143.0 | 256 (strict) |
| 137 | 119 | 2^128.7 | 176.4 | 168.1 | 158.3 | 146.1 | 133.8 | 256 (strict) |
| 161 | 95 | 2^113.9 | 142.9 | 137.5 | 131.2 | 123.6 | 116.6 | 256 (strict) |
| 192 | 64 | 2^85.9 | 97.2 | 95.5 | 92.7 | 89.6 | 86.9 | 256 (strict) |
| 208 | 48 | 2^75.9 | 74.5 | 80.6 | 79.2 | 77.6 | 76.4 | 16 (2^74.2) |

The sketch wins only for regions of about 48 cells or fewer, by 1.7 bits, and such a region starts at depth 208, 47 cells after the peak, where the tree is already 2^100 or less. Over every cut from 8 to 248 and every t, the cheapest "top search plus bottom table" is t = 256 at d = 137: the strict join.

Measured on toys (`g2_toy_sketch.out`, `g2_toy_sketch_6x6.out`; 8 generated boards, one 4x4, six 5x5, one 6x6; 18 to 20 sketch DPs each): no sketch DP has both work under half the strict list's work and a pass rate under 0.5. Those under half the work pass 89% to 100% of wrong halves. The cheapest DP with a pass rate under 0.5 costs 1.1 to 1.9 times the strict work on seven boards, and the worst sketch on each board costs 3 times (4x4), 4 to 54 times (5x5) and more than 18 times (6x6, runs stopped at the cap). One exception of the predicted small-region kind: on a 5x5 with a two-row bottom region and 1,612 solutions, 6 tracked tiles gave 0.73 of the work at pass rate 0.42.

Price: at least 2^128.7; in practice worse than M1. Assumption the price rests on: that tracked subsets are near the best sound oblivious sketch for a region. Class counts and linear sketches are worse per bit in both the formulae and the toys, but I have no proof of extremality (an isoperimetric statement about families of sets; appendix A says what is and is not proved).

### M3. Unsound sketches (L3)

- *Random split of the tiles between the halves* (divide and colour), so that the join key is the seam alone: success per split 2^-239 to 2^-242, each half then costs 2^28 to 2^38, total 2^275 to 2^277. Derived.
- *Any tile-blind restriction of the allowed used sets.* By the relabelling symmetry of the planted ensemble the solution's used set at a depth is uniform over admissible sets, like the tree's. A family keeping a fraction f keeps the solution with probability f and the level by the same f, and earlier levels by less. Cost per success never drops below the tree (appendix A4). Derived; it generalises the survey's D5 point 1.
- *Bitstate hashing of the frontier* (one node per hash cell) is a beam; the re-check's item 7 prices every population size at 2^135.7 to 2^137.2. Cited.

Approximate counts used to steer (L4) carry signal only where the filter of M1 already prunes (appendix B).

## 4. The smallest experiment that would falsify "empty"

The claim to break is M2's assumption. Boards: generated, framed, near-critical, 6x6 and 7x7, 20 each (toys). For the bottom half, optimise the sketch instead of drawing it at random: greedy choice of the tracked set, decremental refinement (add the tiles that repeat most in pseudo-fillings), classes built from shared colours. Measured: DP work over the strict list's work, and pass rate of wrong top halves. Baseline: the strict hashed list (ratio 1, pass rate 2^-b). Kill line for the emptiness claim: a sound sketch with work ratio under 0.5 and pass rate under 0.5 on a majority of 7x7 boards, with the ratio falling from 6x6 to 7x7. Cost: about 2 core-hours compiled. I would not schedule it: even a pass is capped by the join floor, 5.9 bits.

## 5. Odds

- A lossy representation of the used set yields 10 bits on the real puzzle: under 0.5%. It would need both M2's assumption to fail and the join floor to be 4 bits lower than computed here.
- 56 bits: under 0.01%. Every mechanism in the family is bounded by a table at least as large as one side's list, and the two lists cannot both be below 2^128.7 under row-scan generation.

## 6. Why the gap is empty, in three sentences

Testing two halves for compatibility is cheap in bits: complementary used sets make it an equality test, and 63 to 190 bits of hash do everything 256 bits do. What is expensive is the number of things to test, and a lossy state cannot reduce it, because a table that rejects anything must be sparse and a sound sketch only adds entries to it. Unsound sketches discard the solution exactly as fast as they discard nodes, since nothing about a tile-blind family points at the solution.

Proposed extension of O1's scope: "nor lossy states that are functions of the used set and frontier alone (hashes, Bloom filters, tracked subsets, class counts, linear projections, random tile partitions)", graded derived and toy-checked, not independently checked. Outside it: sketches chosen by looking at the instance (which tiles to track, from their colours), which is instance-specific reasoning and belongs to gaps 1 and 5.

## Appendix A. Proofs

**A1. Hash optimality.** Let U be the admissible d-sets, |U| = C. A scheme is (σ, τ, R): σ on top sets, τ on bottom sets, R(σ(A), τ(B)) true whenever B is the complement of A. Put τ'(A) = τ(complement of A) and v(A) = (σ(A), τ'(A)). If v(A) = v(A'), then R(σ(A), τ(complement of A')) = R(σ(A'), τ(complement of A')) is true, so the pair (A, complement of A') is accepted. With n_v the class sizes of v, accepted pairs number at least the sum of n_v^2, at least C^2 / 2^b where b counts the bits of σ and τ together. The pass rate of a wrong pair is at least 2^-b - 1/C. A balanced hash compared for equality attains it.

**A2. Whole-board cut.** A sound layered sketch is an automaton reading tile ids in cell order, with W_j states at layer j, that accepts every permutation of the 256 tiles. Let S_q be the j-sets whose orderings reach state q. From q the automaton must accept every ordering of the complement of every member of S_q, and distinct members have distinct complements, so it accepts at least |S_q| (256 - j)! suffixes, and at least |S_q| j! prefixes reach q. Accepted words are at least j! (256 - j)! sum_q |S_q|^2, at least 256! C(256, j) / W_j. In the first-moment model each accepted word matches colours with about the same probability as a permutation, so pseudo-solutions per solution are at least C_j / W_j. Not proved: the same-probability step for words with repeated tiles (guessed to be accurate to a small factor; the toy runs agree in direction).

**A3. Region version.** For a region of r cells the suffix after layer j uses r - j tiles, not all the rest, so the complement argument weakens to: suffixes disjoint from some member of the class. For tracked subsets this gives the formula of M2. A lower bound valid for every sketch with W_j values is the open step.

**A4. Unsound families.** Relabelling the tiles by a uniform random permutation maps the planted ensemble to itself and sends the used set of any node, and of the solution's prefix, to a uniform admissible set. Let a tile-blind filter keep a fraction f_k of the admissible sets at each depth k where it acts (f_k = 1 elsewhere). Level k of the tree shrinks to at least N_k f_k times the survival of its ancestors, and the solution survives with probability p at most min_k f_k. If the filter acts at one depth d, cost per success is at least (sum of N_k for k below d) / f_d + (sum for k from d on), at least the tree. Filters at several depths compose the same way.

## Appendix B. Numbers and one oddity (all derived, `g2_real_tradeoff.out`)

- Model: tree 2^134.6, peak level 2^130.2 at depth 161, unplanted E[Z] with five clues 2^-24.5 (the planted solution is in addition).
- Used sets per seam in the tree: 2^61.2 at d = 137, 2^64.3 at the peak (uniform seam). The survey's 2^68 used a 62-bit frontier at the peak, where the frontier is mid-row and 66 bits wide.
- Sketch bits needed before a table at depth d prunes anything: 54 (d = 64), 66 (96), 65 (112), 63 (137), 61 (144), 48 (161), 23 (192). A table placed at or after the peak is cheap and useless, since the nodes above it are already paid.
- Pseudo-solutions per solution on the whole board: colour only 2^364; 13 classes of 19 or 20 tiles (56 bits of state) 2^311; 128 pairs (203 bits) 2^128; 58 tracked tiles 2^332; a 58-bit linear sketch 2^306.
- Oddity: the identity N_A N_B = E[Z] x C x (seam) gives a seam of 35 to 39 bits in the fixed-census model against 61.9 to 66 for uniform colours. The model's match probability is not multiplicative across a cut (with a fixed multiset of colours the last edges nearly match by themselves). Whether independent halves of the real tiles collide on the seam at the 35-bit or the 62-bit rate is a measurable question (enumerate both halves of a painted 6x6 with a fixed census) and I did not measure it.
- Approximate counts as a steer (L4). A cell of the M1 table with a b-bit key holds on average μ = N_B / (F 2^b) fillings. The true prefix adds one. With Poisson noise the shift is μ^-1/2 standard deviations: at b = 32 and d = 137, μ = 2^30.8 and the shift is 2^-15 sd. A count carries signal exactly when the filter prunes, at the same table size. Derived; the Poisson spread is guessed. Nothing separate to test; any statistic of this kind belongs to RQ3.
- Bloom filter over the smaller list at d = 137: 14.4, 28.7, 57.4 bits per entry at false-positive rates 10^-3, 10^-6, 10^-12; 2^131 to 2^133 bits in all.

## Appendix C. Scripts

- `g2_real_tradeoff.py` (out: `.out`): cut table, hash trade-off, named sketches, unsound splits, Bloom memory. Under 1 s.
- `g2_region_sketch_price.py`: tracked-subset sketch DP priced on bottom regions, all cuts. About 1 minute.
- `g2_toy_sketch.py` (out: `g2_toy_sketch.out`, `g2_toy_sketch_6x6.out`): toy boards, strict lists, hashed tables, three sketch DP families. 4x4 and 5x5 boards about 30 s each; the 6x6 board several minutes, DP layers capped at 4 million states.

## Appendix D. What I could not check

- Extremality of tracked subsets among sound sketches for a region (M2).
- The 2^128.7 join floor assumes each side is generated by row scan; another order inside a 7 to 9 row band was not priced.
- Which seam width is right for false-join counts on the real tiles, 61.9 bits (uniform) or 35 bits (fixed census); see appendix B. It moves the hash width needed (190 against 217 bits), not the verdict.
- Toys stop at 6x6, where 36 tiles leave little room between "tracks nothing" and "tracks everything".
- Five of the seven papers named in section 2 were not opened.
- Process note: a first 6x6 toy run (it crashed on the state cap) and the region script may have overlapped for up to a minute, against the one-process rule. Total compute about 12 core-minutes.
