# Second analysis report: gap 5, a global structure of this particular tile set

2026-11-13, evening. Memo under review: `notes/gap-5-tile-set-structure.md`, read in full
with its four scripts and their outputs. I wrote my own loader, lattice code, fillers and
tests from the official piece file, then reran the author's scripts 01 and 02 (identical
output). One core, `nice -n 10`, no run over 15 minutes, about 95 core-minutes in
all. My scripts and outputs: `notes/gap-5-review/` (list at the end). Tags:
[measured] on the official pieces by me, [derived] by me, [cited] opened today.

## Summary

The verdict survives, and on one axis it is now better supported than the memo supports
it. But two of its five numbered claims carry no evidential weight as written, one
derivation has a hole that the memo's own literature fills, and one claim does not
transfer to the real puzzle.

1. **The invariant group reproduces.** Z^46 / L = Z^2 + Z/2 for the official tiles, with
   rotations free, by an independent exact Smith form. Three positive controls of my own
   find the invariants I planted. The "one extra invariant" of the 2 x 2 and 4 x 4 covers
   that the memo left unidentified is named and verified here; it is geometry.
2. **The memo computed the wrong group for its own conclusion.** Z^46 / L is the group of
   edge weightings that vanish on every tile (Pak's colouring group). The flux and
   checkerboard laws are not elements of it; they are relations between tile counts
   (Pak's tile invariants). "The flux family is complete" therefore does not follow from
   section 3.2. I measured the right group: at four resolutions, up to
   (tile, rotation, cell colour), every linear or modular relation between counts that
   holds over all fillings of a region is a named sum of matching conditions. The claim
   is true. It was not shown.
3. **D3 has a hole.** "A boundary invariant cannot count uses" is right for the
   tileability criterion and wrong as a description of the theory: tile invariants are
   exactly relations between use counts, and Pak's survey says they were "implicitly
   introduced" by Conway and Lagarias. Column A of test 2 does not cover them. I closed
   the hole with a certificate that does: on 37 of 37 clue-free prefixes tested and on
   4 of 4 clue-pinned ones, no tile invariant of any origin can separate the
   each-tile-once completion from a filling with reuse.
4. **"0 of 184 refuted" has no power.** All 184 prefixes are dead (77 proven by
   exhaustion, the other 107 dead with probability above 1 - 10^-5 by counting). On the 77
   proven dead, the memo's linear program refutes 0, plain arc consistency refutes 52,
   and "can the next row be completed" refutes 62. The null result measures the weakness
   of that linear program, not the absence of structure. "Under 0.3 bits by measurement"
   is a bound on one blind test.
5. **Claim 3 does not transfer to the real puzzle.** With the five clues pinned, 13 of
   31 prefixes at depths 16 and 32 admit no filling even when all 256 tiles may be
   reused. So relaxations that ignore the each-once rule do refute real prefixes. All but
   one of the 13 fall to arc consistency at the clue cells, so this is the known
   clue-window effect and not a global structure, but the memo's sentence "no boundary
   invariant can refute" is a statement about the clue-free tree only.
6. **The tile group is not abelian, and it does not matter.** It has non-abelian images
   in S3, D4, Q8, D5, D6 and S4. Every one of them sees only the tile type (grey, frame,
   interior), never a colour. For the unplaced tiles, the memo's open item, quotients of
   order up to 12 refute 0 of 32 undecided prefixes, and also 0 of 54 prefixes known to be
   untileable with reuse: a null without power. That item stays open.
7. **The integer form of test 1 does refute; the signed form does not.** As a 0/1
   program the same aggregated laws refute 8 of 8 prefixes at depth 192, where the linear
   program refutes none. Mod 2, 3, 5, 7 the system is always solvable: no parity
   obstruction. The top rung of the memo's ladder (every cell its own class) refutes 22 of
   32 dead prefixes at depths 128 and 144 against 12 for arc consistency, at a cost four
   orders above the subtrees it removes.

Nothing I computed opens the gap. What remains open is listed in section 12.

## 1. Rulings

| # | Claim (memo section) | Ruling | Corrected statement or number |
|---|---|---|---|
| 1 | Verdict: "the gap is empty" (0) | **Stands with a narrower scope** | "Nothing found, and three families closed on the tested prefixes: edge weightings of period 1 to 4; relations between tile counts up to (tile, rotation, cell colour); boundary words in the clue-free tree. Not closed: position-resolved integer relaxations, the non-abelian group of the unplaced tiles beyond small quotients." Full wording in section 11 |
| 2 | "complete in the abelian case" (0) | **Unproven** by the memo, **true** by my measurement | Section 3.2 computes edge weightings, a different object from the count relations the flux law lives in. Completeness of the flux family: measured in my section 3 |
| 3 | Price: "0 bits expected; under 0.3 bits by measurement" (0, 3.6) | **Wrong** as a price of the gap | 0.27 bits is the 95% bound on what one linear program removes, and that program is blind at every depth sampled (section 5). No measurement here prices the gap. The expectation of 0 is a judgment I share |
| 4 | No colour automorphism, with or without mirror (3.3) | **Stands** | Exhaustive backtracking instead of colour refinement: 1 map (the identity), 0 mirror maps; 37 achiral tiles, 6 with their mirror image in the set. Two controls pass (`r3b`) |
| 5 | Z^46 / L = Z^2 + Z/2, the Z/2 is "frame colours" (3.2) | **Stands** | Reproduced exactly; definition and controls in section 2 |
| 6 | "All colouring-type invariants (linear or modular)" (0, item 2) | **Stands with a narrower scope** | True for weightings of period 1, and of period 3 (my run). For periods 2 and 4 one more exists (section 2.4). Longer periods were not computed by anyone; weightings with no period are the cell-level linear relaxation |
| 7 | "None mentions an interior colour" (0, item 2) | **Stands with a narrower scope** | The period-2 invariant puts weight -1 on every interior colour. Correct form: none distinguishes one interior colour from another |
| 8 | The extra invariant of the covers "is a property of the 5 and 17 palette split" (3.2) | **Stands**, now proven | It is the weighting (-1)^(a+b) g(c) on horizontal sides and its negative on vertical sides, g = +1 on grey, 0 on frame colours, -1 on interior colours. It says tile type matches cell type. Vacuous on any partial board (section 2.4) |
| 9 | "This closes the page's open sentence: the flux family is complete" (3.2) | **Unproven** by the memo, **true** by my measurement, with one addition | The count relations are: rim-pattern counts, flux per colour, the same split by cell colour, and the flux of one rim column on its own. The last one is not in the memo's list. All are sums of matching conditions (section 3) |
| 10 | D1: every such law is an identity on the filled part | **Stands** | Checked indirectly: the each-once count vector passes every count relation on all prefixes tested |
| 11 | D2: modular counting adds nothing to linear counting plus integrality | **Stands with a narrower scope** | True for congruences valid on all integer points of Ax = b, signed ones included. It says nothing about congruences valid only on 0/1 points (Chvatal-Gomory cuts), and the torsion it defers to was computed for L, not for the prefix systems. I computed the latter mod 2 (section 6) |
| 12 | D3: "a boundary invariant cannot count uses" | **Stands** for the tileability criterion; **wrong** as a general statement | "If a filling with reuse exists, the boundary word is trivial in the tile group. Relations between use counts (tile invariants) are a separate test: compare the each-once count vector with the filling's, modulo the lattice of differences of fillings." Section 4 |
| 13 | Column A: every one of 184 prefixes has a filling with reuse (3.5) | **Stands** | Refound with my own filler on the 37 prefixes of `r4`; on all 184, every homomorphism of the full set into eight small groups is trivial on the region word, as it must be (`r7`) |
| 14 | "No invariant of the tile set alone, of any kind, refutes them" (3.5) | **Stands with a narrower scope** | "Of any kind" covers boundary words only. With my certificate it extends to tile-count invariants on the 37 prefixes tested. It is a statement about the clue-free tree (ruling 17) |
| 15 | Test 1: 0 of 184 refuted (3.5) | **Stands** as a count for the linear program, **no evidential weight**; **wrong** for "the laws" | My rebuild of the linear program: 0 of 35 (paths 0 to 2). The same laws as a 0/1 program refute 8 of 8 at depth 192. Sections 5 and 6 |
| 16 | Control: the LP bites only with three rows or fewer left (3.5) | **Stands** (not rerun) | This is the memo's own evidence that test 1 was applied only where it is blind: the deepest real prefix has four rows left |
| 17 | Claim 3 read for the puzzle: "cannot refute any prefix" | **Wrong** for the clue-pinned tree | 13 of 31 clue-pinned prefixes at depths 16 and 32 have no filling with reuse of all 256 tiles; 12 of those 13 are refuted by arc consistency at the clue cells. Past row 2: 292 of 304 filled, 1 none, 11 undecided. Section 7 |
| 18 | Column B and "it is not a gain" (3.5) | **Stands** | My numbers agree in kind: at depth 160 the exact subtree has median 17 nodes. A sound test there saves tens of nodes |
| 19 | No sub-families; tensor ranks as in painted boards (3.4) | **Stands** (rerun only) | Script 02 rerun, identical output. Not recomputed independently |
| 20 | "200 of the 588 bits ... are not search bits" (3.4) | **Stands** (rerun only) | Same |
| 21 | Section 4: a constraint outside the family "would have to use the each-tile-once rule in an essential way, which is ... not a structure of this tile set" | **Wrong** as a boundary of the gap | Tile invariants use the rule linearly and are a structure of the tile set. They are inside the gap. They happen to be empty (section 4) |
| 22 | Section 5 experiment (toy, full linear relaxation against arc consistency) | **Stands**, and run here on the real tiles | Cell-level linear relaxation: refutes 6 of 16 dead prefixes at depth 128 and 16 of 16 at depth 144; arc consistency 3 and 9. Ratio 2.0 and 1.8 against a kill line of 2; fails the cost clause by four orders. Section 6 |
| 23 | Section 7: "no hidden parity system of the Tseitin kind" (derived from D2 and no torsion in L) | **Unproven** as derived, **supported** by measurement | The torsion that matters is that of the prefix systems, not of L. Coarse system solvable mod 2 on 184 of 184 clue-free and 312 of 312 clue-pinned prefixes; mod 3, 5, 7 on 12 of 12, with no rank drop. Cell-level system solvable mod 2 on 48 of 48. Section 6 |
| 24 | Literature: Thurston, Reid, Pak "not opened. Nothing below rests on them" | **Wrong** on the second sentence | D3 and section 4 rest on what Pak's survey is about. Section 10 |
| 25 | Odds 0.3% for 10 bits | not ruled | A judgment. Nothing here moves it up |

## 2. The invariant group, recomputed (`r1_lattice.py`, `r1b_cover_invariant.py`)

### 2.1 Definition used

Generators h_c and v_c for each of the 23 colours c (grey is colour 0): 46 in all. A tile
(top, right, bottom, left) = (t, r, b, l) placed in a cell has boundary word
h_b v_r h_t^-1 v_l^-1 (walk round the cell anticlockwise from its bottom left corner). Its
abelian image is the vector h_b - h_t + v_r - v_l in Z^46. **Rotations are free**: each
tile contributes its four rotations. L is the lattice spanned by these vectors (1,004
distinct ones). The
invariant group is Z^46 / L; a weighting of sides with values in an abelian group A that
vanishes on every tile in every rotation is an element of Hom(Z^46 / L, A). This is the
memo's definition up to a global sign.

### 2.2 Result [measured, exact]

| tile set | rank of L | Z^46 / L |
|---|---|---|
| 256 official tiles, rotations free | 44 | **Z^2 + Z/2** |
| same, rank of L mod p | 43 at p = 2; 44 at p = 3, 5, 7, 11, 13, 17, 19, 23 | |
| 196 interior tiles alone | 32 | Z^14 (Z^2 on the 34 coordinates they use) |
| 60 border tiles alone | 44 | Z^2 + Z/2 |
| 251 tiles without the five clue tiles | 44 | Z^2 + Z/2 |
| stored orientation only (not the puzzle; for the record) | 40 | Z^6 |

Elementary divisors by a pure-Python integer Smith form written for this review, not by
sympy. The two free generators are "all ones on h" and "all ones on v"; the Z/2 is "1 on
the five frame colours, h and v alike" (checked: vanishes mod 2 on every tile, not over
Z). The memo's row stands.

One remark on what this number is worth. Sixty border tiles alone already give the full
answer. With rotations free, a tile (t, r, b, l) puts both h_b - h_t + v_r - v_l and
h_r - h_l + v_t - v_b in L, and a few dozen generic tiles are enough to span everything
that the two colour-blind weightings and one parity do not forbid. A random tile set
gives the same group (30 of 30 painted boards in the memo, 5 of 5 in mine). So
Z^2 + Z/2 is true and close to content-free: it says the official set is generic for
this test, which is all the memo claims for it.

### 2.3 Positive controls [measured; generated tile sets, not Eternity II]

| control | planted | found |
|---|---|---|
| 16 x 16 painted board whose second ring of cells is joined by three dedicated colours (every tile has 0 or 2 sides in them) | one more Z/2 | Z^2 + Z/2 + Z/2, 5 of 5 boards; plain boards give Z^2 + Z/2, 5 of 5 |
| 12 x 12 torus board, colours in three charge classes, opposite sides of every tile in the same class | free rank 8 (2 unused grey coordinates plus 2 per class) | Z^8 |
| four-tile toy | none planted; Smith form against brute-force enumeration of all weightings mod 2, 3, 4 | 32, 81, 512 weightings both ways |

The computation finds a planted torsion invariant and a planted free one.

### 2.4 The covers, and the invariant the memo did not identify

Period-k weightings (coordinates per side type, cell class in Z_k^2, colour):

| k | coordinates | rank | corank | colour-blind weightings | rank drop mod 2, 3, 5, 7, 11, 13 |
|---|---|---|---|---|---|
| 1 | 46 | 44 | 2 | 2 | 1 at p = 2, else 0 |
| 2 | 184 | 178 | 6 | 5 | 0 |
| 3 | 414 | 404 | 10 | 10 | 1 at p = 2, else 0 |
| 4 | 736 | 718 | 18 | 17 | 0 |

The memo's table reproduces. The extra invariant at k = 2 and 4 is [measured, and a
two-line proof]: weight (-1)^(a+b) g(c) on a horizontal side of colour c at cell class
(a, b), and -(-1)^(a+b) g(c) on a vertical side, with g(grey) = +1, g(frame) = 0,
g(interior) = -1. It vanishes on a tile iff g(top) + g(bottom) = g(right) + g(left) in
every rotation: -2 = -2 for an interior tile, 0 = 0 for an edge tile (grey opposite an
interior colour, two frame colours opposite each other), 1 = 1 for a corner tile. So it
exists for any set made of corner, edge and interior tiles with a frame palette, whatever
the colours, which proves the memo's guess. On a board it carries no information: which
sides are grey, frame or interior is fixed by the cell, so its value on any region is
fixed by the shape of the region.

I added period 3 (not in the memo): nothing beyond the colour-blind weightings and the
same Z/2. Two limits of this table that the memo does not state: periods beyond 4 were
not computed, and the absence of torsion is checked at primes up to 13 only for k above
1 (exact Smith form at k = 1 only).

## 3. The flux family is complete: measured (`r8_count_groups.py`)

Section 3.2 of the memo ends "the flux family is complete". The flux law says, for each
colour, that as many tiles show it on their east side as on their west side. That is a
relation between **counts of tiles by rotation**, and it holds because each joint has two
equal sides, not because of any weighting in Hom(Z^46 / L, A). Pak's survey separates the
two objects: the colouring group O(T) (weightings vanishing on every tile, the memo's
computation) and the group of tile invariants G(T) (relations between the numbers of
copies of each tile over all tilings of a region), and notes that "in general not all
tile invariants can be obtained by the extended coloring arguments" (p. 2, [cited]). So
completeness has to be measured on G, and Pak gives the method (p. 7): a set of pairs of
tilings of the same region whose count differences span a lattice of rank above n - r
certifies that the group has rank below r.

Method. One region (the empty cells below a prefix). Sample fillings with tiles reusable,
any of the 256. Record each as a count vector at four resolutions. D = lattice spanned by
the differences. Every relation that holds over all fillings vanishes on D, so Z^n / D
bounds the group of relations from above; the relations I can name bound it from below.

| resolution | coordinates seen | rank of D | relations (free) | named | unexplained, over Q and mod 2, 3, 5, 7 |
|---|---|---|---|---|---|
| tile | 256 | 249 | 7, plus 16 mod 2 | 7, plus 16 mod 2 | 0 |
| tile x cell colour | 512 | 482 | 30 | 30 | 0 |
| tile x rotation | 960 | 910 | 50 | 50 | 0 |
| tile x rotation x cell colour | 1,912 | 1,814 | 98 | 98 | 0 |

[measured] Identical on two regions: depth 16 of path 0 (2,760 fillings) and depth 96 of
path 5. The rank is the same with 80% of the fillings as with all, so the sample has
converged. Exact Smith form at the first two resolutions (torsion (Z/2)^16 and none);
ranks over a 31-bit prime and mod 2, 3, 5, 7 at the other two.

The named relations, all of them sums of matching conditions or geometry:

- how many cells of each rim pattern, and of each cell colour, the region has;
- for each colour, east sides = west sides and south sides = north sides up to the
  frontier (flux), and the same with the two sides taken on opposite cell colours
  (checkerboard);
- at the "tile" resolution, where rotation is forgotten: the number of corner, edge and
  interior tiles; for each frame colour, tiles leaving through it minus tiles entering
  through it along the rim (the rim below a prefix is one path, so this is fixed by its
  two ends); and for each colour the parity of its number of sides (the memo's D1);
- **the flux of the left rim column alone.** My first run left 4 relations unexplained at
  (tile, rotation) and 8 with cell colours (`r8_depth16_first_run.out`). They are the
  south = north balance per frame colour restricted to the left-edge cells, which the
  per-colour flux lumps with the right column. With them added, nothing is left.

So the memo's sentence is true, with one item added to the family, and with the scope:
position-free counts up to (tile, rotation, cell colour), on two regions, for fillings
with all 256 tiles. Counts resolved by row or by cell are the ladder of the memo's
section 5 and are not covered by this.

## 4. Tile invariants: the hole in D3, closed (`r4_tile_count_invariants.py`)

D3 argues that the boundary word of a region lies in the normal subgroup generated by the
tile words as soon as one filling with reuse exists, so column A kills every boundary
invariant. That is correct [cited: Thurston p. 757, "the plane region can be tiled, only
if the group element which describes the boundary of the region is the trivial element"].
The memo then heads the paragraph "a boundary invariant cannot count uses" and concludes
in section 4 that anything using the each-tile-once rule is outside the gap.

The literature the memo lists but did not open says otherwise. Pak's group of invariants
"is a group of linear relations between the number of copies of tiles in tilings of the
same region" (abstract), and "tile invariants were implicitly introduced in [CL] in order
to obtain new tileability criteria" (Remark 1.2). A tile invariant constrains an
each-once completion directly: its count vector u is the indicator of the unplaced
tiles, and every invariant must take the same value on u as on any filling with reuse.

Certificate [derived]. Let a(A) in Z^256 count the copies of each tile in a filling A,
rotations merged. Let D be the lattice spanned by a(A_j) - a(A_0) over sampled fillings
with reuse. If u - a(A_0) lies in D, then every homomorphism of Z^256 that is constant on
the fillings of the region takes on u the value it takes on A_0: no tile invariant, of
any modulus and any origin (colouring, height function, non-abelian), can tell the
each-once completion from a filling with reuse. A sampled D is a sublattice of the true
one, so membership is a proof and not an estimate.

Result [measured, exact Smith forms, 400 fillings per prefix]:

| prefixes | Z^256 / D | u - a(A_0) in D |
|---|---|---|
| clue-free, paths 0, 1, 2 at all twelve depths and path 3 at depths 16 and 32: 37 | Z^7 + (Z/2)^16, every time | **37 of 37** |
| clue-pinned, clues kept in the fillings: path 0 at depth 16, paths 1, 6, 15 at depths 48, 96, 144 | Z^7 + (Z/2)^16, every time | **4 of 4** |

The group Z^7 + (Z/2)^16 is entirely named: three counts (corner, edge, interior tiles),
four independent rim flows (five frame colours, one dependency), and sixteen independent
colour parities. All are in the memo's family, and D1 shows a valid prefix satisfies
them. So the memo's conclusion holds for tile invariants too. It holds because I checked,
not because of D3.

The same construction with only the unplaced tiles reusable (version B) was not run.
Where a version B filling exists it would be the sharper statement.

## 5. Does "0 of 184 refuted" have power? (`r2a_prefix_death.py`)

A sound test can only refute dead prefixes, so the first question is how many of the 184
are dead, and the second is whether any test with known power refutes them.

**All of them are dead.** [measured] Exact each-once search below each prefix, cap
400,000 nodes:

| depth | prefixes | proven dead | median nodes of a dead subtree | arc consistency refutes | next row cannot be completed |
|---|---|---|---|---|---|
| 16 to 96 | 96 | 0 (cap) | | 0 | 0 |
| 112 | 16 | 5 | 17,858 | 2 | 2 |
| 128 | 16 | 16 | 920 | 3 | 7 |
| 144 | 16 | 16 | 83 | 9 | 15 |
| 160 | 16 | 16 | 17 | 14 | 14 |
| 176 | 16 | 16 | 19 | 16 | 16 |
| 192 | 8 | 8 | 17 | 8 | 8 |
| all | 184 | **77** | | **52** | **62** |

For the 107 not exhausted: the clue-free level 16 holds about 10^14.3 nodes and level 32
about 10^20.6 (Knuth estimate, 20,000 probes each), against about 14,702 x 784 = 10^7.1
complete boards in the clue-free tree (the starter-clue estimate spread over 196 cells
and 4 rotations) [derived]. A given depth-16 prefix is alive with probability about
10^-7.3, so the chance that any of the 107 is alive is below 10^-5.

**The memo's test refutes none of the 77 proven dead (its own run; mine agrees on the 35
prefixes I reran); arc consistency refutes 52.** Arc
consistency here is the plainest version: candidate sets per empty cell from the unplaced
tiles, no all-different, pruned to a fixed point, no search. The memo's own control
agrees: on painted boards the linear program bites only with three rows or fewer left,
and the deepest real prefix has four rows left. So test 1 was run exclusively at depths
where it is known to be blind, and the 0 is what a blind test returns.

What the null means, then:

- At depths 112 to 192, where truth is available: the linear program of the aggregated
  laws refutes nothing where arc consistency refutes 52. That is a fact about one
  relaxation.
- At depths 16 to 96, where a refutation would remove a large subtree: nothing in the
  memo or here has measurable power, because no prefix there can be proven dead at this budget. A null
  result at those depths cannot be told from a test that never fires. The only handle is
  a planted control, and the memo's control says "blind".
- The price sentence "under 0.3 bits by measurement" should go. It is 1 - 0.05^(1/16) =
  17% turned into bits, a correct calculation about a test with no power. The 16 paths
  are also not 184 independent prefixes: 122 of the 168 deeper prefixes extend the
  previous prefix of the same path [measured].

A caution in the other direction, so that this section is not read as "the gap is
open". Refuting a node at depth 160 saves about 17 nodes (the median dead subtree). Even
a perfect test at the row boundaries from 128 down is worth little. To matter, a test
has to refute at depths below 100, and there I found nothing either: arc consistency 0
of 96, next-row test 0 of 96, tile invariants pass, boundary words pass.

## 6. The integer form, and the top rung (`r6_integer_and_power.py`)

The memo ran test 1 as a linear relaxation and left its integer form undecided (5 s
timeouts). I rebuilt the system from the description (classes: row, column parity, rim
pattern, with rotation; one equation per tile, per class, and per colour and class for
east against west and south against north, frontier included) and ran it in the forms
the memo did not.

[measured]

| form of test 1 | prefixes | refuted |
|---|---|---|
| linear relaxation (reproduction) | 35 clue-free: paths 0 to 2, all depths | 0 |
| signed integers mod 2 (is A x = b solvable over GF(2); exact) | 184 clue-free | 0 |
| signed integers mod 3, 5, 7 (exact), and rank of A mod p against its rank mod a 31-bit prime | 12: path 0, all depths | 0; ranks equal, so no torsion at 3, 5, 7 |
| **0/1 integer program**, 45 s limit | 8: every prefix at depth 192 | **8 of 8**, each decided in 4 to 20 s |
| 0/1 integer program, 60 s limit | 6 at depth 176 | undecided, 6 of 6 timeouts |
| linear relaxation, five clues pinned (clue tiles confined to their class and rotation) | 62: 8 at depths 16 and 32, 54 at depths 48 to 176 on 6 paths | 0 |
| signed integers mod 2, five clues pinned | 312 | 0 |
| **cell level** (every cell its own class), linear relaxation | 16 at depth 128, 16 at depth 144; all proven dead | **6 and 16** (arc consistency: 3 and 9) |
| cell level, linear relaxation | 16 at depth 112, of which 5 proven dead | 2 (arc consistency: the same 2) |
| cell level, signed integers mod 2 | 48: depths 112, 128, 144 | 0 |

Three readings.

**The integer form bites where the linear one does not.** With four rows left the 0/1
program of the same aggregated laws refutes every prefix, the linear program none. So
item 4 of the verdict ("the laws ... refuted 0 of 184") is true of the relaxation and
false of the laws. It changes no price: the eight prefixes die in a median of 17 nodes
and arc consistency refutes all eight. With five rows left the 0/1 program does not
finish in a minute, so how early integrality starts to bite is not known.

**The gap between the two forms is not a parity.** Over signed integers the system is
solvable modulo 2 on every prefix, and modulo 3, 5, 7 where I ran it, with no rank drop.
Whatever the 0/1 program sees comes from the bounds, that is from cuts of the
Chvatal-Gomory kind, which D2 does not cover. The memo's sentence in section 7 ("no
hidden parity system of the Tseitin kind") is right, and is now measured on the prefix
systems instead of inferred from L.

**The memo's ladder is not flat, and its top is not affordable.** Section 5 of the memo
proposes to test the top rung on 8 x 8 toys. On the real tiles, at the two depths where
every prefix is proven dead and arc consistency still misses some, the cell-level linear
relaxation refutes 22 of 32 against 12 for arc consistency: ratios 2.0 at depth 128 and
1.8 at depth 144, against the memo's kill line of 2. It refutes everything arc
consistency refutes and 10 prefixes more (`r2b_depth112_flags.out`); at depth 112 the two
coincide (2 of 16, with 3 more proven dead that neither sees). On the cost clause it fails
outright: a feasible instance takes HiGHS about 17 s, against dead subtrees of 920 and
83 nodes in the median. The top rung has real power and no price. That agrees with the
ROUTES row "LP / ILP relaxations", and it is a property of linear relaxation against arc
consistency, as the memo says, not of this tile set.

## 7. Prefixes with the five clues pinned (`r5_clue_prefixes.py`, `r5b_clue_more.py`)

None of the memo's 184 prefixes has a clue pinned. I sampled the real tree with the memo's
protocol (randomised row scan, first arrival at each row boundary, 200,000 nodes per
restart), the five clue tiles reserved and forced at their cells in their rotations, with
a one-cell look-ahead to a pinned neighbour. 64 restarts; 35 get past the two clues of
row 2. Kept: every prefix of the first 16 restarts, and the prefixes from depth 48 on of
the other 48. In all 335 prefixes: 31 at depths 16 and 32, 304 at depths 48 to 192 on 35
paths. Fillings keep the clues still to come pinned in place.

[measured]

| depth | prefixes | proven dead (cap 300,000) | arc consistency refutes | A, all tiles reusable: filled / none exists / undecided | B, unplaced tiles reusable: filled / none exists / undecided |
|---|---|---|---|---|---|
| 16 | 16 | 10 | 12 | 11 / **4** / 1 | 2 / 10 / 4 |
| 32 | 15 | 13 | 13 | 5 / **9** / 1 | 0 / 13 / 2 |
| 48 | 35 | 0 | 0 | 34 / 0 / 1 | 9 / 0 / 26 |
| 64 | 35 | 0 | 0 | 34 / 0 / 1 | 6 / 0 / 29 |
| 80 | 35 | 0 | 0 | 33 / 0 / 2 | 1 / 0 / 34 |
| 96 | 35 | 5 | 1 | 34 / 0 / 1 | 0 / 1 / 34 |
| 112 | 35 | 35 | 29 | 31 / 0 / 4 | 0 / 29 / 6 |
| 128 | 35 | 35 | 33 | 34 / **1** / 0 | 0 / 33 / 2 |
| 144 | 32 | 32 | 25 | 32 / 0 / 0 | 0 / 27 / 5 |
| 160 | 32 | 32 | 31 | 31 / 0 / 1 | 0 / 32 / 0 |
| 176 | 29 | 29 | 29 | 29 / 0 / 0 | 0 / 29 / 0 |
| 192 | 1 | 1 | 1 | 0 / 0 / 1 | 0 / 1 / 0 |

Three things change against the clue-free tree.

1. **Column A is no longer all "filled".** 14 clue-pinned prefixes (4, 9 and 1) have no
   filling even when every one of the 256 tiles may be reused. By the memo's own D3 the
   converse is what protects a prefix, so here a boundary-type relaxation does refute.
   The memo's item 3 is a statement about the tree without clues.
2. **The refutations are local.** Of the 13 at depths 16 and 32, arc consistency run
   with all 256 tiles allowed in every cell refutes 12; at depth 32, 3 of the 9 fail
   already because no tile at all has the pair of adjacent colours that a cell next to a
   clue needs. One prefix at depth 16 passes arc consistency and still has no filling
   with reuse (`r5b_part1_first_run.out`): a window larger than two cells, still next to
   a clue. This is the clue-window effect the thesis plan already records under O3. It
   is cell-by-cell matching seen one or two cells ahead, not a structure of the set.
3. **Away from the clues the memo's picture holds.** From depth 48 on, 292 of 304
   prefixes have a filling with reuse, 11 are undecided at the cap and 1 has none. Arc
   consistency refutes nothing from depth 48 to 80 and almost everything from 112, which
   is earlier than in the clue-free tree (2 of 16 at depth 112 there, 29 of 35 here)
   because of the clue in row 8.

Tile-count certificate with the clues pinned (section 4), 400 fillings each: path 0 at
depth 16 and paths 1, 6, 15 at depths 48, 96, 144. Four of four: Z^256 / D =
Z^7 + (Z/2)^16, fully named, u - a(A_0) in D. Only four, because a filling with the two
clues of row 13 pinned costs about 150,000 nodes with my filler (two minutes per prefix).

The aggregated-law tests on these prefixes are in section 6.

## 8. The non-abelian tile group (`r7_nonabelian_quotients.py`)

The group: G(T) = < h_c, v_c for the 23 colours | h_b v_r h_t^-1 v_l^-1 for every tile
(t, r, b, l) of T in every rotation >. A region tileable by T with reuse has a boundary
word that is trivial in G(T). I did not compute G(T). I enumerated **all** homomorphisms
of G(T) into eight small groups, by backtracking with unit propagation on the relations;
each one is a boundary invariant, and it is non-abelian information when its image is.
Control: a tile set built from a planted map to S3 gives the planted map back.

**The 256 official tiles** [measured, exhaustive for each group]:

| group | order | homomorphisms | with non-abelian image |
|---|---|---|---|
| S3 | 6 | 36 | 6 |
| D4 | 8 | 160 | 16 |
| Q8 | 8 | 128 | 48 |
| D5 | 10 | 80 | 20 |
| A4 | 12 | 96 | 0 |
| D6 | 12 | 288 | 48 |
| S4 | 24 | 408 | 72 |
| Heisenberg group mod 3 | 27 | 297 | 0 |

So the tile group of the official set is **not abelian**, which the memo neither says nor
denies. But every one of the 210 homomorphisms with non-abelian image is constant on
(horizontal or vertical) x (grey, frame colour, interior colour): it never tells two
interior colours apart, nor two frame colours. These quotients belong to the three tile
types, like the period-2 invariant of section 2.4. (A weighting whose sign alternates
with the cell is a map to a dihedral group, not to an abelian one, so the dihedral
quotients are probably that same invariant reduced; I did not check this.) Any set of
corner, edge and interior tiles has them.
Control: on all 184 prefixes, every homomorphism of every one of the eight groups sends
the region word to the identity, as column A requires. This also checks the orientation
of my words.

**The unplaced tiles** (the memo's open item: 66 of 144 prefixes undecided in column B).
My filler, three attempts of 200,000 nodes, decides more: of 184 prefixes, 98 have a
filling with reuse of the unplaced tiles only, 54 have none, 32 are undecided (1 at depth
64, 3 at 96, 8 at 112, 13 at 128, 7 at 144). For the 86 not filled I enumerated all
homomorphisms of the unplaced tiles' group into the six groups of order up to 12, and
ran the abelian test in its integer form (is the abelianised region word in the lattice
spanned by the unplaced tiles).

| | prefixes | abelian integer test refutes | any of S3, D4, Q8, D5, A4, D6 refutes |
|---|---|---|---|
| column B undecided | 32 | 0 | 0 |
| column B "none exists" (the region is known to be untileable even with reuse) | 54 | **0** | **0** |
| column B filled (the test must pass) | 98 | 0 | not run |

Read the second row before the first. On 54 regions that are known to have no filling,
these invariants detect nothing. The reason is visible in the counts: for each of the 86
prefixes and each of the six groups, the unplaced tiles' group has exactly as many
homomorphisms as the full set's (36, 160, 128, 80, 96, 288). Sixty-four leftover tiles
already force every small quotient down to the tile-type quotients. So through groups of
order up to 12 the non-abelian group of the unplaced tiles carries type information
only, and "0 of 32" is a null with no power.

Ruling on the memo's residue ("the non-abelian invariants of the unplaced tiles at
depths below 144"): **still open, and unlikely to be reachable through small finite
quotients.** Thurston's tribone example (p. 764) is the standard warning that a quotient
can be blind where the group is not. What would settle it is in section 12.

## 9. Structure the memo may not have looked for (`r3_structure.py`)

All [measured] on the official tiles.

**Counts.** Grey 64 sides; each frame colour 24; interior colours 48 (five of them) or 50
(twelve). Every count is even, as a board requires. No frame colour on an interior tile,
no interior colour on a corner.

**Joints.** The plan speaks of 60 frame-adjacent interior edges. The count is 60 joints
between rim tiles, 56 joints between rim and interior (one per edge tile; I call them the
seam), and 364 interior joints: 480 in all.

**The seam.** The 56 inward sides of the edge tiles use all 17 interior colours, between
1 and 6 times each. Against a uniform draw of 56 of the 420 interior joints: chi-square
10.8, 73% of 5,000 draws are at least as uneven. No restricted seam palette. The only
exact constraint linking rim and interior is supply (each seam colour needs an interior
tile to face it), which arc consistency contains.

**The rim as a directed graph.** Vertices: the 5 frame colours. Arcs: the 60 border
tiles, from the colour a clockwise walk enters by to the colour it leaves by. In-degree
and out-degree are 12 at every vertex: balanced, as a closed rim requires. This Eulerian
condition is exactly the four rim-flow invariants of section 4, and it is the only exact
identity of the rim: the certificate there shows there is nothing else at the level of
tile counts.

**Tiles as 4-cycles and as diagonals on the 17 interior colours**, against 200 painted
boards with the official counts:

| statistic | official | painted: min, mean, max | share of boards at or beyond |
|---|---|---|---|
| adjacent colour pairs absent (of 153) | 2 | 0, 1.5, 6 | 0.49 |
| opposite colour pairs absent | 8 | 4, 11.8, 19 | 0.14 |
| adjacent sides of equal colour | 47 | 29, 44.1, 63 | 0.32 |
| opposite sides of equal colour | 23 | 8, 22.3, 34 | 0.49 |
| tiles with four distinct colours | 132 | 120, 136.2, 157 | 0.31 |
| tiles with exactly one repeated colour | 61 | 36, 55.7, 72 | 0.26 |
| tiles with two pairs | 0 | 0, 1.6, 6 | 0.17 |
| tiles with a triple | 3 | 0, 2.4, 7 | 0.42 |

Nothing stands out. Degree parities in both graphs are automatic (each side has two
neighbours; each colour count is even).

**Parity per colour with the checkerboard.** Every joint has one black and one white
cell, so for each colour the tiles on black cells carry half of its sides. Mod 2 this is
22 equations on the 256 unknowns "tile i sits on black": rank 21, consistent. At most 21
bits out of 256, and each equation is the mod 2 reading of a count that test 1 contains
and that section 3 shows to be in the named family (the 30 relations at the resolution
tile x cell colour, with no torsion beyond them).

**Clues.** Cells: (2,2) and (13,13) black, (2,13), (13,2) and (8,7) white. The clue
tiles and their rotations shift the flux the other 251 tiles must cancel by at most 2
per colour (vertical: colours 8, 9, 12, 15, 16, 20, 21, 22; horizontal: 9, 11, 12, 13,
17, 18, 20, 21). Removing the five clue tiles leaves Z^46 / L unchanged (section 2.2).
The count relations of section 3 do not see position, so pinning cannot break them; it
only fixes five terms. No identity is created or broken at the root, and none can be: a
solution with the clues in place exists by construction of the puzzle, so every exact
necessary condition holds there. What the clues do create is local: section 7.

## 10. The three sources the memo did not open

| source | status | what it says that bears on the memo |
|---|---|---|
| Thurston, "Conway's tiling groups", Amer. Math. Monthly 97 (1990) 757 to 773 | **Opened**, pages 757 to 764 read (scan at <https://www.cimat.mx/ciencia_para_jovenes/pensamiento_matematico/thurston.pdf>) | p. 757: the condition is necessary ("can be tiled, only if"). p. 761: "coloring arguments are the abelian part of the group theory"; "coloring arguments in general cannot give as much information"; the algebraic condition "is not sufficient to guarantee a tiling by lozenges". p. 764: for tribones a quotient of the tile group "was not sufficient to detect" the obstruction and the full group is needed. Supports the memo's hierarchy. The last point is a warning for my section 8: small quotients can miss what the group sees |
| Pak, "Tile invariants: new horizons", Theoret. Comput. Sci. 302 (2003) 303 to 331 | **Opened**, preprint of 20 December 2000, pages 1 to 8 read (<https://www.math.ucla.edu/~pak/papers/tilesurvey.pdf>; the memo's 404 was another address) | Quoted in sections 3 and 4. Also: Theorem 2.3, the invariants that survive signed tilings are exactly those from extended colourings; Theorem 3.1, the rank of the group of invariants is undecidable in general and NP-hard for bounded regions; p. 7, the certificate by pairs of tilings that my sections 3 and 4 use. This is the source the memo needed most |
| Reid, "Tile homotopy groups", Enseign. Math. 49 (2003) 123 to 155 | **Not opened** (the journal archive serves a verification page to automated readers). Opened instead: the record of the article in the volume's table of contents, and the author's own abstract of a talk of that title (MIT combinatorics seminar, 4 April 2001, <https://math.mit.edu/combin/2001sp.html>) | From the abstract, as paraphrased by the fetch tool and not verbatim: the Conway and Lagarias method assigns a finitely presented group to a set of tiles; a strategy is given that succeeds where the method had failed; examples exist where it gives restrictions that generalised checkerboard colourings cannot; and there are restrictions the tile homotopy group cannot detect. Nothing in the memo is attributed to Reid, and nothing here rests on him |

Attribution check. The memo attributes nothing specific to the three. Its use of Conway
and Lagarias (colouring arguments also refute signed tilings; boundary invariants are
stronger) agrees with Thurston p. 761. The sentence "Nothing below rests on them" is the
problem: D3 and section 4 draw a boundary (count versus boundary) that Pak's survey
exists to erase.

## 11. Corrected statements

Verdict, as it should read: "No exact constraint derivable from the tile set alone was
found beyond sums of cell-by-cell matching conditions. Closed by computation: edge
weightings of period 1 to 4 (group Z^2 + Z/2, plus one geometric invariant at even
period); relations between tile counts up to (tile, rotation, cell colour) (all named,
all sums of matching conditions); boundary words and tile-count invariants on the
sampled clue-free prefixes. Not priced by any measurement: the aggregated-law test has
no power at the depths sampled. Not closed: position-resolved integer relaxations and
the non-abelian group of the unplaced tiles."

Item 2: "All edge weightings of period 1 that vanish on every tile form Z^2 + Z/2. None
distinguishes one interior colour from another."

Item 3: "In the clue-free tree, every one of 184 sampled prefixes has a filling with
reuse of all tiles, so no boundary word refutes them. In the clue-pinned tree this fails
near the clues, for local reasons."

Item 4: "The aggregated laws as one linear program refute 0 of 184 prefixes, 77 of which
are proven dead and 52 of which arc consistency refutes."

D3: as in ruling 12.

## 12. Left open, worth a study

Ordered by what they would settle, with my guess of the cost. None is a likely source
of bits; each closes a sentence that is currently open.

1. **Column B with a better filler, then the group itself on the controls.** At depths
   16 to 96, where a refutation would be worth something, my filler already finds a
   filling with reuse of the unplaced tiles for 92 of 96 prefixes (the memo had 60), so
   the memo's residue is nearly gone there: a filling ends the question for that prefix,
   for every invariant, abelian or not. Left: 4 prefixes at depths 64 and 96, and 28 at
   depths 112 to 144 where subtrees are small. To close it with power: run a real
   computer-algebra system (coset enumeration, nilpotent quotients, a rewriting system)
   on the unplaced tiles' group for the 54 prefixes where no filling exists, and ask
   whether the region word is trivial. If the full group is as blind as its small
   quotients on those 54, the item is closed. Half a study, needs GAP or equivalent.
2. **A control with power for the upper rungs.** The 0/1 form of test 1 and the
   cell-level relaxation both refute real prefixes that the memo's linear program does
   not. Nobody knows how early they start: on real prefixes truth is unavailable above
   depth 112. The memo's own control design (painted 16 x 16 boards, a census-preserving
   exchange of two tiles) run with the cell-level relaxation and with the 0/1 coarse
   program at 5 to 10 rows left would say where each rung goes blind. One to two
   core-hours. My expectation: blind beyond 6 or 7 rows.
3. **Is test 1's equation system complete at its own resolution?** Section 3 shows the
   flux family is complete for counts by (tile, rotation, cell colour). The same
   measurement at the resolution of test 1 (tile, rotation, row, column parity) would say
   whether its equations are all the linear relations there are, so that only the bounds
   remain. About 13,000 coordinates at depth 144; needs that many fillings and a sparse
   rank. One study.
4. **The clue windows as a table.** One clue-pinned prefix at depth 16 has no filling
   with reuse and passes arc consistency. The exact local constraint each clue puts on
   the cells around it (which frontier colours admit a filling of the 3 x 3 window) is a
   finite table. It is cell-by-cell matching, an engine with good propagation has most of
   it, and it is cheap to compute. One core-hour; a footnote unless the row-scan profile
   of 2^134.4 turns out not to include it.
5. **Version B of the tile-count certificate** (fillings with the unplaced tiles only),
   and the certificate on more than four clue-pinned prefixes. A better filler for the
   two clues of row 13 is the only obstacle.
6. **Reid's paper**, by someone with library access. I read only its abstract-level
   description.

Closed here and not worth returning to: periods 1 to 4 of the edge weightings, the
extra invariant of the covers, automorphisms, the seam, the rim digraph, colour-pair
graphs, checkerboard parities, clue identities at the root.

## Files in `gap-5-review/`

All paths under `notes/gap-5-review/`. `README.txt` there gives the commands.

- `rcommon.py`, `rfill.py`: loader, exact integer Smith form, fillers (reuse or each
  once, pinned cells).
- `r1_lattice.py`, `.out`: section 2.1 to 2.3. `r1b_cover_invariant.py`, `.out`, and
  `r1b_k3.out` (period 3): 2.4.
- `r2b_depth112_flags.py`, `.out`: dead, arc consistency and cell-level relaxation per
  prefix at depths 112 to 144 (section 6).
- `r2a_prefix_death.py`, `.out`: section 5. Writes `prefixes_noclue.json` (the memo's
  184 prefixes, regenerated with the memo's generator and seeds).
- `r3_structure.py`, `.out`; `r3b_automorphisms.py`, `.out`: section 9 and ruling 4.
- `r4_tile_count_invariants.py`; `r4_A_path0.out` (first version of the report line),
  `r4_A.out` (37 prefixes; stopped by me after path 3, depth 32, to free the core).
- `r5_clue_prefixes.py`, `.out`; `r5b_clue_more.py`, `r5b_part1_first_run.out` (locality
  table; that run was stopped in its part 3), `r5b_parts13.out` (certificate on three
  deep clue paths; its part 1 uses a later fill order that decides fewer cases),
  `r5b_part2.out` (48 more restarts): section 7. `prefixes_clue.json`,
  `prefixes_clue_more.json`.
- `r6_integer_and_power.py`; `r6_coarse.out`, `r6_clue.out`, `r6_cell.out`,
  `r6_cell_depth112.out`, `r6_ip_depth192.out`, `r6_ip_depth176.out`, `r6_modp.out`
  (`r6_modp_first_run_subset.out` is a first version that sampled columns and decided
  little): section 6.
- `r7_nonabelian_quotients.py`; `r7_full_set.out` (eight groups, full set),
  `r7_nonabelian_quotients.out` (six groups, unplaced tiles, all prefixes),
  `r7_first_run_static_order.out` (first, slow version; same counts): section 8.
- `r8_count_groups.py`; `r8_depth16_first_run.out` (4 and 8 relations unexplained),
  `r8_depth16.out`, `r8_depth96.out`: section 3.
- `run_r6_r7.sh`, `batch.log`: the batch that ran r6 and the first r7.

Author's scripts rerun: `01_lattice_and_automorphisms.py` and
`02_decomposition_tensor_classes.py`, output identical to the committed `.out` files.
Scripts 03 and 04 were not rerun as such: 04's prefix generator is imported by `r2a`, and
03's linear program is rebuilt in `r6`.

Two notes on reproduction. `rfill.py` changed its cell order around pinned cells after
`r5` had run (a row holding a clue is now filled together with the row above it); `r5`
rerun today would decide fewer of the depth-16 cases in column A, as part 1 of
`r5b_parts13.out` shows. A "none exists" is a proof under either order. And my imports of
the author's generator wrote a `__pycache__` folder inside `gap-5/`; I removed it,
the author's files are untouched.

Not verified by me: the memo's control table for test 1 (painted boards), its tensor
and palette-split numbers beyond the rerun, and the node counts of its column B.
