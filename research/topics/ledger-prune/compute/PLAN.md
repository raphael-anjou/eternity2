# Reproduction plan: ledger-prune

> **Repro tier: exact, conditional: exhaustion node counts are seed-deterministic and the committed files carry no wall-clock values, but WHICH arms are censored at the 25 s cap is hardware-dependent; byte-for-byte reproduction holds on hardware no slower than the recording machine (Apple Silicon, single core), otherwise more arms censor.**
> exact = re-run reproduces committed results byte-for-byte. seeded-statistical =
> committed results are the record of our run (hardware noted in the article);
> reproduction = fresh run agreeing within the stated tolerance. qualitative =
> the effect (sign/ordering/band) reappears; exact digits are lineage-bound.

## (a) The exact claim, with expected numbers

Source: research vault, `vault/concepts/blackwood-ledger-prune.md` (vol-220,
measured 2026-07-10, M1, single thread, release) and
`vault/concepts/ledger-color-deficit.md` (vol-213/214 precursor definition).

Claim under reproduction. In a row-major break-allowance DFS, maintain per
colour c:

- S(c): half-edge supply over all unused pieces (all 4 sides each),
- D(c): frontier demand (exposed sides of placed cells facing empty cells,
  plus grey rim demand of empty rim cells, D(grey) initialised to 64 on the
  16x16, plus hint right/bottom sides),
- r: total remaining charged-break budget,
- u: unresolved free junctions (top/left junctions of pinned hint cells,
  uncharged by the engine).

Then any completion within budget satisfies both:

1. deficit: sum_c max(0, D(c) - S(c)) <= r
2. parity: popcount over c of (S(c) - D(c)) mod 2 <= 2r + u

and pruning on the failure of either is sound (never cuts a within-budget
completion).

Expected numbers, cited from the vault page:

- Soundness gate: on the three community 464 boards plus our 460, replayed
  under each board's own zero-slack break line, 0 fires and 0
  incremental-vs-recomputed inconsistencies over 251 judged cells per board.
- Record-regime race (fixed 180-degree-flipped prefix of 464 board 1, suffix
  budget = the real tail's break count, both arms run to completion or
  exhaustion):

  | prefix D | suffix cells / breaks | NoPrune nodes | LEDGER nodes | ratio |
  |---|---|---|---|---|
  | 232 | 24 / 2 | 433 | 289 | 1.5x |
  | 224 | 32 / 5 | 471,898 | 3,373 | 140x |
  | 216 | 40 / 6 | 24,558,812 | 24,688 | 995x |
  | 208 | 48 / 8 | 2,840,101,549 | 655,903 | 4,330x |

- The ratio compounds with suffix depth (branching-factor reduction), parity
  fires dominate deficit fires roughly 5-10:1 in the small-r regime, and with
  a large budget the prune fires on ~0.1 percent of nodes (wrong-regime null).
- Scope caveat carried from the vault: the original engine's relaxed buckets
  admit at most one mismatch per cell against (required_top, required_left),
  so the vault's certificate rows prove "no completion with <= r suffix breaks
  AND <= 1 break per cell". This reproduction charges breaks per edge with no
  per-cell cap and keeps all four rim edges hard, so its exhaustions are the
  cleaner per-edge proper-frame statement; node counts are therefore expected
  to be close but not identical to the vault engine's.

## (b) Scoring-convention mapping

The kit's canonical scorer (`score_cells` / `Instance::finish`) counts
rim-excluding matched internal edges; max 480 on the 16x16. The finding's
"breaks" (charged conf) are mismatched internal junctions of a full board, so
on a complete board:

    breaks = 480 - matched_edges_score

A "464 board" is a full board with 16 breaks; a suffix budget of k breaks
means every completion found scores at least 480 - (prefix breaks) - k under
the kit scorer. Strict-5/5 vs matched-edges: the vault measurements ran on
hinted boards where hint cells are pinned and their top/left junctions are
uncharged (the u term). This reproduction phase runs hint-free (u = 0), which
only strengthens the parity condition and leaves soundness arguments intact;
the hinted u > 0 case is a faithful extension once community boards are
available (see gaps). The kit's `pin_solution_hints` provides the pinning
mechanism for that extension.

## (c) Scale-faithfulness

Two separate questions:

1. Soundness (the theorem) is size- and instance-independent: the invariance
   lemmas hold on any width/height/colour count. A small-N soundness replay is
   a valid test and cannot falsely refute.
2. The magnitude of the node reduction is NOT small-N faithful. The vault
   table shows the multiplier growing 1.5x -> 140x -> 995x -> 4,330x as the
   suffix deepens from 24 to 48 cells; a small board caps the suffix depth and
   would show only the 1.5x-140x end of the curve. Reporting a small-N ratio
   as "the" ratio would falsely understate (not refute) the claim. The vault's
   wrong-regime null is the same warning from the other side: with big budgets
   the prune is a pure loss, so a test with sloppy budgets would falsely
   refute the value claim while soundness still passes.

Consequence: run soundness anywhere, run the ratio measurement on the
canonical 16x16 shape with zero-slack or slack-minus-one budgets and suffix
depths 24 to 48, matching the vault regime.

## (d) Exact reproduction steps

Phase 1 (this crate, runnable now, generated instances):

1. `cargo run --release -- --mode soundness [--seed S]`
   Generates the official-shaped 16x16/22 framed instance for seed S with a
   known solution, replays the solution row-major, and at every prefix depth
   evaluates both LEDGER conditions with r = 0 (the true tail needs zero
   breaks). Expected: zero fires at every depth, mirroring the vault gate.
2. `cargo run --release -- --mode ab [--seed S] [--suffix K] [--budget R] [--cap-secs T]`
   Fixes the first 256-K solution cells as a prefix, then exhausts the
   K-cell suffix (counting ALL completions with <= R charged breaks) twice:
   prune off and prune on. Reports nodes, completions (must be EQUAL between
   arms, the correctness check), prune fires, and the node ratio. Sweep K in
   {20, 24, 28, 32} and R in {1, 2, 3} across >= 8 seeds; report
   min/median/max of the ratio per (K, R). Expected: ratio > 1 growing with
   K, completions identical in both arms.
3. The board every run touches is emitted as an eternity2.dev URL.

Phase 2 (needs the gap items below): repeat the vault's exact certificate
rows on 464 board 1 flipped 180 degrees at D in {232, 224, 216, 208} with the
real tail's break counts, single thread, and compare against the cited table.

Implementation notes for this skeleton: the ledger is recomputed from scratch
at every node (O(cells + unused pieces)), which is sound and byte-identical in
fires to an incremental ledger but slower per node; the vault's nps overhead
claims (~3x, amortised) need the incremental version and are out of scope for
phase 1. The prune is evaluated once per node entry rather than per candidate
(a strictly weaker but still sound placement of the same test); per-candidate
judging is the phase 2 fidelity item, needed to reproduce the late-break
funneling behaviour.

## Kit gaps observed

- No bundled community record boards (the 464s, 469, 470): the exact cited
  node counts and the certificate rows are tied to 464 board 1; phase 2 needs
  those boards as data files plus a 180-degree board-flip helper.
- No break-allowance search example: every kit example is strict placement;
  a configurable "conflicts_allowed" budget/schedule concept does not exist
  in the kit and had to be built here.
- `SolveOutcome` carries only a scalar `nodes`: no channel for prune-fire
  counts or A/B diagnostics, so the reproduction prints its own report
  instead of flowing through the sweep runner's Summary.
- No fixed-prefix exhaustion runner: `sweep` always starts from
  `instance.seed_board()`; continuing from a prefix works by calling
  `Solver::solve` directly, but nothing aggregates A/B node ratios.
