# Judge report: A4, quality-diversity search over basin descriptors

**Verdict: kill.** The test is valid and its preregistered kill criterion is met. MAP-Elites found fewer distinct near-best basins than parallel tempering at equal compute on both sizes. It was higher on 0 of 60 puzzles (58 PT wins, 2 ties), and the one-sided exact Wilcoxon p is 1 on both sizes. No verified bug, leakage or baseline defect changes the direction.

Reviewed: branch `devin/r1-quality-diversity-basins` at `dd590db`, directory `research/experiments/quality-diversity-basins`. The branch was not modified. The judge's work ran in a detached worktree and in scratch directories outside the repo.

## 1. Preregistration and kill criterion

- Commit order: `6d116b4` PREREG.md (11:30:19 UTC), then `20480d4` engine and runner (11:30:27), `0a31297` calibration, `analyse.py` and recipe (11:35:12), `54254b5` main results and README (12:12:27), and `dd590db` reproduction note (12:41:46). PREREG.md comes before every result file. The analysis script was committed before the main-grid results. PREREG.md has no later edits.
- Kill criterion: the agenda's A4 entry reads "No more distinct basins than PT at equal compute on both sizes." PREREG.md quotes this verbatim. Its operational reading (killed unless ME is significantly greater on at least one size after Holm at 0.05, one-sided exact Wilcoxon on puzzle-paired means) is a faithful and lenient rendering.
- `PREREG_AMENDMENTS.md` does not exist, and there is nothing it would need to cover. The one exploratory analysis (`scripts/exploratory_own_best.py`, which thresholds each method on its own best) is labelled exploratory and also favours PT. The outcome rests on no post hoc analysis.

## 2. Recomputed numbers

The judge's own script (`recompute.py`, SciPy and NumPy, reading only the committed `results/basins.jsonl`) gave these results:

| | Reported | Recomputed |
|---|---|---|
| 8x8 B(2,0.25) PT / ME | 22.33 / 5.63 | 22.33 / 5.63 |
| 8x8 mean diff ME-PT [95% CI] | -16.70 [-20.43, -13.20] | -16.70 [-20.48, -13.22] (separate bootstrap RNG) |
| 8x8 ME>PT / PT>ME / ties | 0 / 30 / 0 | 0 / 30 / 0 |
| 8x8 one-sided Wilcoxon p | 1 | 1 (SciPy) |
| 10x10 B(2,0.25) PT / ME | 5.30 / 1.00 | 5.30 / 1.00 |
| 10x10 mean diff [95% CI] | -4.30 [-5.28, -3.35] | -4.30 [-5.30, -3.37] |
| 10x10 ME>PT / PT>ME / ties | 0 / 28 / 2 | 0 / 28 / 2 |
| 10x10 one-sided p | 1 | 1 |
| Basins per core-hour 8x8 PT / ME | 1019 / 244 | 1019 / 244 |
| Basins per core-hour 10x10 PT / ME | 538 / 105 | 538 / 105 |
| Best score 8x8 PT / ME (of 112) | 105.40 / 104.52, p=2.7e-7 | 105.40 / 104.52, exact two-sided p=2.68e-7 (own DP) |
| Best score 10x10 PT / ME (of 180) | 164.40 / 162.57, p=3.6e-7 | 164.40 / 162.57, exact p=3.58e-7 |
| Total main-grid CPU | 14,367 s | 14,367.06 s |

The bootstrap intervals differ only in the second decimal because the resampling RNG differs. Every point estimate, count and p-value matches.

## 3. Rerun

The judge ran `just experiments quality-diversity-basins` in full from a clean worktree at `dd590db` on the 8-vCPU VM (7 parallel). It took 31 min 9 s wall time. The regenerated `basins.jsonl` matches the committed file in **240 of 240 rows** once `cpu_s` and `wall_s` are removed. The regenerated `summary.json` reads `outcome: killed` with an identical sensitivity table.

## 4. Independent re-derivation of the basin count (reduced instance)

The engine stores only counts in `basins.jsonl`, so the judge wrote a separate Python implementation (`indep_basins.py`). It used its own edge scorer on the generator's pieces, its own whole-board rotation (`new[r][c] = old[n-1-c][r]`, piece turned once), its own distance minimised over four rotations, and its own leader clustering. It took the regenerated per-run candidate files as input. The reduced instance was 8x8, puzzle seeds 1 to 9, 36 runs:

- Every best board and every one of the stored candidates rescored to exactly the stored score (0 mismatches). Candidates are exactly unique.
- `k2_boards` and `B(2,0.25)` matched the engine in **36 of 36 runs**.
- As a check on the clustering choice, an order-free count (connected components of the "distance < 0.25" graph) gives lower numbers for PT (for example puzzle 1: 33.5 leaders vs 20.5 components). PT still exceeds ME on all 9 puzzles (component means PT 15.7, ME 4.4).

## 5. Leakage, compute and baseline

- **Planted solution.** Puzzles come from `generate` with 0 hints. Both methods start from `greedy_fill_initial` on `Board::empty` with the same replica seeds. No code path gives either method the solved board. The `planted_solution_scores_full` test only checks the scorer. 0 of 120 runs per size reached a full score, as expected for calibrated hard settings.
- **Equal compute.** Evaluations match exactly per run (320M at 8x8, 160M at 10x10, 16,000 and 8,000 bursts of 20,000 steps for both methods). The measured CPU ratio ME/PT is 1.06 and 0.98 (rerun 1.03 at 10x10). Both methods use the same `run_sa_steps_fixed_temp` kernel, the same ladder and the same burst length.
- **Baseline.** PT is a sequential re-implementation of the repo's `run_pt_from` default path. It omits the early stop, which is required so both methods spend the full budget. `pt_matches_repo_run_pt_from` passes and checks the best score and board against the repo function. The baseline is the agenda's baseline and is not weakened.
- **Candidate opportunity.** Both methods offer one burst-best board per 20,000-step burst, so each gets the same number of offers into the same pool rule.

## 6. Rigor checklist (adapted)

- Null model / baseline fairness: PT is the agenda's named baseline at matched evaluations and CPU. Pass.
- Claims no stronger than the data: the README claims "killed for this implementation" and lists the untested QD designs (README.md:169). Pass.
- Exact search kept separate from record hunting: neither applies. These are generated small puzzles with no record claims.
- Hardware and literature dated: the hardware is recorded. MAP-Elites is described at the level used. Pass.
- Numbers against committed raw results: all reproduced (section 2). Pass.
- Cross-page consistency: AGENDA, LEDGER, index.json and web/ are untouched, so no page restates the result yet. Whoever records the kill should link this directory.

## 7. Adversarial review

There were no parallel subagents. The four lenses were run as separate sequential passes by the judge. Each serious candidate finding was checked by computation before it was counted.

- **Design validity.** The main threat was that the candidate definition favours PT. PT replica chains keep moving, while ME restarts bursts from a small set of elites. An ME burst that fails to improve on its elite returns the elite again, and deduplication drops it. Boards that only enter the archive as burst-final boards are never counted. The judge declared a sensitivity subset in advance (8x8, puzzles 1 to 3, both seeds, both methods, full 320M budget) with a variant that also offers burst-final boards (ME) and current replica boards (PT) to the pool. Per-puzzle means for ME rose from 4.5 / 8.0 / 6.0 to 6.5 / 12.5 / 7.0. PT was unchanged at 33.5 / 12.5 / 32.5, because the offers consume no RNG. The result is one tie and no ME win, so this lowers the margin but does not reverse the conclusion. Leader-clustering order dependence was also checked (section 4) and does not change the direction. The quality guard was not triggered, and it did not need to be, since ME did not win.
- **Scope.** One ME variant was tested (96-cell grid, uniform elite selection, uniform temperature from the ladder including T=2.0, burst restarts). There was one colour count per size, the lowest candidate, chosen by the preregistered rule. The README states all of this. The kill covers the agenda item as specified, not quality-diversity in general.
- **Prose.** README.md and PREREG.md contain no em dashes and no banned words. Numbers in the prose match the summary. "Per measured core-hour the gap is the same" (README.md:103) is accurate because CPU per run is nearly equal.
- **Code and reproducibility.** `cargo build --release` and all three crate tests pass. The full recipe reproduces bit-identically apart from timings. One weakness: per-run candidate files (34 MB) are not committed, so the basin counts can be re-derived only by rerunning (about 30 min on 7 cores).

## 8. Findings

`file:line | severity | claim | problem | fix`

- `engine/src/main.rs:374 | minor | ME candidate pool is the burst-best board | ME bursts restart from archive elites, so many burst-bests repeat an elite and are deduplicated; archive boards inserted as burst-final boards are never counted | also offer burst-final boards or count archive elites; judge variant on 3 puzzles: ME 6.5/12.5/7.0 vs PT 33.5/12.5/32.5, direction unchanged`
- `engine/src/main.rs:197-199 | minor | B(k,delta) counts distinct basins | leader clustering is order-dependent and gives PT higher counts than an order-free count on dense near-best sets (8x8 p1: 33.5 vs 20.5) | report connected components as a secondary statistic; PT > ME on 9/9 puzzles under it`
- `README.md:92 | info | basin counts are reproducible | per-run candidate boards are not committed, so counts cannot be re-derived from committed data without the 30-minute rerun | commit a compressed candidate file or a per-run hash of the candidate set`
- `engine/src/main.rs:367 | info | ME keeps many basins alive | burst temperature is drawn uniformly from the ladder, so a quarter of ME bursts run at T >= 1.0 from elites, which is one design choice among many | a next-round QD design should tune or anneal burst temperature; covered by the README scope note`
- `results/calibration.json | info | result at two sizes | one colour count per size (8, 10), the lowest candidate, each with 0/8 calibration solves | scope only; no fix required for this kill`

## 9. Next round

A4 as stated is killed and should be recorded in the ledger as such. A revival would be a new agenda item with a different design rather than a rerun: an archive-counted ME (archive contents in the basin pool), curiosity- or novelty-weighted cell selection, a cold-temperature burst schedule, and a PT-plus-archive hybrid, scored with an order-free basin count. A first pass at 8x8 and 10x10, 30 puzzles, 2 seeds and 3 variants plus PT costs about 12 core-hours, or about 2 hours wall time on 7 cores.
