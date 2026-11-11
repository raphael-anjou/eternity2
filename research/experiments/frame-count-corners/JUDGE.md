# JUDGE: A5, exact count of frame placements with the corner constraint

Branch `devin/r1-frame-count-corners`, reviewed at `c4e23dc`. Judge session did
not run the original experiment. Nothing on the branch was modified.

## Verdict: keep

The test is valid and the pre-registered kill criterion is not met. K1 (M1
disagrees with brute force on a reduced ring): 0 mismatches on the 34
instances where M3 finished, and 0 mismatches on all 45 instances plus 17
extra larger instances against a judge-written board-level count that does not
share the experiment's ring abstraction. K2 (state space does not fit in
memory): three official runs exited 0 at 17.49 GiB peak RSS against a 30 GiB
threshold. The validation-thinness guard (at least 30 of 45 finished) is met
with 34. The official integer
N = 40519056449429419697492635191544381440 reproduces from the committed
residues, from a clean rerun of the `just` recipe (all three primes bit
identical), and agrees
with a judge-written 4e9-sample Monte Carlo to 0.22 percent (z = +0.74).

## 1. Pre-registration

- `PREREG.md` was added in `6fc2fa0` (2026-11-11 11:35 UTC), alone in that
  commit. All code and every file under `results/` first appear in `e9fa190`
  (12:21 UTC); `c4e23dc` (12:59 UTC) only edits `README.md`. Order is correct.
- `PREREG.md:94-95` quotes the agenda kill criterion verbatim ("Method
  disagrees with brute force on the reduced ring, or the state space does not
  fit in memory."), matching `research/program/AGENDA.md` A5.
- The operationalisation (`PREREG.md:97-110`) is fixed before results: exact
  equality, one mismatch kills; 30 GiB RSS or OOM kills; fewer than 30 of 45
  M3 completions is inconclusive. The analysis plan, cell list (9 cells x seeds
  1 to 5), node cap 2e10, three primes and the MC comparison formula are all
  pre-registered.
- No `PREREG_AMENDMENTS.md` exists and `PREREG.md` is unchanged after
  `6fc2fa0` (`git log` on the path shows one commit). The README states "No
  amendments". Nothing post hoc was found: the reported statistics are exactly
  the pre-registered ones.
- The secondary MC comparison is reported as pre-registered and is
  explicitly non-binding (`PREREG.md:117-121`).

## 2. Recomputed numbers (judge scripts, outside the repo)

Judge script reading `results/official_p{0,1,2}.json`, the official piece CSV
and `research/topics/ring-purity/results/ring_count.json`:

| quantity | experiment | judge |
|---|---|---|
| arborescences (Bareiss determinant of the 5-colour Laplacian) | 3432 | 3432 |
| EC = 3432 x (11!)^5 | 3.47797030249103657835e41 | same integer |
| prod m_ab! | 660451885056 | 660451885056 |
| CRT pairs {0,1},{0,2},{1,2} | identical | identical, W6 = 15337626164089830494138310 |
| N_cyc = prod m_ab! x W6 | 10129764112357354924373158797886095360 | same |
| N = 4 N_cyc | 40519056449429419697492635191544381440 | same |
| log10 N | 37.60766 | 37.60765932356936 |
| f_exact = N_cyc / EC | 39327246574589308959329/1350268319657182464000000000 | same fraction, 2.9125504910441832e-05 |
| committed MC f, n, hits | 2.775e-5, 16,000,000, 444 | same |
| z with committed s.e. 1.3169e-6 | -1.04 | -1.0445 |
| z with s.e. evaluated at f_exact (1.3492e-6) | not reported | -1.0195 |
| exact binomial P(X <= 444 given f_exact) | not reported | 0.160 (two-sided 0.32) |
| MC 95 percent interval | [2.517e-5, 3.033e-5] | [2.5169e-5, 3.0331e-5], contains f_exact |
| peak RSS p0, p1, p2 (from `.time`) | 17.49 GiB | 17.4935, 17.4853, 17.4867 GiB, exit 0 |

Uniqueness of the CRT result: W6 <= EC / prod m_ab! = 5.3e29, far below the
product of any two of the 61-bit primes (about 5.3e36), so the pairwise
agreement fixes the integer.

### Independent baseline check (judge Monte Carlo)

A judge-written C sampler (`mc.c`) on the official arc multigraph rebuilt from
the engine piece CSV: uniform Euler circuit by Wilson arborescence plus random
out-edge orders (BEST bijection), hit when the four corner arcs sit at
positions spaced 15 apart.

| run | samples | hits | f | z vs f_exact |
|---|---|---|---|---|
| seed 1 | 40,000,000 | 1,162 | 2.905e-5 | -0.09 |
| seed 20261111, 3 threads | 3,999,999,999 | 116,755 | 2.91888e-5 (s.e. 8.53e-8) | +0.74 |

The 4e9-sample estimate is 0.22 percent from f_exact and its 95 percent
interval [2.9022e-5, 2.9356e-5] contains it. This independently confirms
the official exact value to about three significant figures, which is the
only part of the official result that M1 alone otherwise carries.

### Independent exact re-derivation on reduced rings

A judge-written Python counter (`indep.py`) works from the raw piece list of
`generate_framed(n, colors, seed, true)` (dumped by a separate small Rust
binary), not from the experiment's `ring_of` / `Ring` abstraction. It handles
corner and edge rotation on the actual board cells itself, sweeps the border
cell by cell clockwise, and counts labelled placements by a DP over (cell,
multiset of used pieces, last colour) with the four corner cells fixed in
place. For n = 5 it was also checked against a labelled brute force over the
board (6 instances, 0 mismatches).

| set | instances | judge = N_M1 |
|---|---|---|
| all 45 pre-registered reduced instances | 45 | 45/45 |
| of which M3 capped (6x6/3 s4, 7x7/4 s1-5, 8x8/5 s1-5) | 11 | 11/11 |
| extra 9x9/6, 10x10/6, 10x10/22 (side 7 and 8, 5 frame colours) | 15 | 15/15 |
| extra 12x12/22 (side 10, 40 edge pieces, 5 frame colours) | 2 finished (seeds 1, 2) | 2/2 |

The extra instances were run with the final experiment binary (`--cap 1`, so
M1 only) and compared against the judge counter. Seeds 3 to 5 of 12x12/22 did
not finish inside the judge counter's 50 minute timeout (Python) and are not
counted. Example: 12x12/22 seed 1, N = 5552793081497592004608 by both.

### Rerun of the recipe

`just experiments frame-count-corners` from a clean worktree at `c4e23dc` with
the final (post-clippy) code, run concurrently with judge jobs. Reduced: both
`reduced_m1_m3.jsonl` and `reduced_m2.jsonl` are identical to the committed
files on every field except wall-clock seconds (45/45 rows, including
`M3_nodes` and all W(sigma)). Official: `official_p{0,1,2}.json` are identical to the committed files on
every field except `seconds` and `sigma_seconds` (all six W(sigma) residues,
max layer 527625153, max join 108618782). Peak RSS 18344560, 18341960 and
18338684 kB (17.49 GiB each), exit 0, wall 13:30, 14:14, 14:13 (slower than
committed because judge jobs shared the CPUs). The regenerated `summary.json`
differs from the committed one in 129 leaves, all timing or RSS fields; N,
f_exact, z and every count are identical. Total rerun about 60 minutes, so the
full recipe was run rather than a subset.

## 3. Rigor checklist (adapted to an experiment)

- **Null model and baseline fairness.** The baseline is a Monte Carlo
  estimator of the same quantity under the same directed-multigraph model; it
  is not a competing method, so compute parity is not the question. The
  comparison is consistency, and it is reported as non-binding. The baseline
  was taken as committed, not tuned. The judge's own sampler agrees with both.
- **Leakage of the planted solution.** Reduced rings come from a solved-board
  generator, but M1, M2 and M3 receive only the shuffled, rotated piece list
  through `ring_of`; no placement or seed-derived position reaches the
  counters. The judge counter also uses only pieces. A planted solution
  guarantees N >= 1 on each reduced ring; it does not make agreement between
  methods easier. No leakage.
- **Claims no stronger than the data.** README states the exact count, its
  consistency with MC, and explicitly bounds the scope (brute force only to
  n = 8, official value on M1 alone, M2 not run on the official ring). That
  matches the evidence. The judge's extra checks strengthen it.
- **Numbers against committed raw results.** All headline numbers recompute
  (section 2).
- **Scope.** Counts frame placements with fixed corner cells and no rotation
  quotient; interior-facing colours ignored. Defined at `README.md:8-15`, and
  consistent with `ring_count.py`'s `log10_valid_frame_placements` (4 f EC).
- **Site-wide grep.** The "about 10^37.6" statement on several site pages
  (`ring-purity.mdx`, `rigidity-wall.mdx`, `theorem-sweep.mdx`,
  `rare-color-geography.mdx`) remains correct (exact 10^37.6077).
  `research/topics/ring-purity/article.md:74` and
  `web/content/research/why/ring-purity.mdx:173` still call the exact count
  open; that is for the follow-up publication step, not a defect of the test.

## 4. Adversarial review

Parallel subagents were not used; the four lenses were done as separate
passes by the judge, and every finding below was verified against source or
raw results.

- **Design validity.** K1 compares M1 with M3, but both consume the same
  `ring_of` reduction (`src/main.rs:33`, `src/main.rs:458-475`), so a bug in
  the piece-to-arc translation (rotation convention, corner arc orientation)
  would be shared and invisible to K1. The judge counter does not use
  `ring_of` and agrees on 62 instances, so this risk is closed for this
  verdict. M2 also shares `ring_of` output (it reads the M1 jsonl).
- **Scope.** The brute-force-validated rings have side length at most 6 and
  at most 21 edge types; the official ring has side 14 and 24 types with
  max layer 5.3e8 states. Agreement at small size does not by itself prove
  the large run is free of overflow or hashing faults, but residue mode is
  tested against exact u128 on 45/45, CRT is consistent across three primes,
  and the judge MC matches to 0.22 percent.
- **Prose.** No em dashes or banned phrases in `README.md`, `PREREG.md`,
  scripts or source. Claims are hedged appropriately.
- **Code and reproducibility.** The recipe in `research/experiments/justfile`
  matches the README commands. The official results in `e9fa190` were
  produced by a pre-clippy binary (README note); the judge rerun with the
  final code reproduces them (see above).

## 5. Findings

`file:line | severity | claim | problem | fix`

| file:line | severity | claim | problem | fix |
|---|---|---|---|---|
| research/experiments/frame-count-corners/src/main.rs:458-475 | low | K1 checks M1 against an independent brute force | M3 (`Brute::new(&r)`) and M2 (reads the M1 jsonl) consume the same `ring_of` reduction as M1, so a piece-to-arc bug (rotation, corner arc direction) would be shared and invisible to K1 | add a board-level brute force from raw pieces to `cargo test`; the judge counter from raw pieces agrees on 62 instances, so not blocking |
| research/experiments/frame-count-corners/README.md:136-140 | low | official value rests on M1 alone | correct as stated; the only independent check of the official integer is Monte Carlo precision (judge 4e9 samples: 0.22 percent, z = +0.74) | run a second exact implementation on the official ring (modular M2 or a Rust board DP) before the number is published as exact on the site |
| research/experiments/frame-count-corners/PREREG.md:117-118 | info | z = (f_MC - f_exact) / 1.317e-6 | s.e. evaluated at f_MC rather than at f_exact; z = -1.02 at f_exact versus -1.04; exact binomial two-sided p = 0.32 | none needed; optionally report both |
| research/experiments/frame-count-corners/PREREG.md:106-107 | info | validation not thin (34 of 45) | the 11 capped instances are exactly the high-multiplicity cells (6x6/3 s4, 7x7/4, 8x8/5), so brute force coverage is skewed toward rings with few repeated types | judge counter covers all 11 (11/11); next round should pick cells where brute force finishes at higher multiplicity |
| research/experiments/frame-count-corners/README.md:3-4 | info | pre-registration pushed before any run | git shows commit order (11:35 vs 12:21 UTC), not push time | none; commit order is the verifiable claim and holds |
| research/experiments/frame-count-corners/README.md:69-72 | info | clean checkout reproduces | official results in `e9fa190` came from a pre-clippy binary | judge rerun at `c4e23dc` reproduces every non-timing field; none |
| research/topics/ring-purity/article.md:74 | low | "An exact count with the corner constraint is still open." | now superseded by this experiment | update in the publication step, with the same change at web/content/research/why/ring-purity.mdx:173 (protected files, outside this test) |

## 6. Next round

A5 is closed as an exact count. Two follow-ups, in order:

1. Second exact implementation on the official ring, before the integer is
   published as exact: port the judge's board-level DP (cell sweep over
   multisets of edge types with the four corners fixed, no `ring_of`) to Rust
   with the same three primes, and require bit-identical residues. Estimate
   8 cores, under 2 hours, under 20 GiB, by the same layer-size argument.
2. A new agenda entry, not part of A5: extend the transfer matrix to the
   second ring (frame plus ring-2 adjacencies through the interior-facing
   colours) and measure state-space growth against a 31 GiB budget, with
   brute force on 6x6 and 7x7 generated boards as the kill check. Estimate
   one session, 8 cores, a few hours; kill if the ring-2 layer exceeds memory
   on 10x10.
