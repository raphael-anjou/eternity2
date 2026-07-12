---
id: clue-puzzle-pieces
title: The clue puzzles, recovered and solved
summary: The pieces of all four Eternity II clue puzzles, which the community withheld on copyright grounds, survive in two solvers' example files — Clues 1 & 2 in Le Bail's 2007 SHORTER, all four in Wortmann's Eternity2Puzzles.jl. Here they are, cross-checked, solved and verified from scratch.
status: published
created: 2026-07-12
updated: 2026-07-12
contributors:
  - name: Raphaël Anjou
    role: recovery, verification and write-up
credits:
  - Clue 1 & 2 piece sets recovered from Armel Le Bail's SHORTER solver (cristal.org, 2007).
  - Clue 3 & 4 piece sets recovered from Janos Wortmann's Eternity2Puzzles.jl (GitHub, 2024–25).
  - Official clue-puzzle position files recovered from the Internet Archive Wayback Machine (eternity2.net, 2007).
tags:
  - clue-puzzles
  - enumeration
  - reference
sources:
  - label: "Armel Le Bail's SHORTER solver, with the Clue 1 & 2 example files (cristal.org)"
    url: http://www.cristal.org/Eternity-II/
  - label: "Janos Wortmann's Eternity2Puzzles.jl, with piece files for all four clue puzzles (GitHub)"
    url: https://github.com/jwortmann/Eternity2Puzzles.jl
  - label: "eternity2.net clue-puzzle position files (Wayback Machine, 2007-10-17)"
    url: https://web.archive.org/web/20071017024639/http://eternity2.net:80/e2pieces_clue2.txt
  - label: "kissat SAT solver (Biere et al.)"
    url: https://github.com/arminbiere/kissat
reproduce:
  - cd research/topics/clue-puzzle-pieces && ./run.sh
results:
  - label: Clue Puzzle 1 solution (JSON, computed here)
    path: results/clue1-solution.json
  - label: Clue Puzzle 2 solution (JSON, computed here)
    path: results/clue2-solution.json
  - label: Clue Puzzle 3 solution (JSON, computed here)
    path: results/clue3-solution.json
  - label: Clue Puzzle 4 solution (JSON, computed here)
    path: results/clue4-solution.json
site:
  render: true
  dataFile: results/clue2-solution.json
---

# The clue puzzles, recovered and solved

Tomy sold four small companion puzzles for Eternity II. Solve one, submit the
answer, and the official site revealed one piece's placement on the main board.
Their piece designs fell under the same copyright claim as the prize puzzle, so
the community, which owned and solved them, deliberately never published the
layouts. For eighteen years the actual puzzles were, in effect, missing.

They were not missing. The pieces of all four clue puzzles survive in the example
files of two solver projects, and this page recovers them, cross-checks the two
sources, solves each from scratch, and checks the solutions.

## Where they survive

**Clues 1 & 2.** Armel Le Bail wrote a Fortran solver, SHORTER, in August 2007 and
published it at cristal.org with three worked examples: a 36-piece 6×6, a 72-piece
12×6, and the full 256-piece board. The 256-piece file is the real official
Eternity II set, which is how the provenance checks out: Le Bail was working from
the genuine Tomy data, reading the piece format defined at the then-live
eternity2.net. The 36- and 72-piece files are Clue Puzzle 1 and Clue Puzzle 2,
with their real motifs intact.

**Clues 3 & 4.** These are absent from SHORTER but present in a second, much more
recent project: Janos Wortmann's `Eternity2Puzzles.jl`, a Julia package on GitHub
that bundles all four clue puzzles as plain piece files (`clue1.txt` …
`clue4.txt`). Its `clue1` and `clue2` are the same two puzzles Le Bail shipped —
we canonicalise each file into a rotation-invariant, colour-relabelled multiset of
pieces and Wortmann's Clue 1 and Clue 2 match Le Bail's exactly, piece for piece.
A source that reproduces the two puzzles we can already cross-check is a
trustworthy source for the two we cannot, so its Clue 3 (36, 6×6) and Clue 4 (72,
12×6) stand on the same footing.

The files eternity2.net published itself, recovered from the Wayback Machine,
confirm the sizes and board positions of Clues 1 and 2 but not the motifs: they
place each as a block on a 16×16 grid (the 6×6 at rows 5–10, columns 5–10; the
6×12 at rows 5–10, columns 2–13) and collapse every interior edge to a single
placeholder colour. (The site never hosted files for Clues 3 or 4, which launched
after its last capture.) Those files show *where* Clues 1 and 2 sit; the solver
files are the public source for *what* the pieces are, for all four.

## Solved and verified

Each puzzle is solved here with a SAT solver rather than a hand-written search.
The encoding is direct: one variable per (cell, piece, rotation), clauses for one
piece per cell, each piece used once, grey on the rim, and every shared edge
matched. `kissat` settles each instance in well under a second, and a separate
pass validates the returned solution independently: all pieces used exactly once,
every internal edge matched, grey only on the outer ring.

All four solve cleanly. Clues 1 and 3 match all 60 of their internal edges; Clues
2 and 4 match all 126 of theirs. The solved boards are rendered on the clue-puzzles
page straight from the solver's output, drawn in the real Eternity II motifs.

A note on method: a naive row-major backtracker is a trap here. Clue Puzzle 1
has an enormous number of solutions yet an ordering-hostile structure, and a
plain backtracker can churn through billions of placements without landing on
one. The SAT encoding sidesteps the ordering problem entirely and proves the
answer.

## What this is and is not

This recovers and verifies all four clue puzzles. It does not touch the copyright
question those layouts were withheld over; it reports that they are, in fact,
already public in two solvers' example files, and shows them solved. The
placements the puzzles revealed on the main board are documented separately in
[Known facts](/research/build/known-facts) and the full history of the four
puzzles is on [the clue puzzles page](/research/build/clue-puzzles).
