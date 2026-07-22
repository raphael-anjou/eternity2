---
id: design-recipe-464
title: "The design recipe behind the 464 record: scan direction picks the basin"
summary: The July 2026 community 464 boards carry all their broken edges in the top rows while the public 460 carries them in the bottom rows, mirror images that identify scan direction as the primary basin selector of the modified Blackwood DFS.
status: draft
created: 2026-07-22
updated: 2026-07-22
contributors:
  - name: Raphael Anjou
    role: author
credits:
  - Igor Pejic and benj39100, who posted the 464 boards and described the method as a modified Blackwood DFS (groups.io "Eternity2", thread "Record of Eternity2 with 5 hints?", July 2026).
  - Brendan Blackwood, whose break-scheduled DFS is the base algorithm; Carlos Fernandez, who described the rotate-and-place bidirectional construction.
tags:
  - records
  - decoding-records
  - search-design
sources:
  - label: "groups.io Eternity2 thread #11906-11919 (July 2026)"
    url: https://groups.io/g/eternity2
  - label: "Eternity II board viewer (e2.bucas.name)"
    url: https://e2.bucas.name
reproduce:
  - cd research/topics/design-recipe-464/compute && cargo run --release > ../results/break-geometry.json
results:
  - label: Break-geometry profiles (JSON, computed here)
    path: results/break-geometry.json
site:
  render: false
  dataFile: null
---

# The design recipe behind the 464 record: scan direction picks the basin

> **Status:** draft · **Created:** 2026-07-22 · **Updated:** 2026-07-22
> **Authors:** Raphael Anjou · **Credits:** Igor Pejic, benj39100 (boards and method description); Brendan Blackwood (base algorithm); Carlos Fernandez (bidirectional construction idea)

## Summary

In July 2026 the community record with the five official clues jumped from 460
to 464. The posters described their method only as a "modified Blackwood's DFS
algorithm". This topic reproduces the analysis that identifies what the
modification most plausibly is. The evidence is geometric: where on the board
the broken edges sit.

The three posted 464 boards and the public 460 are all complete, clue legal
boards, but they live in three mutually near orthogonal basins (about 243 to
250 of 256 tiles differ pairwise). Attributing each broken edge to a cell in
row major order shows a sharp pattern. The public 460 has all 20 of its breaks
in the bottom four rows (rows 12 to 15) and rows 0 to 11 perfectly clean. The
464 boards have all 16 of their breaks in the top six rows (rows 0 to 5) and
rows 6 to 15 perfectly clean. These are mirror images. A row major top to
bottom DFS pushes its error budget into the last placed rows, which is exactly
the 460's signature. Breaks concentrated at the top are the signature of a
bottom to top construction. The conclusion is that scan direction is the
primary basin selector: building from the opposite end of the board is what
made the fresh 464 basins reachable, and it explains why they are near
orthogonal to the top down 460. One of the 464 boards reaches its score with
zero cells carrying two breaks, so a standard at most one break per cell DFS
can represent that basin directly.

This is a design recipe, not a record claim: the levers it isolates are scan
direction (which end of the board the DFS fills last), a break budget spent in
the last placed band, and anytime capture of the best complete board across
restarts.

## Reproduction

The claim is checkable from the boards alone. The five boards (three 464s, two
public 460 postings) are bundled as viewer URLs in `compute/boards.txt` and
repeated per board inside the results file. The `compute/` crate decodes each
URL's edge grid, rescores it with the site's one canonical scorer, verifies the
five official clues, and computes the break geometry: per cell break counts
(attributing each mismatched interior adjacency to the row major later cell),
per row totals, cells with two breaks, and the pairwise tile Hamming distances
that establish the basin map. The run takes under a second; the output is
byte stable across reruns (exact tier).

Measured against the expected numbers of `compute/PLAN.md`:

| Board | Score (exp / got) | Breaks (exp / got) | Break rows (exp / got) | Double breaks (exp / got) | Clues |
|---|---|---|---|---|---|
| 464 basin C (b0) | 464 / 464 | 16 / 16 | r0..r5 / r0..r5 | 1 / 1 | 5/5 |
| 464 basin B (b1) | 464 / 464 | 16 / 16 | r1..r5 / r1..r5 | 2 / 2 | 5/5 |
| 464 basin B (b2) | 464 / 464 | 16 / 16 | r1..r5 / r1..r5 | 0 / 0 | 5/5 |
| 460 public (a) | 460 / 460 | 20 / 20 | r12..r15 / r12..r15 | 3 / 3 | 5/5 |
| 460 public (b) | 460 / 460 | 20 / 20 | r12..r15 / r13..r15 | 3 / 3 | 5/5 |

Every board is complete (256 placed) and the break identity breaks = 480 minus
score holds on all five. The basin C break cell list came out exactly as
expected: [12, 27, 31, 38, 40, 44, 46, 53, 58, 62, 63, 71, 73, 77, 93], with
the single double break at cell 77. The tile Hamming basin map also matched:
b0 vs b1 = 250, b0 vs b2 = 250, b1 vs b2 = 6, and every 464 sits 243 to 247
tiles away from the public 460s (the two 460 postings differ from each other by
only 9 tiles, one basin). The one sub row note: 460 posting (b) happens to have
its row 12 clean, so its break band is r13..r15, still inside the expected
r12..r15 bottom band.

Verdict: reproduced. All numeric claims in the plan came out exactly, and the
mirror image geometry (464 breaks confined to the top band, 460 breaks to the
bottom band, clean everywhere else) is visible directly in the `break_grid`
field of the results file. Representative board, the basin C 464, first URL in
`compute/boards.txt` labelled `464-basinC-b0` and echoed in
`results/break-geometry.json`.
