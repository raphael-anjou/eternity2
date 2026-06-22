---
id: phase-transition
title: Tuned to the hardness peak
summary: Eternity II's colors are split 17 interior to 5 frame-only, which is exactly where edge-matching puzzles are at their hardest.
status: published
created: 2026-06-22
updated: 2026-06-22
contributors:
  - name: Raphaël Anjou
    role: author
tags:
  - structure
  - complexity
sources:
  - label: "Mateu, Hamadi. Edge matching puzzles as hard SAT/CSP benchmarks. Constraints, Springer 2012."
    url: https://link.springer.com/article/10.1007/s10601-012-9128-9
  - label: "Ansótegui, Béjar, Fernández, Mateu. How Hard is a Commercial Puzzle: the Eternity II Challenge."
    url: https://repositori.udl.cat/server/api/core/bitstreams/0b6533fe-54e5-4070-85fe-80f7d35837d8/content
reproduce:
  - cd research/topics/phase-transition/compute && cargo run --release > ../results/color-split.json
results:
  - label: Color split of the official set (JSON, computed here)
    path: results/color-split.json
site:
  render: true
  dataFile: results/color-split.json
---

# Tuned to the hardness peak

> **Created:** 2026-06-22 · **Updated:** 2026-06-22

## Summary

Eternity II uses 22 colors. Sorted by where they appear in the official set, they
split cleanly: 17 colors live in the interior, and 5 colors appear only on the
frame. That is not a round number someone picked for looks. Around 17 interior
colors is exactly where this kind of puzzle is hardest to solve, the point where
you'd expect roughly a single solution to exist. The puzzle was built to sit
right on that peak.

## Background

Most hard search problems have a difficulty knob. Make the problem too loose and
there are many solutions, so any decent search trips over one quickly. Make it too
tight and there are no solutions, which is often easy to prove. In between sits a
narrow band where solutions are scarce but not absent, and that band is where
search blows up. For many problem families this transition is sharp and
well-studied; people call it a phase transition, by analogy with water freezing.

Edge-matching puzzles have exactly this behavior, and the knob is the number of
colors. Too few colors and pieces fit together in countless ways. Too many and
they barely fit at all. The published analysis of framed edge-matching puzzles
puts the peak at around 17 interior colors: the setting where a puzzle of this
size has on the order of one expected solution. Eternity II uses 17 interior
colors.

## Method

We read the official 256-piece set straight from the engine and look at every
edge color. Color 0 is the grey rim. Each piece is a corner (two grey edges), an
edge piece (one grey edge), or an interior piece (no grey edge). For each non-grey
color we record whether it ever appears on an interior piece, and whether it ever
appears on the frame. That tells us the size of the interior palette and how many
colors are reserved for the border.

## Results

The official set breaks down as:

| | Count |
|---|---:|
| Corner pieces | 4 |
| Edge pieces | 56 |
| Interior pieces | 196 |
| **Interior colors** | **17** |
| **Frame-only colors** | **5** |

The 5 frame-only colors are colors 1 through 5; the 17 interior colors are 6
through 22. Every interior color also shows up on the frame (the border has to
connect to the inside), but the reverse is not true: those 5 border colors never
appear in the interior at all. They are the rare colors, kept to the edge of the
board.

So the headline parameter, 17 interior colors, is real and sits exactly on the
known difficulty peak.

## Why it matters

This is the clearest single sign that Eternity II was engineered to be hard rather
than hard by accident. The board size, the piece count, and the color split all
point at the same target: a puzzle with about one solution, placed at the worst
possible spot for any search to find it. It's the same reason a well-set exam has
questions that are neither trivial nor impossible. The difficulty was chosen.

It also lines up with what you can measure directly. On the Algorithms page, the
work a solver does on generated puzzles climbs steeply as the color count
approaches this range, for puzzles far smaller than the real one. The full 16x16
board sits past the point where that curve has already gone vertical.

## Reproduce

```sh
cd research/topics/phase-transition/compute
cargo run --release > ../results/color-split.json
```

Instant and deterministic; reproduces the committed `results/color-split.json`
byte-for-byte.

## Notes

- "Around 17" is the right way to read the published peak: the transition is a
  band, not a single integer, and the exact location shifts a little with board
  size and with how the frame is counted. The point is that Eternity II's
  parameters land on it rather than off to either side.
- The phase-transition view is a complement to the older result that
  edge-matching is NP-complete in general. NP-completeness says the worst case is
  hard; the phase transition says this particular puzzle was placed in the hard
  region on purpose.
