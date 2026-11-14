# pin-map amendments

## A1 (2026-11-13 11:42 CET): p10 cut to board 1, stage order changed

What: the `all` chain was stopped during `p10` after board 1 of 5 finished
(board 2 partial; its rows are kept and the analysis uses only boards with
every unit). `n8` and `mrv8` run next; `p10` resumes for boards 2..5 only if
the wall-clock budget allows, and the README states how many boards it has.

Why: `p10` measured 423 s per board on this shared core (575 exact trees,
the 20 random draws at k = 2, 3 dominate), so the full stage would push the
report about 40 minutes past the two-hour target. PREREG lists `p10` as the
second stage to drop for time; the first (`mrv8` k > 8) costs almost nothing,
and item 4 (bb-mrv) answers a question no other stage does, so it is kept
whole. The killed `p10` run has a start line with no end line in
`results/run-log.jsonl`.

When: after `s8`, `g8`, `p8`, `pairs8`, `s8c`, `g8c`, `p8c`, `s10`, `g10`
had finished, before `n8` and `mrv8` started.

Addendum (12:06 CET): after `n8` and `mrv8` finished, `p10` resumed and
completed board 2; it was stopped again during board 3 to report on time.
The analysis uses boards 1 and 2 for policies (b) to (g) at 10x10 (rows of
board 3 are in `results/p10.jsonl` but incomplete, so excluded). Two p10
start lines in `results/run-log.jsonl` have no end line.

## A2 (2026-11-13, added after judgment): item 2 reported as aggregate ratio

What: PREREG item 2 asks for (median, mean) of worth(pair) / (worth(p) +
worth(q)) by distance. The README reports a worth-weighted aggregate ratio
(sum of pair worths over sum of single worths) by distance, and the median
only overall (0.957; mean 0.937).

Why recorded: the judge found it was not declared as a deviation. Per group
the aggregate / median / mean are: adjacent 0.753 / 0.836 / 0.825; distance 2
0.854 / 0.912 / 0.887; distance 3 0.914 / 0.937 / 0.920; distance 4 or more
0.958 / 0.966 / 0.955; same row 0.847 / 0.922 / 0.878; different rows 0.942 /
0.959 / 0.945. The means do not change the conclusion. No result file changed.
