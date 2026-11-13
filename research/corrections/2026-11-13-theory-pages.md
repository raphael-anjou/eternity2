# Corrections to the theory pages, 2026-11-13

Sentence-level corrections to pages under `web/content/research/why/`. Each
entry gives the page, the old sentence, the new sentence, and the evidence. The
English text is quoted; the French and Spanish pages carry the same correction
in their own wording. Every number written was recomputed for this log from the
committed piece file (`engine/data/official_eternity2.csv`) or from committed
results, by the scripts in `2026-11-13-theory-pages.scripts/` (stdlib Python or
dependency-free Node, single thread). Each script's output is committed next to
it as `.out`.

| Script | Computes |
|---|---|
| `piece_facts.py` | near-twin pairs (stored orientation and up to rotation, by kind), two-side-keeping turns, how the file stores pieces, the fixed-orientation cap for the stored file and for random orientations |
| `crossing.py` | first-moment solution counts of planted N x N boards (17 interior + 5 frame colours) with and without seams, frame and clues; checks itself against 1.153e7 and 14,702 on the official counts |
| `strip_entropy.js` | the strip eigenvalues lambda_n and h(n) = log10(lambda_n)/n for widths 1 to 6 (widths 1 to 4 reproduce `research/topics/entropy-area-law/results/entropy.json`) |
| `distinct_fraction.js` | the share of colour-valid patches that use distinct pieces, sampled exactly (checks itself against the committed exact 2x2 and 3x3 values) |

The three entropy scripts (`strip_entropy.js`, `distinct_fraction.js`,
`entropy_fields.py`) and their `.out` files have since moved to
`research/topics/entropy-area-law/compute/scripts/`, where `just
research-entropy-area-law-scripts` reruns them. A second seeded run of the
sampler (seeds 101 to 106) is committed there as
`distinct_fraction.seed100.out`; `entropy_fields.py` now reads both runs and
prints the crossing for each (44.2 and 44.4 cells) and the spread between
them. Entries below name these files as they stood when written. Likewise
`defect2_moves.py` and its `.out` moved to
`research/topics/parity-defect-floor/compute/scripts/` (output unchanged, byte
for byte).

## why/parity-defect-floor

1. Description and metaDescription.
   - Old: "The floor below perfect is 478, and at most 76 one-move near misses can surround any solution."
   - New: "The floor below perfect is 478, and every solution has about 150 one-move neighbours sitting exactly there."
   - Evidence: `piece_facts.out`: 125 near-twin pairs up to rotation (50 interior, 73 edge, 2 corner) and 29 two-side-keeping turns on 23 tiles; 125 + 29 = 154.
2. Intro.
   - Old: "the census that bounds how many one-move near misses can surround any solution."
   - New: "the census that counts the one-move near misses around every solution."
   - Evidence: as 1.
3. Twin swaps bullet.
   - Old: "only two kinds of single move can do that. [...] Two interior tiles that agree [...] The set contains exactly **50** such near-twin pairs."
   - New: "Two kinds of single move do that [...] Two tiles of the same kind that agree [...] The set contains exactly **125** such near-twin pairs, 50 of them interior, 73 edge and 2 corner."
   - Evidence: `piece_facts.out`. "Only" was dropped because adjacent swaps that change the shared colour are a third kind; their count was not recomputed here, so no number is given for them.
4. The shell paragraph.
   - Old: "Adding them up, any solution has at most $50 + 23 + 3 = 76$ defect-2 neighbours [...] This is an upper bound from the bag [...] the realized count for the designed solution is open. The ceiling stands regardless. [...] near misses are sparse, a few dozen boards at most [...]"
   - New: "The three quarter-turn tiles are among the 23 [...] the 23 tiles allow 29 distinct turns. Both kinds of move work in every solution. [...] So every solution has about $125 + 29 = 154$ single-move neighbours at 478, a floor from the bag and not a ceiling (a pair that happens to sit side by side across its differing edge is the one exception). An earlier version of this page gave a ceiling of 76 here; [...] it is withdrawn. [...]"
   - Evidence: `piece_facts.out` (29 moves on 23 tiles). The "every solution" step is the argument written on the page: rotate the incoming piece into the outgoing piece's orientation, and exactly the one differing joint breaks at each cell.
5. History section.
   - Old: "and caps the one-move shell around every solution at 76."
   - New: "and puts about 150 one-move boards at 478 around every solution."
6. Check-it-yourself paragraph.
   - Old: "the 50/23/3 defect-2 generators"
   - New: "the 50 interior near-twin pairs and the 23 turning tiles [...] (The checker's `defect2_generator_bound` check still sums its interior counts to the withdrawn 76 [...])"
   - Evidence: `research/topics/parity-defect-floor/results/census.json` holds the 76 check; the topic is not changed by this log.

## why/why-e2-is-hard

1. Description.
   - Old: "a 307-of-480 cap holds if nothing rotates."
   - New: "A 307-of-480 cap if nothing rotates describes the file's stored orientation, not the pieces."
   - Evidence: `piece_facts.out`: all 196 interior pieces are stored with their smallest colour on top and all 56 edge pieces with grey on top; stored cap 307; random orientations of all 256 pieces cap at mean 402.4, sd 9.6, range 369 to 436 (2,000 draws, seed 20261113).
2. metaDescription: same qualifier on "a 307-of-480 fixed-orientation cap".
3. Table, Twins row.
   - Old: "5 twin pairs and 114 near-twin pairs in 32,640 pairings"
   - New: "5 twin pairs in 32,640 pairings; 125 near-twin pairs up to rotation (114 counted in the stored orientation)"
   - Evidence: `piece_facts.out` (114 stored; 125 up to rotation, same kind).
4. Table, Fixed-orientation cap row.
   - Old: "at most 307 of 480 joins can match if no piece rotates | not compared; rotation is structurally necessary here"
   - New: "at most 307 of 480 joins can match if no piece leaves its stored orientation | a property of the file: random fixed orientations cap near 402"
5. Five twins section.
   - Old: "Loosening the question to near-twins, pairs sharing 3 of their 4 same-position edges in the stored orientation, adds 114 pairs in 79 groups."
   - New: "Loosening the question to near-twins, pairs of the same kind that agree on three of their four sides after some rotation, adds 125 pairs: 50 interior, 73 edge, 2 corner. (Counted in the stored orientation only, the figure is 114 pairs in 79 groups; it describes how the file writes the pieces down.)"
6. Rotation section.
   - Old: "A pipeline that fixes orientations early therefore concedes at least 173 joins before search begins, and 307 sits far below every high board on the records page."
   - New: "The number belongs to the published orientation [...] Random fixed orientations of all 256 pieces cap near 402 (2,000 draws, standard deviation 10, lowest 369), and the orientations of a solution cap at 480. So 307 says the file's orientation is far from a solution's; it does not price a pipeline that fixes orientations some other way. An earlier version of this page read it as a cost of at least 173 joins [...]; that reading is withdrawn."
7. "What these properties rule out", orientation bullet.
   - Old: "Fixing rotations early concedes 173 of 480 joins by a counting argument, before any search runs."
   - New: "No piece is symmetric, so every rotation is a real choice; the 307 cap of the stored file shows only that the published orientation is far from a solution's."

## why/how-hard-is-this-instance

1. Description.
   - Old: "and both walls move with the colour count, close to where the boards cross the expected-one-solution size."
   - New: "and both walls move with the colour count."
   - Evidence: `crossing.out`, below.
2. Bench paragraph.
   - Old: "Planted boards are not canonical Eternity II; they plausibly admit very many solutions, where the real puzzle's first-moment estimate is about 14,702 with the centre clue and about one with all five."
   - New: "Planted boards are not canonical Eternity II. At these sizes they are far more constrained than the real puzzle: the same first-moment estimate that gives about 14,702 for Eternity II with the centre clue gives a random 12×12 board with 17 interior colours and five clues about $10^{-41}$ solutions, so a planted board of that size almost surely has only its own."
   - Evidence: `crossing.out`: census-exact first moment, five clues, N = 12: log10 = -41.4. The same function returns 14,702.3 and 1.153e7 on the official counts.
3. Crossing paragraph.
   - Old: "Where the walls sit lines up with where these boards cross the expected-one-solution size [...] crosses one between N = 12 and N = 13 at 17 interior colours ($10^{-3.3}$ at 12, $10^{3.1}$ at 13) and near N = 16 at 21 [...] Both solvers hit their wall near this crossing [...]"
   - New: "The walls do not sit at the expected-one-solution size. Counting every joint, frame and seams included, [...] a board with 17 interior colours and no clues expects $10^{-28}$ solutions at 12×12, $10^{-6}$ at 15×15 and $10^{7}$ at 16×16; with five clues it stays below one at every size up to 16×16 ($10^{-7}$ there). [...] An earlier version of this page placed the crossing between 12×12 and 13×13 with a formula that left out the seam joints and the frame; that placement is withdrawn."
   - Evidence: `crossing.out`: page formula -3.3 (12), 3.1 (13) reproduced; census-exact no clue -28.4 (12), -5.5 (15), +7.1 (16); five clues -44.4 to -7.4 for N = 10 to 16. The 21-colour figures were dropped with the formula, not recomputed.

## why/entropy-area-law

1. Entropy chart caption.
   - Old: "Decreasing toward the limit ≈ 0.67. The first two points are computed exactly here; the rest from an offline sweep."
   - New: "Decreasing toward a limit near 0.44. Each added row multiplies the count by about 2.72 (10 to the power 0.435), so h(n) is close to 0.435 + 1.23/n; the widths shown are still far from the limit."
   - Evidence: `strip_entropy.out`: lambda_1..6 = 46.182, 125.745, 342.502, 932.949, 2541.305, 6922.392; h = 1.6645, 1.0497, 0.8449, 0.7425, 0.6810, 0.6400; increments 0.4350, then 0.4352 four times. Widths 1 to 4 equal the committed `research/topics/entropy-area-law/results/entropy.json`. 1.6645 - 0.4352 = 1.2293.
2. Section "Then the area law bites", renamed "Then the use-once rule bites".
   - Old: "It collapses, and it collapses in the area, not the perimeter: $\rho(n) \approx \exp(-\alpha n^2)$, $\alpha \approx 0.044$." plus the fit paragraph.
   - New: exact values 0.892 (2x2) and 0.648 (3x3) from the committed counts, then "drawing color-valid patches four cells wide uniformly at random [...] gives $\rho = 0.33$ at 16 cells, 0.11 at 24, 0.024 at 32 and 0.0004 at 48. It collapses, and faster than an area law: $-\ln\rho$ per cell rises from 0.07 at 16 cells to 0.16 at 48. [...] Side-by-side cells hold the same piece about 2.6% of the time [...] five times the chance level of 1 in 196 [...]"
   - Evidence: `distinct_fraction.out`: 4x4 0.3263 (s.e. 0.0007), 4x6 0.1060 (0.0005), 4x8 0.02383 (0.00024), 4x12 0.000375 (0.000019); side-by-side repeats 2.58% to 2.61%. The sampler reproduces the committed exact 2x2 (0.8921 against 0.892166) and 3x3 (0.6473 against 0.647680). -ln(0.3263)/16 = 0.070; -ln(0.000375)/48 = 0.164. The "-ln rho = 0.021 a + C(m,2)/196" law was not written: on these samples it runs high (5.9e-4 against 3.75e-4 at 48 cells).
3. Collapse point.
   - Old: "Extrapolating, the fraction of realizable patches drops below one in a thousand near 155 cells. (An earlier off-site study reported a steeper $\alpha \approx 0.085$ and an ~80-cell collapse; [...])"
   - New: "The fraction of realizable patches drops below one in a thousand near 45 cells. An earlier version of this page fitted $\rho(n) \approx \exp(-0.044 n^2)$ to the two exact points and extrapolated a collapse near 155 cells; the sampled fraction at 48 cells is about 300 times below that fit, so the fit and the 155-cell figure are withdrawn."
   - Evidence: log-linear interpolation between 32 cells (0.02383) and 48 cells (0.000375) crosses 1e-3 at 44.2 cells; the old fit gives exp(-0.0445 x 48) = 0.118 at 48 cells, 315 times the sample. The off-site 0.085 sentence was removed with the fit it was compared to.
4. rho chart: the curve exp(-areaLawExponent n^2) read from `web/src/data/entropy-area-law.json` was replaced by the seven points above (exact to 9 cells, sampled beyond), with a matching caption.
5. Theorem section.
   - Old: "Fekete's lemma gives the limit as an infimum, which is exactly why the curve above decreases" / "$0 < h_\infty \le [...] 1.6645$ [...] Positivity holds because the grammar supports exponentially many chains, so the density is strictly between zero and 1.6645, measured near 0.67." / "Its specific value near 0.67 is carried from an offline sweep."
   - New: "[...] so every computed width bounds it from above" / "$0 \le h_\infty \le [...] 1.6645$ [...] Because the limit is an infimum, every computed width is an upper bound: $h(6) = 0.640$ gives $h_\infty \le 0.640$. The increments [...] sit at 0.4352 from width 2 to width 6, so the limit is close to 0.435, next to the mean-field value $\log_{10}(784/289) = 0.433$. Strict positivity does not follow from this argument [...]" / "Its value, about 0.435, is read from the strip eigenvalues for widths 1 to 6 [...] An earlier version of this page gave the limit as about 0.67; since $h(6)$ is already below that, the figure is impossible and is withdrawn."
   - Evidence: `strip_entropy.out`; log10(784/289) = 0.4334. The rigorous lower bound 0.067 was not written (not recounted).
6. Front matter `repro.scope` rewritten to say which numbers the topic command reproduces and that its results file still carries the withdrawn values.

## why/design-recipe

1. Title.
   - Old: "Designed to be unsolvable: the recipe"
   - New: "Designed to be hard: the recipe"
   - Evidence: the body states "Nothing about hardness is proven for this single planted instance" and the model it cites expects about 14,702 solutions with the starter (`crossing.out`, check line). The old title claimed more than the body.
2. Lede.
   - Old: "a short list of design rules that, followed together, produce the hardest edge-matching puzzle a given number of pieces can make, while still guaranteeing that a solution exists."
   - New: "a short list of design rules that, in Owen's simplified model, push a puzzle of this size toward its hardest setting while still guaranteeing that a solution exists. Nothing about hardness is proven for this one instance."
3. Duplicate null.
   - Old: "so an unfiltered generator produces no symmetric and no duplicate interior piece roughly one time in five. The absence fits a filter but does not prove one; the flat frequency distribution below is the ingredient that is clearly non-random."
   - New: "[...] roughly one time in five. The border changes the picture: there are only 425 edge-piece designs ($5 \times 5$ frame colours times 17 inward colours) for 56 pieces, so a set cut from a randomly painted solved board with the official colour counts expects about four pairs of pieces equal up to rotation, three of them among the border pieces, and has no repeat anywhere among the 256 in only about 1.4% of cases [...]. The absence of duplicates is good evidence of a filter; the absence of symmetric pieces is not. The flat frequency distribution below is the other ingredient that is clearly non-random."
   - Evidence: `duplicates_by_kind.out` (20,000 painted boards): mean 4.10 rotation-duplicate pairs, 0.85 interior, 3.04 edge, 0.21 corner; no repeat among all 256 in 1.46%. Committed `research/topics/adversarial-piece-set/results/null_model.json`: mean 4.09, 1.38% with none. 425 = 5 x 5 x 17.

## why/sigma-cycles

1. Loop size.
   - Old: "Here's the catch. Between the best boards that loop is enormous."
   - New: "Here's the catch. Between the best boards that loop is enormous, and exactly as large as it would be between two unrelated arrangements: a random reshuffle that keeps corners, edges and interior pieces in their own classes has a largest loop of median 119 cells (5% to 95%: 66 to 187), and the record pairs have a median of 119."
   - Evidence: `sigma_null.out`: corpus (committed `research/topics/sigma-cycles/results/sigma-cycles.json`, 246 pairs) largest cycle 6 to 195, median 119; class-preserving uniform permutation, 20,000 draws: median 119, 5% 66, 95% 187.
2. Near-twin count.
   - Old: "The canonical piece set contains 5 twin pairs and 114 near-twin pairs"
   - New: "[...] 5 twin pairs and 125 near-twin pairs counted up to rotation (114 in the file's stored orientation)"
   - Evidence: `piece_facts.out`.
3. Why it matters.
   - Old: "and you can't hop to a neighbour either, because the nearest better board is one indivisible move of many cells away, with no improving step to lead you into it."
   - New: "and walking the loop from one known strong board to another never passes a board as good as its start. That rules out interpolating between known boards. It does not test perturb-and-reoptimise moves, which are a different operation."
   - Evidence: the census on the page tests prefixes of the loop between two known boards only. The title ("looks impossible") was left as it is; the "single parity relation" paragraph was left unchanged (the claim that it is a sampling accident was not checked).

## why/index, why/theorem-sweep, why/walls-and-methods (hub pages repeating corrected claims)

1. index, piece-theft paragraph.
   - Old: "the use-each-piece-once rule, the quiet second rule that carries the hardness, one felt as a single dead cell and one measured as the collapse of genuinely distinct boards past a patch of low hundreds of cells."
   - New: "[...] the quiet second rule, one felt as a single dead cell and one measured as the collapse of genuinely distinct boards past a patch of about 45 cells."
   - Evidence: `distinct_fraction.out` (see the entropy entry). "That carries the hardness" was dropped because the entropy page states that its measurement is narrower than that.
2. theorem-sweep, parity entry.
   - Old: "and around any solution there are at most 76 one-move boards at that defect."
   - New: "and every solution has about 150 one-move boards at that defect (125 near-twin swaps and 29 turns in place, a floor, not a ceiling)."
   - Evidence: `piece_facts.out`.
3. walls-and-methods, entropy row.
   - Old: "[...] giving an in-repo exponent α≈0.044; the entropy limit near 0.67 and the collapse point are extrapolations | Proven, internal (the block counts) + measured (the fit) | [...] the α fit and collapse scale are this project's own | The theorem guarantees a positive limit exists; it does not fix its value. The exponent is fit over n≤3 [...]"
   - New: "[...] sampled patches four cells wide put the distinct-piece fraction below one in a thousand near 45 cells, faster than an area law; strip eigenvalues to width 6 put the entropy limit at most 0.640 and close to 0.435 | Proven, internal (the block counts and the upper bound) + measured (the sampled fractions) | [...] the sampled fractions and strip values are this project's own | The theorem guarantees that the limit exists and bounds it from above; its value near 0.435 is read from widths up to 6. Fractions beyond 3×3 are sampled, not counted |"
   - Evidence: `strip_entropy.out`, `distinct_fraction.out`.

## why/irreducible-hard-region

1. Description and metaDescription.
   - Old: "a row-major search stalls at a median of 12 of 16 rows and more budget does not move it." / "then stalls whatever the budget."
   - New: "a row-major search stalls at a median of 12 of 16 rows, and 2.7 times more budget leaves it within one row, as an ordinary exponential tree would." / "then stalls; 2.7 times more budget moves it by at most one row."
   - Evidence: committed `research/topics/irreducible-hard-region/results/hard_region_budget4m.json` (eight seeds at four million nodes, as tabled on the page: four up one row, one down one row, three unchanged).
2. Heading and conclusion of the budget section.
   - Old: "## The stall is a wall, not a slow start" / "One more check separates a wall from a search that simply ran short of nodes." / "The search is not slow; it is stopped, and it is stopped at the last region it tries to fill. That is the budget-insensitive failure the conjecture predicts, reproduced directly."
   - New: "## More budget, same frontier" / "One more check asks whether a larger budget moves the frontier." / "This test cannot tell a wall from an ordinary exponential tree. If the tree grows by a factor $b$ per cell at the frontier, a 2.7 times larger budget buys about $\log 2.7 / \log b$ more cells, less than one cell whenever $b$ exceeds 2.7, so a frontier that stays inside one row is what an ordinary tree also gives. A test with power needs budgets spread over several decades."
   - Evidence: argued, no new measurement. The proposed model figure (1.5 million nodes reaching depth 203.0 and 4 million reaching 204.2 in the first-moment model) was not recomputed here and is not on the page.

## why/complex-theory

1. Plateau bullet.
   - Old: "**Plateau (depth 50–200).** The tree is at its widest, about $10^{45}$ ways to extend, while the solution count barely moves."
   - New: "**Plateau (depth 50–200).** The tree is at its widest here, though not flat: the width climbs from about $10^{27}$ at depth 50 to a peak near $10^{46}$ at depth 161 and is back to about $10^{41}$ by depth 200; it is within one decade of the peak only for depths about 140 to 178."
   - Evidence: `complex_theory_reference.out`, the output of the committed `research/topics/complex-theory/reference/complex_theory.c` (compiled with gcc and libgmp, unmodified): width 1.5623e27 at depth 50, 2.054e41 at 100, peak 7.3016e45 at 161, 5.0174e40 at 200, 14,702 at 256; within one decade of the peak for depths 139 to 178.
2. Collapse bullet.
   - Old: "The width falls from $10^{45}$ back to about $10^{4}$."
   - New: "The width falls from about $10^{41}$ back to about $10^{4}$."
3. Funnel paragraph.
   - Old: "it climbs astronomically, then barely moves for a hundred cells across the plateau"
   - New: "it climbs astronomically, then climbs far more slowly for a hundred cells across the plateau"
   - Evidence: as 1 (about 0.17 decades per cell from depth 50 to 161, against about 0.55 in the first 50).
   - The regime name "plateau" was kept because the rest of the page uses it.

## why/rigidity-wall

1. Front matter `rigor: proven` changed to `rigor: measured`: the description itself says most MIP runs are notebook results not yet committed; the committed evidence is the radius-4 SAT halo (`research/topics/rigidity-sat-halo/results/halo-sat.json`).
2. Twin swaps.
   - Old: "On the community 469, all 114 such single swaps were tried:"
   - New: "On the community 469, all 114 such single swaps were tried (the pairs that share three edge colours in the file's stored orientation; counted up to rotation there are 125):"
   - Evidence: `piece_facts.out`. The experiment itself (on the 114) is reported as it was run.
3. Skeleton paragraph.
   - Old: "with at most about $10^7$ solutions"
   - New: "with an expected $10^7$ solutions without clues, a first-moment estimate and not a bound"
   - Evidence: `crossing.out` check line, 10^7.062 = 1.153e7 without clues. A first moment is an expectation over an ensemble, not an upper bound for this instance.
4. Border null.
   - Old: "against at most about $1.15\times10^{7}$ solutions, each with one border, so a random legal border hosts a 480 with probability below about $10^{-30}$"
   - New: "against an expected $1.15\times10^{7}$ solutions without clues (a first-moment estimate, not a bound), each with one border, so under that estimate a random legal border hosts a 480 with probability of order $10^{-30}$"
   - Evidence: 10^(7.062 - 37.6) = 10^-30.5.
5. Section opening.
   - Old: "The page so far proves the wall;"
   - New: "The page so far documents the wall on the boards tested;"

# Data files and neighbouring pages

The pages above were corrected first; this section brings the committed data,
the components that read it, and the pages that repeated the withdrawn
figures into line. Same format: page or file, old text, new text, evidence.
New scripts sit beside the others, each with its committed `.out`:

| Script | Computes |
|---|---|
| `defect2_moves.py` | near-twin pairs and swaps by kind, two-side-keeping turns as tiles and as moves, the single-move floor at 478, and the old 76 sum for comparison |
| `entropy_fields.py` | from `strip_entropy.out` and `distinct_fraction.out`: the h(n) table and increments, the upper bound on h_infinity, -ln(rho) per cell, the 1e-3 crossing by log-linear interpolation, and what the withdrawn fit predicted at each sampled size |

## research/topics/entropy-area-law (results, compute, article) and web/src/data/entropy-area-law.json

1. Where the old values came from.
   - `hInfinity` 0.67: a constant hard-coded in `compute/src/main.rs` (`println!("  \"hInfinity\": 0.67,")`), described as coming from an "offline width-n sweep". Never computed in the repo. h(3) and h(4) were hard-coded the same way.
   - `areaLawExponent` 0.0445: a least-squares fit of ln rho = -alpha n^2 through the origin over the three exact points n = 1..3, computed in the crate.
   - `distinctnessCollapseCells` 155: round(ln(1000)/alpha), an extrapolation of that fit.
2. Compute changed (not rerun: the B(3) search needs a Rust build and minutes on 8 cores). The crate no longer emits the constant 0.67, the fit or the extrapolation. It emits `hInfinityUpperBound` (the smallest computed h(n)) and the width-2 increment, keeps the per-n `alphaByN` as a description of the exact points, and carries, verbatim and with their source named, h(3) to h(6), the increments for widths 3 to 6, the sampled fractions, the 44-cell crossing and a `withdrawn` block. The committed results file is what the edited crate prints, wall-clock seconds aside.
3. Results file, fields.
   - Old: `"hInfinity": 0.67` / New: `"hInfinityUpperBound": 0.6400` with a note that no value of the limit is computed, plus `hIncrementByWidth` (0.4350, then 0.4352 four times). Evidence: `strip_entropy.out`, `entropy_fields.out` (h(6) = 0.6400; 0.67 exceeds it, impossible for an infimum).
   - Old: `hByWidth` widths 1 to 4, widths 3 and 4 "from the offline width-n sweep" / New: widths 1 to 6 with `lambda` and `source` ("compute crate" for 1 and 2, "strip_entropy.js" for 3 to 6). Evidence: `strip_entropy.out`.
   - New: `sampledDistinctFraction`, six rows (2x2 0.8921, 3x3 0.6473, 4x4 0.3263, 4x6 0.1060, 4x8 0.02383, 4x12 0.000375, with standard errors and draw counts). Evidence: `distinct_fraction.out`.
   - New: `distinctFractionBelowOnePerThousandCells` 44. Evidence: `entropy_fields.out`, 44.2 cells between the 32- and 48-cell samples.
   - Old fields moved under `withdrawn` (date 2026-11-13) with their old value, old note and a reason: `hInfinity` 0.67 (not computed; exceeds h(6)), `areaLawExponent` 0.0445 (the fit gives 0.118 at 48 cells against the sampled 0.000375, 315 times higher; -ln(rho) per cell rises from 0.070 at 16 cells to 0.164 at 48), `distinctnessCollapseCells` 155 (from the withdrawn exponent; the samples cross 1e-3 near 44 cells). Evidence: `entropy_fields.out`.
4. `web/src/data/entropy-area-law.json` is a byte copy of the results file.
5. article.md. Summary rewritten to the measured values; the h(n) table extended to width 6 with sources; the area-law fit section replaced by the sampled table; a "What was withdrawn" section; "0 < h_infinity" softened to the upper bound only (strict positivity is not shown by the argument); the off-site 0.085 comparison dropped with the fit. `updated` 2026-11-13. `compute/README.md` changed to match.
   - Old summary: "the hardness lives entirely in the use-each-piece-once rule [...] fit an area law with exponent alpha near 0.044."
   - New summary: "colour-valid patches grow exponentially (the strip entropy h(n) is 0.640 per cell at width 6, an upper bound on the limit); the share of those patches that use distinct pieces is counted exactly to 3x3 (0.892, 0.648) and sampled beyond (0.33, 0.11, 0.024, 0.0004 at 16, 24, 32, 48 cells), crossing one in a thousand near 44 cells."

## web/src/components/research/EntropyScarcityLab.tsx

1. `note` (EN, FR, ES).
   - Old: "The in-repo fit over blocks up to 3×3 gives an area-law exponent near 0.044; extrapolating, the fraction drops below one in a thousand near 155 cells, the scale of the smallest moves between the best known boards. The hardness isn't in matching colours [...]; it's in the quiet once-each rule [...]"
   - New: "Past 3×3 it is sampled, not counted: for patches four cells wide, about 33% at 16 cells, 11% at 24, 2.4% at 32 and 0.04% at 48, below one in a thousand near 44 cells. Over that range it falls faster than any fixed area law. Matching colours alone leaves plenty of valid patches; the use-once rule is what removes most of them."
   - Evidence: `distinct_fraction.out`, `entropy_fields.out`. The link to the moves between record boards was dropped: their loops match a random null (`sigma_null.out`).
2. `n3note` (EN, FR, ES): adds "random sampling puts the share that survives near 33%" at 4×4. Evidence: `distinct_fraction.out` (0.3263).
3. Header comment: "That decline, compounding over the AREA of a patch, is the wall" removed. Strings and comments only; no type changes.

## research/topics/parity-defect-floor (census.json, checker, PLAN, article)

1. Checker `compute/src/main.rs` (not rerun: needs a Rust build).
   - Old: counts interior near-twin pairs (50), half-turn tiles (23), quarter-turn tiles (3) and asserts `defect2_generator_bound` = 50 + 23 + 3 = 76.
   - New: counts near-twin pairs per kind with the turn of the second tile restricted to those that keep its grey sides where the first has them (interior 50, edge 73, corner 2), the swaps (125, one alignment per pair), half-turn tiles (23), quarter-turn tiles (3), their union (23), the turns (23 + 2 x 3 = 29) and asserts `defect2_single_move_floor` = 125 + 29 = 154.
   - Evidence: `defect2_moves.out` (independent stdlib recount; the 3 quarter-turn tiles are all among the 23 half-turn tiles; each gives two boards, 90 and 270). Agrees with `piece_facts.out`.
2. `results/census.json`: the `defect2_generator_bound` check replaced by the nine checks above, all `ok`; 21 checks, `all_ok: true`, written as the edited checker prints it.
3. Why 76 was wrong: it left out the 73 edge and 2 corner near-twin pairs and the second direction of each quarter turn, counted the 3 quarter-turn tiles as separate tiles when they are among the 23, and called a count of moves that work in every solution an upper bound, when it bounds the 478 neighbours from below (a swapped pair sitting side by side across its differing side is the one exception; adjacent swaps that change the shared colour can add more).
4. article.md and `compute/PLAN.md`: the 76 ceiling restated as a floor of about 154, with the withdrawal stated; check table and count (17 to 21) updated; `updated` 2026-11-13.

## build/exact/lp-relaxations

1. Pairing-LP paragraph.
   - Old: "Freeze every piece in one fixed orientation and the bound is 307.00, integral and matching a closed-form count, but unsound for the real puzzle because rotations are disallowed."
   - New: "Freeze every piece in the orientation the published piece file stores it in and the bound is 307.00 [...] The 307 describes that stored orientation, not the pieces: random fixed orientations of all 256 pieces cap near 402 (2,000 draws, standard deviation 9.6), and a solution's orientations allow 480."
   - Evidence: `piece_facts.out` (stored cap 307; random orientations mean 402.4, sd 9.6, range 369 to 436; a solution's cap 480).
2. Ladder recap: "fixed rotations 307" changed to "stored rotations 307".

## open-problems, the area-law rigidity item

1. Heading, callout, wall, tried, tag.
   - Old: "Is local rigidity a consequence of the area-law distinctness collapse?" [...] "The smallest moves that separate the best known boards and the scale at which genuinely distinct partial boards collapse are both a patch of low hundreds of cells." [...] "Entry tag. Hard, well-mapped."
   - New: "Does the distinct-piece collapse bear on local rigidity?" [...] "This question was first asked because the two seemed to share a scale [...] They do not. [...] below one in a thousand near 45 cells, not 155 as first extrapolated. The loops that separate the best known boards have a median of 119 cells, the same as between two random reshuffles of the pieces." [...] "Entry tag. Unattended, premise withdrawn."
   - Evidence: `distinct_fraction.out`, `entropy_fields.out` (44.2 cells), `sigma_null.out` (record pairs median 119; class-preserving null median 119). "Rigidity is established independently, by MIP halo-optimality proofs" softened inside the item to "measured independently, on the boards tested", matching the rigidity page's `rigor: measured`.

## why/walls-and-methods

1. Rigidity row, caveat.
   - Old: "The MIP proof table and the board-wide ≤476 bound are sourced to the off-site paper"
   - New: "[...] the ≤476 bound are sourced to the off-site paper; that bound holds only for boards that keep the incumbent outside its top four rows (469 + 123 − 116), not board-wide"
   - Evidence: `why/rigidity-wall.mdx`, the top-four-rows region: dual bound 123 against the incumbent's 116 with the rest of the board fixed; 469 + 7 = 476. A region bound with the outside fixed says nothing about boards that differ outside it.
2. Border balance row.
   - Old: "An exact necessary condition; the four known full solutions satisfy it;"
   - New: "An exact necessary condition, satisfied by every complete solution by definition;"
   - Evidence: no full Eternity II solution is known (`build/analysis/solution-counting`); the "four known full solutions" have no committed source; that they were the four clue puzzles is an unverified guess. The identity holds for every solution by definition, so no sample is needed.

## why/theorem-sweep

1. Entropy entry.
   - Old: "**The entropy area law (measured).** [...] shows the first rule is generous and the second carries most of the hardness, with piece distinctness collapsing the count of legal blocks at a measurable scale. The exponent is a fit over the small exactly counted range."
   - New: "**Entropy and distinct pieces (measured).** [...] the share of those patches that use distinct pieces falls below one in a thousand near 45 cells (counted exactly to 3×3, sampled beyond on patches four cells wide), faster than any fixed area law. That says the second rule removes most locally valid patches; it does not show that the first rule is easy, since edge matching alone is NP-complete in general."
   - Evidence: `distinct_fraction.out`, `entropy_fields.out`; the NP-completeness references are those on the entropy page (Demaine and Demaine 2007; van Emde Boas 1997).

## why/irreducible-hard-region

1. Coupling paragraph.
   - Old: "Our boards come from the kit's framed generator, which is planted-solvable and plausibly admits many solutions, where the real puzzle's first-moment estimate is about 14,702 with the centre clue"
   - New: "[...] planted-solvable and, like the official set without clues, expected to admit many solutions: the first-moment estimate behind solution counting gives about $10^{7}$ for a random framed 16x16 board with 17 interior and 5 frame colours and no clues, against about $1.15\times10^{7}$ for the official set with no clues (about 14,702 once the centre clue is fixed)."
   - Evidence: `crossing.out`: census column at N = 16, 10^7.1; check line, official counts with no clue 10^7.062 = 1.153e7 and with the starter 14,702.3. The generated boards carry no hints, so the no-clue figure is the like-for-like one; the old sentence set a no-clue family against a with-clue estimate.

# Integration: code-produced results and leftovers

This section reruns the two corrected checkers, removes the last copies of the
record and of the withdrawn entropy framing, and aligns pages that disagreed
with each other. Same format: page or file, old text, new text, evidence.

## Records constants (web/src/data/records-timeline.ts and its readers)

1. One home for the strict five-clue record.
   - Old: `ExperimentScoreChart.tsx`, `BenchmarkLeaderboard.tsx`, `DfsStudyLeaderboard.tsx`, `pages/Status.tsx` and `RecordsView.tsx` each ran `RECORDS.find((r) => r.strictRecord)` with its own guard.
   - New: `records-timeline.ts` exports `STRICT_RECORD_ROW` and `STRICT_RECORD_SCORE` (466 today), read by all five.
   - Evidence: the row flagged `strictRecord` (board `FiveClue_466_2c037e70f7e9`, re-verified in `research/topics/record-boards`).
2. The open record is not given the same treatment: no timeline row carries an open-record flag (the 470 appears twice, 2021 and the 2024 tie), so `CEILING = 470` in `ExperimentScoreChart.tsx`, `recordScore: "470 / 480"` in `pages/Status.tsx` and the `Puzzle.tsx` table stay as written. Adding an `openRecord` flag is a data-model change left for a later step.
3. `RecordTimeline.tsx` idle caption.
   - Old: "the strict five-clue track climbed from 460 to 465 in 2026."
   - New: "the strict five-clue track went from 460 (2023) to 466 (2026)." The two ends are read from the first `regime: "strict"` row and from `STRICT_RECORD_ROW`.
   - Evidence: Gauthier's 460 is dated 2023-03-09 in the timeline; the 2026 climb went 464 (July), 465 (August), 466 (September). The old caption had the wrong end point and placed the 460 in 2026.

## Dead record fields in the experiment data

1. `web/src/data/dfs-study.json` `community_5clue_record: 465` and `web/src/data/single-core-benchmark.json` `community: 465` removed, with their type declarations in `DfsStudyLeaderboard.tsx` and `BenchmarkLeaderboard.tsx`; `research/experiments/dfs-study/scripts/make_site_json.py` and `research/experiments/single-core-benchmark/scripts/make_site_json.py` no longer write them (both generators still wrote 464, so the committed JSON had been hand-edited away from its generator).
   - Evidence: no component, plugin or script reads either field (grep over `web/src`, `web/plugins`, `web/scripts`, `research/experiments`); the `community` engine array in `dfs-study.json` is a different field and is kept.

## why/walls-and-methods

1. The four walls.
   - Old: "**The area law**. Genuinely-distinct partial boards collapse as the filled area grows, but no local score can see that global fact." Column "Area law"; "the anchor methods [...] hit the area-law wall in the interior"; metaDescription "the area law".
   - New: "**The use-once rule**. Among colour-valid patches, the share that uses distinct pieces falls fast as the patch grows (measured on patches up to 48 cells), and no local score sees a repeat that has not happened yet." Column "Use-once rule"; "run into the use-once rule in the interior".
   - Evidence: `distinct_fraction.out` (sampled up to 4x12 = 48 cells); the area-law exponent is withdrawn (section above).
2. Three shifts paragraph.
   - Old: "with the exponent fit over the exactly-counted range."
   - New: "the area-law exponent once fitted to them is withdrawn, because sampled larger patches fall far faster than the fit."
   - Evidence: `entropy_fields.out`, fit/sample ratio 315 at 48 cells.
3. The shape of the gap.
   - Old: "The community's 468 to 470 boards and the strict-track 465 came from fast depth-first search plus large compute"
   - New: "The community's 468 to 470 boards (2020 to 2024) and the strict-track 465 of August 2026 came from [...]"
   - Evidence: `records-timeline.ts` rows; the standing strict record (466) is on `/research/records`, already linked in the sentence.

## why/theorem-sweep, why/prune-vs-speed, why/permutation-code-wall, build/analysis/solution-counting, build/reduce/nogood-learning

1. Link texts and summaries that named the withdrawn area law.
   - theorem-sweep metaDescription. Old: "a measured entropy area law". New: "measured entropy and use-once counts".
   - prune-vs-speed. Old: "**The area law**: the count of genuinely-distinct partial boards collapses as the filled area grows". New: "**The use-once rule**: the share of colour-valid patches that use distinct pieces falls fast as the patch grows".
   - nogood-learning. Old: "a different quantity from the volume term that the entropy area law page fits." New: "a different quantity from the distinct-piece fraction that the entropy and use-once rule page measures." (No term is fitted any more.)
   - permutation-code-wall, solution-counting, theorem-sweep: link texts "area law" / "entropy and the area law" now read "use-once collapse", "entropy", "entropy and use-once results", "entropy and the use-once rule".
   - Evidence: the entropy page no longer states an area law (section above).

## why/index

1. Shared scale.
   - Old: "one measured as the collapse of genuinely distinct boards past a patch of about 45 cells" [...] "whether rigidity is itself a consequence of the distinctness collapse it shares a hundred-odd-cell length scale with"
   - New: "one measured as the share of colour-valid patches that use distinct pieces falling below one in a thousand near 44 cells (patches four cells wide, sampled)" [...] "whether rigidity is itself a consequence of the distinctness collapse"
   - Evidence: `entropy_fields.out` (crossing at 44.2 cells); the shared scale was withdrawn on `open-problems` (record-pair loops have a median of 119 cells, the same as the class-preserving null, `sigma_null.out`), so the hub no longer repeats it.

## The 44-cell crossing, site-wide

1. Old: "near 45 cells" on entropy-area-law, theorem-sweep, walls-and-methods, open-problems and why/index. New: "near 44 cells", as in `EntropyScarcityLab.tsx` and `results/entropy.json` (`distinctFractionBelowOnePerThousandCells: 44`).
   - Evidence: `entropy_fields.out`, log-linear interpolation between the sampled 32-cell and 48-cell fractions crosses 1e-3 at 44.2 cells.

## why/border-balance

1. Old: "they agree for every solution of every puzzle, the four known full solutions included." New: "they agree for every solution of every puzzle."
   - Evidence: no full Eternity II solution is known; the identity holds by definition and needs no sample. The walls-and-methods row was corrected the same way earlier today.

## why/rigidity-wall

1. Old: "so even the unclosed case cannot exceed the wall by much (it implies a board-wide bound of ≤ 476)."
   - New: "so re-solving those rows with the rest of McGavin's 469 held fixed gains at most 7 edges: no board that keeps the 469 unchanged outside its top four rows scores above 476. It says nothing about boards that differ elsewhere."
   - Evidence: the region's dual bound 123 against the incumbent's 116, outside fixed: 469 + 7 = 476.

## why/entropy-area-law

1. Title. Old: "Entropy and the area law". New: "Entropy and the use-once rule" (FR "L'entropie et la règle d'usage unique", ES "La entropía y la regla de uso único"). URL and slug unchanged; `research/topics/entropy-area-law/article.md` and its compute README retitled to match.
2. `repro.scope`. Old: "The committed results file still carries values this page has withdrawn (h_infinity 0.67, the area-law exponent near 0.044 and the 155-cell collapse)." New: the results file carries the script values verbatim with their source named, and the withdrawn values only under a `withdrawn` key with the reason.
   - Evidence: `results/entropy.json` as regenerated by the crate (Part 1 below).
3. Width chart. Old: x axis `domain={[1, 4]}`, 4 ticks, which clipped widths 5 and 6. New: `domain={[1, 6]}`, 6 ticks.
   - Evidence: `hByWidth` has six rows.

## build/exact/sat-csp-encodings and build/dead-ends (cube and conquer)

1. sat-csp-encodings. Old: "has not been attempted at scale on the full puzzle as far as we know (checked 2026-11)." New: "has no public attempt at scale on the full puzzle (checked 2026-11); the dead ends page grades the one unpublished report as an untested claim."
2. dead-ends. Old: "It joins the long line of exact methods that meet the same wall [...]" New: "If the report holds, it joins the long line [...]", since the entry is graded an untested claim.
   - Evidence: the dead-ends entry already said no public or committed source reports a run at scale; the encodings page said no attempt existed at all, which the dead-ends entry's private report contradicts.

## Part 1: the two checkers, rerun

Both checkers were edited and their JSON written earlier the same day without
running them. Both were built and run for this section, and each output was
compared byte for byte with the committed file.

1. `research/topics/parity-defect-floor`: `cargo run --release` output is byte-identical to the committed `results/census.json`. Nothing changed.
2. `research/topics/entropy-area-law`: the crate was built from a copy that differs from the committed source only in its thread count (one thread instead of up to eight, so as to run on one core). The output is byte-identical to the committed `results/entropy.json` except for the three `wallSeconds` fields (n = 2: 0.16 against 0.22; n = 3: 1661.99 against 420.49; n = 4: 7.22 against 12.42), which are wall-clock times and differ between runs and thread counts by design (the compute README says so). Every count, ratio, eigenvalue and carried value matches, including A(3) = 195,941,569,440, B(3) = 126,907,480,216 and A(4) = 62,607,390,876,341,976. The committed file was kept, since its times come from the unmodified multi-threaded command; `web/src/data/entropy-area-law.json` is already byte-identical to it.
3. The correction scripts: `piece_facts.py`, `crossing.py`, `defect2_moves.py`, `duplicates_by_kind.py` (13 s), `sigma_null.py` and `entropy_fields.py` each reproduce their committed `.out` byte for byte under `python3 -I`. `strip_entropy.js` reproduces widths 1 to 5 exactly, and `node strip_entropy.js 6 6` gives lambda 6922.392 and h(6) = 0.6400 (305 s), as committed; only the printed seconds differ. `distinct_fraction.js` was not rerun (a multi-minute sampling run). `complex_theory_reference.out` has no script beside it.
