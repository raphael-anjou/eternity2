# Referee report on the theory pages

Night 2026-11-12. Scope: the 29 English pages under `web/content/research/why/`
plus `web/content/research/build/analysis/solution-counting.mdx` (30 pages).
Every "checked" below means the number was recomputed or the proof was followed.
Scripts and their outputs are in
`research/program/night/2026-11-12/memos/theory-referee/` (`01` to `10`, stdlib
Python run with `python3 -I`, except `05` and `05b` which use numpy through
`uv run --with numpy`; one core throughout). No page was edited.

## 1. Summary

1. Verdicts over 30 pages: 8 sound, 14 sound with a caveat to add, 3 overstated, 3 wrong, 2 unverifiable from the repo.
2. Wrong: `entropy-area-law` (the limit 0.67 and the 155-cell collapse), `parity-defect-floor` (the "at most 76" shell; the 479 theorem itself is correct), `how-hard-is-this-instance` (where planted boards cross one expected solution).
3. Overstated: `sigma-cycles`, `design-recipe`, `why-e2-is-hard`. Unverifiable: `rigidity-wall` (MIP table), `mismatch-geometry` (corpus).
4. Finding 1. The entropy limit is not 0.67. I recomputed h(1) to h(6): 1.6645, 1.0497, 0.8449, 0.7425, 0.6810, 0.6400. Fekete's infimum, the page's own theorem, gives h_inf <= 0.6400, and the row increments are 0.4352 to four digits, so h_inf is about 0.435 (mean field 0.4334). The 0.67 is the width-5 value read as the limit.
5. Finding 2. The distinctness decay is not an area law. The two committed exact points fit -ln rho = 0.0209 x (adjacent pairs) + C(m,2)/196 with the birthday coefficient coming out at 1.002. That puts rho below 1e-3 at about 46 cells, not 155, and agrees with the birthday term the solution-counting page already states.
6. Finding 3. "At most 76 one-move near misses" is wrong in count and in direction. Every near-twin pair (125 up to rotation: 50 interior, 73 edge, 2 corner) and every in-place turn keeping two sides (29) gives a 478 board from any solution, so the shell has at least about 154 boards. Checked on painted solved boards: the count of 478 neighbours equals the bag count each time.
7. Finding 4. `how-hard-is-this-instance` places the expected-one-solution crossing at N = 12 to 13 for 17 interior colours by dropping the 4(N-2) seam joints. With them the 12x12 expectation is 10^-28 and the crossing is at N = 16, which is what `phase-transition` says. The planted 10x10 to 14x14 boards are deep in the unique-solution regime, not "plausibly many solutions".
8. Finding 5. Two bag statistics are artefacts of the CSV's stored orientation: the 307-of-480 "fixed-orientation cap" (402 +/- 9 under random fixed orientations, 480 under a solution's) and the "114 near-twin pairs" used on three pages (the rotation-invariant count is 125).
9. Sound and verified exactly: 14,702.27 with the starter, 4.1e-8 with five clues, 1.153e7 without hints, the 784 ratio, the naive 0.0209 and its 2^19.4 gap, the 22 from free frame pairing, W_geom = 10^559.94, mu = 0.00944, BEST = 3432 x (11!)^5 = 3.478e41, flux ranks 22 / 44 / 40 / 21, the forbidden-pattern table, the sigma-cycle census (246 / 1,154 / 54,238), the ANOVA (F = 1.239, p = 0.254).
10. Null models: present and adequate on `no-forced-moves`, `piece-theft`, `forbidden-patterns`, `why-e2-is-hard`, `ring-purity`, `frame-is-not-the-basin`. Missing on `sigma-cycles` headline (a random class-preserving permutation gives a largest cycle of median 121; the corpus median is 119), `irreducible-hard-region` budget test, `phase-transition` 8x8 sweep.
11. Second moment: the solution-counting page says the planted bias has not been computed. Within the same independent-edge model it is 1 + E[Z] plus about 0.03 (small-overlap terms, dominated by border and corner swaps). On the real bag a fully specified hole has exactly zero alternative tiles, because no two pieces are rotation-duplicates; the 0.0094 "extra candidates" is an ensemble figure.
12. The frame count has three independent values that agree: Owen's culled search 10^37.61, the repo's BEST plus sampling 10^37.59 (5% s.e.), my census-exact first moment 10^37.71. The solution-counting page asks for an independent method and does not cite the one in the repo.
13. Upgrades done tonight: rigorous 0.067 <= h_inf <= 0.640 (lower bound by four toroidal 3x3 blocks with a common boundary); a census-exact annealed landscape that reproduces 10^7.06 at 480 and the parity gap at 479; the full-first-moment phase line c*(N).
14. Theory that could guide search: the complex-theory depth profile (it predicts 99.5% of nodes beyond depth 132 and 79% beyond 150; measured 99% and 70%), the phase line c*(N) for choosing benchmark boards, and the census-exact completion count as a node score.
15. Fifteen cross-page disagreements are tabled in section 3; several are one sentence repeated on the hub pages (`index`, `walls-and-methods`, `theorem-sweep`).

## 2. Page by page

Format: (1) central claim and status, (2) mathematics, (3) null model, (4) title against body, (5) cross-page, (6) verdict.

### build/analysis/solution-counting.mdx

1. First-moment models give about 14,702 solutions with the starter and about 4e-8 besides the planted one with five clues; status: estimate (lines 49 to 56).
2. Checked. Census-exact formula (lines 283 to 291) gives 14,702.27; no starter 1.1527e7; ratio 784.0000; five clues 4.10e-8; naive product 0.02092 and gap 2^19.42 (lines 263 to 270); 2^20 correction gives 2.19e4, a factor 1.49 above (line 279); free frame pairing gives 22.0 (line 296); toy 3x3 0.5625 and 0.0043; Poisson(3.2) <= 2 is 38.0% (line 131). Assumptions: uniform random pairing of half-edges given the census, pieces otherwise independent.
3. The estimate is the null model. Second moment: stated as not computed (lines 196 to 207). It can be, see section 4, U4.
4. Title and summary match the body.
5. Lines 217 to 221 ask for an independent method for the frame count; `ring-purity.mdx:161-170` has one and is not cited. Lines 323 to 327 (birthday term) contradict `entropy-area-law.mdx:181`.
6. `sound with a caveat to add`. Deciding fact: 14,702.27 reproduces from the committed piece set; the planted bias is computable to first order.

### why/complex-theory.mdx

1. A depth-by-depth first moment gives the tree width and 14,702 at depth 256; status: estimate (badge "conjectured").
2. Checked by compiling the committed reference C (`07_complex_theory_reference.out`): 14,702 at depth 256, cumulative 1.365e47, peak 7.3e45 at depth 161 against the closed form 161.8 (line 211). 14,702 / 784^4 = 3.9e-8 (line 85); pinning the pieces directly gives 4.1e-8. The Markov remark (lines 189 to 191) is correct.
3. Not applicable; calibration is external.
4. Lines 142 to 154 misdescribe the table. Width is 1.6e27 at depth 50, 2.1e41 at 100, 5.0e40 at 200. It is within one decade of the peak only for depths about 140 to 178, not "depth 50 to 200", and the "collapse" starts from 10^40.7, not 10^45.
5. The model's own cumulative profile puts 99.45% of nodes beyond depth 132 and 79% beyond 150; the page quotes the measured 99% and 70% (line 149) without saying the model predicts them.
6. `sound with a caveat to add`. Deciding fact: the reference table reproduces; the three-regime prose does not match it.

### why/phase-transition.mdx

1. 17 interior colours sit near the colour count where a random puzzle of this size expects about one solution; status: heuristic plus measurement.
2. Checked: Owen's I = 17.138 and 7.558 for 8x8 (lines 73, 85); strip expectation crosses one between L = 19 (10^-0.08) and L = 20 (10^0.63) (line 140). With the full census-exact first moment the crossing at N = 16 is at 17.68 colours without hints and 17.40 with the starter (`09_phase_line.out`), so the claim holds with the canonical formula too.
3. The 8x8 sweep (lines 87 to 95) matches the committed file: 0/30 solved at 6, 7, 8; 12, 11, 14, 12 of 30 at 1, 2, 3, 5; 27 at 4; 23 at 9. No control separates interior hardness from frame hardness: with one interior colour the interior is trivial, yet 18 of 30 fail, so the row-major DFS is failing on the 5-colour ring. Cheapest control: rerun with the frame pre-placed from the planted solution, or with a border-first order.
4. Title "Near the hardness peak" is no stronger than the body.
5. Contradicted by `how-hard-is-this-instance.mdx:283-290`.
6. `sound with a caveat to add`. Deciding fact: the 8x8 "hard band" sits on a baseline where 53% to 63% of boards already fail at 1, 2, 3 and 5 colours, because of the frame.

### why/how-hard-is-this-instance.mdx

1. A single instance has no complexity class; on planted boards CP-SAT pushes the wall a size past a restarting DFS, "close to where the boards cross the expected-one-solution size" (lines 8 to 11); status: exposition plus measurement.
2. The complexity discussion is correct. 1.115e557 = W_geom / 784, checked. The crude formula at line 285 reproduces its own numbers (-3.3, 3.1, -6.2, 2.4) but omits the 4(N-2) seam joints and the frame. With them (same census-exact formula that gives 14,702): at 17 interior colours log10 E is -32.6, -31.5, -28.4, -23.2, -15.6, -5.6, +7.0 for N = 10 to 16; at 21 colours it is -30.9 at N = 16. The crossing at 17 colours is between N = 15 and 16.
3. Not applicable.
4. Description lines 10 to 11 and body lines 283 to 290 claim the walls line up with the crossing. They do not: the 10x10 to 14x14 boards are 16 to 33 decades below one expected extra solution.
5. Line 228 ("plausibly admit very many solutions") is false for these sizes; `phase-transition.mdx:56-59` and the solution-counting formula place the 17-colour crossing at N = 16.
6. `wrong` for the crossing claim, probability 95%. The remaining 5% is a generator whose colour statistics differ from "17 interior plus 5 border, colour-balanced". The measured solve counts are not affected.

### why/entropy-area-law.mdx

1. Reusable-tile patches are abundant with entropy density tending to about 0.67, and the distinct-piece fraction decays as exp(-0.044 n^2), below 1e-3 near 155 cells; status: theorem (existence of the limit), exact counts (n <= 3), fit.
2. Subadditivity and Fekete (lines 232 to 245) are correct, but Fekete gives the limit as an infimum, not monotone decrease (line 241). I recomputed the strip eigenvalues by power iteration: lambda_1..6 = 46.18, 125.7, 342.5, 932.9, 2541.3, 6922.4, so h = 1.6645, 1.0497, 0.8449, 0.7425, 0.6810, 0.6400. The first four match the page, including the two it labels offline. Since h_inf = inf h(n), h_inf <= 0.6400 < 0.67. The increments log10(lambda_{n+1}/lambda_n) are 0.4350, 0.4352, 0.4352, 0.4352, 0.4352, so h(n) = 0.4352 + 1.2293/n and h_inf is about 0.435. The positivity argument (line 255, "exponentially many chains") is not a proof; one-dimensional chains do not give two-dimensional entropy, and my first constructive attempt (rows of tiles with equal top and bottom) has growth rate exactly 1. A proof exists: four toroidal 3x3 blocks share one boundary signature, so h_inf >= log10(4)/9 = 0.067 (`06_entropy_checks.out`).
   Area law: rho(2) = 0.892166 and rho(3) = 0.647680 are right, and the fit arithmetic (0.0445, 155) follows from them. The form does not. Removing the birthday term C(m,2)/196 leaves 0.02087 per adjacent pair at n = 2 and 0.02089 at n = 3; solving for both coefficients gives 0.0209 and 1.002. The local term is the excess chance that a neighbour is the same piece, predicted (1 + 3/17)/46.1 - 1/196 = 0.0204. A strip Monte Carlo (20,000 uniform chains per length, L up to 40) gives 0.020 to 0.025. The law predicts rho(4) = 0.33 (page: 0.49), rho < 1e-3 at 46 cells, and 1e-29 at 155 cells.
3. No null is needed for the counts. The fit is two points with a rising exponent (0.029, 0.048), which the site's rule calls a conjecture.
4. "Collapses in the area, not the perimeter" (line 178) and "measured near 0.67" (line 257) are stronger than the evidence and incorrect.
5. `solution-counting.mdx:323-327` states the birthday form; `walls-and-methods.mdx:79` and `index.mdx:95` repeat 0.67 and "low hundreds of cells".
6. `wrong`. Probability the 0.67 limit is wrong: 99% (it violates the page's own infimum once h(6) is known). Probability the 155-cell figure is too large by a factor of two or more: 90%.

### why/the-470-wall.mdx

1. An annealed count of boards by score falls smoothly, crosses 1 at 480 and shows nothing at 470; the overlap gap is a conjecture; status: exact arithmetic on a stated model.
2. Checked: p_f = 0.2, p_i = 0.058844, mu = 0.0094366, W_geom = 10^559.9417, mean 36.714, sd 5.732, and the table rows at 480, 478, 475, 470, 465, 460 (1.28, 8.66, 18.40, 33.10, 46.69, 59.55). The independence error is priced at 480 only. I computed the census-exact version (uniform pairing given the colour counts): 7.06 at 480, minus infinity at 479, 12.29 at 478, 20.61 at 475, 34.36 at 470 (`08_landscape_census_exact.out`). The correction is +5.8 decades at 480 and +1.3 at 470.
3. The landscape is the null model.
4. Title and description are careful. One sentence is too strong: "no break, kink or gap anywhere in the 460s or 470s" (lines 133 to 134). The Bernoulli model puts 10^5.1 boards at 479, which the parity theorem forbids.
5. Line 96 to 97: "about one extra candidate in a hundred" for a surrounded hole. On this piece set the number is exactly 0 or 1 tile per fully specified hole, because no two pieces are rotation-duplicates (`why-e2-is-hard.mdx:104`).
6. `sound with a caveat to add`. Deciding fact: every number reproduces; the model misses the 479 gap that another page proves.

### why/rigidity-wall.mdx

1. On the record boards tested no refill within radius 4 of the defects improves the score; status: "proven" badge, with most MIP runs stated as not committed (lines 4 to 7, 173 to 179).
2. Arithmetic checked: 768, 32,640, 522,240, 6,144, 31,375, 8,448; 3.59 edges per cell at 459; no piece with equal opposite pairs (0 of 256); 1.15e7 / 10^37.6 = 10^-30.5 (lines 607 to 611). The SAT halo file is committed. The MIP table, the censuses, the row enumeration and the 9-border UNSAT set are not.
3. The page supplies its own null twice (lines 131, 607 to 614), which is good practice.
4. Overstatements: "The page so far proves the wall" (line 618); "a board-wide bound of <= 476" (lines 205 to 206), which bounds only boards equal to the 469 outside its top four rows; "mathematically exhausted" (line 439) for uncommitted solves.
5. "At most about 10^7 solutions" (lines 554, 608 to 609) uses a first moment as a bound, against `solution-counting.mdx:329`. "All 114 such single swaps" (line 122) uses the stored-orientation count; 125 pairs exist. The toy theorem (lines 632 to 638) is valid, but its hypothesis (at most one tile per exposed colour) fails on E2 by a factor of 46, so "deep in the rigid regime" (line 640) does not follow from it. The 17,857 "legal chains" (line 318) are inconsistent with strict matching against both neighbouring rows unless mismatches are allowed; the definition is not given.
6. `unverifiable from the repo`. Deciding fact: only the radius-4 SAT reproduction is committed.

### why/sigma-cycles.mdx

1. Between every pair of bundled boards, every proper prefix of every large cycle scores below its start, so "you can't hop to a neighbour"; status: exact census plus notebook experiments.
2. Checked against the committed file: 246 pairs, 1,154 large cycles, 54,238 prefixes, none at or above the start, closest -1, deepest -224, largest cycle 6 to 195 with median 119, two single-cycle pairs. Group order 196! 56! 4!/2 = 10^441.64 = 4.3e441.
3. Missing for the headline. A uniform random class-preserving permutation (196, 56, 4) has a largest cycle of median 121 (5% to 95%: 66 to 187; Golomb-Dickman 0.6243 x 196 = 122). The corpus pairs that move at least 240 cells have median 120.5. The "enormous loop" is what two unrelated arrangements give. The page says so at lines 265 to 266 but leads with the loop as a structural fact. The prefix result has the same null: a prefix imports pieces never matched against their neighbours, and each boundary joint matches with probability 1/17.
4. The title and lines 362 to 366 infer the impossibility of basin hopping from interpolation along one specific path between two known boards. Perturb-and-reoptimise is a different operation and is not tested by the census.
5. The "single parity relation" (lines 254 to 257) is probably a sampling accident: with five generators, each of the seven parity characters vanishes on all of them with probability 1/32, expected 0.22 accidental relations; arbitrary legal boards obey none. "114 near-twin pairs" (line 292) is the stored-orientation count.
6. `overstated`. Deciding fact: the census is exact, and its cycle sizes are those of a random permutation.

### why/no-height-function.mdx

1. Three height constructions fail on four boards; a per-colour parity remains; status: identities plus measurement.
2. The scalar identity and the parity (n_c minus twice the matched joints is even) are correct. The counts are board-specific and committed.
3. Stated (line 156): few defects on 480 joints form a forest by chance.
4. Title equals body.
5. None.
6. `sound`. Deciding fact: both identities are one-line proofs and the page claims nothing beyond them.

### why/permutation-code-wall.mdx

1. Score equals 480 minus syndrome weight; a renaming, with three counted facts; status: identity.
2. Correct and trivial; 480 = 2WH - W - H; 256 distinct pieces up to rotation checked.
3. Not applicable.
4. Lines 94 to 97 and 159 to 163 assert that the use-once rule "carries the difficulty". Nothing on the page measures that.
5. Contradicts `entropy-area-law.mdx:215-226`, which says this reading is narrower than the evidence and unmeasured.
6. `sound with a caveat to add`. Deciding fact: the identity is a definition; the hardness attribution is unsupported.

### why/flux-invariants.mdx

1. For each colour, the piece vectors (east minus west, south minus north) sum to zero over a solution, with a quarter turn acting as multiplication by i; status: theorem.
2. Proof followed: it is the statement that east-facing and west-facing half-edges of each colour are equal in number, likewise south and north. The checkerboard member (line 93) is also a valid invariant. Ranks recomputed in exact arithmetic: complex 22, realified 44, stacked real 40, F2 21 with consistent augmentation. The F2 dependency is explicit: the five frame-colour rows sum to zero because every border piece has exactly two frame-coloured sides. Line 214: the abelianisation of the boundary-word group gives the flux law only, not the census.
3. Stated (lines 139 to 143): any solvable set satisfies it and full rank is generic.
4. Title equals body. The page states that pruning value is unmeasured.
5. None.
6. `sound`. Deciding fact: ranks reproduce and the proof is elementary.

### why/ring-purity.mdx

1. The frame colours saturate the ring with zero slack and a valid ring is an Eulerian circuit with corners 15 apart; status: theorem plus exact count plus sampling.
2. Checked: 56 of 56 edge pieces have two frame sides and the other opposite the grey; in and out degree 12; 14 self-loops; 3,432 arborescences by the Laplacian minor; 3.478e41 circuits; incident pieces 20, 21, 21, 22, 22; uniform corner fraction 3.076e-5; 10^41.541 x 2.775e-5 x 4 = 10^37.587; naive 10^34.29.
3. Stated: automatic for any painted board with a separate frame palette.
4. Title equals body.
5. The census-exact first moment for the ring, 4! 56! (12!)^5 / 60!, is 10^37.71, 0.12 decades above the sampled count and 0.10 above Owen's. This is the one place where a first moment can be set against a near-exact count of this instance, and it is not on the page.
6. `sound`. Deciding fact: every deterministic number reproduces.

### why/border-balance.mdx

1. On a solved board the seam colour tallies agree and the deficit lower-bounds seam mismatches; status: identity.
2. Correct. 86 of 99 = 86.9% and 13 of 99 = 13.1% match the committed split.
3. The uniform expectation is 75.8% interior, 11.7% seam, 12.5% ring; the page does not state it, and it shows the seam share is ordinary while the ring share (0 of 99) is not.
4. Title equals body.
5. "The four known full solutions" (line 52, also `walls-and-methods.mdx:85`) reads as E2 solutions, against `solution-counting.mdx:49`. Presumably the four clue puzzles; say so.
6. `sound`. Deciding fact: the identity is immediate and the page says it prunes nothing.

### why/parity-defect-floor.mdx

1. No full placement scores 479, and "at most 76 one-move near misses can surround any solution"; status: theorem.
2. The 479 proof (lines 75 to 91) is correct. The census (64, 24 x 5, 48 x 5, 50 x 12; demand vector) reproduces. The shell bound (lines 111 to 135) is not correct.
   (a) It counts only interior near-twin pairs (50). Two edge pieces that differ in one coloured side, or two corners that differ in one, also swap to a 478 board: 73 edge pairs and 2 corner pairs. Total 125.
   (b) It counts turning tiles, not turns: 23 half-turn tiles and 3 quarter-turn tiles, but the three quarter-turn tiles are among the 23 and each has three qualifying rotations. There are 29 (tile, rotation) moves.
   (c) Lines 129 to 131 say whether a pair "lines up" depends on the solution. It does not: rotate the incoming piece to the outgoing piece's orientation and exactly one joint breaks at each cell. So these are realised neighbours of every solution, and 125 + 29 = 154 is a lower bound (up to pairs that happen to be adjacent across their differing side), not an upper bound.
   (d) "Only two kinds of single move" misses adjacent swaps that change the shared colour: 22 further interior pairs can do this.
   Test on three painted solved boards: single-swap 478 neighbours 140, 133, 143 against 140, 133, 138 near-twin pairs; rotation neighbours 32, 17, 27, equal to the bag count each time; no 479.
3. The page notes the parity holds for any painted board.
4. Description line 6 to 7 carries the wrong bound.
5. `theorem-sweep.mdx:121` repeats it. "Score 480 means solution" (line 101) is a definition, not a consequence.
6. `wrong` for the 76 bound, probability 98%; the 479 theorem is sound.

### why/piece-theft.mdx

1. (North, west) demands have about three servers and 47 have one; a used sole server is a domain wipe-out; status: exact count plus mechanism.
2. Checked: 269 demands, 47 single-server, mean 2.903 distinct servers (2.914 oriented), maximum 7, histogram identical.
3. Present: 100 generated boards, E2 inside the range.
4. "Where solvers die" is not measured; every backtracking dead end is a domain wipe-out by definition. The hub table already says this.
5. None.
6. `sound with a caveat to add`. Deciding fact: counts exact, mechanism unmeasured.

### why/constraint-immediacy.mdx

1. The sum of already-placed neighbours over any fill order is 480; status: identity plus measurement.
2. Correct. Histograms sum to 480. 108 and 112 induced edges checked.
3. The page runs its own second engine as a control and withdraws most of the ranking.
4. Title equals body.
5. Lines 179 to 184: the isoperimetric comparison ignores the rim. Counting grey sides as constraints, four full rows have 132 constrained sides and an 8x8 corner block 128, and both leave 16 open joints, so "row-major is not on the optimum" does not hold once the frame is included.
6. `sound with a caveat to add`. Deciding fact: the identity is exact; the isoperimetric remark needs the boundary.

### why/rare-color-geography.mdx

1. Five colours appear only on the ring, 24 half-edges each; status: exact count.
2. Checked.
3. Stated: the separation is automatic.
4. Title equals body.
5. "The comparatively loose subsystem" (line 151) against `frame-is-not-the-basin.mdx:4`.
6. `sound`. Deciding fact: census reproduces.

### why/forbidden-patterns.mdx

1. 99.72% of distinct-piece 2x2 placements cannot match; status: exact count.
2. Checked: 14,890 of 38,220 pairs and 6,173,828 of 7,414,680 L-shapes recomputed; 3,993,696 / 1,431,033,240 = 0.2791%; 256 / 83,521 = 0.3065%; 20 void corner pairs recounted; 289 e^(-784/289) = 19.2.
3. Present: the first-moment estimate and the void expectation.
4. Title equals body.
5. Typo at line 124: `pprox` (a lost backslash) will render wrongly.
6. `sound`. Deciding fact: all counts reproduce.

### why/mismatch-geometry.mdx

1. Mismatches pack into the band the search fills last; status: measurement on an unpublished corpus.
2. The loop-probability null (line 127, under 0.2%) is right: 225 squares x (25/480)^4 = 0.17%.
3. Present for the forest claim; the page says the heatmap is the pipeline's signature.
4. Description matches the body.
5. `repro: kind: exact` with no command or topic, while lines 64 to 66 say the corpus is not published.
6. `unverifiable from the repo`. Deciding fact: the 1,964-board corpus is not committed.

### why/irreducible-hard-region.mdx

1. A row-major exact search stalls at a median 12 of 16 rows and "more budget does not move it"; status: measurement.
2. The page correctly says the localisation is true by construction.
3. Missing for the budget claim (lines 144 to 159). In an exponential tree depth grows with the logarithm of the budget; a 2.7x budget buys about one cell. The observed "one row of jitter" is what an ordinary tree gives. Cheapest null: plot depth against log10(budget) over three decades and compare the slope with the complex-theory width at depth 190.
4. "The stall is a wall, not a slow start" is stronger than a 2.7x rerun supports.
5. Lines 178 to 180 contrast "plausibly many solutions" with 14,702. The generated boards have no hints, so the like-for-like figure is 1.15e7, the same ensemble as these boards.
6. `sound with a caveat to add`. Deciding fact: the budget test cannot distinguish a wall from exponential growth.

### why/design-recipe.mdx

1. E2 follows a recipe for a very hard puzzle; status: reconstruction.
2. Checked: 196/289 = 0.68, C(196,2)/20,880 = 0.92, sd 9.6 half-edges (the page says about 9), 17.14.
3. Present for symmetry, duplicates and flatness, but the duplicate null (lines 102 to 109) covers interior pieces only. The committed null and my rerun (4,000 boards: 4.08 duplicate pairs, 3.24 of them among border pieces, none in 1.65%) show the absence is rare once the 56 edge pieces are included.
4. "Designed to be unsolvable" and "the hardest edge-matching puzzle a given number of pieces can make" (lines 43 to 45) against the body's "nothing about hardness is proven for this single planted instance" (line 203).
5. `why-e2-is-hard.mdx:103-106` and `the-470-wall.mdx:80-84` call zero duplicates the likeliest trace of design; this page says the absence "does not prove" a filter.
6. `overstated`. Deciding fact: the title and lede claim more than the hedged body.

### why/hint-geometry.mdx

1. Scattered hints beat contiguous top rows; status: cited measurements plus one 8x8 experiment.
2. Not recomputed; the 16x16 numbers are community measurements and the page says so.
3. The page names the missing controls itself (lines 122 to 131).
4. Title equals the hedged body.
5. Consistent with `complex-theory.mdx:149`.
6. `sound`. Deciding fact: every limit of the evidence is stated on the page.

### why/no-forced-moves.mdx

1. Each interior piece has 73 to 137 right-hand partners; status: exact count.
2. Checked: 73, 125 median, 137, mean 119.0.
3. Present.
4. Title equals body.
5. Misquoted elsewhere, see `prune-vs-speed`.
6. `sound`. Deciding fact: counts and null reproduce.

### why/clue-corridors.mdx

1. A clue-to-clue corridor adds no constraint and hurt a greedy filler; status: counting plus an uncommitted A/B.
2. Checked: b_k; 46.1^9/17 = 5.6e13; depletion 0.830; P(196,9) 4^9 = 9.29e25; fraction 5.0e-13 = 17^-10.
3. The chain baseline is given.
4. Title equals body.
5. "Counts exactly on the real piece set" (line 69): the matrix pairs opposite sides, valid for a straight corridor, but the closest clue pair is offset by (5, 5) and any path turns. The count is also a mean over colour pairs, not the clues' own colours.
6. `sound with a caveat to add`. Deciding fact: arithmetic right; the A/B is not committed.

### why/frame-is-not-the-basin.mdx

1. With one beam, the border choice did not move the score; status: measurement.
2. ANOVA recomputed from the committed scores: F(15,112) = 1.2387, p = 0.254, ICC = 0.029, range 432 to 442.
3. The same-border control is the null. Power: the one-sided 95% upper bound on the border share of variance is 0.17, so an effect of that size is not excluded.
4. The title's first clause is general; the second scopes it.
5. Line 4, "the most constrained part of the puzzle", against `rare-color-geography.mdx:151` and this page's own line 82.
6. `sound with a caveat to add`. Deciding fact: the test is valid and bounds the effect at 17%, not at zero.

### why/prune-vs-speed.mdx

1. A speedup is a constant divisor, a branching cut compounds with depth; status: arithmetic plus small measurement.
2. Checked: 0.95^-256 = 5.04e5; 273,029^(1/36) = 1.416; 43,078; 6.34x.
3. Not applicable.
4. "Eternity II was tuned to be precisely that" (line 90) is a design claim without evidence here.
5. Lines 93 to 95 say every interior cell "still has 73 to 137 legal neighbours" and the branching factor "stays stubbornly high", against this page's own 1.4 (line 66) and `no-forced-moves.mdx:85-91`.
6. `sound with a caveat to add`. Deciding fact: arithmetic exact; one bullet contradicts the page's own measurement.

### why/theorem-sweep.mdx

1. A map of thirteen families tagged by standing; status: index.
2. Tags are mostly accurate. Treewidth 16 for the grid is correct.
3. Not applicable.
4. Fine.
5. Repeats "at most 76" (line 121) and "the second carries most of the hardness" (line 139).
6. `sound with a caveat to add`. Deciding fact: two inherited statements need correcting.

### why/walls-and-methods.mdx

1. A table of walls with their strongest supported form; status: index.
2. Not recomputed beyond the shared numbers.
3. Not applicable.
4. Fine.
5. Inherits 0.67 (line 79), "board-wide <= 476" (line 80), "four known full solutions" (line 85), "every interior cell keeps 73 to 137 legal pieces, so the branching factor never collapses" (line 115).
6. `sound with a caveat to add`. Deciding fact: four inherited statements.

### why/why-e2-is-hard.mdx

1. Five bag measurements against painted sets; status: exact counts plus null.
2. Checked: no symmetric piece, 251 multisets, 5 twins, zero rotation-duplicates. My rerun of the null: 4.08 duplicate pairs, none in 1.65% (committed 1.38% on 20,000), no half-turn-symmetric piece in 52.5%.
3. Present and well matched (official joint-colour counts).
4. Two axes are artefacts of the file. The CSV stores every interior piece with its smallest colour on top and every edge piece with grey on top. The "307-of-480 cap if nothing rotates" (lines 110 to 122) is the cap for that orientation; random fixed orientations give 402 +/- 9 (2,000 draws) and a solution's orientations give 480. "A pipeline that fixes orientations early concedes at least 173 joins" (line 117) is false in general. The 114 near-twin pairs (line 101) are defined in the stored orientation; up to rotation there are 125.
5. Three tests were run and the smallest p is 0.014; say so. A duplicated piece would also be a product defect, so its absence need not be a hardness filter.
6. `overstated`. Deciding fact: two of the five axes measure the CSV, not the piece set.

### why/index.mdx

1. Hub page.
2. Nothing to compute.
3. Not applicable.
4. "The quiet second rule that carries the hardness" (line 93) and "past a patch of low hundreds of cells" (line 95).
5. Against `entropy-area-law.mdx:215-226` and the 46-cell figure above.
6. `sound with a caveat to add`. Deciding fact: two inherited sentences.

## 3. Cross-page contradictions

| Quantity | Page A | Page B | Which is right |
|---|---|---|---|
| Entropy limit h_inf | `entropy-area-law.mdx:167,257,260` and `walls-and-methods.mdx:79`: about 0.67 | `entropy-area-law.mdx:244` (limit is an infimum) with h(6) = 0.6400 | B. h_inf <= 0.640, about 0.435 |
| Distinct fraction decay | `entropy-area-law.mdx:181,193-195`: exp(-0.044 n^2), 1e-3 near 155 cells; `index.mdx:95` "low hundreds" | `solution-counting.mdx:323-327`: e^(-m^2/392), faster than area | B, plus a local term: 46 cells |
| Size at which 17 interior colours expect one solution | `how-hard-is-this-instance.mdx:283-290`: N between 12 and 13 | `phase-transition.mdx:56-59,196-199`; `solution-counting.mdx:283-296` | B: N = 16 |
| Solutions of planted boards | `how-hard-is-this-instance.mdx:228` (10x10 to 14x14, "very many"); `irreducible-hard-region.mdx:178-180` | first moment: 10^-33 to 10^-16 extra at N = 10 to 14; 1.15e7 for hint-free 16x16 | first moment |
| Near-twin pairs | `parity-defect-floor.mdx:121`: 50 | `why-e2-is-hard.mdx:68,101`, `sigma-cycles.mdx:292`, `rigidity-wall.mdx:122`: 114 | neither is the bag total. 125 up to rotation (50 + 73 + 2) |
| One-move boards at 478 | `parity-defect-floor.mdx:7,127,169`, `theorem-sweep.mdx:121`: at most 76 | bag count 125 + 29 | at least about 154 |
| First moment as a bound | `rigidity-wall.mdx:554,608-609`: "at most about 10^7 solutions" | `solution-counting.mdx:329`, `the-470-wall.mdx:150-152`: never a bound | B |
| Where the hardness lives | `permutation-code-wall.mdx:94-97`, `theorem-sweep.mdx:139`, `index.mdx:93`, `rigidity-wall.mdx:300-301`: the use-once rule | `entropy-area-law.mdx:215-226`: narrower, unmeasured | B |
| Branching in the interior | `prune-vs-speed.mdx:93-95`, `walls-and-methods.mdx:115`: 73 to 137, never collapses | `no-forced-moves.mdx:85-91`, `clue-corridors.mdx:60-61`, `prune-vs-speed.mdx:66`: about 2.7 with two neighbours, 0.16 with three, effective 1.4 | B |
| Frame tightness | `frame-is-not-the-basin.mdx:4`: most constrained part | `rare-color-geography.mdx:151`: comparatively loose | B per joint (1/5 against 1/17) |
| Is zero duplicates evidence of a filter | `design-recipe.mdx:102-109`: one in five, not proof | `why-e2-is-hard.mdx:103-106`, `the-470-wall.mdx:80-84`: 1.4% | B; A omits the border pieces, which carry 3.2 of 4.1 expected pairs |
| Extra tiles for a surrounded hole | `the-470-wall.mdx:96-97`, `rigidity-wall.mdx:323-326`: 0.0094 | `why-e2-is-hard.mdx:104`: zero rotation-duplicates | on this instance exactly 0; 0.0094 is the ensemble mean |
| Score ladder near the top | `the-470-wall.mdx:133-134`: no gap in the 470s | `parity-defect-floor.mdx:38`: no board at 479 | B |
| Tree width by depth | `complex-theory.mdx:142-154`: plateau 10^45 over depth 50 to 200 | committed reference C: 1.6e27 at 50, 5.0e40 at 200 | reference table |
| Known full solutions | `border-balance.mdx:52`, `walls-and-methods.mdx:85`: "the four known full solutions" | `solution-counting.mdx:49`: none exhibited | B for E2; A means the clue puzzles |

## 4. Upgrades available

U1. Entropy limit, from "carried from an offline sweep" to exact values plus rigorous bounds. Done: h(1..6) by power iteration (`05`, `05b`; 40 s for n = 6 in float32, 3 GB). Rigorous 0.067 <= h_inf <= 0.640. Width 7 needs a broken-profile implementation (17^7 states, about 10 minutes in Rust). A tighter lower bound needs a larger family of interchangeable toroidal blocks; 266 boundary classes already hold two or more 3x3 blocks.

U2. Distinctness law, from a two-point fit to a tested law. Sample uniform colour-valid 4x4 and 5x5 patches exactly by backward sampling through the transfer operator (17^5 states at width 4) and measure the distinct fraction. Prediction to refute: 0.328 at 4x4 and 0.094 at 5x5; the page's fit says 0.49 and 0.33. Cost: under an hour of code, minutes of CPU.

U3. The 478 shell, from a wrong bound to a theorem. Statement: every solution has at least 29 + 125 - a single-move neighbours at 478, where a is the number of near-twin pairs adjacent across their differing side; an adjacent near-twin swap that changes both shared half-edges must give another solution, by the parity theorem. Proof is four lines (section 2). An upper bound needs the adjacent-swap configurations for border pieces as well; minutes.

U4. Second moment. A planted puzzle has expected count E[Z^2]/E[Z] = 1 + (sum over difference sets D of the expected number of refills of D). Small D, independent-edge model: two interior cells 5.7e-5; three interior cells below 7.9e-5; two non-adjacent edge pieces C(56,2) (1/5)^4 (1/17)^2 = 8.5e-3; two corners 6/625 = 9.6e-3; contiguous edge-piece pairs 3e-3. For compact D of m cells the weight is m! 4^m 17^-(2m + boundary), below 1 for all m < 196 (this is Owen's criterion read locally), about 10^-31 at m = 100. D equal to everything is E[Z]. So the planted expectation is 1 + E[Z] + about 0.03 in the model. I have not summed the intermediate sizes rigorously; that needs a count of cell sets by boundary length and is a day of work. On the real bag the two-cell term is exactly computable: it needs a near-twin pair adjacent across its differing sides.

U5. Annealed landscape, from Bernoulli to census-exact. Done (`08`): factorial moments of the number of monochromatic pairs under a uniform pairing, inverted exactly in rationals. Reproduces 10^7.0617 at 480 and zero at 479. Cost: 2 seconds.

U6. Frame count. An exact count with the corner constraint needs a sum over about 10^12 arc-count vectors (the 56 edge pieces fall in 24 arc types; 1.41e9 sub-multisets of size 14); not modest. Cheap alternative: 100 times more samples (about 20 core-hours at the committed rate of 90 s on 8 cores) brings the standard error from 4.7% to 0.5%. The page should also record the three-way agreement in summary line 12.

U7. Calibration on a large exact count. The solution-counting page asks for the model against Clue 3's 2,195,647,488. With the committed 36-piece file the census-exact first moment is 8.2e8 counting labelled pieces (ratio 0.37) and 7.1e5 counting distinct boards (`10`). The set has duplicate and self-symmetric pieces and two dominant interior colours, so the pairing model is at its worst here, and the convention of the exact count is not in the repo. Inconclusive as it stands; a factor 2.7 if the count is labelled.

U8. Phase line. c*(N), the interior colour count at which the full first moment is one: 4.65, 7.31, 9.93, 12.54, 13.83, 15.12, 17.68 for N = 6, 8, 10, 12, 13, 14, 16 (`09`). The committed 6x6 peak (5 colours) and 8x8 hard band (6 to 8) sit on it.

## 5. Theory that could guide search

Most pages describe the bag. Three results give a number a solver can compute.

C1. Complex-theory depth profile as a prediction of where nodes go. The model gives, for any fill order, the expected nodes at each depth. For the reference order it predicts 0.55% of nodes up to depth 132 and 21% up to depth 150; one backtracker measured 1% and 30%. Test: instrument the committed DFS on the official board for the five orders of `constraint-immediacy` (measured depths 60, 35, 194, 193, 35 of 256), record nodes per depth, and compare with the model's cumulative curve and its predicted depth at the measured node budget. If the model ranks the five orders and places each stall within a row, fill orders can be chosen by computation alone, including orders nobody has run.

C2. The phase line c*(N) as a benchmark design rule. The committed solver comparisons use planted boards at 17 or 21 interior colours and N = 10 to 14, which are 16 to 48 decades into the over-constrained side, where more colours mean stronger propagation. That explains why CP-SAT finished a 26-colour 16x16 in 15 s and says the result does not transfer to E2, which sits on the line. Test: run CP-SAT and the DFS on a grid of (N, c) with N = 8, 10, 12 and c from c* - 3 to c* + 4, ten seeds each, and check that median effort peaks within one colour of c*(N). If it does, every future solver claim should be made at c = round(c*(N)).

C3. Census-exact completion count as a node score. For a partial board, the expected number of perfect completions under uniform pairing of the remaining half-edges, given the frontier colours, is the same closed form as the 14,702 formula on the residual census, and the tolerant version (at most d mismatches) comes from the factorial-moment inversion in `08`. It includes two hard tests for free: per-colour parity of the residual supply against the frontier, and the directional balance of the flux law. Test: on 8x8 planted boards at c*(8), enumerate completions exactly for a few thousand partial boards at depth 30 to 45 and measure the rank correlation between log estimate and log true count; then run a fixed-width beam scored by the estimate against the same beam scored by matched edges. Kill criterion: correlation below 0.5 or no gain in solve rate.

Also usable, lower priority. The distinctness law says a piece-agnostic relaxation over m cells over-counts by 1/rho(m): 3x at 16 cells, 11x at 25, 87x at 36, 2,300x at 49. That prices pattern databases and colour-first methods before they are built: beyond about 25 cells the relaxation is mostly counting unrealisable patches.

## 6. New results and known results

Possibly new, worth a short proper write-up (I did not find them in a quick search, which is weak evidence):

- The frame of an edge-matching puzzle with a separate border palette as a directed Eulerian circuit, counted by the BEST theorem (3,432 x (11!)^5 for E2), with the corner-spacing fraction. The one-row case is known to be an Eulerian trail problem; the ring count with marked corners I did not find.
- The local-plus-birthday law for the distinct-piece fraction, with the birthday coefficient recovered as 1.002 from two exact counts.
- The census-exact annealed landscape with the parity gap at 479 built in.
- The corrected shell statement: at least 154 single-move boards at 478 around any solution of this bag.
- h_inf of the E2 interior grammar: 0.4352 numerically, between 0.067 and 0.640 rigorously, against a mean-field 0.4334.

Known under another name:

- The flux law is the abelianised boundary-word invariant of Conway and Lagarias, "Tiling with polyominoes and combinatorial group theory", J. Combin. Theory A 53 (1990), DOI 10.1016/0097-3165(90)90057-4 (cited on the page), developed as tile homology in M. Reid, "Tile homotopy groups", L'Enseignement Mathematique 49 (2003) 123-155.
- The census-exact first moment (the (n_c - 1)!!/839!! factor) is the pairing probability of the configuration model: B. Bollobas, "A probabilistic proof of an asymptotic formula for the number of labelled regular graphs", European J. Combin. 1 (1980) 311-316, DOI 10.1016/S0195-6698(80)80030-8.
- The Eulerian count: T. van Aardenne-Ehrenfest and N. G. de Bruijn, "Circuits and trees in oriented linear graphs", Simon Stevin 28 (1951) 203-217. The page cites Wikipedia only.
- Giant cycles between unrelated boards are the cycle structure of a uniform permutation: L. A. Shepp and S. P. Lloyd, "Ordered cycle lengths in a random permutation", Trans. Amer. Math. Soc. 121 (1966) 340-357, DOI 10.1090/S0002-9947-1966-0195117-8.
- "Two board pairs already generate the whole group" is Dixon's theorem: J. D. Dixon, "The probability of generating the symmetric group", Math. Z. 110 (1969) 199-205, DOI 10.1007/BF01110210.
- Second moments of solution counts and pair overlaps: D. Achlioptas, A. Naor and Y. Peres, "Rigorous location of phase transitions in hard optimization problems", Nature 435 (2005) 759-764, DOI 10.1038/nature03602. The planted relation E[Z^2]/E[Z] is in the works the pages already cite.
- Entropy of two-dimensional tile grammars: M. Hochman and T. Meyerovitch, "A characterization of the entropies of multidimensional shifts of finite type", Annals of Math. 171 (2010) 2011-2038, arXiv math/0703206. Entropies of such systems are exactly the right-recursively-enumerable numbers, which is why positivity needs a construction and upper bounds come from strips.
- Complex theory as Williams and Hogg's expected partial solutions per depth is already cited on its page.

## 7. What I could not check

- The MIP table, the move censuses, the row-chain enumeration, the nine-border UNSAT set and the group computation on `rigidity-wall` and `sigma-cycles`: notebook results, nothing committed. The parity relation in the generated group could be tested in minutes if the six boards were named.
- The 1,964-board and 421-board corpora behind `mismatch-geometry`.
- The clue-corridor A/B and the 26-colour CP-SAT round on `how-hard-is-this-instance`: stated on their pages as not committed.
- The 8x8 hint experiment in `research/experiments/hint-study`: present, not rerun.
- Every groups.io figure (Owen's 4.05e37, the 9x9 counts, the Clue 3 count and its convention, McGavin's node counts): quoted, not in the repo as data. I did not check message numbers against the archive.
- The corner-spacing sample (2.775e-5 from 16 million circuits): not rerun; I checked only the formula around it and the uniform value 3.076e-5. It is 2.3 standard errors below uniform.
- The second-moment sum for intermediate difference sets (U4) is bounded by an argument, not computed.
- My strip eigenvalues come from power iteration in floating point (float32 at width 6, stable to six digits over five iterations); they are not interval-certified.
- I did not read the French and Spanish pages; every correction above applies to them as well.
