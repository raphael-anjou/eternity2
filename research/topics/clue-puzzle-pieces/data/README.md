# Clue-puzzle piece data — provenance

`clue1_shorter36.txt` and `clue2_shorter72.txt` are the piece sets of Eternity II
**Clue Puzzle 1** (36 pieces, 6×6) and **Clue Puzzle 2** (72 pieces, 12×6),
recovered from the example files bundled with **Armel Le Bail's SHORTER solver**
(Fortran, 2007), published at <http://www.cristal.org/Eternity-II/> as
`shorter36.zip` and `shorter72.zip`. Le Bail's solver reads the piece format
defined at the then-live eternity2.net; its 256-piece example is the real
official Eternity II set, which is how the provenance was confirmed.

File format: one piece per line, `ID T R B L`, where T/R/B/L are the edge
colours in URDL order (up, right, down, left) and colour `0` is the grey rim.

These are the *solvable* puzzles with their real motifs. The files that
eternity2.net itself published (`e2pieces_clue1.txt`, `e2pieces_clue2.txt`,
recovered from the Wayback Machine) are motif-redacted: they show each clue
puzzle's size and board position on a 16×16 grid but collapse every interior
motif to a single placeholder colour, so they are not solvable as puzzles.
