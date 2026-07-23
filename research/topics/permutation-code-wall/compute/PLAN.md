# Reproduction plan: permutation-code-wall

> **Repro tier: exact (structural facts + score/syndrome identity), qualitative
> (the coding-theory framing itself).**
> exact = re-run reproduces committed results byte-for-byte. seeded-statistical =
> committed results are the record of our run (hardware noted in the article);
> reproduction = fresh run agreeing within the stated tolerance. qualitative =
> the effect (sign/ordering/band) reappears; exact digits are lineage-bound.

Source of the claim: research repo, `research/vault/papers/vol-228/WEFT.md`
(vol-228 research paper, family WEFT — the error-correcting-decoder lens). This
plan cites the exact WEFT statements the checker must reproduce and states
plainly which parts are a **reframing** (no new number) and which are a
**measured structural fact**.

## What WEFT is, and what is reproducible

WEFT models a full Eternity II board as a **codeword**. Each of the 480 internal
adjacencies is a parity-like **check**; the **syndrome** `sigma in {0,1}^480`
marks the violated checks; a break is a violated check; and
`score = 480 - ||sigma||_1`. Two hard structural constraints sit on top of the
checks: the **border code** (gray = color 0 confined to the outward rim sides)
and the **permutation code** (the 256 tiles are a permutation of the 256
distinct pieces, each used once).

The reproducible substance splits cleanly:

- The **codeword / syndrome / decoder vocabulary** is a *reframing* of the
  ordinary edge-matching score. It introduces no new number; it renames
  "unmatched interior edges" as "syndrome weight" and "matched-edge score" as
  "480 minus syndrome weight". We reproduce it by showing the kit's own
  canonical scorer already satisfies exactly this identity, bit for bit.
- The **counted structural facts** WEFT relies on (480 checks, 5 border colors,
  256 distinct pieces) are exact properties of the official instance and are
  recomputed here from the data.
- The **WEFT-specific measurements** (the 451 board's 29-break syndrome, the
  min-distance >= 4 sweep, the BP fixed point, the scarcity gap 29 -> 19) are
  properties of one specific champion board (`board451.url`) that is NOT in the
  public kit. Those numbers are NOT re-derived here; the one WEFT number this
  kit reproduction touches is the identity `480 - 29 = 451` itself, checked
  against WEFT's own reported score/break pair. Everything else in Sections
  2-5 of WEFT is out of scope for a kit reproduction and is flagged as such in
  the article's caveats.

## (a) The exact claims and expected numbers

**Group A — structural facts (exact; any deviation refutes):**

| # | Fact (WEFT) | Expected value |
|---|-------------|----------------|
| A1 | Check count: the 480 internal adjacencies, `|{e}| = 480` | 480, three independent ways: `2WH - W - H`, the scorer's enumerated adjacency list, and the kit's `MAX_SCORE_16` |
| A2 | Permutation code: 256 pieces, each a distinct tile (used once) | 256 pieces, all distinct up to rotation |
| A3 | Border code: gray on outward rim only; frame colors confined to the border | exactly 5 frame colors (colors absent from every interior piece), gray on 0 interior half-edges |

**Group B — the syndrome / score identity (exact, measured through the scorer):**

The load-bearing reproduction. On a genuine 480/480 codeword (a solved generated
16x16/22 board), inject EXACTLY k broken checks and confirm the canonical scorer
reports the identity:

| # | Claim | Expected |
|---|-------|----------|
| B1 | `score = 480 - k` and `breaks = k` for injected break count k | holds for k in {0,1,2,5,10,29,60,120} across 5 generator seeds (40 rows) |
| B2 | An independent syndrome recount `||sigma||_1` agrees with breaks | `syndrome_weight == k` on every row |
| B3 | WEFT's own champion numbers obey the identity | `480 - 29 = 451` and `480 - 451 = 29` |

k = 29 is included deliberately: it is WEFT's champion's break count, so row
`k=29` reads `score = 451`, reproducing the exact score/syndrome pairing WEFT
reports for its board, on a generic codeword.

## (b) Scoring-convention mapping

The kit's canonical `score_cells` counts the 480 internal joints of the 16x16
board and excludes the 64 perimeter (gray) rim slots; a right/down adjacency
scores 1 iff the two facing half-edges are equal AND non-border. WEFT uses the
identical convention (480 internal checks, gray never scores), and its
"break" == the kit's `breaks = MAX_SCORE - score` == the kit's unmatched
interior edge count == `||sigma||_1`. These are three names for one quantity;
the reproduction demonstrates that equality rather than assuming it.

This topic makes no record claim, so no strict-5/5 vs matched-edges distinction
is needed. WEFT's champion is a matched-edges 451 (breaks=29); the only WEFT
number crossed here is the identity that ties those two together.

## (c) Scale / instance faithfulness

The three structural facts (A1-A3) are properties of the official 256-piece
16x16 design and are checked on `official_instance(true)` — the real instance,
not a generated small board. The syndrome/score identity (B) is a property of
the scorer and holds on any 16x16 board; it is exercised on genuine 480/480
codewords from `generator::generate_solved(16, 22, seed)` so that the "480
minus k" arithmetic starts from a real zero-syndrome codeword. Both run at the
official 16x16 shape; nothing is scaled down, and the whole check finishes in
under a second.

Fidelity caveat carried forward: the identity is injected on a *solved
generated* board (a valid 480 codeword) rather than the exact official-instance
solution, which the kit does not ship. This is faithful for the identity (a
scorer property independent of which codeword) but means B does not touch the
official instance's specific solution. A1-A3 do run on the official instance.

## (d) Exact reproduction steps

1. `cd research/topics/permutation-code-wall/compute`
2. `cargo run --release > ../results/permutation_code_wall.json`
   (creates `results/` next to `article.md`; runtime well under one second)
3. Compare the emitted JSON against the tables above. Every field in the
   `structural` block must match exactly; every row of
   `syndrome_score_identity.rows` must have `identity_score_eq_480_minus_k`,
   `identity_breaks_eq_k`, and `syndrome_agrees` all true; `all_rows_hold` must
   be true and `weft_451_board_identity.identity_holds` must be true.
4. The binary exits nonzero and prints the first failing clause if any
   deterministic check is violated.

RNG: the injection order uses the kit generator's `XorShift` with a fixed
per-seed key, so reruns are bit-reproducible. The board itself
(`generate_solved`) is deterministic in the seed, so the committed JSON is
byte-stable on rerun.

## Checker structure

`src/main.rs`, single file, deps `e2-kit` (path dep on the starter kit) and
`serde_json`:

- **A.** load `official_instance(true)`; enumerate the scorer's adjacency list
  and cross-check against `2WH - W - H` and `MAX_SCORE_16`; check
  `pieces_distinct_up_to_rotation` and piece count 256 (permutation code);
  derive frame colors from the data and count gray on interior pieces (border
  code).
- **B.** for each seed, build a solved 480 codeword, verify it is score 480 /
  syndrome 0, then inject k breaks by corrupting one half-edge per step to a
  sentinel color that appears on no piece, accepting a step only when the score
  drops by exactly 1 (so k is a KNOWN, verified break count). Confirm the
  identity `score = 480 - k`, `breaks = k`, and an independent syndrome recount.
- print one JSON object to stdout; nonzero exit on any deterministic failure.

## What is a reframing vs a measured fact (stated plainly)

- **Reframing (no new number):** the codeword / syndrome / minimum-distance /
  decoder vocabulary, and the identity `score = 480 - ||sigma||_1`. This is the
  ordinary matched-edge score wearing coding-theory clothes; the reproduction
  shows the kit scorer already embodies it exactly.
- **Measured structural facts (exact):** 480 checks, 256 distinct pieces
  (permutation code), 5 frame colors with gray as a pure rim marker (border
  code). These are real, exhaustively checkable properties of the official
  instance, reproduced here.
- **NOT reproduced (out of scope for the kit):** WEFT's board-specific
  Sections 2-5 — the 451 board's syndrome map, min-distance >= 4 sweep, BP
  fixed point, and the scarcity gap 29 -> 19 — which require the private
  `board451.url` champion and its solver pipeline. Flagged in the article.
