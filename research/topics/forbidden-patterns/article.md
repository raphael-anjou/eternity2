---
id: forbidden-patterns
title: Forbidden patterns
summary: Almost every small patch of pieces you could build is impossible under the official set — 99.72% of 2x2 squares can never be made to match.
status: published
created: 2026-06-22
updated: 2026-06-22
contributors:
  - name: Raphaël Anjou
    role: author
tags:
  - enumeration
  - structure
sources:
  - label: "Eternity II board viewer (e2.bucas.name)"
    url: https://e2.bucas.name
reproduce:
  - cd research/topics/forbidden-patterns/compute && cargo run --release > ../results/feasibility.json
results:
  - label: Patch feasibility counts (JSON, computed here)
    path: results/feasibility.json
site:
  render: true
  dataFile: results/feasibility.json
---

# Forbidden patterns

> **Created:** 2026-06-22 · **Updated:** 2026-06-22

## Summary

Pick a few of Eternity II's interior pieces and try to fit them together into a
small shape, rotating them however you like. Most of the time you simply can't —
the colors won't line up no matter what you do. For a 2x2 square the odds are
brutal: **99.72% of the ways to choose and place four distinct pieces are
impossible.** Only about 1 in 358 actually works.

That single fact says a lot about why the puzzle is so hard, and it gives you a
cheap way to measure how "healthy" a partly-built board is.

## Background

Eternity II is an edge-matching puzzle: 256 square tiles, each with a color on
all four sides, that you drop into a 16x16 grid so touching sides share a color.
Of the 256 tiles, 196 are *interior* pieces — they have no grey rim edge, so
they're the only ones that ever sit inside the board rather than along the
border. Those 196 are the ones we look at here.

When people first meet the puzzle they imagine the trouble is finding the one
arrangement that works. But there's a more basic kind of trouble underneath
that: most *local* arrangements don't even get off the ground. Two pieces side
by side often have no shared color available between them. Add a third, or close
up a little square, and the situation gets far worse, fast.

## Method

Take a shape made of a few cells. Choose that many **distinct** interior pieces
(a real board never repeats a piece) and assign one to each cell. You're allowed
to rotate each piece freely. The placement is **feasible** if some choice of
rotations makes every shared edge inside the shape match, and **forbidden** if no
choice of rotations works.

We count, exactly and exhaustively, the feasible and forbidden placements for
four shapes:

- **two cells side by side** (horizontal), and **two cells stacked** (vertical),
- an **L of three cells**,
- a **2x2 square** of four cells.

Edges follow the engine's convention: up, right, down, left, with clockwise
rotation. There is no sampling — every distinct-piece placement is checked — so
the numbers are exact and reproduce every time. The 2x2 case means walking all
$196 \times 195 \times 194 \times 193 \approx 1.43$ billion ordered quadruples;
a quick check on the two edges touching the top-left piece throws out almost all
of them before the full match test, which keeps the whole run to about twenty
seconds.

## Results

| Shape | Cells | Placements | Feasible | Forbidden | Forbidden % |
|---|---:|---:|---:|---:|---:|
| Two side by side | 2 | 38,220 | 23,330 | 14,890 | 38.96% |
| Two stacked | 2 | 38,220 | 23,330 | 14,890 | 38.96% |
| L of three | 3 | 7,414,680 | 1,240,852 | 6,173,828 | 83.26% |
| 2x2 square | 4 | 1,431,033,240 | 3,993,696 | 1,427,039,544 | 99.72% |

The jump is the story. Two pieces fail to fit about 39% of the time. Add a third
in an L and you're forbidden 83% of the time. Close the square and 99.72% of all
placements are dead on arrival.

The horizontal and vertical pair counts come out identical, which is the expected
sign of a piece set with no built-in directional bias — the colors are
distributed the same way up-down as left-right.

## Why it matters

A correct, complete board contains **zero** forbidden patches of any shape — by
definition, everything matches. So the number of forbidden patches in a board is
a kind of distance-to-valid: the more of them, the further you are from a real
solution, even when two boards happen to have the same number of matched edges.
Low-scoring boards are riddled with forbidden 2x2 squares; the very best known
boards have only a couple of dozen left. Counting them is an easy progress signal
that doesn't care about your search method.

It also explains, from a different angle, why the puzzle resists clever local
fixes. With 99.72% of small squares impossible, the pieces that *do* fit together
are scarce and specific. There's very little room to shuffle things around
locally without breaking something — the good arrangements are rare and rigid.

## Reproduce

```sh
cd research/topics/forbidden-patterns/compute
cargo run --release > ../results/feasibility.json
```

Deterministic: the output reproduces the committed `results/feasibility.json`
byte-for-byte. About twenty seconds on a recent laptop; the 2x2 sweep is the only
slow part.

## Notes

- We count *ordered* placements (each cell of the shape is a distinct position),
  which is why the two-cell shapes have $196 \times 195 = 38{,}220$ placements
  rather than half that. Forbidden percentages are unaffected by the ordering
  choice.
- Larger shapes (2x3, 3x3) are forbidden essentially always — past a 2x2 there's
  little headroom left to measure. The four shapes here capture the full climb
  from "often fine" to "almost never possible".
