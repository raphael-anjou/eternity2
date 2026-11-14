# Partial-match ladder: preregistration amendments

## A1 (2026-11-12, 20:35, before any calibration or test run): probe concurrency

What: the exchange-rate probe ran with 2 concurrent runs on `dell` (one
`break-r1`, one `pt`), not 6.
Why: other tracks held 10 of the 12 threads this track may share on `dell`
when the probe started; only 2 were free.
Effect: both rates are measured under the same machine load, so their ratio
(rho) is what the rule needs; absolute rates are lower than on an idle machine.
No `break-r1` probe run used its whole budget (every one completed a board), so
the preregistered fallback applies: rate = work_used / wall over all runs.

Addendum to A1 (20:28): the probe was stopped after 12 `break-r1` and 3 `pt`
rows because the other tracks were using all 12 shared threads on `dell` (this
track had misread the count as 10). It resumes (the runner skips finished rows)
when `dell` threads free up; rho is computed only after all 32 probe rows exist.

## A2 (2026-11-12, 20:33, before any calibration row): calibration on two machines

What: calibration jobs run on `mac-m1` as well as `dell`, each (entrant,
instance file) job wholly on one machine; every row records its machine.
`sa`, `pt` and `alns` calibration started on `mac-m1` (3 threads).
Why: `dell` had no free thread for this track until about 21:10.
Safeguard: the scores and work are functions of the work counter, not of the
clock. The cross-machine determinism check (PREREG "Test") is completed before
calibration is analysed; if an entrant differs between machines, all of its
calibration rows are taken from one machine (rerun if needed).

## A3 (2026-11-12, 21:35, before any valid calibration row of `break-*`): rho is not an integer

What: rho came out at 0.2 (raw 0.167: `break-r1` makes about 0.64M
placements per second on dell, because each placement scans every unused piece
and rotation, against 3.8M PT move evaluations per second). The job runner
computed `W_place = rho * W_eval` with shell integer arithmetic after the
dell lane truncated rho to 0, so the first `break-*` calibration rows on dell
ran with a budget of 0 placements.
Fix: those rows were deleted unanalysed; `W_place` is now computed in floating
point and rounded to the nearest integer (`scripts/run_jobs.sh`, `run.sh`).
The rule and the value rho = 0.2 are unchanged. Three of the 16 `break-r1`
probe runs used their whole 1e8 budget, so the preregistered rate (runs that
used the whole budget) was computed from those three.

## A4 (2026-11-12, 23:25, after the rung A test): prose correction in CALIBRATION.md

What: two sentences of the "Choice" section of `CALIBRATION.md` were corrected
after the test run: the best-entrant gap range at n = 10 and 12 (5.0% to 11.9%,
not 11.4%) and the count of `break-*` runs without a complete board there (47 of
64, not 62 of 64; completed boards sit 3.3% to 11.1% below the optimum). The
table, the rule and the choice of rung A are unchanged; the generated tables
were always the source.
