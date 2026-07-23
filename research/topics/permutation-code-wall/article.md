---
id: permutation-code-wall
title: A coding-theory lens on the puzzle, and the one identity it rests on
summary: WEFT reads a full Eternity II board as an error-correcting codeword whose 480 internal adjacencies are parity checks, so the matched-edge score is 480 minus the syndrome weight. Most of that lens is a reframing that adds no new number, but three counted facts underneath it are exact and reproducible on the official instance (480 checks, 256 distinct pieces forming a permutation code, five frame colors as a border code), and the load-bearing identity score = 480 minus breaks reproduces bit-for-bit through the kit scorer on genuine codewords with known injected break counts, including the exact 451 = 480 minus 29 pairing the paper reports for its champion.
status: draft
created: 2026-07-23
updated: 2026-07-23
contributors:
  - name: Raphael Anjou
    role: author
credits:
  - Reproduction of the vol-228 WEFT paper's coding-theory / error-correcting-decoder lens on Eternity II.
tags:
  - structure
  - theory
  - coding-theory
sources:
  - label: Eternity II piece set (official instance)
    url: https://e2.bucas.name/
repro:
  tier: exact
  cmd: cd research/topics/permutation-code-wall/compute && cargo run --release > ../results/permutation_code_wall.json
  scope: Reproduces the three counted structural facts on the official 16x16 instance (480 checks three independent ways, 256 pieces distinct up to rotation = permutation code, 5 frame colors with gray a pure rim marker = border code) and the score = 480 minus breaks identity on genuine 480 codewords with exactly-known injected break counts (k in {0,1,2,5,10,29,60,120} x 5 seeds = 40 rows, all hold; k=29 lands on 451). Deterministic, sub-second. The paper's board-specific numbers (the 451 board's syndrome map, min-distance >= 4, the BP fixed point, the 29 -> 19 scarcity gap) require a private champion board and are NOT reproduced; the coding-theory vocabulary is a reframing, flagged as such.
reproduce:
  - cd research/topics/permutation-code-wall/compute && cargo run --release > ../results/permutation_code_wall.json
results:
  - label: Results JSON (structural facts + 40-row identity sweep; deterministic)
    path: results/permutation_code_wall.json
site:
  render: false
  dataFile: null
---

# A coding-theory lens on the puzzle, and the one identity it rests on

> **Status:** draft · **Created:** 2026-07-23 · **Updated:** 2026-07-23
> **Authors:** Raphael Anjou

## Summary

The WEFT paper reads a completed Eternity II board as a codeword in an
error-correcting code. Each of the 480 internal adjacencies is a parity-like
check that either matches or breaks; the vector of broken checks is the
syndrome; and the matched-edge score is exactly 480 minus the syndrome weight.
On top of the checks sit two hard constraints the paper names a border code
(gray confined to the outward rim) and a permutation code (the 256 tiles are a
permutation of the 256 distinct pieces, each used once). The paper's thesis is
that the permutation code, not the edge matching, is what makes the puzzle hard.

Read carefully, the lens has two very different kinds of content. Most of it is
a reframing: codeword, syndrome, minimum distance, decoder are new names for the
ordinary matched-edge score and its neighborhood, and they introduce no new
number. Underneath that vocabulary sit three counted facts that are genuine,
exact, and checkable on the official instance. This reproduction separates the
two, reproduces the exact parts, and confirms the one identity the whole lens
rests on directly through the kit's canonical scorer.

The identity is the load-bearing claim. It reproduces bit-for-bit: on genuine
480-out-of-480 codewords we inject an exactly-known number of broken checks and
the canonical scorer returns 480 minus that number every time, across forty
independent cases, including the case of 29 breaks, where the score is 451, the
exact score-and-syndrome pairing the paper reports for its champion board.

## What is a reframing, and what is a measured fact

Stated plainly, because the distinction is the point of this reproduction.

A reframing, adding no new number: the codeword and syndrome vocabulary, and the
identity score equals 480 minus the syndrome weight. The syndrome weight is the
count of unmatched interior edges. The kit already calls that count breaks and
already defines it as the maximum score minus the achieved score. So the
paper's central identity is not a discovery about the puzzle; it is the
definition of the score, rewritten in coding language. The value of the reframing
is conceptual, not numerical, and this reproduction treats it as such.

A measured structural fact, exact and refutable: there are 480 checks, there are
256 pieces distinct up to rotation, and there are exactly five frame colors that
never touch an interior piece. These are real properties of the official
256-piece design. They are what give the coding lens something concrete to
stand on, and they reproduce exactly.

Out of scope for a kit reproduction: the paper's board-specific results, its
Sections 2 through 5, all live on one particular champion board that the public
starter kit does not ship. The 451 board's per-cell syndrome map, the
minimum-distance-at-least-four sweep, the belief-propagation fixed point, and
the 29-to-19 scarcity gap are not re-derived here. The only paper number this
reproduction crosses is the identity 480 minus 29 equals 451, checked against
the paper's own reported score and break count.

## Reproduction

Everything runs through the starter kit's single canonical scorer, the same
rim-excluding scorer the site and every engine use, so a break here is the same
break the paper means. The checker in `compute/` is one Rust binary with no
sampling and no solver.

For the structural facts it loads the official instance, enumerates the scorer's
own adjacency list and cross-checks the count against the closed form 2WH minus
W minus H and against the kit's maximum-score constant, checks that all 256
pieces are distinct up to rotation (the permutation code), and derives the frame
colors from the data rather than assuming their labels (the border code),
confirming gray never appears on an interior piece.

For the identity it builds genuine 480-codeword boards from the seeded solved
generator, confirms each starts at score 480 with an empty syndrome, then
injects an exactly-known number of broken checks by corrupting one facing
half-edge per step to a color that appears on no piece, accepting a step only
when it drops the score by exactly one. That per-step check is what makes the
injected break count a known quantity rather than an inferred one: k breaks
means exactly k violated checks, each verified individually. The binary then
confirms the identity three ways per case, and re-counts the syndrome
independently as a cross-check.

See `compute/PLAN.md` for the claim-by-claim map, the scoring-convention note,
and the plain reframing-versus-fact split.

### Measured results

Run on Apple Silicon, single core, deterministic, under one second. Committed as
`results/permutation_code_wall.json`, byte-stable on rerun.

Structural facts on the official 16x16 instance, all exact:

| Fact | Expected (WEFT) | Measured |
|---|---|---|
| Check count | 480 | 480, three ways: formula 2WH minus W minus H = 480, enumerated adjacency list 480, kit MAX_SCORE 480 |
| Permutation code | 256 tiles, each used once | 256 pieces, all distinct up to rotation |
| Border code, frame colors | five, confined to the border | 5 frame colors (colors 1..5), 17 interior colors |
| Border code, gray | gray on outward rim only | 0 gray half-edges on any interior piece |

The syndrome-and-score identity, on genuine 480 codewords with exactly-known
injected break counts (five generator seeds, eight break counts, forty rows,
every row holds):

| injected breaks k | score | breaks | independent syndrome recount | 480 minus k |
|---|---|---|---|---|
| 0 | 480 | 0 | 0 | 480 |
| 1 | 479 | 1 | 1 | 479 |
| 2 | 478 | 2 | 2 | 478 |
| 5 | 475 | 5 | 5 | 475 |
| 10 | 470 | 10 | 10 | 470 |
| 29 | 451 | 29 | 29 | 451 |
| 60 | 420 | 60 | 60 | 420 |
| 120 | 360 | 120 | 120 | 360 |

Every row satisfies score equals 480 minus k, breaks equals k, and the
independent syndrome recount equals k, on all five seeds. The row at k equals 29
reads score 451, which is exactly the score-and-syndrome pairing the paper
reports for its champion (breaks 29, score 451): the identity 480 minus 29
equals 451 holds against the paper's own numbers.

## Verdict

Reproduced, with the scope stated. The three counted structural facts underneath
the lens are exact on the official instance. The identity the whole coding-theory
reading rests on reproduces bit-for-bit through the canonical scorer, on genuine
codewords, at every injected break count including the paper's own 29, and its
independent syndrome recount agrees on all forty rows. What is not reproduced is
flagged honestly: the coding vocabulary is a reframing rather than a new number,
and the paper's board-specific measurements need a champion board the public kit
does not carry, so they are out of scope here rather than confirmed or denied.

## Notes / caveats

The identity is exercised on solved generated boards, which are valid 480
codewords, rather than the official instance's specific solution, which the kit
does not ship. This is faithful for an identity that is a property of the scorer
and independent of which codeword is chosen; it simply means the identity arm
does not touch the official solution, while the three structural facts do run on
the official instance. The paper's stronger claim, that the permutation code
carries the puzzle's hardness while the edge-matching code is weak, is a
board-specific measurement (its scarcity gap) that this kit reproduction does
not attempt; nothing here confirms or refutes it.
