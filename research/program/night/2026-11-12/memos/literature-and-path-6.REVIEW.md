# Review: `literature-2026.md` and `solve-path-6-free.md`

Referee pass, 2026-11-12. Independent of both authors. Scripts and raw output:
`memos/literature-and-path-6-review/` (`rerun.sh`, `rerun.out`, `extra.py`,
`extra.out`, `fixed.out`, `stall025.py`, `stall025.out`, `price_check.py`,
`price_check.out`). Compute used: about 7.7 core-minutes on one core.
Web requests used generic technical terms only.

One note on the brief: it gives the critical colours 4.65, 5.99, 7.32, 8.63,
9.94 for n = 5 to 9. `growth-law.REVIEW.md` (line 78 and line 193) gives them
for n = 6 to 10. I used the review's assignment.

## 1. Verdict table

Probability is that the claim is true as the memo states it.

### `literature-2026.md`

| # | Claim | Verdict | P | Reason |
|---|---|---|---|---|
| L1 | A live BOINC project page shows "466/480 Project record, 5 official clues", four 466 boards on 7, 9, 10, 12 November, 259 boards at 465+ this week, an OpenCL application opened on 10 November | holds with a correction | 0.9 that the page says it; 0.6 that the boards honour all five clues | Page opened. All quoted items are there. Live counters have moved (174 computers, 25,562 tasks; memo 172 and 25,621). Dates carry no year. The page gives no validation method and no board download, as the memo says. |
| L2 | "The practitioner record moved this month" (lines 23 to 29, map row 10 at line 864: "reported 466 (7 to 12 November 2026)") | wrong on the date, right that the site is stale | 0.1 for "this month"; 0.85 that the site's 465 is out of date | The memo's own source 10 ships a five-clue 466: `DATA_PROVENANCE.md` in that repository names the record `FiveClue_466_2c037e70f7e9`, attributes it to another contributor through the fleet's public statistics site, and was committed on 2026-10-08. A public five-clue 466 is at least five weeks old. The November dates are the BOINC application's own boards. |
| L3 | The page's node rate, and "distributed farms: saturated" should become "live" | holds with a correction | 0.8 | The memo does not report a node rate. The page shows "886 G nodes / second". The site's fleet page has 5 to 6 billion per second for August, so the reported rate is about 150 times higher. The repository has zero matches for the project's domain, so the BOINC project is new to the site. It looks like the same five-clue effort the site already tracks under another client (same domain as the fleet's statistics site; a contributor name matches), not a second farm. Not verified. |
| L4 | Sewa et al., AAAI 2026: AllDifferent with assignment bounds in toulbar2, 132 QAPLIB instances, 140 weighted N-queens, XCSP 2024 mini-COP, beats CP-SAT and integer programming | holds | 0.95 | The publisher page refused me too. I read the authors' preprint PDF instead: title, six authors, the 132 smallest of 136 QAPLIB instances, 140 N-queens instances (N = 4 to 30), the 2024 XCSP set, comparison with CP-SAT and cplex all match. Volume, pages and DOI confirmed through a search record. |
| L4b | "The sub-board and halo questions are within its published size range" (lines 238 to 240) | not established | 0.4 | In the paper, toulbar2 proves optimality on 33 of 132 QAPLIB instances in 1,200 s (example given: size 20, 337 s). Sizes are below 100 variables with at most N values each. An edge-matching cell has up to 1,024 values. Nothing published covers that domain size. |
| L5 | Edmonds, Edmonds, Mertz: rectangle from a corner has expected width 1 + Θ(ε); frame first, centre block and corner triangle blow up; undated manuscript, code agrees | holds with a correction | 0.9 | PDF read. Title, three authors, affiliations, Theorems 1 to 6 and "We coded the algorithm and it ran just as described" match. Correction: the frame-first rate is 2^Θ(√ε · t), ending at e^Θ(√ε · m) with m the number of edge pieces. The memo writes 2^Θ(√(ε t)) (line 552), which is a different exponent. |
| L5b | "Row-major scanning is optimal for a structural reason" (lines 561 to 563) | wrong | 0.05 | The manuscript says the opposite about a plain row scan: "A first problem is that completing the first row requires exponential growth. A solution to this is to instead grow a rectangle." Theorem 5 requires width and height at least two. A first row filled alone is the edge-first case of Theorem 2. |
| L6 | Site entry `rotations2017` has the wrong authors | holds | 0.99 | arXiv:1703.09421 lists Ebbesen, Fischer, Witt. `PapersView.tsx` line 48 says "J. Bosboom, E. D. Demaine, et al.". |
| L7 | Site entry `kovalsky2014` has the wrong title | holds, and it is worse than stated | 0.97 | I opened the PDF the site links: it is "A Global Approach for Solving Edge-Matching Puzzles", SIAM J. Imaging Sciences 8(2), 2015, authors in the order Kovalsky, Glasner, Basri. The site also has the year (2014), the venue ("preprint") and the author order off. The site's blurb ("assembles a whole jigsaw", "doesn't transfer to E2") is also wrong: the paper is about coloured edge matching, names the 16x16 puzzle, reports 12x12 instances solved and no success on the 16x16. |
| L8 | Nobody has published a weighted CSP, MaxSAT branch and bound, decision diagram or pseudo-Boolean run on edge matching | not established (no counterexample found) | 0.7 | Eight more searches and one arXiv API listing found none. Closest: Salassa et al. 2017 (MILP and Max-Clique, which the memo files under ILP). One lead I could not open: a JFPC 2025 paper returned by an "Eternity II MaxSAT / weighted CSP" query (HTTP 403). An absence from one search engine is a lead, as the memo says. |
| L9 | No peer-reviewed Eternity II paper after 2018, and no learning-based result on edge matching in a paper or thesis | not established (no counterexample found), with one inconsistency | 0.75 | arXiv API: no entry naming the puzzle after mid-2018; the only edge-matching entry is the 2020 complexity paper the site already cites. Inconsistency: the memo's own source 2 (the 2026 bioRxiv preprint) describes a package "including neural-network transformers", so the memo's absence 4 has a preprint-level exception it does not mention. |
| L10 | Gmys: 339 × 10^12 nodes, 13 hours, 256 GPUs, 11 open instances solved, 8 bounds improved, about 64 CPU-years, over 4 million agents | holds | 0.97 | arXiv:2012.09511 abstract opened; every number matches. The abstract names V100 cards for the scaling runs (up to 384), not specifically for the 256-GPU run. |
| L11 | Martinsson: at least two solutions for 2 ≤ q ≤ (2/√e) n, all similar for q ≥ (2+ε) n; (2/√e) × 16 ≈ 19.4 | holds | 0.95 | Abstract opened, n is the side. 1.2131 × 16 = 19.41. The expected-count formula is not in the abstract (the memo flags this). It is the plain first moment, n²! 4^(n²) / q^(2n²), so it is right. |
| L12 | Guzman repository: 1.3 to 1.9 billion nodes per second on a GTX 1070; map row 5 should say "practitioner ports at 10^9 nodes/s on a 2016 card" | holds as a quotation, not established as a fact | 0.95 that the README says it; 0.3 that it is a node rate comparable with the program's | README opened. It does not define a node, names no baseline for its SAT comparison, describes no outside check. It should not enter the map as a measured rate. |

Smaller items checked and found correct: Bosboom et al. (factor 0.9999999851,
"first (correct) proof"; the abstract does not name the earlier paper);
Seifer 2026 (D-Wave hybrid complete only up to 64 tiles, no official-board
score, DOI prefix as given); the BOINC wiki snapshot (pilot since September
2026, 66 users, 215 hosts on 30 September); Hamze et al. at abstract level
(cubic lattice, tiling puzzle; the abstract says "many orders of magnitude",
the memo's "2 to 3 orders" and instance counts are from the full text and I
did not recheck them); the repository-absence statements (zero matches for
Hamze, Martinsson, "population annealing", toulbar2 in the site and program
files).

One thing the memo read and did not flag: the BOINC wiki gives the best
published score as 468 (2020). The site's canonical page says 470. One of the
two is behind.

### `solve-path-6-free.md`

| # | Claim | Verdict | P | Reason |
|---|---|---|---|---|
| P1 | The scripts reproduce `results.txt` and `profile.out` | holds | 0.99 | `rerun.out`: every DFS count, every iteration list and all five `profile.py` lines are identical. 358 CPU-seconds. |
| P2 | "It works on all of them", iterations are 0.05 to 0.6 times the naive backtracker's nodes per solution | holds with a correction | 0.85 | Ratios recomputed: 0.045 to 0.64. "Works" means at least one start solved: on 6x6 with 7 colours seed 1, two of three starts (layouts only) and one of three (with rows and columns) hit the cap. 7x7 seed 0 is missing from the table (its DFS was capped) and the row-and-column variant was not run on it. |
| P3 | The ratio is a meaningful measure, and I = ρ × N carries to the 16x16 | not established | 0.2 | Units differ: the memo itself prices one iteration at 10^3 to 10^4 backtracker nodes. The baseline is the weakest possible. Against a smallest-domain-first DFS with no other propagation, nodes to a first solution are 1.2 to 7 times below the memo's reference on all seven instances and the ratio becomes 0.13 to 2.0 (rows and columns) and 0.25 to 2.9 (layouts only) (`extra.out`, part `base`). Seven instances, 42-fold range in tree size, 6 to 9-fold scatter in the ratio. |
| P4 | ρ falls with n (0.19, 0.079, 0.045) | not established | 0.15 | The instances move away from the critical colour count as n grows: +1.4 colours at n = 5, +2.6 and +3.6 at n = 6, +5.3 at n = 7 (`extra.out`, part `crit`). Size and distance from critical are confounded, and n = 7 is one instance. The memo already calls this trend weak. |
| P5 | The instances are near the critical colour count | wrong (the memo does not claim it, but the headline depends on it) | 0.02 | The generator draws seam colours independently with 3 to 5 ring colours, so the right first moment is the independent-colour one. Critical counts: 3.62 (n = 5), 4.37 (n = 6), 5.75 (n = 7), 6.60 (n = 8). Instances used: 5, 7 and 8, 11, 12. The 8x8 that was "solved" has 12 colours where the program's primary rung has 8 (reviewed critical 7.32 with five frame colours). |
| P6 | "A fixed point is a solution and nothing else is"; "its only resting points are exact solutions" | holds with a correction | 0.9 for the first sentence; 0.1 for the second | True of the map: at a fixed point the layout's embedding lies in the seam subspace. True of the stop rule: 12 terminal layouts passed my own verifier (`fixed.out`). But the code never tests for a fixed point. It stops when a layout is clean, with the map's step still at 1.5 to 5.6 in norm. And the map has other attractors: at β = 1 on 5x5 seed 1, three of three starts end in an exact period-2 cycle between two layouts with 11 mismatched seams, x bounded. At β = 0.25 the capped 6x6 runs do not cycle; they wander (5,400 distinct layouts in the last 6,000 iterations). |
| P7 | N = 10^47.7 nodes per solution for a row scan of the official board; "33 to 34 orders short" | wrong | 0.03 | See section 2. It is not the one-clue exhaustion figure under another convention. It is the evenly spread seam probability again, the error `price-and-ledger.REVIEW.md` found in path 2. |
| P8 | The largest published difference-map result on edge matching is TetraVex 4x4 | holds with a correction | 0.8 | The quoted sentence is in arXiv:1305.2657 (PDF read): "Problems up to size 4 × 4 could be solved in an average of 200 iterations", citing a 2010 student report. I opened the PMC copy of the 2007 PNAS paper the memo could not open: it has no edge-matching result. Unpublished: the site's own page records the method's author sharing code with a worked 5x5 example (list message 9362). Published, different continuous method: Kovalsky et al. report 12x12 instances solved by sequential LP relaxations, which the memo cites from the abstract only. |
| P9 | "Every instance tried was solved exactly, up to 8x8. The published figure for this family is 4x4" | holds with a correction | 0.7 | One 8x8 instance, two starts, 5.4 colours above critical. On the two near-critical instances the memo had a DFS reference for and skipped (5x5, 4 colours, +0.4), the map also solves 10 of 10 starts with ratios 0.07 to 0.21 (`extra.out`, part `near`), so there is no sign of selection at n = 5. Nothing was run near critical above n = 5. |

## 2. The most serious error

**`solve-path-6/profile.py` lines 13 to 14 and 33 (docstring line 7: "Each
family's log-probability is spread evenly over its seams"), carried
into `solve-path-6-free.md` lines 76 to 83, 85 to 88, 94 to 96 and 106 to
108.**

The script charges every seam the geometric mean of the full-board match
probability (1/16.78 inner, 1/4.39 ring). The exact probability of the seams
closed so far is higher at every depth before the end, because the last seams
of each colour are nearly free. `price-and-ledger.REVIEW.md` section 2 found
this in path 2 and measured the error at 4.9 digits after 160 cells.

`price_check.py` runs the price referee's library both ways:

| Clues | Model | log10 tree | log10 solutions | log10 nodes per solution |
|---|---|---|---|---|
| none | spread (this is `profile.py`) | 54.79 | 7.06 | 47.72 |
| none | exact | 50.03 | 7.06 | 42.97 |
| one | exact, row 15 first | 47.14 | 4.17 | 42.97 |
| five | exact, row 15 first | 40.49 | -7.39 | the tree is the cost: 40.49, 40.19 to a first solution |

So:

- 10^47.72 reproduces to the digit as the spread model. The exact figure for
  the same quantity (nodes per expected solution, no clue or one clue) is
  10^42.97. The memo is 4.75 digits high.
- It is not the one-clue exhaustion figure 10^47.14. The two are within 0.6
  digits by accident: the spreading error (4.75) is close to log10 of 14,702
  (4.17).
- The memo checks its script by the solution count (14,702). That is the one
  depth where spread and exact agree, so the check cannot see the error.
- The puzzle to solve has five clues. Against the reconciled price (10^40.19
  nodes to a first solution) the memo's N is 7.5 digits high.

Corrected consequences, keeping the memo's own ρ and budgets: I = 10^38.9 to
10^40.0, so **26 to 27 orders short at 1,000 core-years (memo: 33 to 34), 27
to 28 on three desktops (memo: 35)**. With its "favourable trend" 10^33
iterations, 20 orders short (memo: 28). In the speculative case I = N^(1/4) at
k = 4: 10^10 iterations, about 100 core-years, inside the 1,000 core-year
budget, where the memo says ten times over it. That last change is the only
one that points the other way, and it rests on a hypothesis the memo gives 2
to 3 percent and did not test. The interior hump (10^32.9) and "branching
exceeds 1 until 70 pieces remain" come from the same spread probability and
are also off.

The memo's conclusion (the map follows the tree at k ≤ 1 and is far out of
reach) survives. Its headline number does not.

Second, in `literature-2026.md`: lines 561 to 563 ("row-major scanning is
optimal for a structural reason") contradict the cited manuscript, and lines
23 to 29 date the five-clue 466 to November when the memo's own source 10
carried it on 8 October.

## 3. Notes on method

**Literature, absence claims.** Queries I added to the memo's: the puzzle name
with MaxSAT, weighted CSP, Max-Clique, partial MaxSAT; toulbar2 or cost
function network with the puzzle name, edge matching, Tetravex; decision
diagram or MDD with edge matching, Tetravex, jigsaw; Tetravex or edge matching
with MaxSAT, pseudo-Boolean, answer set programming; the puzzle name with
reinforcement learning, tree search, transformer (English and French, standard
and extended modes); edge matching with reinforcement learning, graph network,
diffusion on arXiv; the puzzle name with QUBO, Ising machine, digital
annealer; peer-reviewed 2019 to 2026 metaheuristics; and the arXiv API for
"Eternity II", "edge-matching" + puzzle, "edge matching puzzle", tetravex,
sorted by date. Nothing after 2018 beyond what the memo lists. Sources I could
not open: the AAAI publisher page (bot wall; replaced by the authors'
preprint), the authors' poster (certificate error), one JFPC 2025 PDF (403),
the PNAS publisher page (403; replaced by the PMC copy).

**Path 6, baseline.** The naive scan's nodes are not a first-row artefact: row
0 holds 2 to 11 percent of the tree, rows 0 and 1 hold 28 to 85 percent. Nodes
to a first solution under shuffled piece orders have a median of 0.33 to 0.77
of "tree nodes / solutions", so the memo's reference is fair as a naive
reference. It is not fair as "a backtracker": the first step toward one
(smallest domain first) removes most of the map's advantage in counts, before
any cost per iteration is charged.

**Path 6, what "exact" covers.** The projections are exact for the sets as
defined (the assignment maximises a linear score over one-hot layouts; the
Viterbi pass is a correct max-plus chain with the rim mask). The step used is
the "form a" update with β = 0.25, which matches the memo's formula. Clue
cells are described in the memo and not implemented in `dm.py`; no run used a
clue.

## 4. What the program may rely on tomorrow

1. The site's two bibliography entries are wrong and can be fixed from the
   sources opened here: `rotations2017` is by Ebbesen, Fischer, Witt;
   `kovalsky2014` is "A Global Approach for Solving Edge-Matching Puzzles",
   Kovalsky, Glasner, Basri, SIAM J. Imaging Sciences 8(2), 916 to 938, 2015,
   and its blurb needs rewriting (edge matching, 12x12 solved, 16x16 not).
2. The strict five-clue 465 on the site is probably out of date. A 466 has
   been public since at least 8 October 2026 and a BOINC dashboard reports
   more. No board has been verified by this program. Verify one before
   changing any number, and do not write "November".
3. The BOINC page reports 886 G nodes per second. Treat it as a dashboard
   figure with an undefined node.
4. The toulbar2 AllDifferent paper exists and says what the memo says. Whether
   it reaches sub-board edge matching (1,024 values per cell) is untested and
   outside its published sizes.
5. The planted-jigsaw manuscript supports "a rectangle grown from a corner with
   both sides at least two is easy in the unique-solution regime; edge first,
   centre block and corner triangle are not". It does not support "row-major
   is optimal": it says a first row alone grows exponentially.
6. "Never tried" for weighted CSP, MaxSAT beyond the project's own, decision
   diagrams and pseudo-Boolean on edge matching: no counterexample after two
   independent searches. Good enough to open a row as "open, none found", not
   as "never tried anywhere".
7. The row-scan price is 10^40.49 nodes with five clues (10^40.19 to a first
   solution), 10^47.14 with one clue, 10^42.97 per expected solution. Path 6's
   10^47.7 is not a third figure; discard it.
8. Path 6's measurements reproduce exactly. What they show: on over-coloured
   planted boards up to 7x7, the relaxed map (β = 0.25) reaches an exact
   solution in 0.05 to 0.6 times the node count of a naive scan, and about
   0.1 to 3 times that of a smallest-domain-first scan, at 10^3 to 10^4 node
   costs per iteration. Plain Douglas-Rachford (β = 1) falls into period-2
   cycles. No trend with n is established and no instance was near critical
   above 5x5.
9. If path 6's first experiment is run, its instances must be the reviewed
   critical rungs (6x6 with 5, 7x7 with 6, 8x8 with 8 colours, five frame
   colours) and its baseline `bb-mrv`, as its own section 6 already says. The
   memo's tonight data does not stand in for that.
