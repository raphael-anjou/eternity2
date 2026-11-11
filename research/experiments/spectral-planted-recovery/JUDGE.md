# Judge report: spectral-planted-recovery (agenda A1)

Judged commit: `ab97264` on `devin/r1-spectral-planted-recovery`. Pre-registration commit `93ce29b`.
Judge did not modify the branch; this file is the report to be committed next to the experiment.

## Verdict: kill

The test is valid and the pre-registered kill criterion is met at both primary configurations.
The primary spectral method (rank-k reconstruction, k = ceil(sqrt N)) ranks the planted neighbour
worse than the colour-rarity baseline, not better: paired normalised-rank dbar = -0.00472
(SE 0.00059, z -7.94) at n8_c8 and dbar = -0.00349 (SE 0.00059, z -5.91) at n10_c12, n = 200
puzzles each. The pass condition was dbar > 2 SE; both configurations miss it by a wide margin
and in the wrong direction. The secondary configurations agree (z -5.11, -4.27). No positive
effect exists under the pre-registered analysis; the only positive results (k = 1, and the `deep`
stratum) are exploratory or interpretive and are themselves explained by a per-piece score with no
pair information.

## 1. Pre-registration and amendments

- `PREREG.md` was added alone in `93ce29b` (2026-11-11 11:27:03 UTC). Code, recipe and every file
  under `results/` were added in `1b4206e` (11:37:53); `README.md` in `ab97264` (11:49:44).
  `PREREG.md` is unchanged after `93ce29b` (`git diff 93ce29b ab97264 -- PREREG.md` is empty).
- The kill criterion in `PREREG.md` is quoted verbatim from the agenda entry and matches it. The
  operationalisation (per-puzzle paired difference of mean normalised rank, SE with n - 1, pass iff
  dbar > 2 SE, both primary configs decide, primary k fixed at ceil(sqrt N), normalised rank
  decides over raw) is fixed in advance and is what `src/main.rs` and `results/summary.json`
  implement.
- `PREREG_AMENDMENTS.md` records (1) a 5-seed smoke run before the full run and (2) an added
  eigenvalue-gap diagnostic. Neither touches the kill statistic, configs, seeds or the primary
  method. Amendment 2 was motivated by smoke-run output, so it is post hoc, but it only serves to
  explain a failure of the pre-registered re-scramble check for two exploratory methods
  (`spectral_k16`, `spectral_k32`); it cannot move the verdict. Judged not a problem.
- Git timestamps cannot prove no code ran before `93ce29b`; the 10-minute gap and the disclosed
  smoke run are consistent with the stated order.

## 2. Recomputed numbers

Two independent recomputations were run by the judge:

(a) `recompute.py`: reads only `results/raw/*.jsonl` (not `summary.json`) and recomputes every
headline statistic. (b) `indep.py`: an independent pipeline in Python/NumPy that takes only the
engine's solved boards (`generate_solved_framed_bc`), re-implements the XorShift scramble, derives
planted neighbours from grid geometry, builds W from the colour census, uses LAPACK `eigh`
(not the crate's Jacobi), and re-implements the mid-rank. Its scramble matches the engine's
`generate_framed` piece for piece at the default border count, and its per-puzzle normalised ranks
for `rarity`, `spectral_primary`, `cand_rarity` and the analytic uniform match the committed raw
files to at most 3.6e-15 on all 800 puzzles; per-puzzle `mean_candidates` matches to 1e-12.

| config | reported dbar / SE / z | judge from raw | judge independent pipeline |
|---|---|---|---|
| n8_c8 (primary) | -0.0047 / 0.0006 / -7.94 | -0.004723 / 0.000595 / -7.939 | -0.004723 / 0.000595 / -7.939 |
| n10_c12 (primary) | -0.0035 / 0.0006 / -5.91 | -0.003489 / 0.000590 / -5.915 | -0.003489 / 0.000590 / -5.915 |
| n8_c14 | -0.0055 / 0.0011 / -5.11 | -0.005529 / 0.001083 / -5.108 | -0.005529 / 0.001083 / -5.108 |
| n10_c21 | -0.0037 / 0.0009 / -4.27 | -0.003739 / 0.000875 / -4.272 | -0.003739 / 0.000875 / -4.272 |

Means (normalised rank, all half-edges), judge from raw: n8_c8 uniform analytic 0.5188, uniform
sim 0.5205, rarity 0.5263, spectral_primary 0.5310, combined 0.5309, cand_rarity 0.5179; n10_c12
0.5174, 0.5183, 0.5251, 0.5286, 0.5284, 0.5170. All match the README. Raw-rank paired z: -7.94,
-5.66, -4.50, -4.20 (match). Versus uniform: rarity z -5.61, -6.82, -7.28, -8.30; spectral z
-8.05, -8.81, -10.12, -10.24. Fraction of puzzles where spectral beats rarity: 0.275, 0.34, 0.38,
0.36. Strata, k sweep, top-1 rates, id_distance control (0.157/0.131/0.183/0.155 solved order vs
0.521/0.516/0.537/0.530 scrambled), census constancy and tiling all match the README.

Reproduction: `just experiments spectral-planted-recovery` on a clean worktree of `ab97264`
(results deleted first) ran in 4 min 0 s (tests 29 s, run 191.5 s) and `diff -r` against the
committed `results/` is empty (byte for byte).

Exact quantities re-derived independently: the analytic uniform expectation mean((m+1)/(2m)) and
the candidate count m = n_c - k_p per half-edge were recomputed from first principles in
`indep.py` and match the committed values; the colour census (28 per colour at n8_c8; 24/32,
28/14, 24/16 per band elsewhere) is constant across all 200 seeds of each config.

## 3. Leakage, compute and baseline fairness

- Leakage: none found. Methods receive only the scrambled piece list; the independent pipeline,
  which never shares code with the crate, reproduces every method score. Re-scramble differences
  are <= 1.3e-15 for every method except `spectral_k16`/`spectral_k32`, and the judge checked that
  every puzzle failing the 1e-6 threshold for those two has a zero eigen-gap at that k (n8_c8:
  38/38 and 42/42; n10_c12: 170 of 171 and 44/44; n8_c14: 0/0 and 12/12; n10_c21: 0/0 and
  27/27), so the README's non-uniqueness explanation holds. The primary k has min gap 0.0029 and
  max reconstruction residual 1.8e-13. The id_distance positive control shows the metric would
  expose leaked order.
- Compute: the spectral method gets strictly more compute (a full eigendecomposition) than the
  rarity lookup it is compared with. That inequality favours the method, so it cannot explain a
  kill.
- Baseline: the kill baseline (rarity = W entry) is itself below uniform at all four configs, so
  the bar set by the agenda is lower than chance. The method fails even this low bar, which makes
  the kill conservative. A keep under this criterion would have been weak evidence (see findings).

## 4. Rigor checklist (adapted to an experiment)

- Null model: uniform (analytic and simulated), rarity, a per-piece rarity score and an
  order-leak positive control are all present. Pass.
- Claims no stronger than data: the README says "this method recovers no planted signal here, not
  that no spectral method can" and lists the untested operators. Pass, with the scope note on
  the generator below.
- Exact search vs record hunting: not applicable.
- Currency: hardware dated (2026-11). No literature cited for planted-partition recovery; the
  untested operators (non-backtracking, Bethe Hessian) are named. Acceptable for a cheap test.
- Numbers against committed raw results: all recomputed, all match.
- Cross-page consistency: nothing published to the site yet; AGENDA, LEDGER, index and web are
  untouched on the branch.

## 5. Adversarial review

Run as four separate passes by the judge (no parallel subagent tool was available in this session):
design validity, scope and completeness, prose, code and reproducibility. Every finding below was
checked against code or data by the judge.

Additional judge probe (exploratory, not a verdict input, `classprobe.py`): on `deep` half-edges
with candidates restricted to interior pieces (so piece class is held fixed), the normalised rank
minus uniform is: rarity +0.0003, +0.0053, +0.0092, +0.0074 (z +0.15, +3.16, +4.67, +4.91) and
spectral +0.0024, +0.0072, +0.0124, +0.0081 (z +0.98, +4.04, +5.91, +4.86) for n8_c8, n10_c12,
n8_c14, n10_c21 (positive = worse than uniform). Spectral is also not better than rarity there (z -3.23, -2.78, -2.49,
-0.62). So once the border/interior split is removed, neither statistic ranks the planted neighbour
above chance; the small deficit below uniform is consistent with a without-replacement effect of
the fixed per-colour census. This supports the README's reading and is the main reason the kill is
not a near miss: on this generator W carries essentially no pair-level planted signal for any
transform of it to recover.

## 6. Findings

`file:line | severity | claim | problem | fix`

- `README.md:136-144 | should-fix | "the gain on deep is accounted for by the border and interior split" | Correct but argued only by comparison with cand_rarity; the direct test (candidates restricted to the planted neighbour's piece class) is not run, and the a priori reason (W is a function of the piece multiset, and an i.i.d.-style painter puts no extra pair correlation on planted pairs) is not stated | Add the class-restricted control (judge numbers above: at or worse than uniform, z +0.15 to +5.9) and one sentence on why no pair signal is expected from this generator`
- `README.md:55-56, 210-216 | should-fix | "E2-faithful" primary configs | Colour multiplicity is matched but the census is exactly uniform within each band, so the "weighted by colour rarity" part of the hypothesis is a constant per band and is not really exercised; this is in Limits but not near the headline | State in the outcome paragraph that the result is for the framed generator with uniform per-band census`
- `research/program/AGENDA.md (A1 Kill) | should-fix | baseline "colour-rarity ranking alone" | The rarity baseline is below uniform at every config (z -5.6 to -8.3), so "better than rarity" is a bar below chance; a keep under it would not have shown above-chance recovery as the hypothesis states | Future kill criteria should require beating max(uniform, rarity, cand_rarity)`
- `PREREG.md:144 / README.md:186-193 | nitpick | re-scramble check passes | The pre-registered 1e-6 check fails for spectral_k16 and spectral_k32; the explanation relies on a diagnostic added after the smoke run (amendment 2). Judge verified the failing puzzles coincide with zero gaps (all but one of 171 zero-gap puzzles at n10_c12 k16 fail; none outside the zero-gap set) | Keep the wording that these two fail the pre-registered check; optionally report them as excluded rather than "pass with explanation"`
- `README.md:112 | nitpick | "spectral by 8.1 to 10.2" paired SE below uniform | n8_c8 z is -8.05, which rounds to 8.0 | Write "8.0 to 10.2"`
- `README.md:83 | nitpick | "n8_c8 168 s" | Judge rerun: n8_c8 155 s, total 191.5 s; wall clock is machine dependent and n8_c8 dominates (64 x 64 Jacobi slower than 100 x 100), likely slow convergence on its degenerate spectrum (not verified) | Give a range or say "about 3 to 4 min"`
- `src/main.rs:182-223 | nitpick | Jacobi eigen-decomposition converged | Fixed cap of 100 sweeps with no flag if the cap is hit; only the residual is recorded (max 1.8e-13 here, and LAPACK agrees, so no effect) | Record the sweep count or assert convergence`
- `src/main.rs:433 | nitpick | simulated uniform ranking | rng.below(m) is a modulo draw (bias of order m / 2^32) and draws integer ranks without ties, so its top-1 rate is not comparable with mid-rank methods | Negligible; note it or compare top-1 against 1/m analytically`
- `PREREG.md (Definitions, candidate set) | nitpick | colour-only candidate set | Includes geometrically impossible candidates (a grey side would face inward), which is where most of the rim loss comes from; pre-registered and disclosed | A next test, if any, should use geometry-feasible candidates`

## 7. What next

A1 should be recorded as killed in the ledger. No larger run of this design is warranted: the class
restricted probe shows no above-chance pair signal in W on generated puzzles, so a larger n or a
larger board would only tighten an effect that is negative. If the program wants to revisit the
idea at all, the only cheap question left is whether the official 256-piece set has pair structure
that random painting lacks: compute the same class-restricted statistics on the official pieces
against a null of colour-multiset-preserving reshuffles (frame and interior seams shuffled
separately, boards rebuilt), and test non-backtracking or Bethe Hessian operators there. Estimate:
1 core, under 1 hour; it needs a new pre-registration and is not a continuation of A1.

## Judge artefacts

`recompute.py` (statistics from raw jsonl), `indep.py` (independent pipeline, 800 puzzles),
`classprobe.py` (class-restricted probe), and a small Rust dumper of the engine's solved boards.
