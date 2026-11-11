# Amendments to PREREG.md

PREREG.md is unchanged. Entries below record what happened between the
pre-registration commit and the full run. None of them touches the kill
criterion, its operationalisation, the configurations, the seeds or the
methods.

## 1. Smoke run on seeds 1 to 5 before the full run (2026-11-11)

What: after the code compiled and the unit tests passed, the binary was run
once with `--seeds 5` (all four configurations, 20 puzzles in total) into a
scratch directory outside the repo, to check that the output files were well
formed. Those 5 seeds are a subset of the registered 200.

Why it is recorded: the smoke run showed per-configuration numbers (spectral
not ahead of rarity on those 5 puzzles) before the full run. Nothing in the
design was changed after seeing them; the only code change after the smoke
run is amendment 2 (a diagnostic) and two clippy lint fixes with no effect on
output.

## 2. Extra eigenvalue-gap diagnostic (2026-11-11)

What: each puzzle now also records `eig.gap_at_k_sweep`, the gap
`lambda_k - lambda_{k+1}` at each exploratory k in {1, 2, 4, 8, 16, 32}, and
the summary counts puzzles with a gap below 1e-9 per k.

Why: in the smoke run the re-scramble leakage check (PREREG, leakage check 1)
gave per-puzzle differences up to about 0.02 for `spectral_k16` and
`spectral_k32` on a few puzzles, while every other method agreed to 1e-15.
A rank-k reconstruction is not unique when `lambda_k = lambda_{k+1}`
(the cut falls inside a degenerate eigenspace, and Jacobi then returns a
basis that depends on the input order). The diagnostic lets the README show
whether those differences coincide with zero gaps, which separates numerical
non-uniqueness from leaked order. The primary k showed no such difference.
