# Frame information: is there a cheap source of bits about the frame?

Night 2026-11-12. Scripts and outputs:
`research/program/night/2026-11-12/memos/frame-information/` (`01` to `07`,
each `.py` with its `.out`). All stdlib Python, one core. Compute spent: about
25 core-minutes, of which about 9 were runs I stopped (see section 9) and 2
were a rerun of `03`. Script `05` reads the JSON that `03` writes, so run `03`
first. The scripts import the referee's library
`../price-and-ledger-review/rv_lib.py` read only. No existing file was edited.
No solver was run on the official 16x16.

Grades: **measured** (by a script here, command given), **reported** (by a file
I opened), **argued** (reasoning, no run).

## Answer

1. No. The frame holds about 20 cheap bits. A solver needs 91 more than the
   first 17 of them to reach 1,000 core-years, and the first 17 are already
   inside the 10^40.49 price.
2. The cheap bits are all at the four corners. The four clue corner blocks give
   17.17 bits on the official pieces for about 10 nodes each (measured,
   exact). The rest of the first interior ring adds about 1 bit.
3. Away from the corners the frame is invisible to any test that fills fewer
   than about 150 of the 191 free interior cells. By then the whole per-frame
   cost (10^8.34 nodes in the model) is spent. Information and cost arrive
   together.
4. One small signal is new: the true border word admits more first-ring fills
   than a shuffled word that passes the same corner tests. It sits 1.6 standard
   deviations high (30 planted 16x16 boards). That ranks it for about 2 bits.
5. Fast refutation cannot pay at any speed: one node per frame is already
   10^37.6 nodes, and 10^32.4 frames survive the corner lists.

## 1. Terms

- **Frame**: the 60 border pieces in place. The official set has 10^37.608
  valid frames (reported: committed count quoted in the review, section B).
- **Border word**: the 56 colours that the edge pieces show to the interior,
  read around the ring. The interior sees the frame only through this word.
- **Inward letter**: one colour of the border word.
- **mu_d**: the mean, over valid frames, of the number of valid partial
  interiors after d interior cells, for a fixed interior order. It is a
  first-moment quantity of the review's model.
- **Sure bits** of a test: minus log2 of the fraction of frames that pass. The
  true frame always passes a sound test, so these bits cannot be wrong.
- **Ranking bits**: what is gained by trying frames in order of a score. They
  shorten the expected time to the first solution. They do nothing for a
  search that must exhaust.
- **Clue corner block**: the 2x2 interior block in a board corner. Its far cell
  is a clue on the official board (clues at (2,2), (2,13), (13,2), (13,13)).
  Three cells are free and eight joints are closed.
- **Reusable pieces**: a relaxation in which one piece may fill several cells.
  A count with reusable pieces is an upper bound on the true count, so a zero
  is a sound refutation.

## 2. The accounting that decides the question (argued)

The cost of frame-first search is the sum, over all frames, of the interior
tree under each. A sound filter removes frames. It lowers the sum only by the
trees of the frames it removes. So a filter is worth something only if it
kills frames whose trees are large, for less than those trees cost.

The clue corner blocks kill 99.9993 percent of frames. Each of those frames
had a tree of about 10 nodes, because any sensible interior order reaches the
clue joints at once. The sum is unchanged: 10^32.4 surviving frames times
10^13.5 nodes each is the same 10^45.9 (model, `01`, section A: both orders
give sum 10^8.34 per frame).

So "bits about the frame" must be counted beyond what the search already
closes. The review's 91 bits are 91 bits beyond the clue blocks. I checked
this: with the best per-frame cost of 10^8.34, the true frame must rank within
10^10.2 of 10^37.6 (measured, `01`, section E: 91.2 bits; row-scan break-even
18.1 bits).

A second consequence, for part (c) of the question. A refuter that costs one
node per frame costs 10^37.6 nodes. That is 19 digits above 1,000 core-years
(10^18.5 nodes). After the corner lists, 10^32.4 frames remain, 13.9 digits
above. No refuter speed rescues an enumeration of frames. Only a method that
never lists frames could, and such a method can only use scores that split
into local pieces of the border. Section 4 measures what local pieces hold.

## 3. Where the information is, in the model (measured on the model)

Command: `python3 -I 01_model_information_curve.py` (91 s, most of it one
class-aware table).

Interior order: the four clue corner blocks first, then the inward spiral.
Five clues, pooled census.

| Interior cells filled | log10 mu | Nodes per frame so far | First-moment sure bits |
|---|---|---|---|
| 3 (one block) | -1.27 | 10^0.96 | 4.2 |
| 12 (four blocks) | -5.18 | 10^0.98 | 17.2 |
| 40 | 2.00 | 10^2.32 | 17.2 |
| 91 (peak) | 7.36 | 10^8.08 | 17.2 |
| 120 | 4.24 | 10^8.34 | 17.2 |
| 140 | 0.14 | 10^8.34 | 17.2 |
| 150 | -4.35 | 10^8.34 | 17.2 |
| 160 | -8.66 | 10^8.34 | 28.8 |
| 191 | -45.10 | 10^8.34 | 149.8 |

Reading: after 12 cells and about 10 nodes, 17.2 bits. Then mu climbs back
above one and stays there until cell 141. The 18th bit arrives near cell 152.
All of the 10^8.34 nodes are spent before it.

Why mu must climb (argued, with the model's numbers). A new interior cell
brings 4 x (pieces left) choices and closes about two joints, each matching
with probability about 1/17.3. The ratio 4 x 191 / 299 is 2.6. It stays above
one until fewer than 75 pieces are left. Only running out of pieces brings mu
down. A corner cell is the exception (two frame joints on one cell), and a
clue is a stronger one (it closes joints for free).

No region does better (measured, `01`, section C). Corner blocks of side k
with five clues: log10 mu is 0.41, -1.27, 0.71, 3.37, 6.63 for k = 1 to 5, and
stays positive up to k = 13 (7.04). After each full spiral ring: 3.02, 6.25,
2.69, -7.28. Along the spiral, mu falls below one for good at cell 141 of
191. I found no region outside the clue blocks with mu below one before that.

The other side of the join is no help (measured, `01`, section D). Valid 14x14
interiors with five clues and no frame: 10^19.23, far fewer than frames. But
listing them costs 10^50.88 nodes by rows. This agrees with path 3: every list
is a region count.

## 4. What the official pieces say, corner by corner (measured, exact)

Command: `python3 -I 02_real_set_corner_blocks.py` (18 s).

For each board corner I enumerated every valid chain of five border pieces
(the corner piece and two edge pieces on each side): 57,131 chains per corner.
For each chain I counted the fills of the clue block with distinct pieces.

| Corner | Chains with a fillable clue block | Fraction | Sure bits | Mean nodes of the test | Distinct fillable 4-tuples of inward letters |
|---|---|---|---|---|---|
| (0,0) | 2,716 | 0.0475 | 4.39 | 9.2 | 1,145 of 22,303 seen |
| (0,15) | 3,499 | 0.0612 | 4.03 | 10.4 | 1,257 |
| (15,0) | 2,875 | 0.0503 | 4.31 | 9.3 | 1,090 |
| (15,15) | 2,637 | 0.0462 | 4.44 | 9.3 | 1,044 |

Sum: **17.17 bits** (model: 17.2). The fractions are over 5-piece chains, not
over full frames; I did not weight chains by the number of frames that extend
them.

Two more numbers from the same script.

- The plain corner cell (clue ignored) is dead in 11.4 percent of chains per
  corner: 0.17 bits per corner, 0.70 in all. The review computed 9.4 percent
  under independence and quotes the site at 9.2 to 11 percent (reported).
- Extending to the 3x3 block (seven border pieces, eight free cells, sampled,
  1,500 per corner): 79 to 85 percent of the fillable chains stay fillable.
  That is 0.23 to 0.34 extra bits per corner, about 1.1 in all, for about 130
  fills per chain. The information is saturating while the cost grows.

These four lists answer part (d). A frame never has to be enumerated to apply
them: each list has about 3,000 entries and touches five border pieces. They
are also what any search that closes clue joints early already uses.

## 5. The first interior ring by transfer matrix (measured)

Command: `python3 -I 04_ring1_transfer_matrix.py` (6 s).

For a border word, the number Z1 of fills of the 52 cells next to the frame,
pieces reusable, is an exact trace of a product of 52 matrices of size 17. I
drew border words as random orders of the official 56 inward letters. This is
not the uniform law over valid frames: lateral colours are ignored.

Clue joints dropped, 4,000 words:

| Test | Words that pass | Sure bits |
|---|---|---|
| Four corner cells | 0.6480 | 0.63 |
| Four corner windows of 3 ring cells | 0.5347 | 0.90 |
| Windows of 5 | 0.5108 | 0.97 |
| Windows of 9 | 0.5048 | 0.99 |
| Windows of 13 | 0.5045 | 0.99 |
| Whole ring, 52 cells | 0.5045 | 0.99 |

The whole ring refutes exactly the words that the four corner windows of 13
cells refute. The 26 cells in the middle of the sides refute nothing. With the
clue joints on, 0 of 400 words pass, as 17 bits predict.

This answers part (a) in two statements.

- The multiset of inward letters is the same for every frame. So is every
  statistic that does not depend on their order. Colour counts, parity
  counts, matching and flow relaxations of single cells carry zero bits
  (argued, exact). I found no counting constraint that sees the order.
- The order is visible only through local windows, and only corner windows
  are short of freedom (measured above).

Reusable pieces cannot refute more in the mean (argued, arithmetic in
`05_law_check.out`): 191 cells and 420 joints give 10^30.3 fills per frame.

## 6. Small boards: when the bits arrive (measured)

Command: `python3 -I -u 03_small_boards_bits_per_depth.py` (109 s), then
`python3 -I 05_law_check.py`.

Five planted 6x6 boards (4 frame colours, 6 interior colours; seeds 1 to 3 are
the review's boards). Every valid frame enumerated (46,208 to 199,680), the
interior searched under each in spiral order, W_d recorded at every depth.

| Board | Total bits | Sure bits at 50% of cost | at 90% of cost | Cost share spent when half the bits are in |
|---|---|---|---|---|
| s1 | 13.7 | 0.39 | 1.57 | 1.000 |
| s2 | 14.1 | 0.06 | 1.25 | 1.000 |
| s3 | 11.5 | 0.28 | 1.30 | 1.000 |
| s4 | 12.8 | 0.36 | 1.98 | 0.999 |
| s5 | 14.6 | 0.16 | 1.40 | 1.000 |

At nine tenths of the cost a sound refuter holds 1.3 to 2.0 of 11.5 to 14.6
bits. Half the bits are in only when 99.9 percent of the cost is spent. This
is the 16x16 model curve of section 3 seen on real trees.

Ranking by W_d does little better. Over 51 (board, depth) pairs with mu_d at
least 0.05, the planted frame's W_d is 12.8 times the mean (3.68 bits); 9.4 of
that factor is the sound filter (3.23 bits); ranking among survivors adds 0.45
bits. The size-bias prediction E[W^2]/E[W]^2 is 17.7 (4.15 bits), the right
size.

Position marginals (the probability of each border piece at each position,
frames weighted by W_d) carry nothing early: at depth 6 the planted frame
scores -1.5, 4.9, -2.5, -0.6, 1.1 bits on the five boards, mean 0.3. They
reach 21 to 28 bits only at depth 16, when the solution is found. This is the
answer to part (b) for marginals over partial boards.

Caution. These boards are more constrained than the official set: with pieces
reusable, 83 to 99.6 percent of their frames already have no fill. So the 6x6
figures overstate what cheap tests see. Larger boards were too dear in Python
(section 7).

Marginals over high-scoring boards (argued, arithmetic in `04`). With five
clues the model expects about 10^24.9 boards with 10 broken joints that have
nothing to do with the solution, against about 1.6 x 10^5 boards obtained from
the solution by turning one or two pieces. The solution's relatives are
10^-19.7 of the population at 470, and 10^-4.0 at 476. A sampler of
high-scoring boards does not see the solution's frame.

## 7. The one signal that is new: friendly border words (measured)

Commands: `python3 -I -u 06_friendliness_planted_16x16.py` (40 s),
`python3 -I -u 07_friendliness_beyond_clue_blocks.py` (7 s).

Idea. To an observer who only counts first-ring fills, the true border word
is like a word drawn in proportion to its count. Words differ in their count
by a wide margin (standard deviation of ln Z1: 2.1 to 2.4), so the true word
should sit high. This is ranking by expected completions, which the review
kept alive as the one thing the zero-information theorem does not remove.

Test on generated 16x16 planted boards (5 frame colours, 17 interior colours),
where the true word is known.

| Statistic | Null words | z of the true word, mean (s.e.) | Ranking worth |
|---|---|---|---|
| Whole first ring, no clues (40 boards, 200 null words each) | shuffled letters | 1.55 (0.12) | about 3.0 bits, plus 0.63 sure |
| Four side strips, depth 1, corners left out | shuffled letters | 0.58 (0.17) | under 1 bit |
| Four side strips, depth 2 | shuffled letters | 0.47 (0.18) | under 1 bit |
| Whole first ring, four corner clues on (30 boards, 60 null words each) | shuffled, every word passing the four clue blocks | 1.62 (0.15) | about 2.2 bits, plus 1.0 sure |

"Ranking worth" is the mean of minus log2 of the true word's rank fraction,
less the 1.44 bits that a signal-free rank gives.

What this says.

- The signal is real: 10 standard errors from zero.
- It is small: about 2 bits after the clue blocks, and one more sure bit (49
  percent of null words that pass the blocks have no ring fill).
- It is a corner effect again. The side strips alone give z near 0.5, and a
  second row of cells adds nothing (0.47 against 0.58). The program's map
  already records that belief-propagation ordering "fades two cells out"
  (reported, `research/program/EXPLORED.md`, line 61).
- The size-bias prediction is z equal to the standard deviation (2.2); the
  measured 1.55 is lower. Reusable pieces and non-lognormal tails are the
  likely reasons (argued).
- On generated boards a clue block passes with probability 0.057 (4.14 bits,
  shuffled letters, reusable pieces), against 0.046 to 0.061 on the official
  pieces (real chains, distinct pieces). The generated boards behave
  like the official set here.

Total cheap information about the frame on a board of the official shape:
17.2 sure (clue blocks) + about 1 sure (ring near the corners) + about 2
ranking = **about 20 bits**. The first 17 to 18 are in the price. The rest is
worth a factor of about 4 on the expected time to a first solution, if a
search can be ordered by it, and nothing on an exhaustive search.

## 8. Answers to the five parts

| Part | Answer | Grade |
|---|---|---|
| (a) What the interior implies about the border colours | Nothing about the multiset (it is fixed). About the order: 0.99 bits from the first ring without clues, all inside corner windows; 17.17 bits from the clue blocks | measured (`02`, `04`), plus one exact argument |
| (b) Solution marginals | Over partial boards: no bits before the end (6x6, all frames). Over high-scoring boards: relatives of the solution are 10^-19.7 of the population at 470. Ranking by first-ring count: about 2 bits | measured (`03`, `06`, `07`), argued (`04` arithmetic) |
| (c) Fast refutation | Dead on count alone: 10^37.6 frames, or 10^32.4 after the corner lists, at one node each. The mean is carried by the 10^-5.2 of frames that pass the clue blocks, 10^13.5 nodes each; nothing cheap tells those apart | measured on the model (`01`), argued |
| (d) Partial frames scored apart | Yes for the corners: four lists of about 3,000 five-piece chains. They hold all the sure bits there are, and row scan already uses them | measured (`02`) |
| (e) Anything better | The friendliness ranking of section 7. Small | measured (`06`, `07`) |

## 9. What I could not check

- **A uniform sample of valid frames of the official set.** My null words are
  shuffled letters. If inward and lateral colours of the official edge pieces
  are related, the corner fractions over real frames differ from mine. The
  chain counts of section 4 do use real lateral colours.
- **The true word of the official puzzle.** The friendliness signal is
  measured on generated boards only. Nobody can measure it on the real one.
- **Boards between 6x6 and 16x16 with every frame enumerated.** 8x8 (5 frame
  colours) has a frame tree above 10^8 nodes; two 7x7 settings did not finish
  one board in 3 core-minutes each. I stopped them. The 6x6 boards are more
  constrained than the official set.
- **Distinct pieces in the ring counts.** `04`, `06`, `07` use reusable
  pieces. Section 4 uses distinct pieces.
- **Stronger refuters** (clause learning, all-different propagation). Not
  run. The count argument of section 2 makes them moot for whole frames, not
  for partial frames inside a row scan.
- **Belief propagation itself.** Not run. I rely on the transfer-matrix result
  and on one line of `EXPLORED.md`.
- **The site pages** on the fluid frame, frame-is-not-the-basin and spectral
  planted recovery. Not opened; quoted only through the review.
- **Whether a full cell order built around the four corner lists beats row
  scan in the model.** Out of scope; the review's hill climb found nothing
  below the spiral for the interior, and path 5 tried ten families.

## 10. Experiment, if one is wanted

The friendliness signal survives as a measurement. It does not survive as a
route to a solution. One experiment would settle whether it converts to nodes.

**Run.** 100 generated planted boards with a unique solution, at the largest
size the compiled strict engine exhausts in under a minute each (expected 9x9
or 10x10 with clues). Two arms, same row scan, same engine: (A) default piece
order at every cell; (B) at each border cell, candidates sorted by the
first-ring count of the corner window they complete (a table lookup). Measure
nodes to the first solution. About 3 core-hours.

**Kill line, written before the run.** Dead if the geometric mean of
nodes(A) / nodes(B) is below 2.0, or if its 95 percent interval includes 1.
Dead also if the ratio at the larger of two sizes is below the ratio at the
smaller.

**Odds.** Ratio above 2: 25 percent. Ratio above 16 (four bits): 3 percent.
That it bears on a full solution of the official puzzle: under 0.5 percent.
The remaining gap is 17 to 20 orders and this is worth at most one. That it
bears on passing 470: about 1 percent. Score hunts do not use exact frames,
and the site reports no border effect on scores (reported through the
review). I would not spend the night's budget on it ahead of the review's
population estimator.
