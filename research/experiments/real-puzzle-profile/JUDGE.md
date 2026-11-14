# JUDGE: track 10, real-puzzle-profile (measurement)

Branch `night/2026-11-12-t10-real-puzzle-profile`, reviewed at `06cde25`. The
judge did not run the experiment and read only the branch. Nothing under
`results/` was modified. Reruns used binaries the judge built from the branch's
committed source into its own target directories, on at most 2 cores, about
18 core-minutes on the real pieces.

## Verdict: keep (measurement), with three wording fixes required in RESULTS.md

M2 (five-clue row-scan tree 10^40.457, 95% interval 10^40.440 to 10^40.473) is
a valid, gated, reproducible measurement and agrees with the independent
first-moment price. M1 (late-node share about 1e-6 under the `record` unlock
list) is valid as a measurement of this port at 1e10 to 1e11 nodes, but by the
preregistered rule it is **flagged budget-dependent**, so the decision reading
at the 1e15 regime is argued, not measured. The headline must say so (fix 1).

## 1. Preregistration and run log

- `PREREG.md` committed in `3c54899` at 2026-11-12 21:50:54 +01:00. A1 (M2
  replaced by the price review's specification) at 21:53:10. Engine and
  `run.sh` at 21:56:39. Every results file first appears in `16b1708` (22:01)
  or later.
- `results/run-log.jsonl`: 44 start lines, all carrying
  `prereg_commit 3c54899 ... 21:50:54` and `prereg_sha256 38a80fac...`, which
  equals the SHA-256 of the committed PREREG.md (checked). The first line
  (stage `equiv`) is at 20:56:44 UTC = 21:56:44 local, after both the PREREG
  and A1 commits. No run preceded A1, so A1 is a before-data amendment.
- Every result row has a matching start/end pair. The two `gate201` runs have
  start lines without end lines and no row, as A7 and RESULTS.md state. The
  `m1b-val.jsonl` file was written three times (A2, A3, A4); only the last
  survives, which A4/A5 describe.
- 13 start lines carry `dirty_paths` 1 or 2 (record s5, s6, gate221 s1 to s6,
  the three m1b runs). The dirty file is not named by the guard. The diff
  between those commits is M1b code only (`bwsmc`, `bwval`, `capture`), and the
  judge's reruns of `record` s5 and `gate221` s1 from HEAD reproduce the
  committed histograms exactly (section 3), so the dirty tree did not change
  the `bw` search.
- A2 to A8 were written after data and are labelled so. Cuts (A7) go beyond
  the PREREG cut order for `gate201` seeds 3 to 6, disclosed.

## 1b. Track label

Measurement, no ordering method compared against a baseline, so the
node-sequence-hash reclassification check does not apply. No arm-versus-arm
claim is made.

## 2. Recomputed numbers (judge's own script over the committed raw files)

| Quantity | RESULTS.md | Judge |
|---|---|---|
| M1 `record` share >= 201 at 1e10, median [min, max], 6 seeds | 4.5e-7 [4.0e-7, 2.0e-6] | 4.48e-7 [4.01e-7, 2.03e-6] |
| M1 `record` median 1e10 / median 1e9 | 2.9 | 2.89 |
| M1 `record-1e11` final, seeds 1, 2 | 1.4e-6, 5.4e-7 | 1.37e-6, 5.35e-7 |
| M1 `record-noquota` median [min, max] | 1.4e-7 [8.1e-8, 2.2e-7] | 1.42e-7 [8.06e-8, 2.25e-7] |
| M1 `gate221` share after first unlock | 0 (never reaches 221) | 0, deepest 204 to 205 |
| M1 histogram peak depth, `record` | 161 | 161 in all seeds |
| M1 nodes at >= 201 per node at 201, `record` | 35 to 71 | consistent (seed 1 at 1e11: 60.4) |
| M2 five clues N = 1e6 | 40.457 [40.440, 40.473] | 40.457 [40.440, 40.473], span 0.035 |
| M2 five clues N = 1e5 | 40.497 [40.414, 40.567] | same |
| M2 centre clue | 47.140 | 47.140 [47.061, 47.208] |
| M2 no clue | 50.039 | 50.039 [50.017, 50.059] |
| M2 sum to free cell 175, N = 1e6 | 40.453 | 40.453 |
| Null (20 painted instances) median, min, max, sd; official rank | 40.61, 40.33, 40.76, 0.125; 5 below | 40.606, 40.329, 40.759, 0.125; 5 below |
| Gate (a) pairs > 3 s.e. | 10 of 1,080 | 10 of 1,110 (0.90%); median abs log10 error 0.0050, max 0.0226 |
| Gate (b) mean frames / exact | 0.99994, z = -0.01 | 0.99994, z = -0.013 |

Every headline number reproduces. The gate (a) denominator differs (1,110
pairs = 30 instances x 37 levels including the root, against 1,080); the
verdict is unchanged.

## 3. Reruns (judge-built binaries)

- M1, 1e10 nodes each: `record` s5 (a dirty-tree run), `gate221` s1 (dirty),
  `record` s3 (the outlier seed). Per-depth histograms and all checkpoints are
  **identical** to the committed rows.
- M2: five clues N = 1e5 seeds 201 to 205, all five totals and extinction
  depths identical; gate (b) seeds 1 to 3, identical totals and final widths.
- `scripts/frames.py` and `scripts/analyze.py` rerun with no diff in
  `results/`.
- Equivalence of the port with `blackwood_bt`: the preregistered check
  compares the node total at the cap, which is equal by construction, plus the
  deepest depth, a weak check. The judge extended it to seeds 1 to 3 at caps
  1e6, 3e6, 1e7, 3e7, 1e8, 3e8: deepest depth agrees at **18 of 18** points,
  across the jumps from depth about 50 to 194 to 209. This is adequate
  evidence that the port follows the same search.

## 4. Findings

1. **M1 headline overreaches the preregistered rule (fix required).**
   PREREG.md: "Uninformative if the share at 1e10 differs from the share at
   1e9 by more than a factor 2 in the median (budget-dependent: then the 1e15
   regime of a record run is not reached by extrapolation and the report says
   so)". The ratio is 2.89. The body says "flagged budget-dependent", but the
   headline reads the 5% decision line as settled. The share is so far below
   5% (factor 10^5) that the qualitative reading is likely right, and the
   ratio is driven by zeros at 1e9 in two seeds, but the 1e15 statement is
   argued from two seeds at 1e11 whose per-decade growth was 1.9 and 1.3, and
   seed 1 is non-monotone (6.6e-6 at 1e8, 7.1e-7 at 1e10, 1.4e-6 at 1e11).
   Fix: in the Headline, add that M1 is flagged budget-dependent by the
   preregistered rule, that the value is measured at 1e10 and 1e11 only, and
   that "below the 5% line at a record run's budget" is argued.
2. **False statement about the no-clue model (fix required).** RESULTS.md says
   "The no-clue tree has no model figure in the memos". `solve-path-5-price.md`
   line 111 gives 10^50.03 (peak 10^48.76 at 161). The measurement 10^50.039
   (peak 48.76 at 161) matches it. Fix: replace the sentence with the
   comparison (+0.009 dex), which strengthens the M2 reading.
3. **Gate (a) denominator (fix required, cosmetic).** 1,080 should be 1,110,
   or state which level is excluded.
4. **Scope of M1 is the `record` list only.** The one schedule where the late
   region may dominate, `gate201` (10 breaks from 201), did not finish, and
   `gate221` never reached its gate. RESULTS.md says this; any downstream
   ledger or agenda entry must not generalise "late share about 1e-6" beyond
   the `record` list or beyond this port (no Blackwood colours, quota, scan or
   restarts).
5. **M2 independence.** The model figures (10^40.49, 10^47.14, 10^50.03) are
   an analytic first-moment computation (`solve-path-5-price.md`, reviewed in
   `price-and-ledger.REVIEW.md`), not an SMC, so the agreement is not
   circular. Caveat recorded by the runner and confirmed: the population
   collapses to about 300 effective families by free depth 175 and dies by
   188 to 204, so the 5-repetition interval cannot exclude a heavy-tailed
   subtree the population never visits. The N = 1e5 and N = 1e6 estimates
   agree (larger N gives a slightly lower, not higher, value), gate (b) hits
   the exact 16x16 frame count, and gate (a) passes, so the estimate is
   accepted at the stated precision.
6. **M1b correctly reported as not measured.** Gate failed by its own rule,
   estimator degenerate. No number from it is used.
7. **Frame stand-ins uninformative**, reported so. Correct.

## 5. Checklist (rigor-check and adversarial-review, adapted)

- Null model: present for M2 (painted census ensemble, 20 instances, official
  set ranks 6th of 21). No null is needed for M1 (a descriptive share).
- Title and description no stronger than body: fails only at finding 1.
- Exact search separate from record hunting: yes, stated "not a record
  attempt".
- Planted-solution leakage: the painted null instances carry one planted
  solution each; it adds about one leaf to a 10^40 tree, irrelevant.
- Censored runs counted as results: no. Killed `gate201` runs report no share.
- Silently dropped seeds: none. All cuts are in A7/A8.
- Wall clock: not a speed claim; M1 throughput about 5e7 nodes/s, `gate201`
  much slower per node, disclosed.
- Grading: measured / reported / argued labels are present and, apart from
  finding 1, correct.
- Explored and dead-end pages: not relied on.

## Unplanned observations

- The painted null sits +0.12 dex above the model in the median (15 of 20
  above the official set). Small and one-sided; worth one sentence in the
  model's caveats, not a correction.
- The DFS spends tens of millions of nodes stuck near depth 50 before jumping
  past 190 (deepest depth 49 to 54 up to 3e7 nodes, then 194 to 209), for both
  engines. The quota prune is what holds it there; `record-noquota` has a
  lower late share than `record`, so the quota is not the reason the share is
  small.
- The preregistered equivalence test (node total at the cap) is vacuous as
  written; future ports should compare a per-depth histogram or the deepest
  depth at several caps.
