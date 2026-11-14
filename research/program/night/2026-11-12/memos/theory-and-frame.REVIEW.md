# Review of `theory-referee.md` and `frame-information.md`

Night 2026-11-12, written 2026-11-13. Independent referee. Scripts and outputs:
`research/program/night/2026-11-12/memos/theory-and-frame-review/` (`01` to
`07`, each `.py` with its `.out`). `01`, `05`, `06` use numpy through
`uv run --with numpy`; the rest are `python3 -I`. One core, about 7
core-minutes in all (one early run of `01` was stopped to remove a wrong
normalisation of my own; it did not affect any sampled number). No existing
file was edited, no git command was run.

Every "reproduced" below means recomputed by code I wrote, not by rerunning
the memo's script. Where I leaned on the refereed library `rv_lib.py` I say so.

## 1. Summary

1. `theory-referee.md` survives. All three "wrong" rulings and all three
   "overstated" rulings hold on independent recomputation. It loses no
   headline. It needs four corrections of detail and it was too lenient on
   three of five passed pages I sampled.
2. `frame-information.md` keeps its conclusion for the tests it ran, but two
   of its supporting statements fail: "the first 17 bits are already inside
   the price" is true for two of the four corner blocks only, and the 6x6
   evidence that "bits arrive only at the end" is a property of plain
   depth-first search, not of refuters in general.
3. New measurements made for this review: h(1) to h(6) by a second method;
   the distinct-piece fraction sampled exactly on 4x4, 4x6, 4x8 and 4x12
   patches; a brute-force 478 shell on fresh painted boards; the crossing
   with and without clues from the refereed model; the friendliness signal
   against a null of real valid frames; arc consistency over the whole
   interior of the official pieces.

## 2. Verdict table

Probability is that the claim is true as the memo states it.

### Part 1, `theory-referee.md`

| # | Claim | Verdict | P | Reason |
|---|---|---|---|---|
| T1 | The entropy limit is not 0.67; h(6) = 0.640 bounds it; it is about 0.435 | holds | 0.99 | My broken-profile count gives lambda 46.18, 125.74, 342.50, 932.95, 2541.30, 6922.37 and increments 0.4350, 0.4352 x4. The page's own four values already give increments 0.4349, 0.4353, 0.4353 (`01`) |
| T2 | The distinct fraction is not an area law; local plus birthday; below 1e-3 near 46 cells, not 155 | holds with a correction | 0.9 | Sampled: 0.326 at 4x4 (memo law 0.328, page 0.49), 0.108 at 4x6, 0.0247 at 4x8, 0.00038 at 4x12 (memo law 0.00059, page 0.118). The form is right; the law runs high by a factor 1.5 at 48 cells. The "birthday coefficient 1.002" is a two-point coincidence: non-adjacent pairs repeat at 1.04 to 1.26 times 1/196 |
| T3 | "At most 76" is wrong in count and direction; at least about 154 | holds | 0.97 | Bag counts reproduced: 50 + 73 + 2 = 125 near-twin pairs, 23 turning tiles giving 29 moves. Brute force on three fresh painted boards: turn neighbours equal the bag count each time (28, 14, 25); swap neighbours 160, 143, 140 against 160, 143, 137 pairs (`02`) |
| T4 | The planted boards do not cross one expected solution at N = 12 to 13; crossing at N = 16 | holds with a correction | 0.95 | Page formula reproduced (-3.3, 3.1). With seams and frame: -28.4 at 12, -5.5 at 15, +7.1 at 16 (refereed model on generated boards). The bench pins five clues: with them it is -41.4 at 12 and still -7.4 at 16, so these boards never reach the crossing at any size tested |
| T5 | 307 of 480 and 114 near-twins are artefacts of the stored orientation | holds | 0.99 | Stored cap 307; 2,000 random orientations 402.3, sd 9.3, minimum 370. All 196 interior pieces are stored with their smallest colour on top. 114 stored, 125 up to rotation |
| T6 | `sigma-cycles` is overstated: the giant loop is what unrelated arrangements give | holds | 0.85 | Largest cycle of a uniform class-preserving permutation: median 119, 5% to 95% 65 to 187. The page's corpus median is 119 |
| T7 | `design-recipe` is overstated (title and lede against the body) | holds | 0.8 | Judgment on wording, shared. Its duplicate null is also too weak in the other direction: interior pieces only gives 0.183; all 256 pieces gives under 1% (`03`) |
| T8 | `why-e2-is-hard` is overstated (two of five axes measure the file) | holds | 0.9 | Follows from T5 |
| T9 | The other 24 pages are sound, sound with a caveat, or unverifiable | holds with a correction | 0.5 | Too lenient on 3 of the 5 I sampled, see section 5 |
| T10 | Rigorous lower bound 0.067 from four toroidal 3x3 blocks | not verified | 0.9 | The argument is valid as written; I did not recount the blocks |
| T11 | Planted second moment is 1 + E[Z] + about 0.03 | not verified | 0.5 | Not recomputed; the memo itself says intermediate sizes are bounded by argument only |
| T12 | Three frame counts agree (10^37.61, 37.59, 37.71) | holds | 0.9 | 4! 56! (12!)^5 / 60! = 10^37.714 by hand; 3,432 arborescences and 3.478e41 circuits reproduced (`06`) |
| T13 | Flux ranks, ring degrees, BEST count | holds | 0.95 | Complex flux rank 22, in and out degree 12, 14 self-loops, 3,432 reproduced (`06`) |

### Part 2, `frame-information.md`

| # | Claim | Verdict | P | Reason |
|---|---|---|---|---|
| F1 | 57,131 chains per corner; 2,637 to 3,499 fillable; 17.17 bits | holds with a correction | 0.95 | Reproduced exactly by a script that does not use `rv_lib` (`03`). The memo leaves the other four clue pieces in the pool; removing them gives 2,508 to 3,404 and 17.45 bits. Fractions are over chains, not over frames (the memo says so) |
| F2 | The first 17 bits are already inside the 10^40.49 price | holds with a correction | 0.3 | True of two blocks. The row scan closes the two near blocks at cell 47 and the two far blocks at cell 239 of 256, after all but 10^-37 of its nodes. See section 4 |
| F3 | Away from the corners a test sees nothing until about 150 of 191 cells are filled | not established | 0.5 | The model column is a first-moment bound, not a measurement of what a test sees. Holds for arc consistency at the root on the official pieces (measured here). Not measured for all-different or clause learning at 16x16 |
| F4 | 6x6: "at nine tenths of the cost a sound refuter holds 1.3 to 2.0 bits" | wrong | 0.05 | With zero search nodes, arc consistency plus all-different refutes 94.4% to 100% of frames and one level of probing refutes every frame without a fill (`04`). The sentence is true only of prefixes of a plain depth-first search |
| F5 | The 2 ranking bits are real | holds | 0.85 | Not an artefact of shuffling. Against real valid frames of the same pieces: z = 1.40 (0.18); against shuffled letters on the same boards: 1.34 (0.16). Worth 1.7 bits by mean rank, 2.3 by the memo's measure (`05`) |
| F6 | The multiset of inward colours is the same for every frame, so order-free statistics carry zero bits | holds with a correction | 0.9 | Exact. "Matching relaxations of single cells carry zero bits" holds for the 48 ring cells that see one letter. The four corner cells see an ordered pair and are dead with probability 0.0945 each (exact) |
| F7 | Fast refutation of whole frames cannot pay: 10^32.4 frames survive the corner lists | holds | 0.95 | Arithmetic on the frame count; it does not cover refuting partial frames inside a search, as the memo says |
| F8 | Headline: no cheap information about the frame beyond about 20 bits | holds with a correction | 0.75 | Stands for counting, corner lists, the first-ring count and arc consistency over the whole interior. Open for all-different and clause learning on partial frames |

## 3. The most serious error

`frame-information.md`, lines 209 to 211, repeated in the answer at lines 23
to 26:

> At nine tenths of the cost a sound refuter holds 1.3 to 2.0 of 11.5 to 14.6
> bits. Half the bits are in only when 99.9 percent of the cost is spent. This
> is the 16x16 model curve of section 3 seen on real trees.

The table behind it measures one refuter, a plain depth-first fill in spiral
order, and measures cost in that search's own nodes. The memo's own output
contradicts the general reading: `frame-information/03_small_boards_bits_per_depth.out`,
seed 1, reports "no fill even with pieces reused: 0.991 (6.77 bits)" at the
root, against 0.39 bits at half the cost in the table.

My run on the same three boards (`04_refuter_dependence.out`, 1,200 uniformly
sampled valid frames each, zero unsound refutations):

| Board | Corner cells | Arc consistency | plus all-different | plus one level of probing | Exact (truth) |
|---|---|---|---|---|---|
| s1 | 1.25 bits | 6.32 | at least 10.2 | at least 10.2 | at least 10.2 |
| s2 | 0.28 | 2.04 | 4.16 | at least 10.2 | at least 10.2 |
| s3 | 0.56 | 3.56 | 7.64 | 10.2 | 10.2 |

("At least 10.2" means every one of 1,200 sampled frames was refuted.) All of
these are obtained before the first search node. So on 6x6 "when the bits
arrive" is set by the refuter, and the boards cannot be used as evidence about
16x16 in either direction. The interior trees there are 14 to 33 nodes, so
nothing is gained in time either; the point is only that the sentence is not a
fact about refuters.

The same confusion sits in section 3 of the memo (`01_model_information_curve.py`,
the column "bits a depth-d test can give = max(0, -log2 mu_d)"). mu_d is a
mean. A mean above one does not say that most frames survive; it is silent.
The memo's table handles this correctly for the corner blocks (17.2 bits stay
once earned) but then reads the silence between cells 12 and 150 as "the frame
is invisible".

What rescues the conclusion on the official pieces is a measurement the memo
did not make. Arc consistency over all 196 interior cells, 1,500 shuffled
border words, pieces reusable, no clues (`05_friendliness_null.out`): dead at a
corner cell 0.313 (0.54 bits), arc consistency on the first ring only 0.4293
(0.81 bits), arc consistency on the whole interior 0.4313 (0.81 bits). Going
deeper than the first ring adds three words in 1,500. Belief propagation used
as a refuter can do no more than arc consistency, since its hard part is the
same support test. So for this family of refuters the memo's conclusion holds
on the real pieces, and it holds because of the piece set (46 candidates per
seam letter, 17 colours), not because of the first-moment curve.

What stays open: all-different propagation and clause learning at 16x16. At
the root all-different can do little (the smallest domains are the corner
cells, about 2.7 tiles). Inside a search, a learned clause can remove a set of
partial frames at once, and the count argument of the memo's section 2 does
not apply to that. I see no cheap experiment that settles it; the 6x6 boards
do not.

## 4. Part 2 in detail

**(a) Corner counts.** Reproduced exactly: 57,131 chains at each corner;
2,716, 3,499, 2,875, 2,637 fillable at (0,0), (0,15), (15,0), (15,15); 22,303
inward 4-tuples seen; 17.17 bits. One defect in the memo's script
(`02_real_set_corner_blocks.py`, the `preset` passed to `en.run`): only the
block's own clue piece is marked used, so the other four clue pieces can fill
block cells. With all five clue pieces removed the counts are 2,620, 3,404,
2,669, 2,508 and the sum is 17.45 bits. The difference is 0.28 bits and
changes nothing downstream.

**(b) "Already inside the price".** Refereed model, row scan from row 15
(`03_corner_chains_and_row_scan.out`):

| Clues on the board | Row-scan sum | Against all five |
|---|---|---|
| all five | 10^40.49 | |
| centre and the two at row 13 (met first) | 10^41.35 | +0.86 digits |
| centre and the two at row 2 (met last) | 10^46.27 | +5.77 |
| centre only | 10^47.14 | +6.64 |
| none | 10^50.03 | +9.53 |

The three anchor values match tonight's measured tree sizes (10^40.457,
10^47.140, 10^50.039). The two clues the scan meets first are worth 5.77
digits (19 bits) to it. The two it meets last are worth 0.86 digits, and that
comes from two fewer pieces in the pool, not from their joints: their blocks
are closed at scan cell 239, when the nodes still to come are 10^3.3 out of
10^40.49. So 8.4 of the 17.17 bits (the blocks at (0,0) and (0,15)) are not
"inside the price". They are information about pieces the row scan places
after its cost is spent.

They also cannot be put inside. Closing the two far blocks first and then
running the same scan costs 10^41.13, which is 0.64 digits worse (`04`, part
C); the 10^6.7 live chain-and-block states at the top are carried through the
whole tree and repaid only at the end. So the memo's practical conclusion
(the corner lists give the row scan nothing more) holds, for a different
reason than the one it gives.

**(c) Refuter dependence.** See section 3.

**(d) The friendliness signal.** 24 planted 16x16 boards, 120 null words of
each kind per board, same statistic (first-ring count, pieces reusable, no
clues), my own transfer-matrix code (`05`):

| Statistic | Null | Null words alive | z of the true word | Mean rank among alive |
|---|---|---|---|---|
| first ring | shuffled letters | 0.650 | 1.34 (0.16) | 0.158 |
| first ring | real valid frames | 0.651 | 1.40 (0.18) | 0.157 |
| four corner cells only | shuffled letters | 0.757 | 0.78 (0.17) | 0.292 |
| four corner cells only | real valid frames | 0.773 | 0.75 (0.16) | 0.305 |

The two nulls agree in every column, so the signal is not an artefact of
shuffling letters. Three limits. The real frames come from a randomised
depth-first search, so they are valid but not uniform. The generated boards
have independent lateral and inward colours by construction; the official set
may not, and that cannot be tested without the true frame. The reusable-piece
part of the statistic was not tested against distinct pieces. On size: a mean
rank of 0.157 against 0.5 is a factor 3.2 on the expected position, 1.7 bits;
the memo's log measure gives 2.3 on my boards and 2.2 to 3.0 on its own, and
that measure depends on how many null words are drawn. "About 2 bits" is fair.
About half of the z is already in the candidate counts of the four corner
cells.

**(e) The zero-bit argument.** Correct. Every valid frame uses all 56 edge
pieces and each shows its one fixed colour inward, so the multiset is the same
for every frame. The one place the wording goes too far is "matching and flow
relaxations of single cells": the four corner cells of the interior each see
an ordered pair of letters, and which eight letters fall there does depend on
the frame. That is the 0.57 bits of the corner-cell test (exact per-corner
dead probability 0.0945 under a shuffled word, `06`), already counted by the
memo elsewhere.

## 5. Part 1 in detail

### The six flagged pages: who is right

In all six the memo is right and the site page should change.

**`entropy-area-law`.** Two independent computations now agree on h(1) to
h(6) = 1.6645, 1.0497, 0.8449, 0.7425, 0.6810, 0.6400, and my exact integer
counts match the committed A(2), A(3), A(4) digit for digit. Since the limit is
an infimum, 0.640 is an upper bound and 0.67 is impossible. The limit is close
to 0.435: each added row multiplies the count by 10^0.4352, constant to four
digits from width 2. For the distinct fraction I sampled uniform colour-valid
patches exactly (completion counts, then forward sampling; the sampler gives
0.64765 at 3x3 with a million samples against the exact 0.647680):

| Patch | Cells | Sampled rho | Memo law | Page fit |
|---|---|---|---|---|
| 4x4 | 16 | 0.3258 (0.0006) | 0.328 | 0.491 |
| 4x6 | 24 | 0.1077 (0.0007) | 0.111 | 0.344 |
| 4x8 | 32 | 0.0247 (0.0004) | 0.0269 | 0.241 |
| 4x12 | 48 | 0.00038 (0.00004) | 0.00059 | 0.118 |

The page's fit is wrong by a factor 300 at 48 cells. The memo's law has the
right form and runs slightly high, because pairs of cells that are close but
not adjacent repeat a piece more often than 1/196 (measured 0.0064 per pair at
2x2 falling to 0.0053 at 4x12). Adjacent pairs repeat at 0.026. The fraction
passes one in a thousand at about 45 cells.

**`parity-defect-floor`.** The 479 theorem is sound. The shell count is wrong
as the memo says. A near-twin pair gives a 478 board from any solution, by
turning each piece into the other's orientation; nothing depends on the
solution except the rare case where the two sit next to each other across
their differing side. The page also omits the border pieces, which hold 75 of
the 125 pairs.

**`how-hard-is-this-instance`.** The page's formula drops the 4(N-2) seam
joints and the frame. Owen's own criterion on `phase-transition` counts two
joints per piece; with that, the same arithmetic gives -27.9 at 12x12, not
-3.3. With the five pinned clues of the bench the expected number of extra
solutions is 10^-44 to 10^-29 for N = 10 to 14. These boards are far on the
over-constrained side at every size, and "plausibly admit very many solutions"
is false.

**`sigma-cycles`, `design-recipe`, `why-e2-is-hard`.** Rulings confirmed, see
T5 to T8.

### Corrections to the memo itself

1. Summary item 5 and section 6 ("the birthday coefficient recovered as 1.002"):
   drop it. The coefficient is 1.04 to 1.26 depending on distance. Quote the
   sampled values instead of the law beyond 4x4.
2. Summary item 7 ("the crossing is at N = 16"): true without clues. The bench
   has five clues and does not cross at 16 either (10^-7.4).
3. Section 2, `design-recipe`, item 3: the memo's rerun gives 1.65% for no
   duplicate among 256; with uniform colours I get 0.45% (4.65 repeated pieces
   expected, 3.68 of them on the border). The committed 1.4% uses the official
   colour counts. All three say the same thing; the page's "one time in five"
   is the interior alone (0.183 here).
4. Leniency, next.

### Five passed pages, sampled for leniency

| Page | Memo | Mine | Why |
|---|---|---|---|
| `rigidity-wall` | unverifiable | overstated | The front matter says `rigor: proven` while the description says most MIP runs are not committed. Line 618 "The page so far proves the wall", line 554 "at most about 10^7 solutions" (a first moment, for the puzzle without clues), lines 205 to 206 "a board-wide bound of <= 476". The memo lists all three and still files the page as unverifiable |
| `irreducible-hard-region` | sound with a caveat | overstated | Section "The stall is a wall, not a slow start". In the refereed model a row scan with 1.5 million nodes reaches depth 203.0 and with 4 million reaches 204.2: 1.2 cells, 0.07 of a row (`07`). The page saw moves inside one row. Its data are what an ordinary tree gives, so "the search is not slow; it is stopped" has no support |
| `complex-theory` | sound with a caveat | sound, with one wrong paragraph | "Plateau (depth 50 to 200) ... about 10^45": the width is 10^27.6 at depth 50, 10^41.7 at 100, 10^48.5 at 150, 10^43.6 at 200 without clues, a range of 21 decades. The model itself is in good standing: its total with the centre clue, 10^47.135, sits on tonight's measured 10^47.140 |
| `flux-invariants` | sound | sound | Rank 22 reproduced |
| `ring-purity` | sound | sound | Degrees, self-loops, 3,432 and 3.478e41 reproduced |

Three of five are filed one grade too kindly. Expect the same rate among the
other 19: the count "3 overstated" is a floor.

## 6. Sentence-level corrections for the site

### Corrections I would stand behind

**`web/content/research/why/entropy-area-law.mdx`**

- Line 167, caption. Now: "Decreasing toward the limit ≈ 0.67. The first two
  points are computed exactly here; the rest from an offline sweep."
  Proposed: "Decreasing toward a limit near 0.44. Each added row multiplies
  the count by about $10^{0.435}$, so $h(n) \approx 0.435 + 1.23/n$; the
  widths shown are still far from the limit."
- Lines 255 to 257. Now: "Positivity holds because the grammar supports
  exponentially many chains, so the density is strictly between zero and
  1.6645, measured near 0.67."
  Proposed: "Because the limit is an infimum, every computed width is an upper
  bound: $h(6) = 0.640$ gives $h_\infty \le 0.640$. The increments
  $\log_{10}(\lambda_{n+1}/\lambda_n)$ sit at 0.4352 from width 2 to width 6,
  so the limit is close to 0.435, next to the mean-field value
  $\log_{10}(784/289) = 0.433$."
- Lines 259 to 261. Now: "Its specific value near 0.67 is carried from an
  offline sweep."
  Proposed: "Its value, about 0.435, is read from the strip eigenvalues for
  widths 1 to 6."
- Lines 177 to 182. Now: "It collapses, and it collapses in the area, not the
  perimeter: $\rho(n) \approx \exp(-\alpha n^2)$, $\alpha \approx 0.044$."
  Proposed: "It collapses, and faster than an area law. Two effects add.
  Neighbouring cells hold the same piece about 2.6% of the time, five times
  the chance level of 1 in 196, and every other pair of cells repeats at about
  the chance level. For a patch of $m$ cells with $a$ adjacent pairs,
  $-\ln\rho \approx 0.021\,a + \binom{m}{2}/196$."
- Lines 193 to 195. Now: "Extrapolating, the fraction of realizable patches
  drops below one in a thousand near 155 cells."
  Proposed: "Sampling colour-valid patches directly gives 0.33 at 16 cells,
  0.025 at 32 and 0.0004 at 48: the fraction of realizable patches drops below
  one in a thousand near 45 cells."
- The front-matter `scope`, the data file's `hInfinity`, `areaLawExponent` and
  `distinctnessCollapseCells`, and `walls-and-methods.mdx:79`, `index.mdx:95`
  carry the same numbers.

**`web/content/research/why/parity-defect-floor.mdx`**

- Lines 6 to 7 (and the `metaDescription`). Now: "The floor below perfect is
  478, and at most 76 one-move near misses can surround any solution."
  Proposed: "The floor below perfect is 478, and every solution has about 150
  one-move neighbours sitting exactly there."
- Lines 127 to 133. Now: "Adding them up, any solution has at most
  $50 + 23 + 3 = 76$ defect-2 neighbours reachable by a single swap or
  rotation. This is an upper bound from the bag: whether a given twin pair
  actually lines up inside a specific solution depends on that solution, so
  the realized count for the designed solution is open. The ceiling stands
  regardless. Around the very top of the ladder, near misses are sparse, a few
  dozen boards at most, while rungs further down are populated astronomically."
  Proposed: "Both kinds work in every solution. To swap a twin pair, turn each
  piece into the other's orientation: one joint breaks at each cell, wherever
  the two pieces sit. Counting the border as well, the set holds 125 such
  pairs (50 interior, 73 edge, 2 corner), and the 23 turning tiles allow 29
  distinct turns. So every solution has about 154 single-move neighbours at
  478, a floor from the bag and not a ceiling (a pair that happens to sit
  side by side across its differing edge is the one exception). Next to the
  astronomical populations further down, the shell is still small."
- Lines 118 to 121: "Two interior tiles" should read "Two tiles of the same
  kind", and "exactly **50** such near-twin pairs" should read "exactly
  **125** such near-twin pairs, 50 of them interior".
- Line 169. Now: "and caps the one-move shell around every solution at 76."
  Proposed: "and puts about 150 one-move boards at 478 around every solution."
- `theorem-sweep.mdx:121` repeats the bound.

**`web/content/research/why/how-hard-is-this-instance.mdx`**

- Lines 10 to 11 (description). Now: "and both walls move with the colour
  count, close to where the boards cross the expected-one-solution size."
  Proposed: "and both walls move with the colour count."
- Lines 227 to 229. Now: "Planted boards are not canonical Eternity II; they
  plausibly admit very many solutions, where the real puzzle's first-moment
  estimate is about 14,702 with the centre clue and about one with all five."
  Proposed: "Planted boards are not canonical Eternity II. At these sizes they
  are far more constrained than the real puzzle: the same first-moment
  estimate that gives about 14,702 for Eternity II with the centre clue gives
  a 12×12 board with 17 interior colours and five clues about $10^{-41}$
  solutions beyond the planted one."
- Lines 283 to 291. Now: "Where the walls sit lines up with where these boards
  cross the expected-one-solution size for their colour count. A crude
  first-moment estimate for the interior alone, [...] At 17 interior colours
  that is where CP-SAT slows and starts timing out. Both solvers hit their
  wall near this crossing, and CP-SAT pushes it out by a size or two compared
  with the restarting DFS."
  Proposed: "The walls do not sit at the expected-one-solution size. Counting
  every joint, frame and seams included, a board with 17 interior colours and
  no clues expects $10^{-28}$ extra solutions at 12×12, $10^{-6}$ at 15×15 and
  $10^{7}$ at 16×16; with five clues it stays below one at every size here.
  Both solvers fail on boards that almost surely have a single solution, and
  CP-SAT pushes the wall out by a size or two compared with the restarting
  DFS."

**`web/content/research/why/why-e2-is-hard.mdx`**

- Lines 117 to 119. Now: "A pipeline that fixes orientations early therefore
  concedes at least 173 joins before search begins, and 307 sits far below
  every high board on the [records page](/research/records)."
  Proposed: "The number belongs to the published orientation, which stores
  every interior piece with its smallest colour on top. Random fixed
  orientations cap near 402 (2,000 draws, standard deviation 9), and the
  orientations of a solution cap at 480. So 307 says the file's orientation is
  far from a solution's; it does not price a pipeline that fixes orientations
  some other way."
- Lines 100 to 102. Now: "Loosening the question to near-twins, pairs sharing
  3 of their 4 same-position edges in the stored orientation, adds 114 pairs
  in 79 groups."
  Proposed: "Loosening the question to near-twins, pairs that agree on three
  of their four sides after some rotation, adds 125 pairs: 50 interior, 73
  edge, 2 corner. (Counted in the stored orientation only, the figure is
  114.)"
- Line 8, line 69 and line 180 carry the 307 and the 173 and need the same
  qualifier. `sigma-cycles.mdx:292` and `rigidity-wall.mdx:122` carry the 114.

**`web/content/research/why/design-recipe.mdx`**

- Lines 106 to 109. Now: "so an unfiltered generator produces no symmetric and
  no duplicate interior piece roughly one time in five. The absence fits a
  filter but does not prove one; the flat frequency distribution below is the
  ingredient that is clearly non-random."
  Proposed: "so an unfiltered generator produces no symmetric and no duplicate
  interior piece roughly one time in five. The border changes the picture:
  there are only 425 edge-piece designs for 56 pieces, a painted set expects
  three to four repeated border pieces, and no repeat anywhere among the 256
  happens in about one painted set in a hundred
  ([the piece-set page](/research/why/why-e2-is-hard) has the null). The
  absence of duplicates is good evidence of a filter; the absence of symmetric
  pieces is not."
- Lines 42 to 45. Now: "It is the output of a recipe: a short list of design
  rules that, followed together, produce the hardest edge-matching puzzle a
  given number of pieces can make, while still guaranteeing that a solution
  exists."
  Proposed: "It is the output of a recipe: a short list of design rules that,
  in Owen's simplified model, push a puzzle of this size toward its hardest
  setting while still guaranteeing that a solution exists. Nothing about
  hardness is proven for this one instance."

**`web/content/research/why/sigma-cycles.mdx`**

- Line 46. Now: "Here's the catch. Between the best boards that loop is
  enormous."
  Proposed: "Here's the catch. Between the best boards that loop is enormous,
  and exactly as large as it would be between two unrelated arrangements: a
  random reshuffle of the pieces has a largest loop of median 119 cells, and
  the record pairs have a median of 119."
- Lines 363 to 366. Now: "You can't climb out of a good board locally, and you
  can't hop to a neighbour either, because the nearest better board is one
  indivisible move of many cells away, with no improving step to lead you into
  it."
  Proposed: "You can't climb out of a good board locally, and walking the loop
  from one known strong board to another never passes a board as good as its
  start. That rules out interpolating between known boards. It does not test
  perturb-and-reoptimise moves, which are a different operation."

**Also, from the leniency sample**

- `irreducible-hard-region.mdx`, lines 157 to 159. Now: "The search is not
  slow; it is stopped, and it is stopped at the last region it tries to fill.
  That is the budget-insensitive failure the conjecture predicts, reproduced
  directly."
  Proposed: "This test cannot tell a wall from an ordinary exponential tree.
  In the first-moment model a 2.7 times larger budget moves the frontier by
  about one cell, well inside the one row of jitter observed. A test with
  power needs budgets spread over several decades."
  The heading at line 144 and the description at lines 6 to 7 ("more budget
  does not move it") need the same change.
- `complex-theory.mdx`, lines 145 to 146 and 151. Now: "**Plateau (depth
  50–200).** The tree is at its widest, about $10^{45}$ ways to extend" and
  "The width falls from $10^{45}$ back to about $10^{4}$."
  Proposed: "**Bulk (depth 50–200).** The width climbs from about $10^{27}$ to
  a peak near $10^{46}$ at depth 161 and is back to about $10^{41}$ by depth
  200; it is within one decade of the peak only for depths about 140 to 178"
  and "The width falls from about $10^{41}$ back to about $10^{4}$." (Figures
  from the memo's rerun of the committed reference table; mine, without clues,
  have the same shape.)
- `rigidity-wall.mdx`, line 12: `rigor: proven` should be `rigor: measured`
  until the MIP runs are committed. Line 618, "The page so far proves the
  wall", proposed: "The page so far documents the wall on the boards tested".
  Line 554 and lines 608 to 609, "at most about $10^7$ solutions", proposed:
  "an expected $10^7$ solutions without clues (a first-moment estimate, not a
  bound)".

### Corrections I would not stand behind yet

- Stating the rigorous lower bound "0.067" on the entropy page. The argument
  is sound; I did not recount the four toroidal blocks.
- Stating the distinctness law as exact, or the "birthday coefficient 1.002".
  Use the sampled values.
- The memo's "22 further interior pairs" that reach 478 by an adjacent swap
  (`parity-defect-floor`, item d), and so any exact shell size. "About 150"
  and "at least" are safe; an exact count or an upper bound is not.
- `sigma-cycles`: that the "single parity relation" is a sampling accident.
  Plausible (expected 0.22 accidental relations), not checked.
- `sigma-cycles` title. "Looks impossible" is hedged already; I would change
  the two sentences above and leave the title.
- The memo's second-moment figure (1 + E[Z] + 0.03) and the Clue 3 calibration
  (U4, U7). Not recomputed.
- Any exact share for "no duplicate among 256 pieces": 0.45% with uniform
  colours, 1.4% to 1.65% with the official counts. "About one in a hundred" is
  safe.
- The memo's "possibly new" list (section 6). No literature check was made
  here.
- The 20 other passed pages. Not read.

## 7. What the program may rely on tomorrow

1. h(1) to h(6) = 1.6645, 1.0497, 0.8449, 0.7425, 0.6810, 0.6400; the entropy
   limit is at most 0.640 and close to 0.435. Two independent codes.
2. The distinct-piece fraction of colour-valid patches: 0.326 at 16 cells,
   0.108 at 24, 0.025 at 32, 0.0004 at 48. A piece-agnostic relaxation over
   more than about 30 cells is counting mostly unrealisable patches.
3. Bag facts: 125 near-twin pairs up to rotation (50, 73, 2); 29 two-side-
   keeping turns on 23 tiles; every solution has about 154 single-move
   neighbours at 478; the stored orientation is canonical (smallest colour on
   top), so any statistic taken "in the stored orientation" is about the file.
4. Planted boards with 17 interior colours and five clues are below one
   expected extra solution at every size up to 16. Benchmarks on them measure
   the over-constrained regime, not the regime of the real puzzle without
   clues.
5. Corner blocks on the official pieces: 57,131 chains per corner, 2,508 to
   3,404 fillable with the five clue pieces out of the pool, 17.45 bits
   (17.17 by the memo's convention).
6. In the row-scan model the two corner clues met first are worth 5.77 digits
   and the two met last 0.86. Closing the far blocks first costs 0.64 digits.
   The 10^40.49 price is not improved by the corner lists.
7. Arc consistency over the whole interior of the official pieces sees
   nothing beyond the corner windows of the first ring (0.81 bits, no clues).
   Belief propagation as a refuter is covered by this.
8. The friendliness ranking is real on generated boards, about 2 bits, and
   half of it is the candidate count of the four corner cells.
9. Do not rely on: the 6x6 boards as evidence about when information arrives
   at 16x16; the first-moment curve as a statement about what a test sees;
   "no cheap frame information" for all-different or clause learning on
   partial frames, which nobody has measured.
10. Do not rely on the memo's count of sound pages. Three of five sampled were
    one grade too kind, and two more site sections (`irreducible-hard-region`
    budget test, `rigidity-wall` rigor badge) should be treated as overstated.
