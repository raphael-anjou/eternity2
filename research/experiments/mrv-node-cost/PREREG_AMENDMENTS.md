# Preregistration amendments: Track 0, bb-mrv node cost

## A1: output file layout

- **What:** `PREREG.md` (Files section) names `results/gate.jsonl`,
  `results/gate-supp.jsonl` and `results/timing.jsonl`. The run writes split
  files under `results/raw/` instead (`gate-r6-cal.jsonl`, `gate-r6-test.jsonl`,
  `gate-r7-*.jsonl`, `gate-r8-*.jsonl`, `gate-supp-n9.jsonl`,
  `gate-supp-n10.jsonl`, `time-r6.jsonl`, `time-r7.jsonl`, `time-r8.jsonl`,
  `time-r8-fresh.jsonl`), and `analysis/analyze.py` reads them from there.
- **Why:** one file per stage and rung lets `run.sh` skip finished pieces and
  resume an interrupted run.
- **When:** found by the judge after the run (see `JUDGE.md`, section 3).
  Recorded here after the results existed; `PREREG.md` is unchanged.
- **Effect on the gate or the kill criterion:** none. Same rows, same
  instances, same fields; only the file layout differs.
