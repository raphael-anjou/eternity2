# Reproduction plan: exact-tail-endgame

> **Repro tier: seeded-statistical (per-config delta distribution across producer
> seeds; the sign and shape of "exact tail vs producer's own tail" is the claim).**
> exact = re-run reproduces committed results byte-for-byte. seeded-statistical =
> committed results are the record of our run (hardware noted in the article);
> reproduction = fresh run agreeing within the stated tolerance. qualitative =
> the effect (sign/ordering/band) reappears; exact digits are lineage-bound.

Class: **exact-methods** (an exact optimisation of a sub-board, used as an
instrument to decide whether the tail is the binding constraint on the score).

## (a) The exact claim, with expected numbers

Sources: research vault `vault/papers/vol-228/TAILFORGE.md` and
`vault/papers/vol-234/R3-COMB-SQUEEZE.md`.

1. **The tail band is exactly solvable as a small constraint problem.** Given a
   frozen top half, the bottom band (a handful of rows) is a quadratic
   (bilinear) assignment: the objective has genuine free-free product terms, so
   it is not a linear/Hungarian assignment, but it IS small enough for CP-SAT to
   close in seconds. TAILFORGE: a 21-cell tail closes in 0.3 s; a 35-cell tail in
   13-160 s; naive Rust DFS B&B is hopeless (3.2e9 nodes, only reaches 444).
2. **The exact tail can beat the producer's own tail on the identical frozen
   top.** TAILFORGE: `bf_finisher --pin-depth 220` scores 451; exact CP-SAT on
   the identical 35-cell free set scores **452**, proven optimal (+1, the record
   was tail-limited). But the direction is regime-dependent:
   - A narrow (empties-only, ~21-cell) tail is **producer-limited**: exact
     completion ties or barely moves the producer (TAILFORGE 448 ceiling).
   - A wider seam-inclusive band (the producer's last 1-2 rows) is where exact
     optimisation extracts real points.
3. **A comb / wide-beam producer's tail can be TIGHT (un-squeezable).** R3-COMB:
   the column-aligned teeth region of a wide-beam comb board is **proven OPTIMAL
   at the base score** (delta 0) - the producer already exact-optimised its
   frontier. The headroom is the frozen-seam relaxation, not the teeth. So the
   answer to "is the tail the binding constraint?" is **producer-dependent**.
4. **Double incoming-breaks are load-bearing at cap>=2.** TAILFORGE: on a 35-cell
   tail, cap=1 -> 448, cap=2 -> 449; and on one 22-cell tail cap=1 is INFEASIBLE
   while cap=2 -> 448. A single double incoming-break buys a real point that the
   <=1-break/cell producer cannot build.

Composite claim to reproduce, small scale: *for a strong producer board with a
frozen top, the bottom band is exactly optimisable by CP-SAT in seconds; the
exact-tail full-board score is >= the producer's own score (delta >= 0), and
whether the delta is strictly positive (tail-limited) or zero (tail-tight) is a
producer/depth property, not a universal.* Cap=1 vs cap=2 quantifies the
double-break lever.

## (b) Scoring-convention mapping

- Scorer is the kit's canonical **rim-excluding interior matched edges**; max on
  a full 16x16 board is 480. Identical to `score_cells` and to the published
  `tail-finishability-frostline` topic on the same official piece set.
- Producer score = that scorer applied to the greedy row-major board.
- Exact score = that same scorer applied to the board after the tail band is
  replaced by the CP-SAT optimum (frozen top unchanged).
- **Delta = exact_score - producer_score** is the finding's headline quantity.
- The **incoming-break cap** mirrors the producer's per-cell regime exactly:
  a free cell's incoming edges are its TOP (from cell-N) and LEFT (from cell-1);
  `conf(c) = [top mismatch] + [left mismatch] <= cap`. cap=1 is the
  `<=1-break/cell` regime; cap=2 admits a double incoming-break.
- **Hints:** the 5 canonical hints from `official.json` are pinned and **never
  freed**, so every board is strict-5-safe by construction (the strict-5/5
  analogue). The verifier re-checks hint conformance independently.

## (c) Scale-faithfulness

This is a **small-scale** reproduction, deliberately scoped to stay under ~20
minutes, and the article states what it does and does not settle.

- **Producer.** The source used bespoke record-track producers (comb / wide-beam
  / ledger-campaign champions at 422-457). This topic uses the simpler seeded
  **greedy row-major** filler (the frostline producer, verbatim) on the official
  16x16 set. Its boards score lower (mid-400s), so the ABSOLUTE numbers (448/449/
  451/452) are NOT expected to reproduce byte-for-byte. What reproduces is the
  MECHANISM: the tail band is exactly solvable in seconds, and the exact-tail
  score relates to the producer's own tail as the sources describe (delta >= 0,
  sometimes strictly positive, sometimes tail-tight; cap=2 >= cap=1).
- **Band height.** Small tail bands (2-3 rows = 32-48 free cells) to keep each
  CP-SAT solve in the seconds-to-tens-of-seconds range. The source's own
  region-size ladder shows >~48-cell windows stop converging in a short budget,
  so 2-3 rows is both tractable and the productive window.
- **What the small-scale run settles:** that the bottom-band endgame IS an exact
  small constraint problem (CP-SAT closes it), that exact optimisation never
  loses to the producer (delta >= 0 always, by construction the exact solve
  dominates the greedy tail), the sign/shape of the delta across seeds, and that
  cap=2 dominates cap=1. **What it does NOT settle:** whether exact tailing moves
  a genuine RECORD (that needs a record-track producer at 455+, not a greedy
  mid-400s board), and the exact TAILFORGE digits (lineage-bound to their
  producers). Those are stated as caveats, not claimed.

## (d) Exact reproduction steps

From this `compute/` directory (`uv` for the CP-SAT arm):

```sh
uv run tailforge.py --seeds 24 --rows 2,3 --caps 1,2 \
    --time-limit 60 --workers 8 --out ../results/tailforge.json
```

Each (seed, rows, cap) cell is one CP-SAT solve; 24 seeds x 2 band heights x 2
caps = 96 solves, most closing in well under a minute at 2-3 rows.

Acceptance: in `results/tailforge.json`, every record has `delta >= 0` and
`verified: true`; the per-config summary reports the delta distribution
(min/median/max/mean), the count of tail-limited (delta>0) vs tail-tight
(delta=0) seeds, and cap=2's delta stochastically dominates cap=1's. Report
per-config min/median/max deltas across seeds (variance reporting is mandatory).
Then write the `results/` tables into the article; flip nothing to `published`.
