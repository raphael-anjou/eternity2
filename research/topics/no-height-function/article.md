---
id: no-height-function
title: Why the puzzle has no height function
summary: A dimer-style height function would turn color-matching into a smoothness law and give breaks a conserved oriented charge annihilable in pairs. Two direct measurements on committed high-scoring boards kill it: the break set has tens of odd-degree dual cores (breaks are open strings, not dual cycles) and the signed per-color current fails to globally conserve. The only surviving invariant is an unsigned per-color parity bit, not a Burgers vector.
status: draft
created: 2026-07-23
updated: 2026-07-23
contributors:
  - name: Raphael Anjou
    role: author
credits:
  - Reformulation of the E2 phase space in crystal-defect language (dual height, dislocation cores, Burgers current) from the vol-228 physics sub-wave.
tags:
  - structure
  - theory
  - defects
sources:
  - label: Eternity II piece set (official instance)
    url: https://e2.bucas.name/
repro:
  tier: qualitative
  cmd: cd research/topics/no-height-function/compute && cargo run --release > ../results/no_height_function.json
  scope: Measures both break-topology obstructions (odd dual cores; non-conserved signed per-color current) on four committed high-scoring official-instance boards (scores 463/460/458/460); deterministic, sub-second. Our boards differ from the paper's, so exact core counts and current sums differ; the reproduced object is the QUALITATIVE law, not the paper's integers.
reproduce:
  - cd research/topics/no-height-function/compute && cargo run --release > ../results/no_height_function.json
results:
  - label: Results JSON (per-board cores + non-conserved currents; deterministic)
    path: results/no_height_function.json
site:
  render: false
  dataFile: null
---

# Why the puzzle has no height function

> **Status:** draft · **Created:** 2026-07-23 · **Updated:** 2026-07-23
> **Authors:** Raphael Anjou

## Summary

There is a beautiful idea from statistical physics that, if it worked, would
reshape how we attack Eternity II. In dimer and six-vertex models a tiling
carries a **height function**: a scalar living on the faces that steps up or
down by a fixed amount across each edge. When the tiling is perfect the height
is single-valued; a defect is a point of nonzero holonomy, a **Burgers
vector**, and defects are created and destroyed only in charge-conserving
pairs. That structure is what makes worm and directed-loop updates possible:
moves that are non-local on the lattice but local in height space, sliding a
defect pair together until they annihilate.

If Eternity II had such a height, a mismatched joint (a **break**) would be a
dislocation with a conserved oriented charge, and there would be a principled
non-local move that lowers the break count by cancelling a pair. This note
records why that hope fails. The failure is not a matter of taste; it is two
concrete, checkable facts about the break set of any real board, and the
checker in `compute/` measures both on our committed record boards.

**First obstruction: breaks are open strings, not closed loops.** Put the
height increment on a joint exactly when it is broken. Then the holonomy around
a dual vertex (a grid corner) is the number of incident broken joints modulo
two, and the height is single-valued only if every dual vertex has even
break-degree, i.e. only if the break set is a union of dual cycles. It is not.
On every board we measured, tens of dual vertices have **odd** break-degree.
These are dislocation cores, the endpoints of open break-strings. A height
cannot close around them.

**Second obstruction: the oriented charge is not conserved.** Resolve by color
to try to save it. For a color, orient each half-broken joint from the
color-carrying cell toward its non-matching neighbor and sum the resulting
per-cell vectors. A genuine conserved current would sum to zero over the whole
board. It does not: on every board, several colors have a nonzero global sum.
The reason is structural. On a solution every color edge is matched to an edge
of the same color, so there are no half-broken joints and the current is
trivially zero; a half-broken joint is an unpaired half-edge, and its signed
contribution has nothing to cancel against.

What survives is far weaker than a height. The only conserved topological
invariant of E2 breaks is an **unsigned per-color parity** — one bit per color,
an element of a 22-dimensional space over GF(2) — not an integer or a
two-dimensional Burgers vector. A bit is not the holonomy of any height
function. So the dimer/six-vertex height program is closed from the defect
side, and with it the dream of a clean worm move that walks defects to
annihilation. (The physical intuition it leaves behind, that the right escape
from a local optimum is a correlated cluster move rather than a single-site
one, is a separate and more positive story.)

There is also a third, purely algebraic obstruction that needs no board at all:
a per-face scalar height telescopes to zero holonomy around every interior
grid vertex, identically, whatever the colors and whatever is broken. The four
face values around a corner enter the loop sum once with each sign and cancel,
so a scalar height is blind to breaks before we even ask about conservation.
That one is a one-line identity and is stated here rather than measured.

## Reproduction

**This is a qualitative reproduction, and deliberately so.** Both measured
quantities are functions of the specific board. The source paper measured them
on three particular boards it happened to have (a 451, a 463, a 458); we ship
four different committed record boards (scores 463, 460, 458, 460, from
`topics/record-boards/boards.json`). The exact core counts and current sums
therefore differ from the paper's by construction, and we never present our
integers as the paper's. What reproduces is the **law**: on every board, odd
dual cores exist and are numerous, and at least one per-color current has a
nonzero global sum. Either failing would admit a height function on that board;
the checker hard-fails if it ever sees zero cores or a fully conserved current.

The checker loads each board through the starter kit's `parse_board_edges`,
re-scores it with the canonical `score_cells` (refusing any board whose
re-scored value disagrees with its committed score, so the topology is always
measured on a board of known, verified quality), then makes one linear pass to
count odd-degree dual cores and non-conserved per-color currents. See
`compute/PLAN.md` for the claim-by-claim mapping, the scoring-convention note,
and why the obstruction must be checked on real high-scoring official-instance
boards rather than a generated small board (on which it would be either vacuous
or off-instance).

Measured results (committed as `results/no_height_function.json`, byte-stable
on rerun; no RNG; Apple Silicon, single core, runtime well under one second):

| Board (ours) | Verified score | Breaks | Odd dual cores | Colors with nonzero current |
|---|---|---|---|---|
| v129-palimpsest-463 | 463 | 17 | 22 | 10 |
| v181-keyring-460 | 460 | 20 | 22 | 9 |
| v175-gauntlet-458 | 458 | 22 | 34 | 15 |
| v155-prior-460 | 460 | 20 | 26 | 12 |

For orientation, the paper reported **28 / 22 / 32** odd cores on its own 451 /
463 / 458 boards; our four boards land at **22 / 22 / 34 / 26**, squarely in the
same regime. Our best board (463, only 17 breaks from a hypothetical solution)
still carries 22 open-string cores and 10 non-conserved color currents — the
obstruction does not soften as the board approaches a solution, which is the
whole point. And a small coincidence worth flagging: the paper's headline
non-conserved current was color 6 summing to (-1, -3) on its 451 board; our
v155-prior-460 board shows color 14 summing to (1, -3), the same magnitude
regime on an independent board. The specific colors and signs are lineage-bound
labels; the nonzero-ness is the law.

**Verdict: REPRODUCED (qualitative tier).** Both obstructions reappear on every
board we ship, with per-board magnitudes inside the paper's regime. No height /
Burgers-vector function exists for these boards, exactly as the source claims.

## Notes / caveats

The exact core counts and per-color current sums are **our boards'** numbers,
not the paper's; they are meant to be read as "in the same regime as", never as
a bit-for-bit reproduction of the source table. The color labels attached to
non-conserved currents depend on the instance's color numbering and carry no
independent meaning; only the count of colors with a nonzero sum, and the
sums' nonzero-ness, are the reproduced content. The third obstruction (a scalar
height telescopes to zero holonomy) is a proved identity and is asserted here
rather than recomputed. The positive companion finding from the same source —
that a correlated two-tile co-rotation can cross the single-swap barrier that a
height-guided worm move was meant to cross — is a separate result and is not
measured in this topic.
