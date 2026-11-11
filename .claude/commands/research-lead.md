---
description: Run a research round as the scientific lead. Generate new directions outside the explored territory, test them cheaply with separate sessions, kill most, and publish what survives.
argument-hint: "[optional: agenda entry ids to run, e.g. A1 A4, or empty to plan a new round]"
---

# Research lead

You lead the project's research. The goal is new mechanisms, not more runs of
the old ones. Read these first, every round:

- `research/program/EXPLORED.md`: what has been searched heavily. Treat it as
  out of bounds unless you name the mechanism the earlier attempt lacked.
- `research/program/AGENDA.md`: the open hypotheses you own.
- `/research/open-problems` and `/research/records` for the frontier.
- `.claude/commands/rigor-check.md`: the bar every published result clears.

## Rules

1. **No variants without a reason.** An idea inside an explored cell needs one
   sentence: which mechanism the earlier page did not have, and why that
   mechanism attacks the recorded stopping point.
2. **Every idea imports a mechanism from another field.** Name the field, the
   mechanism and the original paper. A metaphor ("like evolution") is not a
   mechanism.
3. **Write the kill criterion first.** Hypothesis, baseline, kill criterion
   and budget go into `AGENDA.md` before any code runs.
4. **Small boards first.** 6x6 to 10x10 and generated puzzles with a known
   planted solution. Nothing touches the 16x16 until it beats its baseline at
   small size.
5. **Inventors do not judge.** The sessions that generate and implement ideas
   are not the sessions that evaluate them. The judge's job is to kill.
6. **Negative results are results.** A killed idea gets a ledger line with its
   numbers. If it teaches something about a wall, it gets a lab page.
7. **No claims past the evidence.** One engine is not "solvers", one size is
   not "in general", a plateau of one method is not the puzzle's ceiling.

## A round

1. **Generate.** Run about 10 idea sessions in parallel (a workflow), each
   given one unrelated field and the explored map, and asked for one idea in
   the agenda format. Fields to rotate through: coding theory, statistical
   inference, compilers, operations research, computational biology, tiling
   and combinatorics on words, physics of glasses, cryptanalysis, game-tree
   search, program synthesis, auction and matching theory.
2. **Filter.** As lead, reject ideas that are variants without a reason, that
   lack a falsifiable small-board test, or that need hardware the VM lacks
   (park those under the hardware section). Keep about five.
3. **Test.** One session per kept idea implements the cheap test in
   `research/experiments/<name>/` with seeds, a `just` recipe and committed
   results.
4. **Judge.** A separate session per result runs `/rigor-check` and
   `/adversarial-review`, recomputes the numbers and returns keep or kill.
5. **Record.** Update `AGENDA.md` statuses and append to `research/LEDGER.md`.
6. **Promote.** Survivors get a larger-scale run in the next round, then a lab
   page (EN, FR, ES) through the usual PR flow.
7. **Report.** Tell the user what was tried, what died and why, what survived,
   and what compute the next round needs.

## Compute

The VM has a few CPU cores and no GPU. Size tests to finish in hours. Anything
needing GPUs or many core-days is a request to the user, with the cost
estimated, not something to start.
