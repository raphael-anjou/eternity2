# Gap sessions: common brief (2026-11-13, afternoon)

Seven desk sessions, one per gap in the arc's working explanation of why Eternity II costs 2^134.4 nodes. Each session digs into ONE gap: literature, mathematics, algorithm design, small checks. The owner wants a solution, not a record. A method that gains fewer than about 10 bits is a footnote; say so and move on.

## Read first, in this order

All paths are under `<repo>/research/program/night/`.

1. `2026-11-13/THESIS_PLAN.md`: the question, the numbers, obstructions O1 to O3 and their stated scope.
2. `2026-11-13/ROUTES.md`: the index of 60 routes already examined. Anything you propose must be checked against it. If your idea is a row there, say which and what is different.
3. `2026-11-13/surveys/` (eight field surveys and two referee reports) and `2026-11-13/memos/`.
4. `2026-11-13/STATUS.md`, entries from 12:15 on: today's results (D4 tilt of about 1 bit, arc consistency about 5 bits of nodes for 13 bits of cost, D5 pin map on small boards, D3 pins on painted 16x16 boards).
5. `2026-11-12/REPORT.md` for last night's sixteen tracks. Known error in it: the interior under the true frame is NOT 10^8.3; it is about 10^13.6 in the mean.

Useful code and data (read only): the measured level profile of the real tree at commit `dc71ad56`, `research/experiments/real-puzzle-profile/` (use `git -C <repo> show dc71ad56:<path>`); experiment worktrees `<repo>` to `wt-d5`; the official pieces under `research/experiments/single-core-benchmark/engine/data/puzzles/`.

## Facts to hold

- 16x16, 256 tiles, 22 colours (5 on the frame, 17 inside), five clues at (2,2), (2,13), (13,2), (13,13), (8,7).
- Row scan with five clues: 2^134.4 nodes, widest level at depth 158, first-moment model within 0.03 digits.
- Feasible for the owner: about 10^15 nodes (one month). The gap is 56 to 73 bits.
- Expected number of solutions: about 14,702 with the starter clue, about 1 with all five. Never write "one solution".

## What to deliver

One memo, written to `2026-11-13/memos/gap-<n>-<slug>.md` (the file name is in your assignment), in plain prose, no em dash anywhere, under 2,500 words plus appendices:

1. The gap restated precisely, and what exactly the existing evidence does and does not cover.
2. What the literature has (cite with links you have opened; mark anything not opened as unverified).
3. Your best one to three concrete mechanisms. For each: the algorithm stated so that someone could implement it, a price in bits on the real 16x16 with the derivation, and the single assumption the price rests on.
4. For each mechanism, the smallest experiment that would falsify it: board size, what is measured, the baseline, the kill line, the cost in core-hours.
5. Your own odds that it yields 10 bits, and 56 bits.
6. If the gap is empty, say so and give the argument. An empty gap with a proof sketch is a full result. Do not manufacture a proposal.

## Rules

- You are not asked to confirm that the gap is promising. The lead rates gaps 1 to 4 at a few percent each for a few bits. Disagree only with numbers.
- Check every derived number with a short script where one is possible. Scripts go in `2026-11-13/memos/gap-<n>-scripts/`. Python as `python3 -I` (stdlib) or `uv run --with <pkg>`; never pip.
- Compute: at most 1 core, `nice -n 10`, no single run over 5 minutes, about 30 core-minutes in total. No remote machine. No build of the web site.
- Write only inside your memo file and your scripts folder. Do not commit, do not push, do not edit any other file.
- Toy boards must be labelled as toys. Results on generated boards are never Eternity II progress.
- Your memo is a draft until a referee has ruled on it. Mark each statement measured, derived, cited or guessed.
- Web requests must not carry the owner's name, email or project name.

Final reply to the lead: under 250 words. The verdict first (mechanism and price, or "empty"), then the falsifying experiment, then what you could not check.
