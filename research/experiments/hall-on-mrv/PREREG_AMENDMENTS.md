# hall-on-mrv prereg amendments

No change to the hypothesis, arms, grids, metrics, analysis or kill criterion.

## A1 (2026-11-12, found by the judge after the run): R7 calibration seed count

What: PREREG.md section 1 and the README said one R7 calibration seed (1001)
was run before the calibration was stopped. `results/calibration/r7.jsonl`
holds two seeds, 1001 and 1002 (4 arms each, 8 rows; `r7.log` shows both). The
PREREG.md text is frozen and left as written; this entry is the correction, and
the README sentence now states two seeds.

When: found by the judge after the test runs, corrected the same night.

Effect on the kill criterion: none. The calibration rows were used only for
the small check in PREREG.md section 1 and for timing. No arm, grid, metric,
test seed or threshold depends on them.
