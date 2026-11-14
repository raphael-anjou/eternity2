# Gap 6a, row 15: the degree at which linearisation determines a toy board

> **Status (2026-11-13):** not independently checked.
> **Re-check's rulings:** none, no second analysis report exists for this memo.
> **Read with:** `notes/gap-6a-reformulation-algebra-recheck.md` (row 15 and the cost model this memo reports against); `notes/gap-6a-reformulation-algebra.md`.

Draft, 2026-11-13. Everything here is on generated toy boards (3 x 3, 4 x 4, 5 x 5). None of it is Eternity II progress. Statements are marked measured, derived or guessed.

## 1. Reading against the kill line

The kill line, fixed before any run: the claim "the needed degree grows with the board side" is refuted only if degree 2 determines the solution on most boards at both 4 x 4 and 5 x 5; the row is closed if the needed degree at 5 x 5 exceeds that at 4 x 4, or exceeds 3 at 5 x 5.

- **Not refuted.** Degree 2 determines 0 of 10 boards at 4 x 4 and 0 of 5 boards at 5 x 5, over GF(2) and over GF(2^31 - 1), in both encodings [measured].
- **Not closed either.** At 4 x 4 the needed degree is 3 on 6 or 7 boards of 10 and 4 on the others (primary encoding). At 5 x 5 it is at least 3 on all 5 boards, and degree 3 did not fit in 8 GB, so whether it is 3, 4 or more is not measured. Neither closing condition is established. I do not extrapolate one.
- What is measured is growth from side 3 to side 4: degree 2 on 10 of 10 boards at 3 x 3, degree 3 or 4 at 4 x 4.
- Propagation level: degree 2 at every size, on every board. Degree 1 forces 1 or 2 variables at 3 x 3 and none at 4 x 4 and 5 x 5; degree 2 forces a median of 18 of 128 at 4 x 4 and 60 of 461 at 5 x 5.

## 2. What was computed

**Boards.** The painter of `gap-6b/toy_reformulations.py` (copied into `boards.py`, same random call order): cf colours on rim-to-rim edges, ci colours on the other inner edges, grey on the rim, no repeated tile, no rotation-symmetric tile. This is not the repo's Rust generator; it is the small framed painter the plan allowed after checking. Colour counts were chosen by one rule, before any linear algebra: the fewest colours at which the median number of pinned solutions over seeds 0 to 39 (0 to 29 at 5 x 5) is 1, ties to the smaller cf. That gives (cf, ci) = (3, 3) at 3 x 3, (3, 4) at 4 x 4, (3, 6) at 5 x 5 (`scan_*.out`). Boards kept: the first seeds with exactly one pinned solution, 10 at 3 x 3 (seeds 0, 5, 8, 9, 11, 12, 13, 14, 17, 19), 10 at 4 x 4 (0, 1, 3, 5, 7, 9, 12, 13, 15, 17), 5 at 5 x 5 (0, 2, 5, 6, 8).

**Pin.** The corner tile with the smallest index in the shuffled tile list is fixed in cell (0, 0), in its only rotation with grey to the north and west. Whole-board rotations therefore count once. The pin is a substitution: the variables of that cell and of that tile are removed, and so are the placements of the two neighbouring cells whose facing colour differs. Nothing else is propagated.

**Variables.** One per rim-consistent placement (grey exactly on the rim sides): 36, 144 and 484 before the pin, as in the review; 23 to 25, 127 to 131 and 460 to 463 after it.

**Equations.** One linear equation per free cell and per free tile (sum of its variables equals 1); a forbidden product for every pair of placements in adjacent cells with different facing colours; multilinear monomials (x^2 = x).
- *Minimal* encoding (primary, the one in the question): exactly that.
- *Enriched* encoding (secondary): forbidden products are added for two placements in the same cell and for two placements of the same tile. These follow from the system but not at degree 2, so this is a stronger linearisation.

**Linearisation.** Plain XL at degree bound D: every equation times every monomial, total degree at most D, then linear algebra. No derived equation is fed back (this is the static, Nullstellensatz-style procedure; the iterated one, polynomial calculus, is stronger and was not run). A monomial containing a forbidden pair is itself a row, so those columns are dropped and counted in the rank. "Forced" means x - v lies in the row space. "Determined" means every variable is forced. Every forced value was compared with the brute-force solutions: no unsound forcing in any run.

**Code.** `xl.cpp`, three eliminations that must agree: dense reduced echelon form of the rows; dense reduced echelon form of the columns of degree 2 and more, then the orthogonal complement; a sparse echelon form (the one used for the table). Two builds, modulus 2 and modulus 2^31 - 1.

## 3. Checks and controls [all measured]

- **3 x 3 against an independent reference.** `ref_xl.py` (pure Python, explicit product rows, no column removal) and the three C++ eliminations give the same rank and the same forced set on 8 boards, 2 encodings, 2 fields, degrees 1 to 3: 96 of 96 comparisons (`raw/check_3x3.out`).
- **3 x 3 against brute force.** In the enriched encoding at degree 8 (one placement per free cell) the kernel is exactly the span of the solutions [derived: every lower monomial is the sum of its full extensions]. Measured on the same 8 boards, which have 1 to 3 solutions: nullity equals the brute-force solution count and the forced set equals the set of variables constant on the solutions, 16 of 16.
- **Negative control.** 4 x 4 seed 2 has two pinned solutions, which agree on 112 of 126 variables. Never determined; at degree 3 the forced count is 105 (minimal, GF(2)) or exactly 112 (the other three), never a variable on which the solutions differ.
- **Positive control.** 4 x 4 seed 0 with every cell pinned except cells 5, 10 and 15: degree 1 determines the 3 remaining variables in all four settings.

## 4. Results

Boards determined, by degree (minimal encoding first, enriched in brackets):

| size | field | degree 1 | degree 2 | degree 3 | degree 4 |
|---|---|---|---|---|---|
| 3 x 3 | GF(2) | 0 of 10 (0) | 10 of 10 (10) | 10 of 10 (10) | not run |
| 3 x 3 | GF(2^31 - 1) | 0 of 10 (0) | 10 of 10 (10) | 10 of 10 (10) | not run |
| 4 x 4 | GF(2) | 0 of 10 (0) | 0 of 10 (0) | 6 of 10 (9) | the 4 left: 4 of 4 (the 1 left: 1 of 1) |
| 4 x 4 | GF(2^31 - 1) | 0 of 10 (0) | 0 of 10 (0) | 7 of 10 (9) | the 3 left: 3 of 3 (the 1 left: 1 of 1) |
| 5 x 5 | GF(2) | 0 of 5 (0) | 0 of 5 (0) | did not fit | not run |
| 5 x 5 | GF(2^31 - 1) | 0 of 5 (0) | 0 of 5 (0) | did not fit | not run |

Degree 4 at 4 x 4 was run only on the four boards (seeds 1, 3, 9, 15) that some setting left open at degree 3; it determines all of them in all four settings. So at 4 x 4 the needed degree is 3 or 4, depending on the board, the field and the encoding. The field matters at 4 x 4, degree 3, minimal encoding: seed 1 is determined over the prime and not over GF(2) (116 of 128 forced).

Variables forced at degree 2 (minimal, either field): 3 to 31 of about 128 at 4 x 4 (median 17 or 18); 40 to 112 of about 461 at 5 x 5 (median 60). The forced share at degree 2 does not obviously fall from 4 x 4 to 5 x 5 (13% at both sizes in the median); this says nothing about determination.

The per-run table for 4 x 4 and 5 x 5 in the minimal encoding is Appendix A. Every run, including 3 x 3 and the enriched encoding, is in `gap-6a-linearisation/table_all.md`; medians per setting are in `summary.md`.

## 5. Cost, against the review's model

The review prices elimination at exponent 2 in the number of monomials. Measured with the sparse elimination, one core (minimal encoding, GF(2), median or seed 0):

| case | monomials | not zeroed | rows | rank | nullity | stored nonzeros | seconds |
|---|---|---|---|---|---|---|---|
| 4 x 4, degree 2 | 8,257 | 6,637 | 3,870 | 4,720 | 3,537 | 61,495 | 0.01 |
| 4 x 4, degree 3 | 349,633 | 188,342 | 199,012 | 291,795 | 57,838 | 3.3 million | 1.5 |
| 4 x 4, degree 4 (seed 1) | 11.0 million | 3.56 million | 5.76 million | 10.4 million | 617,856 | 98 million | 58 |
| 5 x 5, degree 2 | 106,031 | 89,103 | 22,128 | 36,625 | 69,406 | 1.6 million | 0.31 |
| 5 x 5, degree 3 | 16.2 million | 9.85 million (6.58 million enriched) | 4.28 million (3.44 million) | not reached | not reached | over 1.07 billion at abort | abort |

- Where the system ends up determined (4 x 4, degrees 3 and 4), stored nonzeros stay near 9 per monomial and time grows about linearly with the monomial count (a factor 33 in monomials from degree 3 to 4, a factor 25 to 40 in seconds). That is far below exponent 2 [measured, on toys, with a sparse method the model did not assume].
- At 5 x 5, degree 3, the same method stores over 66 nonzeros per monomial before it is a third of the way through, and stops on memory. No exponent can be read from a run that did not finish.
- Over the prime the same runs cost 1.3 to 3 times the GF(2) time (up to 165 s at 4 x 4, degree 4).
- The rank is far from the monomial count at degree 2 (nullity 43% of the monomials at 4 x 4, 65% at 5 x 5) and closes at the degree that determines (nullity 5% at 4 x 4, degree 4).

## 6. What did not fit

5 x 5 at degree 3, exactly:

- Full system, enriched, GF(2), seed 0: stopped by the memory guard after 482 s of CPU, at rank 2,347,572 with 1.07 billion stored nonzeros, 6.58 million nonzero monomials, 3.44 million rows. No rank, no forced set.
- Locality-ordered subset (mode `local`: rows whose multiplier is a pair of placements at Manhattan distance at most k, plus all rows with a smaller multiplier; a subset of the XL rows, so a "determined" would have been sound). All 5 boards, enriched, GF(2): pairs in adjacent cells (k = 1) completed in 3 to 6 s and raise the rank of the linear consequences only from 217 to 283 (degree 2) to 240 to 293, out of 460 to 463 needed. The next class (k = 2) hit the memory guard after 108 to 149 s on every board. Seed 0 also in the minimal encoding (161 s) and over the prime (58 s): same stop.
- For comparison, at 4 x 4 the same subset with k = 2 (of 6) already determines seed 0 in 0.6 to 2.2 s.
- The guard is checked between batches against the allocated arena; the allocation reported at the stop was 8.65 to 8.69 GB, so these runs briefly reserved more than the 8 GB allowed.
- Cost scaling available: 5 x 5 degree 2 took 0.3 s for 106,000 monomials; degree 3 has 153 times more monomials and was not finished within 1,000 times the memory. A better pivot order, an out-of-core or a black-box (Wiedemann) method might fit it; I did not try them.

Also not done: degree 4 on the 4 x 4 boards already determined at degree 3 (not needed); 5 x 5 at degree 4; polynomial calculus (iterated) degree; the repo's Rust generator; boards with more than the near-critical colour count.

## 7. Caveats

- 10 and 5 boards, one colour count per size, one painter. "6 or 7 of 10" is a count, not a rate.
- The needed degree depends on the encoding and the field at 4 x 4. The claim in row 15 does not say which; the table gives all four.
- Determination by linear algebra over a field is weaker than Sherali-Adams at the same degree (no sign conditions), and weaker than polynomial calculus. A result here bounds XL only.
- The 3 x 3 to 4 x 4 growth could be a small-size effect (a 3 x 3 board has one interior cell). It is one step of growth, not a law.
- Row 15's guess is therefore still a guess: supported at sides 3 and 4, untested at 5.

## 8. Files

`gap-6a-linearisation/`: `boards.py` (painter, solver, colour scan), `xlsys.py` (board to system), `xl.cpp` (XL, built as `xl_2` and `xl_p` with `clang++ -O3 -std=c++17 -DPRIME=2u` and `-DPRIME=2147483647u`), `ref_xl.py` (reference), `run.py` (driver: `check`, `controls`, `main`), `make_table.py`, `results.jsonl` (one record per run), `table_all.md`, `table_memo.md`, `summary.md`, `scan_*.out`, `systems/` (the systems as fed to `xl`), `raw/` (console output of every batch, including the aborted ones).

## Appendix A: every run at 4 x 4 and 5 x 5, minimal encoding

"Monomials" is the number of multilinear monomials of degree at most D in the variables left after the pin (the figure the cost model uses). "Rank" is the rank of the whole linearised system, zeroed monomials included; nullity is monomials minus rank. Seconds are CPU seconds of the sparse elimination on one core.

| board | size | encoding | field | degree | monomials | monomials not zeroed | rank | nullity | variables forced | determined | seconds |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 4x4 seed 0 | 4 | minimal | GF(2) | 1 | 129 | 129 | 27 | 102 | 0/128 | no | 0.00 |
| 4x4 seed 0 | 4 | minimal | GF(2) | 2 | 8257 | 6637 | 4720 | 3537 | 14/128 | no | 0.01 |
| 4x4 seed 0 | 4 | minimal | GF(2) | 3 | 349633 | 188342 | 291795 | 57838 | 128/128 | yes | 1.47 |
| 4x4 seed 0 | 4 | minimal | GF(2^31-1) | 1 | 129 | 129 | 27 | 102 | 0/128 | no | 0.00 |
| 4x4 seed 0 | 4 | minimal | GF(2^31-1) | 2 | 8257 | 6637 | 4750 | 3507 | 14/128 | no | 0.01 |
| 4x4 seed 0 | 4 | minimal | GF(2^31-1) | 3 | 349633 | 188342 | 294111 | 55522 | 128/128 | yes | 2.03 |
| 4x4 seed 1 | 4 | minimal | GF(2) | 1 | 129 | 129 | 27 | 102 | 0/128 | no | 0.00 |
| 4x4 seed 1 | 4 | minimal | GF(2) | 2 | 8257 | 6684 | 4672 | 3585 | 31/128 | no | 0.01 |
| 4x4 seed 1 | 4 | minimal | GF(2) | 3 | 349633 | 192130 | 289056 | 60577 | 116/128 | no | 1.47 |
| 4x4 seed 1 | 4 | minimal | GF(2) | 4 | 11017633 | 3557001 | 10399777 | 617856 | 128/128 | yes | 58.41 |
| 4x4 seed 1 | 4 | minimal | GF(2^31-1) | 1 | 129 | 129 | 27 | 102 | 0/128 | no | 0.00 |
| 4x4 seed 1 | 4 | minimal | GF(2^31-1) | 2 | 8257 | 6684 | 4702 | 3555 | 31/128 | no | 0.01 |
| 4x4 seed 1 | 4 | minimal | GF(2^31-1) | 3 | 349633 | 192130 | 291415 | 58218 | 128/128 | yes | 1.91 |
| 4x4 seed 1 | 4 | minimal | GF(2^31-1) | 4 | 11017633 | 3557001 | 10467001 | 550632 | 128/128 | yes | 164.68 |
| 4x4 seed 3 | 4 | minimal | GF(2) | 1 | 129 | 129 | 27 | 102 | 0/128 | no | 0.00 |
| 4x4 seed 3 | 4 | minimal | GF(2) | 2 | 8257 | 6734 | 4620 | 3637 | 24/128 | no | 0.01 |
| 4x4 seed 3 | 4 | minimal | GF(2) | 3 | 349633 | 195686 | 287219 | 62414 | 48/128 | no | 1.67 |
| 4x4 seed 3 | 4 | minimal | GF(2) | 4 | 11017633 | 3665234 | 10374378 | 643255 | 128/128 | yes | 60.86 |
| 4x4 seed 3 | 4 | minimal | GF(2^31-1) | 1 | 129 | 129 | 27 | 102 | 0/128 | no | 0.00 |
| 4x4 seed 3 | 4 | minimal | GF(2^31-1) | 2 | 8257 | 6734 | 4650 | 3607 | 24/128 | no | 0.01 |
| 4x4 seed 3 | 4 | minimal | GF(2^31-1) | 3 | 349633 | 195686 | 289637 | 59996 | 120/128 | no | 2.12 |
| 4x4 seed 3 | 4 | minimal | GF(2^31-1) | 4 | 11017633 | 3665234 | 10443997 | 573636 | 128/128 | yes | 132.25 |
| 4x4 seed 5 | 4 | minimal | GF(2) | 1 | 130 | 130 | 27 | 103 | 0/129 | no | 0.00 |
| 4x4 seed 5 | 4 | minimal | GF(2) | 2 | 8386 | 6757 | 4752 | 3634 | 26/129 | no | 0.01 |
| 4x4 seed 5 | 4 | minimal | GF(2) | 3 | 357890 | 194149 | 296521 | 61369 | 129/129 | yes | 1.31 |
| 4x4 seed 5 | 4 | minimal | GF(2^31-1) | 1 | 130 | 130 | 27 | 103 | 0/129 | no | 0.00 |
| 4x4 seed 5 | 4 | minimal | GF(2^31-1) | 2 | 8386 | 6757 | 4782 | 3604 | 26/129 | no | 0.01 |
| 4x4 seed 5 | 4 | minimal | GF(2^31-1) | 3 | 357890 | 194149 | 298897 | 58993 | 129/129 | yes | 1.72 |
| 4x4 seed 7 | 4 | minimal | GF(2) | 1 | 128 | 128 | 27 | 101 | 0/127 | no | 0.00 |
| 4x4 seed 7 | 4 | minimal | GF(2) | 2 | 8129 | 6550 | 4649 | 3480 | 25/127 | no | 0.01 |
| 4x4 seed 7 | 4 | minimal | GF(2) | 3 | 341504 | 185327 | 284311 | 57193 | 127/127 | yes | 1.25 |
| 4x4 seed 7 | 4 | minimal | GF(2^31-1) | 1 | 128 | 128 | 27 | 101 | 0/127 | no | 0.00 |
| 4x4 seed 7 | 4 | minimal | GF(2^31-1) | 2 | 8129 | 6550 | 4679 | 3450 | 25/127 | no | 0.01 |
| 4x4 seed 7 | 4 | minimal | GF(2^31-1) | 3 | 341504 | 185327 | 286618 | 54886 | 127/127 | yes | 1.58 |
| 4x4 seed 9 | 4 | minimal | GF(2) | 1 | 132 | 132 | 27 | 105 | 0/131 | no | 0.00 |
| 4x4 seed 9 | 4 | minimal | GF(2) | 2 | 8647 | 7026 | 4802 | 3845 | 7/131 | no | 0.01 |
| 4x4 seed 9 | 4 | minimal | GF(2) | 3 | 374792 | 208376 | 307029 | 67763 | 64/131 | no | 1.89 |
| 4x4 seed 9 | 4 | minimal | GF(2) | 4 | 12091432 | 3995482 | 11368638 | 722794 | 131/131 | yes | 59.04 |
| 4x4 seed 9 | 4 | minimal | GF(2^31-1) | 1 | 132 | 132 | 27 | 105 | 0/131 | no | 0.00 |
| 4x4 seed 9 | 4 | minimal | GF(2^31-1) | 2 | 8647 | 7026 | 4832 | 3815 | 7/131 | no | 0.02 |
| 4x4 seed 9 | 4 | minimal | GF(2^31-1) | 3 | 374792 | 208376 | 309556 | 65236 | 82/131 | no | 2.55 |
| 4x4 seed 9 | 4 | minimal | GF(2^31-1) | 4 | 12091432 | 3995482 | 11441439 | 649993 | 131/131 | yes | 127.34 |
| 4x4 seed 12 | 4 | minimal | GF(2) | 1 | 129 | 129 | 27 | 102 | 0/128 | no | 0.00 |
| 4x4 seed 12 | 4 | minimal | GF(2) | 2 | 8257 | 6654 | 4701 | 3556 | 20/128 | no | 0.01 |
| 4x4 seed 12 | 4 | minimal | GF(2) | 3 | 349633 | 189752 | 290572 | 59061 | 128/128 | yes | 1.45 |
| 4x4 seed 12 | 4 | minimal | GF(2^31-1) | 1 | 129 | 129 | 27 | 102 | 0/128 | no | 0.00 |
| 4x4 seed 12 | 4 | minimal | GF(2^31-1) | 2 | 8257 | 6654 | 4731 | 3526 | 21/128 | no | 0.02 |
| 4x4 seed 12 | 4 | minimal | GF(2^31-1) | 3 | 349633 | 189752 | 292903 | 56730 | 128/128 | yes | 1.86 |
| 4x4 seed 13 | 4 | minimal | GF(2) | 1 | 129 | 129 | 27 | 102 | 0/128 | no | 0.00 |
| 4x4 seed 13 | 4 | minimal | GF(2) | 2 | 8257 | 6625 | 4731 | 3526 | 12/128 | no | 0.01 |
| 4x4 seed 13 | 4 | minimal | GF(2) | 3 | 349633 | 187734 | 292106 | 57527 | 128/128 | yes | 1.53 |
| 4x4 seed 13 | 4 | minimal | GF(2^31-1) | 1 | 129 | 129 | 27 | 102 | 0/128 | no | 0.00 |
| 4x4 seed 13 | 4 | minimal | GF(2^31-1) | 2 | 8257 | 6625 | 4761 | 3496 | 12/128 | no | 0.01 |
| 4x4 seed 13 | 4 | minimal | GF(2^31-1) | 3 | 349633 | 187734 | 294444 | 55189 | 128/128 | yes | 1.95 |
| 4x4 seed 15 | 4 | minimal | GF(2) | 1 | 132 | 132 | 27 | 105 | 0/131 | no | 0.00 |
| 4x4 seed 15 | 4 | minimal | GF(2) | 2 | 8647 | 7047 | 4781 | 3866 | 3/131 | no | 0.01 |
| 4x4 seed 15 | 4 | minimal | GF(2) | 3 | 374792 | 209918 | 306309 | 68483 | 21/131 | no | 2.43 |
| 4x4 seed 15 | 4 | minimal | GF(2) | 4 | 12091432 | 4044903 | 11357411 | 734021 | 131/131 | yes | 38.60 |
| 4x4 seed 15 | 4 | minimal | GF(2^31-1) | 1 | 132 | 132 | 27 | 105 | 0/131 | no | 0.00 |
| 4x4 seed 15 | 4 | minimal | GF(2^31-1) | 2 | 8647 | 7047 | 4811 | 3836 | 3/131 | no | 0.02 |
| 4x4 seed 15 | 4 | minimal | GF(2^31-1) | 3 | 374792 | 209918 | 308846 | 65946 | 107/131 | no | 3.27 |
| 4x4 seed 15 | 4 | minimal | GF(2^31-1) | 4 | 12091432 | 4044903 | 11431306 | 660126 | 131/131 | yes | 133.85 |
| 4x4 seed 17 | 4 | minimal | GF(2) | 1 | 130 | 130 | 27 | 103 | 0/129 | no | 0.00 |
| 4x4 seed 17 | 4 | minimal | GF(2) | 2 | 8386 | 6750 | 4763 | 3623 | 8/129 | no | 0.01 |
| 4x4 seed 17 | 4 | minimal | GF(2) | 3 | 357890 | 193406 | 297917 | 59973 | 129/129 | yes | 1.64 |
| 4x4 seed 17 | 4 | minimal | GF(2^31-1) | 1 | 130 | 130 | 27 | 103 | 0/129 | no | 0.00 |
| 4x4 seed 17 | 4 | minimal | GF(2^31-1) | 2 | 8386 | 6750 | 4793 | 3593 | 8/129 | no | 0.02 |
| 4x4 seed 17 | 4 | minimal | GF(2^31-1) | 3 | 357890 | 193406 | 300289 | 57601 | 129/129 | yes | 2.19 |
| 5x5 seed 0 | 5 | minimal | GF(2) | 1 | 461 | 461 | 45 | 416 | 0/460 | no | 0.00 |
| 5x5 seed 0 | 5 | minimal | GF(2) | 2 | 106031 | 89103 | 36625 | 69406 | 60/460 | no | 0.31 |
| 5x5 seed 0 | 5 | minimal | GF(2) | 3 | 16223051 | 9851105 | did not fit (memory limit) | n/a | n/a | not known | 160.61 |
| 5x5 seed 0 | 5 | minimal | GF(2^31-1) | 1 | 461 | 461 | 45 | 416 | 0/460 | no | 0.00 |
| 5x5 seed 0 | 5 | minimal | GF(2^31-1) | 2 | 106031 | 89103 | 36675 | 69356 | 60/460 | no | 0.34 |
| 5x5 seed 2 | 5 | minimal | GF(2) | 1 | 463 | 463 | 45 | 418 | 0/462 | no | 0.00 |
| 5x5 seed 2 | 5 | minimal | GF(2) | 2 | 106954 | 89608 | 37128 | 69826 | 40/462 | no | 0.31 |
| 5x5 seed 2 | 5 | minimal | GF(2^31-1) | 1 | 463 | 463 | 45 | 418 | 0/462 | no | 0.00 |
| 5x5 seed 2 | 5 | minimal | GF(2^31-1) | 2 | 106954 | 89608 | 37178 | 69776 | 40/462 | no | 0.34 |
| 5x5 seed 5 | 5 | minimal | GF(2) | 1 | 462 | 462 | 45 | 417 | 0/461 | no | 0.00 |
| 5x5 seed 5 | 5 | minimal | GF(2) | 2 | 106492 | 89003 | 37225 | 69267 | 112/461 | no | 0.31 |
| 5x5 seed 5 | 5 | minimal | GF(2^31-1) | 1 | 462 | 462 | 45 | 417 | 0/461 | no | 0.00 |
| 5x5 seed 5 | 5 | minimal | GF(2^31-1) | 2 | 106492 | 89003 | 37275 | 69217 | 112/461 | no | 0.35 |
| 5x5 seed 6 | 5 | minimal | GF(2) | 1 | 464 | 464 | 45 | 419 | 0/463 | no | 0.00 |
| 5x5 seed 6 | 5 | minimal | GF(2) | 2 | 107417 | 90568 | 36687 | 70730 | 48/463 | no | 0.34 |
| 5x5 seed 6 | 5 | minimal | GF(2^31-1) | 1 | 464 | 464 | 45 | 419 | 0/463 | no | 0.00 |
| 5x5 seed 6 | 5 | minimal | GF(2^31-1) | 2 | 107417 | 90568 | 36737 | 70680 | 48/463 | no | 0.37 |
| 5x5 seed 8 | 5 | minimal | GF(2) | 1 | 463 | 463 | 45 | 418 | 0/462 | no | 0.00 |
| 5x5 seed 8 | 5 | minimal | GF(2) | 2 | 106954 | 90066 | 36675 | 70279 | 68/462 | no | 0.32 |
| 5x5 seed 8 | 5 | minimal | GF(2^31-1) | 1 | 463 | 463 | 45 | 418 | 0/462 | no | 0.00 |
| 5x5 seed 8 | 5 | minimal | GF(2^31-1) | 2 | 106954 | 90066 | 36725 | 70229 | 68/462 | no | 0.36 |
