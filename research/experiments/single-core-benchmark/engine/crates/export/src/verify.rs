// Canonical board verifier — vol-118 T7 consolidation.
//
// Replaces per-bin verification logic with a single, comprehensive
// VerifyReport struct + verify() function. Catches:
//   - piece-uniqueness violations (one piece placed twice)
//   - border-consistency violations (piece's BORDER edge facing
//     interior, or interior edge facing puzzle boundary)
//   - hint-compliance failures (canonical hints not in their pinned
//     positions+rotations)
//   - edge-match score (matched / total adjacencies among placed cells)
//
// The border-consistency check is the load-bearing addition: it
// catches the class of bugs that escaped vol-15 → vol-118 in the
// blackwood-fast candidate-bucket bug.

use eternity2_core::{Board, Hints, PieceId, Puzzle, Rotation, BORDER};

use crate::score::score_board;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BorderViolation {
    pub position: u32,
    pub side: BorderSide,
    /// What the piece's edge actually is at this side.
    pub piece_edge_color: u8,
    /// What was expected (BORDER if facing outside the puzzle, an
    /// interior color if facing inside).
    pub expected: ExpectedKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderSide {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpectedKind {
    /// Position is on the puzzle border on this side; edge must be BORDER (0).
    BorderEdge,
    /// Position is interior on this side; edge must be a non-BORDER color.
    InteriorEdge,
}

#[derive(Debug, Clone)]
pub struct HintCompliance {
    pub total: u32,
    pub matched: u32,
    /// For each hint that DOES NOT match, the (hint, actual_placement) pair.
    pub mismatches: Vec<HintMismatch>,
}

#[derive(Debug, Clone)]
pub struct HintMismatch {
    pub hint_position: u32,
    pub expected_piece_id: PieceId,
    pub expected_rotation: Rotation,
    pub actual: Option<(PieceId, Rotation)>,
}

#[derive(Debug, Clone)]
pub struct VerifyReport {
    pub placed_count: u32,
    pub total_cells: u32,
    pub matched: u32,
    pub total_adjacencies: u32,
    pub unique_pieces: u32,
    pub duplicate_pieces: Vec<PieceId>,
    pub border_violations: Vec<BorderViolation>,
    pub hint_compliance: HintCompliance,
}

impl VerifyReport {
    /// Returns true iff: all placed pieces unique, no border
    /// violations, all hints matched. (Edge score is informational.)
    #[must_use]
    pub fn is_legal(&self) -> bool {
        self.duplicate_pieces.is_empty()
            && self.border_violations.is_empty()
            && self.hint_compliance.mismatches.is_empty()
    }

    /// Returns true iff the board is legal AND fully placed.
    #[must_use]
    pub fn is_legal_complete(&self) -> bool {
        self.is_legal() && self.placed_count == self.total_cells
    }
}

#[must_use]
pub fn verify(puzzle: &Puzzle, hints: &Hints, board: &Board) -> VerifyReport {
    let total_cells = puzzle.cell_count();
    let mut placed_count = 0u32;
    let mut seen_pieces: std::collections::HashMap<PieceId, u32> =
        std::collections::HashMap::new();
    let mut duplicate_pieces: Vec<PieceId> = Vec::new();

    for pos in 0..total_cells {
        if let Some((pid, _)) = board.get(pos) {
            placed_count += 1;
            *seen_pieces.entry(pid).or_insert(0) += 1;
        }
    }
    for (&pid, &count) in &seen_pieces {
        if count > 1 {
            duplicate_pieces.push(pid);
        }
    }
    duplicate_pieces.sort_by_key(|p| u32::from(*p));

    let unique_pieces = seen_pieces.len() as u32;

    // Border-consistency check: for each placed cell, verify that
    // its 4 rotated edges respect the cell's position.
    let mut border_violations: Vec<BorderViolation> = Vec::new();
    let w = puzzle.width;
    let h = puzzle.height;
    for pos in 0..total_cells {
        let Some((pid, rot)) = board.get(pos) else { continue };
        let Some(piece) = puzzle.piece(pid) else { continue };
        let e = piece.edges.rotated(rot).as_array();
        let x = pos % w;
        let y = pos / w;

        // North side
        let on_top = y == 0;
        check_side(&mut border_violations, pos, BorderSide::Top, e[0], on_top);
        // East
        let on_right = x + 1 == w;
        check_side(&mut border_violations, pos, BorderSide::Right, e[1], on_right);
        // South
        let on_bottom = y + 1 == h;
        check_side(&mut border_violations, pos, BorderSide::Bottom, e[2], on_bottom);
        // West
        let on_left = x == 0;
        check_side(&mut border_violations, pos, BorderSide::Left, e[3], on_left);
    }

    // Hint compliance.
    let mut hint_mismatches: Vec<HintMismatch> = Vec::new();
    let mut hint_matched = 0u32;
    for hint in hints.hints.iter() {
        let actual = board.get(hint.position);
        let matches = actual
            .map(|(pid, rot)| pid == hint.piece_id && rot == hint.rotation)
            .unwrap_or(false);
        if matches {
            hint_matched += 1;
        } else {
            hint_mismatches.push(HintMismatch {
                hint_position: hint.position,
                expected_piece_id: hint.piece_id,
                expected_rotation: hint.rotation,
                actual,
            });
        }
    }
    let hint_compliance = HintCompliance {
        total: hints.hints.len() as u32,
        matched: hint_matched,
        mismatches: hint_mismatches,
    };

    let (matched, total_adjacencies) = score_board(puzzle, board);

    VerifyReport {
        placed_count,
        total_cells,
        matched,
        total_adjacencies,
        unique_pieces,
        duplicate_pieces,
        border_violations,
        hint_compliance,
    }
}

fn check_side(
    out: &mut Vec<BorderViolation>,
    position: u32,
    side: BorderSide,
    edge_color: u8,
    on_boundary: bool,
) {
    if on_boundary && edge_color != BORDER {
        out.push(BorderViolation {
            position,
            side,
            piece_edge_color: edge_color,
            expected: ExpectedKind::BorderEdge,
        });
    } else if !on_boundary && edge_color == BORDER {
        out.push(BorderViolation {
            position,
            side,
            piece_edge_color: edge_color,
            expected: ExpectedKind::InteriorEdge,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eternity2_core::{Edges, Hint, Piece};

    fn tiny_puzzle() -> Puzzle {
        // 2x2 puzzle, 1 interior color.
        // Layout (TL TR BL BR):
        //   Piece 0: top-left corner    (top=BORDER, right=1, bottom=1, left=BORDER)
        //   Piece 1: top-right corner   (top=BORDER, right=BORDER, bottom=1, left=1)
        //   Piece 2: bottom-left corner (top=1, right=1, bottom=BORDER, left=BORDER)
        //   Piece 3: bottom-right       (top=1, right=BORDER, bottom=BORDER, left=1)
        let pieces = vec![
            Piece::new(0, Edges::new(BORDER, 1, 1, BORDER)),
            Piece::new(1, Edges::new(BORDER, BORDER, 1, 1)),
            Piece::new(2, Edges::new(1, 1, BORDER, BORDER)),
            Piece::new(3, Edges::new(1, BORDER, BORDER, 1)),
        ];
        Puzzle::new(2, 2, 2, pieces).expect("tiny puzzle")
    }

    #[test]
    fn legal_complete_board() {
        let p = tiny_puzzle();
        let mut b = Board::empty(&p);
        b.place(0, 0u16, Rotation::R0);
        b.place(1, 1u16, Rotation::R0);
        b.place(2, 2u16, Rotation::R0);
        b.place(3, 3u16, Rotation::R0);
        let hints = Hints::new(vec![]);
        let report = verify(&p, &hints, &b);
        assert!(report.is_legal_complete(), "{:?}", report);
        assert_eq!(report.placed_count, 4);
        assert_eq!(report.duplicate_pieces.len(), 0);
        assert_eq!(report.border_violations.len(), 0);
    }

    #[test]
    fn detects_border_violation() {
        // Place top-left piece (which has BORDER on top+left) at the
        // bottom-right cell (which needs BORDER on bottom+right).
        // 3 violations expected: top is BORDER but not on top, left
        // is BORDER but not on left, bottom is non-BORDER but on
        // bottom, right is non-BORDER but on right.
        let p = tiny_puzzle();
        let mut b = Board::empty(&p);
        b.place(3, 0u16, Rotation::R0);
        let hints = Hints::new(vec![]);
        let report = verify(&p, &hints, &b);
        assert!(!report.is_legal());
        assert_eq!(report.border_violations.len(), 4, "{:?}", report);
    }

    #[test]
    fn detects_duplicate_piece() {
        let p = tiny_puzzle();
        let mut b = Board::empty(&p);
        b.place(0, 0u16, Rotation::R0);
        b.place(1, 0u16, Rotation::R0); // duplicate
        let hints = Hints::new(vec![]);
        let report = verify(&p, &hints, &b);
        assert!(!report.is_legal());
        assert!(report.duplicate_pieces.contains(&PieceId::from(0u16)));
    }

    #[test]
    fn detects_hint_mismatch() {
        let p = tiny_puzzle();
        let mut b = Board::empty(&p);
        b.place(0, 1u16, Rotation::R0); // wrong piece
        let hints = Hints::new(vec![Hint {
            position: 0,
            piece_id: PieceId::from(0u16),
            rotation: Rotation::R0,
        }]);
        let report = verify(&p, &hints, &b);
        assert!(!report.is_legal());
        assert_eq!(report.hint_compliance.matched, 0);
        assert_eq!(report.hint_compliance.mismatches.len(), 1);
    }
}
