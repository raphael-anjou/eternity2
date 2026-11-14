# Gap 5. A global structure of this particular tile set

> **Status (2026-11-13):** independently re-checked, see `notes/gap-5-tile-set-structure-recheck.md`.
> **Re-check's rulings:**
> - Verdict "the gap is empty": stands with a narrower scope. Re-check's wording: "Nothing found, and three families closed on the tested prefixes: edge weightings of period 1 to 4; relations between tile counts up to (tile, rotation, cell colour); boundary words in the clue-free tree. Not closed: position-resolved integer relaxations, the non-abelian group of the unplaced tiles beyond small quotients."
> - "Complete in the abelian case" and "the flux family is complete": unproven by the memo (section 3.2 computes edge weightings, a different object from the count relations), true by the re-check's measurement.
> - Price "0 bits expected; under 0.3 bits by measurement": wrong as a price of the gap. 0.27 bits is the 95% bound on what one linear program removes, and that program is blind at every depth sampled.
> - "All colouring-type invariants" and "none mentions an interior colour": stand with a narrower scope. The period-2 invariant puts weight -1 on every interior colour; correct form: none distinguishes one interior colour from another. For periods 2 and 4 one more invariant exists; longer periods were not computed by anyone.
> - D2 (modular counting adds nothing): stands with a narrower scope, it says nothing about congruences valid only on 0/1 points.
> - D3 "a boundary invariant cannot count uses": stands for the tileability criterion, wrong as a general statement. Tile invariants (relations between use counts) are a separate test; the second analysis closed the hole on 37 of 37 clue-free prefixes and 4 of 4 clue-pinned ones.
> - "No invariant of the tile set alone, of any kind, refutes them": stands with a narrower scope, a statement about the clue-free tree.
> - Test 1, "0 of 184 refuted": a correct count with no evidential weight, and wrong for "the laws". All 184 prefixes are dead; arc consistency refutes 52 of the 77 proven dead; the same laws as a 0/1 program refute 8 of 8 at depth 192.
> - Claim 3 ("cannot refute any prefix") for the real puzzle: wrong for the clue-pinned tree. 13 of 31 clue-pinned prefixes at depths 16 and 32 admit no filling even with reuse; 12 of the 13 fall to arc consistency at the clue cells.
> - Section 4, a constraint outside the family "would have to use the each-tile-once rule ... not a structure of this tile set": wrong as a boundary of the gap; tile invariants are inside the gap and happen to be empty.
> - "No hidden parity system of the Tseitin kind": unproven as derived, supported by measurement. Thurston, Reid, Pak "not opened. Nothing below rests on them": wrong on the second sentence.
> - Odds 0.3% for 10 bits: not ruled, a judgment.
> **Read with:** `notes/gap-5-tile-set-structure-recheck.md` (section 11 gives the corrected statements, section 12 what is left open).

Desk study, 2026-11-13 afternoon. DRAFT until independently re-checked. Scripts and raw outputs:
`gap-5/` next to this file. Compute used: about 26 core-minutes, one core,
longest single run 9.6 minutes (after the cap was raised to 15). Every statement
carries a grade: **measured** (a script here ran it on the official pieces),
**derived** (argued here, short proof), **cited**, **guessed**.

## 0. Verdict

**The gap is empty.** No exact constraint derivable from the tile set alone was found
that is not already a sum of cell-by-cell matching conditions, and the family of such
sums is shown to be complete in the abelian case and powerless on the sampled prefixes
in the general case. Price: **0 bits expected; under 0.3 bits by measurement** before
depth 192, and under 0.001 bits after it.

1. No symmetry: no colour permutation, with or without a mirror, preserves the set
   (measured, exact).
2. All colouring-type invariants of the set (linear or modular) form the group
   Z^2 + Z/2: edge counts of a region, and "a region cuts the frame ring an even number
   of times". None mentions an interior colour. Same on 30 of 30 painted boards
   (measured, exact).
3. No boundary invariant of the tile set, abelian or not, can refute any of 184 sampled
   row-scan prefixes of the real pieces (measured; the implication is derived).
4. The laws that know which tiles remain (flux, checkerboard, row census, as one linear
   program) refuted 0 of 184 prefixes at depths 16 to 192; on painted boards they bite
   only with three rows or fewer left (measured).
5. No sub-families by palette; tensor ranks as in painted boards (measured).

## 1. The gap, and what the evidence covered

Covered before today. Thirty-one statistics (about 25 distinct, per the second analysis) of the
piece set against the exchangeable painter, and the numbering as a sort
(`how-the-puzzle-was-made.md`, independently re-checked). The flux law on the site
(`web/content/research/why/flux-invariants.mdx`, graded sound by last night's theory
second analysis): ranks 22, 44, 40 and 21 mod 2, a checkerboard member, and two explicit
admissions, "whether any other linear invariant exists is not proven here" and "pruning
value unmeasured". ROUTES rows: Hall and cut-multiset pruning (A17, Track 4), "forbidden
pattern, border supply, Eulerian pruning" (claim without data), the typical-set colour
ledger (never built), ring purity (the frame as an Eulerian circuit).

Not covered, and done here: completeness of the list of invariants, modular and
non-abelian ones, automorphisms, sub-families, the tensor, and whether any of it
refutes a prefix of the real pieces.

## 2. Literature

- Conway and Lagarias, "Tiling with polyominoes and combinatorial group theory", JCTA 53
  (1990) 183 to 208. **Opened** (scan of the paper, pages 1 to 3 read:
  <https://olympiads.mccme.ru/lktg/2009/4/articles/conway.pdf>). The abstract gives the
  hierarchy this memo uses: "Any coloring (or weighting) argument that proves
  nonexistence of a tiling of a region also proves nonexistence of any signed tiling";
  boundary invariants are strictly stronger. Page 184 notes that coloured squares (Wang
  tiles) become shapes by serrating the edges, which is the dictionary to edge matching.
- Demaine and Demaine, "Jigsaw puzzles, edge matching, and polyomino packing", Graphs and
  Combinatorics 23 (2007). **Opened** (<https://erikdemaine.org/papers/Jigsaw_GC/>):
  NP-complete, with conversions between the three puzzle types. Only
  instance-specific structure could help, which is what section 3 looks for.
- "The topology of tile invariants" (arXiv:1507.02315). **Abstract opened**: the tile
  counting group as a quotient of a second homology group; the object of section 3.2.
- Thurston (1990), Reid (2003) and Pak's survey of tile invariants: **not opened** (403,
  not found, 404). Nothing below rests on them.

I found no source that applies boundary invariants to an edge-matching instance with an
each-tile-once rule (a statement about my search).

## 3. Results

### 3.1 Three short derivations

**D1. Every law of this kind is an identity on the filled part (derived).** Take any
set J of joints and any weights. Summing the matching conditions of the joints in J gives
a law. Flux (all horizontal joints, or all vertical), the checkerboard law (joints by
the parity of their left cell), per-row and per-column censuses, flows per colour, and
the per-colour parity are all of this form. On a valid partial board every joint inside
the filled region is matched, so every such law restricted to the filled region holds
identically. In particular the parity "unplaced half-edges of colour c minus frontier
demand is even" is automatic: the total is even, internal joints use an even number, and
what is left of the placed half-edges is exactly the frontier. So these laws can never
refute a prefix through what has been placed. They can only act as look-ahead on what
has not: "the unplaced tiles cannot satisfy the laws on the empty cells".

**D2. Modular counting adds nothing to linear counting plus integrality (derived).** If
a congruence u.x = d (mod m) holds on all integer points of the affine space Ax = b of
the matching and assignment equations, then u vanishes mod m on the integer kernel of A,
which is a saturated lattice, so u lies in the saturated row lattice plus m Z^n. A
parity argument that uses only the balance equations is a linear argument read mod m.
The only place a genuinely modular fact can hide is torsion in the tile lattice, which
3.2 computes.

**D3. A boundary invariant cannot count uses (derived from the cited definition).** The
Conway and Lagarias condition is that the boundary word of a region lies in the normal
subgroup generated by the tile words. Any filling of the region, with tiles reused as
often as wanted, puts the word in that subgroup. So if a reuse filling exists, every
boundary invariant, abelian or not, is trivial on that region. The whole board is
tileable (the puzzle was made from a solved board), so at the root all of them are
trivial.

### 3.2 The abelian invariants, exactly (`01_lattice_and_automorphisms.py`)

Coordinates: one per (horizontal or vertical, colour), grey included: 46. Each tile in
each rotation gives the vector top minus bottom plus right minus left. L is the lattice
they span. Hom(Z^46 / L, A) is the set of all weightings with values in A that vanish on
every tile, that is, every colouring argument.

| object | official set | 30 painted boards (same colour counts) |
|---|---|---|
| rank of L | 44 of 46 | 44, all 30 |
| Smith form of L | 43 ones and one 2 | torsion rank 1 at p = 2, none at 3 to 13, all 30 |
| Z^46 / L | Z^2 + Z/2 | same |
| 2 x 2 cover (184 coordinates; contains the checkerboard) | corank 6 = 5 geometric + 1; no torsion at p up to 13 | same, all 30 |
| 4 x 4 cover (736 coordinates) | corank 18 = 17 geometric + 1; no torsion | not run |

Measured, exact (rank over a 31-bit prime field plus the exact Smith form for the first
row). The rational annihilator at k = 1 is printed in the output: "all ones on
horizontal coordinates" and "all ones on vertical coordinates". The Z/2 is the weighting
"1 on the five frame colours": every tile has zero or two frame-coloured sides. On a
board it says a closed region cuts the ring an even number of times, which is geometry
(derived). The one extra invariant of the covers is present on every painted board, so
it is a property of the 5 and 17 palette split and not of this set; I did not identify
its support (script 2, part a, was inconclusive).

Reading: for the official set there is **no** colouring argument, linear or modular,
involving an interior colour. This closes the page's open sentence for boundary
weightings: the flux family is complete.

### 3.3 Automorphisms (same script)

Colour refinement separates all 23 colours on the official set and on 30 of 30 painted
boards, so the only colour permutation preserving the set is the identity (measured,
exact: refinement classes are a superset of orbits). The mirror image of the set has a
different multiset of colour signatures, so no mirror combined with a recolouring works
either. No interior tile has its mirror image in the set; 6 border tiles do. Bits: 0.

### 3.4 Sub-families and the tensor (`02_decomposition_tensor_classes.py`)

Table in appendix A, all measured against 40 painted boards; its last two rows are a
Gaussian local-limit estimate (derived approximation, not an exact count). A zone structure would need many tiles pure on each
side of some palette split; the best split leaves 19 pure tiles of 196 on the smaller
side, as in painted boards. No rank deficiency, no block structure.

The last two rows answer the "flows" question. Per colour, the joints are a perfect
matching between half-edges on black cells and on white cells; each such polytope is
integral (bipartite), so alone it gives no cut. Jointly they pin 200 of the 588 bits of
the coarse class of the interior tiles. Those are not search bits: by D1 a search
already satisfies them on everything it has placed, and guessing the class first means
choosing among about 2^388 balanced classes (derived from the table).

### 3.5 Do the laws refute real prefixes? (`03_prefix_conservation_lp.py`, `04_reuse_completion.py`)

Prefixes: randomised row-scan depth-first search on the official pieces, **no clue
pinned** (the clue-free tree), first arrival at each row boundary, 16 independent
restarts of 200,000 nodes. 184 prefixes at depths 16 to 192 (8 reach 192). First
arrivals are not a uniform sample of a level.

**Test 1: the aggregated laws.** After k rows, the unplaced tiles must be assignable to
classes (row, column parity, rim pattern, rotation) so that per colour and per row east
faces meet west faces across both parities, south faces of each row equal north faces of
the next per parity, and row k reproduces the frontier per parity. This contains flux,
checkerboard and every row census. Linear relaxation, HiGHS.

| | refuted |
|---|---|
| official pieces, depths 16 to 192, 184 prefixes on 16 paths | **0 of 184** |
| control, painted boards, true remainder with two tiles exchanged for two placed tiles of the same eight colours (census intact), 10 boards: 8, 6 rows left | 0 of 10, 0 of 10 by the LP; integer program undecided in 20 s |
| same, 4 rows left | LP 0 of 10; integer program refutes 3, undecided 7 |
| same, 3 rows left; 2 rows left | LP refutes 10 of 10; 9 of 9 |

Measured. The integer version timed out (5 s) on all 23 real prefixes of a first run and
is undecided there. A first control that exchanged one tile was void: it breaks the
parity of two colour counts, which D1 says a valid prefix cannot do (70 of 72 refuted,
`03_first_run_one_tile_control.out`, kept for the record).

**Test 2: any boundary invariant.** By D3 a reuse filling of the empty cells kills every
boundary invariant of the frontier.

| depth | A: any of the 256 tiles reusable | B: only unplaced tiles reusable |
|---|---|---|
| 16 to 96 (96 prefixes) | filled 96 | filled 60, undecided at the cap 36 |
| 112 to 144 (48) | filled 48 | filled 4, undecided 30, no filling exists 14 |
| 160 to 192 (40) | filled 40 | no filling exists 40 |

Measured. Column A is a proof for these 184 prefixes: no invariant of the tile set
alone, of any kind, refutes them. Column B shows where look-ahead by relaxation does
refute: from about depth 144, past which the level widths fall. It is not a gain. The
reuse tree contains the exact subtree under the same order (derived), and the measured
sizes say the same: at depth 160 the exact subtree has a median of 17 nodes against 24
for the reuse refutation (16 prefixes; maxima 100 and 25,716).

### 3.6 Price

- Before depth 192: 0 of 16 independent paths refuted at any row boundary. The 95% upper
  bound on the share of paths refuted is 17%, which is 0.27 bits (measured bound; my
  expectation is 0).
- From 3 rows left, where the control shows power: depth 208 and beyond. The measured
  profile puts 99% of the nodes before free depth 175 and argues a share of 10^-4 past
  190 (cited, `RESULTS.md` at `dc71ad56`). Removing all of it is 0.0001 bits.
- Symmetry: 0 bits. Decomposition: 0 bits.

## 4. Mechanisms

None proposed. The assumption the empty verdict rests on, stated once: **the laws in
question are the sums of matching conditions over cell classes, plus boundary
invariants.** D1 to D3 and the tables cover that family. A constraint outside it would
have to use the each-tile-once rule in an essential way, which is obstruction O1's
territory and gap 1's, not a structure of this tile set.

## 5. The experiment that would falsify "empty"

The family of 3.5 is a ladder: finer cell classes give stronger tests, and the top rung
(every cell its own class) is the full linear relaxation with the assignment
constraints, which dominates every other rung (derived). So one toy measurement closes
or opens the whole ladder.

- Boards: 20 painted 8 x 8 boards at the critical colour count (toys, not Eternity II).
- Measured: at the widest level of the row-scan tree, enumerate nodes; for each dead
  node, does the full linear relaxation refute it; same for arc consistency.
- Baseline: the share arc consistency refutes (today's the log: about 5 bits of nodes
  for 13 bits of cost on the real profile).
- Kill line: the relaxation refutes less than twice the dead nodes arc consistency
  does, or costs more node-equivalents than it removes.
- Cost: about 1 core-hour.

A pass would not be a structure of the tile set. It would say linear relaxation
propagates better than arc consistency (ROUTES row "LP / ILP relaxations", reported,
never measured as a node test).

## 6. Odds

That any exact tile-set constraint of this family yields 10 bits on the real puzzle:
0.3% (guessed; the residue is the non-abelian invariants of the *unplaced* tiles at
depths below 144, column B's undecided cells). 56 bits: under 0.01%.

## 7. What I could not check

- The tile homotopy group of the official set itself (non-abelian) was not computed.
  Column A makes it irrelevant for the sampled prefixes; column B leaves 66 of 144
  prefixes at depth 144 or less undecided for invariants of the unplaced tiles.
- The integer form of test 1 on real prefixes (timeouts).
- Prefixes with the five clues pinned, and prefixes deeper than 192.
- The support of the extra invariant of the 2 x 2 and 4 x 4 covers.
- Polynomial certificates above degree 1 (Sherali-Adams or sum-of-squares levels). D2
  and the absence of torsion say there is no hidden parity system of the Tseitin kind
  for Gaussian elimination to exploit (derived); higher degrees were not examined.

## Appendix A: table of section 3.4

| quantity | official | 40 painted boards: min, mean, max |
|---|---|---|
| best palette split: over all 65,535 subsets S of the 17 interior colours, max of min(tiles painted only from S, tiles painted only from the rest) | 19 (|S| = 9: 21 and 19 tiles) | 18, 19.5, 22 |
| rank of the tile tensor flattened (top,right) by (bottom,left), 289 x 289 | 267 | 251, 263.5, 275 |
| flattened (top,bottom) by (right,left) | 256 | 244, 254.4, 264 |
| flattened top by the rest, 17 x 4913 | 17 | 17, 17, 17 |
| connected components of the first flattening | 9 | 1, 4.8, 10 |
| bits the flux law removes from the 588 bits of (cell parity, rotation) of the interior tiles | 114.2 | 113.7, 114.3, 115.2 |
| same, flux with checkerboard | 200.3 | 200.1, 200.3, 200.5 |


## Appendix B: files in `gap-5/`

- `e2common.py`: loader and exchangeable painter, copied from
  `excluded/`.
- `01_lattice_and_automorphisms.py`, `.out`: sections 3.2 and 3.3. 20 s.
  `uv run --with numpy --with sympy python 01_lattice_and_automorphisms.py`
- `02_decomposition_tensor_classes.py`, `.out`: section 3.4. 2 s. Same command form.
- `03_prefix_conservation_lp.py`: test 1. `... real 16 200000 0 999` gives
  `03_real_lp.out` (8.7 min); `... control 20 10` gives `03_control.out` (9.6 min);
  `03_first_run_one_tile_control.out` is the first run with the void control (4.5 min).
  Needs `uv run --with numpy --with scipy`.
- `04_reuse_completion.py`, `.out`: test 2. `python3 -I 04_reuse_completion.py 16 200000 200000`, 25 s.
