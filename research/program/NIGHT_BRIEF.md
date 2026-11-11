# Night brief

Fill in the date, end time and core counts before launching. Tracks run in
this order; the lead may skip a track only with a written reason in STATUS.md.

Compute split: about 80% of core-hours on method tracks, at most 20% on
optimization tracks. Every track is labelled below; the label goes in
PREREG.md and the judge checks it (see PROTOCOL_TARGETS_PROPOSAL.md).

- Date: YYYY-MM-DD
- End time: 07:00 local
- Machine cores (`sysctl -n hw.ncpu` on macOS): N. Lead and scouts: 1. Leave 1 for the system.

## Track 0. Optimization: bb-mrv per-node cost, identical tree

Label: optimization. Cut bb-mrv's cost per node (allocation per node, domain
copies, hidden-single scan) without changing the search. Gate: the hash of the
node sequence equals the original on every calibration and test seed at R6,
R7 and 8x8 c=10. Target: wall clock ratio, 95% CI. If it passes, Track 1 uses
the faster binary. 1 core, at most 2 hours.

## Track 1. 9x9 high-cap pilot for bb-mrv (compute-heavy)

Label: measurement.

Question: does the bb-mrv / scb paired node ratio keep shrinking past 8x8?
A22 left 9x9 and 10x10 fully censored at 1e7 nodes.
- 9x9 c=11 (A22's +2 track), 8 fresh seeds (not 1..50, not 1001..1010), both solvers, cap 1e8 nodes per run.
- About 16 runs of up to ~2 h each at A22's measured 13k to 22k nodes/s. Give it half the cores.
- Kill: median paired log10 node ratio at 9x9 not below the 8x8 value (A22: ratio 0.147), or more than 4 of 8 pairs censored on both sides (uninformative).
- This is a pilot. It decides whether a 30-seed 9x9/10x10 run is worth requesting; it is not that run.

## Track 2. A20, blocker eviction as the backjump rule inside bb-mrv

Label: method.

AGENDA A20. Same propagation as bb-mrv; on a dead end, undo the decision that
blocks the failed cell (IMS-style) instead of the last one. Control: a twin that
undoes a random earlier decision with identical propagation. Rungs R6, R7, then
8x8 c=10 only if R7 passes. Cap 1e6 nodes. 2 cores.

## Track 3. A21, value order by propagation impact

Label: method.

AGENDA A21. Order a cell's options by how much they shrink neighbouring
domains (least first). Baseline bb-mrv, control bb-mrv with random value order
(A22's mrv-rand). R6, R7, 8x8 c=10. 2 cores.

## Track 4. A17 rebaselined: Hall matching on top of bb-mrv

Label: method.

A17 gained ~10% at 6x6 against the weaker scb baseline. Add the cut-multiset
check to bb-mrv and compare against bb-mrv by PROTOCOL.md, nodes and wall
clock. R6, R7, 8x8 c=10. 1 to 2 cores.

## Track 5 (if cores free up). Baseline tournament

Label: measurement.

Every solver-like component in the repo, including measurement tools
(sat-backdoors propagation, frame counters used as search, hard-ladder arms),
on R6, R7 and 8x8 c=10 with the same counter and 1e6 cap. Output: one ranked
table per rung. This is a measurement, judged like any other.

## Idea slot (no compute)

Goal: new mechanisms, not tuning. Procedure:
1. Ten `inventor` subagents (Opus). Six get one field each from the rotation in
   research-lead.md; four get a PAIR of unrelated fields and must produce a
   mechanism that needs both. Each starts from a named structural stall in the
   ledger, not from the existing code.
2. A `scout` (Sonnet) novelty-checks each against EXPLORED.md and the
   literature. Variants without a named missing mechanism are rejected.
3. Tournament: a fresh Opus `judge` compares surviving proposals in pairs
   (which better attacks a recorded stall with a cheaper falsifying test),
   giving an Elo-style ranking, as in Google's co-scientist.
4. Add one AlphaEvolve-style proposal: evolve bb-mrv's cell and value ordering
   code against PROTOCOL_TARGETS_PROPOSAL.md section 4.
5. Output: top five in AGENDA format, ranked, in night/<date>/IDEAS.md. Nothing
   runs tonight.
