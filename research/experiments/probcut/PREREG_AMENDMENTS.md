# PREREG amendments

The original `PREREG.md` (commit 7ae11f4) is kept unchanged.

## B1 (2026-11-11, before any calibration or test run): probe depth indexing

What: `PREREG.md` defines v_0 as the count of surviving immediate extensions
and, in the same section, v_k as nodes "exactly k cells deeper", which makes
v_0 and v_1 the same quantity. Fixed indexing: v_j = log2(1 + forward-checked
nodes exactly j + 1 cells deeper), j = 0..4. v_0 is the immediate-extension
count forward checking already sees; v_1..v_4 look 2..5 cells deeper. The
ProbCut probe for the selected k counts nodes k + 1 cells deeper. Each v_j
count has a budget of 20,000,000 placements (flagged if hit).

Why: the two definitions in the preregistration conflict; this keeps the
idea's intent (v_0 is the forward-checking baseline signal, k = 1..4 adds
depth beyond it).

Effect: none on the kill criterion's statistics or thresholds.
