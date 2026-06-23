---
id: piece-theft
title: Piece theft, where solvers die
summary: A cell's (north, west) demand can be served by only ~3 pieces on average, and 47 demands by just one; spending that one piece elsewhere kills a future cell while the board still looks full.
status: published
created: 2026-06-23
updated: 2026-06-23
contributors:
  - name: Raphaël Anjou
    role: author
tags:
  - structure
  - search
sources:
  - label: "Eternity II board viewer (e2.bucas.name)"
    url: https://e2.bucas.name
reproduce:
  - cd research/topics/piece-theft/compute && cargo run --release > ../results/piece-theft.json
results:
  - label: (north, west) server-count distribution (JSON, computed here)
    path: results/piece-theft.json
site:
  render: true
  dataFile: results/piece-theft.json
---

# Piece theft, where solvers die

> **Created:** 2026-06-23 · **Updated:** 2026-06-23

## Summary

A depth-first solver that fills the board top-left to bottom-right gets a few rows
for free, then slams into a wall around the middle of the board. This finding says
why. When the solver reaches a cell, the piece above it and the piece to its left
already fix two of its four colors: a north color and a west color. The cell needs
an unused piece that can show that exact pair. Those demands are brutally scarce:
on average only about three pieces can serve a given (north, west) pair, and 47 of
them can be served by a single piece. So if that one piece was placed somewhere
else earlier, the cell is dead, even though the board still has plenty of pieces
left over. That's piece theft, and it's the dominant way these searches fail.

## Method

Take the 196 interior pieces. A solver moving top-left to bottom-right always knows
a cell's north and west colors before it places there. So we enumerate every
(north, west) pair that can occur and count, for each, how many distinct interior
pieces could serve it in some rotation. No sampling.

## Results

| Servers for a demand | Number of demands |
|---:|---:|
| 1 | 47 |
| 2 | 65 |
| 3 | 73 |
| 4 | 48 |
| 5 | 27 |
| 6 | 5 |
| 7 | 4 |

Across the 269 demands that can occur, the mean is just 2.9 possible pieces, and 47
demands (about one in six) have exactly one. The most flexible demand still has
only seven servers.

## Why it matters

This is the mechanism behind the wall. A board that looks healthy, with most pieces
still in the box, can already be doomed: somewhere back up the board, the only piece
that could ever serve an upcoming cell was used for something else. The damage was
done long ago and is invisible at the moment it happens, because that future cell's
demand wasn't even decided yet.

It also explains why a tempting fix doesn't help. You might hope a global check, "do
the remaining pieces still cover the remaining cells?", would catch these
dead-ends early. It doesn't: globally the supply is fine. The failure is local
misallocation of a scarce piece, not a global shortage, so a global lookahead sees
nothing wrong right up until the cell turns out to have no server.

Put this next to [no forced moves](no-forced-moves): every piece has dozens of
places it *could* go, so the solver is never told where a scarce piece must be
saved, yet each scarce piece has exactly one demand it must be saved for. Freedom
to place, no guidance on where to save. That gap is the trap.

## Reproduce

```sh
cd research/topics/piece-theft/compute
cargo run --release > ../results/piece-theft.json
```

Exact and deterministic; reproduces the committed `results/piece-theft.json`
byte-for-byte.

## Notes

- We count distinct pieces per demand. Counting placements (piece and rotation
  together) gives the same demand totals here, because a piece rarely serves the
  same (north, west) pair in two rotations.
- "Demand" is the (north, west) pair a cell needs; the 269 figure is how many such
  pairs actually occur among the interior pieces, out of 17 × 17 = 289 conceivable
  ordered pairs of interior colors.
