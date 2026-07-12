# Clue-puzzle piece data — provenance

These are the piece sets of all four Eternity II clue puzzles, recovered from two
independent solver projects that bundled them as example files.

**Clues 1 & 2** — `clue1_shorter36.txt` (36 pieces, 6×6) and
`clue2_shorter72.txt` (72 pieces, 12×6) come from **Armel Le Bail's SHORTER
solver** (Fortran, 2007), published at <http://www.cristal.org/Eternity-II/> as
`shorter36.zip` and `shorter72.zip`. Le Bail's solver reads the piece format
defined at the then-live eternity2.net; its 256-piece example is the real
official Eternity II set, which is how the provenance was confirmed.
Format: one piece per line, `ID T R B L`, edge colours URDL, colour `0` = grey rim.

**Clues 3 & 4** — `clue3_jwortmann.txt` (36, 6×6) and `clue4_jwortmann.txt`
(72, 12×6) come from **Janos Wortmann's `Eternity2Puzzles.jl`**
(<https://github.com/jwortmann/Eternity2Puzzles.jl>, `pieces/clue3.txt`,
`pieces/clue4.txt`), which carries all four clue puzzles. Cross-check: that repo's
`clue1`/`clue2` are, as rotation-invariant colour-relabelled multisets, identical
to Le Bail's Clue 1 and Clue 2 — so the same source's Clue 3 and Clue 4 are
trusted on that basis. Format: one piece per line, four edge colours URDL,
colour `0` = grey rim (no ID column).

All four are *solvable* puzzles with real, distinct motifs; the solve in
`../compute` closes each one (Clues 1 & 3: 60/60 internal edges; Clues 2 & 4:
126/126). The files that eternity2.net itself published
(`e2pieces_clue1.txt`, `e2pieces_clue2.txt`, recovered from the Wayback Machine)
are motif-redacted: they show each clue puzzle's size and board position on a
16×16 grid but collapse every interior motif to a single placeholder colour, so
they are not solvable as puzzles. The site never hosted files for Clues 3 or 4.
