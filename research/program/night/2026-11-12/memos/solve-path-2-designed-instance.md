# Solve path 2. The designed instance: what the making process leaves behind

Night 2026-11-12, researcher 2 of 6. Lens: this instance was made by someone.
Scripts and raw outputs: `research/program/night/2026-11-12/memos/solve-path-2/`
(`01_piece_set_traces`, `02_bits_needed`, `03_stratified_painters`, each with its
`.py`, `.out` and `.json`). Total compute tonight: about 4.5 minutes on one core.

## 0. Verdict in one paragraph

The lens gives a clean negative and one narrow loophole. If the maker's program
drew the solved board from a distribution that depends on the board only through
its piece set (a fixed colour multiset dealt over the joints, plus any rejection
rule that looks at the pieces), then the piece set carries exactly zero bits about
the arrangement beyond validity. That is a two-line theorem (section 2), not an
empirical claim. Every documented fact about the generator fits that model, and
twelve piece-level statistics computed tonight on the official set sit inside a
matched null (largest |z| 1.37, smallest one-sided p 0.063 before any correction).
The piece numbering is a pure sort (verified on the data), the backs carry only
that number, the clue puzzles are a different design, the clue positions are a
symmetric choice of the maker. The loophole is a painter that balances colours
over coarse regions of the board. Such a painter is invisible in the piece set
(shown by simulation), would carry 29 to 118 bits depending on the partition, and
can only be tested by running the search under it. A solver needs about 93 more
bits than three desktops can pay for in a year. So the approach below is a
conditional one, and my probability for it is about 1 in 20,000.

## 1. Documented history, with the status of each source

Fetched and read tonight:

- Wikipedia, "Eternity II puzzle" (<https://en.wikipedia.org/wiki/Eternity_II_puzzle>).
  Monckton developed the puzzle; Alex Selby and Oliver Riordan wrote the program
  that generated the final design. 256 pieces (4 corner, 56 edge, 196 inner).
  The back of each piece is blank except for an identifying number. A mandatory
  starter piece near the centre. Clue Puzzles 1 (6x6) and 2 (12x6) at launch on
  28 July 2007, Clue Puzzles 3 (36 pieces) and 4 (72 pieces) in 2008; each solved
  clue puzzle gives one piece position on the main board. The rule book says the
  puzzle can be solved without the hints. First scrutiny 31 December 2008: no
  complete entry, 10,000 dollars for 467 of 480 (Louis Verhaard). Contest closed
  31 December 2010, prize unclaimed. No verified solution published.
- plus.maths.org, "Forever rich", 5 February 2007
  (<https://plus.maths.org/content/forever-rich>). Launch material: "There's not
  just one, but thousands of different solutions"; a computer holding one of the
  solutions was destroyed as a publicity act.
- Krzakala and Zdeborova, "Hiding quiet solutions in random constraint
  satisfaction problems", Phys. Rev. Lett. 102, 238701 (2009)
  (<https://arxiv.org/abs/0901.2130>). The planted ensemble can be statistically
  indistinguishable from the random one.

Found by search tonight, abstract level only:

- Martinsson, "A linear threshold for uniqueness of solutions to random jigsaw
  puzzles", Combin. Probab. Comput. 28 (2019) (<https://arxiv.org/abs/1701.04813>).
  For an n by n random jigsaw with q edge types, at least two solutions with high
  probability when q <= 2 e^(-1/2) n, unique when q >= (2 + eps) n. For n = 16,
  2 e^(-1/2) n = 19.4. The model is not edge matching with rotations, so the
  constant does not transfer; the point is that E2's 17 interior colours sit at
  the same scale as a proven uniqueness threshold.
- Ansotegui, Bejar, Fernandez, Mateu, "How hard is a commercial puzzle: the
  Eternity II challenge" (2008), University of Lleida repository copy at
  <https://repositori.udl.cat/server/api/core/bitstreams/0b6533fe-54e5-4070-85fe-80f7d35837d8/content>.
  Generator and phase-transition study of generic edge-matching puzzles.
- Moharrami, Moore, Xu, "The planted matching problem"
  (<https://arxiv.org/abs/1912.08880>). The nearest formal model of "recover the
  planted pairing from pair statistics"; recovery there needs planted edges to
  have a different weight law from the rest.

Taken from the repository's digests of the groups.io archive
(`research/community/digests/`), not re-fetched: groups.io returned HTTP 402 to
the fetch tool tonight, web.archive.org is blocked for it, and archduke.org
returned 403. Treat these as second-hand until someone opens the links.

- The numbering of the pieces is a sort of the pattern codes in the instruction
  booklet's symbol order (msgs 3372, 6174, 7237; digests 0003, 0006, 0008).
- Selby and Riordan wrote the generator, sourced to a Times article (msg 3373).
- The only account of the generation itself is a recollection of a phone call
  (Dave Clark with Monckton, 26 July 2007, msg 4177, quoted in
  `web/content/research/why/design-recipe.mdx`): about 200 inputs typed by the
  judges as entropy, regeneration until the judges were satisfied, one printout,
  vaulted; the random generator described as "gaussian residues of powers of
  appropriately chosen primes". Rumour grade.
- Tomy launch leaflet (msg 901): nobody knows the solution; it was printed
  between pages of random text with everyone out of the room.
- Clue rewards give position and rotation and are the same for every entrant
  (digests 0002 line 189, 0009 line 79). The entry form listed piece numbers
  only, without rotations (digest 0006 line 105).
- Solution counts of the clue puzzles: Clue 1 about 3.6e10, Clue 3 exactly
  2,195,647,488 (digest 0010 lines 61 and 109).
- Expected solutions: 14,702 with the starter, 4e-8 with all five clues (McGavin,
  msg 11193, digest 0015 line 128). The brief for this memo says "about 1 with all
  five clues"; the two agree only if the 1 is the planted solution itself. The
  expected number of other solutions is about 5e-8 (recomputed below).

What the 2008 to 2010 claims revealed about the solution: nothing. Scrutiny
checked entries against the rules and published one score. No partial placement,
no count of matching pieces against the vaulted solution, was ever released.

## 2. The mathematics of a trace

### 2.1 Zero-information theorem

Let B be the maker's solved board, drawn from a distribution mu, and let
S = pieces(B) be what is published (the multiset of pieces, positions and
rotations forgotten), together with the clue placements, whose cells were fixed
in advance.

Claim. If mu(B) depends on B only through pieces(B), then, given S and the clues,
B is uniform over Sol(S), the set of clue-respecting solved boards with piece
multiset S.

Proof. P(B | S) is proportional to mu(B) 1[pieces(B) = S] = f(S) 1[B in Sol(S)]. QED.

The hypothesis covers: dealing a fixed multiset of colours uniformly over the
joints of each class; rejecting sets with duplicate or rotation-symmetric pieces;
rejecting sets whose colour counts are not flat; rejecting sets that the makers'
own solver found too easy; "regenerate until the judges are satisfied" if the
judges saw only pieces. All of these are functions of S.

Consequences.

1. No statistic of the piece set can distinguish the planted solution from any
   other solution, and no statistic can rank "true neighbours" except through
   the marginals of Sol(S), which is the counting problem itself.
2. The spectral test of agenda A1 (`research/experiments/spectral-planted-recovery/`,
   committed, killed by an independent judge) ran on boards from the repository's
   generator, which is an exchangeable painter. Under the theorem the only signal
   available there is the solution marginal, so A1 measured whether a spectral
   proxy approximates that marginal better than colour rarity. It does not. That
   result transfers to the official set if and only if the official painter is
   also exchangeable. If it is not, A1 says nothing about the official set, and
   neither does any other test on generated boards.
3. A trace can carry information only if mu depends on the arrangement beyond
   the piece set. The number of bits is at most log2(|all arrangements| /
   |arrangements the painter can produce|), the support reduction.

### 2.2 Bits a solver needs (computed tonight, `02_bits_needed.py`)

Model: the official colour-side counts kept per class (56 inward border sides,
784 inner-piece sides, 60 + 60 ring sides of the two handednesses), dealt
uniformly over the sides of each class. The probability that a fixed geometry
matches everywhere is exact for that ensemble:

    P = prod_c a_c! / 60!                       (ring, a_c = 12)
      x prod_c (m_c)_(s_c) / (784)_(56)         (seam, s_c inward sides of colour c)
      x prod_c (m_c - s_c - 1)!! / 727!!        (inner)

For partial boards the same quantity is spread evenly over the joints of each
class (effective match probability 1/16.70 per inner joint, 1/17.20 per seam
joint, 1/4.39 per ring joint). That spreading is the one approximation. The
expected node count of a row-scan backtracker that checks left and top joints,
and joints toward a clue as soon as their other end is placed, is the sum over
depths of the expected number of valid partial boards.

| quantity | centre clue only | five clues |
|---|---|---|
| expected solutions (other than the planted one) | 17,130 | 4.8e-8 |
| expected tree, placements | 2^172.7 = 10^52.0 | 2^150.5 = 10^45.3 |
| depth of the widest level | 161 | 161 |

Checks against the repository: `web/content/research/build/analysis/solution-counting.mdx`
gives 14,702 with a formula that pools seam and inner sides; keeping the seam as
its own class moves it to 17,130, a factor 1.17, inside McGavin's "factor of 2".
The same page quotes a plateau about 10^45 wide. Neither repository number is
backed by a committed result file; both are reproduced here from the piece set.

Adding correct placements one at a time, at the cell where the model says each
helps most, buys 9.5 to 9.6 bits per placement (log2 of 196 x 4 = 9.61), down to
a floor near 2^24 after 13 or 14 of them.

| budget | placements affordable (at 1e8 per core-second, assumed) | bits short | extra correct placements needed |
|---|---|---|---|
| 3 desktops x 16 cores x 1 year | 2^57.1 | 93.4 | 10 |
| 1,000 core-years | 2^61.5 | 89.0 | 10 |

Cross-checks. (a) McGavin's measured run: 18 scattered hints ("every third
column on a few odd rows") solve a 16x16 E2-like board in under 15 minutes
(`web/content/research/why/hint-geometry.mdx`, community measurement, not rerun
here), which is 2^33 to 2^36 placements at 1e7 to 1e8 per second. The model
gives 2^32.7 and 2^34.5 for two 18-hint lattices on rows 1, 3, 5, 7, and 2^67.8
for 18 hints spread over rows 1, 5, 9, 13. His exact cells are not in the
repository, so this is agreement in order of magnitude for a top-rows layout,
and a reminder that the same number of hints can be worth 35 bits more or less
depending on where they sit. (b) The repository's backdoor
result (`research/experiments/sat-backdoors/results/summary.json`, committed,
judged keep): pure propagation completes a board from k planted cells with
k = 4.90 at N = 36 and exponent 0.589. Extrapolated to N = 256 that is about 16
cells (power law) or 20 (the judge's affine fit). Both extrapolate far outside
the measured range (N <= 100) and across a change of colour schedule, so read
them as "ten to twenty", which agrees with the tree model.

So the requirement is: about 93 bits beyond the five clues, or equivalently ten
more correct, well-placed pieces. The four optional clues bought only 22 bits of
tree in this model (2^172.7 to 2^150.5) while cutting the solution count by
2^38.4, because of where they sit relative to a row scan.

### 2.3 Bits each candidate trace could carry

| trace | bits about the arrangement | basis |
|---|---|---|
| piece numbering as shipped | 0 | The official list is exactly the lexicographic sort of each piece's minimal rotation, corners then edges then inner (checked tonight on `engine/data/official_eternity2.csv`: fully sorted, all 196 inner pieces stored in minimal rotation). A sort is a function of S. That the printed numbers equal this list rests on msgs 3372, 6174, 7237. |
| printing on the back | 0 | Backs carry only the number (Wikipedia). |
| colour frequencies (24 sides per frame colour; 48 or 50 per inner colour) | 0 | Function of S. |
| colour pairings on pieces | 0 if the painter is exchangeable | Section 3: twelve statistics, all inside the null. |
| clue positions | 0 beyond the placements | Four cells at the corners of a 12x12 square plus the centre: chosen by the maker, not by the solution. The five placements themselves are already counted (22 bits of tree). |
| clue puzzles as samples of the generator | 0 | Different design: 2 or 3 dominant inner colours, 9, 48, 9 and 26 duplicate pairs, 2 or 3 rotation-symmetric pieces each (computed tonight from `research/topics/clue-puzzle-pieces/data/`), and 10^9 to 10^10 solutions, so the maker's own solution of a clue puzzle cannot be identified and its parameters say nothing about the main painter. |
| pseudo-random generator | all of it in principle, none in practice | The program is not public. The one description (rumour grade) reads like a Blum-Blum-Shub generator seeded with a few hundred bits. Without the code there is no seed space to search. |
| scrutiny outcomes 2008 to 2010 | 0 | Nothing about the vaulted solution was released. |
| spatially stratified painter (loophole) | 29 to 118, see section 4 | Not excluded by the piece set when the strata are coarse. |

One observation I cannot grade: in the repository's colour labelling the five
inner colours with 48 sides are exactly labels 6 to 10, the first five inner
labels (chance 1 in 6,188 for a random labelling). If the labels follow the
booklet and the booklet follows the generator's own indexing, that is what a
program building a fixed colour multiset ("the first five colours get 24 joints,
the rest 25") and dealing it out would leave, which is the exchangeable painter.
If the community file was relabelled by frequency, it means nothing. I did not
find the provenance of the labelling in the repository.

## 3. Numerical evidence on the official set (`01_piece_set_traces.py`)

Null ensemble: a solved 16x16 painted with the official colour multiset (12 ring
joints per frame colour; 24 or 25 joints per inner colour over the 56 seam and
364 inner joints), every arrangement equally likely, read off as pieces. 30,000
boards, seed 20261112. The last two columns condition the null on the known
filter (no duplicate, no rotation-symmetric piece; 247 of the 30,000 boards).
Twelve statistics were fixed before the run; all twelve are reported.

| # | statistic | what it would reveal | official | null mean | sd | z | P(<=) | P(>=) | cond. mean | cond. z |
|---|---|---|---|---|---|---|---|---|---|---|
| T1 | dispersion of seam colour counts | painter balances colours on the seam separately | 35.53 | 45.94 | 15.82 | -0.66 | 0.299 | 0.749 | 42.09 | -0.46 |
| T2 | dispersion of corner-side colours | corners designed | 5.20 | 5.68 | 3.70 | -0.13 | 0.660 | 0.609 | 5.31 | -0.03 |
| T3 | edge pieces with the same frame colour on both ring sides | ring designed | 14 | 10.43 | 2.90 | 1.23 | 0.915 | 0.145 | 10.55 | 1.23 |
| T4 | chi-square of the 5x5 ring colour-pair table of edge pieces | edge pieces designed as a balanced set | 8.90 | 16.29 | 5.38 | -1.37 | 0.063 | 0.937 | 14.39 | -1.13 |
| T5 | seam sides carried by the five 24-joint colours | rare colours steered to or from the seam | 16 | 16.03 | 3.17 | -0.01 | 0.569 | 0.560 | 16.14 | -0.05 |
| T6 | inner pieces' equal opposite sides | line or block stratification | 23 | 22.16 | 4.60 | 0.18 | 0.621 | 0.460 | 20.89 | 0.45 |
| T7 | inner pieces' equal adjacent sides | block stratification, local exclusion rules | 47 | 44.33 | 6.45 | 0.41 | 0.693 | 0.363 | 44.45 | 0.41 |
| T8 | mean number of distinct colours per inner piece | same, pooled | 3.658 | 3.673 | 0.036 | -0.40 | 0.368 | 0.685 | 3.678 | -0.55 |
| T9 | chirality: clockwise (a,b) against (b,a) | raster painting with a correlated generator | 109.84 | 122.69 | 14.30 | -0.90 | 0.187 | 0.813 | 119.75 | -0.75 |
| T10 | chi-square of adjacent colour pairs | colour affinities | 148.78 | 135.19 | 16.50 | 0.82 | 0.801 | 0.199 | 133.83 | 0.92 |
| T11 | chi-square of opposite colour pairs | colour affinities along an axis | 127.56 | 135.70 | 16.17 | -0.50 | 0.317 | 0.683 | 134.13 | -0.45 |
| T12 | top eigenvalue of (opposite pairs minus half the adjacent pairs) | colours split into horizontal-preferring and vertical-preferring groups | 15.91 | 15.35 | 1.54 | 0.36 | 0.676 | 0.324 | 15.23 | 0.45 |

Reading. Nothing. The smallest one-sided tail is T4 at 0.063; with twelve
statistics and two tails each the Bonferroni line for 5 percent is 0.0021 per
tail, and the chance that at least one of twelve independent statistics shows a
tail below 0.063 on either side is about 0.80.
The only departure from free painting that the official set shows is the one
the repository already committed (`research/topics/adversarial-piece-set/results/null_model.json`:
zero duplicate pairs against a null mean of 4.09, tail 0.0138), and a duplicate
filter is a function of S.

T1 deserves a sentence. The seam is the one region boundary that is visible in
the piece set, because an edge piece shows which inner colour it turns inward. A
painter that balanced colours region by region would give seam counts of 3 or 4
for every colour (dispersion about 3.5). The official dispersion is 35.5, typical
of free dealing. So where a spatial balance could be seen, it is absent.

Also computed, not counted among the twelve: number of clue-puzzle pieces whose
colour quadruple also occurs in the main set under the files' own colour ids
(4, 4, 4, 2 of 36, 72, 36, 72). The two sources may not share a colour
numbering with the main file, so this is not evidence of anything.

## 4. The loophole, measured (`03_stratified_painters.py`)

Painters that spread every inner colour as evenly as possible over a partition
of the 420 inner-colour joints, uniform within a stratum. 3,000 boards each.
"Bits" is log2(all arrangements / arrangements with that count matrix), upper
value; the lower value subtracts a bound on the number of admissible count
matrices. Official values: 23 equal opposite pairs, 47 equal adjacent pairs.

| painter | strata | bits carried, low to high | mean equal opposite | mean equal adjacent | z of official sum (70) | P(sum >= 70) | P(opposite >= 23) |
|---|---|---|---|---|---|---|---|
| exchangeable | 1 | 0 | 22.24 | 44.25 | 0.44 | 0.343 | 0.460 |
| horizontal joints / vertical joints | 2 | 28.8 to 40.8 | 21.33 | 46.08 | 0.32 | 0.399 | 0.378 |
| quadrants | 4 | 93.5 to 117.5 | 20.51 | 41.20 | 1.10 | 0.153 | 0.317 |
| sixteen 4x4 blocks | 16 | 180.7 to 410.7 | 16.92 | 33.32 | 3.04 | 0.0017 | 0.088 |
| lines (each row of horizontal joints, each column of vertical joints) | 28 | 374.8 to 586.6 | 7.10 | 45.35 | 2.60 | 0.009 | 0 of 3,000 |

Reading. Fine stratification is excluded by the official set: sixteen blocks at
p = 0.0017, lines at p < 1/3,000 on opposite pairs. Coarse stratification
(two or four strata) cannot be seen in the piece set at all: its likelihood for
the official values is within a factor 2.3 of the exchangeable painter's. The
loophole is therefore real, bounded, and undecidable from the pieces.

## 5. The approach: search under a hypothesised painter prior

Stated so it can be built.

1. Hypothesis H(P): the maker's painter balanced each inner colour over a
   coarse partition P of the board's joints (candidates: horizontal/vertical;
   quadrants; halves; the seam is already excluded by T1).
2. Constraint: for every stratum s and colour c, the number of joints of colour
   c inside s lies in {floor(n_c/|P|), ceil(n_c/|P|)}. This is a global
   cardinality constraint that is not implied by validity. It is extra
   information, unlike the supply budgets of the dead-ends page (see section 7).
3. Search: the strict DFS of `engine/src/solver.rs` with one counter per
   (stratum, colour), updated per placement, and a fill order that completes
   strata early (quadrant by quadrant for the quadrant partition; plain row scan
   for horizontal/vertical). Prune when a counter exceeds its ceiling or can no
   longer reach its floor.
4. There is no way to confirm H(P) short of finding the solution. Each P costs
   one full search.

## 6. Cost model for the real 16x16

Need: 93.4 bits (three desktops, one year) or 89.0 bits (1,000 core-years),
from section 2.2, assuming 1e8 placements per core-second (the digests record
7e7 per 1.5 GHz core in 2012; not measured here) and a row-scan tree.

Supply under each hypothesis. A budget prunes only once a stratum is nearly
full, so the usable share of the prior is about the share of strata closed when
the tree is widest. The widest level is depth 161 of 256, so I take a usable
share between 0.5 and 0.75 (assumption S1). I also assume the stratum-first
fill order costs nothing against row scan (assumption S2, optimistic: the
community record says scan-line order is the cheapest known).

| hypothesis | bits carried | usable (S1) | short, 3 desktops | short, 1,000 core-years |
|---|---|---|---|---|
| exchangeable painter (what the evidence favours) | 0 | 0 | 93 bits, 10^28 | 89 bits, 10^27 |
| horizontal/vertical balance | 29 to 41 | 14 to 31 | 62 to 79 bits, 10^19 to 10^24 | 58 to 75 bits |
| quadrant balance | 94 to 118 | 47 to 88 | 5 to 46 bits, 10^1.5 to 10^14 | 1 to 42 bits |

Bottom line. With the painter the evidence points to, this approach is 28
orders of magnitude out of reach and is no approach at all. The single
assumption that would remove most of those orders is "the painter balanced
colours over quadrants (or a partition of similar coarseness)". If true, the
gap falls to between 1.5 and 14 orders. I put that assumption at about 1
percent: nothing documents it, makers who had just broken Eternity I through
its statistical irregularities had every reason to add no structure, the seam
shows no balancing, and the labelling observation of section 2.3 points to a
dealt multiset. I keep it above zero because the piece set cannot exclude it.

## 7. Why it is not already known to fail

- Closest on the site: "Border-supply pruning" and its "dynamic per-colour
  budget" variant (`web/content/research/build/dead-ends.mdx`, lines 816 to 856;
  the audit grades it reported, own rerun not found). Those budgets are supply
  conditions, implied by validity, so propagation already enforces them and they
  fire zero times. A stratum budget is not implied by validity. It removes
  boards that are perfectly valid partial tilings. Whether it is right depends
  on a fact about the maker, not on the puzzle's logic.
- Agenda A1 (spectral planted recovery, killed): tests an exchangeable
  generator, where the theorem already predicts no planted signal. It does not
  bear on a stratified painter.
- Community record: the numbering, steganography and "solution in the design"
  threads (msgs 3360 to 3373, 6173 to 6176, 7237 to 7254) and the generator
  speculation of 2015 (msgs 9538 to 9541) all end at "the numbering is a sort"
  and "guessing the generator is unlikely to work". I found no post that states
  the zero-information result, tests the piece set against a matched null for
  painter structure, or proposes arrangement-level priors.
- Literature: quiet planting (Krzakala and Zdeborova) is the same statement for
  random CSPs on sparse graphs; the site cites it for solution counts only
  (`solution-counting.mdx`). I found no paper on inferring a planted solution
  of an edge-matching puzzle from its piece statistics.

## 8. What is new

1. The zero-information theorem stated for this instance and used to sort every
   candidate trace into "function of the piece set, so zero bits" and
   "arrangement-level, so possibly non-zero". It retires a whole family of ideas
   at once: numbering, piece-pair fingerprints, classifiers of "true neighbours"
   trained on generated boards.
2. The statement of what A1 does and does not say about the official set.
3. A twelve-statistic battery on the official set against a matched null, all
   null, with the seam test as the one direct probe of regional balancing.
4. The loophole quantified: bits carried against visibility, with fine
   stratification excluded at p = 0.0017 and below, coarse stratification
   undecidable.
5. A requirement in bits from the piece set alone: 2^150.5 placements for row
   scan with five clues, 9.6 bits per correct placement, ten more placements or
   93 bits to reach a three-desktop year; and a class-aware first moment (17,130)
   next to the canonical 14,702.

The search itself (DFS with cardinality counters) is not new.

## 9. First experiment (under 10 core-hours)

Question: does a true stratum prior convert into tree reduction at the rate the
cost model assumes (S1 >= 0.5)?

Design. Generate planted boards with a quadrant-balanced painter at 8x8 and
10x10 on the colour schedules of the hard ladder (`research/experiments/hard-ladder/`,
rungs 8x8 c=10 and the 10x10 rung nearest the hardness peak), 30 seeds each.
Arms, same solver, same value order, same node definition as the ladder
protocol: (a) baseline strict DFS; (b) the same DFS with quadrant budgets and
quadrant-first fill order; (c) baseline DFS with quadrant-first fill order and
no budgets, to price assumption S2 separately. For every instance compute the
prior's bits (lower value, as in `03_stratified_painters.py`) and the
conversion rate = log2(nodes(c) / nodes(b)) / bits. Report also
log2(nodes(c) / nodes(a)), the order penalty.

Kill line, written before the run: the approach is dead if the median
conversion rate at 10x10 is below 0.5, or if it is lower at 10x10 than at 8x8
by more than two standard errors, or if the order penalty at 10x10 exceeds the
bits the budgets recover. Any one of the three is enough. Passing does not make
the approach likely; it only keeps the quadrant row of the cost table alive.

A zero-cost companion, already done tonight: the official piece set is
compatible with the quadrant painter (p = 0.15) and excludes the sixteen-block
and line painters.

## 10. Probabilities

- Three desktops, one year, no paid compute: 5e-5 (1 in 20,000).
- 1,000 core-years: 1e-4 (1 in 10,000).

Composition for the first number: painter is coarsely stratified, 1 percent;
the partition actually used is among the three or four one can afford to try,
0.3; conversion and order penalty land at the favourable end of section 6, 0.1;
that product is 3e-5, plus 2e-5 for routes inside this lens that I cannot
price (the generator's code or a designer's statement surfacing). The main
reason the numbers are not higher: the documented facts and all twelve
statistics fit a painter that leaves, by theorem, nothing to find.

## 11. The best idea I rejected

Recover the generator instead of the solution. If the program is a known
deterministic function of a seed of E bits, the solution costs 2^E board
generations of about 10^4 operations each, and E could be as small as 31 to 64
for a home-made generator. Rejected because the code is not public, the only
description is a second-hand phrase that fits a Blum-Blum-Shub design seeded
from about 200 typed inputs, and without the exact code every candidate design
multiplies the seed space by an unknown factor. It becomes the best path on
the day the program or its listing is published; nothing in the record says
that will happen.

## 12. Open points and limits of tonight's work

- Three source links are second-hand (groups.io, web.archive.org, archduke.org
  could not be fetched). The claims that rest on them are marked in section 1.
- The tree model spreads the exact full-board match probability evenly over
  joints; it is a first moment and ignores propagation, so real solvers do
  better by a constant number of bits that I did not measure.
- The 1e8 placements per core-second figure is an assumption.
- The stratified painters in section 4 are my constructions. A maker could have
  used a partition I did not think of; the bound that matters is the general
  one (fine partitions show up in T6 and T7, coarse ones do not).
- The provenance of the colour labelling (section 2.3) is unchecked.
