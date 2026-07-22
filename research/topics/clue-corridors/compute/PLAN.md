# Reproduction plan: clue-corridors

> **Repro tier: mixed: corridor count exact; ladder A/B seeded-statistical (paired seeds, sign + |z| band).**
> exact = re-run reproduces committed results byte-for-byte. seeded-statistical =
> committed results are the record of our run (hardware noted in the article);
> reproduction = fresh run agreeing within the stated tolerance. qualitative =
> the effect (sign/ordering/band) reappears; exact digits are lineage-bound.

## (a) The exact claim, with expected numbers

Source material: the project's vol-242 study of clue linking (concept note
`hint-corridor-linking`, paper `CLUE-LINKING`). Two coupled claims:

**Claim 1 (counting, exact).** On the official 16x16 instance:

- The five clues sit at cells 34, 45, 135, 210, 221, i.e. (x,y) = (2,2),
  (13,2), (7,8), (2,13), (13,13). Pairwise Manhattan distances range 10..22;
  the closest pair is centre to SW at distance 10. No two clues are adjacent.
- With n = 196 interior pieces and C = 22 interior colours, the uniform-model
  branching at k matched sides is b_k = 4n / C^k:
  b_1 = 35.64, b_2 = 1.620, b_3 = 0.0736, b_4 = 0.0033. Pruning requires
  b_k < 1, i.e. k >= 3 contacts.
- On the real piece set, only 17 of the 22 interior colours appear on inner
  (frame-free) pieces; 5 colours are frame-only.
- Per-side supply: the number of (piece, rotation) pairs presenting a given
  inner colour on a given side has mean 46.1, min 43, max 49 (the uniform
  model's 35.6 undercounts by ~29%).
- Transfer matrix T[a][b] = #{(inner piece, rot) : west = a, east = b}. Walk
  counts (piece reuse allowed): L=5 total ~2.0e8, to one fixed target colour
  ~1.1e7; L=9 total ~9.1e14, fixed target ~5.1e13; L=10 total ~4.2e16, fixed
  target ~2.4e15. With the mean-field distinct-piece correction
  prod_{j=0..9} (1 - j/196) = 0.792, the length-10 corridor count to a fixed
  clue colour is ~1.9e15. A commitment satisfiable 1.9e15 ways is vacuous.

  Semantics note (verified against the checker): the source's "total walks"
  is the row sum of T^L from one fixed start colour (a clue's facing colour),
  and its "fixed target" figure is a single T^L entry (both endpoint colours
  fixed). The source cites the values for the actual clue colour pair; the
  checker reports mean and min..max over all inner-colour pairs, and the
  cited values must fall inside that range. Measured stage-1 output: L=10
  entry mean 2.59e15, range [2.30e15, 2.97e15] (contains 2.4e15); row-sum
  mean 4.41e16 (source 4.2e16); corrected estimate 2.05e15 (source ~1.9e15).
  The vacuity conclusion is insensitive to which value in the range is used.

**Claim 2 (A/B, statistical).** Three arms differing only in a corridor phase
(control: clues pinned + max-contact greedy fill + restarts; path1: 1-wide
clue-to-clue L-routes laid first; ribbon2: 2-wide ribbons laid first), 12
paired seeds per cell, 20 s per run, faithful clue geometry, board sizes
N = 8, 10, 12, 14, 16. Expected results (matched-edge means):

| N  | control | path1 delta | ribbon2 delta | ribbon2/path1 |
|---:|---:|---:|---:|---:|
| 8  | 92.50  | -4.67  | -3.00 | 0.64 |
| 10 | 144.17 | -9.58  | -5.50 | 0.57 |
| 12 | 209.83 | -13.58 | -6.75 | 0.50 |
| 14 | 285.67 | -12.00 | -7.83 | 0.65 |
| 16 | 377.17 | -17.67 | -8.08 | 0.46 |

All paired Wilcoxon |z| >= 2.80 (two-sided p < 0.005). Mechanism signatures to
verify: (1) path1 damage grows monotonically with N; (2) ribbon2/path1 ~ 0.55
at every rung. At N=16 the control's worst seed (373) beat path1's best (365).

## (b) Scoring-convention mapping

The kit's `score_cells` / `Instance::finish` is the canonical rim-excluding
matched-edges scorer (max 480 at 16x16, `2wh - w - h` in general). The vol-242
A/B used exactly this convention: raw matched edges on boards where the clue
hints are pinned in **all** arms, so hint compliance is identical by
construction and the strict-5/5 vs matched-edges distinction does not arise.
No record convention is involved; every number is a within-experiment paired
delta (arm minus its own control), which is convention-invariant as long as
one scorer is used throughout. Claim 1 is a pure count and has no scoring
convention at all.

One mapping note: the vol-242 rungs report scores against the scaled optimum
`2N^2 - 2N` (e.g. 480 at N=16, 112 at N=8), the same formula as the kit's
`Instance::max_score`. Nothing to translate.

## (c) Scale faithfulness

Both directions are safe, with one caveat:

- **Small N does not falsely refute.** The effect is present and significant
  already at N=8 (delta -4.67, |z| = 2.80); it *grows* with N because clue
  gaps scale with the board and corridor ambiguity grows like b_1^d. So a
  small-N-only test would understate the damage, never flip its sign. The
  full N = 8..16 ladder is still the faithful reproduction because the
  dose-response (damage vs corridor length) is itself part of the claim.
- **Caveat from the source:** at N=8 the ratio-law clue placement rounds to
  inset-1 corners (adjacent to the frame) where the real puzzle has inset-2,
  so small rungs are slightly easier for clue linking than N=16. Read the
  curve shape across N, not any single rung.
- Claim 1 only makes sense on the canonical 16x16 official piece set (it is a
  statement about the real colour multiset); the checker runs there directly.
- Known ceiling: restarting DFS cannot full-solve faithful rungs at N >= 13
  (vol-242 section 4b), so the A/B compares budget-limited partial scores,
  never solve rates. That is what the source did too.

## (d) Exact reproduction steps

**Stage 1, runnable now (~1 s):**

```sh
cd research/topics/clue-corridors/compute
cargo run --release --bin corridors > ../results/corridor_counts.json
```

Verify against the expected numbers in (a): inner pieces 196, inner colours
17 (5 frame-only), supply mean 46.1 (min 43, max 49), b_1 = 35.64, clue
distance minimum 10, L=10 fixed start-and-target walk range containing
2.4e15, depletion factor 0.792, corrected estimate ~2e15 (source ~1.9e15).
All of these pass as of 2026-07-22 (run takes under one second).

**Stage 2, pending the solver port (see kit gaps):**

1. Generate faithful rungs: seeded instances at N = 8, 10, 12, 14, 16, 22
   scaled interior colours, with clues pinned at the ratio-law cells
   x = round(r_x (N-1)) for the five official clue ratios (at N=16 this must
   reproduce cells 34, 45, 135, 210, 221 exactly; unit-test it).
2. Implement the three arms as one kit `Solver` with an arm switch, sharing
   the fill order, restart loop and RNG discipline so the delta is
   attributable to the corridor phase alone. Corridors are laid nearest-pair
   first as L-routes with endpoints fixed; on dead ends re-route along the
   other L-bend rather than chronological backtracking.
3. Sweep: 3 arms x 5 rungs x 12 paired seeds x 20 s (single-core per run;
   about 60 core-minutes, under 10 wall-minutes on 8 cores).
4. Paired Wilcoxon signed-rank per (arm, N) cell against control; emit the
   delta table and the ribbon2/path1 ratio; check the two mechanism
   signatures from (a).
5. Emit a viewer URL for every produced board alongside the results JSON.
