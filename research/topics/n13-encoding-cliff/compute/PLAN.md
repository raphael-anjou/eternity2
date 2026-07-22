# Reproduction plan: n13-encoding-cliff

> **Repro tier: seeded-statistical (per-rung solve rates within tolerance; cliff shape is the claim).**
> exact = re-run reproduces committed results byte-for-byte. seeded-statistical =
> committed results are the record of our run (hardware noted in the article);
> reproduction = fresh run agreeing within the stated tolerance. qualitative =
> the effect (sign/ordering/band) reappears; exact digits are lineage-bound.

Class: **measurement** (two-arm comparative measurement; the "solver" here is
an instrument, not a record attempt).

## (a) The exact claim, with expected numbers

Source measurements (research vault, vols 242-243: `n13-solvability-cliff`,
`A-CPSAT-CLIFF`, `correlated-solver-agreement-fallacy`; planted framed rungs,
C=26 interior colours, 5 faithful clues):

1. **Heuristic cliff.** Restarting randomized DFS (`cdfs_edges`), seed 1,
   20 s budget: N=10 solves in 1 ms, N=11 in 9 ms, N=12 in 36 ms (264/264);
   N=13 scores **39/312** and N=14 **42/364** at timeout. Robust across 5
   generator seeds (N=12 solves on every seed under 1 s; N=13 fails on every
   seed, scores 15-39). **Not a budget effect**: N=14 at 45 s and at 120 s
   both return 42, identical; ~17.7 M nodes burned in 20 s with zero gain.
2. **Paradigm-wide.** 10 heuristic families tested at N=12 vs N=13 (15 s):
   six full-solve N=12 (including `naive_rowmajor`, no propagation at all);
   **zero** solve N=13 (best 77/312, i.e. 25 percent).
3. **CP-SAT crosses smoothly.** AllDifferent + Element channelling, decision
   form, 5 seeds per rung, 300 s limit, 8 workers: **25/25** full solves for
   N=10..14, median times 0.10 / 0.19 / 0.42 / **0.50** / 0.84 s; N=15 in
   1.47 s, N=16 (480/480 planted) in 15.27 s. No cliff.
4. **Encoding gap.** Same N=12 instance: HiGHS B&B on `x[cell,piece,rot]`
   binaries + sum-to-1 rows returns **11/264** in 300 s (TimeLimit, no
   certificate); CP-SAT returns **264/264 OPTIMAL in 0.42 s**. Roughly three
   orders of magnitude from the encoding alone. (The MIP arm is optional
   here; the claim stands on arms 1-3.)

Composite claim to reproduce: *there exists a size N\* in 10..16 at which
every chronological heuristic collapses (scores pinned far below target,
budget-insensitive) while the structured CP-SAT encoding full-solves the
identical instances in seconds with a smooth time curve through N\*.* In the
source data N\* = 13 at C=26.

## (b) Scoring-convention mapping

- The kit's `score_cells` / `Instance::finish` is the canonical
  **rim-excluding matched-edges** scorer: max score on an NxN board is
  2N(N-1) (480 at 16x16).
- The vault's "planted optimum" per rung (180, 220, 264, 312, 364, 420, 480
  for N=10..16) is the **same convention**: interior adjacencies only, rim
  excluded. Numbers map one-to-one; no conversion needed.
- "Full solve" = score equals 2N(N-1) exactly.
- Implementation note: the kit's `Board` and `score_cells` are fixed 16x16
  (stride 16). The compute bin embeds an NxN sub-board at the top-left of
  that grid (`pos16 = y*16 + x`); empty cells never score and sub-board rim
  edges are colour 0, which the scorer excludes, so `Instance::finish` counts
  exactly the NxN interior matches through the canonical scorer with no
  re-implementation. Verified in a smoke run: an N=8 full solve reports
  112 = 2*8*7 through `finish`. Caveat: `SolveOutput::breaks` and the viewer
  URL are 16x16-shaped and are not meaningful for N < 16.
- Hints: the source rungs pin 5 solution cells (mimicking the official
  clues). The kit's `pin_solution_hints(k=5)` does the same and
  `Instance::seed_board()` places them before search, so every reported
  score is **strict-by-construction** (all 5 clues obeyed). This is the
  analogue of the strict-5/5 convention at 16x16; there is no
  matched-edges-vs-strict ambiguity in this topic because clue conformance
  is enforced, never traded away. The CP-SAT verifier re-checks clue
  conformance independently.

## (c) Scale-faithfulness

This finding is **intrinsically a scale study**; the reproduction must
straddle the boundary.

- Testing only N <= 12 would **falsely refute** the cliff (every family
  solves there, source table: six families at 264/264) and testing only
  N >= 13 would miss the control that makes the heuristic collapse
  meaningful.
- The 16x16 rung is included for the smooth-curve endpoint but is **not**
  canonical E2 and must never be reported as such: planted instances
  plausibly admit astronomically many solutions where E2 is believed to
  admit essentially one, and the original rungs used 26 interior colours
  against E2's 22 (more colours = more constrained = easier). The vault
  flags the "16x16 solved in 15 s" row as deeply misleading without these
  conditions; the draft article repeats the caution.
- **Known fidelity gaps vs the source measurement** (state these in any
  write-up):
  1. The kit generator caps interior colours at 22 (`generator::max_colors`
     = min(interior edges, 22)), so the original C=26 setting cannot be
     generated here. The vault records "whether the cliff moves with C" as
     open; at C=22 the collapse size N\* may shift. The composite claim in
     (a) is deliberately stated over N\* in 10..16, not hard-coded to 13.
  2. Different generator (the kit's colour-balanced framed generator vs the
     research repo's bench-grid rungs), so per-instance scores like 39/312
     are not expected byte-for-byte; the qualitative shape (ms-fast below
     N\*, pinned and budget-insensitive above, CP-SAT smooth throughout) is
     the reproduction target.
  3. One heuristic family (restarting exact-match DFS, the source's
     decisive `cdfs_edges` analogue) is implemented here, not all ten. The
     paradigm claim rests on DFS + the CP-SAT contrast; porting more
     greedy families adds correlated witnesses only (that is the
     methodological point of the finding).

## (d) Exact reproduction steps

From this `compute/` directory (Rust 1.85+, `uv` for the Python arm):

```sh
cargo check                       # skeleton compiles against the starter kit
mkdir -p ../results instances

# Arm 1: heuristic cliff. ~5 sizes x 5 seeds x <=20 s, well under 30 min
# (sizes below the cliff finish in milliseconds).
cargo run --release -- sweep --ns 10,11,12,13,14 --gen-seeds 1,2,3,4,5 \
    --budget-secs 20 > ../results/heuristic.jsonl

# Budget-insensitivity check at one failing size (45 s vs 120 s must tie):
cargo run --release -- sweep --ns 14 --gen-seeds 1 --budget-secs 45
cargo run --release -- sweep --ns 14 --gen-seeds 1 --budget-secs 120

# Arm 2: export the identical instances for the exact arm (instant).
cargo run --release -- export --ns 10,11,12,13,14,15,16 --gen-seeds 1,2,3,4,5 \
    --out instances

# Arm 3: CP-SAT decision solves + independent verification (seconds per
# instance expected; 300 s cap each).
uv run cpsat_cliff.py instances/*.json --time-limit 300 --workers 8 \
    > ../results/cpsat.jsonl
```

Acceptance: in `heuristic.jsonl`, let N\* be the smallest size at which no
seed full-solves; sizes just below N\* full-solve on most or all seeds (near
the boundary individual seeds are a lottery, as a 5-instance smoke at C=22
already showed: N=8 and N=11 solve in ms while N=10 and N=12 miss at 8 s),
and at N\* and above the scores are pinned across the 45 s / 120 s check. In
`cpsat.jsonl`, every instance including N\* and above reports
`full_solve: true` and `verified: true`, with solve times forming a smooth
curve (the smoke: CP-SAT solved the N=10 instance the DFS missed, in
0.31 s, verified). Report per-size min/median/max times across seeds
(variance reporting is mandatory). Then write `results/` tables into the
article and flip nothing to `published` until the numbers are in.
