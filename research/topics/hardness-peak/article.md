---
id: hardness-peak
title: The hardness peak, measured in-repo
summary: Sweeping solver effort against interior-color count traces the difficulty peak Eternity II is tuned to, and an 8x8 sweep tests Owen's one-expected-solution criterion at a second board size against a measured hard band.
status: published
created: 2026-07-22
updated: 2026-07-22
contributors:
  - name: Raphaël Anjou
    role: author
tags:
  - structure
  - complexity
sources:
  - label: "Brendan Owen, Design the hardest puzzle: the 17+5 derivation (eternity2 mailing list, msg 1947, August 2007)"
    url: https://groups.io/g/eternity2/message/1947
  - label: "Ansótegui, Béjar, Fernández, Mateu. How Hard is a Commercial Puzzle: the Eternity II Challenge."
    url: https://repositori.udl.cat/server/api/core/bitstreams/0b6533fe-54e5-4070-85fe-80f7d35837d8/content
  - label: "Mateu, Hamadi. Edge matching puzzles as hard SAT/CSP benchmarks. Constraints, Springer 2012."
    url: https://link.springer.com/article/10.1007/s10601-012-9128-9
reproduce:
  - cd research/topics/hardness-peak && ./run-both.sh
results:
  - label: Solver effort vs interior-color count, 8x8 traversable peak (JSON, computed here)
    path: results/hardness-peak-8x8.json
  - label: Solver effort vs interior-color count, 16x16 real board (JSON, computed here)
    path: results/hardness-peak-16x16.json
site:
  render: false
---

# The hardness peak, measured in-repo

> **Created:** 2026-07-22 · **Updated:** 2026-07-22

## Summary

The phase-transition page argues, from external work, that framed edge-matching
puzzles are hardest at a particular interior-color count near Eternity II's own
17. This topic measures that curve inside the repository, with the site's own
seeded generator and backtracking DFS, and turns the external criterion into a
test that can pass or fail.

Brendan Owen's one-expected-solution criterion predicts the peak interior-color
count for a board of Eternity II's shape as $I = (196! \cdot 4^{196})^{1/392}
\approx 17.14$, matching the official 17 (msg 1947, 2007). The same formula at a
smaller board size predicts $(36! \cdot 4^{36})^{1/72} \approx 7.56$ for an 8x8.
An 8x8 board is small enough that the plain DFS actually solves instances, so its
solve-rate curve rises and falls and the peak is directly observable. **The
measured 8x8 hard band is c = 6, 7, 8, where none of the 30 instances per color
count solve within budget, and Owen's size-8 prediction of 7.56 falls inside
it**, a genuine scale-transfer test of a criterion that was only ever stated for
the full board.

## Background

Edge-matching difficulty has a knob: the number of colors. Too few and pieces fit
together in countless ways, so a search finds one arrangement quickly. Too many
and pieces barely fit, so the constraints resolve the board fast. In between sits
a band where solutions are scarce but present, and that is where a naive search
does the most work. Owen's criterion locates that band by a first-moment estimate:
the interior of an $n \times n$ board holds $P = (n-2)^2$ interior pieces, with
$P!$ arrangements and $4^P$ orientations. Placing them one at a time, each piece
brings on the order of two new edge-matches into contact (its interior seams are
shared with neighbours, so counted once per piece the interior has about $2P$
match constraints; for the real board this is $2 \cdot 196 = 392$, the interior
seams $2(n-2)(n-3) = 364$ plus the $2(n-2) = 28$ interior edges meeting the
border ring). Each match holds with probability $1/I$, so the expected number of
fully matched arrangements is $P! \cdot 4^P / I^{2P}$. Setting that to one gives
$I = (P! \cdot 4^P)^{1/2P}$. It is a mean-field average, not a solution count; it
locates the peak, it does not prove one solution exists.

## Method

Each measurement is one generated board and one solve:

- **Generator.** The board is built by the engine's frame-restricted generator
  with an explicit **5-color border band** (real Eternity II's border-color
  count) and `c` interior colors, so the total palette is `c + 5` and the border
  structure is identical across the whole sweep. The board is solvable by
  construction, then scrambled the same way the site's board generator scrambles.
- **Solver.** The engine's plain row-major backtracking DFS, no hints, run until
  it reaches a first full solution or spends a fixed **node budget** (DFS steps =
  placements + backtracks). Effort is counted in nodes, not seconds, so the curve
  is deterministic and hardware-independent; a recorded wall-time is kept only for
  convenience.
- **Metric.** For each `c` we take the solve rate across seeds and the median
  nodes-to-first-solution. The peak is where the solve rate dips and the median
  effort rises.
- **Criterion.** `hardness_peak --criterion <size>` computes Owen's predicted
  peak color count in-repo, reproducing his 17.14 at size 16.

**Two regimes, two result files.** The sweep runs at two board sizes because they
answer different questions:

- **8x8** (`results/hardness-peak-8x8.json`). Here the DFS actually solves
  instances, so the solve-rate curve rises and falls and the peak is
  **traversable**. This is the regime that produces a peaked curve to set beside
  the size-8 criterion prediction.
- **16x16** (`results/hardness-peak-16x16.json`), the real board. A plain
  unguided DFS cannot solve a full 16x16 at any color count within a feasible
  budget, so nodes-to-solution is pegged at the budget everywhere and only the
  depth reached under budget is measurable; that depth falls **monotonically**
  with `c`, placing the real board at the high end of the constrained regime
  rather than tracing a peak.

## Results

*Both sweeps are committed: the 8x8 (510 rows, 30 seeds per color count, node
budget 30M) and the 16x16 (570 rows, 30 seeds per requested color count, node
budget 20M).*

### 8x8: the traversable peak against the criterion

Predicted peak (Owen's criterion at size 8): **7.56 interior colors**.

Measured, 30 seeds per `c`, node budget 30M:

| interior colors `c` | solve rate | median nodes |
| ---: | ---: | ---: |
| 1 | 12/30 | at budget |
| 2 | 11/30 | at budget |
| 3 | 14/30 | at budget |
| 4 | 27/30 | 964,009 |
| 5 | 12/30 | at budget |
| 6 | **0/30** | **at budget** |
| 7 | **0/30** | **at budget** |
| 8 | **0/30** | **at budget** |
| 9 | 23/30 | 8,486,376 |
| 10 | 30/30 | 547,434 |
| 11 | 30/30 | 126,758 |
| 12 | 30/30 | 45,845 |
| 13 | 30/30 | 41,745 |
| 14 | 30/30 | 26,992 |
| 15 | 30/30 | 26,153 |
| 16 | 30/30 | 19,685 |
| 17 | 30/30 | 21,461 |

The solve rate collapses to 0 of 30 at exactly `c = 6, 7, 8`: all ninety
instances there run to the budget without a solution. Owen's size-8 prediction of
7.56 falls inside this band, between its two central counts. Above it the search
eases cleanly and monotonically: 23 of 30 solve at `c = 9`, then every instance
solves from `c = 10` up, with median effort falling to around 20,000 nodes by
`c = 17`. Below the band the picture is jagged and bimodal (12, 11, 14 of 30 at
`c = 1, 2, 3`, a spike to 27 of 30 at `c = 4`, then back to 12 of 30 at `c = 5`),
which is what the criterion's independence assumption predicts should be least
reliable: low-color boards have astronomically many solutions but also enormous
search plateaus, so effort there is dominated by plateau structure rather than by
solution scarcity. The peak the criterion locates is the high edge of that hard
band, and it is where the measured solve rate reaches zero. The grid step is one
color, so the band is reported as the measured integers `[6, 8]` with the 7.56
prediction inside; no finer location is claimed.

### 16x16: monotone decline and the interior-color ceiling

Measured, 30 seeds per requested `c`, node budget 20M: not one of the 570
instances solves, at any color count, as the calibration predicted (a plain
unguided DFS never completes a full 16x16). The only signal under budget is the
median depth reached, and it declines monotonically as the interior palette
widens:

| interior colors `c` | median deepest cell reached (of 256) |
| ---: | ---: |
| 8 | 238.0 |
| 9 | 237.0 |
| 10 | 234.5 |
| 11 | 232.0 |
| 12 | 228.0 |
| 13 | 222.5 |
| 14 | 218.0 |
| 15 | 213.0 |
| 16 | 205.0 |
| 17 | 199.5 |

So the effort curve does not peak on the full board; the search dies steadily
shallower as `c` rises, placing the real board past the point where the curve has
already gone vertical rather than on a traversable peak.

A framed 16x16 board holds **at most 17 interior colors**, exactly the official
count, so the sweep cannot even run past the peak on a full board: the 30 seeds at
each requested count from 18 to 26 all clamp to 17 interior colors and come back
bit-identical to the `c = 17` rows, seed by seed (same nodes, same depth), flagged
`clampedToCeiling`. This cap is a property of the **format's alphabet, not the
board geometry**: Eternity II is drawn from 22 edge motifs, and reserving 5 of
them for the border band (real Eternity II's border-color count) leaves 17 for the
interior. A larger motif alphabet would raise the cap on the same 16x16 grid; the
geometry alone does not force 17. The full board therefore sits at the color count
the criterion predicts, and the format cannot even express the far side of the
peak.

## Reproduce

```sh
cd research/topics/hardness-peak
./run-both.sh          # both regimes, in order: 8x8 then 16x16
```

`run-both.sh` writes `results/hardness-peak-8x8.json` then
`results/hardness-peak-16x16.json`. To run one regime, call `run.sh` with
`E2_SIZE` set; `COLORS`, `SEEDS`, `NODE_BUDGET`, and `OUT` override the grid,
budget, and output path. Owen's predicted peak for any size:
`compute/target/release/hardness_peak --criterion <size>`. Deterministic given
the same solver build and node budget; the recorded seconds are informational.

## Notes

- "The peak" is a band, not a single integer, and its location shifts with board
  size. The criterion tracks that shift (17.14 at size 16, 7.56 at size 8), which
  is exactly why running the sweep at a second size tests it rather than merely
  restating it.
- Owen's estimate is a first-moment average that assumes independent color
  matches; it locates the peak, it does not count solutions or bound anything.
- This complements, rather than replaces, the external citations on the
  phase-transition page: it is the same phenomenon, measured with the
  repository's own tools, and extended to a second board size.
