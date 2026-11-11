# JUDGE: A3, SAT backdoors on solvable small boards

Branch `devin/r1-sat-backdoors` at `3e62a06`, directory `research/experiments/sat-backdoors`.
Judge: an independent session; the branch was not modified.

## Verdict: keep

The test is valid and the pre-registered kill rule is not met. In regime A (primary) the
fitted exponent is b = 0.589 with bootstrap 95% CI [0.549, 0.626], below the 0.9 threshold,
and D = kbar(10)/100 - kbar(6)/36 = -0.039 with CI [-0.045, -0.034], entirely below 0. I
recomputed every number from the committed raw `results.jsonl` with my own script. The full
run reproduces byte-for-byte, and an independently written propagator confirms the backdoor
sets and the exact minima I re-derived. The survive verdict also holds when the capped
instances are set to the least favourable values their certified bounds allow
(b <= 0.648, bootstrap upper 0.682).

The evidence is weaker than the headline "grows as N^0.589" suggests. Five size points also
fit an affine law k = 3.08 + 0.065 N as well as the power law. A lack-of-fit CI on the five
means reaches 0.92. The same sublinearity appears for random subsets (T50), so it is mostly a
property of propagation plus a colour count that grows with n, not of chosen backdoors. These
points limit what the result means. They do not make the pre-registered test invalid,
because the rule was fixed in advance and the analysis followed it.

## 1. Pre-registration and history

- `41359dd` (11:27:37 UTC) adds only `PREREG.md`. `785439f` (11:46:42) adds the code, the
  justfile recipe and all `results/`. `3e62a06` (11:58:04) edits only README.md. No results
  file exists before the prereg commit. The merge-base diff touches only the experiment
  directory and `research/experiments/justfile`. AGENDA.md, LEDGER.md, index.json and web/
  are untouched.
- The kill criterion in PREREG.md matches AGENDA.md A3 verbatim ("Backdoor size grows
  roughly linearly in cell count."). The operationalisation (CI upper end of b >= 0.9, or the
  CI of D not entirely below 0) was fixed before the results. `analyze.py` implements it
  exactly (`analyze.py:104-135`).
- There is no `PREREG_AMENDMENTS.md` and none was needed. The code matches the prereg on the
  colour table, seeds, node caps (3,000,000 / 500,000), restart and permutation seeds, local
  search cap, T50 definition and bootstrap seed. The only post hoc analysis
  (`post_hoc_sensitivity`) is labelled as such and does not drive the verdict.
- Design choices made before the results that differ from the agenda wording: sizes go up to
  10x10 (the agenda says 6x6 to 9x9), and the propagation is edge arc consistency plus
  all-different singles and hidden singles, not "unit propagation" on a named CNF. Both are
  declared in the prereg and are not post hoc. Arc consistency equals unit propagation on a
  support-encoded CNF, and it is stronger than unit propagation on the plain direct encoding.

## 2. Recomputed numbers (own script on committed raw results)

Script: `recompute.py` (judge workspace, not committed). It reads `results/results.jsonl`,
recomputes T50 from the 200 raw thresholds per instance, and refits and re-bootstraps
everything.

| quantity | reported | recomputed |
|---|---|---|
| A kbar, n=6..10 | 4.90, 7.03, 7.33, 7.93, 9.70 | 4.900, 7.033, 7.333, 7.933, 9.700 |
| A b_k [95% CI] | 0.589 [0.549, 0.626] | 0.5886 [0.5493, 0.6263] (seed 20261111); [0.5517, 0.6289] seed 1; [0.5495, 0.6294] seed 777 |
| A D_k [CI] | -0.039 [-0.045, -0.034] | -0.0391 [-0.0447, -0.0343] |
| A T50 | 10.87, 14.53, 15.77, 16.50, 22.10 | identical (also from raw thresholds) |
| A b_T50 [CI] | 0.606 [0.545, 0.678] | 0.6060 [0.5449, 0.6784] |
| A k/T50 | 0.44..0.48 | 0.451, 0.484, 0.465, 0.481, 0.439 |
| A p_rand(k) mean (max) | <= 0.0005 (0.005) | 0, 0.0005, 0.0003, 0, 0 (max 0.005); pooled 5/30,000 |
| A exact certified | 30, 6, 0, 0, 0 | same; capped lower bounds 6 (24 at n=7), 4 (all 90 at n>=8) |
| B kbar | 2.43, 2.67, 3.67, 3.33, 4.23 | same |
| B b_k [CI], D_k [CI] | 0.523 [0.448, 0.603], -0.025 [-0.030, -0.020] | 0.5228 [0.4478, 0.6027], -0.0253 [-0.0305, -0.0200] |
| B exact | 269/300 | 269/300 |
| post hoc n=7 at lower bound | b = 0.648 | 0.6479 |
| I(n) = (P! 4^P)^(1/2P) | 5.216 .. 9.935 | 5.216, 6.380, 7.558, 8.743, 9.935 |

Further judge analyses. None is pre-registered, and none changes the verdict.

- **Worst case within certified bounds.** OLS weights are negative for n=6,7 and positive for
  n=8..10 (log 64 = 4.159 is above the mean log N of 4.126). The largest b consistent with
  every certified bound therefore puts n=7 capped instances at their lower bound and keeps
  n>=8 at the upper bound: b = 0.648, bootstrap CI [0.612, 0.682]. D can only move further
  below 0, since kbar(6) is exact and kbar(10) is an upper bound. So the upper-bound issue
  cannot flip the verdict.
- **Consistent estimator.** Using the greedy upper bound at every size, so the estimator does
  not switch from exact to upper bound across sizes, gives b = 0.517 (A) and 0.564 (B).
- **Instance-level regression.** OLS of log k on log N over 150 instances gives b = 0.592 (A).
- **Lack of fit.** The bootstrap resamples instances within each size, so it captures sampling
  noise in each kbar but not the scatter of the five means about the line. Local slopes are
  1.17, 0.16, 0.33, 0.95. A classical t(3) CI on the 5-point fit is [0.26, 0.92] for A and
  [0.14, 0.91] for B. Both reach the 0.9 threshold.
- **Affine alternative.** k = 3.08 + 0.065 N has RSS 1.05, against RSS 0.96 for the power law
  0.635 N^0.589. A proportional law k = 0.107 N is clearly worse (RSS 6.08). The data exclude
  k proportional to N. They do not distinguish N^0.59 from linear growth with a positive
  intercept. Extrapolated to N = 256 the two give about 19.7 and 16.6.

## 3. Reproduction

- `just` is not installed on this VM, so I ran the four commands of the
  `sat-backdoors` recipe directly (build, run on 8 threads, analyze, `cmp` twice) from the
  branch checkout. 300 instances took 762 s wall. This run shared the 8 cores with the judge's
  Python checks, against 608 s reported. Both `cmp` checks passed:
  `results.jsonl` and `summary.json` are byte-identical to the committed files.
- `cargo test --release`: 3 passed. `cargo clippy --release --all-targets`: clean.

## 4. Independent re-derivation

I wrote a separate Python propagator (`indep.py`, set-based domains, the same rule family
written from the PREREG text, full sweeps to a fixpoint, no code shared with `lib.rs`). The
puzzles come from the engine generator through a 20-line dump binary.

- All 300 instances: the root has 0 singletons, the option count matches `options_total`, the
  reported backdoor set completes to the planted board, and removing any single cell of it
  breaks completion (every reported set is locally minimal). 0 mismatches.
- Exact minimum by plain enumeration of all (k-1)-subsets, without the singleton-skip rule:
  A n=6 seed 1 (k=5, 66,711 nodes) and seed 17 (k=4, 7,806 nodes); B n=6 seeds 1, 2, 6 and
  B n=7 seed 3. No smaller set completes in any of them, so every reported exact k is
  confirmed.
- Baseline: own RNG, 40 random subsets per size on A n=8 and n=10 seeds 1..5. The
  completion rate is 0.50 at T50 and 0.45 at T50-1, consistent with the T50 definition, and
  0/400 at size k, consistent with p_rand(k) of about 0.
- Code reading: the singleton skip in `dfs` (`src/lib.rs:503`) is exact, because a cell that
  is already a singleton under planted fixes holds its planted option and adds nothing.
  Stale `cnt` values in the all-different pass (`src/lib.rs:296`) only delay a hidden single
  to the next loop and are never unsound. Rules are monotone, so the fixpoint is unique and
  the incremental and from-root propagation agree. My independent propagator agrees on all
  checks.

## 5. Leakage, compute parity, baseline specification

- **Planted solution.** The method fixes cells to their planted placement by definition: the
  agenda asks for the minimum set "on solved instances". This is not leakage into a solver,
  but it means the measured object is a weak backdoor with respect to the planted
  assignment, not a strong backdoor (see findings).
- **Compute.** The baseline is a measurement, not a competing solver. Even so, the greedy and
  local search (excluding the exact search) use about 3k to 7k propagations per instance. The
  200-permutation baseline uses about 1.4k (binary search). The best of 200 random thresholds
  still exceeds k by 1.5 (n=6) to 4.5 (n=10) cells on average, so the advantage of chosen sets
  is not a compute artefact at this scale.
- **Baseline specification.** It matches the agenda ("same measurement on random cell
  subsets of equal size" gives p_rand(k)), and T50 adds a growth curve. It is fair.

## 6. Rigor check (adapted)

1. **Null model.** Present and correctly specified. It shows that the sublinear shape is
   shared by random subsets (b_T50 = 0.61 against b_k = 0.59, with k/T50 nearly constant).
   The README says so.
2. **Claims against data.** The README's verdict paragraph and caveats match the data. The
   runner's headline "grows as N^0.589" is more precise than five size points support (see
   the affine alternative and the lack-of-fit CI).
3. **Exact search against record hunting.** Not applicable. The README states that the
   second half of the hypothesis (backdoor search beats cell-by-cell search) was not tested.
4. **Currency.** Not applicable. Hardware is dated and the literature is cited by name
   (Williams, Gomes, Selman 2003).
5. **Numbers.** All recomputed and matching (section 2).
6. **Scope.** Five sizes, one generator, colours grown with n, one propagation rule. All are
   stated in the README.

## 7. Adversarial review

The tool set has no parallel subagents, so I did the four lenses as separate passes. Every
finding below was verified against code or data.

**Design validity.** The verdict cannot flip on the upper-bound issue (worst case 0.648). The
real weakness is model identification: 5 points, a bumpy curve, and a bootstrap that ignores
lack of fit. The colour count rises 7 to 13 (A) as n grows, which strengthens propagation per
cell, and the border count steps from 2 to 3 at n=9. The n=7 to n=8 plateau and the n=9 to
n=10 jump in A, and the non-monotone B curve, track those steps.

**Scope.** "Strong" backdoor is used for a weak backdoor relative to the planted
assignment. The hypothesis' practical half needs the strong notion, or at least the cost of
guessing the placements, and that was not measured (stated in the README). The colour
confound is stated in the README.

**Prose.** README house style is clean (no em dashes, no banned words). "Overstating the large
sizes can only raise the fitted exponent" (README.md:128) is correct because n=8 sits above
the mean log N, but only by 0.03. "Constant factor of about 2" (README.md:159) is relative to
the random median (T50), not to best-of-random. The best of 200 is closer.

**Code and reproducibility.** Reproducible byte for byte, tests and clippy pass, an
independent re-implementation agrees, and output is deterministic despite threading
(re-sorted). The exact search at n>=8 certifies only k >= 4, against values of 7 to 11, so
calibration of the greedy rests on n=6 and 6 self-selected easy instances at n=7.

## 8. Findings

`file:line | severity | claim | problem | fix`

- PREREG.md:44 / analyze.py:104-135 | should-fix | "b CI upper < 0.9 rules out roughly linear growth" | Bootstrapping instances within size ignores lack of fit across the five means. A t(3) CI on the 5-point fit is [0.26, 0.92] (A), and the affine law k = 3.08 + 0.065 N fits as well as N^0.589 (RSS 1.05 vs 0.96). The rule cannot tell sublinear growth from linear growth with an offset. | Next round: pre-register an affine-vs-power comparison (or the local slope over the top sizes) and widen the size range so the two separate.
- README.md:1, PREREG.md:11 | should-fix | "strong backdoors" | What is measured is a weak backdoor in Williams-Gomes-Selman terms: the planted values of a cell set make propagation complete. A strong backdoor needs every assignment of those cells to be decided by propagation. The agenda's wording carries the same conflation. | Rename to "weak backdoor (planted assignment)" and keep "strong" for a future measurement that refutes all other placements.
- README.md:161 | should-fix | sublinear growth in cell count | The colour count grows with n (A: 7, 8, 10, 12, 13; border 2 to 3 at n=9), so size and colour count are confounded. The curve's bumps line up with the colour steps. | Add a fixed-colour control series, or report k against expected solutions or propagation strength.
- runner headline | should-fix | "grows as N^0.589" | A 5-point exponent quoted to 3 decimals reads as a law. The README itself says it is not a law. | Quote it as "about N^0.6 over n = 6..10 (5 sizes); an affine law fits equally well".
- README.md:157-160 | nitpick | "chosen set buys a constant factor of about 2" | The comparison is with the random median T50. The best of 200 random subsets is 1.5 to 4.5 cells above k, so the factor is about 1.2 to 1.5 against best-of-random. | State both comparisons.
- src/main.rs:65 | nitpick | exact calibration of greedy | The n>=8 node cap certifies only k >= 4. The calibration (gap 0.40 at n=6, 0.67 at n=7) comes from n=6 and the 6 easiest n=7 instances. | Raise the cap or use a SAT/ILP solver to certify a few n=8 instances.
- README.md:128 | nitpick | upper bounds at n>=8 make the fit conservative | True, but the margin rests on log 64 lying 0.03 above the mean log N. | Cite the worst-case bound (b <= 0.648, CI upper 0.682) instead of the sign argument alone.
- PREREG.md:98 | nitpick | "unit propagation plus all-different" (agenda) | Arc consistency on edges was used, which equals UP only on a support encoding. This was declared before results, so it is not a deviation. | One sentence naming the CNF equivalence.

No blocking issues.

## 9. Next round

The question still open is whether the cost of finding a backdoor without the planted solution
grows more slowly than plain search, and whether k is sublinear in N once colours are held
fixed. Proposed test:

1. Sizes n = 6..14 at the Owen peak, plus a fixed-colour series (for example 2 border and 6
   interior colours for every n), 30 seeds each.
2. Certify minima at n = 8..10 with CP-SAT (ortools via `uv`, since `venv` lacks ensurepip),
   minimising |S| subject to propagation completing. If that is impractical, raise the node
   cap and report certified fractions.
3. Pre-register an affine-vs-power-law comparison (AIC, or the local slope over the top
   three sizes with a CI) as the kill statistic, alongside b.
4. Measure the solver-side half of the hypothesis: with the backdoor cell set known but not
   its placements, count the placements of the backdoor cells that propagation does not
   refute (the strong-backdoor search cost). Compare that with DFS nodes to the first
   solution on the same instances.

Estimated compute: 8 to 16 cores for 4 to 8 hours on CPU (the exact certification at
n >= 10 dominates). One AWS c7i.4xlarge (16 vCPU) for about 6 h is enough. No GPU.
