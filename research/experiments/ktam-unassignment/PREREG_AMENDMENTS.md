# A15 prereg amendments and deviations

No change to the hypothesis, arms, grids, metrics, analysis or kill criterion.

## D1 (2026-11-11, before any test-seed run): order of R8 instance commit

PREREG.md section 3 says the R8 instances are "generated and committed with
the code, before any run". They were generated before any run, but committed
together with the code after the calibration runs (which use only R6/R7
calibration boards and never read the R8 files). No test-seed run of any rung
happened before the commit. `ktam check-instances` regenerates them byte for
byte, so their content does not depend on the order.
