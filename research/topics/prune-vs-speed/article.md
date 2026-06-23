---
id: prune-vs-speed
title: Prune beats speed
summary: For E2-class search, shrinking the search space is an exponential lever while raw speed is only a constant one — and the puzzle is engineered so the space barely shrinks.
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
  - label: "Ansótegui et al. — phase transition in edge-matching puzzles"
    url: https://www.iiia.csic.es/~jordi/
reproduce:
  - cd research/topics/prune-vs-speed/compute && cargo run --release > ../results/prune-vs-speed.json
results:
  - label: Hardness curve + lever comparison (JSON, computed here)
    path: results/prune-vs-speed.json
site:
  render: true
  dataFile: results/prune-vs-speed.json
---

# Prune beats speed

> **Created:** 2026-06-23 · **Updated:** 2026-06-23

## Summary

There are two ways to do less work in a backtracking search: go **faster**
(a better engine, more cores) or make the search tree **smaller** (prune
branches that can't lead to a solution, lowering the effective branching
factor). They sound interchangeable. They are not even close — and which one
matters is the difference between fifteen years of effort and a fast machine.

## The maths

A depth-first search of a tree with branching factor $b$ and depth $d$ visits
on the order of $b^d$ leaves.

- A **speedup** of $k\times$ divides the wall-clock by $k$: a *constant* divisor,
  the same whether the tree is ten levels deep or ten thousand.
- A **prune** that lowers the effective branching factor from $b$ to $b'$ divides
  the work by $(b/b')^d$: an *exponential* divisor in the depth.

For E2's $d = 256$, even a few percent off $b$ dwarfs any speedup a real machine
can offer. A 5% cut to the branching factor, applied at every level, is worth
$(1/0.95)^{256} \approx 5\times10^5$ — a five-hundred-thousand-fold speedup's
worth of work, from one cheap structural idea.

So the ordering is: **prune first, then go fast inside the reduced space.** Speed
is not worthless — it lets you run more, deeper pruned searches — but it is never
where the gap to 480 is hiding.

## What we measured

`compute/src/main.rs` makes this concrete with the project's own depth-first
solver, on small generated puzzles (so it runs in seconds and is deterministic):

1. **The hardness curve.** For a fixed board, solve generated puzzles across
   colour counts and record the median nodes explored. The curve rises to a
   peak (the SAT/CSP phase transition — about one expected solution) and falls.
   It is the search *space*, not the clock, that explodes — and Eternity II's
   17 interior colours sit on the analogous peak.
2. **The lever comparison.** From the measured node count at the peak we derive
   the effective branching factor, then report what a generous $1000\times$ raw
   speedup buys versus what a modest 5% prune buys. The prune wins by orders of
   magnitude — and the gap grows astronomically at the 16×16's depth of 256.

See `results/prune-vs-speed.json` for the exact numbers.

## Why E2 is the hard case of this principle

The principle says "reduce the search space." The puzzle's design is precisely
an attack on your ability to do that — four walls that are, underneath, the same
statement *there is nothing local to prune on*:

- **No forced moves** — every interior cell keeps 73–137 legal neighbours, so
  propagation almost never collapses a cell to one choice.
- **On the hardness peak** — no solution-dense region to aim a statistical
  shortcut at (the trick that cracked Eternity I).
- **The area law** — distinct partial boards collapse past ~80 cells, but no
  local signal sees that global collapse, so you can't prune toward it cheaply.
- **Rigidity** — even at a record board the move to a better one is huge and
  indivisible; no gradient to follow.

The lever that matters is the lever the puzzle denies. This is why a 232× faster
engine made the same search cheaper, not smaller, and didn't move the record;
and why every method that *did* move the needle changed the **shape** of the
search instead.

## Caveat

The branching-factor and depth in the demo's tree-size estimate are illustrative
(chosen to be E2-like and legible), not a measurement of a specific solver. The
hardness curve and node counts, however, are real measurements from the engine.
The principle itself — constant divisor versus exponential divisor — is exact.
