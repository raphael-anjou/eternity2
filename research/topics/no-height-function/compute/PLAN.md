# Reproduction plan: no-height-function

> **Repro tier: qualitative.**
> exact = re-run reproduces committed results byte-for-byte. seeded-statistical =
> committed results are the record of our run (hardware noted in the article);
> reproduction = fresh run agreeing within the stated tolerance. qualitative =
> the effect (sign/ordering/regime) reappears; exact digits are lineage-bound.

Source of the claim: research repo, `research/vault/papers/vol-228/PHYS-C.md`
(vol-228 physics sub-wave, charge PHYS-C), Theorems 2 and 3, with the umbrella
finding restated in `research/vault/papers/vol-226/ALG.md` (the only surviving
conserved charge is EXT's unsigned per-color parity, an F_2^22 bit).

## (a) The exact claim and why the numbers are board-specific

PHYS-C reformulates E2 in the language of crystal defects and asks whether the
puzzle admits a dimer-style **height function**: a per-face potential whose
gradient is smooth across matched joints and jumps across breaks, so breaks
would carry a conserved oriented charge (a Burgers vector) annihilable in
pairs. The paper closes this hope with three obstructions. Two of them are
*measured on real boards* and are what this topic reproduces:

| # | Obstruction (PHYS-C) | Paper's measurement |
|---|----------------------|---------------------|
| Thm 2 | The Z/2 dual break-chain is not closed: the break set has dual vertices of ODD break-degree (dislocation cores = open-string endpoints). A height needs every dual vertex even. | 28 odd cores on the 451 board; 22 on 463; 32 on 458 |
| Thm 3 | The signed per-color current is not globally conserved: orient each c-half joint from the c-carrying cell toward its non-c neighbor and sum; a conserved current sums to zero. | Many nonzero global sums on 451, e.g. color 6 -> (-1,-3), color 11 -> (1,3) |

The third obstruction (Thm 1, a per-face scalar height telescopes to zero
holonomy and is blind to breaks) is a PROVED pen-and-paper identity, not a
board measurement, and is stated in the article rather than recomputed.

**Why qualitative, not exact.** Both measured quantities are functions of the
specific board. The paper measured them on three particular boards (a 451, a
463, a 458) that are NOT the boards we ship. Our committed boards are the four
in `topics/record-boards/boards.json` (scores 463/460/458/460). So the exact
integers must differ, by construction. What reproduces is the **structural
law** the paper draws from those integers:

1. odd-degree dual cores exist and are numerous (not a handful of accidental
   endpoints, but tens of them), so breaks are open strings, not dual cycles;
2. at least one per-color signed current has a nonzero global sum, so the
   naive Burgers vector is not conserved.

Either failing on any board would admit a height function on that board and
refute the obstruction. The checker hard-fails if a board has zero cores or a
fully conserved current.

## (b) Scoring-convention mapping

Every board is re-scored in-checker with the kit's canonical `score_cells`
(the one source of truth: matched non-border right/down adjacencies, 480
internal joints, perimeter forced gray). The checker refuses to measure a
board whose re-scored value disagrees with its committed score, so the
topology is always measured on a board of known, verified quality. The four
committed boards are matched-edge boards (score = matched internal edges); no
strict-5/5 claim is made or needed here — the break topology is a property of
the edge grid, independent of hint placement.

A "break" is a joint whose two facing half-edges differ. On these boards
internal joints never carry the gray rim color, so "differ" coincides exactly
with the scorer's "not a non-border match", i.e. breaks = 480 - score.

## (c) Scale-faithfulness

The obstruction is a property of *scored E2 boards*, and it needs a genuine
high-scoring board to be meaningful (on a random board almost every joint is a
break and cores/currents are trivially abundant; the content is that they stay
abundant even at 458-463/480, three-to-five breaks from a hypothetical
solution). We therefore measure on the canonical 16x16 official instance's
real record boards, not on a generated small-N board:

- a generated *solved* small board has zero breaks, so zero cores and zero
  current — vacuously "no obstruction", telling us nothing;
- a generated *scrambled* small board has a different color palette and a
  different piece design, so its core count would not speak to the official
  instance the paper measured.

Hence the checker runs exclusively on committed official-instance boards. The
runtime is well under one second (four boards, one linear pass each), so no
scale-down was needed.

## (d) Exact reproduction steps

1. `cd research/topics/no-height-function/compute`
2. `cargo run --release > ../results/no_height_function.json`
3. The binary re-scores each board, measures odd dual cores and non-conserved
   per-color currents, checks the qualitative law on all four boards, and
   emits one JSON object. It exits nonzero (printing the first failing board)
   if any board is mis-scored or fails the law.
4. Compare against the source regime: our per-board core counts (22-34) sit in
   the paper's regime (22/28/32), and every board has several colors with a
   nonzero current. The exact integers are OUR boards' and are not the paper's.

The measurement is fully deterministic (no RNG): reruns are byte-identical.

## Checker structure

`src/main.rs`, single file, deps `e2-kit` (path dep on the starter kit) and
`serde_json`:

- read the bundled `../../boards.json` (four committed high-scoring boards as
  bucas `board_edges` params, copied from `topics/record-boards/boards.json`);
- parse each board through the kit's `parse_board_edges` (URDL, char->color),
  re-score with `score_cells`, and refuse any board that mis-scores;
- Theorem 2: accumulate break-degree per dual vertex (grid corners, 17x17) and
  count odd-degree vertices (cores);
- Theorem 3: accumulate the signed per-color current and count colors with a
  nonzero global sum;
- assert the qualitative law on every board; print one JSON object; nonzero
  exit on any failure.
