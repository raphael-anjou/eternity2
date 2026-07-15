// Verhaard's set-composition swap-annealing approach to Eternity II.
//
// Source: louis.verhaard, groups.io 105190116 (2008-04-11). The algorithm
// behind Verhaard's 467/480 ceiling. See also:
//
// reference_verhaard_actual_method.md — auto-memory, corrects 's
//     "2x3 ranking + depth banding" memory which was wrong.
//
// Structure (top-down):
//
//   1. Outer SA over piece-set composition. Choose 180-190 of the 196
//      inner pieces to commit to. Metric = #(2x2 sub-tilings exhausted
//      on this set). Swap pieces in/out via Metropolis until a local
//      optimum.    [crate::sa]
//   2. Phase-1 scaffolding backtrack. Place ~80 pieces with the SA-
//      chosen "good 180" set + the 4 corners + 56 edges. Order:
//      "worst performers" first.   [crate::scaffold]
//   3. Phase-2 exhaustive completion. With scaffold pinned, search
//      the remaining ~176 pieces.  [crate::phase2]
//
// 2x2 sub-tiling counting machinery is in [crate::tile2x2].

#![forbid(unsafe_code)]

pub mod sa;
pub mod scaffold;
pub mod tile2x2;
