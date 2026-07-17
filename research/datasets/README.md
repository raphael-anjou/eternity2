# Eternity II dataset

A public dataset for working on the Eternity II edge-matching puzzle, in two
parts: a set of **benchmark instances** to solve, and a **corpus of strong
boards** to learn from. Released into the public domain under
[CC0 1.0](./LICENSE): no attribution required, use it for anything.

Version 1.0.0.

## What Eternity II is, in one paragraph

Eternity II is a 16×16 edge-matching puzzle: 256 square pieces, each with a
colour on each of its four sides, must tile the board so that every pair of
touching sides shares a colour, with a grey border colour on the rim. A board's
score is its number of matched interior edges; the maximum is **480**
(the 16×15 horizontal plus 15×16 vertical interior adjacencies). No full 480
solution is publicly known; the best boards here reach 470.

## Part A — benchmark instances (`instances/`)

The inputs a solver runs on. Fourteen instances, each a self-contained JSON:

| id | size | pieces | hints | max score |
|---|---|---|---|---|
| `e2-16x16-v00` … `v09` | 16×16 | 256 | 8 | 480 |
| `e2-clue-clue1`, `clue3` | 6×6 | 36 | 0 | 60 |
| `e2-clue-clue2`, `clue4` | 12×6 | 72 | 0 | 126 |

The ten 16×16 instances are the official Eternity II piece set with three
corners pinned to fixed pieces, in ten different corner arrangements. Pinning the
corners this way gives ten distinct but equally-hard instances off one piece set,
which is what lets a study compare engines on a *diversity* axis rather than a
single board. All three of this project's engine studies (backtracking, repair,
and the single-core benchmark) solve exactly these ten. The four clue
sub-puzzles are the smaller boards from the puzzle's official clue set; each is
known to have a full solution (max score reachable), which makes them a good
correctness fixture.

Each instance file has the shape:

```json
{
  "id": "e2-16x16-v00",
  "family": "official-16x16-pin3corners",
  "width": 16, "height": 16, "numColors": 22,
  "pieces": [[up, right, down, left], ...],   // one row per piece, URDL colours
  "hints": [{"pos": 0, "piece": 0, "rot": 3}, ...],  // pinned cells (may be empty)
  "maxScore": 480
}
```

`pos` is the row-major cell index (0 = top-left, 255 = bottom-right on 16×16).
`rot` is the piece's clockwise quarter-turn count. Colour 0 is the grey rim.

`instances/index.json` lists all fourteen with their headline numbers.

## Part B — strong-board corpus (`e2-strong-boards.zip`)

**7,658 distinct boards** scoring 400–480, the raw material for methods that
learn from strong boards (position priors, learned move-ordering, anti-pattern
mining). Unzips to a `boards/` directory holding **one file per board**, each a
complete self-describing document in the project's canonical board format:

```
boards/e2b_00001.json
boards/e2b_00002.json
…
index.json                # id -> score, family, for querying without opening files
```

Each board file is a canonical **BoardDoc**, the exact same format every solver
on the site emits and the [format spec](https://eternity2.dev/research/build/formats)
describes in full. Given one of these files, a tool can render the board,
identify every piece, re-derive any other format, or open it in a viewer, with no
companion file:

```json
{
  "name": "e2b_00001",
  "size": 16,
  "score": 469,
  "breaks": 11,
  "board": [15, 160, 140, …],
  "board_hash": 2059303548241384230,
  "board_edges": "adcaaendadwe…",
  "board_pieces": "016161141…",
  "url": "https://eternity2.dev/viewer?puzzle=e2b_00001&puzzle_size=16&board_edges=…&board_pieces=…"
}
```

- **`name`** — a neutral, stable identifier (`e2b_NNNNN`), assigned by sorting on
  (score descending, then edge string), so the numbering is reproducible. It is
  the same string in the filename, the `name` field, and the viewer `url`.
- **`score`** — matched interior edges, and **`breaks`** = `480 − score`. The
  score is recomputed from the board's pieces, never copied from a source field
  (see "How the scores are verified").
- **`board`** — the row-major `piece*4 + rot` placement vector (the piece
  identity edge-only formats cannot recover); **`board_pieces`** is the 1-based
  piece number per cell, **`board_edges`** the URDL colour letters (`a` = grey
  rim). **`board_hash`** is the FNV-1a fingerprint of the placement.
- **`url`** — a ready-to-open [eternity2.dev](https://eternity2.dev/viewer)
  viewer link; paste it in a browser to see the board.
- **`index.json`** carries each board's **family** (`fNNN`), its four corner
  pieces, a proxy for which basin it sits in; families are numbered by size
  (`f001` most populous). There are **28** distinct families.

Score distribution: a sharp spike in the low-450s, where a large single-core
beam harvest concentrates, tailing to **38 boards at 460 or above**, of which the
very best are one at 466 and two at 470.

### This corpus is diverse, not derivative

Before releasing 7,658 boards, we checked they are not just perturbations of one
another (which would make the count misleading). They are not:

- **Exact copies removed:** only 13 of the 7,671 source boards were exact
  duplicates; the rest are distinct.
- **Nearest-neighbour distance:** the *median* board differs from its closest
  sibling by **235 of 256 piece placements**. Almost no board sits within 10
  placements of another, and only 1% within 25; even boards sharing a narrow
  score band are structurally almost entirely different.
- **Basins:** 28 distinct corner families, spread across the corpus rather than
  concentrated in one.

So the corpus is a genuinely varied sample of the strong-board landscape, not a
single basin re-found many times over.

## How the scores are verified

Every score in this dataset is recomputed by counting matched interior
adjacencies on the board's pieces, never trusted from a stored field. In the
source data one board carried a stale score (it claimed 453 but actually scores
456); recomputing corrects it automatically. Each board file is produced by the
project's canonical scorer and format code, the same code every solver on the
site emits through, so re-derive the score yourself and you will get the
published number exactly. The build asserts each board's independently recomputed
score before writing it.

## Rebuilding this dataset

`build/build_dataset.py` regenerates everything from the in-repo instances and
the local board store. The benchmark instances are assembled from the public
variant files; the corpus is deduplicated, rescored, stripped of all
working-store provenance (source paths, internal preset and seed labels,
filenames), and re-keyed with neutral ids. Each board is then converted to a
canonical BoardDoc by the `board_doc` binary in the `e2-io` crate, so the
published files use the exact site format with no reimplementation. Only board
content and verified scores are published.
