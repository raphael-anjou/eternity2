# A15 prereg amendments and deviations

No change to the hypothesis, arms, grids, metrics, analysis or kill criterion.

## D1 (2026-11-11, before any test-seed run): order of R8 instance commit

PREREG.md section 3 says the R8 instances are "generated and committed with
the code, before any run". They were generated before any run, but committed
together with the code after the calibration runs (which use only R6/R7
calibration boards and never read the R8 files). No test-seed run of any rung
happened before the commit. `ktam check-instances` regenerates them byte for
byte, so their content does not depend on the order.

## E1 (2026-11-11, after the binding results): exploratory equal-CPU check at R7

Added at the research lead's request after the verdict, so it is
exploratory and does not change the preregistered verdict, which stays on the
event budget. Reason: B1 runs AC-3 and GAColor propagation inside each node, so
nodes and events do not charge its work per node. What is added: `ktam
explore-cpu` measures per-thread CPU time (`CLOCK_THREAD_CPUTIME_ID`), nodes
per CPU second and the native propagation counter per node for every arm on
the R7 test seeds at 1e6 nodes. The method arms are then rerun at a node cap
that gives them about the same CPU time as a censored B1 run. Hardware
instruction counters are not available on this VM (`perf` reports
`<not supported>`), so no instruction counts are given. Outputs go to
`results/exploratory/cpu/`.
