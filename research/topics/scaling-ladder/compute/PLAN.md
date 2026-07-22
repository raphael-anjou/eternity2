# Reproduction plan — scaling-ladder

> **Repro tier: seeded-statistical (distributional agreement per rung, n>=8 seeds).**
> exact = re-run reproduces committed results byte-for-byte. seeded-statistical =
> committed results are the record of our run (hardware noted in the article);
> reproduction = fresh run agreeing within the stated tolerance. qualitative =
> the effect (sign/ordering/band) reappears; exact digits are lineage-bound.

Source of the claim: research repo, `research/vault/papers/vol-240/SCALING-LADDER.md`
(vol-240, 2026-07-14). Class: **measurement** (with reusable infrastructure as the
deliverable; there is no record claim here).

## (a) The exact claim, with expected numbers

1. **Planted ladder with a free proven ceiling.** An N x N instance built
   backwards from a planted solution is solvable by design and its full-solve
   target is `2N(N-1)` matched internal edges: {N=8: 112, N=10: 180, N=12: 264,
   N=14: 364} (SCALING-LADDER.md sect A.2, lemma "planted optimality"; trivial
   internal-adjacency upper bound achieved by the planted board, so no external
   solver is needed to certify it). CP-SAT/HiGHS independently re-derived the
   optimum where it closed: all three N=8 instances in under 1.02 s, N=10 seed 1
   in 78.7 s; N=10 is the practical ILP edge, larger N timed out at 180 s with
   the ceiling still proven by the trivial bound (sect B.4).

2. **The gap grid** (13 registry algorithms x 11 instances x 3 seeds = 429 jobs,
   12 s single-core budget each, gap = best verified score / ceiling, sect 4.1):
   - The propagating CSP family (`gacolor_ac3` and variants, `verhaard_preferred`,
     `joe_depth150` and variant, plus `naive_rowmajor`) holds **gap = 1.000
     through N=12** (264/264).
   - The border-first family (`border_first_lcv/full/random`, `rare_color_first`)
     and `naive_spiral` break one rung earlier: **0.148 to 0.223 at N=12**, and
     `border_first_random` already 0.300 at N=10.
   - **Universal collapse at N=14**: every method lands at gap 0.055 to 0.157;
     the best any method reached was 57/364 (`naive_spiral`). This independently
     reproduces the vault's documented N=14 hardness threshold
     (`mcgavin-n-row-scaling`) and the ~80-cell distinctness collapse
     (`isentrope`).
   - Throughput inversion: the naive/border family runs 10x to 50x more
     placements per second (e.g. `naive_rowmajor` 205 k/s, `border_first_random`
     496 k/s vs ~5-12 k/s for the gacolor family) yet loses on gap; raw speed is
     not the scaling lever.

3. **Verification discipline**: every emitted board was independently re-scored
   against the instance's own pieces; 429/429 runs verified with 0 mismatches
   (sect 2.2).

4. **Instance faithfulness targets** (sect A.3-A.4): C = 22 interior colours,
   5 rare colours confined to the frame-adjacent ring, rare internal-edge
   fraction 0.25 (= real E2's 5*24/480), rare/uniform per-edge ratio 1.13
   (= real E2), 5 planted hints, duplicate pieces 0 at N=8, at most 4/144 at
   N=12, ~10-11/196 at N=14 (reported, not hidden).

## (b) Scoring-convention mapping

The kit's `score_cells` is the canonical **rim-excluding matched-edges** count:
it counts matched internal adjacencies only, max `2N(N-1)` (480 at N=16). The
vol-240 ladder's ceiling is exactly the same quantity, so the mapping is the
identity: `gap(algo, N) = kit_score / (2N(N-1))`. No strict-5/5 accounting is
involved anywhere in this topic: the ladder instances carry 5 planted hints and
solvers start from the hinted seed board, but the reported number is plain
matched edges, and full solve at gap 1.000 automatically satisfies the hints
(the planted board is the unique way to match every adjacency only in the
degenerate sense; hint conformance is enforced by seeding, not by scoring).
`SolveOutput.max_score` from `Instance::finish` should equal the ceiling on
every rung; the harness asserts the planted board scores exactly `2N(N-1)`
before any solver runs, which pins both conventions together.

## (c) Scale-faithfulness

This finding is **intrinsically about small N**: the deliverable is the
full-solve-vs-N curve on rungs N < 16, so small-N testing is not a proxy here,
it is the object itself. The canonical 16 x 16 official instance is not needed
to reproduce the claim. Two faithfulness caveats, both cited from the sources:

- **Instance-profile sensitivity.** The vol-240 rungs were generated with the
  real-E2 colour profile (C=22, rare=5, rare ring fraction 0.25, sect A.3-A.4);
  the generator sweep showed lower colour counts change duplicate-piece counts
  and hence difficulty. The kit's `generator::generate_framed` balances colours
  and confines five border colours to the frame but does **not** implement the
  rare-ring 0.25 edge-fraction construction, so the exact breakpoints (which
  family breaks at N=12, the depth of the N=14 collapse) may shift on
  kit-generated rungs. A curve-shape reproduction on kit instances is
  meaningful; an exact-number reproduction requires the committed vol-240
  instance set (`crates/bench-grid/instances/vol240/` in the research repo,
  11 SiteInstance JSONs + manifest), which is deterministic by (N, C, rare,
  seed).
- **Do not extrapolate up the ladder.** The sources are explicit (sect B.5)
  that small-N tractability is not a 16 x 16 record predictor: the methods that
  are perfect at N=12 are near worthless at N=14. Any reproduction that reports
  a small-N ranking as a 16 x 16 prediction would misstate the claim. Equally,
  a small-N run that fails to full-solve with weaker baseline solvers does
  **not** refute the claim, because the claim's gap=1.000 rows belong to
  specific propagating algorithms (AC-3 based) that are not yet ported to the
  kit; a false refutation from under-powered baselines is the main hazard.

## (d) Exact reproduction steps

Skeleton run (this crate, kit generator, two baseline solvers). The shared
engine fixes the board size at compile time (`e2-core` `size-N` features; no
runtime N), so one binary is built per rung; the four rungs are exactly the
vol-240 ladder {8, 10, 12, 14}:

1. `cd research/topics/scaling-ladder/compute`
2. `for s in 8 10 12 14; do cargo run --release --features size-$s -- --seeds 1,2,3 --budget-s 12; done > ../results/ladder.jsonl`
   (4 rungs x 3 seeds x 2 solvers at up to 12 s each: under 5 min wall; the
   DFS returns early on every rung it full-solves.)
3. Read the JSONL: one row per (solver, N, seed) with `score`, `ceiling`,
   `gap`, `full_solve`, `nodes`, `wall_s`, `url`. Expected qualitative shape
   (confirmed by a smoke run at reduced budget): `dfs-full-match` full-solves
   N=8 in well under a second (112/112) and degrades up the ladder (0.489 at
   N=12, 0.217 at N=14 in 5 s); `greedy-row-major` sits far below ceiling
   everywhere (0.313 at N=8). Every board URL opens in the viewer.
4. The harness asserts, per rung, that the planted board re-scores to exactly
   `2N(N-1)` through the canonical scorer before any solver runs (the sect A.2
   lemma, executed). This assert is live: a first draft that placed a small
   board into the default 16-wide grid scored 28/60 and was caught by it.

Faithful run (once gaps below are filled):

5. Import the 11 committed vol-240 SiteInstance JSONs in place of the
   kit-generated rungs (same manifest ceilings {112, 180, 264, 364}).
6. Port the algorithm registry (at minimum `gacolor_ac3`, one border-first
   variant, `naive_rowmajor`, `naive_spiral`) as kit `Solver` impls.
7. Re-run step 2 with `--budget-s 12 --seeds 1,2,3` and compare the gap grid
   against the sect 4.1 table; the acceptance criterion is the family split
   (propagating family perfect through N=12, border-first family broken at
   N=12, universal collapse at N=14), not per-cell equality.

Variance: 3 seeds per rung minimum, report best-over-seeds (the source's gap
metric) plus per-seed rows in the JSONL so min/median/max is recoverable.
