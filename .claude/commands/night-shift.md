---
description: Lead an unattended night of Eternity II research. Fable leads, Opus runs and judges, Sonnet scouts and does plumbing.
argument-hint: "[end time, e.g. 07:00]"
---

# Night shift lead

You are the scientific lead for one night. You plan, assign, check and
record. You do not run experiments yourself and you never judge a result.
Follow .claude/commands/research-lead.md; this file only adds the night rules.

## Start (first 20 minutes)

1. Read AGENTS.md, research/program/EXPLORED.md, AGENDA.md, LEDGER.md (last 30 entries),
   research/program/NIGHT_BRIEF.md, hard-ladder/PROTOCOL.md.
2. Check `git status` is clean, `origin/main` contains A22 (`research/experiments/mrv-baseline/`),
   `cargo build --release` works for the ladder, and `nproc` matches the core plan in the brief.
3. Write research/program/night/<date>/STATUS.md: tracks, assigned cores, start time,
   expected finish per track. Commit it on branch night/<date>-lead.

## Running the tracks

- One `runner` subagent per track, in the order and with the cores the brief gives.
  Run tracks in the background so several proceed at once; never oversubscribe cores.
- When a runner reports, start a fresh `judge` subagent on that branch. Never let the runner
  judge, never pass the runner's summary as evidence, only the branch name.
- A judge verdict `invalid` allows ONE fix by a new runner and ONE rejudge. Then record and move on.
- Keep the compute split in the brief (about 80% method, at most 20% optimization). If a runner turns a method track into tuning (only parameters change), stop it and record why.
- Run the idea slot exactly as the brief describes, with `inventor` subagents.
- Use `scout` for novelty checks and paper lookups, `plumber` for lint, CI and ledger drafts.
- Every hour: update STATUS.md (what finished, verdicts, what is running, cores in use) and push it.
- If a track stalls (no commit for 90 minutes), read its log, then either restart it once or mark it
  `abandoned` with the reason.

## Hard limits

- No merges, no force pushes, no pushes to main, no AWS or any paid service, no edits to
  EXPLORED.md or PROTOCOL.md, no `pnpm build`.
- No 16x16 runs. Generated-board results are never official Eternity II progress.
- Kill criteria never change after a test seed has run.
- Stop starting new tracks 90 minutes before the end time; let running ones finish or checkpoint.

## Unplanned observations

Collect the "Unplanned observations" section of every runner and judge report into
night/<date>/OBSERVATIONS.md. Do not act on them tonight unless the brief has a free slot;
they are leads for the morning, not results.

## End (by the end time)

Write research/program/night/<date>/REPORT.md for Raphael, short:
1. One line per track: verdict (judged), headline number, branch.
2. Unplanned observations worth a follow-up, ranked.
3. What failed operationally (stalls, blocked commands, limits hit).
4. Draft AGENDA and LEDGER entries (not committed to main).
Push every branch. Open one draft PR per judged experiment plus one for the lead branch.
