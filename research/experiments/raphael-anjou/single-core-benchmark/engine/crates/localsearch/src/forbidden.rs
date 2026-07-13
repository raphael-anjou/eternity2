// Forbidden-edge soft-penalty support for parallel tempering.
//
// A "forbidden" edge is an interior edge of the puzzle that we want PT
// to keep matched (= colors agree on both sides). Operationally these
// are the top-K "universal mismatch" edges identified offline by
// `scripts/universal_mismatches.py` — interior edges that fail to
// match in 47-63% of all PT-converged plateau states. Resolving all 6
// of the top-6 was the necessary (not sufficient) condition for the
// frame-first 450/480 breakthrough.
//
// The penalty is added at TWO points in PT:
//   (1) Each SA Metropolis test inside `run_sa_steps_fixed_temp` uses
//       an effective delta: delta_eff = delta_raw - K * delta_fmm,
//       where delta_fmm is the change in forbidden-mismatch count.
//   (2) Each PT replica-exchange acceptance uses an effective score
//       difference: (eff_lo - eff_hi) where eff = raw_score - K * fmm.
//
// The tracked `*score` and `*best_score` stay as RAW edge-match counts
// so the reported best is the genuine matched-edge metric. The penalty
// only steers acceptance.
//
// Algorithmic intent: at low T, the cold replica is biased away from
// configurations that violate the structural-defect set, even when SA
// would otherwise wander into them. At high T, the penalty has weak
// effect, preserving exploration. The PT temperature ladder sorts
// configurations by raw_score - K*fmm, so cold chains accumulate
// "good basin" configurations that match all forbidden edges.

use eternity2_core::{Board, Color, Position, Puzzle, BORDER};

/// A single forbidden interior edge identified by both endpoints.
///
/// `lo_pos` < `hi_pos` always (canonicalized at construction). For a
/// horizontal edge, `lo_pos` is the LEFT cell and `hi_pos = lo_pos + 1`;
/// the matching colors are `lo.right_side` (side index 1) vs
/// `hi.left_side` (side index 3).
///
/// For a vertical edge, `lo_pos` is the TOP cell and `hi_pos = lo_pos + W`;
/// the matching colors are `lo.bottom_side` (side index 2) vs
/// `hi.top_side` (side index 0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForbiddenEdge {
    pub lo_pos: Position,
    pub hi_pos: Position,
    /// 0 = vertical (top of `lo` faces bottom of `hi`), 1 = horizontal.
    /// We pack the two side indices implicitly:
    ///   horizontal: lo side 1 (right), hi side 3 (left)
    ///   vertical:   lo side 2 (bottom), hi side 0 (top)
    pub orientation: u8,
}

impl ForbiddenEdge {
    pub fn horizontal(lo_pos: Position) -> Self {
        Self { lo_pos, hi_pos: lo_pos + 1, orientation: 1 }
    }

    pub fn vertical(lo_pos: Position, width: u32) -> Self {
        Self { lo_pos, hi_pos: lo_pos + width, orientation: 0 }
    }

    pub fn lo_side(&self) -> u8 { if self.orientation == 1 { 1 } else { 2 } }
    pub fn hi_side(&self) -> u8 { if self.orientation == 1 { 3 } else { 0 } }
}

/// Parse the universal-mismatch list as produced by
/// `scripts/universal_mismatches.py` (the JSON we'll write from the
/// CLI). Accepts a JSON array of `["h", pos]` or `["v", pos]` entries.
///
/// On error returns the error message as `Err(String)` for clean CLI
/// reporting.
pub fn parse_forbidden_json(s: &str, width: u32) -> Result<Vec<ForbiddenEdge>, String> {
    let v: serde_json::Value = serde_json::from_str(s)
        .map_err(|e| format!("forbidden JSON parse: {e}"))?;
    let arr = v.as_array().ok_or("expected JSON array at top level")?;
    let mut out = Vec::with_capacity(arr.len());
    for (i, item) in arr.iter().enumerate() {
        let pair = item.as_array().ok_or_else(|| format!("entry {i}: not array"))?;
        if pair.len() != 2 {
            return Err(format!("entry {i}: expected [\"h\"|\"v\", pos], got len {}", pair.len()));
        }
        let typ = pair[0].as_str().ok_or_else(|| format!("entry {i}: type not string"))?;
        let pos = pair[1].as_u64().ok_or_else(|| format!("entry {i}: pos not int"))? as Position;
        match typ {
            "h" => out.push(ForbiddenEdge::horizontal(pos)),
            "v" => out.push(ForbiddenEdge::vertical(pos, width)),
            other => return Err(format!("entry {i}: unknown type {other}")),
        }
    }
    Ok(out)
}

/// Index `forbidden_edges_by_cell[cell] = list of edge indices incident
/// to this cell`. Used to evaluate forbidden-mismatch contribution from
/// a small set of touched cells without rescanning the whole list.
///
/// In practice |forbidden| ≤ 20 and the full scan is also O(20), so the
/// index is a clarity win, not a perf win.
pub fn build_edges_by_cell(
    forbidden: &[ForbiddenEdge],
    n_cells: usize,
) -> Vec<Vec<u16>> {
    let mut by_cell: Vec<Vec<u16>> = vec![Vec::new(); n_cells];
    for (i, e) in forbidden.iter().enumerate() {
        by_cell[e.lo_pos as usize].push(i as u16);
        by_cell[e.hi_pos as usize].push(i as u16);
    }
    by_cell
}

/// Whole-board forbidden-mismatch count.
///
/// An edge is "mismatched" iff BOTH endpoints are placed AND the colors
/// at the two facing sides differ AND neither facing color is BORDER
/// (=0; matching the SA score function's behavior).
///
/// O(|forbidden|).
pub fn fmm_full(
    puzzle: &Puzzle,
    board: &Board,
    forbidden: &[ForbiddenEdge],
) -> u32 {
    let mut m = 0u32;
    for e in forbidden {
        if is_mismatched(puzzle, board, e) { m += 1; }
    }
    m
}

/// Forbidden-mismatch contribution from edges incident to `touched`
/// cells. Each edge is counted at most once even if both endpoints are
/// in `touched`.
///
/// O(|touched| × max_incidence). For our use case, `touched` is at
/// most 9 cells (3×3 cluster) and max_incidence is at most 4 (sides).
pub fn fmm_touched(
    puzzle: &Puzzle,
    board: &Board,
    forbidden: &[ForbiddenEdge],
    edges_by_cell: &[Vec<u16>],
    touched: &[Position],
) -> u32 {
    // Use a small bitset of edge indices to avoid double-counting.
    // |forbidden| is small (≤32 in practice) so a u64 bitset suffices.
    debug_assert!(forbidden.len() <= 64, "fmm_touched bitset overflow; bump to Vec<bool>");
    let mut seen: u64 = 0;
    let mut m = 0u32;
    for &p in touched {
        for &eidx in &edges_by_cell[p as usize] {
            let bit = 1u64 << eidx;
            if (seen & bit) != 0 { continue; }
            seen |= bit;
            if is_mismatched(puzzle, board, &forbidden[eidx as usize]) {
                m += 1;
            }
        }
    }
    m
}

#[inline]
fn is_mismatched(puzzle: &Puzzle, board: &Board, e: &ForbiddenEdge) -> bool {
    let Some((pid_lo, rot_lo)) = board.get(e.lo_pos) else { return false; };
    let Some((pid_hi, rot_hi)) = board.get(e.hi_pos) else { return false; };
    let edges_lo = piece_edges_rotated(puzzle, pid_lo, rot_lo);
    let edges_hi = piece_edges_rotated(puzzle, pid_hi, rot_hi);
    let c_lo: Color = edges_lo[e.lo_side() as usize];
    let c_hi: Color = edges_hi[e.hi_side() as usize];
    // Same convention as State::score: BORDER (=0) is treated as
    // "off-board / wildcard"; mismatch only counts when both sides are
    // non-BORDER and differ.
    c_lo != BORDER && c_hi != BORDER && c_lo != c_hi
}

#[inline]
fn piece_edges_rotated(
    puzzle: &Puzzle,
    pid: eternity2_core::PieceId,
    rot: eternity2_core::Rotation,
) -> [Color; 4] {
    let piece = puzzle.piece(pid)
        .expect("piece id from board must exist in puzzle");
    piece.edges.rotated(rot).as_array()
}

/// Per-call context passed to constrained SA inner loops. Owned by
/// the PT loop, threaded by-reference into `run_sa_steps_fixed_temp`.
///
/// Holds borrowed references because the SA inner loop is performance-
/// critical; we want zero allocation per move and zero cloning of the
/// forbidden list.
pub struct ForbiddenContext<'a> {
    pub edges: &'a [ForbiddenEdge],
    /// `edges_by_cell[cell] = list of edge indices`. Precomputed.
    pub edges_by_cell: &'a [Vec<u16>],
    pub k: i64,
}

impl<'a> ForbiddenContext<'a> {
    /// Compute the forbidden-mismatch count over edges incident to any
    /// of the given `touched` cells. Each edge counted at most once.
    /// O(|touched| × max_incidence_per_cell), with |edges| ≤ 64.
    #[inline]
    pub fn fmm_at(&self, puzzle: &Puzzle, board: &Board, touched: &[Position]) -> u32 {
        fmm_touched(puzzle, board, self.edges, self.edges_by_cell, touched)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_endpoints() {
        let e = ForbiddenEdge::horizontal(180);
        assert_eq!(e.lo_pos, 180);
        assert_eq!(e.hi_pos, 181);
        assert_eq!(e.orientation, 1);
        assert_eq!(e.lo_side(), 1);
        assert_eq!(e.hi_side(), 3);
    }

    #[test]
    fn vertical_endpoints() {
        let e = ForbiddenEdge::vertical(91, 16);
        assert_eq!(e.lo_pos, 91);
        assert_eq!(e.hi_pos, 91 + 16);
        assert_eq!(e.orientation, 0);
        assert_eq!(e.lo_side(), 2);
        assert_eq!(e.hi_side(), 0);
    }

    #[test]
    fn parse_top6() {
        // Exact top-6 from RESEARCH_NOTES_5
        let s = r#"[
            ["h", 180], ["h", 91], ["v", 162],
            ["h", 188], ["v", 183], ["v", 180]
        ]"#;
        let v = parse_forbidden_json(s, 16).unwrap();
        assert_eq!(v.len(), 6);
        assert_eq!(v[0], ForbiddenEdge::horizontal(180));
        assert_eq!(v[1], ForbiddenEdge::horizontal(91));
        assert_eq!(v[2], ForbiddenEdge::vertical(162, 16));
    }

    #[test]
    fn build_index() {
        let v = vec![
            ForbiddenEdge::horizontal(180),
            ForbiddenEdge::vertical(180, 16),
        ];
        let by_cell = build_edges_by_cell(&v, 256);
        assert_eq!(by_cell[180], vec![0, 1]);
        assert_eq!(by_cell[181], vec![0]);
        assert_eq!(by_cell[180 + 16], vec![1]);
        assert!(by_cell[0].is_empty());
    }
}
