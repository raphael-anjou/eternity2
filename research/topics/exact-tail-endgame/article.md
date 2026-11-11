---
id: exact-tail-endgame
title: The bottom-band endgame as its own exact problem
summary: >-
  Exact CP-SAT re-solve of the last one or two rows of a greedy-produced, clue-valid 16x16 board, with the top frozen and the tail's own pieces as the pool, under three incoming-break regimes (cap 1 per cell, at most one double-break cell, uncapped). At one row the uncapped exact tail beats the producer on 24/24 seeds by 1 to 9 matched edges (median 4.5); allowing a single double break gives the same optimum on all 24 seeds, while cap 1 is infeasible on 5.
status: draft
created: 2026-07-23
updated: 2026-11-11
contributors:
  - name: Raphael Anjou
    role: author
credits:
  - Vol-228 TAILFORGE and vol-234 R3 comb-squeeze research notes, which posed and measured the exact-tail endgame question.
tags:
  - exact-methods
  - constraint-programming
  - endgame
  - finishability
sources: []
reproduce:
  - cd research/topics/exact-tail-endgame/compute && uv run tailforge.py --seeds 24 --rows 1,2 --caps 1,db1,none --band-seeds 1:24,2:4 --time-limit 45 --workers 8 --out ../results/tailforge.json
results:
  - label: Exact-tail vs producer-tail, per (seed, band, cap) with delta distribution (JSON)
    path: results/tailforge.json
site:
  render: false
  dataFile: null
---

# The bottom-band endgame as its own exact problem

> **Status:** draft · **Created:** 2026-07-23 · **Updated:** 2026-11-11
> **Authors:** Raphael Anjou

## Summary

Exact CP-SAT re-solve of the last one or two rows of a greedy-produced, clue-valid 16x16 board, with the top frozen and the tail's own pieces as the pool, under three incoming-break regimes (cap 1 per cell, at most one double-break cell, uncapped). At one row the uncapped exact tail beats the producer on 24/24 seeds by 1 to 9 matched edges (median 4.5); allowing a single double break gives the same optimum on all 24 seeds, while cap 1 is infeasible on 5.

The greedy producer maximises incoming (top + left) matches per cell and accepts a double
incoming break when nothing better is left: every one of its 24 one-row tails contains 1 to 6
double-break cells. Cap 1 is therefore stricter than the producer, and its infeasibility on 5 seeds
means those 16 pieces admit no arrangement without a double break. Each solve is hinted with the
producer's tail and, where that tail is admissible under the regime, floored at its value, so the
uncapped delta is non-negative by construction; capped deltas are non-negative in every run observed.

Not separated by this experiment: exact vs a cheap non-exact finisher (no local-search control),
and tail-tight vs top-limited (no piece exchange between top and tail, no larger window).

## Reproduction

```sh
just research-exact-tail-endgame
```

24 producer seeds at one row, 4 at two rows, regimes `1,db1,none`, 45 s and 8 CP-SAT workers per
solve (about 10 minutes in total). One-row solves are proven optimal and reproduce exactly; two-row
solves are time-limited feasible incumbents (lower bounds) and reproduce in sign and shape.

### Measured results

| configuration | n | feasible | delta min / median / max | mean |
| --- | --- | --- | --- | --- |
| 1 row, cap 1 | 24 | 19 | 0 / 4 / 8 | 3.95 |
| 1 row, one double break | 24 | 24 | 1 / 4.5 / 9 | 4.75 |
| 1 row, uncapped | 24 | 24 | 1 / 4.5 / 9 | 4.75 |
| 2 rows, cap 1 | 4 | 4 (not proven) | 6 / 12.5 / 17 | 12.0 |
| 2 rows, one double break | 4 | 4 (not proven) | 7 / 12 / 17 | 12.0 |
| 2 rows, uncapped | 4 | 4 (not proven) | 7 / 11.5 / 16 | 11.5 |

Source: `results/tailforge.json` (per-record fields include `producer_tail_double_breaks` and
`producer_tail_admissible`); stderr log in `results/run.log`.
