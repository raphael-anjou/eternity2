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
solution is publicly known; the best boards here reach 469.

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

**1,078 distinct boards** scoring 400–480, the raw material for methods that
learn from strong boards (position priors, learned move-ordering, anti-pattern
mining). Unzips to `boards.csv` and `boards.jsonl`, same rows in both:

```
id,score,family,edges
e2b-00001,469,f008,adcaaendadwe…   (1024 lowercase letters)
```

- **`id`** — a neutral, stable identifier (`e2b-NNNNN`), assigned by sorting on
  (score descending, then edge string), so the numbering is reproducible.
- **`score`** — matched interior edges, **recomputed from the edge string**, not
  copied from any source field (see "How the scores are verified" below).
- **`family`** — the board's corner family (`fNNN`), its four corner pieces. A
  proxy for which basin the board sits in; families are numbered by size, so
  `f001` is the most populous. There are **28** distinct families.
- **`edges`** — the board itself: 256 cells × 4 sides = 1024 lowercase letters,
  row-major, each cell in **up, right, down, left** order, `a` = the grey rim
  (colour 0), `b`, `c`, … the interior colours. This is the same `board_edges`
  string the [e2.bucas.name](https://e2.bucas.name) viewer reads, so any board
  can be pasted straight into that viewer to see it.

Score distribution: a clean bell curve peaking at 452–456 (roughly 55–70 boards
each), tailing to **21 boards at 460 or above**, of which the very best are one
at 466 and two at 469.

### This corpus is diverse, not derivative

Before releasing 1,078 boards, we checked they are not just perturbations of one
another (which would make the count misleading). They are not:

- **Exact copies removed:** only 11 of the 1,089 source boards were exact
  duplicates; the rest are distinct.
- **Nearest-neighbour distance:** the *median* board differs from its closest
  sibling by **60 of 256 piece placements**. Only 3% of boards sit within 10
  placements of another, and only 7% within 25.
- **Basins:** 28 distinct corner families, spread across the corpus rather than
  concentrated in one.

So the corpus is a genuinely varied sample of the strong-board landscape, not a
single basin re-found a thousand times.

## How the scores are verified

Every score in this dataset is recomputed from the board's own edge string by
counting matched interior adjacencies, never trusted from a stored field. In the
source data one board carried a stale score (it claimed 453 but its edges score
456); recomputing from edges corrects it automatically. If you re-derive the
scores yourself with the definition above, you will get the published numbers
exactly.

## Rebuilding this dataset

`build/build_dataset.py` regenerates everything from the in-repo instances and
the local board store. The benchmark instances are assembled from the public
variant files; the corpus is deduplicated, rescored from edges, stripped of all
working-store provenance (source paths, internal preset and seed labels,
filenames), and re-keyed with neutral ids. Only board content and verified
scores are published.
