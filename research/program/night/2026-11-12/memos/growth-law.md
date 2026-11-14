# Growth law for search on planted boards: what to expect, and which mechanisms can change it

Night 2026-11-12. Memo only: derivation plus small numerical checks against committed data.
Scripts: `research/program/night/2026-11-12/memos/growth-law/` (stdlib Python, `python3 -I <script>`).
Generated boards only. Nothing here is a result about the official 16x16 puzzle; section 5 says what the model implies for it and under which assumptions.

## 1. Answer in ten lines

1. For plain depth-first search with a static fill order, the expected number of nodes at depth d is the first-moment count of consistent partial boards, A(d) = (ways to choose and turn the pieces) x (1/c per closed interior seam) x (1/5 per closed frame seam), plus one copy of it hanging off every cell of the planted path. With no fitted parameter this predicts the measured median of row-major DFS within a factor 0.84 to 1.9 on 14 above-critical (n, c) rungs from 6x6 to 8x8 (section 4).
2. A(d) depends only on the set of filled cells. Its logarithm splits into a bulk term P g(x), with P = (n-2)^2, x = c^2/(4P), g(x) = x - 1 - ln x, and surface terms of order n ln n (frame entropy, perimeter of the filled region). Fill order moves only the surface terms.
3. At ladder sizes the surface terms are as large as the bulk term (n = 8: frame ln 4e5 = 12.9 against P/e = 13.2; n = 16: 79 against 72). Growth measured over n = 6..8 is therefore mostly surface, and a slope in n^2 fitted there does not estimate the bulk rate.
4. The one-expected-solution colour count of the whole board (frame included) is not Owen's interior count on small boards: 3.9, 5.3, 6.6, 7.9, 9.3 at n = 6..10 against Owen 5.2, 6.4, 7.6, 8.7, 9.9. They coincide at n = 16 (17.11 against 17.14) because the 5-colour frame happens to cancel there. Every ladder rung, including the "critical" ones, is 0.75 to 3.4 colours above critical. The 16x16 with 17 + 5 colours is at critical (ln S = 2.8 in the uniform-colour model).
5. At the critical count the bulge peaks when a fraction 1 - 1/e = 0.63 of the interior is filled, independent of n to leading order (0.75 at n = 6, 0.64 at n = 16), with bulk rate 1/e nats per interior cell.
6. Local propagation and cell ordering act on the frontier, which has O(n) cells, so in the model they change the count by exp(O(n)) at most: a surface factor, not a change of the bulk rate. Any reasoning that relaxes piece distinctness cannot refute nodes before the peak at all.
7. The model of propagating solvers that I tried (static fail-first order plus lookahead) fails: it does not reproduce `bb-mrv` or scb, by factors of 10 to 200. The classification for those solvers rests on the argument in 6 and on measured ratios, not on a validated model.
8. Measured ratio of plain row-major DFS to `bb-mrv` nodes on the +2 track: 12, 14, 7 at n = 6, 7, 8, and at most about 3 at n = 9 (model DFS over tonight's pilot). The gain does not grow with n. The "gain grows with n" lead of A22 is against scb, whose border-first order pays the number of valid frames (5, 1e3, 4e5, 3e8 at n = 6..9).
9. In the model only two families change the bulk rate: collapsing states that share a frontier (blocked by piece distinctness), and two-sided search, which lowers the rate at critical from 1/e to (ln 2)/2 per interior cell (6% lower) at a memory cost equal to its time.
10. Prediction written before reading the 9x9 pilot: `bb-mrv` median 2.3e7 nodes, 6 to 8 of 8 solved under 1e8; scb 0 or 1 of 8. Outcome on the 10 committed rows: scb 0 of 4 (as predicted), `bb-mrv` 2 of 6 with median above 1e8. The `bb-mrv` prediction was too low by a factor of 4 or more.

## 2. Model and derivation

### 2.1 Setting and notation

Board n x n, N = n^2 cells, w = n - 2, P = w^2 interior pieces, 4w edge pieces, 4 corners. The generator (`engine/src/generator.rs`, `paint_framed`) paints the 4(n-1) seams that join two rim cells along the rim with cf = 5 frame colours, and the other E_I = 2w(w+1) seams with c interior colours, each band cycled then shuffled, so every colour of a band is used equally often up to one. Pieces are then shuffled and turned (`hard-ladder/compute/src/instance.rs`).

A fill order is a sequence of cells. At step t the cell has class (corner, edge, interior), r_t unused pieces of that class, rho_t orientations that respect the rim (1 for corner and edge cells, 4 for interior cells), a_t already filled neighbours across interior-colour seams and f_t across frame-colour seams.

- b_t = r_t rho_t c^(-a_t) cf^(-f_t): expected number of candidates for cell t that agree with the filled neighbours.
- A(d) = prod_{t <= d} b_t: expected number of consistent partial boards on the first d cells, for pieces whose seam colours are independent and uniform (the random ensemble).

### 2.2 What is exact

For DFS without propagation, the nodes at depth d are exactly the consistent partial boards on the first d cells (every such board has a consistent prefix, so the search reaches it). The node metric of the ladder counts one node per assignment that agrees with the rim and the assigned neighbours (`hard-ladder/PROTOCOL.md`), so for `hp-dfs` run to exhaustion, nodes = sum_d Z_d with Z_d the number of consistent partial boards at depth d. E[Z_d] = A(d) holds exactly in the random ensemble with independent uniform colours. Everything below this line is an approximation of the planted ensemble.

A(d) depends only on the set S of filled cells: A(S) = (falling factorials of the three classes) x 4^(interior cells in S) x c^(-e_I(S)) x cf^(-e_F(S)), with e(S) the seams inside S. A fill order is a chain of sets, nothing more.

### 2.3 The planted ensemble

Three corrections, in decreasing order of importance.

**(i) The planted path and its hanging trees.** A planted board has four solutions for free: the board turned by quarter turns (checked: every one of 84 exhaustively enumerated ladder instances has 4, 8 or 16 solutions, `growth-law/data/full_*.jsonl`). With a corner as first cell, each of the four corner pieces starts one planted path. Along one path, at depth j the next cell has the planted candidate plus wrong candidates, and below a wrong candidate the board is treated as generic:

H_j(d) = b'_{j+1} prod_{t = j+2..d} b_t, the expected nodes at depth d that follow one planted path for j cells and leave it at cell j+1,

Z_planted(d) = 4 (1 + sum_{j = 1..d-1} H_j(d)) + H_0(d).

**(ii) Wrong candidates next to planted neighbours see a depleted urn.** A band has E seams, E/c of each colour. Next to a planted neighbour the planted partner holds one of the E/c seams of the wanted colour, so a different piece matches that seam with probability q1 = (E/c - 1)/(E - 1), not 1/c. Hence b'_{j+1} = (r rho - 1) q1_I^a q1_F^f. This matters on small boards (frame at n = 6: 3/19 against 1/5). I added it after seeing that the first version overshot the hanging trees; it has no fitted parameter, but it was chosen after looking at residuals.

**(iii) Inside a hanging tree I keep 1/c per seam.** This is two errors that roughly cancel on the boards checked. A generic piece matches a given half-edge with probability about (2E/c - 1)/(2E - 1), below 1/c. But each filled neighbour also has its own planted partner piece, which matches that seam for sure and the other a - 1 seams by chance. For the first interior cell at 6x6 c = 7 the two together give 1.27 expected candidates (derivation in section 3), against 1.31 for 1/c and 1.14 for the depleted urn alone; measured 1.26.

**First solution.** DFS explores, at each depth j of the planted path it ends on, the wrong consistent siblings that precede the planted candidate in value order. With a uniformly random value order that is half of them on average:

T_first ~ N + (1/2) sum_{j >= 1} W_j, W_j = sum_{d > j} H_j(d),

when other solutions are rare. `model.py` (`first_solution_terms`) also handles other solutions: a wrong sibling holds a solution with probability p_j = sigma_{j+1} / (1 + expected further solutions below it), sigma being the expected solutions below a generic node, and the search stops at the first solution-holding sibling. That part is a heuristic and it fails below critical (section 4).

Because W_j falls roughly geometrically with j, T_first is dominated by the first few branching cells: T_first is about u W_1 with u roughly uniform on (0, 1). Value-order luck alone therefore spreads a single run over a factor 3.3 between quartiles (section 6, result a).

### 2.4 Corrections neglected

- Correlations between seams beyond (ii) and (iii). The exact planted first moment is a sum over placements of c^(cycles - seams), where "cycles" counts closed chains of planted seams reused by the placement. I keep only the planted path. The neglected terms are positive and grow toward full depth. They explain why the real count of valid frames is far above the annealed one: 24 x 56!/5^60 = 2.0e34 for the 16x16 frame against 4.05e37 counted on the official ring (`web/content/research/build/analysis/solution-counting.mdx`, Owen's census). Edmonds, Edmonds and Mertz prove the same effect for an edge chain (their Theorem 2).
- "Shadow" paths: after one wrong piece the search can continue with planted pieces until the row below. Size at most about N n b/c nodes, visible as a tail of 5 to 30 nodes per depth where the model says 4 (section 3).
- Fluctuations between instances (the model gives means; medians are obtained only over value orders).
- Unequal colour counts, the rotation-uniqueness repair of the generator, corners holding 4 of the frame half-edges.

### 2.5 Continuum form: bulk and surface

For interior cells with two filled neighbours and m unused interior pieces, b = 4m/c^2. Write x = c^2/(4P). Then, summing ln b from m = P down:

- the count rises while 4m > c^2 and peaks at m* = c^2/4, that is at interior fraction s* = 1 - x;
- the peak height is ln A* = P g(x) + surface terms, g(x) = x - 1 - ln x;
- the curvature at the peak is 1/m*, so the bulge has width sigma = c/2 cells: less than one row of the board;
- x >= 1 (c >= 2w): no interior bulge at all. x = 1/e: g = 1/e, and P! 4^P c^(-2P) = 1, which is Owen's criterion.

Check of the continuum against the exact product (`regime.py`, table A): at the critical count, ln A*/P is 0.83, 0.58, 0.47, 0.42 at n = 8, 16, 32, 64, falling toward 1/e = 0.368 as the surface share shrinks; the interior fraction at the peak is 0.67, 0.64, 0.63, 0.63.

Surface terms. (a) Frame entropy: the frame alone admits F_n = 24 (4w)!/5^(4(n-1)) placements in the annealed count: 5.3, 980, 4.0e5, 3.2e8, 4.3e11 at n = 6..10, 2.0e34 at n = 16. With 5 frame colours at every size the frame is far below its own critical count, and the order decides when this entropy is paid. (b) Perimeter: for any set S of interior cells inside a filled frame, e(S) <= 2|S| + 2w, so no order closes more than 2w seams ahead of the bulk count. Both are exp(O(n ln n)).

### 2.6 Relation to two published results (read tonight)

- A. Martinsson, "A linear threshold for uniqueness of solutions to random jigsaw puzzles", Combin. Probab. Comput. 28 (2019) 287-302, arXiv:1701.04813, DOI 10.1017/S0963548318000391 (abstract page read; the body was not). Uniform model, no frame palette: at least two non-similar solutions for q <= (2/sqrt e) n, all solutions similar for q >= (2 + eps) n. With side w and q = c these are exactly x = 1/e and x = 1 of section 2.5. So the two thresholds bracket a phase, 1/e < x < 1, where the solution is unique in the first-moment sense and the search still has an exponential bulge with rate g(x). That phase is where every ladder rung sits.
- J. Edmonds, A. Edmonds, I. Mertz, "Search Time when Solving Random Jigsaw Puzzles with a Planted Solution", undated manuscript, https://www.eecs.yorku.ca/~jeff/research/tsot/puzzle/puzzle.pdf (pages 1 to 6 read; no venue or DOI found). They take q = sqrt(4n/eps) with n the number of pieces, so their eps is 4N/q^2 = 1/x, and they work at eps < 1, that is x > 1. There they prove expected width 1 + Theta(eps) for a rectangle grown from a corner (Theorem 5), and blow-ups of 2^Theta(sqrt(eps) side) for an edge-first order, n^Theta(log n) for a block in the middle and 2^Theta(eps side) for a corner triangle (Theorems 2 to 4). This sharpens my statement rather than contradicting it. In their regime the bulk rate g is zero, so the surface term is the whole exponent and order decides between linear and exponential time. Their blow-ups are all exp(O(side)), the surface class of section 2.5. Their Lemma 1 has the same "1 + planted partner" terms as (iii). Their Theorem 6 (more compatible pairs, exponentially many solutions, exponential time for every order) is the regime x < 1.
- Which regime: 1/x is 1.3 to 2.9 on the ladder rungs and 2.71 on the 16x16 (`regime.py`, table B). None is in the easy regime of Edmonds et al. (1/x < 1). The 16x16 sits at Martinsson's lower threshold. The corner-rectangle result does not carry over to any rung the program uses; it would apply to 8x8 from c = 12 up, where the measured DFS cost is indeed flat at 2e4 to 4e4 nodes (section 4) and equals the cost of the first frame row.

## 3. Numerical checks of the derivation

All on one core. CPU used: about 4 s for the ladder enumerations, 81 s for the Monte Carlo (more than intended; the 5x5 c = 3 set is the cost).

**Check 1: my DFS is the ladder's DFS.** `dfs_check.py` reproduces committed `hp-dfs` first-solution node counts exactly (9,147, 61,250, 77,730 on `n6-c5-s1..3`, `hard-ladder/results/test/primary/hp-dfs.jsonl`).

**Check 2: whole trees on committed instances** (`dfs_check.py` then `compare_full.py`; exhaustive row-major trees, per-depth counts averaged over instances).

| n | c | instances | full tree, measured mean | model | ratio | first solution, measured mean | model mean | measured median | model median | max abs ln ratio of Z_d up to the peak | past the peak |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 6 | 7 | 30 | 7,354 | 7,197 | 1.02 | 830 | 918 | 700 | 759 | 0.19 | 0.49 |
| 6 | 6 | 30 | 34,734 | 34,110 | 1.02 | 3,277 | 4,282 | 2,404 | 3,492 | 0.25 | 0.43 |
| 6 | 5 | 12 | 464,552 | 648,991 | 0.72 | 53,808 | 81,141 | 42,612 | 66,702 | 0.42 | 0.58 |
| 7 | 8 | 12 | 398,082 | 476,395 | 0.84 | 46,706 | 59,574 | 41,466 | 52,055 | 0.37 | 0.58 |

The depth profile is within a factor 1.5 up to the peak and 1.8 past it. The model overshoots more as c approaches critical (0.72 at 6x6 c = 5). The measured first-solution cost is about full/9; the model gives about full/8.

**Check 3: fresh boards from an independent generator** (`mc_small.py`: Python copy of the painter, 600 to 3,000 boards per setting, whole tree enumerated). Ratio of measured to model full-tree size: 1.09 (5x5 c = 6), 1.18 (5x5 c = 4), 1.26 (5x5 c = 3), 1.23 (4x4 c = 3), 1.80 (4x4 c = 2). Up to the peak the per-depth ratio is 0.74 to 1.27; the frame row is 17% to 22% below the model at 5x5 (corners hold 4 of 16 frame half-edges, which the model ignores); past the peak the measured counts exceed the model increasingly, up to the solution count (7.6 to 181 measured against 4). Most of that excess is an artefact of my generator, which does not repair duplicate pieces, so duplicates multiply solutions; the rest is the neglected cycle terms of 2.4. Sign of the error is opposite to check 2. Taken together: the model's full-tree mean is within a factor 2 on 9 settings, with no consistent sign.

**Check 4: the partner term, derived then measured.** First interior cell at 6x6 c = 7, two filled neighbours (inward sides of two edge pieces). Generic pieces: 62 x (E_a - 1)(E_b - 1)/((E - 2)(E - 3)) with E = 40, E_a = 40/7 gives 0.98. The two planted partners of the neighbours: each matches one seam for sure and the other with probability about 0.146, adding 0.29. Total 1.27. Measured ratio Z_8/Z_7 on the 30 instances: 679.6/539.5 = 1.26. Plain 1/c gives 1.31.

**Check 5: continuum against exact product.** Section 2.5, table A of `regime.py`.

What was not checked: any board above 7x7 by exhaustive enumeration; the first-solution heuristic below critical; any propagating solver.

## 4. Model against committed data

Medians recomputed from the raw rows by `medians.py` (Kaplan-Meier median as in `hard-ladder/README.md`); 8x8 colour sweep from `research/topics/hardness-peak/results/hardness-peak-8x8.json` (placements, 1.5e7 budget). Model column: `table_hp.py`, median over 2,001 random value orders with mean subtree sizes. No parameter is fitted.

### 4.1 Plain row-major DFS (`hp-dfs`)

| n | c | c minus whole-board critical | x | model median | measured KM median (solved/N), source | model / measured |
|---|---|---|---|---|---|---|
| 6 | 5 | +1.1 | 0.39 | 6.6e4 | 41,823 (30/30) hard-ladder | 1.6 |
| 6 | 6 | +2.1 | 0.56 | 3.5e3 | 2,374 (30/30) hard-ladder | 1.5 |
| 6 | 7 | +3.1 | 0.77 | 790 | 700 (30/30) mrv-baseline | 1.1 |
| 7 | 6 | +0.8 | 0.36 | 8.2e7 | above 1e6 (0/30) hard-ladder | consistent |
| 7 | 7 | +1.8 | 0.49 | 9.3e5 | 693,573 (20/30) hard-ladder | 1.3 |
| 7 | 8 | +2.8 | 0.64 | 4.9e4 | 44,926 (30/30) mrv-baseline | 1.1 |
| 8 | 7 | +0.4 | 0.34 | 5.5e11 | above 1.5e7 (0/30) hardness-peak | consistent |
| 8 | 8 | +1.4 | 0.44 | 1.2e9 | above 1.5e7 (0/30) hardness-peak | consistent |
| 8 | 9 | +2.4 | 0.56 | 1.6e7 | 8.47e6 (23/30) hardness-peak | 1.9 |
| 8 | 10 | +3.4 | 0.69 | 8.8e5 | 483,034 (24/30) mrv-baseline; 509,862 (19/30) cut-multiset; 5.1e5 (30/30) hardness-peak | 1.7 to 1.8 |
| 8 | 11 | +4.4 | 0.84 | 1.55e5 | 1.23e5 (30/30) hardness-peak | 1.3 |
| 8 | 12 | +5.4 | 1.00 | 5.7e4 | 4.02e4 (30/30) | 1.4 |
| 8 | 13 | +6.4 | 1.17 | 3.5e4 | 4.10e4 (30/30) | 0.86 |
| 8 | 14 to 17 | | 1.4 to 2.0 | 2.5e4 to 1.8e4 | 2.7e4 to 1.7e4 (30/30) | 0.84 to 1.08 |
| 9 | 11 | +3.1 | 0.62 | 3.2e8 | above 1e6 (0/30) mrv-baseline | not tested |
| 10 | 12 | +2.7 | 0.56 | 5.3e11 | above 1e6 (0/30) mrv-baseline | not tested |

Where it holds: 14 solved rungs above critical, ratio 0.84 to 1.9, with a bias that grows toward critical (the same bias as in check 2). The flat floor from c = 12 at 8x8 is the first frame row, as the model says.

Where it fails:

- **Below critical it is unusable.** At 8x8 c = 1 to 5 the model gives 1e10 to 1e54 while 11 to 27 of 30 boards solve within 1.5e7 placements (c = 4: 27/30, median 8.9e5). The model has no account of how DFS finds one of very many clustered solutions. The measured curve is also not monotone there (12/30 at c = 5, 27/30 at c = 4), which I cannot explain.
- **It cannot locate the measured hardness peak**, because every board at c = 6, 7, 8 is censored. The model's own maximum is at c = 6 (ln S = +8), one below the whole-board critical value 6.6.
- The 9x9 and 10x10 rows are extrapolations with no measurement behind them.

### 4.2 Propagating solvers: the model fails

I tried two models of propagation (`lookahead.py`): a greedy fail-first static order with a lookahead of L cells (nodes at depth d taken as the minimum of the profile over the next L depths), and forward checking on the whole frontier (each frontier cell nonempty, Poisson candidates, independent cells), on row-major, greedy, border-first and diagonal orders.

| n | c | best of the lookahead variants (greedy, L = 8) | frontier forward checking, greedy | border-first, forward checking | measured `bb-mrv` | measured scb |
|---|---|---|---|---|---|---|
| 6 | 5 | 310 | 1.55e3 | 2.1e3 | 400 | 951 |
| 6 | 7 | 405 | 805 | 1.5e3 | 57 | 106 |
| 7 | 6 | 3.2e7 | 5.6e7 | 3.7e5 | 588,305 | 921,773 |
| 7 | 8 | 3.2e4 | 6.4e4 | 1.8e5 | 3,218 | 7,088 |
| 8 | 10 | 5.8e5 | 1.4e6 | 4.6e7 | 69,036 | 555,342 |

No variant is within a factor 5 across the rungs; errors reach 10 to 200. A static order with independent frontier cells does not describe what arc consistency, hidden singles and a dynamic choice of cell do together. I have no validated model of a propagating solver. What I can report is the measured ratio to plain DFS, with the model standing in for DFS where DFS is censored:

| n | c | x | DFS median / scb median | DFS median / `bb-mrv` median | scb / `bb-mrv` | annealed frames F_n |
|---|---|---|---|---|---|---|
| 6 | 5 | 0.39 | 44 | 91 to 105 | 2.1 to 2.4 | 5.3 |
| 7 | 6 | 0.36 | about 90 (model DFS) | about 140 (model DFS) | 1.6 | 980 |
| 6 | 6 | 0.56 | 7.9 | | | 5.3 |
| 7 | 7 | 0.49 | 8.9 | | | 980 |
| 6 | 7 | 0.77 | 6.6 | 12 | 1.9 | 5.3 |
| 7 | 8 | 0.64 | 6.3 | 14 | 2.2 | 980 |
| 8 | 10 | 0.69 | 0.87 | 7.0 | 8.0 | 4.0e5 |
| 9 | 11 | 0.62 | below 3 (model DFS, pilot) | at most about 3 (model DFS, pilot) | not known | 3.2e8 |

(`bb-mrv` at 6x6 c = 5: 460 on 10 seeds in mrv-baseline, 400 on 20 seeds in backdoor-branching. The model-DFS entries carry the model's 1.1 to 1.9 upward bias.)

### 4.3 The 9x9 c = 11 prediction

Written before reading any pilot row. What I already knew, and it contaminates the prediction: A22's rows (0/30 for both solvers at 1e6, and censored between 1.3e6 and 4.6e6 at the extended cap, `mrv-baseline/README.md`), and from tonight's STATUS.md that one `bb-mrv` run had solved in 9,970,108 nodes while 15 were still running.

- Prediction for `bb-mrv`: model DFS median 3.2e8, divided by the model's bias (about 1.4) and by the measured gain over DFS on the +2 track (7 to 14): median 2.3e7 nodes, 50% interval 1e7 to 5e7, 6 to 8 of 8 solved under 1e8.
- Prediction for scb: border-first pays the frames, F_9 = 3.2e8, so 0 or 1 of 8 solved under 1e8.

Outcome (branch `night/2026-11-12-t1-9x9-pilot`, files `results/raw/wave1-mac-m4.jsonl`, `results/remote/machine-b/raw/wave2-machine-b.jsonl`, `wave3-machine-b.jsonl`, `results/remote/machine-c/raw/wave4-machine-c.jsonl`; 10 of 16 rows committed when read):

- scb: 0 of 4 solved, all censored at 1e8. As predicted.
- `bb-mrv`: 2 of 6 solved (9,970,108 and 78,349,212), 4 censored at 1e8. The median is above 1e8 unless both missing seeds solve, in which case it is the fourth order statistic, at least 7.8e7. **My prediction was too low by a factor of at least 3.4, more likely above 4.** The error is in the assumed gain over DFS: it is not 7 to 14 at 9x9 but at most about 3. The DFS column itself is untested at 9x9.

## 5. Critical colours and the real puzzle's regime

**Why the hardest boards are at or just below the one-expected-solution count.** Above critical, T_first ~ (1/2) sum_j W_j is a fixed fraction of the bulge, and the bulge shrinks by a factor c^(seams closed at the peak) with every added colour: steeply. Below critical, the search stops at the first of S solutions, so cost falls roughly as bulge/S, and bulge/S = A*/A(N) rises with c. The maximum is where S is of order 1. The model reproduces the steep side quantitatively (8x8: 1.2e9, 1.6e7, 8.8e5, 1.6e5 at c = 8, 9, 10, 11; measured 8.5e6, 5e5, 1.2e5 at c = 9, 10, 11) and the other side only in direction.

**Where "critical" is.** Counting the whole board, S = F_n x P! 4^P c^(-2w(w+1)). The interior has 2w more seams than Owen's 2P (the inward sides of the edge pieces) and the frame multiplies S by F_n. Solving S = 1 (`model.py`, `critical_c`; `regime.py` table A):

| n | 6 | 7 | 8 | 9 | 10 | 12 | 16 |
|---|---|---|---|---|---|---|---|
| Owen (interior only) | 5.22 | 6.38 | 7.56 | 8.74 | 9.93 | 12.33 | 17.14 |
| whole board, 5 frame colours | 3.91 | 5.25 | 6.60 | 7.94 | 9.27 | 11.91 | 17.11 |

This explains the ladder's own finding that the hard band sits "at or below Owen's criterion" (`hard-ladder/README.md`): on small boards the whole-board critical count is 0.7 to 1.3 colours lower. The ladder's primary rungs (6x6 c = 5, 7x7 c = 6, 8x8 c = 8) are +1.1, +0.8, +1.4 colours above it, with ln S = -9.9, -8.0, -16.1. The +2 track is +3.1, +2.8, +3.4, +3.1, +2.7 above. No rung of the program is at critical in the whole-board sense.

**Why stronger propagation "helps above critical and not at it".** The model does not support that reading. Against plain DFS, propagation helps more near critical: 44x to about 100x at x near 0.37, 6x to 14x at x from 0.5 to 0.8 (table in 4.2). The observation is about `bb-mrv` against scb, and it confounds colour count with size: the gain is 1.6 to 2.4 at n = 6 and 7 at both colour counts, and 8 at n = 8. My explanation is the frame. scb places the border first, so it pays the valid frames, whose annealed number is 5, 980, 4e5 at n = 6, 7, 8 and does not depend on c. At 8x8 c = 10 the interior is easy (P g(x) = 2.1 nats) and scb's 5.5e5 nodes are of the order of F_8 = 4e5; at 6x6 and 7x7 the interior dominates and the two solvers are close. `mrv-bf` (border-first inside `bb-mrv`) shows the same step: 1.15, 1.10, 1.83 at R6, R7, R8 (`mrv-baseline/README.md`). The pilot agrees in direction (scb 0 of 4 at 9x9). I put 0.6 on this explanation; the cheapest test is scb at 8x8 for c = 12 to 17, where the model says scb stays above about 1e5 nodes while `bb-mrv` falls toward N.

**Peak position at critical.** At the whole-board critical count the peak of A(d) for row-major is at 64%, 67%, 61%, 64%, 64%, 63% of the cells at n = 6, 7, 8, 9, 10, 16, and the interior fraction filled there is 0.75, 0.76, 0.67, 0.69, 0.67, 0.64, tending to 1 - 1/e = 0.632 (0.629 at n = 64). The bulge is c/2 cells wide.

**The 16x16 with 17 + 5 colours, in the model's terms.** x = 0.369 = 1/e to three digits; c minus whole-board critical = -0.11; ln S = 2.8 with uniform colours and no clues. So it is at critical, on the many-solutions side by a tenth of a colour. The site's canonical ensemble estimate with the real colour statistics is larger (about 14,702 with the centre clue, about 1 with all five, `solution-counting.mdx`); each clue divides S by about 784, so with five clues the real puzzle is again at S of order 1. Either way it is at critical, which no ladder rung is. The peak of the row-major bulge is at depth 161 of 256 with ln A* = 115 (10^50), of which the bulk term P/e is 72 and the rest is frame entropy released by that depth.

What must be true for these statements to transfer to the real puzzle: colours close to uniform within each band (the real set is nearly balanced but not exactly); the planted corrections of 2.4 staying of surface order; and the first-solution heuristic being right at S of order 1, which is exactly where it is least tested (it is validated only for ln S below -8). I therefore make no claim about node counts on the 16x16. The claim I do make is about regime: the program's rungs are 1 to 3 colours above critical and the real puzzle is at critical, and on the model's steep side one colour is worth 0.7 to 1.9 decades at n = 8 (model medians at c = 8 to 11) and about 5 decades at n = 16 (`regime.py` table C: 10^44, 10^39, 10^34 at c = 18, 19, 20).

## 6. Classification of mechanisms

"Bulk rate" is the coefficient of P in ln(nodes); "surface" is exp(O(n ln n)); "constant" does not grow with n. Derived means derived in the first-moment model of section 2; argued means an argument I could not check against a validated model of the solver.

| family | predicted effect | why | status | cheapest test |
|---|---|---|---|---|
| Stronger local propagation (AC, singles, hidden singles) | surface factor at most; measured so far constant or shrinking | It refutes a node only if some bounded set of frontier cells has no filling. The frontier has O(n) cells, so the count drops by at most exp(O(n)). Before the peak, frontier cells have more than one expected candidate and survive. | argued; my quantitative model failed (4.2) | Plain DFS at 9x9 c = 11 with cap 1e9 to 1e10 (about 1e6 nodes/s, `mrv-baseline` tables), then the ratio DFS/`bb-mrv` at n = 7, 8, 9. Prediction: at most 3 at n = 9. A ratio that grows by a constant factor per added row would refute "surface at most". |
| Ordering of cells | surface factor, which can be astronomically large | A depends only on the filled set; any order closes at most 2w seams more than another at equal depth; order also decides when the frame entropy F_n is paid. Border-first pays F_n up front. | derived (bound), measured in direction (scb, `mrv-bf`) | scb or `mrv-bf` against `bb-mrv` at 8x8 for c = 12 to 17: the border-first arm should stay near F_8 while the free arm falls. |
| Ordering of values | constant factor, with large run-to-run noise | Cost is about u W_1 with u the rank of the right value among its siblings. An order that is not informed by the solution leaves u uniform. To change the rate a value order would need to rank the planted value first with probability rising with depth. | derived | Rank of the planted value among consistent siblings along the planted path (the harness has the planted board). Mean rank fraction 0.5 means no information. |
| Non-chronological backtracking, no-good learning | the model is silent: zero by construction | The model treats sibling subtrees as independent, so it has no conflict that skips levels. A gain would be evidence of structure the model lacks. Wipeout explanations are small in principle (two neighbours plus the cells holding the about 4P/c^2 pieces that would fit), which is the one reason I do not put this at zero. | not derived; 0.15 on a rate change | In Track 6's census, the fraction of dead nodes whose minimal conflict set skips at least one level, weighted by subtree size. |
| Region reasoning that relaxes piece distinctness (bands by transducers, colour-only checks, relaxed decision diagrams used as a strict filter) | constant factor, at most the share of work past the peak | With m unused pieces, relaxed completions of the rest number about (4m/c^2) per cell, above 1 exactly when the node is before the peak. Such nodes cannot be refuted, whatever the region size. | derived | Census checks A and E split by depth relative to the peak (interior pieces left = c^2/4). Prediction: no refutation before it. |
| Exact region reasoning (k x k windows or k rows with distinct pieces) | no gain without reuse; constant exp(e k^2/8) at critical with a k-row exact check; a rate change only through reuse | Cutting the peak by e^D needs a region of about c sqrt(2D) cells, and refuting it by enumeration costs the e^D it saves: lookahead is search. A check exact on k rows erodes the peak by exp(k^2 n^2/(8 x P)), constant in n: 1.4, 3.9, 21 for k = 1, 2, 3 at critical. Only tabulated refutations change the count, and their key includes the unused pool. | derived (first two), argued (reuse) | Recurrence rate of (boundary, pool) keys in the census, at 7x7 and 8x8. If it falls with n the gain is a constant. |
| Bounds from relaxations (flow, counting, LP) in strict search | constant | Same argument as relaxed regions. The count and parity conditions are identities on a consistent partial board of a planted set (each colour has an even number of half-edges and matched seams remove two), which is why A17 measured zero for them. | derived | Already measured (A17). |
| Decomposition, meet in the middle | changes the bulk rate, slightly, and only near critical | Two half-boards grown from opposite sides cost A at half depth each: rate (1/2)(ln 2 - 1 - ln x) against g(x). At x = 1/e: 0.347 against 0.368, 6% lower, a factor e^4.2 = 65 at P = 196, with memory equal to time. For x >= 1/2 the peak is before half depth and there is no gain, so it cannot be tested on the +2 track (x = 0.56 to 0.77). More than two parts must store or re-enumerate joins as large as the halves. | derived (continuum, not checked numerically) | Count consistent half-boards against the DFS peak on 6x6 c = 4 and 7x7 c = 5, the rungs nearest critical. |
| Restarts and randomisation | constant factor, at most about 2 either way | With cost u W_1 and u uniform, the chance to finish within a cutoff is proportional to the cutoff, so restarts conserve expected work. A gain needs a heavy tail over value orders, which the model does not produce. Heavy tails are reported for structured instances (Gomes, Selman, Crato, Kautz, J. Automated Reasoning 2000, cited from memory, not verified tonight). | derived in the mean-subtree model | Run-length distribution over 50 value-order seeds on 10 fixed instances at 7x7 and 8x8: is the log-variance growing with n, and is the hazard falling? |

**Result (a): value order moves node counts by about 2x across seeds, and random orders solve more 7x7 boards.** The first part is predicted. In the model at 8x8 c = 10 a single run spreads over a factor 3.3 between quartiles from value order alone, and the median of 30 paired ratios between two independent random orders has a 95% range of 0.60 to 1.84 (`regime.py`, part D; row-major profile, mean subtree sizes). The observed 1.62 is inside; 0.495 is just outside, and `bb-mrv` branches on smaller domains than row-major DFS, which widens the spread. A single-seed G1 against a fixed-order baseline is therefore inside the noise for any ratio between about 0.5 and 2. The second part (25 to 27 of 30 against 23) is not predicted and not surprising: the pieces are scrambled, so index order is one more random order, and it is the worst of four exchangeable orders with probability 1/4. Tracks 3 and 4 used the same instances, so they are not two independent confirmations. I put 0.25 on index order being systematically worse.

**Result (b): Hall matching on top of piece singles removes under 1% of nodes.** Predicted in direction. Before the peak every frontier cell has several candidates among m unused pieces, and a Hall violation beyond singles needs at least three cells confined to two pieces, of order k^3/m^4 per node for k constrained cells (below 1e-4 at the peak of 7x7). Violations concentrate near the end of the board where m is small and subtrees are a few nodes. The measured 0.85% of checks failing with only small subtrees cut (OBSERVATIONS.md, Track 4) has that shape. Régin filtering removes values, not only nodes, but acts on the same sets, so I expect a constant below 2 (0.7).

## 7. What I am unsure of, with probabilities

| claim | probability I would put on it |
|---|---|
| The first-moment model predicts median row-major DFS nodes within a factor 2.5 at 9x9 c = 11 (3.2e8) | 0.7 |
| The same at 10x10 c = 12 (5.3e11) | 0.55 |
| The whole-board critical count, not Owen's, is where the measured hardness peak of small generated boards sits (within one colour) | 0.65 |
| Local propagation and dynamic ordering change node counts by a surface factor at most, not the bulk rate | 0.8 |
| The ratio DFS/`bb-mrv` at 9x9 c = 11 is below 5 | 0.7 |
| scb's loss to `bb-mrv` at 8x8 and 9x9 is the frame enumeration of a border-first order | 0.6 |
| Relaxations without piece distinctness refute essentially no node before the peak depth | 0.85 |
| Two-sided search lowers the bulk rate by about 6% at critical and by nothing for x >= 1/2 | 0.6 (continuum algebra, unchecked numerically, joins and distinctness handled loosely) |
| No-good learning or backjumping changes the bulk rate on these boards | 0.15 |
| The 16x16 with clues is "at critical" in the sense S of order 1 and x = 1/e | 0.85 for the regime, no claim on node counts |
| The first-solution heuristic is right within a decade at S of order 1 | 0.3 |

Known weak points of the derivation: correction (ii) was chosen after seeing residuals; (iii) relies on a cancellation that I verified only at n = 5 to 7; the bias of the model grows toward critical (ratio 0.72 at 6x6 c = 5) and I do not know whether it keeps growing; all "surface" statements about propagating solvers are arguments, since my attempt to model them failed.

## 8. Three recommendations for the program

1. **Make plain row-major DFS the growth reference, and judge growth as a ratio to it at matched (n, c).** It is the only solver with a validated prediction, it runs at about 1e6 nodes/s, and its model value is a column anyone can compute (`table_hp.py`). Concretely: run `hp-dfs` at 9x9 c = 11 with a cap of 1e9 to 1e10 nodes on the pilot's seeds, then report DFS/method at n = 7, 8, 9. A method changes the growth law only if that ratio grows by a roughly constant factor per added row; a flat or falling ratio (12, 14, 7, at most 3 for `bb-mrv`) is a constant or less. Stop fitting a slope in n^2 over n = 6..8: there the surface terms are as large as the bulk.
2. **Label every rung by (x, ln S) and add a matched track.** With 5 frame colours no small board has both x = 1/e and S of order 1, which the 16x16 has. Choosing the number of frame colours per size so that the frame entropy cancels (about 3 at n = 6 to 8, 4 at n = 10, 5 at n = 16; from ln(24 (4w)!) - 4(w+1) ln cf = 2w ln c at Owen's c) gives rungs that are critical in both senses. Until then, state that the "critical" rungs are 0.8 to 1.4 colours above critical and the +2 track about 3 above, and do not read the +2 track as a fixed regime (its x goes 0.77, 0.64, 0.69, 0.62, 0.56).
3. **Split every pruning measurement at the peak depth, and require run-seed controls for anything that touches order.** The peak (interior pieces left = c^2/4) separates nodes that no distinctness-relaxed reasoning can refute from nodes that local propagation already refutes; a census share reported without that split cannot say whether a check could ever cut the bulge. For ordering arms, the model's noise floor for a median of 30 paired ratios is 0.6 to 1.8, so at least 5 run seeds per arm are needed before a G1 between 0.5 and 2 means anything. Drop border-first baselines above n = 8: they measure the frame count.

## Files

- `growth-law/model.py`: the model (profiles, planted corrections, first-solution estimate, critical counts).
- `growth-law/medians.py`: KM medians from the committed rows.
- `growth-law/dfs_check.py`, `compare_full.py`, `data/full_*.jsonl`: exhaustive row-major trees on committed instances.
- `growth-law/mc_small.py`: Monte Carlo on fresh 4x4 and 5x5 boards.
- `growth-law/table_hp.py`: section 4.1. `growth-law/lookahead.py`: section 4.2. `growth-law/regime.py`: sections 2.5, 5 and result (a).
