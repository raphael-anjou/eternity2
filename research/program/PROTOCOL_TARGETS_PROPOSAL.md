# Proposed amendment to hard-ladder PROTOCOL.md: targets and track types

Status: proposal, not yet applied. It adds rules and weakens none, so it is
compatible with the protocol's "may add, not weaken" clause. Apply it as a
recorded amendment before the first night that uses it.

## 1. Two track types, declared before any code

- **Method track:** the search tree changes (new propagation, ordering,
  backtracking, decomposition, or a new solver). Judged by section 2.
- **Optimization track:** the search tree must not change. On every
  calibration and test seed, the candidate's node sequence (cell, piece,
  rotation per `try_assign`) is identical to the reference implementation's,
  checked by hashing the sequence. Judged by wall clock and instructions
  only. If any node count differs, the track is reclassified as a method
  track and must pass section 2 instead.
- The label is in PREREG.md. The judge checks it holds.

## 2. Targets for method tracks, in order

1. **Gate.** Every claimed solve verified by the harness; for exact methods,
   no seed where the planted solution's branch is cut. Fails: invalid.
2. **Main target: solves within a fixed work budget at critical colours.**
   Rungs R6 (6x6 c=5) and R7 (7x7 c=6). At 8x8 the critical rung (c=8)
   is unsolved by every current solver, so 8x8 c=10 (amendment A3) is used
   and reported as above-critical. Work = nodes plus counted
   propagation operations (issue #22), or instructions retired on one pinned
   core where `perf` is available. Report both forms when possible.
3. **Effect size.** G1 on work instead of nodes, paired against the
   strongest baseline at that rung (bb-mrv at 8x8 c=10 per A22, scb at R6/R7).
4. **Growth.** G2 on work over n = 6..8 on a fixed colour track. This is the
   only target that bears on the 16x16 board; a constant factor does not.
5. **Guardrail.** Wall clock always reported next to work.

Nodes stay a mandatory column: a method that wins on nodes and loses on work
is reported as such.

## 3. Incomplete methods

Work or time to reach a verified solution on R6/R7. Where nothing solves,
best matched edges at a fixed CPU budget, never compared with exact-search
targets.

## 4. Evolved code (AlphaEvolve style)

Evolution sees only calibration seeds and scores on target 2. Selection uses
targets 3 and 4 on test seeds and one board size larger than any used during
evolution. Evolved code plugs into ordering and propagation hooks only; the
harness, counter and verifier are outside its reach.
