# Hygiene audit, 2026-11-13

Read-only audit of working state. Nothing was edited, committed, pushed or fetched. Counts "ahead" use the local `origin/main` ref as it stood (no fetch was run).

## 1. Worktrees and branches

Commands: `git worktree list --porcelain`; per worktree `git rev-parse --abbrev-ref HEAD`, `git log -1 --format='%h %s'`, `git rev-list --count origin/main..HEAD`, `git status --short`; remote presence from `git branch -r` (match on `origin/<branch>`).

Format: worktree | branch | head | ahead of origin/main | on remote | modified or staged paths, untracked paths. Paths are listed under each line (cap 30).

Only `docs/2026-11-13-corrections` (wt-pub, ahead 0) exists on the remote. Every night and day track branch is local only.

```
## eternity2 | night/2026-11-12-lead | 637a42be research(program): track 15 closed, draft ledger entry, night complete | ahead=109 | remote=no | dirty=0 untracked=8
    ?? best_so_far.json
    ?? bw_portfolio.log
    ?? bw_stage4_score373_1794525557.json
    ?? bw_stage4_score378_1794525556.json
    ?? bw_stage4_score381_1794525534.json
    ?? bw_stage4_score381_1794525557.json
    ?? bw_stage4_score429_1794525561.json
    ?? bw_stage4_score440_1794525600.json
## wt-d1 | night/2026-11-13-d1-frame-cores | 8b0c5b86 research(frame-cores): README method matches A1 and A2 | ahead=120 | remote=no | dirty=0 untracked=1
    ?? research/experiments/frame-cores/results/
## wt-d2 | night/2026-11-13-d2-port-fidelity | b9587fe5 docs: commit second judgment raw outputs | ahead=132 | remote=no | dirty=0 untracked=0
## wt-d3 | night/2026-11-13-d3-orders-and-pins | 1078616f research(orders-and-pins): Part B results, README, run log; Part A pending collec | ahead=150 | remote=no | dirty=0 untracked=0
## wt-d4 | night/2026-11-13-d4-prefix-statistics | edbae965 research(prefix-statistics): unplanned AC follow-up, unrefuted share by depth on  | ahead=155 | remote=no | dirty=0 untracked=0
## wt-d5 | night/2026-11-13-d5-pin-map | 8cfd7fe5 docs(pin-map): weaken over-claims per judge, declare A2 pair-ratio deviation, fix | ahead=155 | remote=no | dirty=0 untracked=0
## wt-docs | night/2026-11-13-docs | bafbe7af fix: reword banned phrases in experiments log at extraction and in the data file | ahead=201 | remote=no | dirty=0 untracked=0
## wt-lead-1113 | night/2026-11-13-lead | 480a8484 research(program): thesis plan draft 2, tracks and seven gaps folded in | ahead=182 | remote=no | dirty=0 untracked=0
## wt-pub | docs/2026-11-13-corrections | 40ad5fb0 docs: repo guides match the corrected records and drop em dashes | ahead=0 | remote=yes | dirty=0 untracked=0
## wt-t0 | night/2026-11-12-t0-node-cost | 0860e322 docs(mrv-node-cost): record the output layout deviation as amendment A1 | ahead=8 | remote=no | dirty=0 untracked=0
## wt-t1 | night/2026-11-12-t1-9x9-pilot | 0af81f7a chore(mrv-scaling-pilot): cargo fmt, fetch.sh cd guard, README judge reading | ahead=20 | remote=no | dirty=0 untracked=0
## wt-t10 | night/2026-11-12-t10-real-puzzle-profile | dc71ad56 research(real-puzzle-profile): judge's three wording fixes, cargo fmt | ahead=66 | remote=no | dirty=0 untracked=0
## wt-t11 | night/2026-11-12-t11-difference-map-bands | d1ed89cc docs(difference-map-bands): post-judgment section, exact censored medians, cargo  | ahead=65 | remote=no | dirty=0 untracked=0
## wt-t12 | night/2026-11-12-t12-cut-keyed-replay | 606e7e52 docs(cut-keyed-replay): corrections after the judgment | ahead=24 | remote=no | dirty=0 untracked=0
## wt-t13 | night/2026-11-12-t13-quota-keying | ba921de3 docs(quota-keying): README corrections after the judgment | ahead=82 | remote=no | dirty=0 untracked=0
## wt-t14 | night/2026-11-12-t14-five-clue-keying | 4ce8a55e docs(five-clue-keying): corrections after the judgment | ahead=89 | remote=no | dirty=0 untracked=0
## wt-t15 | night/2026-11-12-t15-five-clue-quota | b0a185c2 research(five-clue-quota): cargo fmt on the engine, whitespace only | ahead=95 | remote=no | dirty=0 untracked=0
## wt-t2 | night/2026-11-12-t2-blocker-backjump | 388b4167 docs(a20): report tabu-clear counts in README | ahead=8 | remote=no | dirty=0 untracked=0
## wt-t3 | night/2026-11-12-t3-value-impact | a2656271 style: cargo fmt on value-impact-order compute | ahead=5 | remote=no | dirty=0 untracked=0
## wt-t4 | night/2026-11-12-t4-hall-on-mrv | d5134870 docs(hall-on-mrv): correct R7 calibration seed count, add prereg amendment | ahead=9 | remote=no | dirty=0 untracked=0
## wt-t5 | night/2026-11-12-t5-baseline-tournament | 2fcbc132 docs(baseline-tournament): README corrections after the judgment, lockfile note | ahead=17 | remote=no | dirty=0 untracked=0
## wt-t6 | night/2026-11-12-t6-dead-node-census | 966e9c0e docs(dead-node-census): README verdicts follow the judge, amendment 6 | ahead=15 | remote=no | dirty=0 untracked=0
## wt-t7 | night/2026-11-12-t7-partial-match-ladder | 65d05984 docs(partial-match-ladder): README corrections after the judgment | ahead=21 | remote=no | dirty=0 untracked=0
## wt-t8 | night/2026-11-12-t8-score-bound-gap | c96a6d73 research(score-bound-gap): README corrections after the judgment | ahead=15 | remote=no | dirty=0 untracked=0
## wt-t9 | night/2026-11-12-t9-resumable-runs | 6e60130a docs(resumable-runs): correct the forged stack-drop cause, mark v2 cross-machine  | ahead=20 | remote=no | dirty=0 untracked=0
```

Note: the 24 worktrees besides the main repo include wt-t0 to wt-t15 (night 2026-11-12 tracks) as well as the ones named in the request. All are clean except wt-d1 and the main repo (untracked only).

## 2. Untracked files in the main repo root

Commands: `git status --short` in `<repo>`; `ls -lT`; `head -c 200` on each kind. All are small (about 5 KB each, 7 KB for the log), so none was "large". None is ignored by `.gitignore` and none is tracked.

| File | Bytes | Modified | What it appears to be |
|---|---|---|---|
| best_so_far.json | 5386 | 2026-11-13 00:20:00 | Board JSON, first line `{`, name `size_16_official_eternity`; the best board found by the run |
| bw_portfolio.log | 7262 | 2026-11-13 00:20:00 | Blackwood portfolio run log; first line `# attempt 0: NEW BEST complete score=381 (native=false) deepest=52 nodes=30000000 (0.6s)`, with viewer URLs |
| bw_stage4_score373_1794525557.json | 5386 | 00:19:17 | Stage 4 board snapshot, score 373 |
| bw_stage4_score378_1794525556.json | 5387 | 00:19:16 | Stage 4 board snapshot, score 378 |
| bw_stage4_score381_1794525534.json | 5386 | 00:18:54 | Stage 4 board snapshot, score 381 |
| bw_stage4_score381_1794525557.json | 5385 | 00:19:17 | Stage 4 board snapshot, score 381 |
| bw_stage4_score429_1794525561.json | 5384 | 00:19:21 | Stage 4 board snapshot, score 429 |
| bw_stage4_score440_1794525600.json | 5386 | 00:20:00 | Stage 4 board snapshot, score 440 (highest of the set; best_so_far.json is the same size, last written at the same second) |

They were written at about 00:20 on 2026-11-13, in the repo root, by the Blackwood-style solver (`bw_` prefix), run with the working directory at the repo root.

## 3. Experiment directories, wt-d1 to wt-d5

Commands: `ls`, `test -e` per file, `grep -n TODO README.md`, `git ls-files -z <dir> | xargs -0 stat -f '%z %N'` filtered above 10485760 bytes.

| Worktree | Directory | README | PREREG | PREREG_AMENDMENTS | JUDGE | results/run-log.jsonl | Tracked files over 10 MB |
|---|---|---|---|---|---|---|---|
| wt-d1 | research/experiments/frame-cores | yes | yes | yes | NO | yes, but UNTRACKED (all of results/ is untracked) | none |
| wt-d2 | research/experiments/single-core-benchmark (the d2 branch has no `port-fidelity` directory; the work is in the benchmark's engine, bench-audit crate and `FIDELITY.md`) | yes | NO | NO | NO | NO | none (1074 tracked files) |
| wt-d3 | research/experiments/orders-and-pins | yes | yes | yes | NO | yes | results/b/b1.jsonl (13,060,858 bytes); results/gate/g8.jsonl (79,344,487 bytes) |
| wt-d4 | research/experiments/prefix-statistics | yes | yes | yes | yes | yes | none |
| wt-d5 | research/experiments/pin-map | yes | yes | yes | yes | yes | none |

TODO lines in READMEs (line: text, truncated):

- wt-d1 frame-cores README.md
  - 21: `- Stage 1 kill line (PREREG, real H): TODO`
  - 22: `- Stage 2 reading (PREREG + A1, median of b_H + log2 C, R2 core size): TODO`
  - 23: `- Stage 2 A2 floors (no pass line; sample cap log2(60/3) = 4.3 bits): TODO`
  - 24: `- Near-corner / far strata (A2): TODO`
  - 25: `- Bottom arcs, A1 readings and A2 nested cost: TODO`
  - 26: `- Uniform frames, corner claim (beta_H against 17.45 bits): TODO`
  - 27: `- Null boards: TODO`
  - 28: `- Stage 0: TODO (partial check at 08:37: every refuter passed (ii), (iii)`
- wt-d3 orders-and-pins README.md
  - 11: `(TODO below).** No Part A number is given here.`
  - 171: `## TODO after 22:00 (A7)`
- wt-d2, wt-d4, wt-d5: none.

Other facts: wt-d1's head commit subject is "README method matches A1 and A2" while its results table is still all TODO and `results/` (29 paths, e.g. PROGRESS.txt, cal/*.frames) is untracked, 136 KB in total. wt-d3's head commit says "Part A pending collec...".

## 4. Lead folder `research/program/night/2026-11-13/`

Commands: `du -ak`, `ls -l`, `git ls-files | wc -l`, `git status --short --ignored .`, `grep -o` for the U+2014 character per `.md`, a stdlib script that extracts `memos/`, `surveys/`, `research/`, `web/`, `engine/` path tokens from the four files and tests each (as given, and relative to the folder).

Top level (bytes): STATUS.md 82228; ROUTES.md 41507; THESIS_PLAN.md 19594; DRAFTS.md 7227; SURVEY_BRIEF.md 4644; DOCS_BRIEF.md 4276; GAPS_BRIEF.md 4089; memos/ (about 6984 KB on disk); surveys/ (about 484 KB on disk, 12 entries: 9 survey `.md` files, CITATIONS.REVIEW.md, DERIVATIONS.REVIEW.md, DERIVATIONS.REVIEW.scripts/).

memos/ holds: 21 `.md` files (memos for gap-1 to gap-6b, how-the-puzzle-was-made, nogood-frame-search-price, the d3 and d4 design reviews, plus a `.REVIEW.md` for most memos), and `*-scripts` / `*-review-scripts` folders with `.py`, `.out`, `.sh`, `.c` files.

Git: 417 files tracked in the folder. Untracked: none. Ignored (not tracked, present on disk): `memos/how-the-puzzle-was-made.scripts/__pycache__/` and `surveys/DERIVATIONS.REVIEW.scripts/__pycache__/`.

Em dashes (U+2014): zero in every `.md` file in the folder (checked all, STATUS, ROUTES, THESIS_PLAN, DRAFTS and the briefs included). Non-markdown files were not checked.

Paths in STATUS, THESIS_PLAN, ROUTES, DRAFTS that do not exist in this worktree (first occurrence only, `research/...` paths are resolved from this worktree root):

- STATUS.md line 18: `research/corrections` (exists on wt-docs and wt-pub, not on this branch)
- STATUS.md line 49: `research/corrections/2026-11-13-REVIEW.md` (exists only in wt-docs)
- STATUS.md line 90: `engine/src/main.rs` (exists inside the pin-map experiment's own engine on the d5 branch; at repo root `engine/src/` has no main.rs)
- DRAFTS.md line 28: `research/experiments/prefix-statistics` (exists on the d4 branch only)
- DRAFTS.md line 44: `research/experiments/pin-map` (exists on the d5 branch only)

All `memos/...` and `surveys/...` references resolve. ROUTES.md refers to branches and to `N12/`, `M12/` style prefixes that were not expanded by this check.

## 5. Secrets and personal data

Commands: `grep -rInE` with patterns for: `password` assignment, `token=`, `BEGIN ... PRIVATE KEY`, an email address, a home-directory path (`<home>/`, `<remote-home>/`), over the lead folder and the five experiment directories (excluding `target/` and `__pycache__`). Only locations are given.

- password assignment: 0 hits anywhere.
- `token=`: 0 hits anywhere.
- private key block: 0 hits anywhere.
- email address: 0 hits anywhere.
- Home-directory paths: all hits name the owner's own account (one user, no other user's path found). They are not secrets but they do put a local username in tracked files.
  - Lead folder, 26 hits: GAPS_BRIEF.md lines 7, 15; SURVEY_BRIEF.md lines 11, 14; DOCS_BRIEF.md line 6; memos/how-the-puzzle-was-made.REVIEW.md line 167; the rest are in memo scripts: gap-1-scripts/e2real.py (11), gap-1-scripts/04_generalise.py (30), gap-1-review-scripts/r1_real.py (28), r1_merge.py (11), r4_top_side.py (18), gap-3-review-scripts/build.sh (7), gap-4-review-scripts/run16.sh (6), run16b.sh (6), d5_check.py (2), gap-5-review-scripts/r5_clue_prefixes.py (19), r2a_prefix_death.py (21), rcommon.py (7), README.txt (3), nogood-frame-search-price.scripts/01_profile.py (5), 02_corner_cores.py (7), 03_thresholds.py (5), nogood-frame-search-price.REVIEW.scripts/r1_corners.py (11), r2_model.py (5). (Per-file figures are line counts of matching lines.)
  - wt-d1 frame-cores: scripts/dev-sync.sh line 5; scripts/ship.sh line 4; results/run-log.jsonl line 1 (untracked).
  - wt-d2 single-core-benchmark: 319 matching lines, mostly under engine/crates/bench-audit/fidelity-results/second-judgment/d2-rejudge/ (build.out, test.log, abl.txt, abl2.txt, mut-test.log, jobs.txt, runs/*/stdout and stderr) plus scripts/archive-standalone/CALIBRATION.md.
  - wt-d3 orders-and-pins: 14 hits, all in results/run-log.jsonl.
  - wt-d4 prefix-statistics: 134 matching lines, all in results/run-log.jsonl (479 occurrences).
  - wt-d5 pin-map: 15 hits, all in results/run-log.jsonl.

## 6. Needs a human or lead decision

- Commit or discard the eight untracked solver outputs in the main repo root (best_so_far.json, bw_portfolio.log, six bw_stage4 files); decide if they belong under a results folder or are scratch.
- wt-d1: results/ (29 untracked paths) is not committed, and README results are all TODO; decide what to commit before the branch is judged.
- wt-d1 has no JUDGE.md; wt-d3 has none and still has TODO (Part A pending, "after 22:00 (A7)").
- wt-d2 has no PREREG, amendments, JUDGE or run-log in its experiment folder (the work is in `single-core-benchmark`); confirm that is intended.
- wt-d3 tracks two jsonl files of 13 MB and 79 MB; decide whether they stay in git.
- No branch except `docs/2026-11-13-corrections` is on a remote; backup or push policy needs the owner (pushing needs a yes).
- STATUS.md refers to `research/corrections/...` and DRAFTS.md to d4 and d5 experiment folders that only exist on other branches; fix after merge, or qualify the paths.
- Tracked run logs and scripts contain the owner's `<home>/` paths (about 1000 lines in total); decide whether to scrub before any publication.
- Two ignored `__pycache__` directories sit in the lead folder; harmless, delete at the owner's discretion.
