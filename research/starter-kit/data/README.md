# Official Eternity II data

This folder ships the real puzzle so the kit works out of the box.

- **`official.json`** — the official Eternity II puzzle in the site's canonical
  `Puzzle` / `SiteInstance` schema: all 256 pieces in id order (each as four URDL
  edge integers, `0` = grey border) plus the five official clues (`pos`, `piece`,
  `rot`). This is the *same* format the site's viewer and every algorithm read
  (`--puzzle …json`), so the kit ships the real set in our own format, not a
  foreign one.

Load it with `e2_kit::official_instance(with_clues)`, which goes through
`e2-io`'s own `SiteInstance` deserialiser, so the kit does no parsing of its own.
Need `e2pieces.txt`, CSV, or a URL instead? The `convert` example derives every
other format from this on demand.

## Provenance and correctness

This is derived from the project's canonical `official_eternity2.csv` and is
**identical, in piece order, to the set the site's engine uses**. Correctness is
not asserted, it is checked: the kit's test suite confirms the set has Eternity
II's exact border census (4 corners, 56 edges, 196 interior), that it round-trips
through `SiteInstance`, and that the five clues sit at their known cells
(34, 45, 135, 210, 221). If the data were wrong, those tests fail.

## A note on the piece data

This is edge-adjacency data — which colour meets which — the same numbers every
`e2pieces.txt` has encoded since 2007, not the printed artwork or motifs. In the
contest years the community's norm was not to redistribute the set (see the
[toolbox history](https://eternity2.dev/research/build/tooling)); that norm has
since eroded — filled-in piece files have been on public GitHub for years, and
open libraries (jwortmann's Eternity2Puzzles.jl, Dobrogost's eii-puzzles) ship
the set outright. We ship it here for the same reason: reproducibility without a
manual transcription step.
